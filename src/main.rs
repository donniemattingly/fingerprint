extern crate apodize;
extern crate hound;
extern crate image;
extern crate rustfft;
extern crate stats;

extern crate log4rs;
#[macro_use]
extern crate log;

use std::collections::HashMap;

pub mod spectrogram;
pub mod hash;

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();
    test_fingerprinting();
}

fn gen_spectrogram<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let spec = spectrogram::from_wav(wav);
    spec.draw(image);
    info!("{}", spec);
}

fn test_fingerprinting() {
    let mut peaks_map = HashMap::new();
    let sample_hashes = hash::generate_fingerprints("samples/test.wav");
    let num_hashes = sample_hashes.len();
    let mut dups = 0;
    for peak in sample_hashes {
        if (!peaks_map.contains_key(&peak.hash_value)) {
            peaks_map.insert(peak.hash_value, peak.offset);
        } else {
            dups += 1;
        }
    }

    info!("Had {} dup hashes out of {} total hashes", dups, num_hashes);
    info!("Analyzing {} unique hashes", peaks_map.len());

    // let test1_hashes = hash::generate_fingerprints("samples/test.wav");

    // let mut offsets: Vec<(f32, f32)> = Vec::new();
    // for peak in test1_hashes {
    //     match peaks_map.get(&peak.hash_value) {
    //         Some(offset) => offsets.push((*offset, peak.offset)),
    //         None => (),
    //     }
    // }

    let test2_hashes = hash::generate_fingerprints("samples/test_3s_1.wav");

    let mut offsets2: Vec<(f32, f32, String)> = Vec::new();
    for peak in test2_hashes {
        match peaks_map.get(&peak.hash_value) {
            Some(offset) => offsets2.push((*offset, peak.offset, peak.hash_string)),
            None => (),
        }
    }

    info!("Had {} matches for test2", offsets2.len());
    for (o1, o2, hash) in offsets2 {
        info!("{}", o1 - o2);
    }
}
