//! `crypto_secretstream_xchacha20poly1305`
use libsodium_sys::{
    crypto_secretstream_xchacha20poly1305_ABYTES,
    crypto_secretstream_xchacha20poly1305_HEADERBYTES,
    crypto_secretstream_xchacha20poly1305_KEYBYTES,
    crypto_secretstream_xchacha20poly1305_TAG_FINAL,
    crypto_secretstream_xchacha20poly1305_TAG_MESSAGE,
    crypto_secretstream_xchacha20poly1305_TAG_PUSH,
    crypto_secretstream_xchacha20poly1305_TAG_REKEY,
    crypto_secretstream_xchacha20poly1305_init_pull,
    crypto_secretstream_xchacha20poly1305_init_push,
    crypto_secretstream_xchacha20poly1305_messagebytes_max,
    crypto_secretstream_xchacha20poly1305_pull, crypto_secretstream_xchacha20poly1305_push,
    crypto_secretstream_xchacha20poly1305_rekey, crypto_secretstream_xchacha20poly1305_state,
};

stream_module!(
    crypto_secretstream_xchacha20poly1305_state,
    crypto_secretstream_xchacha20poly1305_init_push,
    crypto_secretstream_xchacha20poly1305_push,
    crypto_secretstream_xchacha20poly1305_init_pull,
    crypto_secretstream_xchacha20poly1305_pull,
    crypto_secretstream_xchacha20poly1305_rekey,
    crypto_secretstream_xchacha20poly1305_messagebytes_max,
    crypto_secretstream_xchacha20poly1305_KEYBYTES,
    crypto_secretstream_xchacha20poly1305_HEADERBYTES,
    crypto_secretstream_xchacha20poly1305_ABYTES,
    crypto_secretstream_xchacha20poly1305_TAG_MESSAGE,
    crypto_secretstream_xchacha20poly1305_TAG_PUSH,
    crypto_secretstream_xchacha20poly1305_TAG_REKEY,
    crypto_secretstream_xchacha20poly1305_TAG_FINAL
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::rand;

    // NOTE: it is impossible to allocate enough memory for `msg` below without
    // overflowing the stack. Further, from all the research I've done and what
    // I know it seems impossible with Rust's type model to mock a call to `len`
    // and none of the mocking libraries seem to privde a workaround. Therefore
    // we cannot test en/decrypting plain/ciphertexts that exceed the ~275GB
    // maximum.

    #[test]
    fn decrypt_too_short_ciphertext() {
        let ciphertext = [0; (ABYTES - 1)];
        let key = gen_key();
        let (_, header) = Stream::init_encrypt(&key).unwrap();
        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        // TODO: when custom error types are introduced, this should assert the
        // specific error.
        assert!(stream.decrypt(&ciphertext, None).is_err());
    }

    #[test]
    fn encrypt_decrypt() {
        let mut msg1 = [0; 128];
        let mut msg2 = [0; 34];
        let mut msg3 = [0; 478];

        rand::bytes_into(&mut msg1);
        rand::bytes_into(&mut msg2);
        rand::bytes_into(&mut msg3);

        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c1 = stream.encrypt(&msg1, None, Tag::Message).unwrap();
        assert!(stream.is_not_finalized());
        let c2 = stream.encrypt(&msg2, None, Tag::Push).unwrap();
        assert!(stream.is_not_finalized());
        let c3 = stream.encrypt(&msg3, None, Tag::Final).unwrap();
        assert!(stream.is_finalized());

        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        assert!(stream.is_not_finalized());

        let (m1, t1) = stream.decrypt(&c1, None).unwrap();
        assert_eq!(t1, Tag::Message);
        assert_eq!(msg1[..], m1[..]);

        let (m2, t2) = stream.decrypt(&c2, None).unwrap();
        assert_eq!(t2, Tag::Push);
        assert_eq!(msg2[..], m2[..]);

        let (m3, t3) = stream.decrypt(&c3, None).unwrap();
        assert_eq!(t3, Tag::Final);
        assert_eq!(msg3[..], m3[..]);
    }

    #[test]
    fn encrypt_decrypt_with_ad() {
        let mut msg1 = [0; 128];
        let mut msg2 = [0; 34];
        let mut msg3 = [0; 478];
        let mut ad1 = [0; 224];
        let mut ad2 = [0; 135];

        rand::bytes_into(&mut msg1);
        rand::bytes_into(&mut msg2);
        rand::bytes_into(&mut msg3);
        rand::bytes_into(&mut ad1);
        rand::bytes_into(&mut ad2);

        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c1 = stream.encrypt(&msg1, Some(&ad1), Tag::Message).unwrap();
        let c2 = stream.encrypt(&msg2, Some(&ad2), Tag::Push).unwrap();
        let c3 = stream.encrypt(&msg3, None, Tag::Final).unwrap();

        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        assert!(stream.is_not_finalized());

        let (m1, t1) = stream.decrypt(&c1, Some(&ad1)).unwrap();
        assert_eq!(t1, Tag::Message);
        assert_eq!(msg1[..], m1[..]);
        assert!(stream.is_not_finalized());

        let (m2, t2) = stream.decrypt(&c2, Some(&ad2)).unwrap();
        assert_eq!(t2, Tag::Push);
        assert_eq!(msg2[..], m2[..]);
        assert!(stream.is_not_finalized());

        let (m3, t3) = stream.decrypt(&c3, None).unwrap();
        assert_eq!(t3, Tag::Final);
        assert_eq!(msg3[..], m3[..]);
        assert!(stream.is_finalized());
    }

    #[test]
    fn encrypt_decrypt_with_rekey() {
        let mut msg1 = [0; 128];
        let mut msg2 = [0; 34];
        let mut msg3 = [0; 478];

        rand::bytes_into(&mut msg1);
        rand::bytes_into(&mut msg2);
        rand::bytes_into(&mut msg3);

        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c1 = stream.encrypt(&msg1, None, Tag::Message).unwrap();
        let c2 = stream.encrypt(&msg2, None, Tag::Rekey).unwrap();
        let c3 = stream.encrypt(&msg3, None, Tag::Final).unwrap();

        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        assert!(stream.is_not_finalized());

        let (m1, t1) = stream.decrypt(&c1, None).unwrap();
        assert_eq!(t1, Tag::Message);
        assert_eq!(msg1[..], m1[..]);
        assert!(stream.is_not_finalized());

        let (m2, t2) = stream.decrypt(&c2, None).unwrap();
        assert_eq!(t2, Tag::Rekey);
        assert_eq!(msg2[..], m2[..]);
        assert!(stream.is_not_finalized());

        let (m3, t3) = stream.decrypt(&c3, None).unwrap();
        assert_eq!(t3, Tag::Final);
        assert_eq!(msg3[..], m3[..]);
        assert!(stream.is_finalized());
    }

    #[test]
    fn encrypt_decrypt_with_explicit_rekey() {
        let mut msg1 = [0; 128];
        let mut msg2 = [0; 34];
        let mut msg3 = [0; 478];

        rand::bytes_into(&mut msg1);
        rand::bytes_into(&mut msg2);
        rand::bytes_into(&mut msg3);

        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c1 = stream.encrypt(&msg1, None, Tag::Message).unwrap();
        let c2 = stream.encrypt(&msg2, None, Tag::Push).unwrap();
        stream.rekey().unwrap();
        let c3 = stream.encrypt(&msg3, None, Tag::Final).unwrap();

        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        assert!(stream.is_not_finalized());

        let (m1, t1) = stream.decrypt(&c1, None).unwrap();
        assert_eq!(t1, Tag::Message);
        assert_eq!(msg1[..], m1[..]);
        assert!(stream.is_not_finalized());

        let (m2, t2) = stream.decrypt(&c2, None).unwrap();
        assert_eq!(t2, Tag::Push);
        assert_eq!(msg2[..], m2[..]);
        assert!(stream.is_not_finalized());

        stream.rekey().unwrap();
        assert!(stream.is_not_finalized());

        let (m3, t3) = stream.decrypt(&c3, None).unwrap();
        assert_eq!(t3, Tag::Final);
        assert_eq!(msg3[..], m3[..]);
        assert!(stream.is_finalized());
    }

    #[test]
    fn encrypt_decrypt_with_recycled_vec() {
        let mut msg1 = [0; 128];
        let mut msg2 = [0; 34];
        let mut msg3 = [0; 478];

        rand::bytes_into(&mut msg1);
        rand::bytes_into(&mut msg2);
        rand::bytes_into(&mut msg3);

        let key = gen_key();
        let (mut encrypt_stream, header) = Stream::init_encrypt(&key).unwrap();
        let mut decrypt_stream = Stream::init_decrypt(&header, &key).unwrap();

        let mut c_buf = Vec::new();
        let mut m_buf = Vec::new();
        encrypt_stream
            .encrypt_to_vec(&msg1, None, Tag::Message, &mut c_buf)
            .unwrap();
        let t1 = decrypt_stream
            .decrypt_to_vec(&c_buf, None, &mut m_buf)
            .unwrap();
        assert_eq!(t1, Tag::Message);
        assert_eq!(msg1[..], m_buf[..]);
        assert!(encrypt_stream.is_not_finalized());
        assert!(decrypt_stream.is_not_finalized());

        encrypt_stream
            .encrypt_to_vec(&msg2, None, Tag::Message, &mut c_buf)
            .unwrap();
        let t2 = decrypt_stream
            .decrypt_to_vec(&c_buf, None, &mut m_buf)
            .unwrap();
        assert_eq!(t2, Tag::Message);
        assert_eq!(msg2[..], m_buf[..]);
        assert!(encrypt_stream.is_not_finalized());
        assert!(decrypt_stream.is_not_finalized());

        encrypt_stream
            .encrypt_to_vec(&msg3, None, Tag::Final, &mut c_buf)
            .unwrap();
        let t3 = decrypt_stream
            .decrypt_to_vec(&c_buf, None, &mut m_buf)
            .unwrap();
        assert_eq!(t3, Tag::Final);
        assert_eq!(msg3[..], m_buf[..]);
        assert!(encrypt_stream.is_finalized());
        assert!(decrypt_stream.is_finalized());
    }

    #[test]
    fn cannot_decrypt_after_finalization() {
        let m = [0; 128];
        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c = stream.encrypt(&m, None, Tag::Final).unwrap();
        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        assert!(stream.is_not_finalized());
        stream.decrypt(&c, None).unwrap();
        // TODO: check specific `Err(())` when implemented (#221).
        assert!(stream.decrypt(&c, None).is_err());
    }

    #[test]
    fn cannot_encrypt_after_finalization() {
        let m = [0; 128];
        let key = gen_key();
        let (mut stream, _) = Stream::init_encrypt(&key).unwrap();
        stream.encrypt(&m, None, Tag::Final).unwrap();
        // TODO: check specific `Err(())` when implemented (#221)
        assert!(stream.encrypt(&m, None, Tag::Message).is_err());
    }

    #[test]
    fn cannot_rekey_after_finalization() {
        let m = [0; 128];
        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c = stream.encrypt(&m, None, Tag::Final).unwrap();
        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        assert!(stream.is_not_finalized());
        stream.decrypt(&c, None).unwrap();
        // TODO: check specific `Err(())` when implemented (#221).
        assert!(stream.rekey().is_err());
    }

    #[test]
    fn finalize_with_ad() {
        let mut m = [0; 128];
        let mut ad = [0; 64];
        rand::bytes_into(&mut m);
        rand::bytes_into(&mut ad);
        let key = gen_key();
        let (mut stream, header) = Stream::init_encrypt(&key).unwrap();
        let c1 = stream.encrypt(&m, None, Tag::Message).unwrap();
        let c2 = stream.finalize(Some(&ad)).unwrap();

        let mut stream = Stream::init_decrypt(&header, &key).unwrap();
        let (m1, t1) = stream.decrypt(&c1, None).unwrap();
        assert_eq!(m[..], m1[..]);
        assert_eq!(t1, Tag::Message);

        let (m2, t2) = stream.decrypt(&c2, Some(&ad)).unwrap();
        assert_eq!(m2[..], [0; 0]);
        assert_eq!(t2, Tag::Final);
    }

    #[test]
    fn tag_from_u8() {
        assert_eq!(Tag::Message, Tag::from_u8(0).unwrap());
        assert_eq!(Tag::Push, Tag::from_u8(1).unwrap());
        assert_eq!(Tag::Rekey, Tag::from_u8(2).unwrap());
        assert_eq!(Tag::Final, Tag::from_u8(3).unwrap());
        for i in 4..=u16::from(core::u8::MAX) {
            assert!(Tag::from_u8(i as u8).is_err());
        }
    }
}
