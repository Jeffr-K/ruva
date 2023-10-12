pub use paste::paste;
pub mod message;
pub mod messagebus;
pub mod outbox;
pub mod repository;
pub mod responses;
pub mod unit_of_work;
pub mod prelude {
	pub use crate::message::*;
	pub use crate::messagebus::*;
	pub use crate::outbox::{IOutBox, OutBox};
	pub use crate::repository::REventManager;
	pub use crate::responses::*;
	pub use crate::unit_of_work::{Executor, UnitOfWork};
	pub use async_trait::async_trait;
	pub use hashbrown::HashMap as HandlerMapper;
	pub use paste::paste;
	pub use serde::{Deserialize, Serialize};
}

pub mod event_macros {
	// pub use crate::init_command_handler;
	// pub use crate::init_event_handler;
	pub use crate::prepare_bulk_insert;
}