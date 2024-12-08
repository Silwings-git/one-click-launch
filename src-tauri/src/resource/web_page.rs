use crate::resource::{Resource, ResourceLocation, ResourceType};
use url::Url;

pub struct WebPage {
    name: String,
    url: Option<Url>,
    icon: Option<String>,
}

impl WebPage {
    pub fn new(name: String, url: Option<Url>, icon: Option<String>) -> WebPage {
        Self { name, url, icon }
    }
}

impl Resource for WebPage {
    fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> Option<ResourceLocation> {
        if let Some(url) = &self.url {
            Some(ResourceLocation::from(Url::from(url.clone())))
        } else {
            None
        }
    }

    fn icon(&self) -> Option<ResourceLocation> {
        todo!()
    }

    fn resource_type(&self) -> ResourceType {
        ResourceType::WebPage
    }
}
