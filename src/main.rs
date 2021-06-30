#[macro_use]
extern crate rocket;
extern crate system_shutdown;

use system_shutdown::{reboot, shutdown};

#[get("/shutdown")]
fn shutdown_handler() -> std::string::String {
    match shutdown() {
        Ok(_) => "Shutting down, bye!".into(),
        Err(error) => format!("Failed to shut down: {}", error),
    }
}

#[get("/reboot")]
fn reboot_handler() -> std::string::String {
    match reboot() {
        Ok(_) => "Rebooting, bye!".into(),
        Err(error) => format!("Failed to reboot: {}", error),
    }
}

#[get("/")]
fn index_handler() -> &'static str {
    "Available options: /shutdown /reboot"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![shutdown_handler])
        .mount("/", routes![reboot_handler])
        .mount("/", routes![index_handler])
}