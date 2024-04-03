use std::fs;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use graphql::schema::Query;

mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build schema
    let schema_sdl = Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .finish()
        .sdl();

    // Write the schema in a file
    fs::write("schema.gql", schema_sdl).unwrap();

    Ok(())
}
