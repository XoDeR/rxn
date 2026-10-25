use rx_app::WinitRunner;
use rx_project::ProjectConfig;

fn main() {
    if let Err(err) = WinitRunner::new().run(ProjectConfig::default_for_name("Rx")) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
