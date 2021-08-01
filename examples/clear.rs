use system_config::Config;

fn main() {
    let mut config = Config::new("system-config-example").unwrap();

    config.write_clear().unwrap();
}