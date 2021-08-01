use system_config::Config;

fn main() {
    println!("writing to config \"system-config-multiple-example\"");
    write();

    println!("reading from config \"system-config-multiple-example\"");
    read();
}

fn write() {
    let mut config = Config::new("system-config-multiple-example").unwrap();

    config.write_insert("1", "value 1").unwrap();
    config.write_insert("2", "value 2").unwrap();
    config.write_insert("3", "value 3").unwrap();

    println!("wrote:\n  1    value 1\n  2    value 2\n  3    value 3\n");
}

fn read() {
    let mut config = Config::new("system-config-multiple-example").unwrap();

    let x1 = config.read_get("1").unwrap().unwrap();
    let x2 = config.read_get("2").unwrap().unwrap();
    let x3 = config.read_get("3").unwrap().unwrap();

    println!("read:\n  1    {}\n  2    {}\n  3    {}\n", x1, x2, x3);
}