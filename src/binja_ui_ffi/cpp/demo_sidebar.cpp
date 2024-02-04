#include "demo_sidebar.h"

SidebarWidget* TestSidebarType::createWidget(ViewFrame* frame, BinaryViewRef data) {
    return createWidgetSub(frame, data.GetPtr());
}

void Sidebar_AddSidebarWidgetType(SidebarWidgetType* type) {
    Sidebar::addSidebarWidgetType(type);
}
