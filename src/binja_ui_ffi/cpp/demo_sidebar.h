#pragma once

#include "sidebar.h"

class TestSidebarType : public SidebarWidgetType {
public:
    TestSidebarType(const QImage& icon, const QString& name) : SidebarWidgetType(icon, name){}

    // BinaryViewRef cannot be mapped to rust by value.
    // Therefore we wrap it in C++ and only override the wrapper function.
    // The error in rust when overriding this function:
    // `error[cxx]: passing opaque C++ type by value is not supported`
    // I don't want to manually map that type (for now)
    SidebarWidget* createWidget(ViewFrame* frame, BinaryViewRef data) override;
    virtual SidebarWidget* createWidgetSub(ViewFrame* frame, BinaryNinja::BinaryView* data) = 0;
};

// static functions in classes are not directly linkable, cause they use a mangled name.
// cxx does not support those static functions, therefor we define a wrapper without mangled name.
// see https://github.com/dtolnay/cxx/issues/447
extern "C" void Sidebar_AddSidebarWidgetType(SidebarWidgetType* type);
