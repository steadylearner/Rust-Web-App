#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate tera;

mod routes;
use crate::routes::{ static_files, get, errors };

// tera
use rocket_contrib::templates::Template;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                static_files::file,
                get::index,
            ],
        )
        .register(catchers![errors::not_found])
}

fn main() {
    rocket().launch();
}


