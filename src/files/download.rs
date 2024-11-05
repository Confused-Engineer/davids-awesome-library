use std::io;

use super::File;

impl File
{
    /// Downloads  file from a URL and downloads it to the location specified by 'set_file(...)'
    pub fn download_from(&mut self, uri: &str) -> std::io::Result<()>
    {
        // Set the file path we will use from the Struct
        let file_path = self.file.clone();

        // This will download the file to memory
        let response = reqwest::blocking::get(uri);
        
        // Make sure the download succeeded and did not return an error
        if response.is_err()
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Could Download File"));
        }

        // Set assign a variable to the downloaded bytes
        let mut data_bytes = response.unwrap();

        // Remove the previous file (if it exists)
        if std::path::Path::exists(&std::path::Path::new(&file_path))
        {
            let _ = std::fs::remove_file(file_path.clone());
        }
        
        // Create the file
        let mut out_file = std::fs::File::create(file_path)?;

        // Write the bytes to the file
        let _ = io::copy(&mut data_bytes, &mut out_file)?;

        // Return 'ok' (success)
        Ok(())
    }

}

