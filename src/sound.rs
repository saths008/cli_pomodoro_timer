use dotenv::dotenv;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
pub fn play_sound() {
    dotenv().ok();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let sound_file_location =
        std::env::var("SOUND_FILE_LOCATION").expect("SOUND_FILE_LOCATION must be set");
    let file = BufReader::new(File::open(sound_file_location).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle
        .play_raw(source.convert_samples())
        .expect("audio play error");

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
