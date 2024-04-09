use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, FixedOffset};

#[derive(SimpleObject)]
pub struct Question {
    pub id: ID,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub title: String,
    pub content: String,
    pub course_id: String,
    pub note_id: String,
}
