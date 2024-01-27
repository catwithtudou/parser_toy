use std::string::String;

use bytes::{BufMut, BytesMut};
use structopt::StructOpt;

#[derive(Debug, Clone)]
struct CmdBuilder {
    args: Vec<String>,
}

impl CmdBuilder {
    fn new() -> Self {
        CmdBuilder { args: vec![] }
    }

    fn arg(mut self, arg: &str) -> Self {
        self.args.push(format!("${}", arg.len()));
        self.args.push(arg.to_string());
        self
    }

    fn add_arg(&mut self, arg: &str) {
        self.args.push(format!("${}", arg.len()));
        self.args.push(arg.to_string());
    }

    fn to_bytes(&self) -> BytesMut {
        let mut bytes = BytesMut::new();
        // example
        // ping: *1\r\n$4\r\nPING\r\n   -> args:[$4,PING]
        // get key: *2\r\n$3\r\nGET\r\n$3\r\nkey\r\n -> args:[$3,GET,$3,key]
        bytes.put(&format!("*{}\r\n", self.args.len() / 2).into_bytes()[..]);
        bytes.put(&self.args.join("\r\n").into_bytes()[..]);
        bytes.put(&b"\r\n"[..]);
        bytes
    }
}

#[derive(Debug, Clone)]
pub enum ExistOP {
    NX,
    XX,
}

impl std::str::FromStr for ExistOP {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_ascii_lowercase() == String::from("nx") {
            Ok(ExistOP::NX)
        } else if s.to_ascii_lowercase() == String::from("xx") {
            Ok(ExistOP::XX)
        } else {
            Err("unexpected string, 'NX' or 'XX' expected".to_string())
        }
    }
}


#[derive(Debug, Clone, StructOpt)]
pub enum Commands {
    /// test server status
    Ping,

    /// get string value
    Get {
        /// redis key
        key: String,
    },

    /// set a key with string value
    Set {
        /// redis key
        key: String,

        /// redis key value
        value: String,

        /// set key expiration in seconds, exclusive with px
        #[structopt(short, long)]
        ex: Option<u32>,    // SET key value EX 10

        /// set key expiration in milliseconds, exclusive with ex
        #[structopt(short, long)]
        px: Option<u32>,    // SET key value PX 10

        /// existent flag [NX|XX]
        x: Option<ExistOP>,  // SET key value NX / SET key value XX
    },

    /// increase 1
    Incr {
        /// redis key
        key: String,
    },

    /// get list with limit range
    Lrange {
        /// redis key
        key: String,
        /// start position
        start: i64,
        /// end position
        stop: i64,
    },

    /// push value to list
    Rpush {
        /// redis key
        key: String,

        /// value
        values: Vec<String>,
    },
}

impl Commands {
    pub fn to_bytes(&self) -> bytes::BytesMut {
        let cmd = match self {
            Commands::Ping => CmdBuilder::new().arg("PING").to_bytes(),
            Commands::Get { key } => CmdBuilder::new().arg("GET").arg(key).to_bytes(),
            Commands::Set {
                key, value, ex, px, x
            } => {
                let mut builder = CmdBuilder::new().arg("SET").arg(key).arg(value);

                if let Some(ex) = ex {
                    builder.add_arg("EX");
                    builder.add_arg(&ex.to_string());
                }

                if let Some(px) = px {
                    builder.add_arg("PX");
                    builder.add_arg(&px.to_string());
                }

                if let Some(x) = x {
                    match x {
                        ExistOP::NX => builder.add_arg("NX"),
                        ExistOP::XX => builder.add_arg("XX"),
                    }
                }
                builder.to_bytes()
            }
            Commands::Incr { key } => CmdBuilder::new().arg("INCR").arg(key).to_bytes(),
            Commands::Lrange { key, start, stop } => CmdBuilder::new()
                .arg("LRANGE").arg(key).arg(&start.to_string()).arg(&stop.to_string()).to_bytes(),
            Commands::Rpush { key, values } => {
                let mut builder = CmdBuilder::new().arg("RPUSH").arg(key);
                values.iter().for_each(|v| builder.add_arg(v));
                builder.to_bytes()
            }
        };
        log::debug!("{:?}",cmd);
        cmd
    }
}
