use super::types::User;
use async_graphql::{Context, Object, Result};
use prisma_client::PrismaClient;

#[derive(Default, Clone)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, context: &Context<'_>) -> Result<Vec<User>> {
        let database = context.data::<PrismaClient>().unwrap();

        Ok(database
            .user()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect())
    }
}
