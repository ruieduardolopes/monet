pub mod options;
pub mod subcommands;

fn main() {
    let cliopts = options::get_options_from_cli();

    match cliopts.subcommand_name() {
        Some(subcommand) => match subcommand {
            "client" => match subcommands::client::init() {
                Ok(_) => {},
                Err(_) => {},
            },
            "server" => match subcommands::server::init() {
                Ok(_) => {},
                Err(_) => {},
            },
            _ => panic!("The inserted subcommand {} does not belong to the list of subcommands.", subcommand),
        }
        None => panic!("No subcommand was inserted. This execution needs one to proceed.")
    }
}
