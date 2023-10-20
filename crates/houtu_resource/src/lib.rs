use bevy::app::App;
use bevy::prelude::{Component, Plugin};
use reqwest::header::HeaderMap;
use url::Url;

pub struct HoutuNetResourcePlugin;

impl Plugin for HoutuNetResourcePlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Default)]
pub struct ResourceBuilder {
    url: String,
    retry_count: usize,
}

impl ResourceBuilder {
    pub fn new(url: impl ToString) -> ResourceBuilder {
        ResourceBuilder {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn retry_count(mut self, retry_count: usize) -> ResourceBuilder {
        self.retry_count = retry_count;
        self
    }

    pub fn build(self) -> NetworkResource {
        NetworkResource {
            url: Url::parse(&self.url).expect("parse url error"),
            headers: Default::default(),
            retry_count: 0,
        }
    }
}

#[derive(Debug, Component)]
pub struct NetworkResource {
    url: Url,
    headers: HeaderMap,
    retry_count: usize,
}

impl NetworkResource {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            headers: HeaderMap::new(),
            retry_count: 0,
        }
    }

    pub fn fetch_json(url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json,*/*;q=0.01".parse().unwrap());

        Self {
            url: Url::parse(url).expect("parse url error"),
            headers,
            retry_count: 0,
        }
    }

    /// Get the base uri of the url.
    pub fn get_base_uri(&self, include_query: bool) -> String {
        if include_query {
            let mut path = self.url.path().to_string();
            if let Some(query) = self.url.query() {
                path += format!("?{}", query).as_str();
            }
            if let Some(fragment) = self.url.fragment() {
                path += format!("#{}", fragment).as_str();
            }

            path
        } else {
            self.url.path().to_string()
        }
    }

    /// Get the extension of the url path.
    pub fn extension(&self) -> &str {
        let path = self.url.path();
        let index = path.rfind("/");
        let path = match index {
            Some(i) => &path[i + 1..],
            None => path,
        };

        let index = path.rfind(".");
        match index {
            Some(i) => &path[i + 1..],
            None => "",
        }
    }

    /// Determines whether the url is a data uri.
    pub fn is_data_uri(&self) -> bool {
        self.url.scheme() == "data"
    }

    /// Determines whether the url is a blob uri.
    pub fn is_blob_uri(&self) -> bool {
        self.url.scheme() == "blob"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_resource_extension() {
        use super::ResourceBuilder;

        let resource = ResourceBuilder::new("http://www.test.com/abc.jpg").build();
        assert_eq!(resource.extension(), "jpg");

        let resource = ResourceBuilder::new("http://www.test.com/edf").build();
        assert_eq!(resource.extension(), "");

        let resource = ResourceBuilder::new("http://www.test.com/fgh.").build();
        assert_eq!(resource.extension(), "");

        let resource = ResourceBuilder::new("http://www.test.com/ijk.jpg?abc=123").build();
        assert_eq!(resource.extension(), "jpg");

        let resource = ResourceBuilder::new("http://www.test.com/lmn.jpg?abc=123&def=456").build();
        assert_eq!(resource.extension(), "jpg");
    }
}
