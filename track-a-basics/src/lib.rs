//! Stellar 14 bug hunt, Track A.
//!
//! A tiny wallet and payments library with planted bugs.
//! `types.rs` is correct. Every bug lives in its own file under `src/bugs/`.

pub mod types;

// Compile bugs. Each one is switched on with its own feature, for example `--features bug01`.
#[cfg(feature = "bug01")]
#[path = "bugs/bug01_move.rs"]
pub mod bug01_move;
#[cfg(feature = "bug02")]
#[path = "bugs/bug02_borrow.rs"]
pub mod bug02_borrow;
#[cfg(feature = "bug03")]
#[path = "bugs/bug03_match.rs"]
pub mod bug03_match;
#[cfg(feature = "bug04")]
#[path = "bugs/bug04_types.rs"]
pub mod bug04_types;
#[cfg(feature = "bug05")]
#[path = "bugs/bug05_trait.rs"]
pub mod bug05_trait;
#[cfg(feature = "bug06")]
#[path = "bugs/bug06_option.rs"]
pub mod bug06_option;

// Logic bugs. These compile fine, but their tests fail. Plain `cargo test` runs them.
#[path = "bugs/bug07_paging.rs"]
pub mod bug07_paging;
#[path = "bugs/bug08_compare.rs"]
pub mod bug08_compare;
#[path = "bugs/bug09_withdraw.rs"]
pub mod bug09_withdraw;
#[path = "bugs/bug10_fees.rs"]
pub mod bug10_fees;
#[path = "bugs/bug11_parse.rs"]
pub mod bug11_parse;
#[path = "bugs/bug12_xlm.rs"]
pub mod bug12_xlm;
