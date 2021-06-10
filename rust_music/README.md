# Rust Music
#### A readme specific to the Rust aspects of my music programs, ideally, this will overtake the Python code

## musiclib

### `pub fn pitchcalc`

This function, like its Python counterpart, takes a `&str` input and outputs an `f32` frequency value on the range A1 - Ab1.

##### Example use:
`let root: &str = "G#";`

`let root_freq: f32 = musiclib::pitchcalc(root);`

`println!("{}",rootfreq);`

would return `103.826225`


### `pub fn keysig`

This function takes the root frequency value generated from the previous function `root_freq` along with a number of keys `nkeys:usize`, and a mode choice `form:&str`, and will return a `Vec<f32>` with each notes frequency in ascending order in the mode selected. 

The mode (`form` in function call) must be a lowercase string slice, and can be:
"maj" - Major, "harmin" - Harmonic Minor, "natmin" - Natural Minor, "pent" - Major Pentatonic, "minpent" - Minor Pentatonic, "blues" - Blues, and anything else will default to a chromatic scale, so accuracy will be the friend of your ears.

##### Example use:

`let notes = musiclib::keysig(root_freq,8,"maj")`

`println!("{:?}",notes);`

would return `[51.913113, 58.270504, 65.40643, 69.29571, 77.78181, 87.30714, 97.998955, 103.82628]`
