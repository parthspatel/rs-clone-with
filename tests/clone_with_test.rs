#[cfg(test)]
mod tests {
    use clone_with::CloneWith;
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    #[derive(Debug, Clone, CloneWith)]
    struct MyStruct {
        property1: u32,
        property2: String,
        property3: bool,
    }

    impl Default for MyStruct {
        fn default() -> Self {
            MyStruct {
                property1: 0,
                property2: "".to_string(),
                property3: false,
            }
        }
    }

    // None of the tests are allowed to mutate the ORIGINAL value, so we can use a static.
    // However we use a Mutex here to show that no mutation is happening.
    lazy_static! {
        static ref ORIGINAL: Mutex<MyStruct> = Mutex::new(MyStruct::default());
    }

    #[test]
    pub fn test_clone_u32() {
        let res = ORIGINAL.lock().map(|original| {
            let updated = original.with_property1(42);

            assert_eq!(original.property1, 0);
            assert_eq!(updated.property1, 42);
        });
        assert!(res.is_ok());
    }

    #[test]
    pub fn test_clone_string() {
        let res = ORIGINAL.lock().map(|original| {
            let updated = original.with_property2("foo".to_string());

            assert_eq!(original.property2, "".to_string());
            assert_eq!(updated.property2, "foo".to_string());
        });
        assert!(res.is_ok());
    }

    #[test]
    pub fn test_clone_bool() {
        let res = ORIGINAL.lock().map(|original| {
            let updated = original
                .with_property3(true);

            assert_eq!(original.property3, false);
            assert_eq!(updated.property3, true);
        });
        assert!(res.is_ok());
    }

    #[test]
    pub fn test_clone_all_changes() {
        let res = ORIGINAL.lock().map(|original| {
            let updated = original
                .with_property1(42)
                .with_property2("foo".to_string())
                .with_property3(true)
                ;

            assert_eq!(original.property1, 0);
            assert_eq!(original.property2, "".to_string());

            assert_eq!(updated.property1, 42);
            assert_eq!(updated.property2, "foo".to_string());
        });
        assert!(res.is_ok());
    }
}