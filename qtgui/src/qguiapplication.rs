

// mod ::gui::QGuiApplication
// package qtgui
// /usr/include/qt/QtGui/qguiapplication.h
// #include <qguiapplication.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 29
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QGuiApplication) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QGuiApplication)=16
pub struct QGuiApplication {
  qbase: QCoreApplication,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QGuiApplication_ITF interface {
//    qtcore.QCoreApplication_ITF
//    QGuiApplication_PTR() *QGuiApplication
//}
//func (ptr *QGuiApplication) QGuiApplication_PTR() *QGuiApplication { return ptr }

impl /*struct*/ QGuiApplication {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QGuiApplication {
    return QGuiApplication{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QGuiApplication {
//  type Target = QGuiApplicationBASE;
//
//  fn deref(&self) -> &QGuiApplicationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QGuiApplicationBASE> for QGuiApplication {
//  fn as_ref(& self) -> & QGuiApplicationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qguiapplication.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn metaObject_0<RetType, T: QGuiApplication_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QGuiApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGuiApplication10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QGuiApplication(int &, char **, int)

/*
Initializes the window system and constructs an application object with argc command line arguments in argv.

Warning: The data referred to by argc and argv must stay valid for the entire lifetime of the QGuiApplication object. In addition, argc must be greater than zero and argv must contain at least one valid character string.

The global qApp pointer refers to this application object. Only one application object should be created.

This application object must be constructed before any paint devices (including pixmaps, bitmaps etc.).

Note: argc and argv might be changed as Qt removes command line arguments that it recognizes.
*/
// QGuiApplication(int &, char **, int) ctx.fn_proto_cpp
impl /*struct*/ QGuiApplication {
  pub fn QGuiApplication_0<T: QGuiApplication_QGuiApplication_0>(value: T) -> QGuiApplication {
    let rsthis = value.QGuiApplication_0();
    return rsthis;
    // return 1;
  }
}

pub trait QGuiApplication_QGuiApplication_0 {
  fn QGuiApplication_0(self) -> QGuiApplication;
}
// QGuiApplication(int &, char **, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QGuiApplication_QGuiApplication_0 for (usize,usize,i32) {
  fn QGuiApplication_0(self) -> QGuiApplication {
    // unsafe{_ZN15QGuiApplicationC2ERiPPci()};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QGuiApplicationC2ERiPPci", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QGuiApplication{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:89
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QGuiApplication()

/*

*/
pub fn DeleteQGuiApplication(this :*mut QGuiApplication) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QGuiApplicationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qguiapplication.h:91
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setApplicationDisplayName(const QString &)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn setApplicationDisplayName_0<RetType, T: QGuiApplication_setApplicationDisplayName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setApplicationDisplayName_0();
    // return 1;
  }
}
pub trait QGuiApplication_setApplicationDisplayName_0<RetType> {
  fn setApplicationDisplayName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setApplicationDisplayName_0<(/*void*/)> for (usize) {
  fn setApplicationDisplayName_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication25setApplicationDisplayNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:92
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString applicationDisplayName()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn applicationDisplayName_0<RetType, T: QGuiApplication_applicationDisplayName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationDisplayName_0();
    // return 1;
  }
}
pub trait QGuiApplication_applicationDisplayName_0<RetType> {
  fn applicationDisplayName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_applicationDisplayName_0<usize> for () {
  fn applicationDisplayName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication22applicationDisplayNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDesktopFileName(const QString &)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn setDesktopFileName_0<RetType, T: QGuiApplication_setDesktopFileName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDesktopFileName_0();
    // return 1;
  }
}
pub trait QGuiApplication_setDesktopFileName_0<RetType> {
  fn setDesktopFileName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setDesktopFileName_0<(/*void*/)> for (usize) {
  fn setDesktopFileName_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication18setDesktopFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:95
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString desktopFileName()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn desktopFileName_0<RetType, T: QGuiApplication_desktopFileName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.desktopFileName_0();
    // return 1;
  }
}
pub trait QGuiApplication_desktopFileName_0<RetType> {
  fn desktopFileName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_desktopFileName_0<usize> for () {
  fn desktopFileName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication15desktopFileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:97
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QWindowList allWindows()

/*
Returns a list of all the windows in the application.

The list is empty if there are no windows.

See also topLevelWindows().
*/
impl /*struct*/ QGuiApplication {
  pub fn allWindows_0<RetType, T: QGuiApplication_allWindows_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.allWindows_0();
    // return 1;
  }
}
pub trait QGuiApplication_allWindows_0<RetType> {
  fn allWindows_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_allWindows_0<usize> for () {
  fn allWindows_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication10allWindowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:98
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QWindowList topLevelWindows()

/*
Returns a list of the top-level windows in the application.

See also allWindows().
*/
impl /*struct*/ QGuiApplication {
  pub fn topLevelWindows_0<RetType, T: QGuiApplication_topLevelWindows_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelWindows_0();
    // return 1;
  }
}
pub trait QGuiApplication_topLevelWindows_0<RetType> {
  fn topLevelWindows_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_topLevelWindows_0<usize> for () {
  fn topLevelWindows_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication15topLevelWindowsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:99
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWindow * topLevelAt(const QPoint &)

/*
Returns the top level window at the given position pos, if any.
*/
impl /*struct*/ QGuiApplication {
  pub fn topLevelAt_0<RetType, T: QGuiApplication_topLevelAt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelAt_0();
    // return 1;
  }
}
pub trait QGuiApplication_topLevelAt_0<RetType> {
  fn topLevelAt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_topLevelAt_0<usize> for (usize) {
  fn topLevelAt_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication10topLevelAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:101
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setWindowIcon(const QIcon &)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn setWindowIcon_0<RetType, T: QGuiApplication_setWindowIcon_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWindowIcon_0();
    // return 1;
  }
}
pub trait QGuiApplication_setWindowIcon_0<RetType> {
  fn setWindowIcon_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setWindowIcon_0<(/*void*/)> for (usize) {
  fn setWindowIcon_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication13setWindowIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:102
// index:0
// Public static Visibility=Default Availability=Available
// [8] QIcon windowIcon()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn windowIcon_0<RetType, T: QGuiApplication_windowIcon_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowIcon_0();
    // return 1;
  }
}
pub trait QGuiApplication_windowIcon_0<RetType> {
  fn windowIcon_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_windowIcon_0<usize> for () {
  fn windowIcon_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication10windowIconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:104
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString platformName()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn platformName_0<RetType, T: QGuiApplication_platformName_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.platformName_0();
    // return 1;
  }
}
pub trait QGuiApplication_platformName_0<RetType> {
  fn platformName_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_platformName_0<usize> for () {
  fn platformName_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication12platformNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:106
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWindow * modalWindow()

/*
Returns the most recently shown modal window. If no modal windows are visible, this function returns zero.

A modal window is a window which has its modality property set to Qt::WindowModal or Qt::ApplicationModal. A modal window must be closed before the user can continue with other parts of the program.

Modal window are organized in a stack. This function returns the modal window at the top of the stack.

See also Qt::WindowModality and QWindow::setModality().
*/
impl /*struct*/ QGuiApplication {
  pub fn modalWindow_0<RetType, T: QGuiApplication_modalWindow_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.modalWindow_0();
    // return 1;
  }
}
pub trait QGuiApplication_modalWindow_0<RetType> {
  fn modalWindow_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_modalWindow_0<usize> for () {
  fn modalWindow_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication11modalWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:108
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWindow * focusWindow()

/*
Returns the QWindow that receives events tied to focus, such as key events.
*/
impl /*struct*/ QGuiApplication {
  pub fn focusWindow_0<RetType, T: QGuiApplication_focusWindow_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusWindow_0();
    // return 1;
  }
}
pub trait QGuiApplication_focusWindow_0<RetType> {
  fn focusWindow_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_focusWindow_0<usize> for () {
  fn focusWindow_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication11focusWindowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:109
// index:0
// Public static Visibility=Default Availability=Available
// [8] QObject * focusObject()

/*
Returns the QObject in currently active window that will be final receiver of events tied to focus, such as key events.
*/
impl /*struct*/ QGuiApplication {
  pub fn focusObject_0<RetType, T: QGuiApplication_focusObject_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusObject_0();
    // return 1;
  }
}
pub trait QGuiApplication_focusObject_0<RetType> {
  fn focusObject_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_focusObject_0<usize> for () {
  fn focusObject_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication11focusObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:111
// index:0
// Public static Visibility=Default Availability=Available
// [8] QScreen * primaryScreen()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn primaryScreen_0<RetType, T: QGuiApplication_primaryScreen_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.primaryScreen_0();
    // return 1;
  }
}
pub trait QGuiApplication_primaryScreen_0<RetType> {
  fn primaryScreen_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_primaryScreen_0<usize> for () {
  fn primaryScreen_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication13primaryScreenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:113
// index:0
// Public static Visibility=Default Availability=Available
// [8] QScreen * screenAt(const QPoint &)

/*
Returns the screen at point, or nullptr if outside of any screen.

The point is in relation to the virtualGeometry() of each set of virtual siblings. If the point maps to more than one set of virtual siblings the first match is returned.

This function was introduced in  Qt 5.10.
*/
impl /*struct*/ QGuiApplication {
  pub fn screenAt_0<RetType, T: QGuiApplication_screenAt_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.screenAt_0();
    // return 1;
  }
}
pub trait QGuiApplication_screenAt_0<RetType> {
  fn screenAt_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_screenAt_0<usize> for (usize) {
  fn screenAt_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication8screenAtERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:115
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal devicePixelRatio() const

/*
Returns the highest screen device pixel ratio found on the system. This is the ratio between physical pixels and device-independent pixels.

Use this function only when you don't know which window you are targeting. If you do know the target window, use QWindow::devicePixelRatio() instead.

See also QWindow::devicePixelRatio().
*/
impl /*struct*/ QGuiApplication {
  pub fn devicePixelRatio_0<RetType, T: QGuiApplication_devicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_devicePixelRatio_0<RetType> {
  fn devicePixelRatio_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_devicePixelRatio_0<f64> for () {
  fn devicePixelRatio_0(self , rsthis: & QGuiApplication) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGuiApplication16devicePixelRatioEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:118
// index:0
// Public static Visibility=Default Availability=Available
// [8] QCursor * overrideCursor()

/*
Returns the active application override cursor.

This function returns 0 if no application cursor has been defined (i.e. the internal cursor stack is empty).

See also setOverrideCursor() and restoreOverrideCursor().
*/
impl /*struct*/ QGuiApplication {
  pub fn overrideCursor_0<RetType, T: QGuiApplication_overrideCursor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.overrideCursor_0();
    // return 1;
  }
}
pub trait QGuiApplication_overrideCursor_0<RetType> {
  fn overrideCursor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_overrideCursor_0<usize> for () {
  fn overrideCursor_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication14overrideCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:119
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setOverrideCursor(const QCursor &)

/*
Sets the application override cursor to cursor.

Application override cursors are intended for showing the user that the application is in a special state, for example during an operation that might take some time.

This cursor will be displayed in all the application's widgets until restoreOverrideCursor() or another setOverrideCursor() is called.

Application cursors are stored on an internal stack. setOverrideCursor() pushes the cursor onto the stack, and restoreOverrideCursor() pops the active cursor off the stack. changeOverrideCursor() changes the curently active application override cursor.

Every setOverrideCursor() must eventually be followed by a corresponding restoreOverrideCursor(), otherwise the stack will never be emptied.

Example:


  QApplication::setOverrideCursor(QCursor(Qt::WaitCursor));
  calculateHugeMandelbrot();              // lunch time...
  QApplication::restoreOverrideCursor();



See also overrideCursor(), restoreOverrideCursor(), changeOverrideCursor(), and QWidget::setCursor().
*/
impl /*struct*/ QGuiApplication {
  pub fn setOverrideCursor_0<RetType, T: QGuiApplication_setOverrideCursor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setOverrideCursor_0();
    // return 1;
  }
}
pub trait QGuiApplication_setOverrideCursor_0<RetType> {
  fn setOverrideCursor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setOverrideCursor_0<(/*void*/)> for (usize) {
  fn setOverrideCursor_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication17setOverrideCursorERK7QCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:120
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void changeOverrideCursor(const QCursor &)

/*
Changes the currently active application override cursor to cursor.

This function has no effect if setOverrideCursor() was not called.

See also setOverrideCursor(), overrideCursor(), restoreOverrideCursor(), and QWidget::setCursor().
*/
impl /*struct*/ QGuiApplication {
  pub fn changeOverrideCursor_0<RetType, T: QGuiApplication_changeOverrideCursor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.changeOverrideCursor_0();
    // return 1;
  }
}
pub trait QGuiApplication_changeOverrideCursor_0<RetType> {
  fn changeOverrideCursor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_changeOverrideCursor_0<(/*void*/)> for (usize) {
  fn changeOverrideCursor_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication20changeOverrideCursorERK7QCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:121
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void restoreOverrideCursor()

/*
Undoes the last setOverrideCursor().

If setOverrideCursor() has been called twice, calling restoreOverrideCursor() will activate the first cursor set. Calling this function a second time restores the original widgets' cursors.

See also setOverrideCursor() and overrideCursor().
*/
impl /*struct*/ QGuiApplication {
  pub fn restoreOverrideCursor_0<RetType, T: QGuiApplication_restoreOverrideCursor_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.restoreOverrideCursor_0();
    // return 1;
  }
}
pub trait QGuiApplication_restoreOverrideCursor_0<RetType> {
  fn restoreOverrideCursor_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_restoreOverrideCursor_0<(/*void*/)> for () {
  fn restoreOverrideCursor_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication21restoreOverrideCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:124
// index:0
// Public static Visibility=Default Availability=Available
// [16] QFont font()

/*
Returns the default application font.

See also setFont().
*/
impl /*struct*/ QGuiApplication {
  pub fn font_0<RetType, T: QGuiApplication_font_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_0();
    // return 1;
  }
}
pub trait QGuiApplication_font_0<RetType> {
  fn font_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_font_0<usize> for () {
  fn font_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication4fontEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:125
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setFont(const QFont &)

/*
Changes the default application font to font.

See also font().
*/
impl /*struct*/ QGuiApplication {
  pub fn setFont_0<RetType, T: QGuiApplication_setFont_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_0();
    // return 1;
  }
}
pub trait QGuiApplication_setFont_0<RetType> {
  fn setFont_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setFont_0<(/*void*/)> for (usize) {
  fn setFont_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication7setFontERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:128
// index:0
// Public static Visibility=Default Availability=Available
// [8] QClipboard * clipboard()

/*
Returns the object for interacting with the clipboard.
*/
impl /*struct*/ QGuiApplication {
  pub fn clipboard_0<RetType, T: QGuiApplication_clipboard_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.clipboard_0();
    // return 1;
  }
}
pub trait QGuiApplication_clipboard_0<RetType> {
  fn clipboard_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_clipboard_0<usize> for () {
  fn clipboard_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication9clipboardEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:131
// index:0
// Public static Visibility=Default Availability=Available
// [16] QPalette palette()

/*
Returns the default application palette.

See also setPalette().
*/
impl /*struct*/ QGuiApplication {
  pub fn palette_0<RetType, T: QGuiApplication_palette_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_0();
    // return 1;
  }
}
pub trait QGuiApplication_palette_0<RetType> {
  fn palette_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_palette_0<usize> for () {
  fn palette_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication7paletteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:132
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setPalette(const QPalette &)

/*
Changes the default application palette to pal.

See also palette().
*/
impl /*struct*/ QGuiApplication {
  pub fn setPalette_0<RetType, T: QGuiApplication_setPalette_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_0();
    // return 1;
  }
}
pub trait QGuiApplication_setPalette_0<RetType> {
  fn setPalette_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setPalette_0<(/*void*/)> for (usize) {
  fn setPalette_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication10setPaletteERK8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:134
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers keyboardModifiers()

/*
Returns the current state of the modifier keys on the keyboard. The current state is updated sychronously as the event queue is emptied of events that will spontaneously change the keyboard state (QEvent::KeyPress and QEvent::KeyRelease events).

It should be noted this may not reflect the actual keys held on the input device at the time of calling but rather the modifiers as last reported in one of the above events. If no keys are being held Qt::NoModifier is returned.

See also mouseButtons() and queryKeyboardModifiers().
*/
impl /*struct*/ QGuiApplication {
  pub fn keyboardModifiers_0<RetType, T: QGuiApplication_keyboardModifiers_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.keyboardModifiers_0();
    // return 1;
  }
}
pub trait QGuiApplication_keyboardModifiers_0<RetType> {
  fn keyboardModifiers_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_keyboardModifiers_0<i32> for () {
  fn keyboardModifiers_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication17keyboardModifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:135
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers queryKeyboardModifiers()

/*
Queries and returns the state of the modifier keys on the keyboard. Unlike keyboardModifiers, this method returns the actual keys held on the input device at the time of calling the method.

It does not rely on the keypress events having been received by this process, which makes it possible to check the modifiers while moving a window, for instance. Note that in most cases, you should use keyboardModifiers(), which is faster and more accurate since it contains the state of the modifiers as they were when the currently processed event was received.

See also keyboardModifiers().
*/
impl /*struct*/ QGuiApplication {
  pub fn queryKeyboardModifiers_0<RetType, T: QGuiApplication_queryKeyboardModifiers_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.queryKeyboardModifiers_0();
    // return 1;
  }
}
pub trait QGuiApplication_queryKeyboardModifiers_0<RetType> {
  fn queryKeyboardModifiers_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_queryKeyboardModifiers_0<i32> for () {
  fn queryKeyboardModifiers_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication22queryKeyboardModifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:136
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::MouseButtons mouseButtons()

/*
Returns the current state of the buttons on the mouse. The current state is updated syncronously as the event queue is emptied of events that will spontaneously change the mouse state (QEvent::MouseButtonPress and QEvent::MouseButtonRelease events).

It should be noted this may not reflect the actual buttons held on the input device at the time of calling but rather the mouse buttons as last reported in one of the above events. If no mouse buttons are being held Qt::NoButton is returned.

See also keyboardModifiers().
*/
impl /*struct*/ QGuiApplication {
  pub fn mouseButtons_0<RetType, T: QGuiApplication_mouseButtons_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.mouseButtons_0();
    // return 1;
  }
}
pub trait QGuiApplication_mouseButtons_0<RetType> {
  fn mouseButtons_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_mouseButtons_0<i32> for () {
  fn mouseButtons_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication12mouseButtonsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:138
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setLayoutDirection(Qt::LayoutDirection)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn setLayoutDirection_0<RetType, T: QGuiApplication_setLayoutDirection_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setLayoutDirection_0();
    // return 1;
  }
}
pub trait QGuiApplication_setLayoutDirection_0<RetType> {
  fn setLayoutDirection_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setLayoutDirection_0<(/*void*/)> for (i32) {
  fn setLayoutDirection_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication18setLayoutDirectionEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:139
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::LayoutDirection layoutDirection()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn layoutDirection_0<RetType, T: QGuiApplication_layoutDirection_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.layoutDirection_0();
    // return 1;
  }
}
pub trait QGuiApplication_layoutDirection_0<RetType> {
  fn layoutDirection_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_layoutDirection_0<i32> for () {
  fn layoutDirection_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication15layoutDirectionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:141
// index:0
// Public static inline Visibility=Default Availability=Available
// [1] bool isRightToLeft()

/*
Returns true if the application's layout direction is Qt::RightToLeft; otherwise returns false.

See also layoutDirection() and isLeftToRight().
*/
impl /*struct*/ QGuiApplication {
  pub fn isRightToLeft_0<RetType, T: QGuiApplication_isRightToLeft_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isRightToLeft_0();
    // return 1;
  }
}
pub trait QGuiApplication_isRightToLeft_0<RetType> {
  fn isRightToLeft_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_isRightToLeft_0<bool> for () {
  fn isRightToLeft_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication13isRightToLeftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:142
// index:0
// Public static inline Visibility=Default Availability=Available
// [1] bool isLeftToRight()

/*
Returns true if the application's layout direction is Qt::LeftToRight; otherwise returns false.

See also layoutDirection() and isRightToLeft().
*/
impl /*struct*/ QGuiApplication {
  pub fn isLeftToRight_0<RetType, T: QGuiApplication_isLeftToRight_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLeftToRight_0();
    // return 1;
  }
}
pub trait QGuiApplication_isLeftToRight_0<RetType> {
  fn isLeftToRight_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_isLeftToRight_0<bool> for () {
  fn isLeftToRight_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication13isLeftToRightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:144
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStyleHints * styleHints()

/*
Returns the application's style hints.

The style hints encapsulate a set of platform dependent properties such as double click intervals, full width selection and others.

The hints can be used to integrate tighter with the underlying platform.

See also QStyleHints.
*/
impl /*struct*/ QGuiApplication {
  pub fn styleHints_0<RetType, T: QGuiApplication_styleHints_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.styleHints_0();
    // return 1;
  }
}
pub trait QGuiApplication_styleHints_0<RetType> {
  fn styleHints_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_styleHints_0<usize> for () {
  fn styleHints_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication10styleHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:145
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDesktopSettingsAware(bool)

/*
Sets whether Qt should use the system's standard colors, fonts, etc., to on. By default, this is true.

This function must be called before creating the QGuiApplication object, like this:


  int main(int argc, char *argv[])
  {
      QApplication::setDesktopSettingsAware(false);
      QApplication app(argc, argv);
      ...
      return app.exec();
  }



See also desktopSettingsAware().
*/
impl /*struct*/ QGuiApplication {
  pub fn setDesktopSettingsAware_0<RetType, T: QGuiApplication_setDesktopSettingsAware_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDesktopSettingsAware_0();
    // return 1;
  }
}
pub trait QGuiApplication_setDesktopSettingsAware_0<RetType> {
  fn setDesktopSettingsAware_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setDesktopSettingsAware_0<(/*void*/)> for (bool) {
  fn setDesktopSettingsAware_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication23setDesktopSettingsAwareEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:146
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool desktopSettingsAware()

/*
Returns true if Qt is set to use the system's standard colors, fonts, etc.; otherwise returns false. The default is true.

See also setDesktopSettingsAware().
*/
impl /*struct*/ QGuiApplication {
  pub fn desktopSettingsAware_0<RetType, T: QGuiApplication_desktopSettingsAware_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.desktopSettingsAware_0();
    // return 1;
  }
}
pub trait QGuiApplication_desktopSettingsAware_0<RetType> {
  fn desktopSettingsAware_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_desktopSettingsAware_0<bool> for () {
  fn desktopSettingsAware_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication20desktopSettingsAwareEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:148
// index:0
// Public static Visibility=Default Availability=Available
// [8] QInputMethod * inputMethod()

/*
returns the input method.

The input method returns properties about the state and position of the virtual keyboard. It also provides information about the position of the current focused input element.

See also QInputMethod.
*/
impl /*struct*/ QGuiApplication {
  pub fn inputMethod_0<RetType, T: QGuiApplication_inputMethod_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.inputMethod_0();
    // return 1;
  }
}
pub trait QGuiApplication_inputMethod_0<RetType> {
  fn inputMethod_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_inputMethod_0<usize> for () {
  fn inputMethod_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication11inputMethodEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:152
// index:0
// Public static Visibility=Default Availability=Available
// [8] QFunctionPointer platformFunction(const QByteArray &)

/*
Returns a function pointer from the platformplugin matching function
*/
impl /*struct*/ QGuiApplication {
  pub fn platformFunction_0<RetType, T: QGuiApplication_platformFunction_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.platformFunction_0();
    // return 1;
  }
}
pub trait QGuiApplication_platformFunction_0<RetType> {
  fn platformFunction_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_platformFunction_0<usize> for (usize) {
  fn platformFunction_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication16platformFunctionERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:154
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setQuitOnLastWindowClosed(bool)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn setQuitOnLastWindowClosed_0<RetType, T: QGuiApplication_setQuitOnLastWindowClosed_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setQuitOnLastWindowClosed_0();
    // return 1;
  }
}
pub trait QGuiApplication_setQuitOnLastWindowClosed_0<RetType> {
  fn setQuitOnLastWindowClosed_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setQuitOnLastWindowClosed_0<(/*void*/)> for (bool) {
  fn setQuitOnLastWindowClosed_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication25setQuitOnLastWindowClosedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:155
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool quitOnLastWindowClosed()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn quitOnLastWindowClosed_0<RetType, T: QGuiApplication_quitOnLastWindowClosed_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.quitOnLastWindowClosed_0();
    // return 1;
  }
}
pub trait QGuiApplication_quitOnLastWindowClosed_0<RetType> {
  fn quitOnLastWindowClosed_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_quitOnLastWindowClosed_0<bool> for () {
  fn quitOnLastWindowClosed_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication22quitOnLastWindowClosedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:157
// index:0
// Public static Visibility=Default Availability=Available
// [4] Qt::ApplicationState applicationState()

/*
Returns the current state of the application.

You can react to application state changes to perform actions such as stopping/resuming CPU-intensive tasks, freeing/loading resources or saving/restoring application data.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QGuiApplication {
  pub fn applicationState_0<RetType, T: QGuiApplication_applicationState_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.applicationState_0();
    // return 1;
  }
}
pub trait QGuiApplication_applicationState_0<RetType> {
  fn applicationState_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_applicationState_0<i32> for () {
  fn applicationState_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication16applicationStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:159
// index:0
// Public static Visibility=Default Availability=Available
// [4] int exec()

/*
Enters the main event loop and waits until exit() is called, and then returns the value that was set to exit() (which is 0 if exit() is called via quit()).

It is necessary to call this function to start event handling. The main event loop receives events from the window system and dispatches these to the application widgets.

Generally, no user interaction can take place before calling exec().

To make your application perform idle processing, e.g., executing a special function whenever there are no pending events, use a QTimer with 0 timeout. More advanced idle processing schemes can be achieved using processEvents().

We recommend that you connect clean-up code to the aboutToQuit() signal, instead of putting it in your application's main() function. This is because, on some platforms, the QApplication::exec() call may not return.

See also quitOnLastWindowClosed, quit(), exit(), processEvents(), and QCoreApplication::exec().
*/
impl /*struct*/ QGuiApplication {
  pub fn exec_0<RetType, T: QGuiApplication_exec_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_0();
    // return 1;
  }
}
pub trait QGuiApplication_exec_0<RetType> {
  fn exec_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_exec_0<i32> for () {
  fn exec_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication4execEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:160
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool notify(QObject *, QEvent *)

/*
Reimplemented from QCoreApplication::notify().
*/
impl /*struct*/ QGuiApplication {
  pub fn notify_0<RetType, T: QGuiApplication_notify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notify_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_notify_0<RetType> {
  fn notify_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_notify_0<bool> for (usize,usize) {
  fn notify_0(self , rsthis: & QGuiApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication6notifyEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSessionRestored() const

/*
Returns true if the application has been restored from an earlier session; otherwise returns false.

See also sessionId(), commitDataRequest(), and saveStateRequest().
*/
impl /*struct*/ QGuiApplication {
  pub fn isSessionRestored_0<RetType, T: QGuiApplication_isSessionRestored_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSessionRestored_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_isSessionRestored_0<RetType> {
  fn isSessionRestored_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_isSessionRestored_0<bool> for () {
  fn isSessionRestored_0(self , rsthis: & QGuiApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGuiApplication17isSessionRestoredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:165
// index:0
// Public Visibility=Default Availability=Available
// [8] QString sessionId() const

/*
Returns the current session's identifier.

If the application has been restored from an earlier session, this identifier is the same as it was in that previous session. The session identifier is guaranteed to be unique both for different applications and for different instances of the same application.

See also isSessionRestored(), sessionKey(), commitDataRequest(), and saveStateRequest().
*/
impl /*struct*/ QGuiApplication {
  pub fn sessionId_0<RetType, T: QGuiApplication_sessionId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sessionId_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_sessionId_0<RetType> {
  fn sessionId_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_sessionId_0<usize> for () {
  fn sessionId_0(self , rsthis: & QGuiApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGuiApplication9sessionIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:166
// index:0
// Public Visibility=Default Availability=Available
// [8] QString sessionKey() const

/*
Returns the session key in the current session.

If the application has been restored from an earlier session, this key is the same as it was when the previous session ended.

The session key changes every time the session is saved. If the shutdown process is cancelled, another session key will be used when shutting down again.

See also isSessionRestored(), sessionId(), commitDataRequest(), and saveStateRequest().
*/
impl /*struct*/ QGuiApplication {
  pub fn sessionKey_0<RetType, T: QGuiApplication_sessionKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sessionKey_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_sessionKey_0<RetType> {
  fn sessionKey_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_sessionKey_0<usize> for () {
  fn sessionKey_0(self , rsthis: & QGuiApplication) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGuiApplication10sessionKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:167
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSavingSession() const

/*
Returns true if the application is currently saving the session; otherwise returns false.

This is true when commitDataRequest() and saveStateRequest() are emitted, but also when the windows are closed afterwards by session management.

This function was introduced in  Qt 5.0.

See also sessionId(), commitDataRequest(), and saveStateRequest().
*/
impl /*struct*/ QGuiApplication {
  pub fn isSavingSession_0<RetType, T: QGuiApplication_isSavingSession_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSavingSession_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_isSavingSession_0<RetType> {
  fn isSavingSession_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_isSavingSession_0<bool> for () {
  fn isSavingSession_0(self , rsthis: & QGuiApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QGuiApplication15isSavingSessionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:169
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isFallbackSessionManagementEnabled()

/*
Returns whether QGuiApplication will use fallback session management.

The default is true.

If this is true and the session manager allows user interaction, QGuiApplication will try to close toplevel windows after commitDataRequest() has been emitted. If a window cannot be closed, session shutdown will be canceled and the application will keep running.

Fallback session management only benefits applications that have an "are you sure you want to close this window?" feature or other logic that prevents closing a toplevel window depending on certain conditions, and that do nothing to explicitly implement session management. In applications that do implement session management using the proper session management API, fallback session management interferes and may break session management logic.

Warning: If all windows are closed due to fallback session management and quitOnLastWindowClosed() is true, the application will quit before it is explicitly instructed to quit through the platform's session management protocol. That violation of protocol may prevent the platform session manager from saving application state.

This function was introduced in  Qt 5.6.

See also setFallbackSessionManagementEnabled(), QSessionManager::allowsInteraction(), saveStateRequest(), commitDataRequest(), and Session Management.
*/
impl /*struct*/ QGuiApplication {
  pub fn isFallbackSessionManagementEnabled_0<RetType, T: QGuiApplication_isFallbackSessionManagementEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isFallbackSessionManagementEnabled_0();
    // return 1;
  }
}
pub trait QGuiApplication_isFallbackSessionManagementEnabled_0<RetType> {
  fn isFallbackSessionManagementEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_isFallbackSessionManagementEnabled_0<bool> for () {
  fn isFallbackSessionManagementEnabled_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication34isFallbackSessionManagementEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:170
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setFallbackSessionManagementEnabled(bool)

/*
Sets whether QGuiApplication will use fallback session management to enabled.

This function was introduced in  Qt 5.6.

See also isFallbackSessionManagementEnabled().
*/
impl /*struct*/ QGuiApplication {
  pub fn setFallbackSessionManagementEnabled_0<RetType, T: QGuiApplication_setFallbackSessionManagementEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFallbackSessionManagementEnabled_0();
    // return 1;
  }
}
pub trait QGuiApplication_setFallbackSessionManagementEnabled_0<RetType> {
  fn setFallbackSessionManagementEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_setFallbackSessionManagementEnabled_0<(/*void*/)> for (bool) {
  fn setFallbackSessionManagementEnabled_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication35setFallbackSessionManagementEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:173
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void sync()

/*
Function that can be used to sync Qt state with the Window Systems state.

This function will first empty Qts events by calling QCoreApplication::processEvents(), then the platform plugin will sync up with the windowsystem, and finally Qts events will be delived by another call to QCoreApplication::processEvents();

This function is timeconsuming and its use is discouraged.

This function was introduced in  Qt 5.2.
*/
impl /*struct*/ QGuiApplication {
  pub fn sync_0<RetType, T: QGuiApplication_sync_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sync_0();
    // return 1;
  }
}
pub trait QGuiApplication_sync_0<RetType> {
  fn sync_0(self ) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_sync_0<(/*void*/)> for () {
  fn sync_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication4syncEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:175
// index:0
// Public Visibility=Default Availability=Available
// [-2] void fontDatabaseChanged()

/*
This signal is emitted when application fonts are loaded or removed.

See also QFontDatabase::addApplicationFont(), QFontDatabase::addApplicationFontFromData(), QFontDatabase::removeAllApplicationFonts(), and QFontDatabase::removeApplicationFont().
*/
impl /*struct*/ QGuiApplication {
  pub fn fontDatabaseChanged_0<RetType, T: QGuiApplication_fontDatabaseChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fontDatabaseChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_fontDatabaseChanged_0<RetType> {
  fn fontDatabaseChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_fontDatabaseChanged_0<(/*void*/)> for () {
  fn fontDatabaseChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication19fontDatabaseChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void screenAdded(QScreen *)

/*
This signal is emitted whenever a new screen screen has been added to the system.

See also screens(), primaryScreen, and screenRemoved().
*/
impl /*struct*/ QGuiApplication {
  pub fn screenAdded_0<RetType, T: QGuiApplication_screenAdded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenAdded_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_screenAdded_0<RetType> {
  fn screenAdded_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_screenAdded_0<(/*void*/)> for (usize) {
  fn screenAdded_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication11screenAddedEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void screenRemoved(QScreen *)

/*
This signal is emitted whenever a screen is removed from the system. It provides an opportunity to manage the windows on the screen before Qt falls back to moving them to the primary screen.

This function was introduced in  Qt 5.4.

See also screens(), screenAdded(), QObject::destroyed(), and QWindow::setScreen().
*/
impl /*struct*/ QGuiApplication {
  pub fn screenRemoved_0<RetType, T: QGuiApplication_screenRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenRemoved_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_screenRemoved_0<RetType> {
  fn screenRemoved_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_screenRemoved_0<(/*void*/)> for (usize) {
  fn screenRemoved_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication13screenRemovedEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void primaryScreenChanged(QScreen *)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn primaryScreenChanged_0<RetType, T: QGuiApplication_primaryScreenChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.primaryScreenChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_primaryScreenChanged_0<RetType> {
  fn primaryScreenChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_primaryScreenChanged_0<(/*void*/)> for (usize) {
  fn primaryScreenChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication20primaryScreenChangedEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lastWindowClosed()

/*
This signal is emitted from exec() when the last visible primary window (i.e. window with no parent) is closed.

By default, QGuiApplication quits after this signal is emitted. This feature can be turned off by setting quitOnLastWindowClosed to false.

See also QWindow::close() and QWindow::isTopLevel().
*/
impl /*struct*/ QGuiApplication {
  pub fn lastWindowClosed_0<RetType, T: QGuiApplication_lastWindowClosed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastWindowClosed_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_lastWindowClosed_0<RetType> {
  fn lastWindowClosed_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_lastWindowClosed_0<(/*void*/)> for () {
  fn lastWindowClosed_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication16lastWindowClosedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:180
// index:0
// Public Visibility=Default Availability=Available
// [-2] void focusObjectChanged(QObject *)

/*
This signal is emitted when final receiver of events tied to focus is changed. focusObject is the new receiver.

See also focusObject().
*/
impl /*struct*/ QGuiApplication {
  pub fn focusObjectChanged_0<RetType, T: QGuiApplication_focusObjectChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusObjectChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_focusObjectChanged_0<RetType> {
  fn focusObjectChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_focusObjectChanged_0<(/*void*/)> for (usize) {
  fn focusObjectChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication18focusObjectChangedEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void focusWindowChanged(QWindow *)

/*
This signal is emitted when the focused window changes. focusWindow is the new focused window.

See also focusWindow().
*/
impl /*struct*/ QGuiApplication {
  pub fn focusWindowChanged_0<RetType, T: QGuiApplication_focusWindowChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusWindowChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_focusWindowChanged_0<RetType> {
  fn focusWindowChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_focusWindowChanged_0<(/*void*/)> for (usize) {
  fn focusWindowChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication18focusWindowChangedEP7QWindow", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void applicationStateChanged(Qt::ApplicationState)

/*
This signal is emitted when the state of the application changes.

This function was introduced in  Qt 5.2.

See also applicationState().
*/
impl /*struct*/ QGuiApplication {
  pub fn applicationStateChanged_0<RetType, T: QGuiApplication_applicationStateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationStateChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_applicationStateChanged_0<RetType> {
  fn applicationStateChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_applicationStateChanged_0<(/*void*/)> for (i32) {
  fn applicationStateChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication23applicationStateChangedEN2Qt16ApplicationStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void layoutDirectionChanged(Qt::LayoutDirection)

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn layoutDirectionChanged_0<RetType, T: QGuiApplication_layoutDirectionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.layoutDirectionChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_layoutDirectionChanged_0<RetType> {
  fn layoutDirectionChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_layoutDirectionChanged_0<(/*void*/)> for (i32) {
  fn layoutDirectionChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication22layoutDirectionChangedEN2Qt15LayoutDirectionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:185
// index:0
// Public Visibility=Default Availability=Available
// [-2] void commitDataRequest(QSessionManager &)

/*
This signal deals with session management. It is emitted when the QSessionManager wants the application to commit all its data.

Usually this means saving all open files, after getting permission from the user. Furthermore you may want to provide a means by which the user can cancel the shutdown.

You should not exit the application within this signal. Instead, the session manager may or may not do this afterwards, depending on the context.

Warning: Within this signal, no user interaction is possible, unless you ask the manager for explicit permission. See QSessionManager::allowsInteraction() and QSessionManager::allowsErrorInteraction() for details and example usage.

Note: You should use Qt::DirectConnection when connecting to this signal.

This function was introduced in  Qt 4.2.

See also setFallbackSessionManagementEnabled(), isSessionRestored(), sessionId(), saveStateRequest(), and Session Management.
*/
impl /*struct*/ QGuiApplication {
  pub fn commitDataRequest_0<RetType, T: QGuiApplication_commitDataRequest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.commitDataRequest_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_commitDataRequest_0<RetType> {
  fn commitDataRequest_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_commitDataRequest_0<(/*void*/)> for (usize) {
  fn commitDataRequest_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication17commitDataRequestER15QSessionManager", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:186
// index:0
// Public Visibility=Default Availability=Available
// [-2] void saveStateRequest(QSessionManager &)

/*
This signal deals with session management. It is invoked when the session manager wants the application to preserve its state for a future session.

For example, a text editor would create a temporary file that includes the current contents of its edit buffers, the location of the cursor and other aspects of the current editing session.

You should never exit the application within this signal. Instead, the session manager may or may not do this afterwards, depending on the context. Futhermore, most session managers will very likely request a saved state immediately after the application has been started. This permits the session manager to learn about the application's restart policy.

Warning: Within this signal, no user interaction is possible, unless you ask the manager for explicit permission. See QSessionManager::allowsInteraction() and QSessionManager::allowsErrorInteraction() for details.

Note: You should use Qt::DirectConnection when connecting to this signal.

This function was introduced in  Qt 4.2.

See also isSessionRestored(), sessionId(), commitDataRequest(), and Session Management.
*/
impl /*struct*/ QGuiApplication {
  pub fn saveStateRequest_0<RetType, T: QGuiApplication_saveStateRequest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.saveStateRequest_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_saveStateRequest_0<RetType> {
  fn saveStateRequest_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_saveStateRequest_0<(/*void*/)> for (usize) {
  fn saveStateRequest_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication16saveStateRequestER15QSessionManager", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:188
// index:0
// Public Visibility=Default Availability=Available
// [-2] void paletteChanged(const QPalette &)

/*
This signal is emitted when the palette of the application changes.

This function was introduced in  Qt 5.4.

See also palette().
*/
impl /*struct*/ QGuiApplication {
  pub fn paletteChanged_0<RetType, T: QGuiApplication_paletteChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paletteChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_paletteChanged_0<RetType> {
  fn paletteChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_paletteChanged_0<(/*void*/)> for (usize) {
  fn paletteChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication14paletteChangedERK8QPalette", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void applicationDisplayNameChanged()

/*

*/
impl /*struct*/ QGuiApplication {
  pub fn applicationDisplayNameChanged_0<RetType, T: QGuiApplication_applicationDisplayNameChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationDisplayNameChanged_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_applicationDisplayNameChanged_0<RetType> {
  fn applicationDisplayNameChanged_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_applicationDisplayNameChanged_0<(/*void*/)> for () {
  fn applicationDisplayNameChanged_0(self , rsthis: & QGuiApplication) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QGuiApplication29applicationDisplayNameChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qguiapplication.h:192
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QGuiApplication {
  pub fn event_0<RetType, T: QGuiApplication_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QGuiApplication_event_0<RetType> {
  fn event_0(self , rsthis: & QGuiApplication) -> RetType;
}
impl<'a> /*trait*/ QGuiApplication_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QGuiApplication) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QGuiApplication5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
