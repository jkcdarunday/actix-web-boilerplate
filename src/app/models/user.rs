#[allow(unused)]
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}
