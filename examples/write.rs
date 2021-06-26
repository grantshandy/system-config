use system_config::Config;

fn main() {
    let mut config = Config::new("system-config-example").unwrap();

    config.insert("key1", "value1");
    config.insert("key2", "value2");

    config.write().unwrap();
}