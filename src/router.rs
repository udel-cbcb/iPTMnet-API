use actix_web::*;
use controller;
use actix_web::middleware::cors::Cors;

pub fn init_routes(application: App<super::State>) -> App<super::State> {
        return Cors::for_app(application)
            .send_wildcard()
            .resource("/",|r| r.method(http::Method::GET).f(controller::get_status_controller))
            .resource("/{id}/info", |r| r.method(http::Method::GET).f(controller::get_info_controller))
            .resource("/search",|r|r.method(http::Method::GET).f(controller::search_controller))
            .resource("/browse",|r|r.method(http::Method::GET).f(controller::browse_controller))
            .resource("/{id}/substrate", |r| r.method(http::Method::GET).f(controller::substrate_controller))
            .resource("/{id}/proteoforms",|r|r.method(http::Method::GET).f(controller::proteoforms_controller))
            .resource("/{id}/proteoformsppi",|r|r.method(http::Method::GET).f(controller::proteoformsppi_controller))
            .resource("/{id}/ptmppi",|r|r.method(http::Method::GET).f(controller::ptmppi_controller))
            .resource("/batch_ptm_enzymes",|r|r.method(http::Method::POST).f(controller::batch_ptm_enzymes_controller))
            .resource("/batch_ptm_ppi",|r|r.method(http::Method::POST).f(controller::batch_ptm_ppi_controller))
            .resource("/statistics",|r|r.method(http::Method::GET).f(controller::get_statistics_controller))
            .resource("/{id}/msa",|r|r.method(http::Method::GET).f(controller::get_msa_controller))
            .resource("/{id}/variants",|r|r.method(http::Method::GET).f(controller::get_variants))
            .register();
}