use async_graphql::{Context, Object, Result};
use authentication_cookie::new_authentication_cookie;
use authentication_password_strategy::{hash_password, validate_user_credentials};

use crate::graphql::utils::context::unwrap_context_data;

use super::inputs::LoginInput;

#[derive(Default, Clone)]
pub struct AuthenticationMutations {}

#[Object]
impl AuthenticationMutations {
    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<bool> {
        let (db, _) = unwrap_context_data(ctx);

        let user_id = validate_user_credentials(&input.email, &input.password, db).await?;

        if let Some(user_id) = user_id {
            let cookie = new_authentication_cookie(user_id);

            ctx.insert_http_header("Set-cookie", cookie.to_string());

            Ok(true)
        } else {
            // TODO: return a more specific error
            Ok(false)
        }
    }
}
