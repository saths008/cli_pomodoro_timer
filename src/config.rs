use std::io;
use std::process;
#[derive(Debug)]
pub struct Config {
    work_session_duration: u32,
    break_session_duration: u32,
    number_of_pomodoro_iterations: u32,
}

impl Config {
    pub fn build(
        work_session_duration: &str,
        break_session_duration: &str,
        number_of_pomodoro_iterations: &str,
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

    pub fn get_work_session_duration(&self) -> u32 {
        self.work_session_duration
    }

    pub fn get_break_session_duration(&self) -> u32 {
        self.break_session_duration
    }

    pub fn get_number_of_pomodoro_iterations(&self) -> u32 {
        self.number_of_pomodoro_iterations
    }

    pub fn print_out_config(&self) {
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

pub fn get_pomodoro_config() -> Config {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_config_passes_with_valid_inputs() {
        let config = Config::build("25", "5", "4").unwrap();
        assert_eq!(config.work_session_duration, 25);
        assert_eq!(config.break_session_duration, 5);
        assert_eq!(config.number_of_pomodoro_iterations, 4);
    }

    #[test]
    fn get_work_session_duration_is_correct() {
        let config = Config::build("25", "5", "4").unwrap();
        assert_eq!(config.get_work_session_duration(), 25);
    }
    #[test]
    fn get_break_session_duration_is_correct() {
        let config = Config::build("25", "5", "4").unwrap();
        assert_eq!(config.get_break_session_duration(), 5);
    }
    #[test]
    fn get_number_of_pomodoro_iterations_is_correct() {
        let config = Config::build("25", "5", "4").unwrap();
        assert_eq!(config.get_number_of_pomodoro_iterations(), 4);
    }

    #[test]
    fn build_config_fails_with_invalid_work_session_duration() {
        let config = Config::build("fail", "5", "4");
        assert_eq!(config.is_err(), true);
        match config {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, "Invalid work session duration"),
        }
    }

    #[test]
    fn build_config_fails_with_invalid_break_session_duration() {
        let config = Config::build("25", "fail", "4");
        assert_eq!(config.is_err(), true);
        match config {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, "Invalid break session duration"),
        }
    }

    #[test]
    fn build_config_fails_with_invalid_number_of_pomodoro_iterations() {
        let config = Config::build("25", "5", "fail");
        assert_eq!(config.is_err(), true);
        match config {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err, "Invalid number of pomodoro iterations"),
        }
    }
}
