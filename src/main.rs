#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;

use dotenv::dotenv;

mod app; 
mod schema;

use app::{rocket};

// main
fn main() {
    dotenv().ok();
    rocket().launch();
}
