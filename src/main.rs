use crate::true_gear::def::TrackObject;
use crate::true_gear::TrueGearClient;
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{FromSample, Sample};
use hound::{SampleFormat, WavSpec};
use once_cell::sync::Lazy;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tokio::select;
use serde::Deserialize;
use std::fs;

mod audio;
mod true_gear;

static DATA_BUFFER: Lazy<Arc<Mutex<Vec<u8>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("TrueGear Audio Driver v0.0.1 by xkeyC");

    // 初始化频驱动
    let audio_device = audio::init()?;
    let audio_config = audio_device.default_output_config()?;

    println!("Connecting to TrueGear ...");
    let mut client = true_gear::connect().await?;
    client.test_all().await?; // 震动所有模块 100ms

    let client_arc = Arc::new(Mutex::new(client));
    let client_arc_clone = client_arc.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };
    let spec = wav_spec_from_config(&audio_config);
    println!("Audio spec == {:?}", spec);

    let stream = match audio_config.sample_format() {
        cpal::SampleFormat::I8 => audio_device.build_input_stream(
            &audio_config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, spec),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => audio_device.build_input_stream(
            &audio_config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, spec),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => audio_device.build_input_stream(
            &audio_config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, spec),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => audio_device.build_input_stream(
            &audio_config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, spec),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    stream.play()?;

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            let mut buffer = DATA_BUFFER.lock().expect("Failed to lock buffer");
            if !buffer.is_empty() {
                do_audio_fft(buffer.clone(), client_arc_clone.clone());
                buffer.clear();
            }
        }
    });

    select! {
        _ = tokio::signal::ctrl_c() => {
            drop(stream);
            client_arc.clone().try_lock().unwrap().close().await?;
        }
    }
    Ok(())
}

#[derive(Deserialize)]
struct Config {
    start_freq_bass: i16,
    end_freq_bass: i16,
    start_freq_other: i16,
    end_freq_other: i16,
    bass_default_max_intensity: f32,
    other_default_max_intensity: f32,
    bass_intensity_percent: i32,
    other_intensity_percent: i32,
}

fn load_config() -> Config {
    let config_str = fs::read_to_string("config/config.json")
        .expect("Failed to read config.json");
    serde_json::from_str(&config_str).expect("Failed to parse config.json")
}


fn do_audio_fft(wav_data: Vec<u8>, client_arc_clone: Arc<Mutex<TrueGearClient>>) {

    let config = load_config();

    let mut true_gear_msg_vec: Vec<TrackObject> = Vec::new();
    let spectrum = _get_fft(wav_data);
    let freq_vec: Vec<_> = spectrum.keys().copied().collect();
    if freq_vec.is_empty() {
        return; // pas de data audio
    }

    let max_freq = freq_vec.iter().max().unwrap();
    let freq_interval = *max_freq as f32 / 5.0;
    let start_freq_bass = config.start_freq_bass;
    let end_freq_bass = config.end_freq_bass;
    let start_freq_other = config.start_freq_other;
    let end_freq_other = config.end_freq_other;
//    let end_freq = freq_interval as i16;

    // Calcul de l’intensité sur la bande basse
    let mut bass_intensity = 0.0;
    for (freq, &value) in spectrum.iter() {
        if *freq >= start_freq_bass && *freq < end_freq_bass {
            bass_intensity += value;
        }
    }
    let mut other_intensity = 0.0;
    for (freq, &value) in spectrum.iter() {
        if *freq >= start_freq_other && *freq < end_freq_other {
            other_intensity += value;
        }
    }

    // Normalisation et seuil
    let bass_default_max_intensity = config.bass_default_max_intensity; 
    bass_intensity = bass_intensity / 5.0;
    let bass_intensity_percent = (bass_intensity / bass_default_max_intensity * 100.0) as i32;
    println!("DEBUG Bass Intensity: {:?}", bass_intensity_percent);
    let other_default_max_intensity = config.other_default_max_intensity;
    other_intensity = other_intensity / 5.0;
    let other_intensity_percent = (other_intensity / other_default_max_intensity * 100.0) as i32;
    println!("DEBUG Other Intensity: {:?}", other_intensity_percent);

    if bass_intensity_percent > config.bass_intensity_percent {
        let mut all_vec = crate::true_gear::TRUE_GEAR_SHAKE_MIDDLE_FRONT.to_vec();
        all_vec.append(&mut crate::true_gear::TRUE_GEAR_SHAKE_MIDDLE_BACK.to_vec());

        let track = TrackObject::new_shake_duration(
            Some(60),               
            Some(bass_intensity_percent-20), 
            Some(bass_intensity_percent-20), 
            None,
            all_vec,                 
        );
        true_gear_msg_vec.push(track);
    }
    if other_intensity_percent > config.other_intensity_percent {
        
        let mut all_vec = crate::true_gear::TRUE_GEAR_SHAKE_AROUND_FRONT.to_vec();
        all_vec.append(&mut crate::true_gear::TRUE_GEAR_SHAKE_AROUND_BACK.to_vec());

        let track = TrackObject::new_shake_duration(
            Some(60),               
            Some(other_intensity_percent-20), 
            Some(other_intensity_percent-20), 
            None,
            all_vec,                 
        );
        true_gear_msg_vec.push(track);
    }

    
    tokio::spawn(async move {
        let client_arc_clone = client_arc_clone.clone();
        let true_gear_msg_vec = true_gear_msg_vec.clone();
        tokio::task::spawn_blocking(move || {
            let mut client = client_arc_clone.lock().unwrap();
            tokio::runtime::Handle::current().block_on(async {
                if !true_gear_msg_vec.is_empty() {
                    client.send_shake(true_gear_msg_vec).await.unwrap();
                }
            });
        })
        .await
        .unwrap();
    });
}

fn _get_fft(data: Vec<u8>) -> HashMap<i16, f32> {
    let mut reader = hound::WavReader::new(Cursor::new(data)).expect("Failed to read wav");
    let spec = reader.spec();
    let samples: Vec<f32> = match spec.sample_format {
        SampleFormat::Int => match spec.bits_per_sample {
            8 => reader
                .samples::<i8>()
                .map(|s| s.unwrap() as f32 / i8::MAX as f32)
                .collect(),
            16 => reader
                .samples::<i16>()
                .map(|s| s.unwrap() as f32 / i16::MAX as f32)
                .collect(),
            32 => reader
                .samples::<i32>()
                .map(|s| s.unwrap() as f32 / i32::MAX as f32)
                .collect(),
            _ => panic!("Unsupported bit depth"),
        },
        SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
    };

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut input: Vec<Complex<f32>> = samples
        .iter()
        .map(|&x| Complex { re: x, im: 0.0 })
        .collect();
    fft.process(&mut input);
    let spectrum: Vec<_> = input.iter().map(|c| c.norm()).collect();
    let mut result = HashMap::new();
    for (i, &value) in spectrum.iter().enumerate() {
        let freq = (i as f32 * spec.sample_rate as f32 / samples.len() as f32) as i16;
        result.insert(freq, value);
    }
    result
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> WavSpec {
    WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: SampleFormat::Int,
    }
}

fn write_input_data<T, U>(input: &[T], spec: WavSpec)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if !input.is_empty() {
        let mut buff = Cursor::new(Vec::new());
        let mut writer = hound::WavWriter::new(&mut buff, spec);
        for &sample in input.iter() {
            writer.write_sample(U::from_sample(sample)).unwrap();
        }
        writer.finalize().unwrap();
        let data = buff.into_inner();
        let mut buffer = DATA_BUFFER.lock().unwrap();
        // insert data to buffer, if DATA_BUFFER not empty, split header
        if !buffer.is_empty() {
            buffer.extend_from_slice(&data[44..]);
        } else {
            buffer.extend_from_slice(&data);
        }
    }
}
