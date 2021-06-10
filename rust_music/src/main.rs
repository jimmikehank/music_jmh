extern crate musiclib;

fn main() {
    let mut root: &str = "G#";
    let vec = Vec::<u8>::new();
    let elem = 1u8;
    let root_freq: f32 = musiclib::pitchcalc(root);
    let notes = musiclib::keysig(root_freq,8,"maj");
    println!("{:?}",notes);
}
