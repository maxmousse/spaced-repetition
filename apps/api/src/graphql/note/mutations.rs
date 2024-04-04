use async_graphql::{Context, Object, Result};
use chrono::Utc;
use prisma_client::PrismaClient;

use super::types::{CreateNoteInput, Note};

#[derive(Default, Clone)]
pub struct NoteMutations {}

#[Object]
impl NoteMutations {
    pub async fn create_note(&self, context: &Context<'_>, input: CreateNoteInput) -> Result<Note> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .create(Utc::now().into(), input.title, input.content, vec![])
            .exec()
            .await?
            .into())
    }
}
