use crate::HoutuNetworkResource;
use serde_json::Value;
use std::collections::BTreeMap;
use url::Url;

pub struct ResourceCache {
    cache_entries: BTreeMap<String, CacheEntry>,
}

impl ResourceCache {
    pub fn get_schema_loader(&self, schema: Option<&Value>, resource: HoutuNetworkResource) {
        let cache_key = ResourceCacheKey::get_schema_cache_key(schema, resource);
        match self.cache_entries.get(&cache_key) {
            Some(entry) => entry,
            None => {}
        };
    }

    pub fn get(&mut self, cache_key: &str) -> Option<&String> {
        match self.cache_entries.get_mut(&cache_key) {
            None => None,
            Some(entry) => {
                entry.reference_count += 1;
                Some(&entry.resource_loader)
            }
        }
    }
}

struct CacheEntry {
    reference_count: usize,
    resource_loader: String,
}

impl CacheEntry {
    pub fn new(resource_load: &str) -> Self {
        Self {
            reference_count: 1,
            resource_loader: resource_load.to_string(),
        }
    }
}

pub struct ResourceCacheKey {}

impl ResourceCacheKey {
    pub fn get_schema_cache_key(schema: Option<&Value>, resource: HoutuNetworkResource) -> String {
        if let Some(schema) = schema {
            format!("embedded-schema:{}", schema.to_string())
        } else {
            format!("external-schema:{}", resource.url.to_string())
        }
    }
}

fn get_external_resource_cache_key(resource: &HoutuNetworkResource) -> String {
    get_absolute_uri(&resource.url)
}

fn get_absolute_uri(url: &Url) -> String {
    let mut path = url.path().to_string();
    if let Some(query) = url.query() {
        path += format!("?{}", query).as_str();
    }
    if let Some(fragment) = url.fragment() {
        path += format!("#{}", fragment).as_str();
    }

    path
}
