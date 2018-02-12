extern crate hound;
extern crate image;

extern crate rustfft;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex32;
use std::fmt::{self, Formatter, Display};

#[macro_use]
extern crate apodize;
use apodize::{hanning_iter, hamming_iter, nuttall_iter};

#[macro_use]
extern crate log;
extern crate log4rs;

pub mod spectrogram;

fn main() {
    // println!("Hello, world!");
    log4rs::init_file("log.yaml", Default::default()).unwrap();
    gen_spectrogram("samples/440.wav", "output/mod.png");
}

const minIntensity: f32 = 0.4;

fn gen_spectrogram<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let spec = spectrogram::from_wav(wav);
    spec.draw(image);
}

struct Coord(i32, i32);

struct Peak{
    coord: Coord,
    freq: f32,
    offset: f32,
}

impl Display for Coord{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

fn get_peaks(intensity: Vec<Vec<f32>>, 
             intensity_threshold: f32, 
             freqBins: Vec<f32>, 
             offsets: Vec<f32>) -> Vec<Coord> {

    let w = intensity.len();
    let h = &intensity[0].len();

    let mut peaks: Vec<Coord> = Vec::new();
    
    for i in 1..w-1 {
        for j in 1..*h-1 {
            let val = &intensity[i][j];
            match val{
                val if val < &intensity[i-1][j+1] => (),
                val if val < &intensity[i-1][j-1] => (),
                val if val < &intensity[i+1][j-1] => (),
                val if val < &intensity[i+1][j+1] => (),
                val if val < &intensity[i][j+1] => (),
                val if val < &intensity[i-1][j] => (),
                val if val < &intensity[i][j-1] => (),
                val if val < &intensity[i+1][j] => (),
                val if val < &intensity_threshold => (),
                val => peaks.push(Coord(i as i32, j as i32)),
            }
        }
    }
    
    let possible = w * h;
    let num_peaks = peaks.len();
    
    let percent_peaks = num_peaks as f32 / possible as f32;

    println!("From {} possible", w * h);
    println!("Had {} peaks", peaks.len());
    println!("Ratio of {}", percent_peaks);

    peaks
}


fn generate_fingerprints(peaks: Vec<Peak>, ){
}
