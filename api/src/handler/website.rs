use poem::{
    handler,
    web::{Json, Path},
};

use crate::{
    request_inputs::CreateWebsiteInput,
    request_output::CreateWebsitesOutput,
};

use store::Store;

#[handler]
pub async fn get_website(
    Path(website_id): Path<String>
) -> String {

    let s = Store {};
    

    format!("website : {}", website_id)
}

#[handler]
pub async fn create_website(
    Json(data): Json<CreateWebsiteInput>
) -> Json<CreateWebsitesOutput> {

    let s = Store {};

    let id = s.create_website();

    Json(CreateWebsitesOutput {
        id
    })
}