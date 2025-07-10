use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // params structure
    #[arg(short, long)]
    session: String,
}

fn main() {
    let args = Args::parse(); // params
    let session_file = args.session + ".conf"; // session file
    let output = Command::new("nohup")
        .arg("kitty")
        .arg("--session")
        .arg(session_file)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn() // detach the process
        .expect("Failed to execute kitty command");
    println!("{:?}", output);
}
