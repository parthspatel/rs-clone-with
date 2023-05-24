mod sample_usage {
    use clone_with::CloneWith;

    #[derive(Debug, Clone, CloneWith)]
    struct MyStruct {
        id: u32,
        name: String,
        has_a_pet: bool,
    }

    impl MyStruct {
        pub fn print_greeting(&self) {
            println!("Hi, I'm {}!", self.name);
        }
    }

    #[test]
    pub fn main() {
        let original = MyStruct {
            id: 0,
            name: "".to_string(),
            has_a_pet: false,
        };

        let updated = original
            .with_id(42)
            .with_name("John Smith".to_string());

        assert_eq!(original.id, 0);
        assert_eq!(original.name, "".to_string());
        assert_eq!(original.has_a_pet, false);

        assert_eq!(updated.id, 42);
        assert_eq!(updated.name, "John Smith".to_string());
        assert_eq!(updated.has_a_pet, false);

        updated.print_greeting();

        println!("Original: {:?}", original);
        println!("Updated: {:?}", updated);

        // Original: MyStruct { id: 0, name: "", has_a_pet: false }
        // Updated: MyStruct { id: 42, name: "John Smith", has_a_pet: false }
    }
}