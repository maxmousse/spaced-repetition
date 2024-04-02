use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}
