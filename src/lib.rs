mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            // Relative path
            super::hosting::add_to_waitlist();

            // Absolute path
            crate::front_of_house::hosting::add_to_waitlist();
        }

        fn serve_order() {}

        fn take_payment() {}
    }
}

