use actix_web::{
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer, Result,
};
use async_graphql::http::graphiql_source;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use graphql::schema::{build_schema, AppSchema};

mod graphql;

async fn grahpql_server(schema: web::Data<AppSchema>, gql: GraphQLRequest) -> GraphQLResponse {
    schema.execute(gql.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/", None)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = prisma_client::new_client()
        .await
        .expect("Failed to connect to database");
    let schema = build_schema(db).await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(grahpql_server))
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
