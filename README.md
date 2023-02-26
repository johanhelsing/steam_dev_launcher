# logz plz

Minimal stupid wrapper that just prints stdout and stderr to files.

I made it because steam swallows stdout and stderr by default, so I had no way
to I know how to see why my game was panicking.

I know how to redirect stdout and stdeerr on linux, but on windows I have no
idea, so I did it in rust.

## Usage

```
logz_plz mygame.exe some args here
```

spits out stderr.txt and stdout.txt in the working directory
