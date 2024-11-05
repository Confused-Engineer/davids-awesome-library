/// A library for logging information.
pub mod log;

/// A collection of tools to navigate the windows registry.
/// Only compiles for Windows.
#[cfg(target_os = "windows")] 
pub mod windows_registry;
/// Used for interating with files
/// Use .new() then .set_file("") to set the file used to read/write 
pub mod files;
pub mod folders;
pub mod env;
pub mod yaml;
pub mod database;

#[cfg(test)]
mod tests {

    

    use super::*;

    #[test]
    fn log_test() {

        let mut test = log::LogFile::new();
        test.write_log("great success");
        std::thread::sleep(std::time::Duration::from_secs(5));
        test.clear_log();

    }

    #[cfg(target_os="windows")]
    #[test]
    fn get_product_id()
    {
        
        let mut msi = windows_registry::Applications::new();
        let product_id = msi.find_msi_product_id("vs_community");
        if product_id.is_err()
        {
            println!("product_id_in_error");
            
        } else {
            let product_ids = product_id.unwrap();

            for (key, val) in product_ids
            {
                println!("{}, {}", key, val);
            }
        }

    }

    #[cfg(target_os="windows")]
    #[test]
    fn get_app_details()
    {
        
        let mut app = windows_registry::Applications::new();
        let app_details_wrapped = app.find_app_details("Samsung USB Driver for Mobile Phones");
        if app_details_wrapped.is_err()
        {
            println!("product_id_in_error");
            
        } else {
            let app_details = app_details_wrapped.unwrap();
            
            for (key, val) in app_details
            {
                println!("{}, {:#?}", key, val);
            }
        }

    }
}
