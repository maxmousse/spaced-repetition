use prisma_client::{user, PrismaClient};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum AuthenticationContext {
    Authenticated(AuthenticatedUser),
    Unauthenticated,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

pub async fn get_authentication_context(
    db: &PrismaClient,
    user_id: Option<String>,
) -> Result<AuthenticationContext, Box<dyn Error>> {
    match user_id {
        Some(user_id) => find_authenticated_user(db, user_id).await,
        None => Ok(AuthenticationContext::Unauthenticated),
    }
}

async fn find_authenticated_user(
    db: &PrismaClient,
    user_id: String,
) -> Result<AuthenticationContext, Box<dyn Error>> {
    let user = db
        .user()
        .find_unique(user::UniqueWhereParam::IdEquals(user_id.into()))
        .exec()
        .await?;

    match user {
        Some(user) => Ok(AuthenticationContext::Authenticated(AuthenticatedUser {
            id: user.id,
        })),
        None => Ok(AuthenticationContext::Unauthenticated),
    }
}
