#[derive(Queryable)]
pub struct Mail {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::mails;

#[derive(Insertable)]
#[table_name="mails"]
pub struct NewMail<'a> {
    pub title: &'a str,
    pub body: &'a str,
}