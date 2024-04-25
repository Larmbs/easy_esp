use std::net::{TcpStream, SocketAddr};
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use std::io::{BufReader, Read, Result};



pub struct TCPConn<T> where T: Serialize + for<'a> Deserialize<'a> {
    addr: SocketAddr,
    conn: TcpStream,
    _phantom: PhantomData<T>,
}

impl<T> TCPConn<T> where T: Serialize + for<'a> Deserialize<'a> {
    /// Opens a new device connection
    pub fn open(stream: TcpStream) -> Result<TCPConn<T>> {
        let addr = stream.peer_addr()?;
        let conn = stream;
        Ok(TCPConn {
            addr,
            conn,
            _phantom: PhantomData,
        })
    }
    //fn read_stream(&mut self)
    /// Begins server listening
    pub fn listen(&mut self) {
        // Loop until a exit request or timeout
        loop {
            let buf_reader = BufReader::new(&mut self.conn);


            let data: T = serde_json::from_reader(buf_reader).unwrap();

        }
    }

}
