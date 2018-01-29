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
use nalgebra::core::DMatrix;

fn main() {
    // println!("Hello, world!");
    draw("440.wav", "image.png");
}


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

    let size_pow = 13; // larger than 4
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
    let mut freq_cols: Vec<Vec<f32>> = Vec::new();

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
        let mut freq_vec: Vec<f32> = Vec::new();

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

            freq_vec.push(normalized);
            let quantized = (normalized * 256.0) as u8;
            let intermediate = (pow + 120.0).round() as u8;
            i_data = 255 - quantized;
            image_data.push(i_data);

            // println!("{}, {}", i_data, freq);
        }
        freq_cols.push(freq_vec);
        image_lines += 1;
    }
    

    // for chunk in samples.chunks(chunk_size).filter(|x| x.len() == chunk_size) {
    //     let fft_copy = fft.clone();
    //     let mut signal: Vec<Complex32> = chunk.iter().map(move |&x| Complex32::new(x as f32, 0.0)).collect();
    //     let mut spectrum = vec![Complex32::new(0.0, 0.0); chunk_size];
    //     fft_copy.process(&mut signal, &mut spectrum);
    //     // image_data.push((spectrum/1000.) as u8);
    //     // image_lines += 1;
    //     let half_index = spectrum.len() / 2;
    //     let mut new_spec = spectrum.split_off(half_index);
    //     let sample_max = 2.0_f32.powf(f32::from(reader.spec().bits_per_sample - 1));
    //     for (i, val) in new_spec.iter().enumerate() {
    //         let abs_sq = (val.re.powf(2.0) + val.im.powf(2.0)) * 2.0 / f32::from(chunk_size as u16);
    //         let divd = abs_sq / sample_max;
    //         let pow = 20.0 * divd.log10();
    //         let freq = (half_index - i) * sample_rate as usize / chunk_size;
    //         let i_data: u8;
    //         if pow < -120.0 {
    //             i_data = 0;
    //         } else if pow > 135.0 {
    //             i_data = 255;
    //         } else{
    //             let intermediate = (pow + 120.0).round() as u8;
    //             i_data = 255 - intermediate;
    //         }
    //         image_data.push(i_data);
    //         // println!("{}, {}", i_data, freq);
    //     }
    //     image_lines += 1;
    // }

    println!("Saving image sized {} x {} (needs buffer of {})", img_size, image_lines, img_size * image_lines);
    println!("With a buffer of len {}", image_data.len());
    match image::save_buffer(image, &image_data[..], img_size as u32, image_lines as u32, image::ColorType::Gray(8)) {
        Ok(v) => println!("Success"),
        Err(e) => println!("{}", e),
    }

    // let threads: Vec<thread::JoinHandle<>> = chunks.map(|chunk| {
    //     let fft_copy = fft.clone();
    //     thread::spawn(move || {
    //         let mut spectrum = vec![Complex32::new(0.0, 0.0); chunk_size];
    //         fft_copy.process(&mut chunk, &mut spectrum);
    //         spectrum
    //     })
    // }).collect();

    // for thread in threads {
    //     thread.join().unwrap();
    // }
    // let mut output: Vec<Complex32> = vec![Complex32::new(0.0, 0.0); input.len()];
    // let mut planner = FFTplanner::new(false);
    // let fft = planner.plan_fft(input.len());
    // fft.process(&mut input, &mut output);

    // let mut image_data: Vec<u8> = vec![];
    // let mut image_lines = 0;
    // let chunk_size = 2048;
    // let bin_size = 120;
    // for chunk in samples.chunks(chunk_size).filter(|x| x.len() == chunk_size) {
    //     for bin in (0 .. bin_size).map(|x| (x * chunk_size) as f32) {
    //         let p = goertzel::Parameters::new(bin, sample_rate, chunk_size);
    //         let v = p.start().add(chunk).finish_mag();
    //        
    //         // print!("{:14.0} ", v);
    //     }
    //     // println!("");
    //     image_lines+=1;
    // }
    // println!("{}x{} image", bin_size, image_lines);
}
