#[cfg(test)]
mod tests {
    use actix_web::{http, test::{call_service, TestRequest}};

    use crate::tests::test_helpers::tests::{create_test_app,login_test_user};
    use crate::app::controllers::reference_items_controller::{ReferenceItemData,ReferenceItemDetails};

    #[actix_rt::test]
    async fn test_create_public_reference_item() {
        // setup test app
        let mut app = create_test_app().await;
        // login test user
        let cookie = login_test_user(&mut app).await;

        // create test reference item request
        let item_data = ReferenceItemData {
            title: "A test reference item".to_string(),
            url: Some("http://example.org".to_string()),
            is_public: true,
        };

        // make request
        let req = TestRequest::post()
            .cookie(cookie.clone())
            .set_json(&item_data)
            .uri("/reference_item")
            .to_request();
        let resp = call_service(&mut app, req).await;

        // expect success
        assert_eq!(resp.status(), http::StatusCode::OK);

        // parse json from response
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        let parsed_json: ReferenceItemDetails = serde_json::from_slice(response_body)
            .expect("Failed to parse ReferenceItemDetails from response");

        // expect returned details
        assert!(parsed_json.id > 0, "Expected result to have valid id");
        assert_eq!(parsed_json.title, item_data.title);
        assert_eq!(parsed_json.is_public, item_data.is_public);
    }
}