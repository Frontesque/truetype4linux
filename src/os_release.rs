use std::fs::read_to_string;

pub fn get_full() -> Vec<String> {
    let os_release = read_to_string("/etc/os-release").expect("Unable to open file: '/etc/os-release'");
    let instances = os_release.lines().collect();
    return instances;
}

pub fn get_id() {
    let full = get_full();
}