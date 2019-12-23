use bcrypt;
use chrono::{Utc};
use diesel::prelude::*;
use rocket::{post};
use rocket::response::status::{BadRequest};
use rocket_contrib::json::Json;
use serde::{Deserialize,Serialize};

use crate::app::{PapersideApiDbConn};
use crate::app::models::*;

use crate::schema;

const BCRYPT_COST: u32 = 11;

/// Params for new user registration
#[derive(Deserialize, Serialize)]
pub struct NewRegistrationRequest {
    pub user_name: String,
    pub one_hashed: String, // pre-hashed password from client
}

// login (creates new session)
#[post("/register", format = "application/json", data = "<attr>")]
pub fn registrations_register(conn: PapersideApiDbConn, attr: Json<NewRegistrationRequest>) -> Result<Json<String>,BadRequest<String>> {
    // check if username is taken
    let existing_user_count: i64 = User::by_name(&attr.user_name)
        .count()
        .get_result(&conn.0).expect("Error fetching existing User count");
    if existing_user_count > 0 {
        // registration failed
        return Err(BadRequest(Some("Username taken".into())))
    }

    // creates new user record
    let new_user = NewUser {
        name: (&attr.user_name).into(),
        doublehashed: bcrypt::hash(&attr.one_hashed, BCRYPT_COST).expect("bcrypt error"),
        created_at: Utc::now(),
    };
    let _created_user: User = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .get_result(&conn.0)
        .map_err(|err| BadRequest(Some(err.to_string())))?;

    // returns success
    Ok(Json("Registration success".into()))
}

#[cfg(test)]
mod test {
    use rocket::http::{ContentType,Status};
    use crate::{json_string};
    use crate::app::test::{test_client};
    use crate::app::controllers::sessions_controller::test::{help_login,help_logout};

    #[test]
    fn test_register() {
        // test params
        let user_name = format!("Test User {:?}", chrono::Utc::now());
        let hashed_password = "abcdef0123456789";

        // post to register endpoint
        let response = test_client().post("/register")
            .body(json_string!({
                "user_name": user_name,
                "one_hashed": hashed_password,
            }))
            .header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::Ok);

        // test login to new account
        let r_2 = help_login(&user_name, hashed_password);
        assert_eq!(r_2.status(), Status::Ok);

        // ensure logout
        let r_3 = help_logout();
        assert_eq!(r_3.status(), Status::Ok);


        // attempt duplicate registration (result is BadRequest)
        let mut r_4 = test_client().post("/register")
            .body(json_string!({
                "user_name": user_name,
                "one_hashed": hashed_password,
            }))
            .header(ContentType::JSON).dispatch();
        assert_eq!(r_4.status(), Status::BadRequest);
        assert_eq!(r_4.body_string(), Some("Username taken".into()));
    }
}
