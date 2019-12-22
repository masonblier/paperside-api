use serde::{Deserialize,Serialize};

use crate::schema::reference_items;

// word model
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
