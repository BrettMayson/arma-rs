use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct ArmaContext {
    steam_id: Option<String>,
    file_source: Option<PathBuf>,
    mission_name: Option<String>,
    server_name: Option<String>,
}

impl ArmaContext {
    pub(crate) fn steam_id(&self) -> Option<&str> {
        self.steam_id.as_deref()
    }

    pub(crate) fn file_source(&self) -> Option<&Path> {
        self.file_source.as_deref()
    }

    pub(crate) fn mission_name(&self) -> Option<&str> {
        self.mission_name.as_deref()
    }

    pub(crate) fn server_name(&self) -> Option<&str> {
        self.server_name.as_deref()
    }

    #[must_use]
    pub fn with_steam_id(mut self, steam_id: &str) -> Self {
        self.steam_id = if !steam_id.is_empty() && steam_id != "0" {
            Some(steam_id.to_string())
        } else {
            None
        };
        self
    }

    #[must_use]
    pub fn with_file_source(mut self, file_source: &str) -> Self {
        self.file_source = if !file_source.is_empty() {
            Some(PathBuf::from(file_source))
        } else {
            None
        };
        self
    }

    #[must_use]
    pub fn with_mission_name(mut self, mission_name: &str) -> Self {
        self.mission_name = if !mission_name.is_empty() {
            Some(mission_name.to_string())
        } else {
            None
        };
        self
    }

    #[must_use]
    pub fn with_server_name(mut self, server_name: &str) -> Self {
        self.server_name = if !server_name.is_empty() {
            Some(server_name.to_string())
        } else {
            None
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steam_id_empty() {
        let ctx = ArmaContext::default().with_steam_id("");
        assert_eq!(ctx.steam_id(), None);
    }

    #[test]
    fn steam_id_zero() {
        let ctx = ArmaContext::default().with_steam_id("0");
        assert_eq!(ctx.steam_id(), None);
    }

    #[test]
    fn file_source_empty() {
        let ctx = ArmaContext::default().with_file_source("");
        assert_eq!(ctx.file_source(), None);
    }

    #[test]
    fn mission_name_empty() {
        let ctx = ArmaContext::default().with_mission_name("");
        assert_eq!(ctx.mission_name(), None);
    }

    #[test]
    fn server_name_empty() {
        let ctx = ArmaContext::default().with_server_name("");
        assert_eq!(ctx.server_name(), None);
    }
}
