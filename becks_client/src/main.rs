#![windows_subsystem = "windows"]

fn main() {
    env_logger::init();

    becks_client::run_app();

    becks_network::config::save_config();
}
