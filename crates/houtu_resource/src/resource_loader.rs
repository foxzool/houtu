use crate::HoutuNetworkResource;
use serde_json::Value;

trait ResourceLoader {
    fn get_cache_key(&self) -> String;

    fn load(&self);
}
pub struct MetadataSchemaLoader {
    // schema: Option<Value>,
    // resource: HoutuNetworkResource,
    // cache_key: CacheKey,
}

impl ResourceLoader for MetadataSchemaLoader {
    fn get_cache_key(&self) -> String {
        todo!()
    }

    fn load(&self) {
        todo!()
    }
}
