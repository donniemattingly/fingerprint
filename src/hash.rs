use std::path::Path;
use std::fmt::{self, Display, Formatter};
use std::cmp::Ordering;

use std::collections::HashSet;
extern crate rand;

use spectrogram::*;

extern crate sha1;

const DEFAULT_FAN_VALUE: usize = 15;
const DEFAULT_MIN_INTENSITY: f32 = 0.6;

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
    let intensity_threshold = DEFAULT_MIN_INTENSITY;
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
            freq: coord.1 as f32 * frequency_step,
            offset: coord.0 as f32 * time_step,
        })
        .collect();

    peaks
}

fn hash_peaks(mut peaks: Vec<Peak>) -> Vec<PeakHash> {
    // Order peaks
    peaks.sort();
    let peaks_len = peaks.len();
    let fan_value = DEFAULT_FAN_VALUE;
    let mut hashes: Vec<PeakHash> = Vec::new();

    // hash each constellation pair
    for (i, peak) in peaks.iter().enumerate() {
        let p1 = &peaks[i];
        for j in 1..fan_value {
            match i + j {
                k if k < peaks.len() => hashes.push(hash_peak_pair(&p1, &peaks[i + j])),
                _ => (),
            }
        }
    }

    hashes
}

pub struct PeakHash {
    pub hash_value: String,
    pub hash_string: String,
    pub offset: f32,
}

fn hash_peak_pair(p1: &Peak, p2: &Peak) -> PeakHash {
    let y = rand::random::<f64>();

    let hash_string = format!("{}|{}|{}", p1.freq, p2.freq, p2.offset - p1.offset);
    // if (y > 0.9999) {
    //     info!(
    //         "f1: {} o1: {} --- f2: {} o2: {}",
    //         p1.freq, p1.offset, p2.freq, p2.offset
    //     );
    // }
    let sha1 = sha1::Sha1::from(hash_string);
    let hash_value = sha1.hexdigest();
    PeakHash {
        hash_value: hash_value,
        hash_string: format!(
            "f1: {} f2: {} o: {}",
            p1.freq,
            p2.freq,
            p2.offset - p1.offset
        ),
        offset: p1.offset,
    }
}

pub fn generate_fingerprints<P: AsRef<Path>>(wav: P) -> Vec<PeakHash> {
    let specgram = from_wav(wav);
    info!("{}", specgram);
    let peaks = get_peaks(specgram);
    let hashes = hash_peaks(peaks);

    // let mut offset_set = HashSet::new();
    // for hash in &hashes {
    //     offset_set.insert(format!("{}", hash.offset));
    // }

    // for offset in &offset_set {
    //     info!("{}", offset);
    // }

    hashes
}
