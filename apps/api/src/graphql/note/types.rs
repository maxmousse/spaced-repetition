use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::{course, note, question, section};

use crate::graphql::{course::types::Course, question::types::Question, section::types::Section};
use graphql_common::utils::context::unwrap_context_data;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Note {
    pub id: ID,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub title: String,
    pub content: String,
    pub index: i32,

    #[graphql(skip)]
    pub course_id: String,

    #[graphql(skip)]
    pub section_id: String,
}

#[ComplexObject]
impl Note {
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

    pub async fn section(&self, ctx: &Context<'_>) -> Result<Section> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .section()
            .find_unique(section::UniqueWhereParam::IdEquals(
                self.section_id.clone().into(),
            ))
            .exec()
            .await?
            .unwrap()
            .into())
    }

    pub async fn questions(&self, ctx: &Context<'_>) -> Result<Vec<Question>> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .question()
            .find_many(vec![question::note_id::equals(Some(
                self.id.clone().into(),
            ))])
            .exec()
            .await?
            .into_iter()
            .map(|question| question.into())
            .collect())
    }
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
