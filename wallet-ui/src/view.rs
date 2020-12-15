use std::sync::Arc;

use druid::{ArcStr, Widget, WidgetExt, im::Vector, widget::{Button, Flex, Label, List, TextBox}};
use wallet_core::SlapsDevice;

use crate::data::*;


fn single_device() -> impl Widget<UIDevice> {
    let model = Label::new(UIDevice::display_model);
    let fingerprint = Label::new(UIDevice::display_fingerprint);
    let path = Label::new(UIDevice::display_path);
    let print_xpub_button = Button::new("Print Xpub").on_click(UIDevice::print_xpub);
    Flex::column().with_child(model).with_child(fingerprint).with_child(path).with_child(print_xpub_button)
}

fn devices() -> impl Widget<Vector<UIDevice>> {
    let header = Label::new("ANOTHER LIST OF DEVICES");

    let devices_list = List::new(single_device);

    Flex::column().with_child(header).with_child(devices_list)

}

pub fn build_ui() -> impl Widget<AppState> {
    let address_button = Button::new("Get new address").on_click(AppState::get_address);

    let address_display = Label::raw().lens(AppState::address);

    let balance_display = Label::raw().lens(AppState::balance);

    let copy_button = Button::new("Copy address").on_click(AppState::copy_address);

    let to_address = TextBox::new().lens(AppState::send_to_address);

    let paste_send_button = Button::new("Paste send address").on_click(AppState::paste_send_address);

    let create_tx_button = Button::new("Create transaction").on_click(AppState::create_tx);

    let refresh_devices_button = Button::new("Refresh devices").on_click(AppState::refresh_devices);

    let device_list = devices().lens(AppState::ui_device_list);

    Flex::column()
        .with_child(address_button)
        .with_child(address_display)
        .with_child(copy_button)
        .with_child(balance_display)
        .with_child(to_address)
        .with_child(paste_send_button)
        .with_child(create_tx_button)
        .with_child(refresh_devices_button)
        .with_child(device_list)
}
