pub fn pitchcalc(note: &str) -> f32 {
    // Define base of A1 and frequency multiplier definition of half step
    let base_pitch: f32 = 55.;
    let frac: f32 = 1./12.;
    let half_step = f32::powf(2.,frac);

    match note {
        "A" => {
            println!("Fuck");
            return base_pitch;
        }

        "A#" | "Bb" => {
            return base_pitch*half_step;
        }

        "B" | "Cb" => {
            return base_pitch*f32::powf(half_step,2.);
        }

        "C" | "B#" => {
            return base_pitch*f32::powf(half_step,3.);
        }

        "C#" | "Db" => {
            return base_pitch*f32::powf(half_step,4.);
        }

        "D" => {
            return base_pitch*f32::powf(half_step,5.);
        }

        "D#" | "Eb" => {
            return base_pitch*f32::powf(half_step,6.);
        }

        "E" | "Fb" => {
            return base_pitch*f32::powf(half_step,7.);
        }

        "F" | "E#" => {
            return base_pitch*f32::powf(half_step,8.);
        }

        "F#" | "Gb" => {
            return base_pitch*f32::powf(half_step,9.);
        }

        "G" => {
            return base_pitch*f32::powf(half_step,10.);
        }

        "G#" | "Ab" => {
            return base_pitch*f32::powf(half_step,11.);
        }

        _ => {
            println!("Please enter a valid note, see docs for proper format");
            return 0.;
        }
    }
}
