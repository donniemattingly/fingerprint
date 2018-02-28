# Fingerprint

Fingerprint is a rust library for fingerprinting audio. My approach is broadly based on this
[article](http://willdrevo.com/fingerprinting-and-audio-recognition-with-python/), but this 
project doesn't implement any of the audio recognition. 

This library is designed to be used by mobile devices, but that is currently a work in progress.


You can generate a spectrogram
```rust
    let spec = spectrogram::from_wav(wav);
    spec.draw(image);
```

And generate a list of hashes for some audio
```rust
    let hashes = hash::generate_fingerprints_from_wav("some-audio.wav");
```