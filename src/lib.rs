use bincode::serialize;
use reqwest::blocking::Client;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HTTPError {
    #[error("Invalid read request")]
    InvalidRead,
    #[error("Invalid write request")]
    InvalidWrite,
}

pub struct HTTPClient {
    client: Client,
    base_url: String,
}

impl HTTPClient {
    pub fn new(ip: &str, port: u16) -> HTTPClient {
        HTTPClient {
            client: Client::new(),
            base_url: format!("{ip}:{port}"),
        }
    }
    pub fn read_host<T: Sized + Copy>(
        &mut self,
        pid: i32,
        address: usize,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let size = std::mem::size_of::<T>();
        let url = format!("{}/host/{pid}/{address}/{size}", self.base_url);
        let response = self.client.get(&url).send()?;

        if response.status().is_success() {
            let bytes = response.bytes()?;

            let result: T = unsafe { *bytes.as_ptr().cast() };
            Ok(result)
        } else {
            Err(HTTPError::InvalidRead.into())
        }
    }

    pub fn write_host<T: Sized + Serialize>(
        &mut self,
        pid: i32,
        address: usize,
        value: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let size = std::mem::size_of::<T>();
        let url = format!("{}/host/{pid}/{address}/{size}", self.base_url);
        let bytes = serialize(&value)?;
        let response = self.client.get(&url).body(bytes).send()?;

        match response.status().is_success() {
            false => Err(HTTPError::InvalidWrite.into()),
            _ => Ok(()),
        }
    }
}
