use actix_web::web;
mod path;
mod auth;

pub fn views_factory(app: &mut web::ServiceConfig) {
	let loggin_status: bool = true;
	auth::auth_factory(app, loggin_status);
}