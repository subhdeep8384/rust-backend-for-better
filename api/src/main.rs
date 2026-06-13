use poem::{Route, Server, get, listener::TcpListener, post};

mod handler;
mod request_inputs;
mod request_output;

use handler::website::{create_website, get_website};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}
