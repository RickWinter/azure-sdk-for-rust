//! All the operations the SDK supports against the Cosmos service.

#![allow(missing_docs)]

mod create_collection;
mod create_database;
mod create_document;
mod create_or_replace_attachment;
mod create_or_replace_slug_attachment;
mod create_or_replace_trigger;
mod create_or_replace_user_defined_function;
mod create_permission;
mod create_stored_procedure;
mod create_user;
mod delete_attachment;
mod delete_collection;
mod delete_database;
mod delete_document;
mod delete_permission;
mod delete_stored_procedure;
mod delete_trigger;
mod delete_user;
mod delete_user_defined_function;
mod execute_stored_procedure;
mod get_attachment;
mod get_collection;
mod get_database;
mod get_document;
mod get_partition_key_ranges;
mod get_permission;
mod get_user;
mod list_attachments;
mod list_collections;
mod list_databases;
mod list_documents;
mod list_permissions;
mod list_stored_procedures;
mod list_triggers;
mod list_user_defined_functions;
mod list_users;
mod query_documents;
mod replace_collection;
mod replace_document;
mod replace_permission;
mod replace_stored_procedure;
mod replace_user;

pub use create_collection::*;
pub use create_database::*;
pub use create_document::*;
pub use create_or_replace_attachment::*;
pub use create_or_replace_slug_attachment::*;
pub use create_or_replace_trigger::*;
pub use create_or_replace_user_defined_function::*;
pub use create_permission::*;
pub use create_stored_procedure::*;
pub use create_user::*;
pub use delete_attachment::*;
pub use delete_collection::*;
pub use delete_database::*;
pub use delete_document::*;
pub use delete_permission::*;
pub use delete_stored_procedure::*;
pub use delete_trigger::*;
pub use delete_user::*;
pub use delete_user_defined_function::*;
pub use execute_stored_procedure::*;
pub use get_attachment::*;
pub use get_collection::*;
pub use get_database::*;
pub use get_document::*;
pub use get_partition_key_ranges::*;
pub use get_permission::*;
pub use get_user::*;
pub use list_attachments::*;
pub use list_collections::*;
pub use list_databases::*;
pub use list_documents::*;
pub use list_permissions::*;
pub use list_stored_procedures::*;
pub use list_triggers::*;
pub use list_user_defined_functions::*;
pub use list_users::*;
pub use query_documents::*;
pub use replace_collection::*;
pub use replace_document::*;
pub use replace_permission::*;
pub use replace_stored_procedure::*;
pub use replace_user::*;
