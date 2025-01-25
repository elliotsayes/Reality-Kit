fn main() {
    let mut single_features = vec![];
    if cfg!(feature = "server_ao") {
        single_features.push("server_ao");
    }
    if cfg!(feature = "client_ao") {
        single_features.push("client_ao");
    }
    if cfg!(feature = "client_web") {
        single_features.push("client_web");
    }
    if cfg!(feature = "client_local") {
        single_features.push("client_local");
    }
    if single_features.len() != 1 {
        panic!("Exactly one of server_ao, client_ao, client_web, or client_local must be selected");
    }
}
