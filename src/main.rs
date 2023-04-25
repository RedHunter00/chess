#![allow(dead_code)]
mod board;
mod core;
mod cli;
use log::LevelFilter;
use fern::Dispatch;
use std::io;
use log::Level;
use colored::*;
use std::thread;

fn setup_logger(log_level: LevelFilter) -> Result<(), fern::InitError> {
    let console_logger = Dispatch::new()
        .format(|out, message, record| {
            let mut level = format!("{}", record.level()).white();

            match record.level() {
                Level::Error => level = level.truecolor(240, 76, 76).bold(),
                Level::Warn => level = level.truecolor(245, 245, 67).bold(),
                Level::Debug => level = level.truecolor(79, 193, 255).bold(),
                Level::Info => level = level.truecolor(35, 209, 139).bold(),
                _ => (),
            };

            if record.level() == Level::Debug || record.level() == Level::Trace {
                out.finish(format_args!(
                    "{} [{}][{}][{}][{}] {}",
                    level,
                    chrono::Local::now().format("%H:%M:%S"),
                    thread::current().name().unwrap_or("<undefined>"),
                    record.file().unwrap_or("<undefined>"),
                    record.line().unwrap_or(0),
                    message
                ))
            } else {
                out.finish(format_args!("{} {}", level, message))
            }
        })
        .level(LevelFilter::Off)
        .level_for("chess", log_level)
        .chain(io::stdout());
    //or stderr

    let logger = Dispatch::new().chain(console_logger);

    match logger.apply() {
        Ok(()) => Ok(()),
        Err(e) => Err(fern::InitError::SetLoggerError(e)),
    }
}

fn main() {
    setup_logger(LevelFilter::Trace).unwrap();
    cli::start_game();
}