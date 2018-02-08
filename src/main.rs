extern crate hound;
extern crate image;
extern crate goertzel;

extern crate rustfft;
use rustfft::FFTplanner;
use rustfft::num_complex::Complex32;
use std::thread;

#[macro_use]
extern crate apodize;
use apodize::{hanning_iter, hamming_iter, nuttall_iter};


#[macro_use]
extern crate nalgebra;
use nalgebra::core::{MatrixMN, Dynamic};
use nalgebra::core::coordinates::XY;

fn main() {
    // println!("Hello, world!");
    draw("440.wav", "image.png");
}

type MatrixDD = MatrixMN<f32, Dynamic, Dynamic>;

const minIntensity: f32 = 0.50;

fn draw<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let mut reader = hound::WavReader::open(wav).unwrap();
    let sample_rate = reader.spec().sample_rate;
    println!("Sample rate: {}", sample_rate);
    let channels = reader.spec().channels as usize;
    let mut samples: Vec<i16> = Vec::new();
    for (i, sample) in reader.samples().enumerate() {
        if i % channels == 0 {
            samples.push(sample.unwrap());
        }
    }

    let size_pow = 10; // larger than 4
    let chunk_size = 2_usize.pow(size_pow as u32);
    let img_size = chunk_size / 2;
    let overlap_size = 2_usize.pow(size_pow as u32 - 3) + 2_usize.pow(size_pow as u32 - 4);
    let eff_chunk_size = chunk_size - overlap_size;
    let num_chunks = (samples.len() as f64 / eff_chunk_size as f64).trunc() as usize;

    println!("power of 2: {}, chunk_size: {}, img_size: {}, overlap_size: {}, num_chunks: {}", 
             size_pow, chunk_size, img_size, overlap_size, num_chunks);

    // println!("{} samples", samples.len());
    // println!("{} s of audio", samples.len() as u32 / sample_rate);

    let inverse = false;
    let mut planner = FFTplanner::new(inverse);
    let fft = planner.plan_fft(chunk_size);
    let mut image_data: Vec<u8> = vec![];
    let mut image_lines = 0;
    let mut intensity_data: Vec<f32> = vec![];
    let mut intensity_cols: Vec<Vec<f32>> = vec![];

    for i in 0..num_chunks - 1 {
        let offset = i * eff_chunk_size;
        let chunk = &samples[offset..offset + chunk_size];
        let fft_copy = fft.clone();
        let mut signal: Vec<Complex32> = chunk.iter().map(|&x| x as f32)
            .zip(nuttall_iter(chunk_size).map(|x| x as f32))
            .map(move |(x, win)| Complex32::new(x * win, 0.0)).collect();

        // let mut signal: Vec<Complex32> = chunk.iter()
        //     .map(move |&x| Complex32::new(x as f32, 0.0)).collect();
        let mut spectrum = vec![Complex32::new(0.0, 0.0); chunk_size];
        fft_copy.process(&mut signal, &mut spectrum);
        // image_data.push((spectrum/1000.) as u8);
        // image_lines += 1;
        let half_index = spectrum.len() / 2;
        let mut new_spec = spectrum.split_off(half_index);
        let sample_max = 2.0_f32.powf(f32::from(reader.spec().bits_per_sample - 1));

        let mut intensity_col: Vec<f32> = vec![];

        for (i, val) in new_spec.iter().enumerate() {
            let abs_sq = (val.re.powf(2.0) + val.im.powf(2.0)) * 2.0 / f32::from(chunk_size as u16);
            let divd = abs_sq / sample_max;
            let pow = 20.0 * divd.log10();
            let freq = (half_index - i) * sample_rate as usize / chunk_size;
            let i_data: u8;
            let normalized: f32;
            if pow < -120.0 {
                normalized = 0.0;
            } else if pow > 135.0 {
                normalized = 1.0;
            } else{
                normalized = (pow + 120.0) / 255.0;
            }

            let quantized = (normalized * 256.0) as u8;
            let intermediate = (pow + 120.0).round() as u8;
            i_data = 255 - quantized;
            image_data.push(i_data);
            intensity_data.push(normalized);
            intensity_col.push(normalized);

            // println!("{}, {}", i_data, freq);
        }
        image_lines += 1;
        intensity_cols.push(intensity_col);
    }

    get_peaks(intensity_cols);
    
    println!("Saving image sized {} x {} (needs buffer of {})", img_size, image_lines, img_size * image_lines);
    println!("With a buffer of len {}", image_data.len());
    match image::save_buffer(image, &image_data[..], img_size as u32, image_lines as u32, image::ColorType::Gray(8)) {
        Ok(v) => println!("Success"),
        Err(e) => println!("{}", e),
    }
}

struct Coord(i32, i32);

fn get_peaks(intensity: Vec<Vec<f32>>) -> Vec<Coord> {

    let w = intensity.len();
    let h = &intensity[0].len();
    
    for i in 0..w {
        for j in 0..*h {
            let val = &intensity[i][j];
        }
    }

    vec![Coord(0, 0)]
}
