# TrueGear Suit Audio Driver

This is a fork of the Audio Driver for the TrueGear Suit done by ...

It's already working nicely to transcribe audio to vibration but I wanted to do some slight modification to it to make it better. So here the reason of this fork.

Too, It come with a config if you want to modify some settings in it like sensitivity !

# Config
## *Let's start with the start/end_freq parameters*
*Basically, theses are a range for the Frequency you want the suit to react !*

- **start_freq_bass** : *At how much for the bass frequency in Hz it should start to react for the suit to vibrate ?*
- **end_freq_bass** : *At how much for the bass frequency in Hz it should end to react for the suit to vibrate ?*

- **start_freq_other** : *At how much for the bass frequency in Hz it should start to react for the suit to vibrate ?*
- **end_freq_other** : *At how much for the bass frequency in Hz it should end to react for the suit to vibrate ?*

*As for now, There is only two frequency range to modify. I might add more in the future to separate like high or low mids or some other frequency if people want !*

## *Now here the sensitivity settings !*

- **bass_default_max_intensity** : *This number will dictate the sensitivity of the bass frequency, the lower it is, the more it will vibrate to this frequency ! (Do note that the suit will react "dynamically" (way more bass = way more vibration) to the value)*
- **other_default_max_intensity** : *Same as up here, but for all of the other frequency.*


- **bass_intensity_percent** : *This is another sensitivity settings for the bass frequency that will this time dictate the treshold that it need to reach before sending a vibration to the suit ! This is to avoid making the suit always vibrate*
- **other_intensity_percent** : *Same but, for all of the other frequency.*

## *Here The Pattern Settings !*
*As a notice, for now, they don't work, I will try to make an update soon to make it work !*

- **pattern_bass** : *This will dictate the pattern of the vibration of the suit for the bass ! To know all of the avaliable pattern, look down there !*
- **pattern_other** : *Same, but for all of the other frequency.*


## *Patterns*
*As said up there, it's not working yet ! But here the future list of pattern that will be implemented !*

### *Normal Pattern*

- **all** : *Will make the full suit vibrate !*
- **front** : *Will only make the front of the suit vibrate !*
- **back** : *Will only make the back of the suit vibrate !*
- **none** : *Will not make the suit vibrate !*

### Other Patterns !

### *Middle*

- **middle_all** :
- **middle_front** :
- **midle_back** :

### *Around*

- **around_all** :
- **around_front** :
- **around_back** :

### *Left*

- **left_all** :
- **left_front** :
- **left_back** :

### *Right*

- **right_all** :
- **right_front** :
- **right_back** :

### *Up*

- **up_all** :
- **up_front** :
- **up_back** :

### *Down*

- **down_all** :
- **down_front** :
- **down_back** :
