use binaryninja::logger;
use log::LevelFilter;

use crate::binja_ui_ffi::sidebar::{add_sidebar_widget_type, create_my_sidebar_type};

mod binja_ui_ffi;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn CorePluginInit() -> bool {
    logger::init(LevelFilter::Info).expect("failed to initialize logging");

    unsafe {
        // create and register the custom SidebarType.
        // Calls the C++ functions defined in `sidebar_creator.h`
        let t = create_my_sidebar_type();
        add_sidebar_widget_type(t);
    }

    true
}
