mod os_resource;
mod db_resource;

use crate::resource::GenericResource;

pub trait ResourceProvider {
    /// 获取所有资源
    fn fetch_resources(&self) -> Vec<GenericResource>;
}

pub struct CompositeResourceProvider {
    providers: Vec<Box<dyn ResourceProvider>>,
}

impl CompositeResourceProvider {
    pub fn new() -> Self {
        CompositeResourceProvider {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Box<dyn ResourceProvider>) {
        self.providers.push(provider);
    }
}

impl ResourceProvider for CompositeResourceProvider {
    fn fetch_resources(&self) -> Vec<GenericResource> {
        let mut all_resources = Vec::new();
        for provider in &self.providers {
            all_resources.extend(provider.fetch_resources());
        }
        all_resources
    }
}
