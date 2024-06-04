use async_graphql::{Context, Object, Result};
use authentication_cookie::new_authentication_cookie;

#[derive(Default, Clone)]
pub struct AuthenticationMutations {}

#[Object]
impl AuthenticationMutations {
    pub async fn login(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement a real login with a password strategy
        let cookie = new_authentication_cookie("user_id".to_string());

        ctx.insert_http_header("Set-cookie", cookie.to_string());

        Ok(true)
    }
}
