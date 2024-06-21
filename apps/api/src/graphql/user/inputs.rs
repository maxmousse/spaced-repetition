use async_graphql::{Enum, InputObject};
use graphql_common::types::{
    pagination::Pagination, sort::SortDirection, string_filter::StringFilterInput,
};
use prisma_client::user::{OrderByParam, WhereParam};

#[derive(InputObject)]
pub struct GetUsersInput {
    filter: UserFilter,
    sort: UserSort,
    page: Pagination,
}

impl GetUsersInput {
    pub fn format(self) -> (Vec<WhereParam>, OrderByParam, Pagination) {
        (self.filter.into_where_params(), self.sort.into(), self.page)
    }
}

#[derive(Enum, Copy, Clone, PartialEq, Eq)]
pub enum UserSortProperty {
    Id,
    Email,
}

#[derive(InputObject)]
pub struct UserSort {
    property: UserSortProperty,
    direction: SortDirection,
}

impl Into<OrderByParam> for UserSort {
    fn into(self) -> OrderByParam {
        match self.property {
            UserSortProperty::Id => OrderByParam::Id(self.direction.into()),
            UserSortProperty::Email => OrderByParam::Email(self.direction.into()),
        }
    }
}

#[derive(InputObject)]
pub struct UserFilter {
    pub id: Option<StringFilterInput>,
    pub email: Option<StringFilterInput>,
}

impl UserFilter {
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
