extern crate musiclib;
extern crate periodicsynth;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Sample, SampleFormat};
use periodicsynth::{sin, synth};

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no outputty device");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config!")
        .with_max_sample_rate();
    let config = supported_config.into();
    let err_fn = |err| eprintln!("an error occurred: {}", err);
    let root: f32 = musiclib::pitchcalc("Eb");
    let samp: Vec<f64> = synth(sin, &mut 440f64, 8000);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| samp,
        err_fn,
    );
    stream.play().unwrap();

    // println!("{:?}", samp);
    let notes: Vec<f32> = musiclib::keysig(root, 8, "harmin");
}
