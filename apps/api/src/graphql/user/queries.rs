use crate::graphql::utils::context::unwrap_context_data;

use super::types::User;
use async_graphql::{Context, Object, Result};

#[derive(Default, Clone)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let (db, _) = unwrap_context_data(ctx);

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
