pub mod direct;
pub mod fish;
pub mod indirect;
pub mod locked;
pub mod subset;
pub mod unique;
pub mod wing;

pub use direct::{hidden_single, naked_single};
pub use fish::{jellyfish, swordfish, x_wing};
pub use indirect::{hidden_pair, naked_pair};
pub use locked::{locked_claiming, locked_pointing};
pub use subset::{hidden_quad, hidden_triple, naked_quad, naked_triple};
pub use unique::{
    bug_plus_one, unique_rectangle_type1, unique_rectangle_type2, unique_rectangle_type3,
    unique_rectangle_type4,
};
pub use wing::{xy_wing, xyz_wing};
