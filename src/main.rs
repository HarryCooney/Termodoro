use color_eyre::Result;
mod clock;
mod animation;
mod app;
mod menu;
mod config;
mod pomodoro;
mod config_menu;
mod ui;
mod key_events;
mod icons;
mod new_config;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = app::App::new();
    ratatui::run(|terminal| app.run(terminal))
}
