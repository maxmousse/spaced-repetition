use async_graphql::{InputObject, SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::note;

#[derive(SimpleObject)]
pub struct Note {
    pub id: ID,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub title: String,
    pub content: String,
    pub index: i32,
    pub course_id: String,
    pub section_id: String,
}

#[derive(InputObject)]
pub struct GetNoteInput {
    pub id: ID,
}

#[derive(InputObject)]
pub struct CreateNoteInput {
    pub title: String,
    pub content: String,
    pub index: i32,
    pub course_id: String,
    pub section_id: String,
}

#[derive(InputObject)]
pub struct UpdateNoteInput {
    pub id: ID,
    pub title: String,
    pub content: String,
}

#[derive(InputObject)]
pub struct DeleteNoteInput {
    pub id: ID,
}

impl From<note::Data> for Note {
    fn from(value: note::Data) -> Self {
        Note {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            title: value.title,
            content: value.content,
            index: value.index,
            course_id: value.course_id,
            section_id: value.section_id,
        }
    }
}

impl From<&note::Data> for Note {
    fn from(value: &note::Data) -> Self {
        Note {
            id: value.id.clone().into(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            title: value.title.clone(),
            content: value.content.clone(),
            index: value.index,
            course_id: value.course_id.clone(),
            section_id: value.section_id.clone(),
        }
    }
}
