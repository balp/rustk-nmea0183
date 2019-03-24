// TCP-IP Server for signalK

use std::net::{TcpStream, TcpListener};
use std::io::{Write};

use serde_json::json;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:14124").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let hello = json!({
                     "name": "rustL-nmea0183-json",
                     "version": "0.0.1",
                     "roles": ["slave"]
                });
                let update = json!({
  "context": "vessels.urn:mrn:imo:mmsi:234567890",
  "updates": [
    {
      "source": {
        "label": "N2000-01",
        "type": "NMEA2000",
        "src": "115",
        "pgn": 128275
      },
      "values": [
        {
          "path": "navigation.trip.log",
          "value": 43374
        },
        {
          "path": "navigation.log",
          "value": 17404540
        }
      ]
    }
  ]
});

                stream.write(update.to_string().as_bytes())
                    .expect("Response failed");
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
