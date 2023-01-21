use actix_web::web::ServiceConfig;

mod v0;

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(v0::service());
}
