use system_config::Config;

fn main() {
    let config = Config::new("system-config-example").unwrap();

    let key1 = config.get("key1").unwrap();
    let key2 = config.get("key2").unwrap();

    println!("key1: {}", key1);
    println!("key2: {}", key2);
}