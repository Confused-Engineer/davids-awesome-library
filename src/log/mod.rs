mod log;

/// Used to create and update actions done by the program using a log file.
pub struct LogFile {
    path: std::ffi::OsString,
    name: String,
    dir: String,    
}

impl Default for LogFile
{
    fn default() -> Self {
        Self {
            path: std::ffi::OsString::from(format!("output.log")),
            name: "output.log".to_string(),
            dir: String::new(),
        }
    }
}

impl LogFile {

    /// Creates a file "output.log" in the working directory that can be written out to.
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.write_log("success");
    /// ```
    
    pub fn new() -> Self
    {
        Self::default()
    }
}