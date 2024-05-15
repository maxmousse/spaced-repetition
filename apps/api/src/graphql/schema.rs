use async_graphql::{EmptySubscription, MergedObject, Schema};

use super::authentication::AuthenticationMutations;
use super::note::mutations::NoteMutations;
use super::note::queries::NoteQueries;
use super::user::queries::UserQueries;

#[derive(MergedObject, Default, Clone)]
pub struct Queries(UserQueries, NoteQueries);

#[derive(MergedObject, Default, Clone)]
pub struct Mutations(NoteMutations, AuthenticationMutations);

pub type AppSchema = Schema<Queries, Mutations, EmptySubscription>;

pub async fn build_schema(db: prisma_client::PrismaClient) -> AppSchema {
    Schema::build(Queries::default(), Mutations::default(), EmptySubscription)
        .data(db)
        .finish()
}
