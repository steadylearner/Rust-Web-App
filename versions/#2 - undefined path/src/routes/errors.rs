use rocket::Request;
use rocket_contrib::templates::Template;

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("errors/not-found", &map)
}

// https://api.rocket.rs/rocket_contrib/struct.Template.html

