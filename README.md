# Steam dev launcher

Simple stupid wrapper that lets you run your own executable instead of the
regular one. Which is useful when you're developing a game, and want to test
builds from your machine instead of what's currently in your game's Steam
folder.

It tries hard not to panic and logs as much as possible to `launcher.log`,
`stdout.log` and `stderr.log` in the current working directory, which will be
your Steam game's directory when launched through Steam.

## Features

- logs stdout and stderr to files in the working directory
- logs the commands line args your game was launched with
- set environment variables for your game
- launch a custom executable instead of the one deployed through Steam. Useful
  if you want to run a debug build, but test with Steam invites etc.

## Usage

After installing the app, paste something like this into "Launch Options" in
your game's "Properties" through the steam UI:

e.g.:

```txt
C:\Users\Johan\.cargo\bin\steam_dev_launcher.exe -- %command%
```

Run a debug build of a Bevy, app and set the env var so it finds the assets:

``txt
C:\Users\Johan\.cargo\bin\steam_dev_launcher.exe --env BEVY_ASSET_ROOT=C:/dev/cargo_space/ --custom-exe C:/dev/cargo_space/target/debug/cargo_space.exe -- %command%
```

On Windows, you will see a launcher window with some debug logging for the
launcher. Your game's logs will be in the Steam folder for your game.
