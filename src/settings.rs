#[allow(dead_code)]
pub enum Config {
    Project(String),
    Main(String),
    Locale(String),
    Url(String),
    SiteTitle(String),
    Description(String),
    PageType(String),
    Favicons(String),
    DefaultImage(String),
    PageXLargeImage(String),
}
pub struct HTMLMeta {
    pub site_title: String,
    pub page_description: String,
    pub site_url: String,
    pub page_url: String,
    pub page_og_title: String,
    pub page_og_description: String,
    pub page_image_url: String,
    pub page_image_alt: String,
    pub page_locale: String,
    pub page_type: String,
    pub page_twitter_large_image: String,
    pub page_canonical_url: String,
    pub favicons_folder_url: String,
    pub theme_color_light: String,
    pub theme_color_dark: String,
    pub auto_site_title: bool,
}

pub fn get_meta_config() -> HTMLMeta {
    let default_meta = HTMLMeta {
        site_title: String::from("Website Title"),
        page_description: String::from("Website Description"),
        site_url: String::from("localhost:3069"),
        page_url: String::from(""),
        page_og_title: String::from(""),
        page_og_description: String::from(""),
        page_image_url: String::from(""),
        page_image_alt: String::from(""),
        page_locale: String::from("en_US"),
        page_type: String::from("website"),
        page_twitter_large_image: String::from(""),
        page_canonical_url: String::from(""),
        favicons_folder_url: String::from("./images/favicons"),
        theme_color_light: String::from("#fafafa"),
        theme_color_dark: String::from("#101010"),
        auto_site_title: true,
    };
    default_meta
}
