//! Convenient library for extracting images of cute (or not so cute) characters from websites.
//!
//! You can start with
//! ```
//! use anime_grubber::{agents::waifu_pics::{Waifu, Categories, SFW}, agent::Agent};
//!
//! #[tokio::main]
//! async fn main()  {
//!     let instance = Waifu::new(Categories::SFW(SFW::Dance));
//!     let image = instance.get().await.expect("shit happends");
//!     println!("Fetched image URL: {}", image);
//! }
//! ```
//!
/// A trait for image retrieval agents.
pub mod agent;
pub mod agents;
/// pub errors of this crate
pub mod error;
pub mod gen_enum;
pub mod gen_url;
pub mod result;

pub use crate::{agent::Agent, agents::*, error::Error, result::Result};
