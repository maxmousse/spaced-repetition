use async_graphql::InputObject;

#[derive(InputObject)]
pub struct Pagination {
    pub take: i64,
    pub skip: i64,
}
