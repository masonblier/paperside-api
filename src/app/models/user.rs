use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};
use diesel::prelude::*;
use diesel::expression::{Expression};
use diesel::pg::Pg;
use diesel::query_dsl::{QueryDsl};

use crate::schema::{users};

type AllUserColumns = (
    users::id,
    users::name,
    users::doublehashed,
    users::created_at,
);
type BoxedUserQuery<'a> = users::BoxedQuery<'a, Pg, <AllUserColumns as Expression>::SqlType>;


// user model
#[derive(Clone, Deserialize, Serialize, Identifiable, Queryable, AsChangeset)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub doublehashed: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub doublehashed: String,
    pub created_at: DateTime<Utc>,
}


impl User {
    /// Prepares user query by equal username
    pub fn by_name<'a>(name: &'a str) -> BoxedUserQuery<'a> {
        users::table.into_boxed().filter(users::name.eq(name))
    }
}
