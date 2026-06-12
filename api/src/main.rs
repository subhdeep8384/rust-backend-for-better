use poem::{
    Route, Server, get, handler, listener::TcpListener, post, web::Path
};

#[handler]
async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {name}")
}
#[handler]
async fn website(Path(website_id): Path<String>) -> String {
    format!("website : {}", website_id)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Route::new()
    .at("/status/:website_id", get(website))
    .at( "/website", post(hello))
    ;

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}