use bevy::prelude::Component;
use reqwest::header::HeaderMap;
use std::collections::BTreeMap;
use url::Url;

#[derive(Default)]
pub struct ResourceBuilder {
    url: String,
    headers: BTreeMap<String, String>,
    retry_count: usize,
}

impl ResourceBuilder {
    pub fn new(url: impl ToString) -> ResourceBuilder {
        ResourceBuilder {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn headers(mut self, key: impl ToString, value: impl ToString) -> ResourceBuilder {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn retry_count(mut self, retry_count: usize) -> ResourceBuilder {
        self.retry_count = retry_count;
        self
    }

    pub fn build(self) -> Resource {
        Resource {
            url: Url::parse(&self.url).expect("parse url error"),
            headers: self
                .headers
                .into_iter()
                .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
                .collect(),

            retry_count: 0,
        }
    }
}

#[derive(Debug, Component)]
pub struct Resource {
    pub url: Url,
    headers: HeaderMap,
    retry_count: usize,
}

impl Resource {
    pub fn extension(&self) -> Option<&str> {
        match self.url.path_segments() {
            None => None,
            Some(path_segment) => match path_segment.last() {
                None => None,
                Some(last_seg) => {
                    let last = last_seg.split('.').last();
                    if let Some(last_str) = last {
                        if last_str == last_seg || last_str.is_empty() {
                            None
                        } else {
                            Some(last_str)
                        }
                    } else {
                        None
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_resource_extension() {
        use super::ResourceBuilder;

        let resource = ResourceBuilder::new("http://www.test.com/abc.jpg").build();
        assert_eq!(resource.extension(), Some("jpg"));

        let resource = ResourceBuilder::new("http://www.test.com/edf").build();
        assert_eq!(resource.extension(), None);

        let resource = ResourceBuilder::new("http://www.test.com/fgh.").build();
        assert_eq!(resource.extension(), None);

        let resource = ResourceBuilder::new("http://www.test.com/ijk.jpg?abc=123").build();
        assert_eq!(resource.extension(), Some("jpg"));

        let resource = ResourceBuilder::new("http://www.test.com/lmn.jpg?abc=123&def=456").build();
        assert_eq!(resource.extension(), Some("jpg"));
    }
}
