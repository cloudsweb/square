use crate::schema::{users, posts};

#[derive(Queryable, PartialEq, Debug, Selectable)]
pub struct User {
  id: i64,
}
