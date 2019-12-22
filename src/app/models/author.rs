use serde::{Deserialize,Serialize};

use crate::schema::authors;
use crate::schema::reference_authors;

use super::reference_item::ReferenceItem;

// authors model
#[derive(Deserialize, Serialize, Identifiable, Queryable, AsChangeset)]
#[table_name="authors"]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name="authors"]
pub struct NewAuthor<'a> {
    pub name: &'a str,
}


// references-authors many to many join model
#[derive(Deserialize, Serialize, Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Author)]
#[belongs_to(ReferenceItem, foreign_key="reference_id")]
#[table_name="reference_authors"]
pub struct ReferenceAuthor {
    pub id: i32,
    pub author_id: i32,
    pub reference_id: i32,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name="reference_authors"]
pub struct NewReferenceAuthor {
    pub author_id: i32,
    pub reference_id: i32,
}
