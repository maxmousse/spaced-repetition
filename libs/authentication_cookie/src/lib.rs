use actix_web::{cookie::Cookie, HttpRequest};
const AUTHENTICATION_COOKIE_NAME: &str = "authenticated_user";

/// Given a request, return the id of the user that is authenticated.
pub fn get_authenticated_user_id_from_request(
    req: &HttpRequest,
    _cookie_secret: &str,
) -> Option<String> {
    let cookie = req.cookie(AUTHENTICATION_COOKIE_NAME);

    cookie.map(|c| c.value().to_string())
}

/// Build a new authentication cookie with the given user id.
pub fn new_authentication_cookie(user_id: String) -> Cookie<'static> {
    Cookie::build(AUTHENTICATION_COOKIE_NAME, user_id)
        .secure(true)
        .http_only(true)
        .finish()
}
