use port_check;
use rand::{rng, Rng};
use std::{collections::HashMap, io::Error, vec};

pub struct PortsFound {
    backend: Option<u16>,
    frontend: Option<u16>,
}

const HOST: &str = "http://localhost";
const DEFAULT_PORTS: PortsFound = PortsFound {
    backend: Some(5009),
    frontend: Some(5008),
};

fn generate_port_in_range(
    min: u16,
    max: u16,
    skip: Vec<u16>,
    check_port: bool,
) -> Result<u16, Error> {
    let skip_set: std::collections::HashSet<u16> = skip.into_iter().collect();
    let mut rng = rng();

    let available_ports: Vec<u16> = (min..=max)
        .filter(|port| !skip_set.contains(port))
        .collect();

    if available_ports.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No available ports in range",
        ));
    }

    let port = available_ports[rng.random_range(0..available_ports.len())];

    if check_port {
        if port_check::is_port_reachable(HOST.to_string() + ":" + port.to_string().as_str()) {
            return Ok(port);
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::AddrNotAvailable,
            "Port is used or unavailable",
        ));
    }
    Ok(port)
}

pub fn get_port() -> Result<PortsFound, std::io::Error> {
    let mut ports = PortsFound {
        backend: None,
        frontend: None,
    };

    // Check if 5009 and 5008 available
    if port_check::is_port_reachable(
        HOST.to_string() + ":" + DEFAULT_PORTS.backend.unwrap().to_string().as_str(),
    ) {
        ports.backend = Some(DEFAULT_PORTS.backend.unwrap())
    }
    if port_check::is_port_reachable(
        HOST.to_string() + ":" + DEFAULT_PORTS.frontend.unwrap().to_string().as_str(),
    ) {
        ports.backend = Some(DEFAULT_PORTS.frontend.unwrap())
    }
    if let (Some(frontend), Some(backend)) = (ports.backend, ports.frontend) {
        return Ok(ports);
    }

    // Get some different port
    ports.backend = Some(generate_port_in_range(5000, 6000, vec![], true).unwrap());

    ports.frontend = Some(
        generate_port_in_range(
            5000,
            6000,
            if ports.backend.is_some() {
                vec![ports.backend.unwrap()]
            } else {
                vec![]
            },
            true,
        )
        .unwrap(),
    );

    if ports.backend.is_some() && ports.frontend.is_some() {
        return Ok(ports);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotConnected,
        "No Ports found",
    ))
}
