use crate::handlers::test_handler;
use crate::middlewares::auth_middleware;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dashboard")
            .route("/test", web::get().to(test_handler::test_handler))
            .wrap(auth_middleware::AuthMiddleware),
    );
}
