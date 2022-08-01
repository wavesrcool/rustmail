#[derive(Queryable)]
pub struct Mail {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}