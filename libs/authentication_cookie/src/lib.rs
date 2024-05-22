use actix_web::{cookie::Cookie, dev::ServiceRequest};
const AUTHENTICATION_COOKIE_NAME: &str = "authenticated_user";

/// Given a request, return the id of the user that is authenticated.
pub fn get_authenticated_user_id_from_request(
    req: &ServiceRequest,
    // TODO: remove this parameter and use a secure way to read the secret
    _cookie_secret: &str,
) -> Option<String> {
    let cookie = req.cookie(AUTHENTICATION_COOKIE_NAME);

    // TODO: handle cookie encryption and anti-tampering

    cookie.map(|c| c.value().to_string())
}

/// Build a new authentication cookie with the given user id.
pub fn new_authentication_cookie(user_id: String) -> Cookie<'static> {
    // TODO: add cookie encryption and anti-tampering
    Cookie::build(AUTHENTICATION_COOKIE_NAME, user_id)
        .secure(true)
        .http_only(true)
        .finish()
}
