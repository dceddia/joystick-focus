# Joystick Focus

This is a small program that listens for joystick inputs and then kicks off the [Focus](https://heyfocus.com) app into focus mode.

## To Use

This requires a working Rust environment.

```sh
cargo build --release
cp target/release/joystick-focus /usr/local/bin/joystick-focus
```

## Run on Startup

After the program is installed at `/usr/local/bin/joystick-focus`, copy the included .plist file into `~/Library/LaunchAgents` and run `launchctl load ~/Library/LaunchAgents/co.tinywins.joystick-focus.plist`

