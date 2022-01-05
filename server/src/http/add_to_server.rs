use rocket::response::Redirect;
use std::env;

#[rocket::get("/")]
pub async fn add_to_server() -> Redirect {
    let client_id = env::var("DISCORD_CLIENT_ID").unwrap();
    Redirect::to(format!("https://discord.com/api/oauth2/authorize?client_id={}&scope=bot&permissions=10240&scope=applications.commands", client_id))
}
