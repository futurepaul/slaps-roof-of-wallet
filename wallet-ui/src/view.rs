use std::sync::Arc;

use druid::{
    im::Vector,
    widget::{Button, Flex, Label, List, TextBox, ViewSwitcher},
    ArcStr, Widget, WidgetExt,
};
use wallet_core::SlapsDevice;

use crate::data::*;

fn single_device() -> impl Widget<UIDevice> {
    let model = Label::new(UIDevice::display_model);
    let fingerprint = Label::new(UIDevice::display_fingerprint);
    let path = Label::new(UIDevice::display_path);
    let print_xpub_button = Button::new("Print Xpub").on_click(UIDevice::print_xpub);
    let create_wallet_from_device = Button::new("Create wallet").on_click(UIDevice::create_wallet);
    Flex::column()
        .with_child(model)
        .with_child(fingerprint)
        .with_child(path)
        .with_child(print_xpub_button)
        .with_child(create_wallet_from_device)
}

fn devices() -> impl Widget<Vector<UIDevice>> {
    let header = Label::new("ANOTHER LIST OF DEVICES");

    let devices_list = List::new(single_device);

    Flex::column().with_child(header).with_child(devices_list)
}

fn setup() -> impl Widget<AppState> {
    let header = Label::new("Setup").with_text_size(28.);
    let refresh_devices_button = Button::new("Refresh devices").on_click(AppState::refresh_devices);
    let device_list = devices().lens(AppState::ui_device_list);

    Flex::column()
        .with_child(header)
        .with_child(refresh_devices_button)
        .with_child(device_list)
}

fn transactions() -> impl Widget<AppState> {
    let header = Label::new("Transactions").with_text_size(28.);
    let refresh_balance_button = Button::new("Refresh balance").on_click(AppState::get_balance);
    let balance_display = Label::raw().lens(AppState::balance);
    let print_descriptors = Button::new("Print descriptors").on_click(AppState::print_descriptors);

    let send_button = Button::new("Send").on_click(AppState::go_to_send_route);
    let receive_button = Button::new("Receive").on_click(AppState::go_to_receive_route);

    Flex::column()
        .with_child(header)
        .with_child(balance_display)
        .with_child(refresh_balance_button)
        .with_child(print_descriptors)
        .with_child(send_button)
        .with_child(receive_button)
}

fn send() -> impl Widget<AppState> {
    let header = Label::new("Send").with_text_size(28.);

    let to_address = TextBox::new().lens(AppState::send_to_address);

    let paste_send_button =
        Button::new("Paste send address").on_click(AppState::paste_send_address);

    let create_tx_button = Button::new("Create transaction").on_click(AppState::create_tx);

    let back_button = Button::new("Back").on_click(AppState::go_to_transactions_route);

    Flex::column()
        .with_child(header)
        .with_child(to_address)
        .with_child(paste_send_button)
        .with_child(create_tx_button)
        .with_child(back_button)
}

fn receive() -> impl Widget<AppState> {
    let header = Label::new("Receive").with_text_size(28.);

    let address_button = Button::new("Get new address").on_click(AppState::get_address);

    let address_display = Label::raw().lens(AppState::address);

    let copy_button = Button::new("Copy address").on_click(AppState::copy_address);

    let back_button = Button::new("Back").on_click(AppState::go_to_transactions_route);

    Flex::column()
        .with_child(header)
        .with_child(address_button)
        .with_child(address_display)
        .with_child(copy_button)
        .with_child(back_button)
}

pub fn build_ui() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.active_route,
        |selector, _data, _env| match selector {
            Route::Setup => setup().boxed(),
            Route::Transactions => transactions().boxed(),
            Route::Send => send().boxed(),
            Route::Receive => receive().boxed(),
        },
    )
}
