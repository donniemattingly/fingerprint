use std::path::Path;

extern crate hound;
extern crate image;

extern crate threadpool;
use self::threadpool::ThreadPool;

extern crate rustfft;
use rustfft::FFTplanner;
use rustfft::FFT;
use rustfft::num_complex::Complex32;
use std::fmt::{self, Display, Formatter};

use std::sync::Arc;
use std::sync::Mutex;

extern crate apodize;
use apodize::nuttall_iter;

extern crate stopwatch;
use self::stopwatch::Stopwatch;

const DEFAULT_SIZE_POW: u32 = 10;

pub struct Spectrogram {
    pub data: Vec<Vec<f32>>,
    pub chunk_bits: u32,
    pub frequency_step: f32,
    pub time_step: f32,
}

impl Display for Spectrogram {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Spectrogram [{} s] {{chunk_bits: {}, frequency_step: {}, time_step: {}}}",
            self.data.len() as f32 * self.time_step,
            self.chunk_bits,
            self.frequency_step,
            self.time_step
        )
    }
}

impl Spectrogram {
    pub fn new(
        data: Vec<Vec<f32>>,
        chunk_bits: u32,
        frequency_step: f32,
        time_step: f32,
    ) -> Spectrogram {
        Spectrogram {
            data: data,
            chunk_bits: chunk_bits,
            frequency_step: frequency_step,
            time_step: time_step,
        }
    }

    pub fn draw<P: AsRef<Path>>(&self, image_path: P) {
        let mut sw = Stopwatch::start_new();
        info!("Drawing spectrogram");
        let image_data: Vec<u8> = self.data
            .iter()
            .flat_map(|cols| {
                cols.iter()
                    .map(|val| (val * <u8>::max_value() as f32) as u8)
            })
            .collect();

        debug!("Drawing spectrogram with {} bytes", image_data.len());

        match image::save_buffer(
            image_path,
            &image_data[..],
            self.data[0].len() as u32,
            self.data.len() as u32,
            image::ColorType::Gray(8),
        ) {
            Ok(_) => info!("Saved image successfully"),
            Err(e) => error!("{}", e),
        };

        debug!("Drawing spectrogram took {}ms", sw.elapsed_ms());
    }
}

fn process_chunk(chunk: &[i16], fft: Arc<FFT<f32>>, sample_max: f32) -> Vec<f32> {
    let fft_copy = fft.clone();
    let chunk_size = chunk.len();

    // Apply apodization and convert to a complex number
    let mut signal: Vec<Complex32> = chunk
        .iter()
        .map(|&x| x as f32)
        .zip(nuttall_iter(chunk_size).map(|x| x as f32))
        .map(move |(x, win)| Complex32::new(x * win, 0.0))
        .collect();

    // Create output buffer
    let mut spectrum = vec![Complex32::new(0.0, 0.0); chunk_size];

    // Perform FFT
    fft_copy.process(&mut signal, &mut spectrum);

    // Discard 1/2 since output is symmetrical
    let half_index = spectrum.len() / 2;
    let mut new_spec = spectrum.split_off(half_index);

    let mut intensity_col: Vec<f32> = vec![];

    // Clean up FFT output to be useful
    for val in new_spec.iter() {
        // Convert from complex
        let abs_sq = (val.re.powf(2.0) + val.im.powf(2.0)) * 2.0 / f32::from(chunk_size as u16);
        let divd = abs_sq / sample_max;

        // Convert to dB
        let pow = 20.0 * divd.log10();

        // Convert power to a float between 0 and 1
        let normalized: f32;
        if pow < -120.0 {
            normalized = 0.0;
        } else if pow > 135.0 {
            normalized = 1.0;
        } else {
            normalized = (pow + 120.0) / 255.0;
        }
        intensity_col.push(normalized);
    }

    intensity_col
}

pub fn from_wav<P: AsRef<Path>>(wav: P) -> Spectrogram {
    let mut sw = Stopwatch::start_new();
    let mut reader = hound::WavReader::open(wav).unwrap();
    let sample_rate = reader.spec().sample_rate;
    let channels = reader.spec().channels as usize;

    debug!(
        "Audio sampled at {}Hz with {} channels",
        sample_rate, channels
    );

    // Only take one channel
    let mut samples: Vec<i16> = Vec::new();
    for (i, sample) in reader.samples().enumerate() {
        if i % channels == 0 {
            samples.push(sample.unwrap());
        }
    }

    debug!("Reading samples to memory took {}ms", sw.elapsed_ms());
    sw.reset();

    sw.start();

    // Define FFT chunk parameters
    let size_pow: u32 = DEFAULT_SIZE_POW; // larger than 4
    let chunk_size = 2_usize.pow(size_pow);
    let overlap_size = 2_usize.pow(size_pow - 3) + 2_usize.pow(size_pow - 4);
    let eff_chunk_size = chunk_size - overlap_size;
    let num_chunks = (samples.len() as f64 / eff_chunk_size as f64).trunc() as usize;

    // Determine maximum possible signal value
    let sample_max = 2.0_f32.powf(f32::from(reader.spec().bits_per_sample - 1));

    debug!(
        "Configuration: power of 2: {}, chunk_size: {}, overlap_size: {}, num_chunks: {}",
        size_pow, chunk_size, overlap_size, num_chunks
    );

    // Setup FFT and output Buffers
    let inverse = false;
    let mut planner = FFTplanner::new(inverse);
    let fft = planner.plan_fft(chunk_size as usize);
    // let mut intensity_cols: Vec<Vec<f32>> = vec![];
    // let mut handles = vec![];

    let mut raw: Vec<Vec<f32>> = vec![Vec::new(); num_chunks - 1 as usize];
    let mut output: Mutex<Vec<Vec<f32>>> = Mutex::new(raw);
    let output_ref = Arc::new(output);
    let samples_ref = Arc::new(samples);

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);

    for i in 0..num_chunks - 1 {
        let offset = i * eff_chunk_size;

        // Break the input into num_chunks chunks
        let samples_copy = Arc::clone(&samples_ref);
        let output_clone = Arc::clone(&output_ref);
        let fft_copy = Arc::clone(&fft);

        pool.execute(move || {
            let chunk = &samples_copy[offset..offset + chunk_size];
            let col = process_chunk(chunk, fft_copy, sample_max);
            let mut cols = output_clone.lock().unwrap();
            cols[i] = col;
        });
    }

    pool.join();

    debug!("Generating spectrogram took {}ms", sw.elapsed_ms());
    sw.reset();

    let lock = Arc::try_unwrap(output_ref).expect("Lock still has multiple owners");
    let intensity_cols = lock.into_inner().expect("Mutex cannot be locked");

    let frequency_step = sample_rate as f32 / chunk_size as f32;
    let time_step = eff_chunk_size as f32 / sample_rate as f32;

    debug!(
        "Produced spectrogram {} x {} with freq step: {} and time step: {}",
        intensity_cols.len(),
        intensity_cols[0].len(),
        frequency_step,
        time_step
    );

    Spectrogram {
        data: intensity_cols,
        chunk_bits: size_pow,
        frequency_step,
        time_step,
    }
}
