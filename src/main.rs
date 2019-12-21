#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

mod app; 

use app::{rocket};

// main
fn main() {
    rocket().launch();
}
