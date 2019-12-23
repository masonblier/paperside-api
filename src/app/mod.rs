use rocket::{Rocket,get,routes};
use rocket_contrib::{database};
use rocket_contrib::databases::diesel as pgd;

// local modules
pub mod config;
pub use config::*;

pub mod models;
pub use models::*;

pub mod controllers;
pub use controllers::*;

pub mod fairings;
pub use fairings::*;

// db wrapper type
#[database("paperside_api_db")]
pub struct PapersideApiDbConn(pgd::PgConnection);

// index route
#[get("/")]
fn index(su: SessionUser) -> String {
    match su.user {
        Some(user) => format!("Hello, {}!", user.name),
        None => "Hello, world!".into(),
    }
}


// mount routes from controllers
pub fn rocket() -> Rocket {
    rocket::ignite()
        .attach(PapersideApiDbConn::fairing())
        .mount("/", routes![index])
        .mount("/", routes![
            sessions_login, sessions_logout,
            registrations_register])
        .mount("/reference_items", routes![
            list_reference_items,create_reference_item,read_reference_item,
            update_reference_item,delete_reference_item])
        .attach(AppConfig::manage())
}


#[cfg(test)]
pub mod test {
    use serde_json::Value;
    use once_cell::sync::OnceCell;
    
    use rocket::local::{Client, LocalResponse};
    use rocket::http::Status;

    // shared accessor for request/response client
    pub fn test_client() -> &'static Client {
        static INSTANCE: OnceCell<Client> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let rocket = super::rocket();
            Client::new(rocket).expect("valid rocket instance")
        })
    }

    /// Utility macro for turning `json!` into string.
    #[macro_export]
    macro_rules! json_string {
        ($value:tt) => {
            serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
        };
    }

    // helper function to get json from response value
    pub fn response_json_value(response: &mut LocalResponse) -> Value {
        let body = response.body().expect("no body");
        serde_json::from_reader(body.into_inner()).expect("can't parse value")
    }

    #[test]
    fn hello_world() {
        let mut response = test_client().get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
