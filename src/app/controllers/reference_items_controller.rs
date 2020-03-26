use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize,Serialize};

use crate::app::database::DbPool;
use crate::app::errors::ServiceError;
use crate::app::middleware::session_user::SessionUser;
use crate::app::models::{NewReferenceItem, NewReferenceItemSubmission, ReferenceItem, ReferenceItemSubmission};


/// response data for reference item including submission details
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceItemDetails {
    pub id: i32,
    pub title: String,
    pub url: Option<String>,
    pub is_public: bool,
}

/// GET /reference_items list endpoint
pub async fn list_reference_items(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || {
        use crate::schema::reference_items;
        use crate::schema::reference_item_submissions;
        use crate::schema::reference_item_submissions::dsl::{is_public};
        let conn: &PgConnection = &pool.get().unwrap();

        let items = reference_items::table.left_join(reference_item_submissions::table)
            .filter(is_public.eq(true))
            .get_results::<(ReferenceItem, Option<ReferenceItemSubmission>)>(conn)?;

        return Ok(items.into_iter().map(|(item,item_sub)| {
            ReferenceItemDetails {
                id: item.id,
                title: item.title,
                url: item.url,
                is_public: item_sub.unwrap().is_public,
            }
        }).collect::<Vec<ReferenceItemDetails>>())
    }).await;

    match res {
        Ok(queried_items) => Ok(HttpResponse::Ok().json(&queried_items)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}


/// post data for submitting a potentially-new reference item
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceItemData {
    pub title: String,
    pub url: Option<String>,
    pub is_public: bool,
}

/// POST /reference_items 
pub async fn create_reference_item(
    reference_item_data: web::Json<ReferenceItemData>,
    pool: web::Data<DbPool>,
    session_user: SessionUser,
) -> Result<HttpResponse, ServiceError> {
    let reference_item_data = reference_item_data.into_inner();
    let res = web::block(move || {
        // get db connection from pool
        use crate::schema::reference_items::dsl::{reference_items,url};
        use crate::schema::reference_item_submissions::dsl::{reference_item_submissions};
        let conn: &PgConnection = &pool.get().unwrap();

        // if url exists, check if item is already submitted
        if let Some(data_url) = reference_item_data.url.clone() {
            let existing_item: Result<ReferenceItem,_> = reference_items
                .filter(url.eq(data_url))
                .first(conn);
            if let Ok(item_data) = existing_item {
                // add submission reference for existing reference item
                let new_item_sub = NewReferenceItemSubmission {
                    submitting_user_id: session_user.id,
                    reference_item_id: item_data.id,
                    is_public: reference_item_data.is_public,
                };
                diesel::insert_into(reference_item_submissions)
                    .values(&new_item_sub).execute(conn)?;

                // return details
                return Ok(ReferenceItemDetails {
                    id: item_data.id,
                    title: item_data.title,
                    url: item_data.url,
                    is_public: reference_item_data.is_public,
                });
            }
        }

        // no existing item found, insert new reference item
        let new_item = NewReferenceItem {
            title: reference_item_data.title,
            url: reference_item_data.url,
        };
        let inserted_item: ReferenceItem =
            diesel::insert_into(reference_items).values(&new_item).get_result(conn)?;

        // add submission reference for created item
        let new_item_sub = NewReferenceItemSubmission {
            submitting_user_id: session_user.id,
            reference_item_id: inserted_item.id,
            is_public: reference_item_data.is_public,
        };
        diesel::insert_into(reference_item_submissions)
            .values(&new_item_sub).execute(conn)?;

        // return details
        return Ok(ReferenceItemDetails {
            id: inserted_item.id,
            title: inserted_item.title,
            url: inserted_item.url,
            is_public: reference_item_data.is_public,
        });
    }).await;

    match res {
        Ok(created_item) => Ok(HttpResponse::Ok().json(&created_item)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}
