use std::{
    fs::{
        Metadata
    }
};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use serde::{
    Serialize,
    Deserialize
};
use chrono::{
    DateTime,
    Utc,
    serde::{
        ts_milliseconds,
        ts_milliseconds_option
    }
};

/// FileMetadata struct
/// 
/// Represents a files metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Boolean indicating whether or not this file is readonly
    pub readonly: bool,
    /// Option possibly containing linux file permissions
    pub permissions_opt: Option<u32>,
    /// DateTime indicating when this file was added to the archive
    #[serde(with = "ts_milliseconds")]
    pub added_at: DateTime<Utc>,
    /// DateTime indicatin when this file was last modified
    #[serde(with = "ts_milliseconds_option")]
    pub modified_at: Option<DateTime<Utc>>,
    /// DateTime indicatin when this file was created
    #[serde(with = "ts_milliseconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    /// DateTime indicatin when this file was last accessed
    #[serde(with = "ts_milliseconds_option")]
    pub accessed_at: Option<DateTime<Utc>>,
}

impl Default for FileMetadata {
    fn default() -> Self {
        Self {
            readonly: false,
            permissions_opt: None,
            added_at: Utc::now(),
            modified_at: None,
            accessed_at: None,
            created_at: None
        }
    }
}

impl From<&Metadata> for FileMetadata {
    fn from(metadata: &Metadata) -> Self {
        let created_at: Option<DateTime<Utc>> = match metadata.created() {
            Ok(time) => Some(DateTime::from(time)),
            _ => None,
        };
        let modified_at: Option<DateTime<Utc>> = match metadata.created() {
            Ok(time) => Some(DateTime::from(time)),
            _ => None,
        };
        let accessed_at: Option<DateTime<Utc>> = match metadata.created() {
            Ok(time) => Some(DateTime::from(time)),
            _ => None,
        };
        let perm = metadata.permissions();
        let readonly = perm.readonly();
        let mode_opt: Option<u32>;
        #[cfg(unix)]
        {
            mode_opt = Some(perm.mode());
        }
        #[cfg(not(unix))]
        {
            mode_opt = None;
        }
        
        let added_at = Utc::now();
        Self {
            added_at: added_at,
            created_at: created_at,
            modified_at: modified_at,
            accessed_at: accessed_at,
            readonly: readonly,
            permissions_opt: mode_opt
        }
    }
}