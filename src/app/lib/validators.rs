use diesel::prelude::*;
use diesel::QueryDsl;
use validator::ValidationError;

use crate::app::db::DB;
use crate::app::models::user::User;
use crate::schema::users;

pub fn user_id_exists(id: i32) -> Result<(), ValidationError> {
    let query_result = users::table
        .filter(users::id.eq(id))
        .load::<User>(&mut DB.get().unwrap())
        .map_err(|_| ValidationError::new("User not found"))?;

    if query_result.is_empty() {
        return Err(ValidationError::new("User not found"));
    }

    Ok(())
}
