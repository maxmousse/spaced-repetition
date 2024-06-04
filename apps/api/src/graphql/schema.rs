use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

use super::authentication::mutations::AuthenticationMutations;
use super::note::mutations::NoteMutations;
use super::note::queries::NoteQueries;
use super::user::queries::UserQueries;

/// Queries of the graphql schema
#[derive(MergedObject, Default, Clone)]
pub struct Queries(UserQueries, NoteQueries);

/// Mutations of the graphql schema
#[derive(MergedObject, Default, Clone)]
pub struct Mutations(NoteMutations, AuthenticationMutations);

pub type AppSchemaBuilder = SchemaBuilder<Queries, Mutations, EmptySubscription>;
pub type AppSchema = Schema<Queries, Mutations, EmptySubscription>;

/// Returns a new instance of the AppSchemaBuilder with an empty context
pub fn get_schema_builder() -> AppSchemaBuilder {
    Schema::build(Queries::default(), Mutations::default(), EmptySubscription)
}
