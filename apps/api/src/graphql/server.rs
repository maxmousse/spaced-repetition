use actix_web::{web, HttpMessage, HttpRequest};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use authentication_context::AuthenticationContext;
use std::sync::Arc;

use super::schema::AppSchema;

/// Graphql server handler
pub async fn graphql_server(
    http_req: HttpRequest,
    schema: web::Data<AppSchema>,
    gql: GraphQLRequest,
) -> GraphQLResponse {
    let authentication_context = http_req
        .extensions()
        .get::<Arc<AuthenticationContext>>()
        .unwrap()
        .clone();

    let gql_request = gql.into_inner().data(authentication_context);

    schema.execute(gql_request).await.into()
}
