use chrono::{Duration,Utc};
use diesel::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rocket::{State,post};
use rocket::http::{Cookie,Cookies};
use rocket::response::status::{BadRequest};
use rocket_contrib::json::Json;
use serde::{Deserialize,Serialize};

use crate::app::{PapersideApiDbConn};
use crate::app::config::AppConfig;
use crate::app::models::*;

use crate::schema;

const BCRYPT_COST: u32 = 11;
const SESSION_DAYS: i64 = 14;

/// Params for login to new session
#[derive(Deserialize, Serialize)]
pub struct NewSessionRequest {
    pub user_name: String,
    pub one_hashed: String, // pre-hashed password from client
}

// login (creates new session)
#[post("/login", format = "application/json", data = "<attr>")]
pub fn sessions_login(conn: PapersideApiDbConn, config: State<AppConfig>, mut cookies: Cookies, attr: Json<NewSessionRequest>) -> Result<Json<String>,BadRequest<String>> {
    let user_result: Result<User,_> = User::by_name(&attr.user_name)
            .first(&conn.0);

    if (&user_result).is_ok() {
        if bcrypt::verify(&attr.one_hashed, &user_result.as_ref().unwrap().doublehashed).expect("bcrypt error") {
            // Generate session token
            let rand_token: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .collect();

            // Insert new session row
            let new_session = NewSession {
                user_id: user_result.as_ref().unwrap().id,
                hashed_access_token: bcrypt::hash(&rand_token, BCRYPT_COST).expect("bcrypt error"),
                created_at: Utc::now(),
                last_accessed_at: Utc::now(),
                accessed_by_client_ip: None,
            };
            let created_session: Session = diesel::insert_into(schema::sessions::table)
                .values(&new_session)
                .get_result(&conn.0)
                .map_err(|err| BadRequest(Some(err.to_string())))?;
            
            // set jwt cookie
            let jwt_token = (SessionAuth {
                exp: (Utc::now() + Duration::days(SESSION_DAYS)).timestamp(),
                session_id: created_session.id as i64,
                access_token: rand_token
            }).to_jwt(&config.jwt_secret);
            let cookie = Cookie::build("session_user_jwt_cookie", jwt_token)
                .path("/")
                .secure(true)
                .finish();
            cookies.add_private(cookie);
            
            return Ok(Json("Login success".into()))
        }
    }

    // fail by default
    Err(BadRequest(Some("Invalid username or password".into())))
}

// logout (deletes all sessions)
#[post("/logout")]
pub fn sessions_logout(conn: PapersideApiDbConn, su: SessionUser, mut cookies: Cookies) -> Result<Json<String>,BadRequest<String>> {
    // removes sessions from database
    if let Some(user) = su.user {
        diesel::delete(schema::sessions::table
            .filter(schema::sessions::user_id.eq(user.id)))
            .execute(&conn.0)
            .map_err(|err| BadRequest(Some(err.to_string())))?;
    }

    // removes cookie
    cookies.remove_private(Cookie::named("session_user_jwt_cookie"));

    // returns success
    Ok(Json("Logout Success!".into()))
}

#[cfg(test)]
pub mod test {
    use rocket::local::LocalResponse;
    use rocket::http::{ContentType,Status};
    use crate::{json_string};
    use crate::app::test::{test_client};

    /// exported test helper for login of test account
    pub fn help_login<'a>(user_name: &str, hashed_password: &str) -> LocalResponse<'a> {
        test_client().post("/login")
            .body(json_string!({
                "user_name": user_name,
                "one_hashed": hashed_password,
            }))
            .header(ContentType::JSON).dispatch()
    }

    /// exported test helper for logout
    pub fn help_logout<'a>() -> LocalResponse<'a> {
        test_client().post("/logout").dispatch()
    }

    /// exported test helper for register
    pub fn help_register<'a>(user_name: &str, hashed_password: &str) -> LocalResponse<'a> {
        test_client().post("/register")
            .body(json_string!({
                "user_name": user_name,
                "one_hashed": hashed_password,
            }))
            .header(ContentType::JSON).dispatch()
    }


    #[test]
    fn test_login_logout() {
        // test params
        let user_name = format!("Test User {:?}", chrono::Utc::now());
        let hashed_password = "abcdef0123456789";

        // register test user
        let r_register = help_register(&user_name, hashed_password);
        assert_eq!(r_register.status(), Status::Ok);

        // test login
        let response = help_login(&user_name, hashed_password);
        assert_eq!(response.status(), Status::Ok);

        // check index response
        let mut r_1_c = test_client().get("/").dispatch();
        assert_eq!(r_1_c.status(), Status::Ok);
        assert_eq!(r_1_c.body_string(), Some(format!("Hello, {}!", user_name).into()));

        // test logout
        let response_2 = help_logout();
        assert_eq!(response_2.status(), Status::Ok);

        // check index response
        let mut r_2_c = test_client().get("/").dispatch();
        assert_eq!(r_2_c.status(), Status::Ok);
        assert_eq!(r_2_c.body_string(), Some("Hello, world!".into()));

        // test invalid login
        let mut r_3 = help_login(&user_name, "bad password");
        assert_eq!(r_3.status(), Status::BadRequest);
        assert_eq!(r_3.body_string(), Some("Invalid username or password".into()));
    }
}
