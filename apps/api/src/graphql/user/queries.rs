use super::types::User;
use async_graphql::{Object, Result};

#[derive(Default, Clone)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self) -> Result<Vec<User>> {
        Result::Ok(vec![User {
            id: ("1").to_string(),
            name: "user1".to_string(),
            email: "user1@gmail.com".to_string(),
        }])
    }
}
