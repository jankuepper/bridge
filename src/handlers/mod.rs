pub mod index;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    index::config(cfg);
}
