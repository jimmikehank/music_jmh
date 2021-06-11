extern crate musiclib;
extern crate portaudio;
extern crate periodicsynth;

use periodicsynth::{synth,sin};
use portaudio as pa;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
    let root = musiclib::pitchcalc("Eb");
    let samp = synth(sin, &mut 440f64, 8000);
    println!("{:?}",samp);
    // let notes: Vec<f32> = musiclib::keysig(root,8,"harmin");
    // println!("{}", notes);
}
