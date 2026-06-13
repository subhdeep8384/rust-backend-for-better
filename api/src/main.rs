use poem::{
    Route, Server, get, handler, listener::TcpListener, post, web::{Json, Path}
};

use crate::{request_inputs::CreateWebsiteInput, request_output::CreateWebsitesOutput};

pub mod request_inputs;
pub mod request_output;

#[handler]
async fn get_website(Path(website_id): Path<String>) -> String {
    format!("website : {}", website_id)
}

#[handler]
async fn create_website(Json(data) : Json<CreateWebsiteInput>) -> Json<CreateWebsitesOutput> {
    let url = data.url ;
    // presist this in the database
    let response = CreateWebsitesOutput{
        id : String::from("ID")
    } ;
    Json(response) 
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Route::new()
    .at("/website/:website_id", get(get_website))
    .at( "/website", post(create_website))
    ;

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}