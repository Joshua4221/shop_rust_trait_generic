mod authenticator;
mod company;
mod controller;
mod operation;
mod store;
mod system_handler;
mod users;
mod utils;

use controller::controller;

fn main() {
    if let Err(err) = controller() {
        eprintln!("Application error: {}", err);
    }
}
