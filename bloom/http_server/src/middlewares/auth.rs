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
// 				currentUser, currentSession, err := server.kernel::Service.VerifySessionToken(reqCtx, token)
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
use actix_web::dev::{Service, Transform};
use actix_web::Result;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{Error, HttpMessage};
use kernel::Actor;
use kernel::Error as kernelError;
use std::{
    cell::RefCell,
    convert::TryFrom,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
};
use stdx::futures::{
    future::{ok, Ready},
    Future,
};
use stdx::log::error;

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
    type Error = kernelError;

    fn try_from(header_value: String) -> Result<Self, Self::Error> {
        let parts: Vec<String> = header_value.trim().split(' ').map(|part| part.to_string()).collect();
        if parts.len() != 2 {
            return Err(kernelError::InvalidArgument(String::from(
                "Invalid Authorization token",
            )));
        }

        let token_type = match parts[0].to_lowercase().as_str() {
            "basic" => AuthorizationTokenType::Basic,
            "anonymous" => AuthorizationTokenType::Anonymous,
            _ => {
                return Err(kernelError::InvalidArgument(String::from(
                    "Invalid Authorization token",
                )))
            }
        };

        Ok(AuthorizationToken {
            token_type,
            token: parts[1].clone(),
        })
    }
}

/// Auth middleware.
pub struct AuthMiddleware {
    kernel_service: Arc<kernel::Service>,
}

impl AuthMiddleware {
    pub fn new(kernel_service: Arc<kernel::Service>) -> Self {
        AuthMiddleware {
            kernel_service,
        }
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
    kernel_service: Arc<kernel::Service>,
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
        let mut actor = Actor::None;
        let kernel_service = self.kernel_service.clone();

        Box::pin(async move {
            if let Some(auth_header) = auth_header {
                match AuthorizationToken::try_from(auth_header) {
                    Ok(auth_token) => {
                        actor = match auth_token.token_type {
                            AuthorizationTokenType::Basic => {
                                match kernel_service.decode_and_validate_session_token(auth_token.token).await {
                                    Ok((user, session)) => Actor::User {
                                        user,
                                        session,
                                    },
                                    Err(err) => {
                                        error!("middlewares/auth: decoding session token: {}", err);
                                        Actor::None
                                    }
                                }
                            }
                            AuthorizationTokenType::Anonymous => {
                                match kernel_service
                                    .decode_and_validate_anonymous_token(auth_token.token)
                                    .await
                                {
                                    Ok(anonymous_id) => Actor::Anonymous(anonymous_id),
                                    Err(err) => {
                                        error!("middlewares/auth: decoding anonymous token: {}", err);
                                        Actor::None
                                    }
                                }
                            }
                        };
                    }
                    Err(err) => {
                        error!("middlewares/auth: decoding auth token: {}", err);
                    }
                }
            }

            req.extensions_mut().insert(actor);

            // Ensure `borrow_mut()` and `.await` are on separate lines or else a panic occurs.
            let call_future = service.borrow_mut().call(req);
            call_future.await
        })
    }
}
