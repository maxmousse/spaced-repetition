use actix_web::{HttpResponse, Result};
use async_graphql::http::graphiql_source;

/// GraphQL playground server
pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/", None)))
}
