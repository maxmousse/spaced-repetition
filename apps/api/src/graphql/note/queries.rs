use async_graphql::{Context, Object, Result};
use prisma_client::note;

use graphql_common::utils::context::unwrap_context_data;
use super::types::{GetNoteInput, Note};

#[derive(Default, Clone)]
pub struct NoteQueries;

#[Object]
impl NoteQueries {
    async fn get_note(&self, ctx: &Context<'_>, input: GetNoteInput) -> Result<Option<Note>> {
        let (db, _) = unwrap_context_data(ctx);

        Ok(db
            .note()
            .find_unique(note::id::equals(input.id.into()))
            .exec()
            .await?
            .map(|n| n.into()))
    }

    async fn get_notes(&self, ctx: &Context<'_>) -> Result<Vec<Note>> {
        let (db, _) = unwrap_context_data(ctx);

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
