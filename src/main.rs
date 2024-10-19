use std::env;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

const DEBUG: bool = false;
const CTIMEOUT: Duration = Duration::from_secs(100);
const MAXLEN: usize = 1460;
const CPUIDLEN: usize = 7;
const MAXCLIENTS: usize = 1000;
const VERSION: [u32; 3] = [1, 0, 0];
const IP: &str = "[::]";
const DEFAULT_PORT: u16 = 60050;

#[derive(Clone)]
struct Client {
    cpuid: [u8; CPUIDLEN],
    addr: SocketAddr,
    last_time: SystemTime,
}

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<[u8; CPUIDLEN], Client>> = Mutex::new(HashMap::new());
}

fn find_and_update_client(cpuid: &[u8; CPUIDLEN], addr: SocketAddr) -> Option<Client> {
    let mut clients = CLIENTS.lock().unwrap();
    if let Some(client) = clients.get_mut(cpuid) {
        client.addr = addr;
        client.last_time = SystemTime::now();
        return Some(client.clone());
    }
    None
}

fn add_client(cpuid: [u8; CPUIDLEN], addr: SocketAddr) -> Option<Client> {
    let mut clients = CLIENTS.lock().unwrap();
    let now = SystemTime::now();
    
    if clients.len() < MAXCLIENTS {
        let client = Client {
            cpuid,
            addr,
            last_time: now,
        };
        clients.insert(cpuid, client.clone());
        Some(client)
    } else {
        None
    }
}

fn handle_packet(socket: &UdpSocket, buf: &[u8], addr: SocketAddr) {
    if buf.len() < CPUIDLEN * 2 + 6 || &buf[0..4] != b"NRL2" {
        return;
    }
    
    let sender_cpuid: [u8; CPUIDLEN] = buf[6..6 + CPUIDLEN].try_into().unwrap();
    let receiver_cpuid: [u8; CPUIDLEN] = buf[6 + CPUIDLEN..6 + CPUIDLEN * 2].try_into().unwrap();
    
    if find_and_update_client(&sender_cpuid, addr).is_none() {
        if add_client(sender_cpuid, addr).is_none() {
            println!("[Info] Client full");
            return;
        }
    }
    
    let now = SystemTime::now();
    let clients = CLIENTS.lock().unwrap();
    
    for client in clients.values() {
        if let Ok(duration) = now.duration_since(client.last_time) {
            if duration > CTIMEOUT {
                continue;
            }
        }
        
        if client.cpuid == receiver_cpuid {
            socket.send_to(buf, client.addr).unwrap();
            if DEBUG {println!("[Info] Packet forwarded to: {:?}", client.addr)};
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 2 && args[1] == "-p" {
        args[2].parse::<u16>().unwrap_or(DEFAULT_PORT)
    } else {
        DEFAULT_PORT
    };

    let bind_addr = &format!("{}:{}", IP, port);
    let socket = UdpSocket::bind(bind_addr).expect("[Error] Couldn't bind to address");
    println!("[Info] NRL2730-Rust {}.{}.{} server is running on {}", VERSION[0], VERSION[1], VERSION[2], bind_addr);

    let mut buf = [0u8; MAXLEN];
    
    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf) {
            if DEBUG {println!("[Info] Received packet from: {}", addr)};
            handle_packet(&socket, &buf[..len], addr);
        }
    }
}
