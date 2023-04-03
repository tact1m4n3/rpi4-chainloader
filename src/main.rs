use std::{
    fs,
    io::{self, Read, Write},
    time::Duration,
};

use clap::Parser;
use serialport::TTYPort;

type Result<T> = std::result::Result<T, anyhow::Error>;

const NOTIF: &[u8] = b"OK";

#[derive(Parser)]
struct Args {
    serial_path: String,
    payload_path: String,
    #[arg(short, long, default_value_t = 115200)]
    baud_rate: u32,
}

struct App {
    payload: Vec<u8>,
    port: TTYPort,
}

impl App {
    fn new(args: Args) -> Result<Self> {
        let payload = fs::read(args.payload_path)?;
        let port = serialport::new(&args.serial_path, args.baud_rate)
            .timeout(Duration::MAX)
            .open_native()?;
        Ok(Self { payload, port })
    }

    fn run(&mut self) -> Result<()> {
        println!("[I] waiting for device");
        self.wait_for_device()?;
        println!("[I] device ready for transfer");

        println!("[I] sending payload");
        self.send_payload()?;

        println!("[I] printing serial");
        self.print_serial()?;

        Ok(())
    }

    fn wait_for_device(&mut self) -> Result<()> {
        let mut buf = [0; 2];
        loop {
            self.port.read_exact(&mut buf)?;
            if buf == NOTIF {
                break Ok(());
            }
        }
    }

    fn send_payload(&mut self) -> Result<()> {
        self.port
            .write_all(&(self.payload.len() as u32).to_le_bytes())?;
        self.port.write_all(self.payload.as_slice())?;
        Ok(())
    }

    fn print_serial(&mut self) -> Result<()> {
        let mut buf = [0; 100];
        loop {
            let i = self.port.read(&mut buf)?;
            io::stdout().write_all(&buf[0..i])?;
        }
    }
}

fn main() {
    let args = Args::parse();
    match App::new(args) {
        Ok(mut app) => {
            if let Err(err) = app.run() {
                println!("[E] {err}");
            }
        }
        Err(err) => println!("[E] {err}"),
    }
}
