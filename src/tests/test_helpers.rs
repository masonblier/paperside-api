#[cfg(test)]
pub mod tests {
    use crate::app::database::{get_database_pool};
    use crate::app::identity::get_identity_service;
    use crate::app::routes::build_routes;
    use actix_web::dev::ServiceResponse;
    use actix_web::{test, web, App};

    /// Helper for HTTP GET integration tests
    pub async fn test_get(route: &str) -> ServiceResponse {
        dotenv::from_filename(".env.test").ok();

        let mut app = test::init_service(
            App::new()
                // set up DB pool to be used with web::Data<Pool> extractor
                .data(get_database_pool())
                // identity
                .wrap(get_identity_service())
                // json request parsing config
                .data(web::JsonConfig::default().limit(4096))
                .configure(build_routes)
        )
        .await;

        test::call_service(
            &mut app,
            test::TestRequest::get()
                // .cookie(cookie.clone())
                .uri(route)
                .to_request(),
        )
        .await
    }
}