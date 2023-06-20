use crate::config::Config;
use crate::sound::play_sound;
use std::io::{self, Write};
use std::thread;
use std::time;

pub fn pomodoro_timer(pomodoro_config: Config) {
    let work_session_duration: u32 = pomodoro_config.get_work_session_duration();
    let break_session_duration: u32 = pomodoro_config.get_break_session_duration();
    let number_of_pomodoro_iterations: u32 = pomodoro_config.get_number_of_pomodoro_iterations();

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
        println!();
        println!(
            "Pomodoro iteration {} / {} complete!",
            i, number_of_pomodoro_iterations
        );
    }

    println!("Good work! All pomodoro sessions finished!");
}

pub fn loading_bar(duration: std::time::Duration) {
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
