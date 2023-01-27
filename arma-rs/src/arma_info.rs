use std::path::{Path, PathBuf};

#[derive(Clone, Default)]
pub struct ArmaInfo {
    steam_id: Option<String>,
    file_source: Option<PathBuf>,
    mission_name: Option<String>,
    server_name: Option<String>,
}

impl ArmaInfo {
    pub(crate) fn new(
        steam_id: &str,
        file_source: &str,
        mission_name: &str,
        server_name: &str,
    ) -> Self {
        let steam_id = if !steam_id.is_empty() && steam_id != "0" {
            Some(steam_id.to_string())
        } else {
            None
        };

        let file_source = if !file_source.is_empty() {
            Some(PathBuf::from(file_source))
        } else {
            None
        };

        let mission_name = if !mission_name.is_empty() {
            Some(mission_name.to_string())
        } else {
            None
        };

        let server_name = if !server_name.is_empty() {
            Some(server_name.to_string())
        } else {
            None
        };

        Self {
            steam_id,
            file_source,
            mission_name,
            server_name,
        }
    }

    pub fn steam_id(&self) -> Option<&str> {
        self.steam_id.as_deref()
    }

    pub fn file_source(&self) -> Option<&Path> {
        self.file_source.as_deref()
    }

    pub fn mission_name(&self) -> Option<&str> {
        self.mission_name.as_deref()
    }

    pub fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }
}
