pub fn pitchcalc(note: &str) -> f32 {
    // Define base of A1 and frequency multiplier definition of half step
    let base_pitch: f32 = 27.5;
    let frac: f32 = 1./12.;
    let half_step = f32::powf(2.,frac);

    match note {
        "A" => {
            println!("Fuck");
            return base_pitch*2.;
        }

        "A#" | "Bb" => {
            return base_pitch*f32::powf(half_step,13.);
        }

        "B" | "Cb" => {
            return base_pitch*f32::powf(half_step,14.);
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

pub fn keysig(root:f32, nkeys:usize, form:&str) -> Vec<f32> {

    fn keys_gen(root:f32,steps:Vec<usize>,nkeys:usize) -> Vec<f32> {
        let hs = f32::powf(2.,1./12.);
        let mut vec = Vec::new();
        vec.push(root);
        for e in 1..nkeys {
            let ind:usize = e-1;
            let index = (ind)%steps.len() as usize;
            let multiplier = f32::powf(hs,steps[index] as f32);
            vec.push(vec[ind]*multiplier);
        }
        return vec;
    }

    match form {
        "maj" => {
            let steps = vec![2,2,1,2,2,2,1];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }

        "harmin" => {
            let steps = vec![2,1,2,2,1,3,1];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }

        "natmin" => {
            let steps = vec![2,1,2,2,1,2,2];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }

        "pent" => {
            let steps = vec![2,2,3,2,3];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }

        "minpent" => {
            let steps = vec![3,2,2,3,2];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }

        blues => {
            let steps = vec![3,2,1,1,3,2];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }
        _ => {
            let steps = vec![1,1,1,1,1,1,1,1,1,1,1,1];
            let vec = keys_gen(root,steps,nkeys);
            return vec;
        }
    }
}
