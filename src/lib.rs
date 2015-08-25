pub mod tcp;

extern crate rustc_serialize;

#[cfg(test)]
mod test {
    use tcp;
    use std::collections::HashMap;

    #[test]
    fn tcp_write() {
        let mut object = HashMap::new();
        object.insert("key", "value");

        let mut fluentd = tcp::Fluentd::new("0.0.0.0:24224").unwrap();
        fluentd.write("foo", &object).unwrap();
    }
}
