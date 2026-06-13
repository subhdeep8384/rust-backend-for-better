use serde::{Deserialize, Serialize};

#[derive(Serialize , Deserialize)]
pub struct CreateWebsitesOutput{
    pub id : String
}