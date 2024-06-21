use async_graphql::Enum;
use prisma_client_rust::Direction;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Into<Direction> for SortDirection {
    fn into(self) -> Direction {
        match self {
            Self::Asc => Direction::Asc,
            Self::Desc => Direction::Desc,
        }
    }
}
