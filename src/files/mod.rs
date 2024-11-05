mod read;
mod download;
mod remove;
mod create;
mod modify;
pub mod size;
/// Used for interating with files
/// Use .new() then .set_file("") to set the file used to read/write  
/// 


pub struct File
{
    file: String,
    is_utf16le: bool
}


impl File
{

    // Create a default empty 'File'
    pub fn new() -> Self
    {
        Self { 
            file: String::new(), 
            is_utf16le: false 
        }
    }

    /// Set the file that will be analyzed/read
    /// # Examples
    /// ```
    /// let mut file = davids_awesome_library::files::File::new();
    /// file.set_file("C:\\Windows\\WindowsUpdate.log");
    /// ```
    pub fn set_file(&mut self, filepath: &str) -> &mut Self
    {
        self.file = filepath.to_string();
        self
    }

    /// Setting this currently only helps with attempting to read a file.
    /// If you attempt to read a UTF-16LE file without setting this, you will receive an error.
    pub fn is_utf16le(&mut self) -> &mut Self
    {
        self.is_utf16le = true;
        self
    }
}

