// // MiddlewareAuth is a middleware which checks the `Authorizartion` header. If data is provided the
// // middleware verifies that the data is correct and then fill the context of the current request
// func (server *Server) MiddlewareAuth(next http.Handler) http.Handler {
// 	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

// 		reqCtx := r.Context()

// 		httpCtx := httputil.HTTPCtxFromCtx(r.Context())
// 		authHeader := r.Header.Get("authorization")

// 		if authHeader != "" {
// 			tokenType, token, err := server.decodeAuthorizationHeader(authHeader)
// 			if err != nil {
// 				api.SendError(w, r, http.StatusUnauthorized, err)
// 				return
// 			}

// 			if tokenType == TokenTypeBasic {
// 				currentUser, currentSession, err := server.kernelService.VerifySessionToken(reqCtx, token)
// 				if err != nil {
// 					api.SendError(w, r, http.StatusUnauthorized, err)
// 					return
// 				}
// 				httpCtx.AuthenticatedUser = &currentUser
// 				httpCtx.Session = &currentSession
// 			} else if tokenType == TokenTypeAnonymous {
// 				anonymousID, err := uuid.Parse(token)
// 				if err != nil {
// 					err = kernel.ErrInvalidSession
// 					api.SendError(w, r, http.StatusUnauthorized, err)
// 					return
// 				}
// 				httpCtx.AnonymousID = &anonymousID
// 			} else {
// 				err = kernel.ErrInvalidSession
// 				api.SendError(w, r, http.StatusUnauthorized, err)
// 				return
// 			}
// 		}

// 		next.ServeHTTP(w, r.WithContext(reqCtx))
// 	})
// }

// func (server *Server) decodeAuthorizationHeader(header string) (tokenType, token string, err error) {
// 	header = strings.TrimSpace(header)
// 	parts := strings.Split(header, " ")
// 	if len(parts) != 2 {
// 		err = kernel.ErrInvalidSession
// 		return
// 	}
// 	tokenType = strings.ToLower(parts[0])
// 	token = parts[1]
// 	return
// }
use actix_web::Result;
use actix_web::{dev, Error, FromRequest, HttpMessage, HttpRequest};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    dev::{Service, Transform},
    error::ErrorInternalServerError,
};
use futures::{
    future::{err, ok, Ready},
    Future,
};
use std::{cell::RefCell, convert::TryFrom, pin::Pin, rc::Rc, sync::Arc, task::{Context, Poll}};
use stdx::{log::error, ulid::Ulid};
use kernel::Service as KernelService;

use crate::context::Actor;

/// The header get by the middleware
pub const AUTHORIZATION_HEADER: &str = "authorization";

enum AuthorizationTokenType {
    Basic,
    Anonymous,
}
struct AuthorizationToken {
    pub token_type: AuthorizationTokenType,
    pub token: String,
}

impl TryFrom<String> for AuthorizationToken {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(AuthorizationToken{
            token_type: AuthorizationTokenType::Basic, // TODO
            token: String::new(), // TODO
        })
    }
}

/// Auth middleware.
pub struct AuthMiddleware {
    kernel_service: Arc<KernelService>,
}

impl AuthMiddleware {
    pub fn new(kernel_service: Arc<KernelService>) -> Self {
        AuthMiddleware { kernel_service }
    }
}

impl<S, B> Transform<S> for AuthMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware2<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware2 {
            service: Rc::new(RefCell::new(service)),
            kernel_service: self.kernel_service.clone(),
        })
    }
}

/// Actual actix-web middleware
pub struct AuthMiddleware2<S> {
    service: Rc<RefCell<S>>,
    kernel_service: Arc<KernelService>,
}


impl<S, B> Service for AuthMiddleware2<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.borrow_mut().poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // wrapped in block to avoid holding `Ref` (from `req.headers`) across await point
        let auth_header = {
            if let Some(auth_header) = req.headers().get(AUTHORIZATION_HEADER) {
                match auth_header.to_str() {
                    Ok(auth_header_str) => Some(auth_header_str.to_string()),
                    Err(err) => {
                        error!("middlwares/auth: converting auth header to str: {}", err);
                        None
                    }
                }
            } else {
                None
            }
        };
        let service = self.service.clone();
        let mut actor =  Actor::None;
        let kernel_service = self.kernel_service.clone();

        Box::pin(async move {
            if let Some(auth_header) = auth_header {
                if let Some(auth_token) = AuthorizationToken::try_from(auth_header).ok() {
                    actor = match auth_token.token_type {
                        AuthorizationTokenType::Basic => {
                            let user = kernel_service.decode_and_check_session_token(auth_token.token).await.ok();
                            match user {
                                Some(user) => Actor::User(user),
                                None => Actor::None,
                            }
                        }
                        AuthorizationTokenType::Anonymous => {
                            Actor::Anonymous(Ulid::new().into()) // TODO
                        }
                    };
                }
            }

            req.extensions_mut().insert(actor);

            service.borrow_mut().call(req).await
        })
    }
}

/// Actor extractor
impl FromRequest for Actor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        if let Some(actor) = req.extensions().get::<Actor>() {
            ok(actor.clone())
        } else {
            error!("middlewares/request_id: request_id is missing");
            err(ErrorInternalServerError("request_id is missing"))
        }
    }
}
