use std::path::Path;
use std::fmt::{self, Display, Formatter};
use std::cmp::Ordering;

use spectrogram::*;

struct Coord(i32, i32);

struct Peak {
    coord: Coord,
    freq: f32,
    offset: f32,
}

impl Ord for Peak {
    fn cmp(&self, other: &Peak) -> Ordering {
        self.coord.0.cmp(&other.coord.0)
    }
}

impl PartialOrd for Peak {
    fn partial_cmp(&self, other: &Peak) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Peak {
    fn eq(&self, other: &Peak) -> bool {
        self.coord.0 == other.coord.0
    }
}

impl Eq for Peak {}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

fn get_peaks(spectrogram: Spectrogram) -> Vec<Peak> {
    let intensity_threshold = 0.4;
    let intensity = spectrogram.data;
    let w = intensity.len();
    let h = &intensity[0].len();

    let mut peak_coords: Vec<Coord> = Vec::new();

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
                _ => peak_coords.push(Coord(i as i32, j as i32)),
            }
        }
    }

    let possible = w * h;
    let num_peaks = peak_coords.len();

    let percent_peaks = num_peaks as f32 / possible as f32;

    debug!("From {} possible", w * h);
    debug!("Had {} peaks", peak_coords.len());
    debug!("Ratio of {}", percent_peaks);

    let frequency_step = spectrogram.frequency_step;
    let time_step = spectrogram.time_step;

    let peaks: Vec<Peak> = peak_coords
        .iter()
        .map(move |coord| Peak {
            coord: Coord(coord.0, coord.1),
            freq: coord.0 as f32 * frequency_step,
            offset: coord.1 as f32 * time_step,
        })
        .collect();

    peaks
}

fn hash_peaks(mut peaks: Vec<Peak>) {
    let sorted = peaks.sort();
    // Order peaks
    // Detect constellations
    // hash each constellation pair

    // vec![""]
}

fn hash_peak_pair(p1: Peak, p2: Peak) {}

pub fn generate_fingerprints<P: AsRef<Path>>(wav: P) {
    let specgram = from_wav(wav);
    let peaks = get_peaks(specgram);
    let hashes = hash_peaks(peaks);
}
