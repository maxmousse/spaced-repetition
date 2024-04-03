use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

use super::user::queries::UserQuery;

#[derive(MergedObject, Default, Clone)]
pub struct Query(UserQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema(db: prisma_client::PrismaClient) -> AppSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
