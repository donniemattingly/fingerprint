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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
