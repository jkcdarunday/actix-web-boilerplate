#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
