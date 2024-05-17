use std::sync::Arc;

use actix_web::{guard, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::graphiql_source;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use authentication_context::AuthenticationContext;
use authentication_middleware::AuthenticationMiddleware;
use graphql::schema::{get_schema_builder, AppSchema};

mod graphql;

/// Handler for GraphQL requests.
async fn graphql_server(
    http_req: HttpRequest,
    schema: web::Data<AppSchema>,
    gql: GraphQLRequest,
) -> GraphQLResponse {
    let authentication_context = http_req
        .extensions()
        .get::<Arc<AuthenticationContext>>()
        .unwrap()
        .clone();

    println!("HELLO HELLO HELLO {:?}", authentication_context);

    let gql_request = gql.into_inner().data(authentication_context);

    schema.execute(gql_request).await.into()
}

/// Handler for GraphQL Playground.
async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/", None)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(
        prisma_client::new_client()
            .await
            .expect("Failed to connect to database"),
    );

    let schema_builder = get_schema_builder();
    let schema = schema_builder.data(db.clone()).finish();

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
