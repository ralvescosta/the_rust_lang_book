/*
mod front_of_house {
    mod hosting {
        fn add_to_wait_list() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

we have the schema bellow

crate
  |
  |---front_of_house
  |   |---add_to_wait_list
  |   |---seat_at_table
  |
  |---serving
      |---take_order
      |---server_order
      |---take_payment
*/
fn do_something() {}
mod front_of_house {
    fn do_something() {
        super::do_something()
    }
    mod hosting {
        fn add_to_wait_list() {
            seat_at_table()
        }
        fn seat_at_table() {
            super::super::do_something()
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
