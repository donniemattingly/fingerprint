extern crate apodize;
extern crate hound;
extern crate image;
extern crate rustfft;
extern crate stats;

extern crate stopwatch;
use self::stopwatch::Stopwatch;

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
    let mut sw = Stopwatch::start_new();
    // test_fingerprinting();
    debug!("Everything took {}ms", sw.elapsed_ms());

    // rust_get_hashes();
    // gen_spectrogram("samples/bl.wav", "output/spec.png");
}

fn gen_spectrogram<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let spec = spectrogram::from_wav(wav);
    spec.draw(image);
    info!("{}", spec);
}

fn test_fingerprinting() {
    // let mut peaks_map = HashMap::new();
    let mut multi_peaks_map = MultiMap::new();
    let sample_hashes = hash::generate_fingerprints_from_wav("samples/duke-unc-first-5.wav");
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

    let test2_hashes =
        hash::generate_fingerprints_from_wav("samples/duke-unc-first-5_4-7_bandpass.wav");

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
            if (cur_count > 10) {
                diff_counts.push((cur, cur_count));
            }
            cur = diff;
            cur_count = 0;
        } else {
            cur_count += 1;
        }
    }

    diff_counts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // let mut diff_sum: f32 = 0.0;
    // let mut tot_count: u32 = 0;

    for dc in &diff_counts {
        if dc.1 > 50 {
            info!("{} - {}", dc.0, dc.1);
        }
        // diff_sum += dc.0 * dc.1 as f32 * dc.1 as f32 * dc.1 as f32;
        // tot_count += dc.1 * dc.1 * dc.1;
    }

    // let diff_avg: f32 = diff_sum / tot_count as f32;
    // info!("Estimated time_offset by math: {}", diff_avg);
    info!(
        "Estimated time_offset by high count: {} ({} hash matches)",
        &diff_counts[0].0, &diff_counts[0].1
    )

    //   info!("Had {} matches for test2", offsets2.len());
    //     for (o1, o2, hash) in offsets2 {
    //         // info!("{}", o1 - o2);
    //     }
}
