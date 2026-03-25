pub mod direct;
pub mod indirect;
pub mod locked;

pub use direct::{hidden_single, naked_single};
pub use indirect::{hidden_pair, naked_pair};
pub use locked::{locked_claiming, locked_pointing};
