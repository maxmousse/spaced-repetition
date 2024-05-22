use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use authentication_context::{get_authentication_context, AuthenticationContext};
use authentication_cookie::get_authenticated_user_id_from_request;
use futures_util::future::LocalBoxFuture;
use prisma_client::PrismaClient;
use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};

pub struct AuthenticationMiddleware {}

impl AuthenticationMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let authenticated_user_id = get_authenticated_user_id_from_request(&req, "cookie_secret");

        Box::pin(async move {
            let db = req.app_data::<web::Data<PrismaClient>>().unwrap();
            let authentication_context =
                get_authentication_context(&db, authenticated_user_id).await?;

            req.extensions_mut()
                .insert::<Arc<AuthenticationContext>>(Arc::new(authentication_context));

            let res = service.call(req).await?;

            Ok(res)
        })
    }
}
