

// mod ::widgets::QDockWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qdockwidget.h
// #include <qdockwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 41
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void changeEvent(QEvent *)
// func (this *QDockWidget) InheritChangeEvent(f func(event *qtcore.QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "changeEvent", f)
// }

// void closeEvent(QCloseEvent *)
// func (this *QDockWidget) InheritCloseEvent(f func(event *qtgui.QCloseEvent/*777 QCloseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "closeEvent", f)
// }

// void paintEvent(QPaintEvent *)
// func (this *QDockWidget) InheritPaintEvent(f func(event *qtgui.QPaintEvent/*777 QPaintEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "paintEvent", f)
// }

// bool event(QEvent *)
// func (this *QDockWidget) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void initStyleOption(QStyleOptionDockWidget *)
// func (this *QDockWidget) InheritInitStyleOption(f func(option *QStyleOptionDockWidget/*777 QStyleOptionDockWidget **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "initStyleOption", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QDockWidget)=48
pub struct QDockWidget {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDockWidget_ITF interface {
//    QWidget_ITF
//    QDockWidget_PTR() *QDockWidget
//}
//func (ptr *QDockWidget) QDockWidget_PTR() *QDockWidget { return ptr }

impl /*struct*/ QDockWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDockWidget {
    return QDockWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDockWidget {
//  type Target = QDockWidgetBASE;
//
//  fn deref(&self) -> &QDockWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDockWidgetBASE> for QDockWidget {
//  fn as_ref(& self) -> & QDockWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdockwidget.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDockWidget {
  pub fn metaObject_0<RetType, T: QDockWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDockWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDockWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDockWidget(const QString &, QWidget *, Qt::WindowFlags)

/*
Constructs a QDockWidget with parent parent and window flags flags. The dock widget will be placed in the left dock widget area.

The window title is set to title. This title is used when the QDockWidget is docked and undocked. It is also used in the context menu provided by QMainWindow.

See also setWindowTitle().
*/
// QDockWidget(const QString &, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QDockWidget {
  pub fn QDockWidget_0<T: QDockWidget_QDockWidget_0>(value: T) -> QDockWidget {
    let rsthis = value.QDockWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDockWidget_QDockWidget_0 {
  fn QDockWidget_0(self) -> QDockWidget;
}
// QDockWidget(const QString &, QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDockWidget_QDockWidget_0 for (usize,usize,i32) {
  fn QDockWidget_0(self) -> QDockWidget {
    // unsafe{_ZN11QDockWidgetC2ERK7QStringP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QDockWidgetC2ERK7QStringP7QWidget6QFlagsIN2Qt10WindowTypeEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDockWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:68
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDockWidget(QWidget *, Qt::WindowFlags)

/*
Constructs a QDockWidget with parent parent and window flags flags. The dock widget will be placed in the left dock widget area.

The window title is set to title. This title is used when the QDockWidget is docked and undocked. It is also used in the context menu provided by QMainWindow.

See also setWindowTitle().
*/
// QDockWidget(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QDockWidget {
  pub fn QDockWidget_1<T: QDockWidget_QDockWidget_1>(value: T) -> QDockWidget {
    let rsthis = value.QDockWidget_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDockWidget_QDockWidget_1 {
  fn QDockWidget_1(self) -> QDockWidget;
}
// QDockWidget(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDockWidget_QDockWidget_1 for (usize,i32) {
  fn QDockWidget_1(self) -> QDockWidget {
    // unsafe{_ZN11QDockWidgetC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QDockWidgetC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDockWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:69
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDockWidget()

/*

*/
pub fn DeleteQDockWidget(this :*mut QDockWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QDockWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdockwidget.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * widget() const

/*
Returns the widget for the dock widget. This function returns zero if the widget has not been set.

See also setWidget().
*/
impl /*struct*/ QDockWidget {
  pub fn widget_0<RetType, T: QDockWidget_widget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widget_0(self);
    // return 1;
  }
}
pub trait QDockWidget_widget_0<RetType> {
  fn widget_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_widget_0<usize> for () {
  fn widget_0(self , rsthis: & QDockWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget6widgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidget(QWidget *)

/*
Sets the widget for the dock widget to widget.

If the dock widget is visible when widget is added, you must show() it explicitly.

Note that you must add the layout of the widget before you call this function; if not, the widget will not be visible.

See also widget().
*/
impl /*struct*/ QDockWidget {
  pub fn setWidget_0<RetType, T: QDockWidget_setWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidget_0(self);
    // return 1;
  }
}
pub trait QDockWidget_setWidget_0<RetType> {
  fn setWidget_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_setWidget_0<(/*void*/)> for (usize) {
  fn setWidget_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget9setWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFeatures(QDockWidget::DockWidgetFeatures)

/*

*/
impl /*struct*/ QDockWidget {
  pub fn setFeatures_0<RetType, T: QDockWidget_setFeatures_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFeatures_0(self);
    // return 1;
  }
}
pub trait QDockWidget_setFeatures_0<RetType> {
  fn setFeatures_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_setFeatures_0<(/*void*/)> for (i32) {
  fn setFeatures_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget11setFeaturesE6QFlagsINS_17DockWidgetFeatureEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] QDockWidget::DockWidgetFeatures features() const

/*

*/
impl /*struct*/ QDockWidget {
  pub fn features_0<RetType, T: QDockWidget_features_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.features_0(self);
    // return 1;
  }
}
pub trait QDockWidget_features_0<RetType> {
  fn features_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_features_0<i32> for () {
  fn features_0(self , rsthis: & QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget8featuresEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFloating(bool)

/*

*/
impl /*struct*/ QDockWidget {
  pub fn setFloating_0<RetType, T: QDockWidget_setFloating_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFloating_0(self);
    // return 1;
  }
}
pub trait QDockWidget_setFloating_0<RetType> {
  fn setFloating_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_setFloating_0<(/*void*/)> for (bool) {
  fn setFloating_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget11setFloatingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isFloating() const

/*

*/
impl /*struct*/ QDockWidget {
  pub fn isFloating_0<RetType, T: QDockWidget_isFloating_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFloating_0(self);
    // return 1;
  }
}
pub trait QDockWidget_isFloating_0<RetType> {
  fn isFloating_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_isFloating_0<bool> for () {
  fn isFloating_0(self , rsthis: & QDockWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget10isFloatingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAllowedAreas(Qt::DockWidgetAreas)

/*

*/
impl /*struct*/ QDockWidget {
  pub fn setAllowedAreas_0<RetType, T: QDockWidget_setAllowedAreas_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAllowedAreas_0(self);
    // return 1;
  }
}
pub trait QDockWidget_setAllowedAreas_0<RetType> {
  fn setAllowedAreas_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_setAllowedAreas_0<(/*void*/)> for (i32) {
  fn setAllowedAreas_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget15setAllowedAreasE6QFlagsIN2Qt14DockWidgetAreaEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DockWidgetAreas allowedAreas() const

/*

*/
impl /*struct*/ QDockWidget {
  pub fn allowedAreas_0<RetType, T: QDockWidget_allowedAreas_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allowedAreas_0(self);
    // return 1;
  }
}
pub trait QDockWidget_allowedAreas_0<RetType> {
  fn allowedAreas_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_allowedAreas_0<i32> for () {
  fn allowedAreas_0(self , rsthis: & QDockWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget12allowedAreasEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitleBarWidget(QWidget *)

/*
Sets an arbitrary widget as the dock widget's title bar. If widget is 0, any custom title bar widget previously set on the dock widget is removed, but not deleted, and the default title bar will be used instead.

If a title bar widget is set, QDockWidget will not use native window decorations when it is floated.

Here are some tips for implementing custom title bars:


Mouse events that are not explicitly handled by the title bar widget must be ignored by calling QMouseEvent::ignore(). These events then propagate to the QDockWidget parent, which handles them in the usual manner, moving when the title bar is dragged, docking and undocking when it is double-clicked, etc.
When DockWidgetVerticalTitleBar is set on QDockWidget, the title bar widget is repositioned accordingly. In resizeEvent(), the title bar should check what orientation it should assume:
  QDockWidget *dockWidget = qobject_cast<QDockWidget*>(parentWidget());
  if (dockWidget->features() & QDockWidget::DockWidgetVerticalTitleBar) {
      // I need to be vertical
  } else {
      // I need to be horizontal
  }



The title bar widget must have a valid QWidget::sizeHint() and QWidget::minimumSizeHint(). These functions should take into account the current orientation of the title bar.
It is not possible to remove a title bar from a dock widget. However, a similar effect can be achieved by setting a default constructed QWidget as the title bar widget.


Using qobject_cast() as shown above, the title bar widget has full access to its parent QDockWidget. Hence it can perform such operations as docking and hiding in response to user actions.

This function was introduced in  Qt 4.3.

See also titleBarWidget() and DockWidgetVerticalTitleBar.
*/
impl /*struct*/ QDockWidget {
  pub fn setTitleBarWidget_0<RetType, T: QDockWidget_setTitleBarWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitleBarWidget_0(self);
    // return 1;
  }
}
pub trait QDockWidget_setTitleBarWidget_0<RetType> {
  fn setTitleBarWidget_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_setTitleBarWidget_0<(/*void*/)> for (usize) {
  fn setTitleBarWidget_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget17setTitleBarWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:99
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * titleBarWidget() const

/*
Returns the custom title bar widget set on the QDockWidget, or 0 if no custom title bar has been set.

This function was introduced in  Qt 4.3.

See also setTitleBarWidget().
*/
impl /*struct*/ QDockWidget {
  pub fn titleBarWidget_0<RetType, T: QDockWidget_titleBarWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.titleBarWidget_0(self);
    // return 1;
  }
}
pub trait QDockWidget_titleBarWidget_0<RetType> {
  fn titleBarWidget_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_titleBarWidget_0<usize> for () {
  fn titleBarWidget_0(self , rsthis: & QDockWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget14titleBarWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:101
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAreaAllowed(Qt::DockWidgetArea) const

/*
Returns true if this dock widget can be placed in the given area; otherwise returns false.
*/
impl /*struct*/ QDockWidget {
  pub fn isAreaAllowed_0<RetType, T: QDockWidget_isAreaAllowed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAreaAllowed_0(self);
    // return 1;
  }
}
pub trait QDockWidget_isAreaAllowed_0<RetType> {
  fn isAreaAllowed_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_isAreaAllowed_0<bool> for (i32) {
  fn isAreaAllowed_0(self , rsthis: & QDockWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget13isAreaAllowedEN2Qt14DockWidgetAreaE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QAction * toggleViewAction() const

/*
Returns a checkable action that can be used to show or close this dock widget.

The action's text is set to the dock widget's window title.

See also QAction::text and QWidget::windowTitle.
*/
impl /*struct*/ QDockWidget {
  pub fn toggleViewAction_0<RetType, T: QDockWidget_toggleViewAction_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toggleViewAction_0(self);
    // return 1;
  }
}
pub trait QDockWidget_toggleViewAction_0<RetType> {
  fn toggleViewAction_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_toggleViewAction_0<usize> for () {
  fn toggleViewAction_0(self , rsthis: & QDockWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QDockWidget16toggleViewActionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void featuresChanged(QDockWidget::DockWidgetFeatures)

/*
This signal is emitted when the features property changes. The features parameter gives the new value of the property.

Note: Notifier signal for property features.
*/
impl /*struct*/ QDockWidget {
  pub fn featuresChanged_0<RetType, T: QDockWidget_featuresChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.featuresChanged_0(self);
    // return 1;
  }
}
pub trait QDockWidget_featuresChanged_0<RetType> {
  fn featuresChanged_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_featuresChanged_0<(/*void*/)> for (i32) {
  fn featuresChanged_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget15featuresChangedE6QFlagsINS_17DockWidgetFeatureEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void topLevelChanged(bool)

/*
This signal is emitted when the floating property changes. The topLevel parameter is true if the dock widget is now floating; otherwise it is false.

See also isWindow().
*/
impl /*struct*/ QDockWidget {
  pub fn topLevelChanged_0<RetType, T: QDockWidget_topLevelChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topLevelChanged_0(self);
    // return 1;
  }
}
pub trait QDockWidget_topLevelChanged_0<RetType> {
  fn topLevelChanged_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_topLevelChanged_0<(/*void*/)> for (bool) {
  fn topLevelChanged_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget15topLevelChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void allowedAreasChanged(Qt::DockWidgetAreas)

/*
This signal is emitted when the allowedAreas property changes. The allowedAreas parameter gives the new value of the property.

Note: Notifier signal for property allowedAreas.
*/
impl /*struct*/ QDockWidget {
  pub fn allowedAreasChanged_0<RetType, T: QDockWidget_allowedAreasChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allowedAreasChanged_0(self);
    // return 1;
  }
}
pub trait QDockWidget_allowedAreasChanged_0<RetType> {
  fn allowedAreasChanged_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_allowedAreasChanged_0<(/*void*/)> for (i32) {
  fn allowedAreasChanged_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget19allowedAreasChangedE6QFlagsIN2Qt14DockWidgetAreaEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void visibilityChanged(bool)

/*
This signal is emitted when the dock widget becomes visible (or invisible). This happens when the widget is hidden or shown, as well as when it is docked in a tabbed dock area and its tab becomes selected or unselected.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QDockWidget {
  pub fn visibilityChanged_0<RetType, T: QDockWidget_visibilityChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibilityChanged_0(self);
    // return 1;
  }
}
pub trait QDockWidget_visibilityChanged_0<RetType> {
  fn visibilityChanged_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_visibilityChanged_0<(/*void*/)> for (bool) {
  fn visibilityChanged_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget17visibilityChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void dockLocationChanged(Qt::DockWidgetArea)

/*
This signal is emitted when the dock widget is moved to another dock area, or is moved to a different location in its current dock area. This happens when the dock widget is moved programmatically or is dragged to a new location by the user.

This function was introduced in  Qt 4.3.
*/
impl /*struct*/ QDockWidget {
  pub fn dockLocationChanged_0<RetType, T: QDockWidget_dockLocationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dockLocationChanged_0(self);
    // return 1;
  }
}
pub trait QDockWidget_dockLocationChanged_0<RetType> {
  fn dockLocationChanged_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_dockLocationChanged_0<(/*void*/)> for (i32) {
  fn dockLocationChanged_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget19dockLocationChangedEN2Qt14DockWidgetAreaE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:116
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void changeEvent(QEvent *)

/*
Reimplemented from QWidget::changeEvent().
*/
impl /*struct*/ QDockWidget {
  pub fn changeEvent_0<RetType, T: QDockWidget_changeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changeEvent_0(self);
    // return 1;
  }
}
pub trait QDockWidget_changeEvent_0<RetType> {
  fn changeEvent_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_changeEvent_0<(/*void*/)> for (usize) {
  fn changeEvent_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget11changeEventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:117
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void closeEvent(QCloseEvent *)

/*
Reimplemented from QWidget::closeEvent().
*/
impl /*struct*/ QDockWidget {
  pub fn closeEvent_0<RetType, T: QDockWidget_closeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closeEvent_0(self);
    // return 1;
  }
}
pub trait QDockWidget_closeEvent_0<RetType> {
  fn closeEvent_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_closeEvent_0<(/*void*/)> for (usize) {
  fn closeEvent_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget10closeEventEP11QCloseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:118
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void paintEvent(QPaintEvent *)

/*
Reimplemented from QWidget::paintEvent().
*/
impl /*struct*/ QDockWidget {
  pub fn paintEvent_0<RetType, T: QDockWidget_paintEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paintEvent_0(self);
    // return 1;
  }
}
pub trait QDockWidget_paintEvent_0<RetType> {
  fn paintEvent_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_paintEvent_0<(/*void*/)> for (usize) {
  fn paintEvent_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QDockWidget10paintEventEP11QPaintEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QDockWidget {
  pub fn event_0<RetType, T: QDockWidget_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QDockWidget_event_0<RetType> {
  fn event_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QDockWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QDockWidget5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdockwidget.h:120
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void initStyleOption(QStyleOptionDockWidget *) const

/*
Initialize option with the values from this QDockWidget. This method is useful for subclasses when they need a QStyleOptionDockWidget, but don't want to fill in all the information themselves.

See also QStyleOption::initFrom().
*/
impl /*struct*/ QDockWidget {
  pub fn initStyleOption_0<RetType, T: QDockWidget_initStyleOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initStyleOption_0(self);
    // return 1;
  }
}
pub trait QDockWidget_initStyleOption_0<RetType> {
  fn initStyleOption_0(self , rsthis: & QDockWidget) -> RetType;
}
impl<'a> /*trait*/ QDockWidget_initStyleOption_0<(/*void*/)> for (usize) {
  fn initStyleOption_0(self , rsthis: & QDockWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK11QDockWidget15initStyleOptionEP22QStyleOptionDockWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QDockWidget__DockWidgetFeature = i32;
// 
pub const QDockWidget__DockWidgetClosable :QDockWidget__DockWidgetFeature = 1;
// 
pub const QDockWidget__DockWidgetMovable :QDockWidget__DockWidgetFeature = 2;
// 
pub const QDockWidget__DockWidgetFloatable :QDockWidget__DockWidgetFeature = 4;
// 
pub const QDockWidget__DockWidgetVerticalTitleBar :QDockWidget__DockWidgetFeature = 8;
// 
pub const QDockWidget__DockWidgetFeatureMask :QDockWidget__DockWidgetFeature = 15;
// 
pub const QDockWidget__AllDockWidgetFeatures :QDockWidget__DockWidgetFeature = 7;
// 
pub const QDockWidget__NoDockWidgetFeatures :QDockWidget__DockWidgetFeature = 0;
// 
pub const QDockWidget__Reserved :QDockWidget__DockWidgetFeature = 255;
pub fn QDockWidget_DockWidgetFeatureItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDockWidget", val);
}
pub fn QDockWidget_DockWidgetFeatureItemName_s(val: i32) ->String {
  //var nilthis *QDockWidget
  //return nilthis.DockWidgetFeatureItemName(val);
  return QDockWidget_DockWidgetFeatureItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
