use async_graphql::{Context, Object, Result};
use chrono::Utc;
use prisma_client::note;

use crate::graphql::utils::context::unwrap_context_data;

use super::types::{CreateNoteInput, Note, UpdateNoteInput};

#[derive(Default, Clone)]
pub struct NoteMutations {}

#[Object]
impl NoteMutations {
    pub async fn create_note(&self, ctx: &Context<'_>, input: CreateNoteInput) -> Result<Note> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .note()
            .create_unchecked(
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

    pub async fn update_note(&self, ctx: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let (db, _) = unwrap_context_data(ctx);

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

    pub async fn delete_note(&self, ctx: &Context<'_>, input: UpdateNoteInput) -> Result<Note> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .note()
            .delete(note::id::equals(input.id.into()))
            .exec()
            .await?
            .into())
    }
}
