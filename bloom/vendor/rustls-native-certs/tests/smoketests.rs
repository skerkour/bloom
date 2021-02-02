use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write};

use rustls;
use webpki;
use rustls_native_certs;

fn check_site(domain: &str) {
    let mut config = rustls::ClientConfig::new();
    config.root_store = rustls_native_certs::load_native_certs()
        .unwrap();

    let dns_name = webpki::DNSNameRef::try_from_ascii_str(domain)
        .unwrap();
    let mut sess = rustls::ClientSession::new(&Arc::new(config), dns_name);
    let mut sock = TcpStream::connect(format!("{}:443", domain)).unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);
    tls.write(format!("GET / HTTP/1.1\r\n\
                       Host: {}\r\n\
                       Connection: close\r\n\
                       Accept-Encoding: identity\r\n\
                       \r\n", domain)
              .as_bytes())
        .unwrap();
    let mut plaintext = [0u8; 1024];
    let len = tls.read(&mut plaintext).unwrap();
    assert!(plaintext[..len].starts_with(b"HTTP/1.1 ")); // or whatever
}

#[test]
fn google() {
    check_site("google.com");
}

#[test]
fn amazon() {
    check_site("amazon.com");
}

#[test]
fn facebook() {
    check_site("facebook.com");
}

#[test]
fn netflix() {
    check_site("netflix.com");
}

#[test]
fn ebay() {
    check_site("ebay.com");
}

#[test]
fn apple() {
    check_site("apple.com");
}
