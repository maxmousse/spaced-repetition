use std::sync::Arc;

use actix_web::{guard, web, App, HttpServer};
use authentication_middleware::AuthenticationMiddleware;
use graphql::{playground::graphql_playground, schema::get_schema_builder, server::graphql_server};

mod graphql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Instantiate the database client
    let db = Arc::new(
        prisma_client::new_client()
            .await
            .expect("Failed to connect to database"),
    );

    // Build the GraphQL schema
    let schema_builder = get_schema_builder();
    let schema = schema_builder.data(db.clone()).finish();

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(db.clone()))
            .app_data(web::Data::new(schema.clone()))
            .wrap(AuthenticationMiddleware::new())
            .service(web::resource("/").guard(guard::Post()).to(graphql_server))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
