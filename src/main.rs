use gilrs::{Event, Gilrs};
use std::{process::Command, time::Duration};
use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        // Examine new events
        while let Some(Event {
            id: _id,
            event,
            time: _time,
        }) = gilrs.next_event()
        {
            match event {
                gilrs::EventType::ButtonPressed(_button, code) => {
                    // Parse the button number
                    // On the Mac where I'm writing this, these code strings look like "BUTTON(n)"
                    let button_number: u32 = code
                        .to_string()
                        .replace("BUTTON", "")
                        .replace("(", "")
                        .replace(")", "")
                        .parse()
                        .unwrap_or_default();

                    println!("button pressed: {}", button_number);

                    match button_number {
                        // Big red button on VKB Gladiator Evo
                        3 => focus_timed(45),
                        // Second trigger (full pull)
                        2 => focus_timed(60),
                        // A3 hat left
                        9 => focus_timed(25),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Pause a bit to avoid eating CPU
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn focus_timed(minutes: u32) {
    // The focus:// command will start the app if it's not running, but
    // it won't actually focus, so in that case it needs to be run twice.
    let mut cmd = Command::new("open");
    cmd.arg(format!("focus://focus?minutes={}", minutes));

    if is_focus_app_running() {
        println!("Beginning focus for {} minutes", minutes);
        cmd.spawn().expect("open failed");
    } else {
        println!("Focus app is not running, starting up...");
        cmd.spawn().expect("open failed");

        // give it time to start, then issue the command again
        std::thread::sleep(Duration::from_secs(2));
        println!("Beginning focus for {} minutes", minutes);
        cmd.spawn().expect("open failed");
    }
}

fn is_focus_app_running() -> bool {
    let mut sys = sysinfo::System::new();
    sys.refresh_processes();

    sys.processes()
        .values()
        .any(|process| process.name() == "Focus")
}
