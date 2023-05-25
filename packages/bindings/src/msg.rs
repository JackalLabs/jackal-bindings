use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

// A number of Custom messages that can call into the Jackal bindings
#[cw_serde]
pub enum JackalMsg {
    
    MakeRoot {
        creator: String,
        editors: String,
        viewers: String,
        trackingnumber: String,
    },
}

impl JackalMsg {

    pub fn make_root(creator: String, editors: String, viewers: String, trackingnumber: String) -> Self {
        JackalMsg::MakeRoot {
            creator,
            editors,
            viewers,
            trackingnumber,
        }
    }
}

impl From<JackalMsg> for CosmosMsg<JackalMsg> {
    fn from(msg: JackalMsg) -> CosmosMsg<JackalMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for JackalMsg {}
