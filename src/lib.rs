use std::str::FromStr;

use anyhow::{Result, anyhow};
pub use easy_upnp::{PortMappingProtocol, UpnpConfig};

#[derive(Debug, Clone, Default)]
pub enum Protocol {
	#[default]
	TCP,
	UDP,
}

impl std::fmt::Display for Protocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::UDP => "udp",
			Self::TCP => "tcp",
		})
	}
}

impl From<Protocol> for PortMappingProtocol {
	fn from(value: Protocol) -> Self {
		match value {
			Protocol::TCP => Self::TCP,
			Protocol::UDP => Self::UDP,
		}
	}
}

impl FromStr for Protocol {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"tcp" => Ok(Protocol::TCP),
			"udp" => Ok(Protocol::UDP),
			x => Err(anyhow!("Invalid Protocol `{}`", x)),
		}
	}
}

fn port_forward(port: u16, protocol: Protocol, duration: Option<u32>) -> UpnpConfig {
	UpnpConfig {
		port,
		protocol: protocol.into(),
		address: None,
		duration: duration.unwrap_or(3600),
		comment: "Forward from cupnp CLI tool".into(),
	}
}

fn results(res: Vec<Result<(), easy_upnp::Error>>) -> Result<()> {
	for result in res {
		result?
	}

	Ok(())
}

pub fn expose_port(port: u16, protocol: Protocol, duration: Option<u32>) -> Result<()> {
	results(easy_upnp::add_ports([port_forward(port, protocol, duration)]).collect())
}

pub fn delete_port(port: u16, protocol: Protocol) -> Result<()> {
	results(easy_upnp::delete_ports([port_forward(port, protocol, None)]).collect())
}
