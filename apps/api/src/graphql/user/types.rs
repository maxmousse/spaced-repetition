use async_graphql::SimpleObject;
use prisma_client::user;

#[derive(SimpleObject)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<user::Data> for User {
    fn from(user: user::Data) -> Self {
        User {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        }
    }
}

impl From<&user::Data> for User {
    fn from(user: &user::Data) -> Self {
        User {
            id: user.id.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
        }
    }
}
