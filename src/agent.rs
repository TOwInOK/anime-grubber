use std::borrow::Cow;

use crate::result::Result;
use async_trait::async_trait;
#[async_trait]
/// A trait for image retrieval agents.
///
/// This trait defines the necessary methods for any agent responsible for
/// fetching images. It utilizes asynchronous programming, allowing for
/// non-blocking operations when retrieving images from a source.
///
/// # Methods
/// - `get`: Asynchronously retrieves a single image and returns its URL as a `Cow<'_, str>`.
/// - `get_many`: Asynchronously retrieves multiple images and returns their URLs as a `Vec<Cow<'_, str>>`.
/// - `get_random`: Asynchronously retrieves a random image and returns its URL as a `Cow<'_, str>`.
///
pub trait Agent {
    /// Retrieves a single image.
    ///
    /// Returns image URL as a string reference.
    ///
    /// # Errors
    /// Returns error if image cannot be retrieved.
    ///
    /// # Returns
    /// - `Ok(Cow<'_, str>)` - URL of retrieved image
    /// - `Err(Error)` - If retrieval fails
    async fn get(&self) -> Result<ImageUrl<'_>>;
    /// Retrieves multiple images.
    ///
    /// Returns vector of image URLs.
    ///
    /// # Errors
    /// Returns error if images cannot be retrieved.
    ///
    /// # Returns
    /// - `Ok(Vec<Cow<'_, str>>)` - URLs of retrieved images
    /// - `Err(Error)` - If retrieval fails
    async fn get_many(&self) -> Result<ImageUrls<'_>>;
    /// Retrieves a random image.
    ///
    /// Returns URL of random image.
    ///
    /// # Errors
    /// Returns error if image cannot be retrieved.
    ///
    /// # Returns
    /// - `Ok(Cow<'_, str>)` - URL of random image
    /// - `Err(Error)` - If retrieval fails
    async fn get_random(&self) -> Result<ImageUrl<'_>>;
}

pub type ImageUrl<'a> = Cow<'a, str>;
pub type ImageUrls<'a> = Box<[Cow<'a, str>]>;
