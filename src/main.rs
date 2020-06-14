extern crate tokio;

use std::env;
use std::io::prelude::*;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

mod transport;
use std::borrow::Borrow;
use crate::transport::transport::{Transport, Adapter};
use crate::transport::tcp::Tcp;
use std::error::Error;

// Desired count
// n = 2F + 1 where F is failed/inactive nodes

// enum Mode {
    // MASTER,
    // SERVANT,
// }

#[derive(Debug)]
enum Status {
    WELL,
    AILING,
}

#[derive(Debug)]
struct Node {
    health: Status,
    addr: String,
}

#[derive(Debug)]
struct NodeManager {
    nodes: HashMap<String, Node>,
    transport: Box<Adapter>,
}

impl NodeManager {
    fn new(
        nodes: HashMap<String, Node>,
        transport: Box<dyn Adapter>,
    ) -> NodeManager {
        NodeManager {
            nodes,
            transport,
        }
    }

    fn join(&mut self, id: String, node: Node) {
        self.nodes.insert(id, node);
    }

    fn leave(&mut self, id: String) {
        self.nodes.remove(&id);
    }

    fn heart_beat(_n: Node) {
        // n.addr
    }

    fn poll(&mut self) {
        // @todo
        // self.nodes.each(|n| self.heart_beat(n));
    }

    // Every time a node joins or leaves, must check
    // each node complies with 2F + 1
}

struct Server {
    addr: String,
    manager: NodeManager,
    pub transport: Box<dyn Adapter>,
}

impl Server {

    // new takes the node manager, which stores a list of references to each node
    // it also takes a transport adapter, for spinning up the leader node. The communication
    // between client to leader node, and leader node to follower nodes, are decoupled,
    // so that they can use different protocols.
    pub fn new(manager: NodeManager, transport: Box<dyn Adapter>) -> Self {
        Server {
            addr: "localhost:2222".to_string(),
            manager,
            transport,
        }
    }

    pub fn start(&self, addr: String) -> std::io::Result<()> {
        self.transport.connect(addr);
        Ok(())
    }

    pub fn connect(&mut self, addr: String) -> std::io::Result<()> {
        let mut tr: Transport = self.manager.transport.

        stream.write(format!("connect->{}", addr).as_bytes())?;
        Ok(())
    }

    fn handle(&mut self, msg: String) {

        // Example connect->localhost:1234
        let mut parts = msg.split("->").fuse();
        let cmd = parts.next();
        let msg = parts.next();
        let m = msg.map(|s| s.to_string()).unwrap();

        match cmd {
            Some("connect") => {                
                println!("New node connected: {}", m);
                self.manager.join("new-node".to_string(), Node {
                    addr: m,
                    health: Status::WELL,
                });

                println!("Nodes: {:?}", self.manager.nodes);
            },
            Some("disconnect") => {
                let addr: String = msg.map(|s| s.to_string()).unwrap();
                self.manager.leave(addr);
            },
            Some("health") => {},
            _ => println!("Nothing to be done with this message: {:?}", cmd),
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // Create state for desired count
    // and node status. I.e n reachable nodes, include heart beat perhaps

    // Bi-modal operation, master/servant.
    // Master must self-register and await servants
    // Servants must take a master ip address and join

    // Master
    // Start TCP server
    // Register

    println!("{:?}", args);

    if args[1] == String::from("master") {
        println!("Running in master mode");

        let master_node = Node {
            addr: "localhost:2222".to_string(),
            health: Status::WELL,
        };

        let mut nodes = HashMap::new();
        nodes.insert(String::from("master"), master_node);

        let adapter = Box::from(Tcp::new());
        let manager = NodeManager::new(
            nodes,
            adapter,
        );

        let adapter = Box::from(Tcp::new());
        let mut server = Server::new(manager, adapter);
        match server.start(String::from("localhost:2222")) {
            Ok(()) => println!("server running"),
            Err(e) => panic!("failed to start: {:?}", e),
        };
    }

    if args[1] == String::from("servant") && args[2] != String::from("") {
        println!("Servant mode");

        let adapter = Box::from(Tcp::new());
        let manager = NodeManager::new(
            HashMap::new(),
            adapter,
        );

        let adapter = Box::from(Tcp::new());
        // @todo - should handle finding an available port more gracefully here...
        let mut client = Server::new(manager, adapter);
        match client.connect(args[2].to_string()) {
            Ok(()) => println!("Connected to master node on: {:?}", args[2]),
            Err(e) => panic!("failed to connect to master node on: {:?} error: {:?}", args[2], e),
        }

        // @todo - need to configure this
        match client.start("localhost:2223".to_string()) {
            Ok(()) => println!("Connected to server"),
            Err(e) => println!("Error connecting to server: {:?}", e),
        }
    }

    println!("Hello, world!");

    Ok(())
}
