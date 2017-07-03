pub mod authorization_token;
pub mod client;
pub mod database;

mod request_response;

pub mod collection;
pub mod document;
pub mod list_documents;


use std::fmt;
use azure::core::enumerations;
use azure::core::parsing::FromStringOptional;
use std::str::FromStr;
use azure::core::errors::TraversingError;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);
