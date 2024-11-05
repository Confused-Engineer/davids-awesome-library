#[cfg(target_os="windows")]
use std::os::windows::fs::MetadataExt;

use crate::log::LogFile;

impl LogFile
{

    /// Set the directory to the same directory as the binary, keeps the name "output.log"  
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.default_file_path();
    /// test.write_log("success");
    /// ```

    pub fn default_file_path(&mut self) -> &mut Self {
        
        let exe_name = std::env::current_exe().unwrap().display().to_string().split("\\").last().unwrap().to_string();
        let exe_path = std::env::current_exe().unwrap().display().to_string().replace(&exe_name, "");
        self.dir = exe_path;
        self.update_path();
        self
    }

    /// Set the directory for the log file to a custom one, retains the name "output.log"
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.custom_file_path("..\\");
    /// test.write_log("success");
    /// ```

    pub fn custom_file_path(&mut self, filepath: &str) -> &mut Self {
        self.dir = filepath.to_string();
        self.update_path();
        self
    }

    /// Changes the name of the log file to "log.log"
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.file_name("log.log");
    /// test.write_log("success");
    /// ```

    pub fn file_name(&mut self, filename: &str) -> &mut Self {
        self.name = filename.to_string();
        self.update_path();
        self
    }

    /// Writes out the custom message to the log file
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.write_log("success");
    /// ```

    pub fn write_log(&mut self, message: &str)
    {
        use std::io::Write;
        let file_wrapped = std::fs::OpenOptions::new().write(true).create(true).append(true).open(&self.path);
        if file_wrapped.is_err()
        {
            return;
        }
        
        let mut file = file_wrapped.unwrap();
        let new_line = format!("{} | Message: {}\n", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S"), message);
        println!("{}", new_line);
        let _ = file.write(new_line.as_bytes());
        let _ = file.flush();
    }

    /// Removes the log file, using the write log again will bring it back. 
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.write_log("success");
    /// test.clear_log();
    /// ```
    pub fn clear_log(&mut self)
    {
        let _ = std::fs::remove_file(&self.path);
    }


    /// Removes the log file if larger than size in bytes, using the write log again will bring it back. 
    /// # Examples
    /// ```
    /// let mut test = davids_awesome_library::log::LogFile::new();
    /// test.write_log("success");
    /// test.clear_log_larger_than_size(1000);
    /// ```
    /// This would only delete the log file if it is larger than 1kb
    #[cfg(target_os="windows")]
    pub fn clear_log_larger_than_size(&mut self, size: u64) -> std::io::Result<()>
    {
        let filesize = std::fs::metadata(&self.path)?.file_size();
        if filesize > size
        {
            self.clear_log();
        }
        Ok(())
    }
    

    //Updates the information needed so that writing out the logs is up to date
    fn update_path(&mut self) -> &mut Self
    {
        //println!("{}", format!("{}{}", self.dir, self.name));
        self.path = std::ffi::OsString::from(format!("{}{}", self.dir, self.name));
        self
    }
}