use std::path::Path;

#[derive(Clone)]
pub struct ArmaInfo {
    pub(crate) steam_id: String,
    pub(crate) file_source: String,
    pub(crate) mission_name: String,
    pub(crate) server_name: String,
}

impl ArmaInfo {
    pub fn steam_id(&self) -> Option<&str> {
        if !self.steam_id.is_empty() && self.steam_id != "0" {
            Some(&self.steam_id)
        } else {
            None
        }
    }

    pub fn file_source(&self) -> Option<&Path> {
        if !self.file_source.is_empty() {
            Some(Path::new(&self.file_source))
        } else {
            None
        }
    }

    pub fn mission_name(&self) -> Option<&str> {
        if !self.mission_name.is_empty() {
            Some(&self.mission_name)
        } else {
            None
        }
    }

    pub fn server_name(&self) -> Option<&str> {
        if !self.server_name.is_empty() {
            Some(&self.server_name)
        } else {
            None
        }
    }
}
