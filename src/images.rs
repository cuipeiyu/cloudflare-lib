#[cfg(any(feature = "full", feature = "with-images"))]
mod images_bindings {


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Image unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Can set the creator field with an internal user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    /// Image file name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// User modifiable key-value store. Can be used for keeping references to another
    /// system of record for managing images. Metadata must not exceed 1024 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,

    /// Indicates whether the image can be a accessed only using it's UID. If set to
    /// true, a signed token needs to be generated with a signing key to view the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requireSignedURLs")]
    pub require_signed_ur_ls: Option<bool>,

    /// When the media item was uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<DateTime<Utc>>,

    /// Object specifying available variants for an image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageKey {
    /// Key name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<StatCount>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<VariantVariants>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatCount {
    /// Cloudflare Images allowed usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<f64>,

    /// Cloudflare Images current usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantVariants {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hero: Option<VariantVariantsHero>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantVariantsHero {
    pub id: String,

    /// Allows you to define image resizing sizes for different use cases.
    pub options: VariantVariantsHeroOptions,

    /// Indicates whether the variant can access an image without a signature,
    /// regardless of image access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "neverRequireSignedURLs")]
    pub never_require_signed_ur_ls: Option<bool>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantVariantsHeroOptions {
    /// The fit property describes how the width and height dimensions should be
    /// interpreted.
    pub fit: VariantVariantsHeroOptionsFit,

    /// Maximum height in image pixels.
    pub height: f64,

    /// What EXIF data should be preserved in the output image.
    pub metadata: VariantVariantsHeroOptionsMetadata,

    /// Maximum width in image pixels.
    pub width: f64,

}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariantVariantsHeroOptionsFit {
    #[serde(rename = "scale-down")]
    VariantVariantsHeroOptionsFitScaleDown,
    #[serde(rename = "contain")]
    VariantVariantsHeroOptionsFitContain,
    #[serde(rename = "cover")]
    VariantVariantsHeroOptionsFitCover,
    #[serde(rename = "crop")]
    VariantVariantsHeroOptionsFitCrop,
    #[serde(rename = "pad")]
    VariantVariantsHeroOptionsFitPad,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariantVariantsHeroOptionsMetadata {
    #[serde(rename = "keep")]
    VariantVariantsHeroOptionsMetadataKeep,
    #[serde(rename = "copyright")]
    VariantVariantsHeroOptionsMetadataCopyright,
    #[serde(rename = "none")]
    VariantVariantsHeroOptionsMetadataNone,
}

}

#[cfg(any(feature = "full", feature = "with-images"))]
pub use images_bindings::*;

#[cfg(any(feature = "full", feature = "with-images"))]
impl Client {}
