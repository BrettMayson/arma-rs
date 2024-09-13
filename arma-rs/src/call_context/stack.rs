use std::ffi::CStr;

#[repr(C)]
pub struct RawStackTraceLine {
    // Line number in file (before preprocessing if preprocessed with line numbers)
    pub line_number: u32,
    // File offset in bytes from the start of the file (after preprocessing)
    pub file_offset: u32,
    // Filepath to the source file
    pub source_file: *const libc::c_char,
    // scopeName set on that level
    pub scope_name: *const libc::c_char,
    // Complete fileContent of the sourceFile (after preprocessing, can be combined with fileOffset to find exact location)
    pub file_content: *const libc::c_char,
}

#[repr(C)]
pub struct RawContextStackTrace {
    pub lines: *mut RawStackTraceLine,
    pub line_count: u32,
}

impl RawContextStackTrace {
    pub fn to_lines(&self) -> Option<&[RawStackTraceLine]> {
        unsafe {
            self.lines
                .as_ref()
                .map(|lines_ptr| std::slice::from_raw_parts(lines_ptr, self.line_count as usize))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArmaContextStackTrace {
    pub lines: Vec<ArmaStackTraceLine>,
}

#[derive(Debug, Clone)]
pub struct ArmaStackTraceLine {
    pub line_number: u32,
    pub file_offset: u32,
    pub source_file: String,
    pub scope_name: String,
    pub file_content: String,
}

impl From<*const RawContextStackTrace> for ArmaContextStackTrace {
    fn from(raw: *const RawContextStackTrace) -> Self {
        unsafe {
            let raw = raw.as_ref().unwrap();
            let lines = raw
                .to_lines()
                .unwrap()
                .iter()
                .map(|line| ArmaStackTraceLine {
                    line_number: line.line_number,
                    file_offset: line.file_offset,
                    source_file: CStr::from_ptr(line.source_file)
                        .to_string_lossy()
                        .into_owned(),
                    scope_name: CStr::from_ptr(line.scope_name)
                        .to_string_lossy()
                        .into_owned(),
                    file_content: CStr::from_ptr(line.file_content)
                        .to_string_lossy()
                        .into_owned(),
                })
                .collect();
            Self { lines }
        }
    }
}
