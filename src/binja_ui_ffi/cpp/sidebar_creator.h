#pragma once

#include "ffi_sidebar.cxxqt.h"

// Unfortunately there is no way of creating unmanaged/non-smart pointers for cxx types yet.
// Therefore we write a wrapper to create those.
// This can be generalised to use templates instead, which will be added to cxx-qt in the future
// https://github.com/KDAB/cxx-qt/issues/823

extern "C" MySidebarType* CreateMySidebarType();
extern "C" MySidebar* CreateMySidebar();
