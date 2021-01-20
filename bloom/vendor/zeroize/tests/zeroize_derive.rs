//! Integration tests for `zeroize_derive` proc macros

#[cfg(feature = "zeroize_derive")]
mod custom_derive_tests {
    use zeroize::Zeroize;

    #[derive(Zeroize)]
    #[zeroize(drop)]
    struct ZeroizableTupleStruct([u8; 3]);

    #[test]
    fn derive_tuple_struct_test() {
        let mut value = ZeroizableTupleStruct([1, 2, 3]);
        value.zeroize();
        assert_eq!(&value.0, &[0, 0, 0])
    }

    #[derive(Zeroize)]
    #[zeroize(drop)]
    struct ZeroizableStruct {
        string: String,
        vec: Vec<u8>,
        bytearray: [u8; 3],
        number: usize,
        boolean: bool,
    }

    #[test]
    fn derive_struct_test() {
        let mut value = ZeroizableStruct {
            string: String::from("Hello, world!"),
            vec: vec![1, 2, 3],
            bytearray: [4, 5, 6],
            number: 42,
            boolean: true,
        };

        value.zeroize();

        assert!(value.string.is_empty());
        assert!(value.vec.is_empty());
        assert_eq!(&value.bytearray, &[0, 0, 0]);
        assert_eq!(value.number, 0);
        assert!(!value.boolean);
    }

    /// Test that the custom macro actually derived `Drop` for `ZeroizableStruct`
    trait Droppable: Drop {}

    impl Droppable for ZeroizableStruct {}

    /// Test that `Drop` is not derived in the following case by defining a
    /// `Drop` impl which should conflict if the custom derive defined one too
    #[allow(dead_code)]
    #[derive(Zeroize)]
    struct ZeroizeNoDropStruct([u8; 3]);

    impl Drop for ZeroizeNoDropStruct {
        fn drop(&mut self) {}
    }
}
