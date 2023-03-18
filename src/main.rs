#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::{
    env,
    fs::File,
    process::{Command, Stdio},
};

/// should be as simple and stupid as possible, so it never panics, no matter
/// what arguments you throw at it
///
/// Steam swallows std-out everything should be communicated through files
/// in the working directory, on Steam, this means the games directory.
fn main() {
    let launcher_log_file = File::create("launcher.log")
        // todo: can we somehow report this more nicely?
        .expect("failed to open launcher log file for writing!");

    CombinedLogger::init(vec![
        // on windows, we get a terminal window, useful to see some output in it
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), launcher_log_file),
    ])
    // todo: how can we tell the user that logging init failed?
    .unwrap();

    let mut args = env::args();

    // first lets just log the args, for sanity's sake
    info!("Steam dev launcher: {args:?}");

    let stdout = match File::create("stdout.log") {
        Ok(f) => Stdio::from(f),
        Err(err) => {
            error!("Failed to open stdout log file for writing: {err}");
            return;
        }
    };

    let stderr = match File::create("stderr.log") {
        Ok(f) => Stdio::from(f),
        Err(err) => {
            error!("Failed to open stderr log file for writing: {err}");
            return;
        }
    };

    let Some(this_executable) = args.next() else {
        // I think this can never happen, but let's just be thorough
        error!("no arguments");
        return;
    };

    let Some(replacement_game_executable) = args.next() else {
        // I think this can never happen, but let's just be thorough
        error!("Missing replacement executable. This means your Steam
        configuration is wrong. You probably want to set \"Launch options\" to:
        \"{this_executable} /full/path/to/dev/build %command%\"");
        return;
    };

    // skip original game executable
    let Some(replaced_game_executable) = args.next() else {
        error!("Received no arguments from Steam, this means your Steam
        configuration is wrong. You probably want to set \"Launch options\" to
        \"{this_executable} /full/path/to/dev/build %command%\"");
        return;
    };

    info!("Launching {replacement_game_executable:?} instead of {replaced_game_executable:?}");

    let mut command = Command::new(replacement_game_executable);
    command.args(args);

    info!("Launching game: {command:#?}");

    let status = match command.stdout(stdout).stderr(stderr).status() {
        Ok(status) => status,
        Err(err) => {
            error!("io error while running game executable: {err}");
            // todo: return some specific error code here?
            return;
        }
    };

    match status.code() {
        Some(code) => {
            if code != 0 {
                error!("Exited with error status code: {code}");
            } else {
                info!("Exited with status 0 (no error)");
            }
            // forwarding status code
            std::process::exit(code);
        }
        None => {
            warn!("Game terminated by signal");
            // todo: return some specific error code here?
        }
    }
}
