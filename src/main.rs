use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::{self, BufReader};

fn play_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("beep-01a.mp3").unwrap());
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

// Refactor to include a struct to manage all of this
fn get_pomodoro_config() -> (u32, u32, u32) {
    println!("How long would you like each work session to be? (in minutes)");

    let mut work_session_duration: String = String::new();

    io::stdin()
        .read_line(&mut work_session_duration)
        .expect("Failed to read line");

    let work_session_duration: u32 = match work_session_duration.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("How long would you like each break session to be? (in minutes)");

    let mut break_session_duration: String = String::new();

    io::stdin()
        .read_line(&mut break_session_duration)
        .expect("Failed to read line");

    let break_session_duration: u32 = match break_session_duration.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("How many iterations of the pomodoro technique would you like to do?");

    let mut number_of_pomodoro_iterations: String = String::new();

    io::stdin()
        .read_line(&mut number_of_pomodoro_iterations)
        .expect("Failed to read line");

    let number_of_pomodoro_iterations: u32 = match number_of_pomodoro_iterations.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    println!("Work session duration: {}", work_session_duration);
    println!("Break session duration: {}", break_session_duration);
    println!(
        "Number of pomodoro iterations: {}",
        number_of_pomodoro_iterations
    );

    (
        work_session_duration,
        break_session_duration,
        number_of_pomodoro_iterations,
    )
}

fn main() {
    let (work_session_duration, break_session_duration, number_of_pomodoro_iterations) =
        get_pomodoro_config();

    for i in 1..=number_of_pomodoro_iterations {
        println!(
            "Starting work session {} / {}",
            i, number_of_pomodoro_iterations
        );
        let work_session_duration_in_seconds: u64 = (work_session_duration * 60) as u64;
        std::thread::sleep(std::time::Duration::from_secs(
            work_session_duration_in_seconds,
        ));
        play_sound();
        println!(
            "Starting break session {} / {}",
            i, number_of_pomodoro_iterations
        );
        let break_session_duration_in_seconds: u64 = (break_session_duration * 60) as u64;
        std::thread::sleep(std::time::Duration::from_secs(
            break_session_duration_in_seconds,
        ));
        play_sound();
    }
}
