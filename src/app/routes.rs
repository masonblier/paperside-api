use actix_web::{web};

use crate::app::controllers::*;

// build application routes
pub fn build_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // routes
        .service(
            web::resource("/reference_items")
                .route(web::get().to(reference_items_controller::list_reference_items))
                .route(web::post().to(reference_items_controller::create_reference_item))
        )
        .service(
            web::resource("/register")
                .route(web::post().to(registrations_controller::register_user))
        )
        .service(
            web::resource("/auth")
                .route(web::post().to(auth_controller::login))
                .route(web::delete().to(auth_controller::logout))
                .route(web::get().to(auth_controller::get_me))
        );
}