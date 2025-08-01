pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}



pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        
        security_level::Unknown => match server {
            Ok(URL) => URL.to_string(),
            Err(_) => panic!("")
        },

        security_level::Message => match server {
            Ok(URL) => URL.to_string(),
            Err(_) => panic!("ERROR: program stops"),
        },
        security_level::Warning => match server {
            Ok(URL) => URL.to_string(),
            Err(_) => "WARNING: check the server".to_string(),
        }
        security_level::NotFound => match server {
            Ok(URL) => URL.to_string(),
            Err(err) => println!("Not found: {}", err).
        }
        security_level::UnexpectedUrl => match server {
            Ok(URL) => URL.to_string(),
            Err(err) => panic!("{}", URL),
        }
    }
}
