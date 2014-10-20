// Theme: Threading and messaging.

use std::comm;
use std::task;

pub fn main() {
    let (tx, rx) = comm::channel(); /*
         ~~  ~~    ~~~~~~~~~~~~~     *
         |   |          |            *
         |   |    Returns a pair of  *
         |   |    endpoints          *
         |   |                       *
         |  Receive messages here    *
         |                           *
        Send messages here           */

    task::spawn(proc() {            /*
    ~~~~~~~~~~~ ~~~~                 *
        |        |                   *
        |     Thread body.           *
        |     Takes ownership of     *
        |     things it uses         *
        |     (e.g. `tx`).           *
        |                            *
    Starts a new thread.             */

        let mut factorials = Vec::new();
        let mut i = 0;
        loop {
            let f = factorial(i);
            if f > 128 {
                break;
            }

            factorials.push(f);
            i += 1;
        }
        tx.send(factorials);
    });

    println!("... do something here ...");

    let f = rx.recv_opt().unwrap();

    println!("factorials up to 128 are `{}`", f);
}

fn factorial(n: uint) -> uint {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

