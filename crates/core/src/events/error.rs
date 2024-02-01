use http::StatusCode;
use matrix::events::{exports::ruma_common::OwnedRoomId, relation::RelationType};
use thiserror::Error;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum BoardErrorCode {
    #[error("Failed to parse BoardId.")]
    MalformedBoardId,
    #[error("Failed to parse EventId of relation.")]
    MalformedRelationId,
    #[error("Failed to parse content body.")]
    MalformedContent(#[from] serde_json::Error),
    #[error("You are not a member of the board.")]
    NoMembership(OwnedRoomId),
    #[error("You provided an empty or incorrect relation.")]
    WrongRelation(Option<RelationType>),
}

impl HttpStatusCode for BoardErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            BoardErrorCode::WrongRelation(_)
            | BoardErrorCode::MalformedBoardId
            | BoardErrorCode::MalformedRelationId
            | BoardErrorCode::MalformedContent(_) => StatusCode::BAD_REQUEST,
            BoardErrorCode::NoMembership(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            BoardErrorCode::WrongRelation(_)
            | BoardErrorCode::MalformedBoardId
            | BoardErrorCode::MalformedRelationId
            | BoardErrorCode::MalformedContent(_) => "BAD_REQUEST",
            BoardErrorCode::NoMembership(_) => "FORBIDDEN",
        }
    }
}
