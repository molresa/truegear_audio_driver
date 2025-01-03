use cpal::traits::{DeviceTrait, HostTrait};

pub fn init() -> anyhow::Result<cpal::Device> {
    let host = cpal::default_host();
    let device = host.default_output_device();
    if device.is_none() {
        return Err(anyhow::anyhow!("No output device available"));
    }
    let device = device.unwrap();
    println!("Default audio device == {:?}", device.name()?);
    Ok(device)
}
