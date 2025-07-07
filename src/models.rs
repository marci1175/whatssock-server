use diesel::{
    Selectable,
    prelude::{Insertable, Queryable, QueryableByName},
};

#[derive(Debug, Clone, Selectable, QueryableByName, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccount {
    pub id: i32,
    pub username: String,
    pub passw: String,
    pub email: String,
    pub gender: bool,
    pub created_at: chrono::NaiveDate,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountRegister {
    pub username: String,
    pub passw: String,
    pub email: String,
    pub gender: bool,
}
