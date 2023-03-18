#[macro_use]
extern crate log;
extern crate simplelog;

use clap::Parser;
use simplelog::*;
use std::{
    error::Error,
    fs::File,
    process::{Command, Stdio},
};

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'e', long = "env", value_parser = parse_key_val::<String, String>)]
    env: Vec<(String, String)>,

    // /// Set a custom working directory (pwd) for the spawned child process
    // ///
    // ///  if unset, we will use the working directory of the replaced game
    // /// executable (i.e. it will run in your game's Steam directory)
    // #[clap(long)]
    // working_directory: Option<String>,
    #[clap(long, short = 'c')]
    custom_exe: Option<String>,

    #[arg(last = true)]
    steam_command: Vec<String>,
}

/// Should simple and stupid, and avoid panics as much as possible, as those
/// will not be visible anywhere.
///
/// Steam swallows std-out everything should be communicated through files
/// in the working directory, when launched through Steam, this means the game's
/// directory.
fn main() {
    let launcher_log_file = File::create("launcher.log")
        // todo: can we somehow report this more nicely?
        .expect("failed to open launcher log file for writing!");

    CombinedLogger::init(vec![
        // on Windows, we get a terminal window, useful to see some output in it
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

    // first lets just log the args, for sanity's sake
    info!(
        "Steam dev launcher: {:#?}",
        std::env::args().collect::<Vec<String>>()
    );

    let Some(this_executable) = std::env::args().next() else {
        // I think this can never happen, but let's just be thorough
        error!("no arguments");
        return;
    };

    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            error!("Failed to parse launcher args: {err}");
            return;
        }
    };

    info!("Parsed launcher args: {args:#?}");

    let Args {
        env,
        // working_directory,
        custom_exe,
        steam_command,
    } = args;

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

    let mut steam_args = steam_command.iter();

    // consume original game executable
    let Some(original_executable) = steam_args.next() else {
        error!("Received no arguments from Steam, this means your Steam
        configuration is wrong. You probably want to set \"Launch options\" to
        \"{this_executable} -- %command%\"");
        return;
    };

    let executable = if let Some(custom) = custom_exe.as_ref() {
        info!("Launching {custom:?} instead of {original_executable:?}");
        custom
    } else {
        original_executable
    };

    let mut command = Command::new(executable);
    // note with original_executable skipped
    command.args(steam_args);
    command.envs(env);

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
