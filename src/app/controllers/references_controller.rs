use diesel::prelude::*;
use rocket::{get,post,put,delete};
use rocket::response::status::{BadRequest};
use rocket_contrib::json::Json;
use serde::{Deserialize,Serialize};

use crate::app::{PapersideApiDbConn};
use crate::app::models::*;

use crate::schema;

// holds response data for each item and loaded associations
#[derive(Deserialize, Serialize)]
pub struct ReferenceItemResponse {
    reference: ReferenceItem,
    authors: Vec<Author>
}

// holds create attributes for reference and associations
#[derive(Deserialize, Serialize)]
pub struct NewReferenceItemRequest {
    reference: NewReferenceItem,
}

// list
#[get("/")]
pub fn list_reference_items(conn: PapersideApiDbConn) -> Result<Json<Vec<ReferenceItemResponse>>,BadRequest<String>> {
    let reference_items = schema::reference_items::table
        .order(schema::reference_items::id.desc())
        .load::<ReferenceItem>(&conn.0)
        .map_err(|err| BadRequest(Some(err.to_string())))?;
    // gathers related data for each row
    let response_items: Result<Vec<ReferenceItemResponse>,BadRequest<String>> = reference_items.into_iter().map(|item| load_reference_item_response(&conn, item)).collect();
    // return all as json
    Ok(Json(response_items?))
}

// create
#[post("/", format = "application/json", data = "<attr>")]
pub fn create_reference_item(conn: PapersideApiDbConn, attr: Json<NewReferenceItemRequest>) -> Result<Json<ReferenceItem>,BadRequest<String>> {
    let new_reference_item = attr.into_inner();

    diesel::insert_into(schema::reference_items::table)
        .values(&new_reference_item.reference)
        .get_result(&conn.0)
        .map(|reference_item| Json(reference_item))
        .map_err(|err| BadRequest(Some(err.to_string())))
}

// read
#[get("/<reference_item_id>")]
pub fn read_reference_item(conn: PapersideApiDbConn, reference_item_id: i32) -> Result<Json<ReferenceItemResponse>,BadRequest<String>> {
    use crate::schema::reference_items::dsl::*;

    let reference_item = reference_items
        .find(reference_item_id)
        .first(&conn.0)
        .map_err(|_err| BadRequest(Some(format!("Reference not found: {}", reference_item_id))))?;
    let response_item = load_reference_item_response(&conn, reference_item)?;
    Ok(Json(response_item))
}

// update
#[put("/<reference_item_id>", format = "application/json", data = "<attr>")]
pub fn update_reference_item(conn: PapersideApiDbConn, reference_item_id: i32, attr: Json<ReferenceItem>) -> Result<Json<ReferenceItem>,BadRequest<String>> {
    let reference_item: ReferenceItem = attr.into_inner();
    diesel::update(schema::reference_items::table.find(reference_item_id))
        .set(&reference_item)
        .get_result(&conn.0)
        .map(|reference_item| Json(reference_item))
        .map_err(|err| BadRequest(Some(err.to_string())))
}

// delete
#[delete("/<reference_item_id>")]
pub fn delete_reference_item(conn: PapersideApiDbConn, reference_item_id: i32) -> Result<Json<usize>,BadRequest<String>> {
    diesel::delete(schema::reference_items::table.find(reference_item_id))
        .execute(&conn.0)
        .map(|count| Json(count))
        .map_err(|err| BadRequest(Some(err.to_string())))
}


// loads associated data for reference item
fn load_reference_item_response(conn: &PapersideApiDbConn, reference_item: ReferenceItem) -> Result<ReferenceItemResponse,BadRequest<String>> {
    let authors = get_authors_for_reference(&conn, &reference_item)?;
    Ok(ReferenceItemResponse {
        reference: reference_item,
        authors
    })
}

// join function to get all authors for a reference
fn get_authors_for_reference(conn: &PapersideApiDbConn, reference_item: &ReferenceItem) -> Result<Vec<Author>,BadRequest<String>> {
    use diesel::pg::expression::dsl::any;
    let reference_author_ids = ReferenceAuthor::belonging_to(reference_item).select(schema::reference_authors::author_id);
    schema::authors::table
        .filter(schema::authors::id.eq(any(reference_author_ids)))
        .load::<Author>(&conn.0)
        .map_err(|err| BadRequest(Some(err.to_string())))
}



#[cfg(test)]
mod test {
    use rocket::http::{ContentType,Status};
    use crate::{json_string};
    use crate::app::test::{response_json_value,test_client};


    #[test]
    fn test_crud_of_reference_items() {
        // test params
        let title = "Generating Large Images from Latent Vectors";
        let url = "http://blog.otoro.net/2016/04/01/generating-large-images-from-latent-vectors/";

        // test create
        let mut response = test_client().post("/reference_items")
            .body(json_string!({
                "reference": {
                    "title": title,
                    "url": url
                }
            }))
            .header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::Ok);

        // check values of deserialized first item
        let response_json = response_json_value(&mut response);
        assert_eq!(response_json.get("title").unwrap().as_str(), Some(title));

        // id of created item
        let created_id = response_json.get("id").unwrap().as_i64().unwrap();


        // test list
        let mut response = test_client().get("/reference_items").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // get deserialized first item
        let response_json = response_json_value(&mut response);
        let first_reference = response_json.get(0).unwrap().get("reference").unwrap();
        assert_eq!(first_reference.get("title").unwrap().as_str(), Some(title));
        assert_eq!(first_reference.get("url").unwrap().as_str(), Some(url));


        // test read
        let mut response = test_client().get(format!("/reference_items/{}", created_id)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        // get deserialized item
        let response_json = response_json_value(&mut response);
        let read_reference = response_json.get("reference").unwrap();
        assert_eq!(read_reference.get("title").unwrap().as_str(), Some(title));
        assert_eq!(read_reference.get("url").unwrap().as_str(), Some(url));


        // test update
        let title_2 = "Modified Title";
        let mut update_response = test_client().put(format!("/reference_items/{}", created_id))
            .body(json_string!({
                "id": created_id,
                "title": title_2,
                "url": url
            }))
            .header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::Ok);

        // get deserialized item
        let updated_response_json = response_json_value(&mut update_response);
        assert_eq!(updated_response_json.get("title").unwrap().as_str(), Some(title_2));
        assert_eq!(updated_response_json.get("url").unwrap().as_str(), Some(url));


        // test delete
        let mut deleted_response = test_client().delete(format!("/reference_items/{}", created_id)).dispatch();
        assert_eq!(deleted_response.status(), Status::Ok);

        // check deleted count
        let deleted_response_count = response_json_value(&mut deleted_response).as_i64();
        assert_eq!(deleted_response_count, Some(1));
    }
}
