use super::types::User;
use async_graphql::{Context, Object, Result};
use prisma_client::PrismaClient;

#[derive(Default, Clone)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    async fn get_users(&self, context: &Context<'_>) -> Result<Vec<User>> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect())
    }
}
