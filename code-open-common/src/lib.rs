use rmp_serde::{self, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{convert::TryInto, mem::size_of};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeOpenInfo {
    pub remote_host_name: String,
    pub remote_dir_full_path: String,
}

impl CodeOpenInfo {
    pub fn new(remote_host_name: String, remote_dir_full_path: String) -> Self {
        Self {
            remote_host_name,
            remote_dir_full_path,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeOpenRequest {
    Open(CodeOpenInfo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeOpenResponse {
    Ok,
    Error(String),
}

#[derive(Debug)]
pub struct SerializedDataContainer {
    size: usize,
    data: Vec<u8>,
}

impl SerializedDataContainer {
    pub fn new(v: &[u8]) -> Self {
        Self {
            size: v.len(),
            data: v.to_owned(),
        }
    }

    pub fn to_one_vec(&self) -> Vec<u8> {
        let mut ret = vec![];

        ret.append(&mut self.size.to_le_bytes().to_vec());
        ret.append(&mut self.data.clone());

        ret
    }

    pub fn from_reader<T>(reader: &mut T) -> Result<Self, std::io::Error>
    where
        T: Read,
    {
        let mut size_buffer = [0; size_of::<usize>()];
        reader.read_exact(&mut size_buffer).and_then(|_| {
            let size = usize::from_le_bytes(size_buffer);
            println!("size : {:?}", size);
            let mut data = vec![];

            reader.take(size as u64).read_to_end(&mut data)?;

            Ok(Self { size, data })
        })
    }

    pub fn from_one_vec(v: Vec<u8>) -> Option<Self> {
        if v.len() >= size_of::<usize>() {
            let size = usize::from_le_bytes(
                v[0..size_of::<usize>()]
                    .try_into()
                    .expect("Failed to parse size of the data container"),
            );
            let data = v[size_of::<usize>()..]
                .try_into()
                .expect("Failed to get data of the data container");

            Some(Self { size, data })
        } else {
            None
        }
    }

    pub fn from_serializable_data<T>(t: &T) -> Option<Self>
    where
        T: Serialize,
    {
        let mut data = vec![];
        t.serialize(&mut Serializer::new(&mut data)).ok().map(|_| {
            let size = data.len();
            Self { size, data }
        })
    }

    pub fn to_serializable_data<T: for<'de> Deserialize<'de>>(&self) -> Option<T> {
        rmp_serde::from_slice(&self.data).ok()
    }
}

#[derive(Debug)]
pub struct CodeOpenConfig {
    pub ip: String,
    pub port: u16,
}

pub static DEFAULT_IP: &str = "localhost";
pub static DEFAULT_PORT: u16 = 3000;

impl Default for CodeOpenConfig {
    fn default() -> Self {
        Self {
            ip: DEFAULT_IP.to_owned(),
            port: DEFAULT_PORT,
        }
    }
}

impl CodeOpenConfig {
    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}
