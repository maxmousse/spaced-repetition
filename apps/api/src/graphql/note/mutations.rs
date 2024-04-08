use async_graphql::{Context, Object, Result};
use chrono::Utc;
use prisma_client::{note, PrismaClient};

use super::types::{CreateNoteInput, Note, UpdateNoteInput};

#[derive(Default, Clone)]
pub struct NoteMutations {}

#[Object]
impl NoteMutations {
    pub async fn create_note(&self, context: &Context<'_>, input: CreateNoteInput) -> Result<Note> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .create_unchecked(
                Utc::now().into(),
                input.title,
                input.content,
                input.index,
                input.course_id,
                input.section_id,
                vec![],
            )
            .exec()
            .await?
            .into())
    }

    pub async fn update_note(&self, context: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .update_unchecked(
                note::id::equals(input.id.into()),
                vec![
                    note::updated_at::set(Utc::now().into()),
                    note::title::set(input.title),
                    note::content::set(input.content),
                ],
            )
            .exec()
            .await?
            .into())
    }

    pub async fn delete_note(&self, context: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let db = context.data::<PrismaClient>().unwrap();

        Ok(db
            .note()
            .delete(note::id::equals(input.id.into()))
            .exec()
            .await?
            .into())
    }
}
