use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::{course, note, section};

use crate::graphql::{course::types::Course, note::types::Note};
use graphql_common::utils::context::unwrap_context_data;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Section {
    id: ID,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    title: String,
    description: Option<String>,
    index: i32,

    #[graphql(skip)]
    course_id: String,
}

#[ComplexObject]
impl Section {
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

    pub async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .note()
            .find_many(vec![note::section_id::equals(self.id.clone().into())])
            .exec()
            .await?
            .into_iter()
            .map(|note| note.into())
            .collect())
    }
}

impl From<section::Data> for Section {
    fn from(value: section::Data) -> Self {
        Section {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            title: value.title,
            description: value.description,
            course_id: value.course_id,
            index: value.index,
        }
    }
}

impl From<&section::Data> for Section {
    fn from(value: &section::Data) -> Self {
        Section {
            id: value.id.clone().into(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            title: value.title.clone(),
            description: value.description.clone(),
            course_id: value.course_id.clone(),
            index: value.index,
        }
    }
}
