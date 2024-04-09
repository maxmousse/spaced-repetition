use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::{course, note, question, section, user, PrismaClient};

use crate::graphql::{
    note::types::Note, question::types::Question, section::types::Section, user::types::User,
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Course {
    id: ID,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    title: String,
    description: Option<String>,

    #[graphql(skip)]
    author_id: String,
}

#[ComplexObject]
impl Course {
    pub async fn author(&self, ctx: &Context<'_>) -> Result<User> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .user()
            .find_unique(user::UniqueWhereParam::IdEquals(
                self.author_id.clone().into(),
            ))
            .exec()
            .await?
            .unwrap()
            .into())
    }

    pub async fn sections(&self, ctx: &Context<'_>) -> Result<Vec<Section>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .section()
            .find_many(vec![section::course_id::equals(self.id.clone().into())])
            .exec()
            .await?
            .into_iter()
            .map(|section| section.into())
            .collect())
    }

    pub async fn notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .find_many(vec![note::course_id::equals(self.id.clone().into())])
            .exec()
            .await?
            .into_iter()
            .map(|note| note.into())
            .collect())
    }

    pub async fn questions(&self, ctx: &Context<'_>) -> Result<Vec<Question>> {
        let db = ctx.data::<PrismaClient>().unwrap();

        Ok(db
            .question()
            .find_many(vec![question::course_id::equals(self.id.clone().into())])
            .exec()
            .await?
            .into_iter()
            .map(|question| question.into())
            .collect())
    }
}

impl From<course::Data> for Course {
    fn from(value: course::Data) -> Self {
        Course {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            title: value.title,
            description: value.description,
            author_id: value.author_id.into(),
        }
    }
}

impl From<&course::Data> for Course {
    fn from(value: &course::Data) -> Self {
        Course {
            id: value.id.clone().into(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            title: value.title.clone(),
            description: value.description.clone(),
            author_id: value.author_id.clone().into(),
        }
    }
}
