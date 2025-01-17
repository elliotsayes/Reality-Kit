fn main() {
    let mut single_features = vec![];
    if cfg!(feature = "ao_server") {
        single_features.push("ao_server");
    }
    if cfg!(feature = "ao_client") {
        single_features.push("ao_client");
    }
    if cfg!(feature = "web_client") {
        single_features.push("web_client");
    }
    if cfg!(feature = "local_client") {
        single_features.push("local_client");
    }
    if single_features.len() != 1 {
        panic!("Exactly one of ao_server, ao_client, web_client, or local_client must be selected");
    }
}
