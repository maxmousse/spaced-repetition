use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, FixedOffset};

#[derive(SimpleObject)]
struct Section {
    id: ID,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    title: String,
    description: Option<String>,
    course_id: String,
    index: i32,
}
