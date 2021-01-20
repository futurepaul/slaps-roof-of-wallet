use druid::{AppLauncher, WindowDesc};

mod data;
use data::AppState;

mod view;
use view::build_ui;

mod selectors;

mod delegate;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Slaps Roof Of Wallet")
        .window_size((400.0, 400.0));

    let app = AppLauncher::with_window(main_window);

    let sink = app.get_external_handle();

    let initial_state = AppState::new(sink);

    let delegate = delegate::Delegate { }; 
    
    app.delegate(delegate).launch(initial_state)
        .expect("Failed to launch application");
}
