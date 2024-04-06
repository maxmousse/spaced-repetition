use async_graphql::{Context, Object, Result};
use prisma_client::{note, PrismaClient};

use super::types::{GetNoteInput, Note};

#[derive(Default, Clone)]
pub struct NoteQueries;

#[Object]
impl NoteQueries {
    async fn get_note(&self, context: &Context<'_>, input: GetNoteInput) -> Result<Option<Note>> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .find_unique(note::id::equals(input.id.into()))
            .exec()
            .await?
            .map(|n| n.into()))
    }

    async fn get_notes(&self, context: &Context<'_>) -> Result<Vec<Note>> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect())
    }
}
