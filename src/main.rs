use std::{
    env,
    fs::File,
    process::{Command, Stdio},
};

fn main() {
    let stdout = Stdio::from(File::create("stdout.txt").unwrap());
    let stderr = Stdio::from(File::create("stderr.txt").unwrap());

    let mut args = env::args();
    let _ = args.next(); // ignore first, (this is us)
    let command = args.next().unwrap();

    let status = dbg!(Command::new(command).args(args))
        .stdout(stdout)
        .stderr(stderr)
        .status()
        .expect("io error while running command");

    std::process::exit(status.code().unwrap());
}
