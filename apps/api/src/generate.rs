use std::fs;

use async_graphql::{EmptySubscription, Schema};

use graphql::schema::{Mutations, Queries};

mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build schema
    let schema_sdl = Schema::build(Queries::default(), Mutations::default(), EmptySubscription)
        .finish()
        .sdl();

    // Write the schema in a file
    fs::write("schema.gql", schema_sdl).unwrap();

    Ok(())
}
