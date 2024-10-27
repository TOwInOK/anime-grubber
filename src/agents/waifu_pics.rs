use std::borrow::Cow;
use std::time::Duration;

use crate::agent::{ImageUrl, ImageUrls};
use crate::error::Error;
use crate::result::Result;
use crate::{agent::Agent, gen_enum, url};
use async_trait::async_trait;
use miniserde::{json, Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::StatusCode;
use tracing::{debug, info, instrument, trace};

const SOLO_URL: &str = "https://api.waifu.pics";
const MANY_URL: &str = "https://api.waifu.pics/many";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90);
const DEFAULT_POOL_MAX_IDLE: usize = 32;
#[derive(Debug, Clone)]
/// An image-fetching agent from the [Waifu.pics API](https://waifu.pics/docs).
///
/// This crate provides methods to retrieve images based on specified categories.
/// It supports fetching single images, multiple images and random images.
/// The images are returned as `Cow<str>` containing URLs.
///
/// # Examples
/// ```rust
/// use anime_grubber::agent::Agent;
/// use anime_grubber::agents::waifu_pics::{Waifu, Categories, SFW};
/// use std::borrow::Cow;
/// async fn example() {
///     let mut waifu = Waifu::new(Categories::SFW(SFW::Dance));
///
///     // Get single image URL
///     let image_url: Cow<str> = waifu.get().await.unwrap();
///
///     // Get multiple image URLs
///     let many_urls: Box<[Cow<str>]> = waifu.get_many().await.unwrap();
///
///     // Get random image URL
///     let random_url: Cow<str> = waifu.get_random().await.unwrap();
/// }
/// ```
pub struct Waifu {
    pub categorie: Categories,
    client: reqwest::Client,
}

impl Waifu {
    /// Creates a new instance of `Waifu` with the specified category.
    ///
    /// # Parameters
    /// - `categorie`: The category of images to fetch, which can be either SFW (Safe for Work)
    ///   or NSFW (Not Safe for Work).
    ///
    /// # Returns
    /// Returns a new `Waifu` instance.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agents::waifu_pics::{Waifu, Categories, SFW};
    ///
    /// let Waifu = Waifu::new(Categories::SFW(SFW::Dance));
    /// ```
    #[instrument(skip(categorie))]
    pub fn new(categorie: Categories) -> Self {
        let client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .pool_idle_timeout(DEFAULT_POOL_IDLE_TIMEOUT)
            .pool_max_idle_per_host(DEFAULT_POOL_MAX_IDLE)
            .default_headers({
                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers
            })
            .build()
            .expect("Failed to create HTTP client");

        Self { categorie, client }
    }

    /// Updates the category of the `Waifu` instance.
    ///
    /// # Parameters
    /// - `categorie`: The new category of images to fetch.
    ///
    /// # Example
    /// ```rust
    /// use anime_grubber::agents::waifu_pics::{Waifu, Categories, SFW, NSFW};
    ///
    /// let mut Waifu = Waifu::new(Categories::SFW(SFW::Dance));
    /// Waifu.set_categorie(Categories::NSFW(NSFW::Neko));
    /// ```
    #[instrument(skip(self, categorie))]
    pub fn set_categorie(&mut self, categorie: Categories) {
        self.categorie = categorie;
    }
}

#[async_trait]
impl Agent for Waifu {
    #[instrument(skip(self))]
    async fn get(&self) -> Result<ImageUrl<'_>> {
        info!("Fetch data");
        let category: &str = (&self.categorie).into();
        let aspect = self.categorie.nested_str();
        let url = url!(SOLO_URL, category, aspect);

        let res = self.client.get(url).send().await?;
        if res.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::NotFound);
        }
        trace!("Response received: status={}", res.status());
        trace!("res -> {:#?}", res);
        let res_text = res.text().await?;

        let conveted = json::from_str::<SoloImage>(&res_text)?;

        Ok(conveted.into())
    }

    #[instrument(skip(self))]
    async fn get_many(&self) -> Result<ImageUrls<'_>> {
        info!("Fetch many data");
        let category: &str = (&self.categorie).into();
        let aspect = self.categorie.nested_str();
        let url = url!(MANY_URL, category, aspect);

        let res = self
            .client
            .post(url)
            .body(json::to_string(&Body::default()))
            .send()
            .await?;
        if !res.status().is_success() {
            return match res.status() {
                StatusCode::NOT_FOUND => Err(Error::NotFound),
                StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimit),
                status => Err(Error::RequestFailed(status)),
            };
        }
        trace!("Response received: status={}", res.status());
        trace!("res -> {:#?}", res);
        let res_text = res.text().await?;
        let conveted = json::from_str::<ManyImages>(&res_text)?;

        Ok(conveted.into())
    }

    #[instrument(skip(self))]
    async fn get_random(&self) -> Result<ImageUrl<'_>> {
        info!("Fetch \"random\" data");
        self.get().await
    }
}

#[derive(Debug, Deserialize)]
struct SoloImage {
    url: String,
}
impl<'a> From<SoloImage> for Cow<'a, str> {
    fn from(value: SoloImage) -> Self {
        Cow::Owned(value.url)
    }
}
#[derive(Debug, Serialize, Default)]
struct Body<'a> {
    exclude: Box<[&'a str]>,
}

#[derive(Debug, Deserialize)]
struct ManyImages {
    files: Vec<String>,
}
impl<'a> From<ManyImages> for Box<[Cow<'a, str>]> {
    fn from(value: ManyImages) -> Self {
        value.files.into_iter().map(Cow::Owned).collect()
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
