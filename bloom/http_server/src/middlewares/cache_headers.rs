// // MiddlewareCache sets the correct headers for CDN caching
// func (server *Server) MiddlewareCache(next http.Handler) http.Handler {
// 	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
// 		w.Header().Set("Cache-Control", "public, max-age=0, s-maxage=31536000")
// 		w.Header().Set("X-Accel-Expires", "31536000")
// 		next.ServeHTTP(w, r)
// 	})
// }
