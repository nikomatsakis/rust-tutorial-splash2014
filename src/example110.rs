// Theme: Threads and messaging

#![allow(dead_code)]

use std::comm;
use std::comm::{Sender, Receiver};
use std::task;

struct UserRecord {
    name: String, age: uint
}

enum Request {
    UserList(Sender<Vec<String>>),
    Age(String, Sender<Option<uint>>),
}

pub fn main() {
    let (request_tx, request_rx) = comm::channel();
    //   ~~~~~~~~~~  ~~~~~~~~~~    ~~~~~~~~~~~~~
    //       |            |              |
    //       |            |        Create a pair of
    //       |            |        endpoints for requests
    //       |            |
    //       |       Where requests
    //       |       will be received
    //       |
    //   Where requests
    //   should be sent

    let (server_complete_tx, server_complete_rx) = comm::channel();
    //   ~~~~~~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~~~~    ~~~~~~~~~~~~~
    //           |                   |                  |
    //           +-- Channel pair for communicating server is done

    task::spawn(proc() {
        do_server(request_rx);

        server_complete_tx.send(());
    });

    let (client_complete_tx, client_complete_rx) = comm::channel();

    task::spawn(proc() {
        do_client(request_tx);

        client_complete_tx.send(());
    });

    server_complete_rx.recv(); // Wait for server.

    client_complete_rx.recv(); // Wait for client.

    println!("All done!");
}

fn do_server(server_rx: Receiver<Request>) {
    let users = vec![UserRecord { name: format!("Hugo"),
                                  age: 22 },
                     UserRecord { name: format!("Eliza"),
                                  age: 44 },
                     UserRecord { name: format!("Bill"),
                                  age: 66 }];
    loop {
        match server_rx.recv_opt() {
            Err(_) => {
                // Other side hung up.
                println!("Server: client gone");
                return;
            }

            Ok(UserList(respond_to)) => {
                println!("Server: client requested user list");

                let user_names =
                    users.iter()                   // Iterate (by ref) over the UserRecords
                         .map(|r| r.name.clone())  // Create a copy of each name
                    //        ~~~ ~~~~~~~~~~~~~~
                    //         |        |
                    //     Closure    Result of closure
                         .collect();               // Create a vector of the results

                respond_to.send(user_names);
            }

            Ok(Age(name, respond_to)) => {
                println!("Server: client requested age of `{}`", name);

                let opt_age =
                    users.iter()                   // Iterate (by ref) over the UserRecords
                         .find(|r| r.name == name) // Yields an Option<&UserRecord>
                         .map(|r| r.age);          // If Some, extract age, yielding Option<uint>
                respond_to.send(opt_age);
            }
        }
    }
}

fn do_client(_server_tx: Sender<Request>) {
    // Exercise 1: Write a client that requests the list of users and then the age of each one.
}
