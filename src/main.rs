#[macro_use]
extern crate rocket;
extern crate system_shutdown;

use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};
use system_shutdown::{reboot, shutdown};

#[get("/shutdown")]
fn shutdown_handler() -> String {
    match shutdown() {
        Ok(_) => "Shutting down, bye!".into(),
        Err(error) => format!("Failed to shut down: {}", error),
    }
}

#[get("/reboot")]
fn reboot_handler() -> String {
    match reboot() {
        Ok(_) => "Rebooting, bye!".into(),
        Err(error) => format!("Failed to reboot: {}", error),
    }
}

#[get("/")]
fn index_handler() -> Template {
    Template::render("index", context! {})
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![shutdown_handler])
        .mount("/", routes![reboot_handler])
        .mount("/", routes![index_handler])
        .attach(Template::fairing())
}