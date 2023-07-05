use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

// A number of Custom messages that can call into the Jackal bindings
#[cw_serde]
pub enum JackalMsg {

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

    // This is actually for the storage module, not filetree. The initial vision was for filetree and storage to each have
    // their own 'gateway' contracts. 
    // However, it may work out best that we call this contract 'jackal storage gateway' to serve as the gateway for both modules. 

    BuyStorage {
        foraddress: String,
        duration: String,
        bytes: String,
        paymentdenom: String,
    },

    // For jackaljs integration
    PostAndSign {
        account: String,
        hashparent: String,
        hashchild: String,
        contents: String,
        viewers: String,
        editors: String,
        trackingnumber: String,
        cid: String,
        payonce: bool,
    },

    DeleteAndCancel {
        hashpath: String,
        account: String,
        cids: String,
    },
}

impl JackalMsg {

    pub fn post_key(key: String) -> Self {
        JackalMsg::PostKey {
            key,
        }
    }

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

    pub fn delete_file(hashpath: String, account: String) -> Self {
        JackalMsg::DeleteFile {
            hashpath,
            account,
        }
    }

    pub fn buy_storage(foraddress: String, duration: String, bytes: String, paymentdenom: String) -> Self {
        JackalMsg::BuyStorage {
            foraddress,
            duration,
            bytes,
            paymentdenom,
        }
    }

    pub fn post_and_sign(account: String, hashparent: String, hashchild: String, contents: String, viewers: String, editors: String, trackingnumber: String, cid: String, payonce: bool) -> Self {
        JackalMsg::PostAndSign { 
            account,
            hashparent,
            hashchild,
            contents,
            viewers,
            editors,
            trackingnumber,
            cid,
            payonce,
        }
    }

    pub fn delete_and_cancel(hashpath: String, account: String, cids: String) -> Self {
        JackalMsg::DeleteAndCancel {
            hashpath,
            account,
            cids,
        }
    }
}

impl From<JackalMsg> for CosmosMsg<JackalMsg> {
    fn from(msg: JackalMsg) -> CosmosMsg<JackalMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for JackalMsg {}
