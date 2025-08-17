# TrueGear Suit Audio Driver

This is a fork of the Audio Driver for the TrueGear Suit done by xkeyC

It's already working nicely to transcribe audio to vibration but I wanted to do some slight modification to it to make it better. So here the reason of this fork.

Too, It come with a config if you want to modify some settings in it like sensitivity !

# Config
## *Other Settings*
*Will Work in V0.3 !*

- **update_time** : *How often the audio data should be analyzed in ms*
- **vibration_time** : *How much time you want the vibration to persist in ms*

## *Let's start with the start/end_freq parameters*
*Basically, theses are a range for the Frequency you want the suit to react !*

- **start_freq_bass** : *At how much for the bass frequency in Hz it should start to react for the suit to vibrate ?*
- **end_freq_bass** : *At how much for the bass frequency in Hz it should end to react for the suit to vibrate ?*

- **start_freq_treble** : *At how much for the treble frequency in Hz it should start to react for the suit to vibrate ?*
- **end_freq_treble** : *At how much for the treble frequency in Hz it should end to react for the suit to vibrate ?*

- **start_freq_other** : *At how much for the other frequency in Hz it should start to react for the suit to vibrate ?*
- **end_freq_other** : *At how much for the other frequency in Hz it should end to react for the suit to vibrate ?*

*As for now, There is only three frequency range to modify. I might add more in the future to separate some other frequency if people want !*

## *Now here the sensitivity settings !*

- **bass_default_max_intensity** : *This number will dictate the sensitivity of the bass frequency, the lower it is, the more it will vibrate to this frequency ! (Do note that the suit will react "dynamically" (way more bass = way more vibration) to the value)*
- **treble_default_max_intensity** : *Same as up here, but for all of the treble frequency.*
- **other_default_max_intensity** : *Same as up here, but for all of the other frequency.*


- **bass_intensity_percent** : *This is another sensitivity settings for the bass frequency that will this time dictate the treshold that it need to reach before sending a vibration to the suit ! This is to avoid making the suit always vibrate*
- **treble_intensity_percent** : *Same but, for all of the treble frequency.*
- **other_intensity_percent** : *Same but, for all of the other frequency.*


- **bass_intensity_max_percent** : *This number will be the maximum strenght send to the suit If the max intensity percentage is reached ! (As an exemple, can be useful if you have an high threshold but only need to send small vibration to the suit)*
- **treble_intensity_max_percent** : *Same but, for all of the treble frequency.*
- **other_intensity_max_percent** : *Same but, for all of the other frequency.*

## *Here The Pattern Settings !*
*Do work in the V0.2 ! But, Small notice, Left/Right pattern work only in V0.3 !*

- **pattern_bass** : *This will dictate the pattern of the vibration of the suit for the bass ! To know all of the avaliable pattern, look down there !*
- **pattern_other** : *Same, but for all of the other frequency.*
- **pattern_treble** : *Same, but for all of the treble frequency.*


## *Patterns*

### *Normal Pattern*

- **all** : *Will make the full suit vibrate*
- **front** : *Will only make the front of the suit vibrate*
- **back** : *Will only make the back of the suit vibrate*
- **none** : *Will not make the suit vibrate*

### Other Patterns !

### *Cross*
*Only Work in V0.3*
- **cross_left** : *Will make the left front side and the right back side of the suit vibrate !*
- **cross_right** : *Will make the right front side and the left back side of the suit vibrate !*

### *Middle*

- **middle_all** : *Will only make the middle of the suit vibrate*
- **middle_front** : *Same, but only for the front of the suit*
- **midle_back** : *Same, but only for the back of the suit*

### *Around*

- **around_all** : *Will only make the around/corner of the suit vibrate*
- **around_front** : *Same, but only for the front of the suit*
- **around_back** : *Same, but only for the back of the suit*

### *Left*

- **left_all** : *Will only make the left side of the suit vibrate*
- **left_front** : *Same, but only for the front of the suit*
- **left_back** : *Same, but only for the back of the suit*

### *Right*

- **right_all** : *Will only make the right side of the suit vibrate*
- **right_front** : *Same, but only for the front of the suit*
- **right_back** : *Same, but only for the back of the suit*

### *Up*

- **up_all** : *Will only make the up part of the suit vibrate*
- **up_front** : *Same, but only for the front of the suit*
- **up_back** : *Same, but only for the back of the suit*

### *Down*

- **down_all** : *Will only make the down part of the suit vibrate*
- **down_front** : *Same, but only for the front of the suit*
- **down_back** : *Same, but only for the back of the suit*


MIT License

Copyright (c) 2025 xkeyC (xkeyc@qq.com)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
