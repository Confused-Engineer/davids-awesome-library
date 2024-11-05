pub fn get_home() -> Result<String, std::io::Error>
{
    #[cfg(target_os="windows")]
    let home = std::env::var_os("userprofile");
    #[cfg(not(target_os="windows"))]
    let home = std::env::var_os("HOME");
    
    if home.is_none()
    {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Local ENV Failed"))
    }

    let home_unwrap = home.unwrap();
    Ok(home_unwrap.to_str().unwrap().trim().to_string())
}

pub fn get_exe_dir() -> Result<String, std::io::Error>
{
    let exe_path = std::env::current_exe()?.display().to_string();
    
    #[cfg(target_os="windows")]
    let exe = exe_path.split("\\").last().unwrap();

    #[cfg(not(target_os="windows"))]
    let exe = exe_path.split("/").last().unwrap();

    let current_path = std::env::current_exe()?.display().to_string().replace(exe, "");
    Ok(current_path)
}



pub fn set_exe_dir() -> std::io::Result<()>
{
    let exe_path = std::env::current_exe()?.display().to_string();
    
    #[cfg(target_os="windows")]
    let exe = exe_path.split("\\").last().unwrap();

    #[cfg(not(target_os="windows"))]
    let exe = exe_path.split("/").last().unwrap();

    let current_path = std::env::current_exe()?.display().to_string().replace(exe, "");
    let _ = std::env::set_current_dir(current_path);
    Ok(())
}

#[cfg(test)]
mod tests {

    

    use super::*;

    #[test]
    fn home() {
        println!("{}", get_home().unwrap())
    }
}