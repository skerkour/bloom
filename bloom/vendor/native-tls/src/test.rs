use hex;
#[allow(unused_imports)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use super::*;

macro_rules! p {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        }
    };
}

// This nested mod is needed for ios testing with rust-test-ios
mod tests {
    use super::*;

    #[test]
    fn connect_google() {
        let builder = p!(TlsConnector::new());
        let s = p!(TcpStream::connect("google.com:443"));
        let mut socket = p!(builder.connect("google.com", s));

        p!(socket.write_all(b"GET / HTTP/1.0\r\n\r\n"));
        let mut result = vec![];
        p!(socket.read_to_end(&mut result));

        println!("{}", String::from_utf8_lossy(&result));
        assert!(result.starts_with(b"HTTP/1.0"));
        assert!(result.ends_with(b"</HTML>\r\n") || result.ends_with(b"</html>"));
    }

    #[test]
    fn connect_bad_hostname() {
        let builder = p!(TlsConnector::new());
        let s = p!(TcpStream::connect("google.com:443"));
        builder.connect("goggle.com", s).unwrap_err();
    }

    #[test]
    fn connect_bad_hostname_ignored() {
        let builder = p!(TlsConnector::builder()
            .danger_accept_invalid_hostnames(true)
            .build());
        let s = p!(TcpStream::connect("google.com:443"));
        builder.connect("goggle.com", s).unwrap();
    }

    #[test]
    fn connect_no_root_certs() {
        let builder = p!(TlsConnector::builder().disable_built_in_roots(true).build());
        let s = p!(TcpStream::connect("google.com:443"));
        assert!(builder.connect("google.com", s).is_err());
    }

    #[test]
    fn server_no_root_certs() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .disable_built_in_roots(true)
            .add_root_certificate(root_ca)
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    fn server() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    #[cfg(not(target_os = "ios"))]
    fn server_pem() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.pem");
        let root_ca = Certificate::from_pem(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    fn peer_certificate() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let socket = p!(builder.accept(socket));
            assert!(socket.peer_certificate().unwrap().is_none());
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .build());
        let socket = p!(builder.connect("foobar.com", socket));

        let cert_der = include_bytes!("../test/cert.der");
        let cert = socket.peer_certificate().unwrap().unwrap();
        assert_eq!(cert.to_der().unwrap(), &cert_der[..]);

        p!(j.join());
    }

    #[test]
    fn server_tls11_only() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(identity)
            .min_protocol_version(Some(Protocol::Tlsv11))
            .max_protocol_version(Some(Protocol::Tlsv11))
            .build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .min_protocol_version(Some(Protocol::Tlsv11))
            .max_protocol_version(Some(Protocol::Tlsv11))
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    fn server_no_shared_protocol() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(identity)
            .min_protocol_version(Some(Protocol::Tlsv12))
            .build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            assert!(builder.accept(socket).is_err());
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .max_protocol_version(Some(Protocol::Tlsv11))
            .build());
        assert!(builder.connect("foobar.com", socket).is_err());

        p!(j.join());
    }

    #[test]
    fn server_untrusted() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            // FIXME should assert error
            // https://github.com/steffengy/schannel-rs/issues/20
            let _ = builder.accept(socket);
        });

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::new());
        builder.connect("foobar.com", socket).unwrap_err();

        p!(j.join());
    }

    #[test]
    fn server_untrusted_unverified() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    fn import_same_identity_multiple_times() {
        let buf = include_bytes!("../test/identity.p12");
        let _ = p!(Identity::from_pkcs12(buf, "mypass"));
        let _ = p!(Identity::from_pkcs12(buf, "mypass"));
    }

    #[test]
    fn shutdown() {
        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            assert_eq!(p!(socket.read(&mut buf)), 0);
            p!(socket.shutdown());
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        p!(socket.shutdown());

        p!(j.join());
    }

    #[test]
    #[cfg_attr(target_os = "ios", ignore)]
    fn tls_server_end_point() {
        let expected = "4712b939fbcb42a6b5101b42139a25b14f81b418facabd378746f12f85cc6544";

        let buf = include_bytes!("../test/identity.p12");
        let identity = p!(Identity::from_pkcs12(buf, "mypass"));
        let builder = p!(TlsAcceptor::new(identity));

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let binding = socket.tls_server_end_point().unwrap().unwrap();
            assert_eq!(hex::encode(binding), expected);

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder()
            .add_root_certificate(root_ca)
            .build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        let binding = socket.tls_server_end_point().unwrap().unwrap();
        assert_eq!(hex::encode(binding), expected);

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }
}
