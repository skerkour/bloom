// we allow unconditional_recursion because the lint seems to bug when resolvers methods call the object's
// methods
#[allow(unconditional_recursion)]
use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;
use stdx::log::info;

mod context;
mod middlewares;

pub mod api;
pub use context::{RequestContext, ServerContext};

async fn route_index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("public/index.html")?)
}

pub async fn run(kernel_service: Arc<kernel::Service>) -> Result<(), ::kernel::Error> {
    let config = kernel_service.config();
    let context = Arc::new(ServerContext {
        kernel_service: kernel_service.clone(),
    });

    let endpoint = format!("0.0.0.0:{}", config.http.port);
    info!("Starting HTTP server. endpoint={:?}", &endpoint);
    HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&context))
            .wrap(middlewares::AuthMiddleware::new(kernel_service.clone()))
            .wrap(middleware::Compress::default())
            .wrap(middlewares::RequestIdMiddleware)
            .wrap(middlewares::SecurityHeadersMiddleware::new(Arc::clone(&config)))
            .wrap(middlewares::CacheHeadersMiddleware)
            .service(
                web::scope("/api")
                    .wrap(middlewares::NoCacheHeadersMiddleware)
                    .service(web::resource("").route(web::get().to(api::index)))
                    // kernel
                    .service(
                        web::scope("/kernel")
                            .service(
                                web::scope("/commands")
                                    // User
                                    .service(
                                        web::resource("/register")
                                            .route(web::post().to(api::kernel::commands::register)),
                                    )
                                    .service(
                                        web::resource("/complete_registration")
                                            .route(web::post().to(api::kernel::commands::complete_registration)),
                                    )
                                    .service(
                                        web::resource("/sign_in").route(web::post().to(api::kernel::commands::sign_in)),
                                    )
                                    .service(
                                        web::resource("/complete_sign_in")
                                            .route(web::post().to(api::kernel::commands::complete_sign_in)),
                                    )
                                    .service(
                                        web::resource("/revoke_session")
                                            .route(web::post().to(api::kernel::commands::revoke_session)),
                                    )
                                    .service(
                                        web::resource("/verify_email")
                                            .route(web::post().to(api::kernel::commands::verify_email)),
                                    )
                                    .service(
                                        web::resource("/delete_my_account")
                                            .route(web::post().to(api::kernel::commands::delete_my_account)),
                                    )
                                    .service(
                                        web::resource("/complete_two_fa_setup")
                                            .route(web::post().to(api::kernel::commands::complete_two_fa_setup)),
                                    )
                                    .service(
                                        web::resource("/setup_two_fa")
                                            .route(web::post().to(api::kernel::commands::setup_two_fa)),
                                    )
                                    .service(
                                        web::resource("/disable_two_fa")
                                            .route(web::post().to(api::kernel::commands::disable_two_fa)),
                                    )
                                    .service(
                                        web::resource("/complete_two_fa_challenge")
                                            .route(web::post().to(api::kernel::commands::complete_two_fa_challenge)),
                                    )
                                    .service(
                                        web::resource("/update_my_profile")
                                            .route(web::post().to(api::kernel::commands::update_my_profile)),
                                    )
                                    // Group
                                    .service(
                                        web::resource("/create_group")
                                            .route(web::post().to(api::kernel::commands::create_group)),
                                    )
                                    .service(
                                        web::resource("/delete_group")
                                            .route(web::post().to(api::kernel::commands::delete_group)),
                                    )
                                    .service(
                                        web::resource("/update_group_profile")
                                            .route(web::post().to(api::kernel::commands::update_group_profile)),
                                    )
                                    .service(
                                        web::resource("/quit_group")
                                            .route(web::post().to(api::kernel::commands::quit_group)),
                                    )
                                    .service(
                                        web::resource("/invite_people_in_group")
                                            .route(web::post().to(api::kernel::commands::invite_people_in_group)),
                                    )
                                    .service(
                                        web::resource("/accept_group_invitation")
                                            .route(web::post().to(api::kernel::commands::accept_group_invitation)),
                                    )
                                    .service(
                                        web::resource("/decline_group_invitation")
                                            .route(web::post().to(api::kernel::commands::decline_group_invitation)),
                                    )
                                    .service(
                                        web::resource("/cancel_group_invitation")
                                            .route(web::post().to(api::kernel::commands::cancel_group_invitation)),
                                    )
                                    .service(
                                        web::resource("/remove_member_from_group")
                                            .route(web::post().to(api::kernel::commands::remove_member_from_group)),
                                    ),
                            )
                            .service(
                                web::scope("/queries").service(
                                    web::resource("/signed_storage_url")
                                        .route(web::post().to(api::kernel::queries::signed_storage_url)),
                                ),
                            ),
                    )
                    // files
                    .service(
                        web::scope("/files")
                            .service(
                                web::scope("/commands")
                                    .service(
                                        web::resource("/move_files_to_trash")
                                            .route(web::post().to(api::files::commands::move_files_to_trash)),
                                    )
                                    .service(
                                        web::resource("/restore_files_from_trash")
                                            .route(web::post().to(api::files::commands::restore_files_from_trash)),
                                    )
                                    .service(
                                        web::resource("/empty_trash")
                                            .route(web::post().to(api::files::commands::empty_trash)),
                                    )
                                    .service(
                                        web::resource("/move_files")
                                            .route(web::post().to(api::files::commands::move_files)),
                                    )
                            )
                            .service(
                                web::scope("/queries")
                                    .service(web::resource("/file").route(web::post().to(api::files::queries::file)))
                                    .service(web::resource("/trash").route(web::post().to(api::files::queries::trash))),
                            ),
                    )
                    .default_service(
                        // 404 for GET request
                        web::resource("").to(api::p404),
                    ),
            )
            .service(
                // serve webapp
                actix_files::Files::new("/", &config.http.public_directory)
                    .index_file("index.html")
                    .prefer_utf8(true)
                    .default_handler(web::route().to(route_index)),
            )
    })
    .bind(endpoint)?
    .run()
    .await?;

    Ok(())
}
