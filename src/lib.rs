//! Convenient library for extracting images of cute (or not so cute) characters from websites.
//!
//! You can start with
//! ```
//! use anime_grubber::{agents::vailfupics::{Faifu, Categories, SFW}, agent::Agent};
//!
//! #[tokio::main]
//! async fn main()  {
//!     let instance = Faifu::new(Categories::SFW(SFW::Dance));
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
mod gen_enum;
mod gen_url;
mod result;
