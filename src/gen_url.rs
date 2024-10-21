/// Generates a URL based on the provided base URL, category, and aspect.
///
/// This macro takes three arguments: a base URL, a category, and an aspect,
/// and constructs a URL in the format `{base_url}/{category}/{aspect}` where
/// both `category` and `aspect` are converted to lowercase.
///
/// # Parameters
/// - `$base_url`: The base URL used to construct the final address.
/// - `$categoier`: The category to be appended to the URL.
/// - `$aspect`: The aspect to be appended to the URL.
///
/// # Returns
/// Returns a `String` containing the generated URL.
///
/// # Example
/// ```rust
/// use anime_grubber::{gen_enum, url};
///
/// let base = "https://api.example.com";
/// let category = "Animals";
/// let aspect = "Cute";
///
/// let generated_url = url!(base, category, aspect);
/// assert_eq!(generated_url, "https://api.example.com/animals/cute");
/// ```
#[macro_export]
macro_rules! url {
    ($base_url:expr, $categoier:expr, $aspect:expr) => {{
        use tracing::debug;
        let __url = format!(
            "{}/{}/{}",
            $base_url,
            $categoier.to_lowercase(),
            $aspect.to_lowercase()
        );
        debug!("url {:#?}", &__url);
        __url
    }};
}
