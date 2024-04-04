use async_graphql::{SimpleObject, ID};
use prisma_client::user;

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<user::Data> for User {
    fn from(value: user::Data) -> Self {
        User {
            id: value.id.into(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
        }
    }
}

impl From<&user::Data> for User {
    fn from(value: &user::Data) -> Self {
        User {
            id: value.id.clone().into(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
        }
    }
}
