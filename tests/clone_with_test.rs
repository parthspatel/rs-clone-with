use clone_with::CloneWith;

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use super::*;

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

    #[test]
    pub fn test_clone_u32() {
        let original = MyStruct::default();
        let updated = original.with_property1(42);

        assert_eq!(original.property1, 0);
        assert_eq!(updated.property1, 42);
    }

    #[test]
    pub fn test_clone_string() {
        let original = MyStruct::default();
        let updated = original.with_property2("foo".to_string());

        assert_eq!(original.property2, "".to_string());
        assert_eq!(updated.property2, "foo".to_string());
    }

    #[test]
    pub fn test_clone_bool() {
        let original = MyStruct::default();
        let updated = original
            .with_property3(true);

        assert_eq!(original.property3, false);
        assert_eq!(updated.property3, true);
    }

    #[test]
    pub fn test_clone_all_changes() {
        let original = MyStruct::default();
        let updated = original
            .with_property1(42)
            .with_property2("foo".to_string())
            .with_property3(true)
            ;

        assert_eq!(original.property1, 0);
        assert_eq!(original.property2, "".to_string());

        assert_eq!(updated.property1, 42);
        assert_eq!(updated.property2, "foo".to_string());
    }
}