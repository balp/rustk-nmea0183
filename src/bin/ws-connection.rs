// WebSocket nmea0183 signalK provider

extern crate ws;


use std::rc::Rc;
use std::cell::Cell;
use std::thread;
use std::time;

use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

use serde_json::json;

struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

impl Server {
    fn new(out: ws::Sender,
           count: Rc<Cell<u32>>,) -> Self {
        let out_inner = out.clone();
        thread::spawn(move || {
            let mut value = 1000;
            loop {
                println!("Send update");
                value += 1;
                let update = json!({
                    "context": "vessels.urn:mrn:imo:mmsi:1234567890",
                    "updates": [{
                        "source": {
                            "label": "N2000-01",
                            "type": "NMEA2000",
                            "src": "115",
                            "pgn": 128275
                        },
                        "values": [
                        {
                            "path": "navigation.trip.log",
                            "value": value
                        },
                        {
                            "path": "navigation.log",
                            "value": 1000 + value
                        }
                        ]
                    }
                    ]
                });
                match out_inner.send(update.to_string()) {
                    Ok(_) => (),
                    Err(_) => println!("Unable to send update")
                }
                thread::sleep(time::Duration::from_millis(1 * 1000));
            }

        });


        Self {
            out: out,
            count: count,
        }
    }
}

impl Handler for Server {


    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let hello = json!({
            "name": "rustL-nmea0183-json",
            "version": "0.0.1",
            "roles": ["slave"]
        });
        println!("New connection {}, {}", self.count.get(), shake.request);
        match self.out.send(hello.to_string()) {
            Ok(_) => (),
            Err(_) => println!("Unable to send Hello")
        }
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Tell the user the current count
        println!("The number of live connections is {}", self.count.get());

        // Echo the message back
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }

}

fn main() {
  // Cell gives us interior mutability so we can increment
  // or decrement the count between handlers.
  // Rc is a reference-counted box for sharing the count between handlers
  // since each handler needs to own its contents.
  let count = Rc::new(Cell::new(0));
  listen(
      "0.0.0.0:14123",
      |out| Server::new(out, count.clone())
  ).unwrap()
}
