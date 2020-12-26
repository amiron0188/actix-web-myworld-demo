use actix_web::web;

mod api;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
       // /api/user
            .service(web::scope("/user")
                .service(api::user::register)
                .service(api::user::delete)
                .service(api::user::update)
                .service(api::user::list)
        
        )
        // /api/auth
            .service(web::scope("/auth")
                .service(api::auth::login)
        
        )
    );
}
