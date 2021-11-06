use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JavaConfig {
    minimum_version: Option<usize>,
}
