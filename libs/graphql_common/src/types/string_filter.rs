use async_graphql::InputObject;
use prisma_client::read_filters::StringFilter;

#[derive(InputObject)]
pub struct StringFilterInput {
    equals: Option<String>,
    in_list: Option<Vec<String>>,
    not_in_list: Option<Vec<String>>,
    contains: Option<String>,
    starts_with: Option<String>,
    ends_with: Option<String>,
}

impl StringFilterInput {
    pub fn into_filters(self) -> Vec<StringFilter> {
        let mut filters = Vec::new();

        if let Some(value) = self.equals {
            filters.push(StringFilter::Equals(value));
        }
        if let Some(values) = self.in_list {
            filters.push(StringFilter::InVec(values));
        }
        if let Some(values) = self.not_in_list {
            filters.push(StringFilter::NotInVec(values));
        }
        if let Some(value) = self.contains {
            filters.push(StringFilter::Contains(value));
        }
        if let Some(value) = self.starts_with {
            filters.push(StringFilter::StartsWith(value));
        }
        if let Some(value) = self.ends_with {
            filters.push(StringFilter::EndsWith(value));
        }

        filters
    }
}
