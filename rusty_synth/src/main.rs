extern crate fundsp;

fn main() {
    let freq: f64 = 55.;
    let phase: f64 = 0.;
    let amp = |x: f64, y: u32| -> f64 { x + f64::from(y) };
    println!("{}", amp(1., 2));
}
