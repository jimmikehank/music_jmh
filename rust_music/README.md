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
