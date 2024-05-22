use std::sync::Arc;

use async_graphql::Context;
use authentication_context::AuthenticationContext;
use prisma_client::PrismaClient;

pub type ContextData = (Arc<PrismaClient>, Arc<AuthenticationContext>);

/// Unwraps the PrismaClient and AuthenticationContext from the graphql context.
pub fn unwrap_context_data(ctx: &Context<'_>) -> ContextData {
    let db = ctx.data::<Arc<PrismaClient>>().unwrap();
    let auth_ctx = ctx.data::<Arc<AuthenticationContext>>().unwrap();

    (db.clone(), auth_ctx.clone())
}
