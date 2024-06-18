use std::sync::Arc;

use password_auth;
use prisma_client::{user, PrismaClient};
use prisma_client_rust::QueryError;

/// Hash a password using the password_auth crate.
pub fn hash_password(password: &str) -> String {
    password_auth::generate_hash(password)
}

/// Check if a password is valid by comparing it to a hash.
pub fn is_password_valid(password: &str, hash: &str) -> bool {
    password_auth::verify_password(password, hash).is_ok()
}

/// Validate user credentials and return user ID if credentials are correct.
pub async fn validate_user_credentials(
    user_email: &str,
    password: &str,
    db: Arc<PrismaClient>,
) -> Result<Option<String>, QueryError> {
    let user = db
        .user()
        .find_unique(user::email::equals(user_email.to_string()))
        .exec()
        .await?;

    Ok(user.and_then(|user| {
        if is_password_valid(password, &user.password) {
            Some(user.id)
        } else {
            None
        }
    }))
}
