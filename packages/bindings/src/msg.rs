use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

// A number of Custom messages that can call into the Jackal bindings
#[cw_serde]
pub enum JackalMsg {
    
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
    }
}

impl JackalMsg {

    pub fn make_root(editors: String, viewers: String, trackingnumber: String) -> Self {
        JackalMsg::MakeRoot {
            editors,
            viewers,
            trackingnumber,
        }
    }

    pub fn post_files(account: String, hashparent: String, hashchild: String, contents: String, viewers: String, editors: String, trackingnumber: String) -> Self {
        JackalMsg::PostFiles { 
            account,
            hashparent,
            hashchild,
            contents,
            viewers,
            editors,
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
