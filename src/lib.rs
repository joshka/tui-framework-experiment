//! # tui-framework-experiment
//!
//! [![Crates.io Badge]][Crate] [![License Badge]](#license) [![Docs.rs Badge]][API Docs]<br>
//! [![Deps.rs Badge]][Dependencies] [![Codecov.io Badge]][Coverage] [![Discord Badge]][Ratatui
//! Discord]
//!
//! `tui-framework-experiment` is a Rust crate with extra widgets for [Ratatui].
//!
//! ## Installation
//!
//! ```shell
//! cargo add tui-framework-experiment
//! ```
//!
//! ## Usage
//!
//! ```rust
//! // TODO: Add usage examples
//! ```
//!
//! ## Example
//!
//! ![Button](https://vhs.charm.sh/vhs-MSE5G9byLklG23xdJwbqR.gif)
//!
//! [Crates.io Badge]: https://img.shields.io/crates/v/tui-framework-experiment?logo=rust&style=for-the-badge
//! [License Badge]: https://img.shields.io/crates/l/tui-framework-experiment?style=for-the-badge
//! [Docs.rs Badge]: https://img.shields.io/docsrs/tui-framework-experiment?logo=rust&style=for-the-badge
//! [Deps.rs Badge]:
//!     https://deps.rs/repo/github/joshka/tui-framework-experiment/status.svg?style=for-the-badge
//! [Codecov.io Badge]:
//!     https://img.shields.io/codecov/c/github/joshka/tui-framework-experiment?logo=codecov&style=for-the-badge&token=BAQ8SOKEST
//! [Discord Badge]:
//!     https://img.shields.io/discord/1070692720437383208?label=ratatui+discord&logo=discord&style=for-the-badge
//!
//! [Crate]: https://crates.io/crates/tui-framework-experiment
//! [API Docs]: https://docs.rs/crate/tui-framework-experiment/
//! [Dependencies]: https://deps.rs/repo/github/joshka/tui-framework-experiment
//! [Coverage]: https://app.codecov.io/gh/joshka/tui-framework-experiment
//! [Ratatui Discord]: https://discord.gg/pMCEU9hNEj
//!
//! [Ratatui]: https://crates.io/crates/ratatui

pub mod button;
pub mod events;
pub mod stack_container;
pub mod toggle_switch;

pub use button::{Button, State as ButtonState, Theme as ButtonTheme};
pub use stack_container::StackContainer;
