use poem::{
    Route,
    Server,
    get,
    post,
    listener::TcpListener,
};

mod request_inputs;
mod request_output;
mod handler;

use handler::website::{
    get_website,
    create_website,
};

#[tokio::main]
async fn main()
-> Result<(), Box<dyn std::error::Error>> {

    let app = Route::new()
        .at(
            "/website/:website_id",
            get(get_website)
        )
        .at(
            "/website",
            post(create_website)
        );

    Server::new(
        TcpListener::bind("127.0.0.1:3000")
    )
    .run(app)
    .await?;

    Ok(())
}