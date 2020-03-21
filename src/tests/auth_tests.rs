#[cfg(test)]
mod tests {
    use actix_web::{http};

    use crate::tests::test_helpers::tests::{test_get};

    #[actix_rt::test]
    async fn test_unauthorized_get_me() {
        let resp = test_get("/auth").await;
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }
}