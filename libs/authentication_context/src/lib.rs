use prisma_client::{user, PrismaClient};
use std::error::Error;

/// Represents the authentication state during a request
///
/// User can be either authenticated or unauthenticated
#[derive(Debug, Clone)]
pub enum AuthenticationContext {
    Authenticated(AuthenticatedUser),
    Unauthenticated,
}

/// Data related to an authenticated user
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

/// Given a user id, build the associated authentication context by reading the database
pub async fn get_authentication_context(
    db: &PrismaClient,
    user_id: Option<String>,
) -> Result<AuthenticationContext, Box<dyn Error>> {
    match user_id {
        Some(user_id) => find_authenticated_user(db, user_id).await,
        // TODO: This case means that the user is considered authenticated by the http layer (a cookie was received) but the user was not found in the database
        // This could be a sign of a deleted user or a cookie that was not cleaned up
        // This should probably be handled properly by explicitly dropping the cookie
        None => Ok(AuthenticationContext::Unauthenticated),
    }
}

/// Find a user by id and return an authenticated user if found
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
