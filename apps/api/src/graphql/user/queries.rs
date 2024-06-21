use crate::graphql::user::inputs::GetUsersInput;

use graphql_common::utils::context::unwrap_context_data;

use super::types::User;
use async_graphql::{Context, Object, Result};
use prisma_client::user;

#[derive(Default, Clone)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    async fn get_users(&self, ctx: &Context<'_>, input: GetUsersInput) -> Result<Vec<User>> {
        let (db, _) = unwrap_context_data(ctx);
        let (filter, sort, page) = input.format();

        Ok(db
            .user()
            .find_many(filter)
            .order_by(sort)
            .take(page.take)
            .skip(page.skip)
            .exec()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect())
    }

    async fn get_user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .user()
            .find_unique(user::id::equals(id))
            .exec()
            .await?
            .unwrap()
            .into())
    }
}
