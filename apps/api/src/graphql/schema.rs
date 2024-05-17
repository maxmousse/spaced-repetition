use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

use super::authentication::mutations::AuthenticationMutations;
use super::note::mutations::NoteMutations;
use super::note::queries::NoteQueries;
use super::user::queries::UserQueries;

#[derive(MergedObject, Default, Clone)]
pub struct Queries(UserQueries, NoteQueries);

#[derive(MergedObject, Default, Clone)]
pub struct Mutations(NoteMutations, AuthenticationMutations);

pub type AppSchemaBuilder = SchemaBuilder<Queries, Mutations, EmptySubscription>;
pub type AppSchema = Schema<Queries, Mutations, EmptySubscription>;

pub fn get_schema_builder() -> AppSchemaBuilder {
    Schema::build(Queries::default(), Mutations::default(), EmptySubscription)
}
