use std::env::var;
use std::fs::read_to_string;

fn get_local_machine_id() -> String {
    read_to_string("/var/lib/dbus/machine-id")
        .or_else(|_| read_to_string("/etc/machine-id"))
        .unwrap()
        .trim()
        .to_string()
}

fn get_socket_path() -> String {
    if let Ok(address) = var("IBUS_ADDRESS_FILE") {
        return address;
    }

    // TODO: Support X11
    let display = var("WAYLAND_DISPLAY").unwrap();

    let p = format!("{}-unix-{}", get_local_machine_id(), display);

    let path = var("$XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").and_then(|home| Ok(format!("{}/.config", home))))
        .and_then(|config_dir| Ok(format!("{}/ibus/bus/{}", config_dir, p)))
        .unwrap();

    path
}

fn get_address() -> String {
    if let Ok(address) = var("IBUS_ADDRESS") {
        return address;
    }

    // read address from ~/.config/ibus/bus/socket-file
    let file = read_to_string(get_socket_path()).unwrap();
    let mut lines = file.lines().filter(|x| !x.starts_with('#')); // Filter out the comments.

    // First line is the IBUS_ADDRESS
    let address = lines
        .next()
        .unwrap()
        .strip_prefix("IBUS_ADDRESS=")
        .unwrap()
        .to_string();
    // Next line is the IBUS_DAEMON_PID
    let _pid = lines
        .next()
        .unwrap()
        .strip_prefix("IBUS_DAEMON_PID=")
        .unwrap();

    address
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    
    #[test]
    fn test_socket_file_exists() {
        assert!(read_to_string(get_socket_path()).is_ok());
    }

    #[test]
    fn test_get_address() {
        let address = get_address();
        // The address should have ':' in it.
        assert!(address.find(':').is_some());
    }
}
