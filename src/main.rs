#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

mod app; 
mod schema;

use app::{rocket};

// main
fn main() {
    rocket().launch();
}
