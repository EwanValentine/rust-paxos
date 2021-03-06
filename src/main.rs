use std::env;
use std::io::prelude::*;
use std::collections::HashMap;

mod transport;
use crate::transport::transport::{Adapter, Transport};
use crate::transport::tcp::Tcp;
use std::error::Error;
use async_std::prelude::*;
use async_std::io;

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
struct NodeManager<'a> {
    nodes: HashMap<String, Node>,
    transport: Box<Transport<'a>>,
}

impl NodeManager<'_> {
    fn new(
        nodes: HashMap<String, Node>,
        transport: Box<Transport>,
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

struct Server<'a> {
    addr: String,
    manager: NodeManager<'a>,
    pub transport: &'a Transport<'a>,
}

impl<'a> Server<'a> {

    // new takes the node manager, which stores a list of references to each node
    // it also takes a transport adapter, for spinning up the leader node. The communication
    // between client to leader node, and leader node to follower nodes, are decoupled,
    // so that they can use different protocols.
    pub fn new(manager: NodeManager, transport: &'a Transport) -> Self {
        Server {
            addr: "localhost:2222".to_string(),
            manager,
            transport,
        }
    }

    pub fn listen(self) -> std::io::Result<()> {
        self.transport.listen(|res| {
            let msg = res.to_ascii_lowercase();
            println!("Test: {:?}", msg)
        })
    }

    pub fn connect<T>(&mut self, addr: String) -> std::io::Result<()> {
        let stream: std::io::Result<T> = self.transport.connect(addr);
        match stream {

            Ok(t) => self.transport.write(format!("connect->{}", addr).as_bytes())?,
            _ => {}
        }
        Ok(())
    }

    fn callback(&mut self, msg: String) {

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

#[async_std::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // Create state for desired count
    // and node status. I.e n reachable nodes, include heart beat perhaps

    // Bi-modal operation, leader/follower.
    // Master must self-register and await servants
    // Servants must take a master ip address and join

    // Leader
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

        let t = match Tcp::connect(String::from("localhost:5454")) {
          Ok(adapter) => adapter,
          Err(err) => {},
        };

        let transport: Box<Transport> = Transport::new(&t);
        let manager: NodeManager = NodeManager::new(
            nodes,
            transport,
        );

        let transport: Box<Transport> = Transport::new(&t);

        // Takes a server connection
        let mut server = Server::new(manager, &transport);
        match server.connect::<Tcp>(String::from("localhost:5454")) {
            Ok(()) => println!("server running"),
            Err(e) => panic!("failed to start: {:?}", e),
        }
    }

    if args[1] == String::from("servant") && args[2] != String::from("") {
        println!("Servant mode");

        let t = Tcp::connect(String::from("localhost:5454"));
        let transport: Box<Transport> = Transport::new(&t);
        let manager: NodeManager = NodeManager::new(
            HashMap::new(),
            transport,
        );

        // let api_conn: Tcp = Tcp::connect(args[2].to_string());

        let transport: Box<Transport> = Transport::new(&t);

        let mut client = Server::new(manager, &transport);
        match client.connect::<Tcp>(String::from("localhost:5454")) {
            Ok(()) => println!("Connected to master node on: {:?}", args[2]),
            Err(e) => panic!("failed to connect to master node on: {:?} error: {:?}", args[2], e),
        }
    }

    println!("Hello, world!");

    Ok(())
}
