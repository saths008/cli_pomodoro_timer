use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::Write;
use std::io::{self, BufReader};
use std::process;
use std::thread;
use std::time;
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

#[derive(Debug)]
struct Config {
    work_session_duration: u32,
    break_session_duration: u32,
    number_of_pomodoro_iterations: u32,
}

impl Config {
    fn build(
        work_session_duration: &String,
        break_session_duration: &String,
        number_of_pomodoro_iterations: &String,
    ) -> Result<Config, &'static str> {
        let work_session_duration: u32 = match work_session_duration.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return Err("Invalid work session duration");
            }
        };

        let break_session_duration: u32 = match break_session_duration.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return Err("Invalid break session duration");
            }
        };

        let number_of_pomodoro_iterations: u32 = match number_of_pomodoro_iterations.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return Err("Invalid number of pomodoro iterations");
            }
        };

        Ok(Config {
            work_session_duration,
            break_session_duration,
            number_of_pomodoro_iterations,
        })
    }

    fn get_work_session_duration(&self) -> u32 {
        self.work_session_duration
    }

    fn get_break_session_duration(&self) -> u32 {
        self.break_session_duration
    }

    fn get_number_of_pomodoro_iterations(&self) -> u32 {
        self.number_of_pomodoro_iterations
    }

    fn print_out_config(&self) -> () {
        println!(
            "Work session duration: {} minutes.",
            self.get_work_session_duration()
        );
        println!(
            "Break session duration: {} minutes.",
            self.get_break_session_duration()
        );
        println!(
            "Number of pomodoro iterations: {}.",
            self.get_number_of_pomodoro_iterations()
        );
    }
}
// Refactor to include a struct to manage all of this
fn get_pomodoro_config() -> Config {
    println!("How long would you like each work session to be? (in minutes)");
    let mut work_session_duration: String = String::new();
    io::stdin()
        .read_line(&mut work_session_duration)
        .expect("Failed to read line");

    println!("How long would you like each break session to be? (in minutes)");

    let mut break_session_duration: String = String::new();

    io::stdin()
        .read_line(&mut break_session_duration)
        .expect("Failed to read line");

    println!("How many iterations of the pomodoro technique would you like to do?");

    let mut number_of_pomodoro_iterations: String = String::new();

    io::stdin()
        .read_line(&mut number_of_pomodoro_iterations)
        .expect("Failed to read line");

    let config = match Config::build(
        &work_session_duration,
        &break_session_duration,
        &number_of_pomodoro_iterations,
    ) {
        Ok(config) => config,
        Err(error) => {
            println!("Problem parsing arguments: {}", error);
            process::exit(1);
        }
    };

    config.print_out_config();
    config
}

fn loading_bar(duration: std::time::Duration) {
    // Create a new thread to run the timer
    let start_time = time::Instant::now();
    let _timer_thread = thread::spawn(move || {
        // Loop until the duration is reached
        while (time::Instant::now() - start_time) < duration {
            // Clear the current line
            print!("\r");

            // Calculate the progress percentage
            let elapsed = time::Instant::now() - start_time;
            let elapsed_as_seconds = elapsed.as_secs_f32();
            let duration_as_seconds = duration.as_secs_f32();
            let progress = (elapsed_as_seconds / duration_as_seconds) * 100.0;
            let elapsed_as_seconds = elapsed.as_secs_f32() as u32;
            let duration_as_seconds = duration.as_secs_f32() as u32;

            // Print the loading bar
            let bar_width = 40;
            let num_filled = (progress / 100.0 * (bar_width as f32)) as usize;
            let num_empty = bar_width - num_filled;
            print!(
                "[{}{}]   {} mins / {} mins",
                "=".repeat(num_filled),
                " ".repeat(num_empty),
                elapsed_as_seconds / 60,
                duration_as_seconds / 60
            );

            // Flush the output to make sure it's immediately displayed
            io::stdout().flush().unwrap();

            // Delay for a short interval (e.g., 100 milliseconds)
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    // Print a new line after the loading bar completes
    println!();
}

pub fn pomodoro_timer() {
    let pomodoro_config: Config = get_pomodoro_config();
    let work_session_duration: u32 = pomodoro_config.work_session_duration;
    let break_session_duration: u32 = pomodoro_config.break_session_duration;
    let number_of_pomodoro_iterations: u32 = pomodoro_config.number_of_pomodoro_iterations;

    for i in 1..=number_of_pomodoro_iterations {
        println!(
            "Starting work session {} / {}",
            i, number_of_pomodoro_iterations
        );
        let work_session_duration_in_seconds: u64 = (work_session_duration * 60) as u64;
        loading_bar(std::time::Duration::from_secs(
            work_session_duration_in_seconds,
        ));
        std::thread::sleep(std::time::Duration::from_secs(
            work_session_duration_in_seconds,
        ));
        println!();
        println!("Work session complete!");
        play_sound();
        println!(
            "Starting break session {} / {}",
            i, number_of_pomodoro_iterations
        );
        let break_session_duration_in_seconds: u64 = (break_session_duration * 60) as u64;
        loading_bar(std::time::Duration::from_secs(
            break_session_duration_in_seconds,
        ));
        std::thread::sleep(std::time::Duration::from_secs(
            break_session_duration_in_seconds,
        ));
        println!();
        println!("Break session complete!");
        play_sound();
    }
}
