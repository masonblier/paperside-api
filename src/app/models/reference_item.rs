use serde::{Deserialize,Serialize};

use crate::schema::reference_items;
use crate::schema::reference_item_submissions;

// reference item model
#[derive(Deserialize, Serialize, Identifiable, Queryable, AsChangeset)]
#[table_name="reference_items"]
pub struct ReferenceItem {
    pub id: i32,
    pub title: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name="reference_items"]
pub struct NewReferenceItem {
    pub title: String,
    pub url: Option<String>,
}

// reference item model
#[derive(Deserialize, Insertable, Serialize, Queryable)]
#[table_name="reference_item_submissions"]
pub struct ReferenceItemSubmission {
    pub submitting_user_id: i32,
    pub reference_item_id: i32,
    pub is_public: bool,
}