mod entities;
mod handlers;
mod query_root;
mod state;

use server::init;

fn main() {
    init().unwrap()
}
