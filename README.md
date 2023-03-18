# Steam dev launcher

Simple stupid wrapper that lets you run your own executable instead of the
regular one and doesn't eat the stdout and stderr logs.

It tries hard not to panic and logs as much as possible to `launcher.log`,
`stdout.log` and `stderr.log` in the current working directory, which will be
your Steam game's directory when launched through Steam.

## Features

- logs stdout and stderr to files in the working directory
- logs the commands line args your game was launched with
- set environment variables for your game
- launch a custom executable instead of the one deployed through Steam, but
  still forwards the command line args from steam. Useful if you want to run a
  debug build, but test with Steam invites etc.
- logs the exit code for your game, useful if it crashes when loading dynamic
  libraries etc.
- logs if the game was killed by a signal

## Usage

After installing the app, paste something like this into "Launch Options" in
your game's "Properties" through the steam UI:

e.g.:

```txt
C:\Users\Johan\.cargo\bin\steam_dev_launcher.exe -- %command%
```

Run a debug build of a Bevy, app and set the env var so it finds the assets:

```txt
C:\Users\Johan\.cargo\bin\steam_dev_launcher.exe --env BEVY_ASSET_ROOT=C:/dev/cargo_space/ --custom-exe C:/dev/cargo_space/target/debug/cargo_space.exe -- %command%
```

On Windows, you will see a launcher window with some debug logging for the
launcher. Your game's logs will be in the Steam folder for your game.

Example `launcher.log` after accepting a Steam lobby invite:

```txt
17:25:02 [INFO] Steam dev launcher: [
    "C:\\Users\\Johan\\.cargo\\bin\\steam_dev_launcher.exe",
    "-e",
    "BEVY_ASSET_ROOT=C:/dev/cargo_space/",
    "--custom-exe",
    "C:/dev/cargo_space/target/debug/cargo_space.exe",
    "--",
    "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Cargo Space\\cargo_space.exe",
    "--launcher",
    "+connect_lobby",
    "109775243842407186",
]
17:25:02 [INFO] Parsed launcher args: Args {
    env: [
        (
            "BEVY_ASSET_ROOT",
            "C:/dev/cargo_space/",
        ),
    ],
    custom_exe: Some(
        "C:/dev/cargo_space/target/debug/cargo_space.exe",
    ),
    steam_command: [
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Cargo Space\\cargo_space.exe",
        "--launcher",
        "+connect_lobby",
        "109775243842407186",
    ],
}
17:25:02 [INFO] Launching "C:/dev/cargo_space/target/debug/cargo_space.exe" instead of "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Cargo Space\\cargo_space.exe"
17:25:02 [INFO] Launching game: "C:/dev/cargo_space/target/debug/cargo_space.exe" "--launcher" "+connect_lobby" "109775243842407186"
17:25:06 [INFO] Exited with status 0 (no error)
```
