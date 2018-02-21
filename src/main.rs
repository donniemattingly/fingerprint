extern crate apodize;
extern crate hound;
extern crate image;
extern crate rustfft;
extern crate stats;

extern crate log4rs;
#[macro_use]
extern crate log;

use std::collections::HashMap;

extern crate multimap;

use multimap::MultiMap;

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
    // let mut peaks_map = HashMap::new();
    let mut multi_peaks_map = MultiMap::new();
    let sample_hashes = hash::generate_fingerprints("samples/test.wav");
    let num_hashes = sample_hashes.len();
    let mut dups = 0;
    for peak in sample_hashes {
        multi_peaks_map.insert(peak.hash_value, peak.offset);
    }

    info!("Had {} dup hashes out of {} total hashes", dups, num_hashes);
    // info!("Analyzing {} unique hashes", peaks_map.len());

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
    let mut diffs: Vec<f32> = Vec::new();
    for peak in test2_hashes {
        match multi_peaks_map.get_vec(&peak.hash_value) {
            Some(offset_list) => for offset in (*offset_list).iter() {
                diffs.push(*offset - peak.offset);
            },
            None => (),
        }
    }

    diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut diff_counts: Vec<(f32, u32)> = Vec::new();
    let mut cur: f32 = 0.0;
    let mut cur_count: u32 = 0;
    for diff in diffs {
        if (diff != cur) {
            if (cur_count > 3) {
                diff_counts.push((cur, cur_count));
            }
            cur = diff;
            cur_count = 0;
        } else {
            cur_count += 1;
        }
    }

    diff_counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for dc in diff_counts {
        info!("{} - {}", dc.0, dc.1);
    }

    // info!("Had {} matches for test2", offsets2.len());
    // for (o1, o2, hash) in offsets2 {
    //     // info!("{}", o1 - o2);
    // }
}
