extern crate rand;
extern crate structopt;

mod password;
use password::generate_password;

use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "rt", about = "developer tools in Rust")]
enum Command {
    #[structopt(about = "generate a random password")]
    Password {
        #[structopt(short, long, default_value = "12")]
        len: usize,
    },
}

fn main() {
    let command = Command::from_args();

    match command {
        Command::Password { len } => {
            let password = generate_password(len);
            println!("{}", password);
        }
    }
}