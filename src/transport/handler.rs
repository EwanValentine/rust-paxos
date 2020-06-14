pub mod handler {
    trait Callback {
        fn callback(&self);
    }

    impl <T: Fn()> Callback for T {
        fn callback(&self) {
            self()
        }
    }
}