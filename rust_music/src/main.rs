extern crate musiclib;

fn main() {
    let mut root: &str = "G#";
    let root_freq: f32 = musiclib::pitchcalc(root);

    println!("{}: {}Hz",root,root_freq);
}
