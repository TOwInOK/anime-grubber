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
/// - `get`: Asynchronously retrieves a single image and returns its URL as a `String`.
/// - `get_many`: Asynchronously retrieves multiple images and returns their URLs as a `Vec<String>`.
/// - `get_random`: Asynchronously retrieves a random image and returns its URL as a `String`.
///
pub trait Agent {
    /// Asynchronously fetches a single image based on the current category and aspect.
    ///
    /// This method constructs a URL using the category and aspect generated from
    /// the current state of the `Waifu` instance. It performs an HTTP GET request
    /// to retrieve the image. If the image is not found, it returns an error.
    ///
    /// # Returns
    /// Returns a `Result<String>`, containing the URL of the fetched image or an
    /// error if the request fails.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agent::Agent;
    /// use anime_grubber::agents::waifu_pics::{Waifu, Categories, SFW};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let Waifu = Waifu::new(Categories::SFW(SFW::Dance));
    ///     let image_url = Waifu.get().await.expect("Failed to fetch image");
    ///     println!("Fetched image URL: {}", image_url);
    /// }
    /// ```
    async fn get(&self) -> Result<String>;

    /// Asynchronously fetches multiple images based on the current category and aspect.
    ///
    /// This method constructs a URL using the category and aspect generated from
    /// the current state of the `Waifu` instance. It performs an HTTP POST request
    /// to retrieve an array of images. If the images are not found, it returns an error.
    ///
    /// # Returns
    /// Returns a `Result<Vec<String>>`, containing a vector of URLs for the fetched
    /// images or an error if the request fails.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agent::Agent;
    /// use anime_grubber::agents::waifu_pics::{Waifu, Categories, SFW};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let Waifu = Waifu::new(Categories::SFW(SFW::Dance));
    ///     let many_images = Waifu.get_many().await.expect("Failed to fetch images");
    ///     for image_url in many_images {
    ///         println!("Fetched image URL: {}", image_url);
    ///     }
    /// }
    /// ```
    async fn get_many(&self) -> Result<Vec<String>>;
    /// Asynchronously fetches a random image.
    ///
    /// This method calls the `get` method to retrieve a random image. It behaves
    /// the same as `get`, returning a URL of a randomly selected image.
    ///
    /// # Returns
    /// Returns a `Result<String>`, containing the URL of the fetched random image
    /// or an error if the request fails.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agent::Agent;
    /// use anime_grubber::agents::waifu_pics::{Waifu, Categories, SFW};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let Waifu = Waifu::new(Categories::SFW(SFW::Dance));
    ///     let random_image_url = Waifu.get_random().await.expect("Failed to fetch random image");
    ///     println!("Fetched random image URL: {}", random_image_url);
    /// }
    /// ```
    async fn get_random(&self) -> Result<String>;
}
