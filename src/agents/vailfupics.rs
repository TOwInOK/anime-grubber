use crate::error::Error;
use crate::result::Result;
use crate::{agent::Agent, gen_enum, url};
use async_trait::async_trait;
use miniserde::{json, Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use tracing::{debug, info, instrument, trace};

const SOLO_URL: &str = "https://api.waifu.pics";
const MANY_URL: &str = "https://api.waifu.pics/many";

#[derive(Debug)]
/// A struct representing an image-fetching agent from the [Waifu.pics API](https://waifu.pics/docs).
///
/// The `Faifu` struct implements the `Agent` trait and provides methods to retrieve
/// images based on specified categories. It supports fetching single images, multiple images,
/// and random images. The struct utilizes asynchronous operations for efficient network
/// requests.
///
/// # Fields
/// - `categorie`: The category of images to fetch, either SFW (Safe for Work) or NSFW (Not Safe for Work).
///
/// # Example
/// ```rust
/// use anime_grubber::agent::Agent;
/// use anime_grubber::agents::vailfupics::{Faifu, Categories, SFW};
///
/// async fn test() {
/// let mut faifu = Faifu::new(Categories::SFW(SFW::Dance));
/// let image_url = faifu.get().await.expect("Problem with intrnet");
/// let many_images = faifu.get_many().await.expect("Problem with intrnet");
/// let random_image = faifu.get_random().await.expect("Problem with intrnet");
/// }
/// ```
pub struct Faifu {
    categorie: Categories,
}

impl Faifu {
    /// Creates a new instance of `Faifu` with the specified category.
    ///
    /// # Parameters
    /// - `categorie`: The category of images to fetch, which can be either SFW (Safe for Work)
    ///   or NSFW (Not Safe for Work).
    ///
    /// # Returns
    /// Returns a new `Faifu` instance.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agents::vailfupics::{Faifu, Categories, SFW};
    ///
    /// let faifu = Faifu::new(Categories::SFW(SFW::Dance));
    /// ```
    #[instrument(skip(categorie))]
    pub fn new(categorie: Categories) -> Self {
        Self { categorie }
    }

    /// Updates the category of the `Faifu` instance.
    ///
    /// # Parameters
    /// - `categorie`: The new category of images to fetch.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agents::vailfupics::{Faifu, Categories, SFW, NSFW};
    ///
    /// let mut faifu = Faifu::new(Categories::SFW(SFW::Dance));
    /// faifu.set_categorie(Categories::NSFW(NSFW::Neko));
    /// ```
    #[instrument(skip(self, categorie))]
    pub fn set_categorie(&mut self, categorie: Categories) {
        self.categorie = categorie;
    }

    /// Generates string representations of the current category and aspect.
    ///
    /// This method returns a tuple containing the string representations of the
    /// current category and aspect.
    ///
    /// # Returns
    /// Returns a tuple `(&str, &str)` representing the category and aspect.
    #[instrument(skip(self))]
    fn gen_aspects(&self) -> (&str, &str) {
        debug!("Custing aspects");
        let category: &str = (&self.categorie).into();
        let aspect: &str = match &self.categorie {
            Categories::SFW(value) => value.into(),
            Categories::NSFW(value) => value.into(),
        };
        debug!("Category -> {} \\<::>/ Aspect -> {}", &category, &aspect);
        (category, aspect)
    }
}

#[async_trait]
impl Agent for Faifu {
    #[instrument(skip(self))]
    async fn get(&self) -> Result<String> {
        info!("Fetch data");
        let (category, aspect) = self.gen_aspects();
        let url = url!(SOLO_URL, category, aspect);

        let res = reqwest::get(url).await?;
        if res.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::NotFound);
        }

        trace!("res -> {:#?}", res);
        let res_text = res.text().await?;

        let conveted = json::from_str::<SoloImage>(&res_text)?;

        Ok(conveted.into())
    }

    #[instrument(skip(self))]
    async fn get_many(&self) -> Result<Vec<String>> {
        info!("Fetch many data");
        let (category, aspect) = self.gen_aspects();
        let url = url!(MANY_URL, category, aspect);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let req = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        let res = req
            .post(url)
            .body(json::to_string(&Body::default()))
            .send()
            .await?;
        if res.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::NotFound);
        }
        trace!("res -> {:#?}", res);
        let res_text = res.text().await?;

        let conveted = json::from_str::<ManyImages>(&res_text)?;

        Ok(conveted.into())
    }

    #[instrument(skip(self))]
    async fn get_random(&self) -> Result<String> {
        info!("Fetch \"random\" data");
        self.get().await
    }
}

#[derive(Debug, Deserialize)]
struct SoloImage {
    url: String,
}
impl From<SoloImage> for String {
    fn from(value: SoloImage) -> Self {
        value.url
    }
}
#[derive(Debug, Serialize, Default)]
struct Body {
    exclude: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ManyImages {
    files: Vec<String>,
}
impl From<ManyImages> for Vec<String> {
    fn from(value: ManyImages) -> Self {
        value.files
    }
}

gen_enum!(
    SFW,
    [
        Waifu, Neko, Shinobu, Megumin, Bully, Cuddle, Cry, Hug, Awoo, Kiss, Lick, Pat, Smug, Bonk,
        Yeet, Blush, Smile, Wave, Highfive, Handhold, Nom, Bite, Glomp, Slap, Kill, Kick, Happy,
        Wink, Poke, Dance, Cringe,
    ]
);

gen_enum!(NSFW, [Waifu, Neko, Trap, Blowjob,]);

gen_enum!(Categories, [SFW(SFW), NSFW(NSFW)]);
