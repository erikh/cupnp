use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(version, about="cupnp: Manage uPnP port forwards easily on the CLI", long_about=None)]
struct MainArgs {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
	#[command(about = "Expose a Port from your Router to this Machine")]
	Expose(ExposePortArgs),
	#[command(about = "Delete a Port Forward (Any Port) on your Router")]
	Delete(DeletePortArgs),
}

#[derive(Parser, Debug, Clone)]
struct ExposePortArgs {
	#[arg(help = "Port to Expose")]
	port: u16,
	#[arg(help = "Protocol to Expose Port on: either 'tcp' or 'udp'")]
	protocol: Option<cupnp::Protocol>,
	#[arg(help = "Duration of Lease in seconds; this is not respected by a lot of home routers")]
	duration: Option<u32>,
}

#[derive(Parser, Debug, Clone)]
struct DeletePortArgs {
	#[arg(help = "Port to delete")]
	port: u16,
	#[arg(help = "Protocol of Port to Delete: either 'tcp' or 'udp'")]
	protocol: Option<cupnp::Protocol>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = MainArgs::parse();
	match args.command {
		Command::Expose(args) => {
			let protocol = args.protocol.unwrap_or_default();
			cupnp::expose_port(args.port, protocol.clone(), args.duration)?;
			println!(
				"Exposed port {} over {}{}",
				args.port,
				protocol,
				if let Some(duration) = args.duration {
					&format!(", leased for {} seconds", duration)
				} else {
					""
				}
			);
		}
		Command::Delete(args) => {
			let protocol = args.protocol.unwrap_or_default();
			cupnp::delete_port(args.port, protocol.clone())?;

			println!("Deleted port {} on {}", args.port, protocol);
		}
	}
	Ok(())
}
