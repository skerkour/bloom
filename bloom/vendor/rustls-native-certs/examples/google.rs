use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write, stdout};

use rustls;
use rustls_native_certs;
use webpki;

use rustls::Session;

fn main() {
    let mut config = rustls::ClientConfig::new();
    config.root_store = rustls_native_certs::load_native_certs()
        .expect("could not load platform certs");

    let dns_name = webpki::DNSNameRef::try_from_ascii_str("google.com")
        .unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::connect("google.com:443")
        .expect("cannot connect");
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);
    tls.write(concat!("GET / HTTP/1.1\r\n",
                      "Host: google.com\r\n",
                      "Connection: close\r\n",
                      "Accept-Encoding: identity\r\n",
                      "\r\n")
              .as_bytes())
        .expect("write failed");
    let ciphersuite = tls.sess.get_negotiated_ciphersuite()
        .expect("tls handshake failed");
    writeln!(&mut std::io::stderr(), "Current ciphersuite: {:?}", ciphersuite.suite)
        .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext)
        .unwrap();
    stdout().write_all(&plaintext)
        .unwrap();
}
