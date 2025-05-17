// discovery.rs
use std::collections::HashMap;

pub fn match_service(path: &str) -> Option<&'static str> {
    let mut map = HashMap::new();
    map.insert("/service-a", "http://localhost:4001");
    map.insert("/service-b", "http://localhost:4002");

    for (route, addr) in map {
        if path.starts_with(route) {
            return Some(addr);
        }
    }

    None
}
