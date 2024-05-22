use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::{course, note, question};

use crate::graphql::{
    course::types::Course, note::types::Note, utils::context::unwrap_context_data,
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Question {
    pub id: ID,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub content: String,

    #[graphql(skip)]
    pub course_id: String,

    #[graphql(skip)]
    pub note_id: Option<String>,
}

#[ComplexObject]
impl Question {
    pub async fn course(&self, ctx: &Context<'_>) -> Result<Course> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .course()
            .find_unique(course::UniqueWhereParam::IdEquals(
                self.course_id.clone().into(),
            ))
            .exec()
            .await?
            .unwrap()
            .into())
    }

    pub async fn note(&self, ctx: &Context<'_>) -> Result<Option<Note>> {
        let (db, _) = unwrap_context_data(ctx);

        match &self.note_id {
            None => return Ok(None),
            Some(note_id) => Ok(Some(
                db.note()
                    .find_unique(note::UniqueWhereParam::IdEquals(note_id.clone().into()))
                    .exec()
                    .await?
                    .unwrap()
                    .into(),
            )),
        }
    }
}

impl From<question::Data> for Question {
    fn from(value: question::Data) -> Self {
        Question {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            content: value.content,
            course_id: value.course_id,
            note_id: value.note_id,
        }
    }
}

impl From<&question::Data> for Question {
    fn from(value: &question::Data) -> Self {
        Question {
            id: value.id.clone().into(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            content: value.content.clone(),
            course_id: value.course_id.clone(),
            note_id: value.note_id.clone(),
        }
    }
}
