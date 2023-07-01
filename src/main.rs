use std::process::Command;
use std::io::{self, Write};

fn main() {
    // get output volume
    let o_volume = String::from_utf8(
        Command::new("pactl")
            .arg("get-sink-volume")
            .arg("@DEFAULT_SINK@")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split("/")
    .nth(1)
    .unwrap()
    .trim()
    .to_string();

    // check whether output is muted
    let o_mute = String::from_utf8(
        Command::new("pactl")
            .arg("get-sink-mute")
            .arg("@DEFAULT_SINK@")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap().strip_prefix("Mute: ").unwrap().trim() == "yes";

    // let m_volume = String::from_utf8(
    //     Command::new("pactl")
    //         .arg("get-source-volume")
    //         .arg("@DEFAULT_SOURCE@")
    //         .output()
    //         .unwrap()
    //         .stdout,
    // )
    // .unwrap()
    // .split("/")
    // .nth(1)
    // .unwrap()
    // .trim()
    // .to_string();

    // check if mic is muted
    let m_mute = String::from_utf8(
        Command::new("pactl")
            .arg("get-source-mute")
            .arg("@DEFAULT_SOURCE@")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap().strip_prefix("Mute: ").unwrap().trim() == "yes";

    // generate output

    print!("Vol: ");
    if o_mute {
        print!("X");
    }
    print!("{} MIC: ", o_volume);
    if m_mute {
        print!("off");
    } else {
        print!("on");
    }

    //flush it
    io::stdout().flush().unwrap();
}
