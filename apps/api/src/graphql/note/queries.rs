use async_graphql::{Context, Object, Result};
use prisma_client::PrismaClient;

use super::types::Note;

#[derive(Default, Clone)]
pub struct NoteQueries;

#[Object]
impl NoteQueries {
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
