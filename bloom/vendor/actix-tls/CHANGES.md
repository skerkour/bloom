# Changes

## Unreleased - 2020-xx-xx


## 2.0.0 - 2020-09-03
* `nativetls::NativeTlsAcceptor` is renamed to `nativetls::Acceptor`.
* Where possible, "SSL" terminology is replaced with "TLS".
    * `SslError` is renamed to `TlsError`.
    * `TlsError::Ssl` enum variant is renamed to `TlsError::Tls`.
    * `max_concurrent_ssl_connect` is renamed to `max_concurrent_tls_connect`.


## 2.0.0-alpha.2 - 2020-08-17
* Update `rustls` dependency to 0.18
* Update `tokio-rustls` dependency to 0.14
* Update `webpki-roots` dependency to 0.20


## [2.0.0-alpha.1] - 2020-03-03
* Update `rustls` dependency to 0.17
* Update `tokio-rustls` dependency to 0.13
* Update `webpki-roots` dependency to 0.19


## [1.0.0] - 2019-12-11
* 1.0.0 release


## [1.0.0-alpha.3] - 2019-12-07
* Migrate to tokio 0.2
* Enable rustls acceptor service
* Enable native-tls acceptor service


## [1.0.0-alpha.1] - 2019-12-02
* Split openssl acceptor from actix-server package
