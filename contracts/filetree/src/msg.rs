use cosmwasm_schema::{cw_serde, QueryResponses};
use jackal_bindings::FilesResponse;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {

    PostKey {
        key: String,
    },

    MakeRoot {
        editors: String,
        viewers: String,
        trackingnumber: String,
    },

    PostFiles {
        account: String,
        hashparent: String, 
        hashchild: String,
        contents: String,
        viewers: String,
        editors: String,
        trackingnumber: String,
    },

    DeleteFile {
        hashpath: String,
        account: String,
    },

    BuyStorage {
        foraddress: String,
        duration: String,
        bytes: String,
        paymentdenom: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(FilesResponse)]
    GetFiles {
        address: String,
        contents: String,
        owner: String,
        viewing_access: String,
        edit_access: String,
        tracking_number: String,
    },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetFilesResponse {
    pub files: String,
}
