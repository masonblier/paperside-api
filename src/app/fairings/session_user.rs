use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::Outcome::*;

use bcrypt;

use crate::app::{AppConfig,PapersideApiDbConn};
use crate::app::models::{Session,User,SessionUser,SessionAuth};

// fairing/guard to store loaded session data in request-local cache
impl<'a, 'r> FromRequest<'a, 'r> for SessionUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        Success(req.local_cache(|| session_user_from_request(req)).clone())
    }
}

/// matches private cookie session token to database session and user values
pub fn session_user_from_request<'a,'r>(req: &'a Request<'r>) -> SessionUser {
    let config: State<AppConfig> = req.guard().expect("Sessions - Rocket Guard AppConfig error");
    let conn: PapersideApiDbConn = req.guard().expect("Sessions - Rocket Guard DbConn error");
    let mut cookies = req.cookies();
    
    if let Some(ref jwt_cookie) = cookies.get_private("session_user_jwt_cookie") {
        // get auth struct from jwt encoded cookie value
        let auth = SessionAuth::from_jwt(jwt_cookie.value(), &config.jwt_secret);

        if let Some(auth_) = &auth {
            // get user info from session auth
            let user = get_user_from_session_auth(&auth_, &conn);

            if user.is_some() {
                // return session
                return SessionUser {
                    auth,
                    user,
                }
            }
        }
    }

    // Something went wrong, return None
    SessionUser {
        auth: None,
        user: None,
    }
}


/// gets user from session stored in db or None
pub fn get_user_from_session_auth(auth: &SessionAuth, conn: &PapersideApiDbConn) -> Option<User> {
    use diesel::prelude::*;
    use crate::schema::sessions::dsl::*;
    use crate::schema::users::dsl::*;

    // query session instance from db
    let session_row: Session = sessions
        .find(auth.session_id as i32)
        .first(&conn.0)
        .expect("Failed to query session from JWT session_id");

    // verify session access_token
    if bcrypt::verify(&auth.access_token, &session_row.hashed_access_token).expect("bcrypt error") {
        // query user data
        let mut user: User = users
            .find(session_row.user_id)
            .first(&conn.0)
            .expect("Failed to query user from JWT session");

        // erase reference to doublehashed value
        user.doublehashed = "".into();

        return Some(user)
    }

    // Something went wrong, no user found
    None
}
