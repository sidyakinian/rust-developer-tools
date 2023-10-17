extern crate rand;
extern crate structopt;

mod password;
use password::generate_password;

mod screenshare;
use screenshare::{hide_files};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rt", about = "developer tools in Rust")]
enum Command {
    #[structopt(about = "generate a random password")]
    Password {
        #[structopt(short, long, default_value = "12")]
        len: usize,
    },
    #[structopt(about = "hide all files from desktop into a documents folder")]
    Hide,
    // #[structopt(about = "put back desktop files from documents folder")]
    // Show,
}

fn main() {
    let command = Command::from_args();

    match command {
        Command::Password { len } => {
            let password = generate_password(len);
            println!("{}", password);
        }
        Command::Hide => {
            if let Err(e) = hide_files() {
                eprintln!("Error hiding files: {}", e);
            }
        }
    }
}