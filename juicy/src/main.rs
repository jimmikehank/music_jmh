use rodio::{source::FromIter, source::Source, Decoder, OutputStream, Sink};
use std::f32::consts::PI;
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let wavedef_1 = WaveformDef {
        freq: 110.,
        amp: 0.2,
        phase: 3.,
        dur: 4.,
        fs: 48000,
    };

    let wave1 = wavedef_1.generate_wave();
    let buf_size: usize = 2048;
    let first_samp: f32 = 0.;
    println!("{}", wave1.len());
    let mut buf = Buffer {
        buffer: wave1,
        buffer_size: buf_size,
        current_index: 0,
        current_sample: first_samp,
        channels: 1,
        fs: 48000,
    };

    let sink = Sink::try_new(&stream_handle).unwrap();

    // sink.append(buf);
    // sink.sleep_until_end();
}

struct Buffer {
    buffer: Vec<f32>,
    buffer_size: usize,
    current_index: usize,
    current_sample: f32,
    channels: u16,
    fs: u32,
}

impl Iterator for Buffer {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.current_index += 1;
        self.current_sample = self.buffer[self.current_index];
        Some(self.current_sample)
    }
}

impl Source for Buffer {
    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }
    fn channels(&self) -> u16 {
        return self.channels;
    }
    fn sample_rate(&self) -> u32 {
        return self.fs;
    }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_nanos(0))
    }
}

impl Buffer {
    fn write_sample_to_buffer(&mut self, sample: f32) {
        self.buffer.push(sample);
    }
}

struct WaveformDef {
    freq: f32,
    amp: f32,
    phase: f32,
    dur: f32,
    fs: u32,
}

impl WaveformDef {
    fn generate_wave(&self) -> Vec<f32> {
        let n: u32 = self.fs * self.dur as u32;
        let mut out: Vec<f32> = vec![0.; n as usize];
        let mut t: f32 = 0.;
        for i in 0..n {
            t = i as f32 / self.fs as f32;
            let theta: f32 = 2. * PI * self.freq * t + self.phase * PI / 180.;
            out[i as usize] = self.amp * theta.sin();
        }
        return out;
    }
}
