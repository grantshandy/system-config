use system_config::Config;

fn main() {
    Config::write_clear_by_name("system-config-example").unwrap();
    Config::write_clear_by_name("system-config-multiple-example").unwrap();
}