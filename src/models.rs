use super::schema::mails;


#[derive(Queryable)]
pub struct Address {
    pub id: i32,
    pub pass: bool,
    pub address: String,
    pub name: String,
}

#[derive(Queryable)]
pub struct Message {
    pub m_id: String,
    pub m_0: String,
    pub m_1: String,
    pub ts: String,
}

#[derive(Queryable)]
pub struct Mail {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable)]
#[table_name="mails"]
pub struct NewMail<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

