use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::query::{
    FilesResponse, JackalQuery
};

/// This is a helper wrapper to easily use our custom queries
pub struct JackalQuerier<'a> {
    querier: &'a QuerierWrapper<'a, JackalQuery>,
}

impl<'a> JackalQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<JackalQuery>) -> Self {
        JackalQuerier { querier }
    }

    pub fn get_files(
        &self,
        address: String,
        contents: String,
        owner: String,
        viewing_access: String,
        edit_access: String,
        tracking_number: String,
    ) -> StdResult<FilesResponse> {
        let full_denom_query = JackalQuery::Files {
            address,
            contents,
            owner,
            viewing_access,
            edit_access,
            tracking_number,
        };
        let request: QueryRequest<JackalQuery> = JackalQuery::into(full_denom_query);
        self.querier.query(&request)
    }

}
