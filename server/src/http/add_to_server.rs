use rocket::response::Redirect;

#[rocket::get("/install")]
pub async fn add_to_server() -> Redirect {
    let client_id = crate::env::discord_client_id();
    Redirect::to(format!("https://discord.com/api/oauth2/authorize?client_id={}&scope=bot%20applications.commands&permissions=10240", client_id))
}
