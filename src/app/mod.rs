use rocket::{Rocket,get,routes};
use rocket_contrib::{database};
use rocket_contrib::databases::diesel as pgd;

// db wrapper type
#[database("paperside_api_db")]
pub struct PapersideApiDbConn(pgd::PgConnection);

// index route
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


// mount routes from controllers
pub fn rocket() -> Rocket {
    rocket::ignite()
        .attach(PapersideApiDbConn::fairing())
        .mount("/", routes![index])
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
