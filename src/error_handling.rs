fn connect_database(host: Option<String>) {
    match host {
        Some(host_name) => println!("Connecting to database: {host_name}"),
        None => panic!("No database provided"),
    }
}

fn connect_cache(host: &Option<String>) -> Result<String, String> {
    match host {
        Some(host_name) => Ok(format!("Connected to cache: {host_name}")),
        None => Err(format!("No cache available"))
    }
}

fn connect_mq(host: &Option<String>) -> Result<String, String> {
    match host {
        Some(host_name) => Ok(format!("Connected to mq: {host_name}")),
        None => Err(format!("No mq provided"))
    }
}

fn connect_app(host: Option<String>) -> Result<String, String> {
    connect_cache(&host)?;
    connect_mq(&host)?;
    Ok("Apps connected".to_string())
}

#[test]
fn test_unrecoverable_error() {
    connect_database(Some(String::from("localhost")));
    connect_database(None);
}

#[test]
fn test_recoverable_error() {
    let cache = connect_cache(&Some(String::from("Redis")));
    match cache {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}"),
    }
    let cache = connect_cache(&None);
    match cache {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}"),
    }
}

#[test]
fn test_connect_app() {
    let app = connect_app(Some("localhost".to_string()));
    match app {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}")
    } 
}