#[cfg(test)]
mod tests {
    use actix_web::{http, test::{call_service, TestRequest}};

    use crate::tests::test_helpers::tests::{create_test_app};
    use crate::app::controllers::auth_controller::AuthRequestData;
    use crate::app::middleware::session_user::SessionUser;

    #[actix_rt::test]
    async fn test_unauthorized_get_me() {
        // setup test app
        let mut app = create_test_app().await;

        // make request
        let req = TestRequest::get()
            .uri("/auth")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect UNAUTHORIZED 401 response
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_login_and_get_me() {
        // setup test app
        let mut app = create_test_app().await;

        // create test auth request
        let auth_data = AuthRequestData {
            name: "test_user".to_string(),
            password: "test_user".to_string()
        };

        // make login request
        let req = TestRequest::post()
            .set_json(&auth_data)
            .uri("/auth")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect success
        assert_eq!(resp.status(), http::StatusCode::OK);

        // get cookie set by login request
        let cookie = resp.response().cookies().next().unwrap().to_owned();

        // make get /auth request
        let req = TestRequest::get()
            .cookie(cookie.clone())
            .uri("/auth")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect success
        assert_eq!(resp.status(), http::StatusCode::OK);

        // parse json from response
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let parsed_json: SessionUser = serde_json::from_slice(response_body)
            .expect("Failed to parse SessionUser from GET /auth response");

        // expect returned user name
        assert_eq!(parsed_json.name, auth_data.name);
    }

    #[actix_rt::test]
    async fn test_login_logout_and_get_me() {
        // setup test app
        let mut app = create_test_app().await;

        // create test auth request
        let auth_data = AuthRequestData {
            name: "test_user".to_string(),
            password: "test_user".to_string()
        };

        // make login request
        let req1 = TestRequest::post()
            .set_json(&auth_data)
            .uri("/auth")
            .to_request();
        let resp1 = call_service(&mut app, req1).await;

        // expect success
        assert_eq!(resp1.status(), http::StatusCode::OK);

        // get cookie set by login request
        let cookie = resp1.response().cookies().next().unwrap().to_owned();

        // make logout (DELETE /auth) request
        let req2 = TestRequest::delete()
            .cookie(cookie.clone())
            .uri("/auth")
            .to_request();
        let resp2 = call_service(&mut app, req2).await;

        // expect success
        assert_eq!(resp2.status(), http::StatusCode::OK);

        // make get /auth request
        let req3 = TestRequest::get()
            .cookie(cookie.clone())
            .uri("/auth")
            .to_request();
        let resp3 = call_service(&mut app, req3).await;

        // expect UNAUTHORIZED 401 response
        assert_eq!(resp3.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_bad_login() {
        // setup test app
        let mut app = create_test_app().await;

        // create test auth request
        let auth_data = AuthRequestData {
            name: "test_user".to_string(),
            password: "not_the_password".to_string()
        };

        // make login request
        let req = TestRequest::post()
            .set_json(&auth_data)
            .uri("/auth")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect an error
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }
}