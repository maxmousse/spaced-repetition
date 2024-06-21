use async_graphql::InputObject;
use graphql_common::types::string_filter::StringFilterInput;
use prisma_client::user::WhereParam;

#[derive(InputObject)]
pub struct GetUsersInput {
    // Filters
    pub id: Option<StringFilterInput>,
    pub email: Option<StringFilterInput>,
}

impl GetUsersInput {
    pub fn into_where_params(self) -> Vec<WhereParam> {
        let mut params = Vec::new();

        if let Some(id) = self.id {
            params.extend(
                id.into_filters()
                    .into_iter()
                    .map(WhereParam::Id)
                    .collect::<Vec<WhereParam>>(),
            );
        }
        if let Some(email) = self.email {
            params.extend(
                email
                    .into_filters()
                    .into_iter()
                    .map(WhereParam::Email)
                    .collect::<Vec<WhereParam>>(),
            );
        }

        params
    }
}
