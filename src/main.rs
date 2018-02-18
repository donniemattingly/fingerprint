extern crate apodize;
extern crate hound;
extern crate image;
extern crate rustfft;

extern crate log4rs;
#[macro_use]
extern crate log;

pub mod spectrogram;
pub mod hash;

fn main() {
    // println!("Hello, world!");
    log4rs::init_file("log.yaml", Default::default()).unwrap();
    // gen_spectrogram("samples/test.wav", "output/mod.png");
    hash::generate_fingerprints("samples/test.wav");
}

fn gen_spectrogram<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let spec = spectrogram::from_wav(wav);
    spec.draw(image);
    info!("{}", spec);
}
