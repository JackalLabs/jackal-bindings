use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CustomQuery};

// TO DO
// It may be the case that only 'address' and 'owner' are needed. Consider changing.
#[cw_serde]
#[derive(QueryResponses)]
pub enum JackalQuery {

    #[returns(FilesResponse)]
    Files {
        address: String,
        contents: String,
        owner: String,
        viewing_access: String,
        edit_access: String,
        tracking_number: String,
    },
}

impl CustomQuery for JackalQuery {}

impl JackalQuery {
    
    pub fn get_files(
        address: String,
        contents: String,
        owner: String,
        viewing_access: String,
        edit_access: String,
        tracking_number: String,
    ) -> Self {
        JackalQuery::Files { 
            address: address, 
            contents: contents, 
            owner: owner,
            viewing_access: viewing_access,
            edit_access: edit_access,
            tracking_number: tracking_number }
    }

}

#[cw_serde]
pub struct FilesResponse {
    pub file: String,
}
