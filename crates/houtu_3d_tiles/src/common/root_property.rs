use serde::Deserialize;
use crate::common::extension::Extension;
use crate::common::extras::Extras;

/// A basis for storing extensions and extras.
#[derive(Debug, Deserialize)]
pub struct RootProperty {
    /// Dictionary object with extension-specific objects.
    pub extensions: Option<Extension>,
    /// Application-specific data.
    pub extras: Option<Extras>,
}