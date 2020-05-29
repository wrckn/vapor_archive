use crate::{};

use std::{};

use serde::{
    Serialize,
    Deserialize
};
use chrono::{
    DateTime,
    Utc,
    serde::ts_milliseconds
};

/// FileMetadata struct
/// 
/// Represents a files metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Boolean indicating whether or not this file is readonly
    pub readonly: bool,
    /// Option possibly containing linux file permissions
    pub permissions_opt: Option<u32>,
    /// DateTime indicating when this file was added to the archive
    #[serde(with = "ts_milliseconds")]
    pub added_at: DateTime<Utc>,
}

impl FileMetadata {
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    pub fn with_permissions(mut self,  permissions: u32) -> Self {
        self.permissions_opt = Some(permissions);
        self
    }
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            readonly: false,
            permissions_opt: None,
            added_at: Utc::now()
        }
    }
}