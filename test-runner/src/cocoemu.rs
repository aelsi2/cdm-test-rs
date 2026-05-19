use anyhow::{Context, Result, anyhow};
use nix::sys::signal::{self, SIGINT};
use nix::unistd::Pid;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    path::Path,
    process::{Child, Command, Stdio},
};
use tungstenite::{Message, WebSocket, connect, stream::MaybeTlsStream};

pub struct Cocoemu {
    child: Child,
    websocket: WebSocket<MaybeTlsStream<TcpStream>>,
}

#[derive(Debug, Deserialize, PartialEq)]
enum Status {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "FAIL")]
    Failure,
}

#[derive(Debug, Deserialize)]
pub struct Response<B> {
    #[serde(rename = "status")]
    status: Status,
    #[serde(flatten)]
    body: Option<B>,
}

#[derive(Debug, Deserialize)]
pub struct Registers {
    #[serde(rename = "registers")]
    registers: Vec<i32>,
}

impl<B> Response<B> {
    fn body(self) -> Result<B> {
        self.body.ok_or(anyhow!("Cocoemu response has no body"))
    }
}

impl Cocoemu {
    const APP_NAME: &'static str = "cocoemu-server";

    fn connect(child: &mut Child, port: u16) -> Result<WebSocket<MaybeTlsStream<TcpStream>>> {
        let stdout = child
            .stdout
            .take()
            .context("Failed to open cocoemu-server stdout")?;
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            if line.contains("Started debug server") {
                break;
            }
        }

        let uri = format!("ws://localhost:{}", port);
        let (websocket, _) = connect(&uri).context("Could not establish websocket connection")?;
        Ok(websocket)
    }

    fn send_message<T: Serialize>(&mut self, message: &T) -> Result<()> {
        let json = serde_json::to_string_pretty(message).unwrap();
        self.websocket.write(Message::Text(json.into()))?;
        self.websocket.flush()?;
        Ok(())
    }

    fn recv_response<B>(&mut self) -> Result<Response<B>>
    where
        B: DeserializeOwned,
    {
        let response : Response<B> =
            loop {
                let message = self.websocket.read()?;
                match message {
                    Message::Text(bytes) => break Ok(serde_json::from_slice(bytes.as_bytes())
                        .context("Could not parse message")?),
                    Message::Ping(bytes) => self.websocket.send(Message::Pong(bytes))?,
                    _ => break Err(anyhow!("Unexpected message: {:?}", message)),
                };
            }?;
        if response.status == Status::Ok {
            Ok(response)
        } else {
            Err(anyhow!("Cocoemu error"))
        }
    }

    fn initialize(&mut self) -> Result<()> {
        self.send_message(&json!({
            "action": "init",
            "target": "cdm16",
            "memoryConfiguration": "vonNeumann",
        }))?;
        self.recv_response::<()>()?;
        Ok(())
    }

    pub fn new(port: u16) -> Result<Self> {
        let mut child = Command::new(Self::APP_NAME)
            .arg("-p")
            .arg(format!("{}", port))
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("Could not spawn cocoemu-server")?;
        let websocket = match Self::connect(&mut child, port) {
            Ok(ws) => ws,
            Err(error) => {
                _ = signal::kill(Pid::from_raw(child.id() as i32), SIGINT);
                _ = child.wait();
                return Err(error);
            }
        };
        let mut cocoemu = Self { child, websocket };
        cocoemu.initialize()?;
        Ok(cocoemu)
    }

    pub fn run(&mut self, path: &Path) -> Result<bool> {
        self.send_message(&json!({
            "action": "reset",
        }))?;
        self.recv_response::<()>()?;

        self.send_message(&json!({
            "action": "load",
            "source": "path",
            "path": path.display().to_string(),
        }))?;
        self.recv_response::<()>()?;

        self.send_message(&json!({
            "action": "run",
            "stopConditions": [],
        }))?;
        self.recv_response::<()>()?;
        self.recv_response::<()>()?;

        self.send_message(&json!({
            "action": "getRegisters",
        }))?;
        let registers = self.recv_response::<Registers>()?.body()?.registers;
        if registers.is_empty() {
            return Err(anyhow!("Received empty register array from cocoemu-server"));
        }
        Ok(registers[0] == 0)
    }
}

impl Drop for Cocoemu {
    fn drop(&mut self) {
        _ = signal::kill(Pid::from_raw(self.child.id() as i32), SIGINT);
        _ = self.child.wait();
    }
}
