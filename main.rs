pub mod db_config;

fn main() {
    let db_conf = db_config::read_config();
    println!("{}", db_conf);
}
