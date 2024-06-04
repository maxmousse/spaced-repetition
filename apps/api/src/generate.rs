use std::fs;

use graphql::schema::get_schema_builder;

mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Build schema SDL
    let schema_sdl = get_schema_builder().finish().sdl();

    // Write the schema in a file
    fs::write("schema.gql", schema_sdl).unwrap();

    Ok(())
}
