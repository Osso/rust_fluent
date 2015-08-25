extern crate rust_fluent;
use rust_fluent::tcp;

use std::collections::HashMap;


fn main() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("key".to_string(), "value".to_string());

    let mut fluentd = tcp::Fluentd::new("0.0.0.0:24224").unwrap();
    fluentd.write("foo", &obj).unwrap();
}
