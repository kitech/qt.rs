

// mod ::widgets::QDesktopWidget
// package qtwidgets
// /usr/include/qt/QtWidgets/qdesktopwidget.h
// #include <qdesktopwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
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

// void resizeEvent(QResizeEvent *)
// func (this *QDesktopWidget) InheritResizeEvent(f func(e *qtgui.QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QDesktopWidget)=48
pub struct QDesktopWidget {
  qbase: QWidget,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDesktopWidget_ITF interface {
//    QWidget_ITF
//    QDesktopWidget_PTR() *QDesktopWidget
//}
//func (ptr *QDesktopWidget) QDesktopWidget_PTR() *QDesktopWidget { return ptr }

impl /*struct*/ QDesktopWidget {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDesktopWidget {
    return QDesktopWidget{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDesktopWidget {
//  type Target = QDesktopWidgetBASE;
//
//  fn deref(&self) -> &QDesktopWidgetBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDesktopWidgetBASE> for QDesktopWidget {
//  fn as_ref(& self) -> & QDesktopWidgetBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qdesktopwidget.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDesktopWidget {
  pub fn metaObject_0<RetType, T: QDesktopWidget_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDesktopWidget()

/*

*/
// QDesktopWidget() ctx.fn_proto_cpp
impl /*struct*/ QDesktopWidget {
  pub fn QDesktopWidget_0<T: QDesktopWidget_QDesktopWidget_0>(value: T) -> QDesktopWidget {
    let rsthis = value.QDesktopWidget_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDesktopWidget_QDesktopWidget_0 {
  fn QDesktopWidget_0(self) -> QDesktopWidget;
}
// QDesktopWidget() ctx.fn_proto_cpp
impl<'a> /*trait*/ QDesktopWidget_QDesktopWidget_0 for () {
  fn QDesktopWidget_0(self) -> QDesktopWidget {
    // unsafe{_ZN14QDesktopWidgetC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QDesktopWidgetC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDesktopWidget{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDesktopWidget()

/*

*/
pub fn DeleteQDesktopWidget(this :*mut QDesktopWidget) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QDesktopWidgetD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qdesktopwidget.h:62
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVirtualDesktop() const

/*

*/
impl /*struct*/ QDesktopWidget {
  pub fn isVirtualDesktop_0<RetType, T: QDesktopWidget_isVirtualDesktop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVirtualDesktop_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_isVirtualDesktop_0<RetType> {
  fn isVirtualDesktop_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_isVirtualDesktop_0<bool> for () {
  fn isVirtualDesktop_0(self , rsthis: & QDesktopWidget) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget16isVirtualDesktopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:64
// index:0
// Public Visibility=Default Availability=Available
// [4] int numScreens() const

/*

*/
impl /*struct*/ QDesktopWidget {
  pub fn numScreens_0<RetType, T: QDesktopWidget_numScreens_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numScreens_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_numScreens_0<RetType> {
  fn numScreens_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_numScreens_0<i32> for () {
  fn numScreens_0(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget10numScreensEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] int screenCount() const

/*

*/
impl /*struct*/ QDesktopWidget {
  pub fn screenCount_0<RetType, T: QDesktopWidget_screenCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenCount_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenCount_0<RetType> {
  fn screenCount_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenCount_0<i32> for () {
  fn screenCount_0(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget11screenCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:66
// index:0
// Public Visibility=Default Availability=Available
// [4] int primaryScreen() const

/*

*/
impl /*struct*/ QDesktopWidget {
  pub fn primaryScreen_0<RetType, T: QDesktopWidget_primaryScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.primaryScreen_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_primaryScreen_0<RetType> {
  fn primaryScreen_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_primaryScreen_0<i32> for () {
  fn primaryScreen_0(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget13primaryScreenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:68
// index:0
// Public Visibility=Default Availability=Available
// [4] int screenNumber(const QWidget *) const

/*
Returns the index of the screen that contains the largest part of widget, or -1 if the widget not on a screen.

See also primaryScreen.
*/
impl /*struct*/ QDesktopWidget {
  pub fn screenNumber_0<RetType, T: QDesktopWidget_screenNumber_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenNumber_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenNumber_0<RetType> {
  fn screenNumber_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenNumber_0<i32> for (usize) {
  fn screenNumber_0(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget12screenNumberEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:69
// index:1
// Public Visibility=Default Availability=Available
// [4] int screenNumber(const QPoint &) const

/*
Returns the index of the screen that contains the largest part of widget, or -1 if the widget not on a screen.

See also primaryScreen.
*/
impl /*struct*/ QDesktopWidget {
  pub fn screenNumber_1<RetType, T: QDesktopWidget_screenNumber_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenNumber_1(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenNumber_1<RetType> {
  fn screenNumber_1(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenNumber_1<i32> for (usize) {
  fn screenNumber_1(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget12screenNumberERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QWidget * screen(int)

/*
Returns a widget that represents the screen with index screen (a value of -1 means the default screen).

If the system uses a virtual desktop, the returned widget will have the geometry of the entire virtual desktop; i.e., bounding every screen.

See also primaryScreen, screenCount, and virtualDesktop.
*/
impl /*struct*/ QDesktopWidget {
  pub fn screen_0<RetType, T: QDesktopWidget_screen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screen_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screen_0<RetType> {
  fn screen_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screen_0<usize> for (i32) {
  fn screen_0(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QDesktopWidget6screenEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:73
// index:0
// Public Visibility=Default Availability=Available
// [16] const QRect screenGeometry(int) const

/*
Returns the geometry of the screen with index screen. The default screen is used if screen is -1.

See also screenNumber().
*/
impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry_0<RetType, T: QDesktopWidget_screenGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenGeometry_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenGeometry_0<RetType> {
  fn screenGeometry_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenGeometry_0<usize> for (i32) {
  fn screenGeometry_0(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget14screenGeometryEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:74
// index:1
// Public Visibility=Default Availability=Available
// [16] const QRect screenGeometry(const QWidget *) const

/*
Returns the geometry of the screen with index screen. The default screen is used if screen is -1.

See also screenNumber().
*/
impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry_1<RetType, T: QDesktopWidget_screenGeometry_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenGeometry_1(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenGeometry_1<RetType> {
  fn screenGeometry_1(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenGeometry_1<usize> for (usize) {
  fn screenGeometry_1(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget14screenGeometryEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:75
// index:2
// Public inline Visibility=Default Availability=Available
// [16] const QRect screenGeometry(const QPoint &) const

/*
Returns the geometry of the screen with index screen. The default screen is used if screen is -1.

See also screenNumber().
*/
impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry_2<RetType, T: QDesktopWidget_screenGeometry_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenGeometry_2(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenGeometry_2<RetType> {
  fn screenGeometry_2(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenGeometry_2<usize> for (usize) {
  fn screenGeometry_2(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget14screenGeometryERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:78
// index:0
// Public Visibility=Default Availability=Available
// [16] const QRect availableGeometry(int) const

/*
Returns the available geometry of the screen with index screen. What is available will be subrect of screenGeometry() based on what the platform decides is available (for example excludes the dock and menu bar on macOS, or the task bar on Windows). The default screen is used if screen is -1.

See also screenNumber(), screenGeometry(), and QScreen::availableGeometry().
*/
impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry_0<RetType, T: QDesktopWidget_availableGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableGeometry_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_availableGeometry_0<RetType> {
  fn availableGeometry_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_availableGeometry_0<usize> for (i32) {
  fn availableGeometry_0(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget17availableGeometryEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:79
// index:1
// Public Visibility=Default Availability=Available
// [16] const QRect availableGeometry(const QWidget *) const

/*
Returns the available geometry of the screen with index screen. What is available will be subrect of screenGeometry() based on what the platform decides is available (for example excludes the dock and menu bar on macOS, or the task bar on Windows). The default screen is used if screen is -1.

See also screenNumber(), screenGeometry(), and QScreen::availableGeometry().
*/
impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry_1<RetType, T: QDesktopWidget_availableGeometry_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableGeometry_1(self);
    // return 1;
  }
}
pub trait QDesktopWidget_availableGeometry_1<RetType> {
  fn availableGeometry_1(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_availableGeometry_1<usize> for (usize) {
  fn availableGeometry_1(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget17availableGeometryEPK7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:80
// index:2
// Public inline Visibility=Default Availability=Available
// [16] const QRect availableGeometry(const QPoint &) const

/*
Returns the available geometry of the screen with index screen. What is available will be subrect of screenGeometry() based on what the platform decides is available (for example excludes the dock and menu bar on macOS, or the task bar on Windows). The default screen is used if screen is -1.

See also screenNumber(), screenGeometry(), and QScreen::availableGeometry().
*/
impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry_2<RetType, T: QDesktopWidget_availableGeometry_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.availableGeometry_2(self);
    // return 1;
  }
}
pub trait QDesktopWidget_availableGeometry_2<RetType> {
  fn availableGeometry_2(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_availableGeometry_2<usize> for (usize) {
  fn availableGeometry_2(self , rsthis: & QDesktopWidget) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDesktopWidget17availableGeometryERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resized(int)

/*
This signal is emitted when the size of screen changes.
*/
impl /*struct*/ QDesktopWidget {
  pub fn resized_0<RetType, T: QDesktopWidget_resized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resized_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_resized_0<RetType> {
  fn resized_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_resized_0<(/*void*/)> for (i32) {
  fn resized_0(self , rsthis: & QDesktopWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDesktopWidget7resizedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void workAreaResized(int)

/*
This signal is emitted when the work area available on screen changes.
*/
impl /*struct*/ QDesktopWidget {
  pub fn workAreaResized_0<RetType, T: QDesktopWidget_workAreaResized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.workAreaResized_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_workAreaResized_0<RetType> {
  fn workAreaResized_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_workAreaResized_0<(/*void*/)> for (i32) {
  fn workAreaResized_0(self , rsthis: & QDesktopWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDesktopWidget15workAreaResizedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void screenCountChanged(int)

/*
This signal is emitted when the number of screens changes to newCount.

This function was introduced in  Qt 4.6.

Note: Notifier signal for property screenCount. 

See also screenCount.
*/
impl /*struct*/ QDesktopWidget {
  pub fn screenCountChanged_0<RetType, T: QDesktopWidget_screenCountChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenCountChanged_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_screenCountChanged_0<RetType> {
  fn screenCountChanged_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_screenCountChanged_0<(/*void*/)> for (i32) {
  fn screenCountChanged_0(self , rsthis: & QDesktopWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDesktopWidget18screenCountChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void primaryScreenChanged()

/*
This signal is emitted whenever the primary screen changes.

Note: This doesn't mean the QDesktopWidget::primaryScreen index will necessarily be different, but now it will refer to the new primary screen.

This function was introduced in  Qt 5.6.

Note: Notifier signal for property primaryScreen. 

See also primaryScreen and screenGeometry().
*/
impl /*struct*/ QDesktopWidget {
  pub fn primaryScreenChanged_0<RetType, T: QDesktopWidget_primaryScreenChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.primaryScreenChanged_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_primaryScreenChanged_0<RetType> {
  fn primaryScreenChanged_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_primaryScreenChanged_0<(/*void*/)> for () {
  fn primaryScreenChanged_0(self , rsthis: & QDesktopWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QDesktopWidget20primaryScreenChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qdesktopwidget.h:90
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Reimplemented from QWidget::resizeEvent().
*/
impl /*struct*/ QDesktopWidget {
  pub fn resizeEvent_0<RetType, T: QDesktopWidget_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QDesktopWidget_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QDesktopWidget) -> RetType;
}
impl<'a> /*trait*/ QDesktopWidget_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QDesktopWidget) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDesktopWidget11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
