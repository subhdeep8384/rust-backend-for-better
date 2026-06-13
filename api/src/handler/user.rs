use poem::{
    handler ,
    web::{Json , Path}
};
use serde::{Deserialize , Serialize} ;

#[derive(Serialize , Deserialize)]
struct CreateUserInput {
    name : String ,
    email : String 
}
#[derive(Serialize , Deserialize)]
struct  CreateUserOutput {
    id : String ,
}


use crate::{
    request_inputs::CreateWebsiteInput,
    request_output::CreateWebsitesOutput
};
use store::Store;

#[handler]
pub async fn create_user(
    Json(data ) : Json<CreateUserInput>
) -> Json<CreateUserOutput> {
    let s = Store{} ;
    let user = s.create_user(data.name , data.email);
    Json(CreateUserOutput{
        id : user ,
    })
}

#[handler]
pub async fn getUser(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput>{
    let s = Store{} ;
    let user = s.getUser(data.name, data.email) ;
    Json(CreateUserOutput { id: user })
}