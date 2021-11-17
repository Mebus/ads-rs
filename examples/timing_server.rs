use std::net::*;
use std::io::*;

fn main() {
    let srv = TcpListener::bind("127.0.0.1:48999").unwrap();
    let resp = b"\x00\x00(\x00\x00\x00\x7f\x00\x00\x01\x01\x01\x00\
                 \x00\x01\x02\x03\x04\x05\x06S\x03\x02\x00\x05\x00\
                 \x0c\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\
                 \x00\x00\x00\x00\x04\x00\x00\x00\xa4\xe0\xfbD";
    loop {
        if let Ok((mut clt, _)) = srv.accept() {
            clt.set_nodelay(true).unwrap();
            std::thread::spawn(move || {
                let mut resp = resp.to_vec();
                let mut buf = [0; 50];
                loop {
                    clt.read_exact(&mut buf).unwrap();
                    resp[34..38].copy_from_slice(&buf[34..38]);
                    clt.write_all(&resp).unwrap();
                }
            });
        }
    }
}