pub mod direct;
pub mod fish;
pub mod indirect;
pub mod locked;
pub mod subset;

pub use direct::{hidden_single, naked_single};
pub use fish::x_wing;
pub use indirect::{hidden_pair, naked_pair};
pub use locked::{locked_claiming, locked_pointing};
pub use subset::{hidden_triple, naked_triple};
