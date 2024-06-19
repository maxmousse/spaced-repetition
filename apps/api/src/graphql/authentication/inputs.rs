use async_graphql::InputObject;

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
