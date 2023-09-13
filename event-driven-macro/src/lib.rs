use aggregate::{render_aggregate_token, render_entity_token};

use message::{find_identifier, render_event_visibility, render_message_token};
// use outbox::render_outbox_token;
use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn};

#[macro_use]
extern crate quote;
mod aggregate;
mod dependency;
mod error;
mod message;
#[proc_macro_derive(Message, attributes(internally_notifiable, externally_notifiable, identifier))]
pub fn message_derive(attr: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(attr.clone()).unwrap();
	let propagatability = render_event_visibility(&ast);
	let identifier = find_identifier(&ast);

	render_message_token(&ast, propagatability, identifier)
}

#[proc_macro_derive(Aggregate)]
pub fn aggregate_derive(attr: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(attr.clone()).unwrap();

	render_aggregate_token(&ast)
}

/// Define a Application Error type that can be used in the event-driven-library.
///
/// Before deriving this, you must impl `Debug`traits.
///
/// This macro can be only used in enum.
///
/// ## Attributes
///
/// - `#[crates(...)]` - Specify the name of root of event-driven-library crate. (Default is `event_driven_library`)
/// - `#[error]` - Specify the error matching for `BaseError::StopSentinel`.
/// - `#[error_with_event]` - Specify the error matching for `BaseError::StopSentinelWithEvent`.
/// - `#[database_error]` - Specify the error matching for `BaseError::DatabaseError`.
/// - `#[service_error]` - Specify the error matching for `BaseError::ServiceError`.
#[proc_macro_derive(ApplicationError, attributes(error, error_with_event, database_error, service_error, crates))]
pub fn error_derive(attr: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(attr).unwrap();

	error::render_error_token(&ast)
}

#[proc_macro_derive(Entity)]
pub fn entity_derive(attr: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(attr.clone()).unwrap();

	render_entity_token(&ast)
}

#[proc_macro_derive(Command)]
pub fn command_derive(attr: TokenStream) -> TokenStream {
	let ast: DeriveInput = syn::parse(attr.clone()).unwrap();
	let name = ast.ident;

	quote!(
		impl Command for #name{}
	)
	.into()
}

#[proc_macro_attribute]
pub fn dependency(_: TokenStream, input: TokenStream) -> TokenStream {
	let ast: ItemFn = syn::parse_macro_input!(input as ItemFn);
	dependency::register_dependency(ast)
}
