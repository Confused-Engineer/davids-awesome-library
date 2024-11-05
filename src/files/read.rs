use std::io::{self, Read};
use super::File;

impl File
{


    /// Set the file that will be analyzed/read, then gets the last (non-empty) line in the file 
    /// # Examples
    /// ```
    /// let mut file = davids_awesome_library::files::File::new();
    /// let result: Result<String, std::io::Error> = file.set_file("C:\\Windows\\WindowsUpdate.log").last_file_line();
    /// ```
    pub fn last_file_line(&mut self) -> io::Result<String>
    {
        let file = self.file_to_string()?;

        let last_line = file.split("\n")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .last();
        
        if last_line.is_none()
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is Empty"));
        }
        
        let last_line = last_line.unwrap().to_string();

        Ok(last_line)
    }

    /// Set the file that will be analyzed/read, then gets the first (non-empty) line in the file 
    /// # Examples
    /// ```
    /// let mut file = davids_awesome_library::files::File::new();
    /// let result: Result<String, std::io::Error> = file.set_file("C:\\Windows\\WindowsUpdate.log").first_file_line();
    /// ```
    
    pub fn first_file_line(&mut self) -> io::Result<String>
    {
        let file = self.file_to_string()?;

        let first_line = file.split("\n")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .next();
        
        if first_line.is_none()
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File is Empty"));
        }
        
        let first_line = first_line.unwrap().to_string();

        Ok(first_line)
    }

    /// Set the file that will be analyzed/read, then gets the first (non-empty) line in the file 
    /// # Examples
    /// ```
    /// let mut file = davids_awesome_library::files::File::new();
    /// let result: Result<String, std::io::Error> = file.set_file("C:\\temp\\loggg.log").find_line_by_text("MainEngineThread is returning");
    /// ```
    
    pub fn find_line_by_text(&mut self, text: &str) -> io::Result<String>
    {
        let file = self.file_to_string()?;


        let first_line = file.split("\n")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty());
        
        for line in first_line
        {
            if line.contains(text)
            {
                return Ok(line.to_string());
            }
        }

        return Err(io::Error::new(io::ErrorKind::InvalidData, "String not Found"));
    }

    /// Reads the metadata to determine the amount of time that has passed between now and the last write in seconds
    pub fn time_since_last_write(&mut self) -> io::Result<u64>
    {
        let file = std::fs::File::open(&self.file)?;
        let last_modified_date = std::fs::File::metadata(&file)?
        .modified()?
        .elapsed().unwrap().as_secs();

        return Ok(last_modified_date);


    }


    /// Converts a file to a string so it can be read.
    fn file_to_string(&mut self) -> io::Result<String>
    {
        if self.is_utf16le
        {

            let mut raw_file = std::fs::File::open(&self.file)?;
            let mut buffer = Vec::new();
            raw_file.read_to_end(&mut buffer).unwrap();
            let file = Self::decode_utf16le(buffer);
            return Ok(file);
        }
        
        std::fs::read_to_string(&self.file)
        
    }

    /// Because Rust assumes a file is UTF-8, extra work needs to be done to convert UTF-16LE to a String
    fn decode_utf16le(buf: Vec<u8>) -> String {
        let enc = encoding_rs::Encoding::for_label("utf-16le".as_bytes());
        let mut dec = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(enc)
            .build(&buf[..]);
        let mut res = String::new();
        dec.read_to_string(&mut res).unwrap();
        res
    }


}