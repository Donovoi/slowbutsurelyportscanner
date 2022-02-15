// A fast port scanner written in rust

use std::{net::{TcpListener, TcpStream, IpAddr, Ipv4Addr, SocketAddr}, thread, time::Duration};
use std::io::{self, Write};







fn main() {
    // Get the ipaddress user wants to scan and the port range defined like 0-1024 then check if any ports are open for that ip address and return the result
    let ipaddress = get_ipaddress();
    let port_range = get_port_range();
    // Create all possible SocketAddr objects for the given ipaddress and port range
    let mut addrs = get_socket_addrs(ipaddress, &port_range);
    // Create a vector of all the open ports
    let mut open_ports = vec![];
    for port in port_range {
        show_progress(port);

        // Set the timeout for the connection
        let timeout = Duration::from_millis(1000019);
        // Create a TcpStream object for the given SocketAddr
       if let Ok(stream) = TcpStream::connect_timeout(&addrs[0], timeout)
        {
            // If the connection is successful, add the port to the open_ports vector
            open_ports.push(port);

            // Let the user know that this port and ipaddress is open
            println!("Port {} is open on {}", port, ipaddress);
            // Close the connection
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        }
        else {
            // If the ip address doesn't have the port open, print the ip and the port
            println!("Port {} is closed on {}", port, ipaddress);
            
        }
             // Remove the SocketAddr from the vector
             addrs.remove(0);   
       
    }
}

fn get_socket_addrs(ipaddress: IpAddr, port_range: &[u16]) -> Vec<SocketAddr> {
    let mut socket_addrs = vec![];
    for port in port_range {
        socket_addrs.push(SocketAddr::new(ipaddress, *port));
    }
    socket_addrs
}

// Create the get_ipaddress function to get the ipaddress from the user, make sure it is the correct format
fn get_ipaddress() -> IpAddr {
    let mut ipaddress = String::new();
    println!("Enter the ipaddress you want to scan: ");
    std::io::stdin().read_line(&mut ipaddress).expect("Failed to read line");
    let ipaddress = ipaddress.trim();
    let ipaddress: IpAddr = ipaddress.parse().expect("Please enter a valid ipaddress");
    ipaddress
}

// Create the get_port_range function to get the port range from the user. must be the port range defined like 0-1024 function will create a minimum port and maxiumum port and return all ports within that range
fn get_port_range() -> Vec<u16> {
    let mut port_range = String::new();
    println!("Enter the port range you want to scan: ");
    std::io::stdin().read_line(&mut port_range).expect("Failed to read line");
    let port_range = port_range.trim();
    let port_range: Vec<&str> = port_range.split("-").collect();
    let min_port: u16 = port_range[0].parse().expect("Please enter a valid port");
    let max_port: u16 = port_range[1].parse().expect("Please enter a valid port");
    let mut port_range = vec![];
    for port in min_port..max_port {
        port_range.push(port);
    }
    port_range
}

// Show progress of port scan
fn show_progress(port: u16) {
    print!(".");
    io::stdout().flush().unwrap();
}