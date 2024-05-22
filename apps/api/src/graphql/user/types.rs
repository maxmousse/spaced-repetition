use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, FixedOffset};
use prisma_client::{course, user};

use crate::graphql::{course::types::Course, utils::context::unwrap_context_data};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: ID,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[ComplexObject]
impl User {
    pub async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<Course>> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .course()
            .find_many(vec![course::author_id::equals(self.id.clone().into())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}

impl From<user::Data> for User {
    fn from(value: user::Data) -> Self {
        User {
            id: value.id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
        }
    }
}

impl From<&user::Data> for User {
    fn from(value: &user::Data) -> Self {
        User {
            id: value.id.clone().into(),
            created_at: value.created_at.clone(),
            updated_at: value.updated_at.clone(),
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            email: value.email.clone(),
        }
    }
}
