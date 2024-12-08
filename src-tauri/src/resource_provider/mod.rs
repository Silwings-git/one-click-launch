mod db_resource;
mod os_resource;

use crate::error::OCLError;
use crate::resource::GenericResource;

pub trait ResourceProvider {
    /// 获取所有资源
    fn fetch_resources(&self) -> Result<Vec<GenericResource>, OCLError>;
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
    fn fetch_resources(&self) -> Result<Vec<GenericResource>, OCLError> {
        let mut all_resources = Vec::new();
        for provider in &self.providers {
            all_resources.extend(provider.fetch_resources()?);
        }
        Ok(all_resources)
    }
}
