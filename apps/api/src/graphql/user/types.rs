use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::user;

#[derive(SimpleObject)]
pub struct User {
    pub id: ID,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<user::Data> for User {
    fn from(value: user::Data) -> Self {
        User {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.update_at,
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
            created_at: value.created_at.clone(),
            updated_at: value.update_at.clone(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
        }
    }
}
