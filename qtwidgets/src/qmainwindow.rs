

// mod ::widgets::QMainWindow
// package qtwidgets
// /usr/include/qt/QtWidgets/qmainwindow.h
// #include <qmainwindow.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 56
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

// void contextMenuEvent(QContextMenuEvent *)
// func (this *QMainWindow) InheritContextMenuEvent(f func(event *qtgui.QContextMenuEvent/*777 QContextMenuEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "contextMenuEvent", f)
// }

// bool event(QEvent *)
// func (this *QMainWindow) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QMainWindow)=48
pub struct QMainWindow {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMainWindow_ITF interface {
//    QWidget_ITF
//    QMainWindow_PTR() *QMainWindow
//}
//func (ptr *QMainWindow) QMainWindow_PTR() *QMainWindow { return ptr }

impl /*struct*/ QMainWindow {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMainWindow {
    return QMainWindow{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMainWindow {
//  type Target = QMainWindowBASE;
//
//  fn deref(&self) -> &QMainWindowBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMainWindowBASE> for QMainWindow {
//  fn as_ref(& self) -> & QMainWindowBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qmainwindow.h:62
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn metaObject_0<RetType, T: QMainWindow_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QMainWindow_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMainWindow(QWidget *, Qt::WindowFlags)

/*
Constructs a QMainWindow with the given parent and the specified widget flags.

QMainWindow sets the Qt::Window flag itself, and will hence always be created as a top-level widget.
*/
// QMainWindow(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl /*struct*/ QMainWindow {
  pub fn QMainWindow_0<T: QMainWindow_QMainWindow_0>(value: T) -> QMainWindow {
    let rsthis = value.QMainWindow_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMainWindow_QMainWindow_0 {
  fn QMainWindow_0(self) -> QMainWindow;
}
// QMainWindow(QWidget *, Qt::WindowFlags) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMainWindow_QMainWindow_0 for (usize,i32) {
  fn QMainWindow_0(self) -> QMainWindow {
    // unsafe{_ZN11QMainWindowC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QMainWindowC2EP7QWidget6QFlagsIN2Qt10WindowTypeEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMainWindow{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:95
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMainWindow()

/*

*/
pub fn DeleteQMainWindow(this :*mut QMainWindow) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QMainWindowD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qmainwindow.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize iconSize() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn iconSize_0<RetType, T: QMainWindow_iconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSize_0(self);
    // return 1;
  }
}
pub trait QMainWindow_iconSize_0<RetType> {
  fn iconSize_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_iconSize_0<usize> for () {
  fn iconSize_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow8iconSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIconSize(const QSize &)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setIconSize_0<RetType, T: QMainWindow_setIconSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIconSize_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setIconSize_0<RetType> {
  fn setIconSize_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setIconSize_0<(/*void*/)> for (usize) {
  fn setIconSize_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow11setIconSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ToolButtonStyle toolButtonStyle() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn toolButtonStyle_0<RetType, T: QMainWindow_toolButtonStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolButtonStyle_0(self);
    // return 1;
  }
}
pub trait QMainWindow_toolButtonStyle_0<RetType> {
  fn toolButtonStyle_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_toolButtonStyle_0<i32> for () {
  fn toolButtonStyle_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow15toolButtonStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolButtonStyle(Qt::ToolButtonStyle)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setToolButtonStyle_0<RetType, T: QMainWindow_setToolButtonStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolButtonStyle_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setToolButtonStyle_0<RetType> {
  fn setToolButtonStyle_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setToolButtonStyle_0<(/*void*/)> for (i32) {
  fn setToolButtonStyle_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow18setToolButtonStyleEN2Qt15ToolButtonStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:104
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAnimated() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn isAnimated_0<RetType, T: QMainWindow_isAnimated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAnimated_0(self);
    // return 1;
  }
}
pub trait QMainWindow_isAnimated_0<RetType> {
  fn isAnimated_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_isAnimated_0<bool> for () {
  fn isAnimated_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow10isAnimatedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDockNestingEnabled() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn isDockNestingEnabled_0<RetType, T: QMainWindow_isDockNestingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDockNestingEnabled_0(self);
    // return 1;
  }
}
pub trait QMainWindow_isDockNestingEnabled_0<RetType> {
  fn isDockNestingEnabled_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_isDockNestingEnabled_0<bool> for () {
  fn isDockNestingEnabled_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow20isDockNestingEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool documentMode() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn documentMode_0<RetType, T: QMainWindow_documentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.documentMode_0(self);
    // return 1;
  }
}
pub trait QMainWindow_documentMode_0<RetType> {
  fn documentMode_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_documentMode_0<bool> for () {
  fn documentMode_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow12documentModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDocumentMode(bool)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setDocumentMode_0<RetType, T: QMainWindow_setDocumentMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setDocumentMode_0<RetType> {
  fn setDocumentMode_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setDocumentMode_0<(/*void*/)> for (bool) {
  fn setDocumentMode_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow15setDocumentModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:114
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabWidget::TabShape tabShape() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn tabShape_0<RetType, T: QMainWindow_tabShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabShape_0(self);
    // return 1;
  }
}
pub trait QMainWindow_tabShape_0<RetType> {
  fn tabShape_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_tabShape_0<i32> for () {
  fn tabShape_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow8tabShapeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabShape(QTabWidget::TabShape)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setTabShape_0<RetType, T: QMainWindow_setTabShape_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabShape_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setTabShape_0<RetType> {
  fn setTabShape_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setTabShape_0<(/*void*/)> for (i32) {
  fn setTabShape_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow11setTabShapeEN10QTabWidget8TabShapeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:116
// index:0
// Public Visibility=Default Availability=Available
// [4] QTabWidget::TabPosition tabPosition(Qt::DockWidgetArea) const

/*
Returns the tab position for area.

Note: The VerticalTabs dock option overrides the tab positions returned by this function.

This function was introduced in  Qt 4.5.

See also setTabPosition() and tabShape().
*/
impl /*struct*/ QMainWindow {
  pub fn tabPosition_0<RetType, T: QMainWindow_tabPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabPosition_0(self);
    // return 1;
  }
}
pub trait QMainWindow_tabPosition_0<RetType> {
  fn tabPosition_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_tabPosition_0<i32> for (i32) {
  fn tabPosition_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow11tabPositionEN2Qt14DockWidgetAreaE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTabPosition(Qt::DockWidgetAreas, QTabWidget::TabPosition)

/*
Sets the tab position for the given dock widget areas to the specified tabPosition. By default, all dock areas show their tabs at the bottom.

Note: The VerticalTabs dock option overrides the tab positions set by this method.

This function was introduced in  Qt 4.5.

See also tabPosition() and setTabShape().
*/
impl /*struct*/ QMainWindow {
  pub fn setTabPosition_0<RetType, T: QMainWindow_setTabPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTabPosition_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setTabPosition_0<RetType> {
  fn setTabPosition_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setTabPosition_0<(/*void*/)> for (i32,i32) {
  fn setTabPosition_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow14setTabPositionE6QFlagsIN2Qt14DockWidgetAreaEEN10QTabWidget11TabPositionE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDockOptions(QMainWindow::DockOptions)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setDockOptions_0<RetType, T: QMainWindow_setDockOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDockOptions_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setDockOptions_0<RetType> {
  fn setDockOptions_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setDockOptions_0<(/*void*/)> for (i32) {
  fn setDockOptions_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow14setDockOptionsE6QFlagsINS_10DockOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:121
// index:0
// Public Visibility=Default Availability=Available
// [4] QMainWindow::DockOptions dockOptions() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn dockOptions_0<RetType, T: QMainWindow_dockOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dockOptions_0(self);
    // return 1;
  }
}
pub trait QMainWindow_dockOptions_0<RetType> {
  fn dockOptions_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_dockOptions_0<i32> for () {
  fn dockOptions_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow11dockOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:123
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSeparator(const QPoint &) const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn isSeparator_0<RetType, T: QMainWindow_isSeparator_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSeparator_0(self);
    // return 1;
  }
}
pub trait QMainWindow_isSeparator_0<RetType> {
  fn isSeparator_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_isSeparator_0<bool> for (usize) {
  fn isSeparator_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow11isSeparatorERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:126
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenuBar * menuBar() const

/*
Returns the menu bar for the main window. This function creates and returns an empty menu bar if the menu bar does not exist.

If you want all windows in a Mac application to share one menu bar, don't use this function to create it, because the menu bar created here will have this QMainWindow as its parent. Instead, you must create a menu bar that does not have a parent, which you can then share among all the Mac windows. Create a parent-less menu bar this way:


  QMenuBar *menuBar = new QMenuBar(0);



See also setMenuBar().
*/
impl /*struct*/ QMainWindow {
  pub fn menuBar_0<RetType, T: QMainWindow_menuBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menuBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_menuBar_0<RetType> {
  fn menuBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_menuBar_0<usize> for () {
  fn menuBar_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow7menuBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenuBar(QMenuBar *)

/*
Sets the menu bar for the main window to menuBar.

Note: QMainWindow takes ownership of the menuBar pointer and deletes it at the appropriate time.

See also menuBar().
*/
impl /*struct*/ QMainWindow {
  pub fn setMenuBar_0<RetType, T: QMainWindow_setMenuBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenuBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setMenuBar_0<RetType> {
  fn setMenuBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setMenuBar_0<(/*void*/)> for (usize) {
  fn setMenuBar_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow10setMenuBarEP8QMenuBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * menuWidget() const

/*
Returns the menu bar for the main window. This function returns null if a menu bar hasn't been constructed yet.

This function was introduced in  Qt 4.2.

See also setMenuWidget().
*/
impl /*struct*/ QMainWindow {
  pub fn menuWidget_0<RetType, T: QMainWindow_menuWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.menuWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_menuWidget_0<RetType> {
  fn menuWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_menuWidget_0<usize> for () {
  fn menuWidget_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow10menuWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMenuWidget(QWidget *)

/*
Sets the menu bar for the main window to menuBar.

QMainWindow takes ownership of the menuBar pointer and deletes it at the appropriate time.

This function was introduced in  Qt 4.2.

See also menuWidget().
*/
impl /*struct*/ QMainWindow {
  pub fn setMenuWidget_0<RetType, T: QMainWindow_setMenuWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMenuWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setMenuWidget_0<RetType> {
  fn setMenuWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setMenuWidget_0<(/*void*/)> for (usize) {
  fn setMenuWidget_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow13setMenuWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:134
// index:0
// Public Visibility=Default Availability=Available
// [8] QStatusBar * statusBar() const

/*
Returns the status bar for the main window. This function creates and returns an empty status bar if the status bar does not exist.

See also setStatusBar().
*/
impl /*struct*/ QMainWindow {
  pub fn statusBar_0<RetType, T: QMainWindow_statusBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.statusBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_statusBar_0<RetType> {
  fn statusBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_statusBar_0<usize> for () {
  fn statusBar_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow9statusBarEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStatusBar(QStatusBar *)

/*
Sets the status bar for the main window to statusbar.

Setting the status bar to 0 will remove it from the main window. Note that QMainWindow takes ownership of the statusbar pointer and deletes it at the appropriate time.

See also statusBar().
*/
impl /*struct*/ QMainWindow {
  pub fn setStatusBar_0<RetType, T: QMainWindow_setStatusBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStatusBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setStatusBar_0<RetType> {
  fn setStatusBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setStatusBar_0<(/*void*/)> for (usize) {
  fn setStatusBar_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow12setStatusBarEP10QStatusBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * centralWidget() const

/*
Returns the central widget for the main window. This function returns zero if the central widget has not been set.

See also setCentralWidget().
*/
impl /*struct*/ QMainWindow {
  pub fn centralWidget_0<RetType, T: QMainWindow_centralWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.centralWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_centralWidget_0<RetType> {
  fn centralWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_centralWidget_0<usize> for () {
  fn centralWidget_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow13centralWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCentralWidget(QWidget *)

/*
Sets the given widget to be the main window's central widget.

Note: QMainWindow takes ownership of the widget pointer and deletes it at the appropriate time.

See also centralWidget().
*/
impl /*struct*/ QMainWindow {
  pub fn setCentralWidget_0<RetType, T: QMainWindow_setCentralWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCentralWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setCentralWidget_0<RetType> {
  fn setCentralWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setCentralWidget_0<(/*void*/)> for (usize) {
  fn setCentralWidget_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow16setCentralWidgetEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * takeCentralWidget()

/*
Removes the central widget from this main window.

The ownership of the removed widget is passed to the caller.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QMainWindow {
  pub fn takeCentralWidget_0<RetType, T: QMainWindow_takeCentralWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.takeCentralWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_takeCentralWidget_0<RetType> {
  fn takeCentralWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_takeCentralWidget_0<usize> for () {
  fn takeCentralWidget_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMainWindow17takeCentralWidgetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCorner(Qt::Corner, Qt::DockWidgetArea)

/*
Sets the given dock widget area to occupy the specified corner.

See also corner().
*/
impl /*struct*/ QMainWindow {
  pub fn setCorner_0<RetType, T: QMainWindow_setCorner_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCorner_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setCorner_0<RetType> {
  fn setCorner_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setCorner_0<(/*void*/)> for (i32,i32) {
  fn setCorner_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow9setCornerEN2Qt6CornerENS0_14DockWidgetAreaE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:145
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DockWidgetArea corner(Qt::Corner) const

/*
Returns the dock widget area that occupies the specified corner.

See also setCorner().
*/
impl /*struct*/ QMainWindow {
  pub fn corner_0<RetType, T: QMainWindow_corner_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.corner_0(self);
    // return 1;
  }
}
pub trait QMainWindow_corner_0<RetType> {
  fn corner_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_corner_0<i32> for (i32) {
  fn corner_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow6cornerEN2Qt6CornerE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addToolBarBreak(Qt::ToolBarArea)

/*
Adds a toolbar break to the given area after all the other objects that are present.
*/
impl /*struct*/ QMainWindow {
  pub fn addToolBarBreak_0<RetType, T: QMainWindow_addToolBarBreak_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addToolBarBreak_0(self);
    // return 1;
  }
}
pub trait QMainWindow_addToolBarBreak_0<RetType> {
  fn addToolBarBreak_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_addToolBarBreak_0<(/*void*/)> for (i32) {
  fn addToolBarBreak_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow15addToolBarBreakEN2Qt11ToolBarAreaE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertToolBarBreak(QToolBar *)

/*
Inserts a toolbar break before the toolbar specified by before.
*/
impl /*struct*/ QMainWindow {
  pub fn insertToolBarBreak_0<RetType, T: QMainWindow_insertToolBarBreak_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertToolBarBreak_0(self);
    // return 1;
  }
}
pub trait QMainWindow_insertToolBarBreak_0<RetType> {
  fn insertToolBarBreak_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_insertToolBarBreak_0<(/*void*/)> for (usize) {
  fn insertToolBarBreak_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow18insertToolBarBreakEP8QToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addToolBar(Qt::ToolBarArea, QToolBar *)

/*
Adds the toolbar into the specified area in this main window. The toolbar is placed at the end of the current tool bar block (i.e. line). If the main window already manages toolbar then it will only move the toolbar to area.

See also insertToolBar(), addToolBarBreak(), and insertToolBarBreak().
*/
impl /*struct*/ QMainWindow {
  pub fn addToolBar_0<RetType, T: QMainWindow_addToolBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addToolBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_addToolBar_0<RetType> {
  fn addToolBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_addToolBar_0<(/*void*/)> for (i32,usize) {
  fn addToolBar_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow10addToolBarEN2Qt11ToolBarAreaEP8QToolBar", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:153
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addToolBar(QToolBar *)

/*
Adds the toolbar into the specified area in this main window. The toolbar is placed at the end of the current tool bar block (i.e. line). If the main window already manages toolbar then it will only move the toolbar to area.

See also insertToolBar(), addToolBarBreak(), and insertToolBarBreak().
*/
impl /*struct*/ QMainWindow {
  pub fn addToolBar_1<RetType, T: QMainWindow_addToolBar_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addToolBar_1(self);
    // return 1;
  }
}
pub trait QMainWindow_addToolBar_1<RetType> {
  fn addToolBar_1(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_addToolBar_1<(/*void*/)> for (usize) {
  fn addToolBar_1(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow10addToolBarEP8QToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:154
// index:2
// Public Visibility=Default Availability=Available
// [8] QToolBar * addToolBar(const QString &)

/*
Adds the toolbar into the specified area in this main window. The toolbar is placed at the end of the current tool bar block (i.e. line). If the main window already manages toolbar then it will only move the toolbar to area.

See also insertToolBar(), addToolBarBreak(), and insertToolBarBreak().
*/
impl /*struct*/ QMainWindow {
  pub fn addToolBar_2<RetType, T: QMainWindow_addToolBar_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addToolBar_2(self);
    // return 1;
  }
}
pub trait QMainWindow_addToolBar_2<RetType> {
  fn addToolBar_2(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_addToolBar_2<usize> for (usize) {
  fn addToolBar_2(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMainWindow10addToolBarERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void insertToolBar(QToolBar *, QToolBar *)

/*
Inserts the toolbar into the area occupied by the before toolbar so that it appears before it. For example, in normal left-to-right layout operation, this means that toolbar will appear to the left of the toolbar specified by before in a horizontal toolbar area.

See also insertToolBarBreak(), addToolBar(), and addToolBarBreak().
*/
impl /*struct*/ QMainWindow {
  pub fn insertToolBar_0<RetType, T: QMainWindow_insertToolBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.insertToolBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_insertToolBar_0<RetType> {
  fn insertToolBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_insertToolBar_0<(/*void*/)> for (usize,usize) {
  fn insertToolBar_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow13insertToolBarEP8QToolBarS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeToolBar(QToolBar *)

/*
Removes the toolbar from the main window layout and hides it. Note that the toolbar is not deleted.
*/
impl /*struct*/ QMainWindow {
  pub fn removeToolBar_0<RetType, T: QMainWindow_removeToolBar_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeToolBar_0(self);
    // return 1;
  }
}
pub trait QMainWindow_removeToolBar_0<RetType> {
  fn removeToolBar_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_removeToolBar_0<(/*void*/)> for (usize) {
  fn removeToolBar_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow13removeToolBarEP8QToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeToolBarBreak(QToolBar *)

/*
Removes a toolbar break previously inserted before the toolbar specified by before.
*/
impl /*struct*/ QMainWindow {
  pub fn removeToolBarBreak_0<RetType, T: QMainWindow_removeToolBarBreak_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeToolBarBreak_0(self);
    // return 1;
  }
}
pub trait QMainWindow_removeToolBarBreak_0<RetType> {
  fn removeToolBarBreak_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_removeToolBarBreak_0<(/*void*/)> for (usize) {
  fn removeToolBarBreak_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow18removeToolBarBreakEP8QToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:159
// index:0
// Public Visibility=Default Availability=Available
// [1] bool unifiedTitleAndToolBarOnMac() const

/*

*/
impl /*struct*/ QMainWindow {
  pub fn unifiedTitleAndToolBarOnMac_0<RetType, T: QMainWindow_unifiedTitleAndToolBarOnMac_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unifiedTitleAndToolBarOnMac_0(self);
    // return 1;
  }
}
pub trait QMainWindow_unifiedTitleAndToolBarOnMac_0<RetType> {
  fn unifiedTitleAndToolBarOnMac_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_unifiedTitleAndToolBarOnMac_0<bool> for () {
  fn unifiedTitleAndToolBarOnMac_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:161
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ToolBarArea toolBarArea(QToolBar *) const

/*
Returns the Qt::ToolBarArea for toolbar. If toolbar has not been added to the main window, this function returns Qt::NoToolBarArea.

See also addToolBar(), addToolBarBreak(), and Qt::ToolBarArea.
*/
impl /*struct*/ QMainWindow {
  pub fn toolBarArea_0<RetType, T: QMainWindow_toolBarArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolBarArea_0(self);
    // return 1;
  }
}
pub trait QMainWindow_toolBarArea_0<RetType> {
  fn toolBarArea_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_toolBarArea_0<i32> for (usize) {
  fn toolBarArea_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow11toolBarAreaEP8QToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:162
// index:0
// Public Visibility=Default Availability=Available
// [1] bool toolBarBreak(QToolBar *) const

/*
Returns whether there is a toolbar break before the toolbar.

See also addToolBarBreak() and insertToolBarBreak().
*/
impl /*struct*/ QMainWindow {
  pub fn toolBarBreak_0<RetType, T: QMainWindow_toolBarBreak_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolBarBreak_0(self);
    // return 1;
  }
}
pub trait QMainWindow_toolBarBreak_0<RetType> {
  fn toolBarBreak_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_toolBarBreak_0<bool> for (usize) {
  fn toolBarBreak_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow12toolBarBreakEP8QToolBar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:165
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addDockWidget(Qt::DockWidgetArea, QDockWidget *)

/*
Adds the given dockwidget to the specified area.
*/
impl /*struct*/ QMainWindow {
  pub fn addDockWidget_0<RetType, T: QMainWindow_addDockWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addDockWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_addDockWidget_0<RetType> {
  fn addDockWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_addDockWidget_0<(/*void*/)> for (i32,usize) {
  fn addDockWidget_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow13addDockWidgetEN2Qt14DockWidgetAreaEP11QDockWidget", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:166
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addDockWidget(Qt::DockWidgetArea, QDockWidget *, Qt::Orientation)

/*
Adds the given dockwidget to the specified area.
*/
impl /*struct*/ QMainWindow {
  pub fn addDockWidget_1<RetType, T: QMainWindow_addDockWidget_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addDockWidget_1(self);
    // return 1;
  }
}
pub trait QMainWindow_addDockWidget_1<RetType> {
  fn addDockWidget_1(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_addDockWidget_1<(/*void*/)> for (i32,usize,i32) {
  fn addDockWidget_1(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow13addDockWidgetEN2Qt14DockWidgetAreaEP11QDockWidgetNS0_11OrientationE", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void splitDockWidget(QDockWidget *, QDockWidget *, Qt::Orientation)

/*
Splits the space covered by the first dock widget into two parts, moves the first dock widget into the first part, and moves the second dock widget into the second part.

The orientation specifies how the space is divided: A Qt::Horizontal split places the second dock widget to the right of the first; a Qt::Vertical split places the second dock widget below the first.

Note: if first is currently in a tabbed docked area, second will be added as a new tab, not as a neighbor of first. This is because a single tab can contain only one dock widget.

Note: The Qt::LayoutDirection influences the order of the dock widgets in the two parts of the divided area. When right-to-left layout direction is enabled, the placing of the dock widgets will be reversed.

See also tabifyDockWidget(), addDockWidget(), and removeDockWidget().
*/
impl /*struct*/ QMainWindow {
  pub fn splitDockWidget_0<RetType, T: QMainWindow_splitDockWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.splitDockWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_splitDockWidget_0<RetType> {
  fn splitDockWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_splitDockWidget_0<(/*void*/)> for (usize,usize,i32) {
  fn splitDockWidget_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow15splitDockWidgetEP11QDockWidgetS1_N2Qt11OrientationE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabifyDockWidget(QDockWidget *, QDockWidget *)

/*
Moves second dock widget on top of first dock widget, creating a tabbed docked area in the main window.

See also tabifiedDockWidgets().
*/
impl /*struct*/ QMainWindow {
  pub fn tabifyDockWidget_0<RetType, T: QMainWindow_tabifyDockWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabifyDockWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_tabifyDockWidget_0<RetType> {
  fn tabifyDockWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_tabifyDockWidget_0<(/*void*/)> for (usize,usize) {
  fn tabifyDockWidget_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeDockWidget(QDockWidget *)

/*
Removes the dockwidget from the main window layout and hides it. Note that the dockwidget is not deleted.
*/
impl /*struct*/ QMainWindow {
  pub fn removeDockWidget_0<RetType, T: QMainWindow_removeDockWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeDockWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_removeDockWidget_0<RetType> {
  fn removeDockWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_removeDockWidget_0<(/*void*/)> for (usize) {
  fn removeDockWidget_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow16removeDockWidgetEP11QDockWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:173
// index:0
// Public Visibility=Default Availability=Available
// [1] bool restoreDockWidget(QDockWidget *)

/*
Restores the state of dockwidget if it is created after the call to restoreState(). Returns true if the state was restored; otherwise returns false.

See also restoreState() and saveState().
*/
impl /*struct*/ QMainWindow {
  pub fn restoreDockWidget_0<RetType, T: QMainWindow_restoreDockWidget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restoreDockWidget_0(self);
    // return 1;
  }
}
pub trait QMainWindow_restoreDockWidget_0<RetType> {
  fn restoreDockWidget_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_restoreDockWidget_0<bool> for (usize) {
  fn restoreDockWidget_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:175
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::DockWidgetArea dockWidgetArea(QDockWidget *) const

/*
Returns the Qt::DockWidgetArea for dockwidget. If dockwidget has not been added to the main window, this function returns Qt::NoDockWidgetArea.

See also addDockWidget(), splitDockWidget(), and Qt::DockWidgetArea.
*/
impl /*struct*/ QMainWindow {
  pub fn dockWidgetArea_0<RetType, T: QMainWindow_dockWidgetArea_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dockWidgetArea_0(self);
    // return 1;
  }
}
pub trait QMainWindow_dockWidgetArea_0<RetType> {
  fn dockWidgetArea_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_dockWidgetArea_0<i32> for (usize) {
  fn dockWidgetArea_0(self , rsthis: & QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow14dockWidgetAreaEP11QDockWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:181
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray saveState(int) const

/*
Saves the current state of this mainwindow's toolbars and dockwidgets. This includes the corner settings which can be set with setCorner(). The version number is stored as part of the data.

The objectName property is used to identify each QToolBar and QDockWidget. You should make sure that this property is unique for each QToolBar and QDockWidget you add to the QMainWindow

To restore the saved state, pass the return value and version number to restoreState().

To save the geometry when the window closes, you can implement a close event like this:


  void MyMainWindow::closeEvent(QCloseEvent *event)
  {
      QSettings settings("MyCompany", "MyApp");
      settings.setValue("geometry", saveGeometry());
      settings.setValue("windowState", saveState());
      QMainWindow::closeEvent(event);
  }



See also restoreState(), QWidget::saveGeometry(), and QWidget::restoreGeometry().
*/
impl /*struct*/ QMainWindow {
  pub fn saveState_0<RetType, T: QMainWindow_saveState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saveState_0(self);
    // return 1;
  }
}
pub trait QMainWindow_saveState_0<RetType> {
  fn saveState_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_saveState_0<usize> for (i32) {
  fn saveState_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QMainWindow9saveStateEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:182
// index:0
// Public Visibility=Default Availability=Available
// [1] bool restoreState(const QByteArray &, int)

/*
Restores the state of this mainwindow's toolbars and dockwidgets. Also restores the corner settings too. The version number is compared with that stored in state. If they do not match, the mainwindow's state is left unchanged, and this function returns false; otherwise, the state is restored, and this function returns true.

To restore geometry saved using QSettings, you can use code like this:


  void MainWindow::readSettings()
  {
      QSettings settings("MyCompany", "MyApp");
      restoreGeometry(settings.value("myWidget/geometry").toByteArray());
      restoreState(settings.value("myWidget/windowState").toByteArray());
  }



See also saveState(), QWidget::saveGeometry(), QWidget::restoreGeometry(), and restoreDockWidget().
*/
impl /*struct*/ QMainWindow {
  pub fn restoreState_0<RetType, T: QMainWindow_restoreState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restoreState_0(self);
    // return 1;
  }
}
pub trait QMainWindow_restoreState_0<RetType> {
  fn restoreState_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_restoreState_0<bool> for (usize,i32) {
  fn restoreState_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMainWindow12restoreStateERK10QByteArrayi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:185
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QMenu * createPopupMenu()

/*
Returns a popup menu containing checkable entries for the toolbars and dock widgets present in the main window. If there are no toolbars and dock widgets present, this function returns a null pointer.

By default, this function is called by the main window when the user activates a context menu, typically by right-clicking on a toolbar or a dock widget.

If you want to create a custom popup menu, reimplement this function and return a newly-created popup menu. Ownership of the popup menu is transferred to the caller.

See also addDockWidget(), addToolBar(), and menuBar().
*/
impl /*struct*/ QMainWindow {
  pub fn createPopupMenu_0<RetType, T: QMainWindow_createPopupMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.createPopupMenu_0(self);
    // return 1;
  }
}
pub trait QMainWindow_createPopupMenu_0<RetType> {
  fn createPopupMenu_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_createPopupMenu_0<usize> for () {
  fn createPopupMenu_0(self , rsthis: & QMainWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMainWindow15createPopupMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:190
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAnimated(bool)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setAnimated_0<RetType, T: QMainWindow_setAnimated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnimated_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setAnimated_0<RetType> {
  fn setAnimated_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setAnimated_0<(/*void*/)> for (bool) {
  fn setAnimated_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow11setAnimatedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:191
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDockNestingEnabled(bool)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setDockNestingEnabled_0<RetType, T: QMainWindow_setDockNestingEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDockNestingEnabled_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setDockNestingEnabled_0<RetType> {
  fn setDockNestingEnabled_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setDockNestingEnabled_0<(/*void*/)> for (bool) {
  fn setDockNestingEnabled_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow21setDockNestingEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setUnifiedTitleAndToolBarOnMac(bool)

/*

*/
impl /*struct*/ QMainWindow {
  pub fn setUnifiedTitleAndToolBarOnMac_0<RetType, T: QMainWindow_setUnifiedTitleAndToolBarOnMac_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setUnifiedTitleAndToolBarOnMac_0(self);
    // return 1;
  }
}
pub trait QMainWindow_setUnifiedTitleAndToolBarOnMac_0<RetType> {
  fn setUnifiedTitleAndToolBarOnMac_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_setUnifiedTitleAndToolBarOnMac_0<(/*void*/)> for (bool) {
  fn setUnifiedTitleAndToolBarOnMac_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:198
// index:0
// Public Visibility=Default Availability=Available
// [-2] void iconSizeChanged(const QSize &)

/*
This signal is emitted when the size of the icons used in the window is changed. The new icon size is passed in iconSize.

You can connect this signal to other components to help maintain a consistent appearance for your application.

See also setIconSize().
*/
impl /*struct*/ QMainWindow {
  pub fn iconSizeChanged_0<RetType, T: QMainWindow_iconSizeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iconSizeChanged_0(self);
    // return 1;
  }
}
pub trait QMainWindow_iconSizeChanged_0<RetType> {
  fn iconSizeChanged_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_iconSizeChanged_0<(/*void*/)> for (usize) {
  fn iconSizeChanged_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow15iconSizeChangedERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void toolButtonStyleChanged(Qt::ToolButtonStyle)

/*
This signal is emitted when the style used for tool buttons in the window is changed. The new style is passed in toolButtonStyle.

You can connect this signal to other components to help maintain a consistent appearance for your application.

See also setToolButtonStyle().
*/
impl /*struct*/ QMainWindow {
  pub fn toolButtonStyleChanged_0<RetType, T: QMainWindow_toolButtonStyleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolButtonStyleChanged_0(self);
    // return 1;
  }
}
pub trait QMainWindow_toolButtonStyleChanged_0<RetType> {
  fn toolButtonStyleChanged_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_toolButtonStyleChanged_0<(/*void*/)> for (i32) {
  fn toolButtonStyleChanged_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow22toolButtonStyleChangedEN2Qt15ToolButtonStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:201
// index:0
// Public Visibility=Default Availability=Available
// [-2] void tabifiedDockWidgetActivated(QDockWidget *)

/*
This signal is emitted when the tabified dock widget is activated by selecting the tab. The activated dock widget is passed in dockWidget.

This function was introduced in  Qt 5.8.

See also tabifyDockWidget() and tabifiedDockWidgets().
*/
impl /*struct*/ QMainWindow {
  pub fn tabifiedDockWidgetActivated_0<RetType, T: QMainWindow_tabifiedDockWidgetActivated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabifiedDockWidgetActivated_0(self);
    // return 1;
  }
}
pub trait QMainWindow_tabifiedDockWidgetActivated_0<RetType> {
  fn tabifiedDockWidgetActivated_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_tabifiedDockWidgetActivated_0<(/*void*/)> for (usize) {
  fn tabifiedDockWidgetActivated_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow27tabifiedDockWidgetActivatedEP11QDockWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:206
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void contextMenuEvent(QContextMenuEvent *)

/*
Reimplemented from QWidget::contextMenuEvent().
*/
impl /*struct*/ QMainWindow {
  pub fn contextMenuEvent_0<RetType, T: QMainWindow_contextMenuEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenuEvent_0(self);
    // return 1;
  }
}
pub trait QMainWindow_contextMenuEvent_0<RetType> {
  fn contextMenuEvent_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_contextMenuEvent_0<(/*void*/)> for (usize) {
  fn contextMenuEvent_0(self , rsthis: & QMainWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QMainWindow16contextMenuEventEP17QContextMenuEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qmainwindow.h:208
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QMainWindow {
  pub fn event_0<RetType, T: QMainWindow_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QMainWindow_event_0<RetType> {
  fn event_0(self , rsthis: & QMainWindow) -> RetType;
}
impl<'a> /*trait*/ QMainWindow_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QMainWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QMainWindow5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QMainWindow__DockOption = i32;
// 
pub const QMainWindow__AnimatedDocks :QMainWindow__DockOption = 1;
// 
pub const QMainWindow__AllowNestedDocks :QMainWindow__DockOption = 2;
// 
pub const QMainWindow__AllowTabbedDocks :QMainWindow__DockOption = 4;
// 
pub const QMainWindow__ForceTabbedDocks :QMainWindow__DockOption = 8;
// 
pub const QMainWindow__VerticalTabs :QMainWindow__DockOption = 16;
// 
pub const QMainWindow__GroupedDragging :QMainWindow__DockOption = 32;
pub fn QMainWindow_DockOptionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QMainWindow", val);
}
pub fn QMainWindow_DockOptionItemName_s(val: i32) ->String {
  //var nilthis *QMainWindow
  //return nilthis.DockOptionItemName(val);
  return QMainWindow_DockOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
