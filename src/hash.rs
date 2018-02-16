use std::path::Path;
use std::fmt::{self, Display, Formatter};

use spectrogram::*;

struct Coord(i32, i32);

struct Peak {
    coords: Coord,
    freq: f32,
    offset: f32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

fn get_peaks(spectrogram: Spectrogram) -> Vec<Coord> {
    let intensity_threshold = 0.4;
    let intensity = spectrogram.data;
    let w = intensity.len();
    let h = &intensity[0].len();

    let mut peaks: Vec<Coord> = Vec::new();

    for i in 1..w - 1 {
        for j in 1..*h - 1 {
            let val = &intensity[i][j];
            match val {
                val if val < &intensity[i - 1][j + 1] => (),
                val if val < &intensity[i - 1][j - 1] => (),
                val if val < &intensity[i + 1][j - 1] => (),
                val if val < &intensity[i + 1][j + 1] => (),
                val if val < &intensity[i][j + 1] => (),
                val if val < &intensity[i - 1][j] => (),
                val if val < &intensity[i][j - 1] => (),
                val if val < &intensity[i + 1][j] => (),
                val if val < &intensity_threshold => (),
                _ => peaks.push(Coord(i as i32, j as i32)),
            }
        }
    }

    let possible = w * h;
    let num_peaks = peaks.len();

    let percent_peaks = num_peaks as f32 / possible as f32;

    debug!("From {} possible", w * h);
    debug!("Had {} peaks", peaks.len());
    debug!("Ratio of {}", percent_peaks);

    peaks
}

pub fn generate_fingerprints<P: AsRef<Path>>(wav: P) {
    let specgram = from_wav(wav);
    let peaks = get_peaks(specgram);
}
