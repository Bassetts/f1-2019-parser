use std::error::Error;

use f1_telemetry_parser::{parse_packet, TelemetryData, MAXIMUM_PACKET_SIZE};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:20777";
    let mut socket = UdpSocket::bind(&addr).await?;

    loop {
        let mut buf: Vec<u8> = vec![0; MAXIMUM_PACKET_SIZE];
        if socket.recv(&mut buf).await? != 0 {
            tokio::spawn(async move {
                let packet = match parse_packet(&buf) {
                    Ok(packet) => packet,
                    Err(error) => return println!("{}", error),
                };

                match packet.data {
                    TelemetryData::Motion(_data) => {
                        println!("Motion");
                    }
                    TelemetryData::Session(_data) => {
                        println!("Session");
                    }
                    TelemetryData::Lap(_data) => {
                        println!("Lap");
                    }
                    TelemetryData::Event(_data) => {
                        println!("Event");
                    }
                    TelemetryData::Participants(_data) => {
                        println!("Participants");
                    }
                    TelemetryData::CarSetups(_data) => {
                        println!("CarSetups");
                    }
                    TelemetryData::CarTelemetry(_data) => {
                        println!("CarTelemetry");
                    }
                    TelemetryData::CarStatus(_data) => {
                        println!("CarStatus");
                    }
                }
            });
        }
    }
}
