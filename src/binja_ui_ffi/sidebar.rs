//! We use some experimental features of cxx-qt, so we currently use the custom
//! branch: https://github.com/binja-tools/cxx-qt/tree/custom
//! It has implemented two additional PRs to the main branch:
//! https://github.com/KDAB/cxx-qt/pull/667 - This allows the usage of `cxx_name` on functions, that are marked `cxx_override`
//! https://github.com/KDAB/cxx-qt/pull/451 - Allow class inheritance without them being a `qobject`. Needed for custom binaryninja class overrides (e.g. SidebarType).

extern "C" {
    // link to C/C++ function wrappers in sidebar_creator.h
    #[link_name = "CreateMySidebarType"]
    pub fn create_my_sidebar_type() -> *mut MySidebarType;
    #[link_name = "CreateMySidebar"]
    fn create_my_sidebar() -> *mut MySidebar;
    #[link_name = "Sidebar_AddSidebarWidgetType"]
    pub fn add_sidebar_widget_type(t: *mut MySidebarType);
}

// This module contains the stuff cxx-qt/cxx uses to create bindings.
// If you use RustRover you will see a lot of errors in this module.
// This is a bug in RustRover, the errors don't actually exist.
#[cxx_qt::bridge]
mod ffi_sidebar {
    // cxx-qt mapped file includes
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("cxx-qt-lib/qimage.h");
    }

    // binja includes
    unsafe extern "C++" {
        include!("binaryninjaapi.h");
        include!("uitypes.h");
        include!("sidebar.h");
    }

    // demo includes
    unsafe extern "C++" {
        include!("demo_sidebar.h");
    }

    // cxx-qt typedefs
    unsafe extern "C++" {
        type QString = cxx_qt_lib::QString;
        type QImage = cxx_qt_lib::QImage;
    }

    // binja typedefs
    unsafe extern "C++" {
        type SidebarWidget;
        type ViewFrame;
        type BinaryViewRef;
    }

    #[namespace = "BinaryNinja"]
    unsafe extern "C++" {
        type BinaryView;
    }

    // demo typedefs
    // Create subclasses for the types we want to register.
    // These are compiled down to actual C++ classes that are later used in
    // `sidebar_creator.h` to "new" them.
    unsafe extern "RustQt" {
        #[qobject]
        #[base = "SidebarWidget"]
        type MySidebar = super::MySidebarRust;

        #[base = "TestSidebarType"]
        type MySidebarType = super::MySidebarTypeRust;
    }

    // overrides
    // This is the equivalent to defining a (virtual) function `override` in the C++
    // subclass.
    // The "virtual" properties of C++ work too, so these functions
    // are correctly called from within Qt/C++/Binja.
    // Actual implementations of these override functions are further down
    // (outside of this module).
    unsafe extern "RustQt" {
        #[cxx_override]
        unsafe fn closing(self: Pin<&mut MySidebar>);

        #[cxx_override]
        #[cxx_name = "createWidgetSub"]
        unsafe fn create_widget(
            self: Pin<&mut MySidebarType>,
            view_frame: *mut ViewFrame,
            binary_view: *mut BinaryView,
        ) -> *mut SidebarWidget;
    }

    // impls
    // define the constructors of the C++ classes.
    // This is here for cxx-qt to know about them, it is not proper rust syntax.
    // The actual implementations are further down below (outside of this module).
    // Documentation of this: https://docs.rs/cxx-qt/latest/cxx_qt/trait.Constructor.html
    impl cxx_qt::Constructor<(), BaseArguments = (QString,)> for MySidebar {}
    impl cxx_qt::Constructor<(), BaseArguments = (QImage, QString)> for MySidebarType {}
}

use std::pin::Pin;

use cxx_qt::CxxQtType;
use ffi_sidebar::*;
use log::info;

pub struct MySidebarRust;
pub struct MySidebarTypeRust;

impl MySidebar {
    // implement the virtual function override defined in the bridge module at the
    // top.
    unsafe fn closing(self: Pin<&mut Self>) {
        info!("deleting my sidebar!");
    }

    pub fn new_ptr() -> *mut Self {
        unsafe { create_my_sidebar() }
    }
}

impl MySidebarType {
    // implement the virtual function override defined in the bridge module at the
    // top.
    unsafe fn create_widget(
        self: Pin<&mut Self>,
        _view_frame: *mut ViewFrame,
        _binary_view: *mut BinaryView,
    ) -> *mut SidebarWidget {
        MySidebar::new_ptr() as *mut SidebarWidget
    }
}

// implement the Constructor defined in the bridge module at the top.
// This is a constructor that currently has hardcoded base-class arguments.
impl cxx_qt::Constructor<()> for MySidebar {
    type BaseArguments = (QString,);
    type InitializeArguments = ();
    type NewArguments = ();

    fn route_arguments(
        _arguments: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        let t = QString::from("test");
        ((), (t,), ())
    }

    fn new(_arguments: Self::NewArguments) -> <Self as CxxQtType>::Rust {
        MySidebarRust {}
    }
}

// implement the Constructor defined in the bridge module at the top.
// This is a constructor that currently has hardcoded base-class arguments.
impl cxx_qt::Constructor<()> for MySidebarType {
    type BaseArguments = (QImage, QString);
    type InitializeArguments = ();
    type NewArguments = ();

    fn route_arguments(
        _arguments: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        let data = include_bytes!("../../icon.png");
        let image = QImage::from_data(data, None);

        let string = QString::from("test_str");
        ((), (image.unwrap(), string), ())
    }

    fn new(_arguments: Self::NewArguments) -> <Self as CxxQtType>::Rust {
        MySidebarTypeRust {}
    }
}
