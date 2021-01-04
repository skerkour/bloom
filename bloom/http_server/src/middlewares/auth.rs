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
