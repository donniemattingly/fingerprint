extern crate hound;
extern crate image;

extern crate rustfft;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex32;

#[macro_use]
extern crate apodize;
use apodize::{hanning_iter, hamming_iter, nuttall_iter};

#[macro_use]
extern crate log;
extern crate log4rs;

pub mod spectrogram;
pub mod hash;

fn main() {
    // println!("Hello, world!");
    log4rs::init_file("log.yaml", Default::default()).unwrap();
    // gen_spectrogram("samples/test.wav", "output/mod.png");
    hash::generate_fingerprints("samples/440.wav");
}

const minIntensity: f32 = 0.4;

fn gen_spectrogram<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let spec = spectrogram::from_wav(wav);
    spec.draw(image);
    info!("{}", spec);
}
