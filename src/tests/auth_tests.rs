#[cfg(test)]
mod tests {
    use actix_web::{http, test::{call_service, TestRequest}};

    use crate::tests::test_helpers::tests::{create_test_app};

    #[actix_rt::test]
    async fn test_unauthorized_get_me() {
        // setup test
        let mut app = create_test_app().await;

        // make request
        let req = TestRequest::get()
            .uri("/auth")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect UNAUTHORIZED 401 response
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }
}