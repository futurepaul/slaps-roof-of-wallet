use druid::{Widget, widget::{Button, Flex, Label}};

use crate::data::*;

use wallet_core;

pub fn build_ui() -> impl Widget<AppState> {
    let hello = wallet_core::hello_from_wallet_core();
    let test_label = Label::new(hello);

    let address_button = Button::new("Print new address").on_click(AppState::print_address);

    let print_balance = Button::new("Print current balance").on_click(AppState::print_balance);

    Flex::column().with_child(test_label).with_child(address_button).with_child(print_balance)
}
