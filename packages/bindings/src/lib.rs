mod msg;
mod querier;
mod query;

pub use msg::JackalMsg;
pub use querier::JackalQuerier;
pub use query::{
    FilesResponse, JackalQuery,
};

// TO DO
// Osmosis uses this type of 'signal' but am currently unsure of how it works 'behind the hood'
// Will flesh out.

// This is a signal, such that any contract that imports these helpers will only run on the
// jackal blockchain
#[no_mangle]
extern "C" fn requires_jackal() {}
