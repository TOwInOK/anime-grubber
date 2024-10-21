use crate::agent::Agent;
use crate::result::Result;
use async_trait::async_trait;
use miniserde::{json, Deserialize};
use url::Url;

const BASE_URL: &str = "https://api.waifu.pics/";
macro_rules! url {
    ($base_url:expr, $categoier:expr, $aspect:expr) => {{
        Url::parse($base_url)?.join($categoier)?.join($aspect)?
    }};
}

pub struct Faifu {
    categorie: Categories,
}

impl Faifu {
    pub fn new(categorie: Categories) -> Self {
        Self { categorie }
    }

    pub fn set_categorie(&mut self, categorie: Categories) {
        self.categorie = categorie;
    }
}

#[async_trait]
impl Agent for Faifu {
    async fn get(&self) -> Result<String> {
        let category: &str = (&self.categorie).into();
        let aspect: &str = match &self.categorie {
            Categories::SWF(value) => value.into(),
            Categories::NSWF(value) => value.into(),
        };

        let url = url!(BASE_URL, category, aspect);
        let response = reqwest::get(url).await?.text().await?;

        let solo_image: SoloImage = json::from_str(&response)?;
        Ok(solo_image.into())
    }
    async fn get_many(&self) -> Result<Vec<String>> {
        todo!()
    }
    async fn get_random(&self) -> Result<String> {
        todo!()
    }
}

pub enum Categories {
    SWF(SWF),
    NSWF(NSFW),
}

pub enum SWF {
    Waifu,
    Neko,
    Shinobu,
    Megumin,
    Bully,
    Cuddle,
    Cry,
    Hug,
    Awoo,
    Kiss,
    Lick,
    Pat,
    Smug,
    Bonk,
    Yeet,
    Blush,
    Smile,
    Wave,
    Highfive,
    Handhold,
    Nom,
    Bite,
    Glomp,
    Slap,
    Kill,
    Kick,
    Happy,
    Wink,
    Poke,
    Dance,
    Cringe,
}
pub enum NSFW {
    Waifu,
    Neko,
    Trap,
    Blowjob,
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

#[derive(Debug, Deserialize)]
struct ManyImages {
    files: Vec<String>,
}
impl From<ManyImages> for Vec<String> {
    fn from(value: ManyImages) -> Self {
        value.files
    }
}

impl From<&Categories> for &str {
    fn from(value: &Categories) -> Self {
        match value {
            Categories::SWF(_) => "swf",
            Categories::NSWF(_) => "nswf",
        }
    }
}
impl From<&NSFW> for &str {
    fn from(value: &NSFW) -> Self {
        match value {
            NSFW::Waifu => "waifu",
            NSFW::Neko => "neko",
            NSFW::Trap => "trap",
            NSFW::Blowjob => "blowjob",
        }
    }
}
impl From<&SWF> for &str {
    fn from(value: &SWF) -> Self {
        match value {
            SWF::Neko => "neko",
            SWF::Shinobu => "shinobu",
            SWF::Megumin => "megumin",
            SWF::Bully => "bully",
            SWF::Cuddle => "cuddle",
            SWF::Cry => "cry",
            SWF::Hug => "hug",
            SWF::Awoo => "awoo",
            SWF::Kiss => "kiss",
            SWF::Lick => "lick",
            SWF::Pat => "pat",
            SWF::Smug => "smug",
            SWF::Bonk => "bonk",
            SWF::Yeet => "yeet",
            SWF::Blush => "blush",
            SWF::Smile => "smile",
            SWF::Wave => "wave",
            SWF::Highfive => "highfive",
            SWF::Handhold => "handhold",
            SWF::Nom => "nom",
            SWF::Bite => "bite",
            SWF::Glomp => "glomp",
            SWF::Slap => "slap",
            SWF::Kill => "kill",
            SWF::Kick => "kick",
            SWF::Happy => "happy",
            SWF::Wink => "wink",
            SWF::Poke => "poke",
            SWF::Dance => "dance",
            SWF::Cringe => "cringe",
            SWF::Waifu => "waifu",
        }
    }
}
