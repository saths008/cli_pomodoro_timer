use config::Config;
mod config;
mod pomodoro_timer_state;
mod sound;

pub fn run() {
    let pomodoro_config: Config = config::get_pomodoro_config();
    pomodoro_timer_state::pomodoro_timer(pomodoro_config);
}
