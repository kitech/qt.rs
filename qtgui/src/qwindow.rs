

// mod ::gui::QWindow
// package qtgui
// /usr/include/qt/QtGui/qwindow.h
// #include <qwindow.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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

// void exposeEvent(QExposeEvent *)
// func (this *QWindow) InheritExposeEvent(f func(arg0 *QExposeEvent/*777 QExposeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "exposeEvent", f)
// }

// void resizeEvent(QResizeEvent *)
// func (this *QWindow) InheritResizeEvent(f func(arg0 *QResizeEvent/*777 QResizeEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "resizeEvent", f)
// }

// void moveEvent(QMoveEvent *)
// func (this *QWindow) InheritMoveEvent(f func(arg0 *QMoveEvent/*777 QMoveEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "moveEvent", f)
// }

// void focusInEvent(QFocusEvent *)
// func (this *QWindow) InheritFocusInEvent(f func(arg0 *QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusInEvent", f)
// }

// void focusOutEvent(QFocusEvent *)
// func (this *QWindow) InheritFocusOutEvent(f func(arg0 *QFocusEvent/*777 QFocusEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "focusOutEvent", f)
// }

// void showEvent(QShowEvent *)
// func (this *QWindow) InheritShowEvent(f func(arg0 *QShowEvent/*777 QShowEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "showEvent", f)
// }

// void hideEvent(QHideEvent *)
// func (this *QWindow) InheritHideEvent(f func(arg0 *QHideEvent/*777 QHideEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "hideEvent", f)
// }

// bool event(QEvent *)
// func (this *QWindow) InheritEvent(f func(arg0 *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void keyPressEvent(QKeyEvent *)
// func (this *QWindow) InheritKeyPressEvent(f func(arg0 *QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyPressEvent", f)
// }

// void keyReleaseEvent(QKeyEvent *)
// func (this *QWindow) InheritKeyReleaseEvent(f func(arg0 *QKeyEvent/*777 QKeyEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "keyReleaseEvent", f)
// }

// void mousePressEvent(QMouseEvent *)
// func (this *QWindow) InheritMousePressEvent(f func(arg0 *QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mousePressEvent", f)
// }

// void mouseReleaseEvent(QMouseEvent *)
// func (this *QWindow) InheritMouseReleaseEvent(f func(arg0 *QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseReleaseEvent", f)
// }

// void mouseDoubleClickEvent(QMouseEvent *)
// func (this *QWindow) InheritMouseDoubleClickEvent(f func(arg0 *QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseDoubleClickEvent", f)
// }

// void mouseMoveEvent(QMouseEvent *)
// func (this *QWindow) InheritMouseMoveEvent(f func(arg0 *QMouseEvent/*777 QMouseEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "mouseMoveEvent", f)
// }

// void wheelEvent(QWheelEvent *)
// func (this *QWindow) InheritWheelEvent(f func(arg0 *QWheelEvent/*777 QWheelEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "wheelEvent", f)
// }

// void touchEvent(QTouchEvent *)
// func (this *QWindow) InheritTouchEvent(f func(arg0 *QTouchEvent/*777 QTouchEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "touchEvent", f)
// }

// void tabletEvent(QTabletEvent *)
// func (this *QWindow) InheritTabletEvent(f func(arg0 *QTabletEvent/*777 QTabletEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "tabletEvent", f)
// }

// bool nativeEvent(const QByteArray &, void *, long *)
// func (this *QWindow) InheritNativeEvent(f func(eventType *qtcore.QByteArray, message unsafe.Pointer /*666*/, result unsafe.Pointer /*666*/) bool) {
//  qtrt.SetAllInheritCallback(this, "nativeEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QWindow)=40
pub struct QWindow {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWindow_ITF interface {
//    qtcore.QObject_ITF
//    QSurface_ITF
//    QWindow_PTR() *QWindow
//}
//func (ptr *QWindow) QWindow_PTR() *QWindow { return ptr }

impl /*struct*/ QWindow {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWindow {
    return QWindow{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWindow {
//  type Target = QWindowBASE;
//
//  fn deref(&self) -> &QWindowBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWindowBASE> for QWindow {
//  fn as_ref(& self) -> & QWindowBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qwindow.h:97
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QWindow {
  pub fn metaObject_0<RetType, T: QWindow_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QWindow_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWindow(QScreen *)

/*
Creates a window as a top level on the targetScreen.

The window is not shown until setVisible(true), show(), or similar is called.

See also setScreen().
*/
// QWindow(QScreen *) ctx.fn_proto_cpp
impl /*struct*/ QWindow {
  pub fn QWindow_0<T: QWindow_QWindow_0>(value: T) -> QWindow {
    let rsthis = value.QWindow_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWindow_QWindow_0 {
  fn QWindow_0(self) -> QWindow;
}
// QWindow(QScreen *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWindow_QWindow_0 for (usize) {
  fn QWindow_0(self) -> QWindow {
    // unsafe{_ZN7QWindowC2EP7QScreen()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QWindowC2EP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWindow{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:145
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QWindow(QWindow *)

/*
Creates a window as a top level on the targetScreen.

The window is not shown until setVisible(true), show(), or similar is called.

See also setScreen().
*/
// QWindow(QWindow *) ctx.fn_proto_cpp
impl /*struct*/ QWindow {
  pub fn QWindow_1<T: QWindow_QWindow_1>(value: T) -> QWindow {
    let rsthis = value.QWindow_1();
    return rsthis;
    // return 1;
  }
}

pub trait QWindow_QWindow_1 {
  fn QWindow_1(self) -> QWindow;
}
// QWindow(QWindow *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWindow_QWindow_1 for (usize) {
  fn QWindow_1(self) -> QWindow {
    // unsafe{_ZN7QWindowC2EPS_()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QWindowC2EPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWindow{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:146
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWindow()

/*

*/
pub fn DeleteQWindow(this :*mut QWindow) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QWindowD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qwindow.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSurfaceType(QSurface::SurfaceType)

/*
Sets the surfaceType of the window.

Specifies whether the window is meant for raster rendering with QBackingStore, or OpenGL rendering with QOpenGLContext.

The surfaceType will be used when the native surface is created in the create() function. Calling this function after the native surface has been created requires calling destroy() and create() to release the old native surface and create a new one.

See also surfaceType(), QBackingStore, QOpenGLContext, create(), and destroy().
*/
impl /*struct*/ QWindow {
  pub fn setSurfaceType_0<RetType, T: QWindow_setSurfaceType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSurfaceType_0(self);
    // return 1;
  }
}
pub trait QWindow_setSurfaceType_0<RetType> {
  fn setSurfaceType_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setSurfaceType_0<(/*void*/)> for (i32) {
  fn setSurfaceType_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14setSurfaceTypeEN8QSurface11SurfaceTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:149
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QSurface::SurfaceType surfaceType() const

/*
Reimplemented from QSurface::surfaceType().

Returns the surface type of the window.

See also setSurfaceType().
*/
impl /*struct*/ QWindow {
  pub fn surfaceType_0<RetType, T: QWindow_surfaceType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.surfaceType_0(self);
    // return 1;
  }
}
pub trait QWindow_surfaceType_0<RetType> {
  fn surfaceType_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_surfaceType_0<i32> for () {
  fn surfaceType_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow11surfaceTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:151
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QWindow {
  pub fn isVisible_0<RetType, T: QWindow_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QWindow_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:153
// index:0
// Public Visibility=Default Availability=Available
// [4] QWindow::Visibility visibility() const

/*

*/
impl /*struct*/ QWindow {
  pub fn visibility_0<RetType, T: QWindow_visibility_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibility_0(self);
    // return 1;
  }
}
pub trait QWindow_visibility_0<RetType> {
  fn visibility_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_visibility_0<i32> for () {
  fn visibility_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow10visibilityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:154
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisibility(QWindow::Visibility)

/*

*/
impl /*struct*/ QWindow {
  pub fn setVisibility_0<RetType, T: QWindow_setVisibility_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisibility_0(self);
    // return 1;
  }
}
pub trait QWindow_setVisibility_0<RetType> {
  fn setVisibility_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setVisibility_0<(/*void*/)> for (i32) {
  fn setVisibility_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow13setVisibilityENS_10VisibilityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void create()

/*
Allocates the platform resources associated with the window.

It is at this point that the surface format set using setFormat() gets resolved into an actual native surface. However, the window remains hidden until setVisible() is called.

Note that it is not usually necessary to call this function directly, as it will be implicitly called by show(), setVisible(), and other functions that require access to the platform resources.

Call destroy() to free the platform resources if necessary.

See also destroy().
*/
impl /*struct*/ QWindow {
  pub fn create_0<RetType, T: QWindow_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QWindow_create_0<RetType> {
  fn create_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_create_0<(/*void*/)> for () {
  fn create_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow6createEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:158
// index:0
// Public Visibility=Default Availability=Available
// [8] WId winId() const

/*
Returns the window's platform id.

For platforms where this id might be useful, the value returned will uniquely represent the window inside the corresponding screen.

See also screen().
*/
impl /*struct*/ QWindow {
  pub fn winId_0<RetType, T: QWindow_winId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.winId_0(self);
    // return 1;
  }
}
pub trait QWindow_winId_0<RetType> {
  fn winId_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_winId_0<u64> for () {
  fn winId_0(self , rsthis: & QWindow) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow5winIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:160
// index:0
// Public Visibility=Default Availability=Available
// [8] QWindow * parent(QWindow::AncestorMode) const

/*
Returns the parent window, if any.

If mode is IncludeTransients, then the transient parent is returned if there is no parent.

A window without a parent is known as a top level window.

This function was introduced in  Qt 5.9.

See also setParent().
*/
impl /*struct*/ QWindow {
  pub fn parent_0<RetType, T: QWindow_parent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_0(self);
    // return 1;
  }
}
pub trait QWindow_parent_0<RetType> {
  fn parent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_parent_0<usize> for (i32) {
  fn parent_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow6parentENS_12AncestorModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:161
// index:1
// Public Visibility=Default Availability=Available
// [8] QWindow * parent() const

/*
Returns the parent window, if any.

If mode is IncludeTransients, then the transient parent is returned if there is no parent.

A window without a parent is known as a top level window.

This function was introduced in  Qt 5.9.

See also setParent().
*/
impl /*struct*/ QWindow {
  pub fn parent_1<RetType, T: QWindow_parent_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parent_1(self);
    // return 1;
  }
}
pub trait QWindow_parent_1<RetType> {
  fn parent_1(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_parent_1<usize> for () {
  fn parent_1(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow6parentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setParent(QWindow *)

/*
Sets the parent Window. This will lead to the windowing system managing the clip of the window, so it will be clipped to the parent window.

Setting parent to be 0 will make the window become a top level window.

If parent is a window created by fromWinId(), then the current window will be embedded inside parent, if the platform supports it.

See also parent().
*/
impl /*struct*/ QWindow {
  pub fn setParent_0<RetType, T: QWindow_setParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setParent_0(self);
    // return 1;
  }
}
pub trait QWindow_setParent_0<RetType> {
  fn setParent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setParent_0<(/*void*/)> for (usize) {
  fn setParent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9setParentEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isTopLevel() const

/*
Returns whether the window is top level, i.e. has no parent window.
*/
impl /*struct*/ QWindow {
  pub fn isTopLevel_0<RetType, T: QWindow_isTopLevel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isTopLevel_0(self);
    // return 1;
  }
}
pub trait QWindow_isTopLevel_0<RetType> {
  fn isTopLevel_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_isTopLevel_0<bool> for () {
  fn isTopLevel_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow10isTopLevelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:166
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isModal() const

/*
Returns whether the window is modal.

A modal window prevents other windows from getting any input.

See also QWindow::modality.
*/
impl /*struct*/ QWindow {
  pub fn isModal_0<RetType, T: QWindow_isModal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isModal_0(self);
    // return 1;
  }
}
pub trait QWindow_isModal_0<RetType> {
  fn isModal_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_isModal_0<bool> for () {
  fn isModal_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow7isModalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:167
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowModality modality() const

/*

*/
impl /*struct*/ QWindow {
  pub fn modality_0<RetType, T: QWindow_modality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modality_0(self);
    // return 1;
  }
}
pub trait QWindow_modality_0<RetType> {
  fn modality_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_modality_0<i32> for () {
  fn modality_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow8modalityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:168
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setModality(Qt::WindowModality)

/*

*/
impl /*struct*/ QWindow {
  pub fn setModality_0<RetType, T: QWindow_setModality_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModality_0(self);
    // return 1;
  }
}
pub trait QWindow_setModality_0<RetType> {
  fn setModality_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setModality_0<(/*void*/)> for (i32) {
  fn setModality_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setModalityEN2Qt14WindowModalityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:170
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QSurfaceFormat &)

/*
Sets the window's surface format.

The format determines properties such as color depth, alpha, depth and stencil buffer size, etc. For example, to give a window a transparent background (provided that the window system supports compositing, and provided that other content in the window does not make it opaque again):


  QSurfaceFormat format;
  format.setAlphaBufferSize(8);
  window.setFormat(format);



The surface format will be resolved in the create() function. Calling this function after create() has been called will not re-resolve the surface format of the native surface.

When the format is not explicitly set via this function, the format returned by QSurfaceFormat::defaultFormat() will be used. This means that when having multiple windows, individual calls to this function can be replaced by one single call to QSurfaceFormat::setDefaultFormat() before creating the first window.

See also format(), create(), destroy(), and QSurfaceFormat::setDefaultFormat().
*/
impl /*struct*/ QWindow {
  pub fn setFormat_0<RetType, T: QWindow_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QWindow_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9setFormatERK14QSurfaceFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:171
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSurfaceFormat format() const

/*
Reimplemented from QSurface::format().

Returns the actual format of this window.

After the window has been created, this function will return the actual surface format of the window. It might differ from the requested format if the requested format could not be fulfilled by the platform. It might also be a superset, for example certain buffer sizes may be larger than requested.

Note: Depending on the platform, certain values in this surface format may still contain the requested values, that is, the values that have been passed to setFormat(). Typical examples are the OpenGL version, profile and options. These may not get updated during create() since these are context specific and a single window may be used together with multiple contexts over its lifetime. Use the QOpenGLContext's format() instead to query such values.

See also setFormat(), create(), requestedFormat(), and QOpenGLContext::format().
*/
impl /*struct*/ QWindow {
  pub fn format_0<RetType, T: QWindow_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QWindow_format_0<RetType> {
  fn format_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_format_0<usize> for () {
  fn format_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:172
// index:0
// Public Visibility=Default Availability=Available
// [8] QSurfaceFormat requestedFormat() const

/*
Returns the requested surface format of this window.

If the requested format was not supported by the platform implementation, the requestedFormat will differ from the actual window format.

This is the value set with setFormat().

See also setFormat() and format().
*/
impl /*struct*/ QWindow {
  pub fn requestedFormat_0<RetType, T: QWindow_requestedFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestedFormat_0(self);
    // return 1;
  }
}
pub trait QWindow_requestedFormat_0<RetType> {
  fn requestedFormat_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_requestedFormat_0<usize> for () {
  fn requestedFormat_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow15requestedFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:174
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlags(Qt::WindowFlags)

/*

*/
impl /*struct*/ QWindow {
  pub fn setFlags_0<RetType, T: QWindow_setFlags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlags_0(self);
    // return 1;
  }
}
pub trait QWindow_setFlags_0<RetType> {
  fn setFlags_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setFlags_0<(/*void*/)> for (i32) {
  fn setFlags_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow8setFlagsE6QFlagsIN2Qt10WindowTypeEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:175
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowFlags flags() const

/*

*/
impl /*struct*/ QWindow {
  pub fn flags_0<RetType, T: QWindow_flags_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flags_0(self);
    // return 1;
  }
}
pub trait QWindow_flags_0<RetType> {
  fn flags_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_flags_0<i32> for () {
  fn flags_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow5flagsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:176
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFlag(Qt::WindowType, bool)

/*
Sets the window flag flag on this window if on is true; otherwise clears the flag.

This function was introduced in  Qt 5.9.

See also setFlags(), flags(), and type().
*/
impl /*struct*/ QWindow {
  pub fn setFlag_0<RetType, T: QWindow_setFlag_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFlag_0(self);
    // return 1;
  }
}
pub trait QWindow_setFlag_0<RetType> {
  fn setFlag_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setFlag_0<(/*void*/)> for (i32,bool) {
  fn setFlag_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow7setFlagEN2Qt10WindowTypeEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:177
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowType type() const

/*
Returns the type of the window.

This returns the part of the window flags that represents whether the window is a dialog, tooltip, popup, regular window, etc.

See also flags() and setFlags().
*/
impl /*struct*/ QWindow {
  pub fn type__0<RetType, T: QWindow_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QWindow_type__0<RetType> {
  fn type__0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_type__0<i32> for () {
  fn type__0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:179
// index:0
// Public Visibility=Default Availability=Available
// [8] QString title() const

/*

*/
impl /*struct*/ QWindow {
  pub fn title_0<RetType, T: QWindow_title_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.title_0(self);
    // return 1;
  }
}
pub trait QWindow_title_0<RetType> {
  fn title_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_title_0<usize> for () {
  fn title_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow5titleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:181
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOpacity(qreal)

/*

*/
impl /*struct*/ QWindow {
  pub fn setOpacity_0<RetType, T: QWindow_setOpacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOpacity_0(self);
    // return 1;
  }
}
pub trait QWindow_setOpacity_0<RetType> {
  fn setOpacity_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setOpacity_0<(/*void*/)> for (f64) {
  fn setOpacity_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow10setOpacityEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal opacity() const

/*

*/
impl /*struct*/ QWindow {
  pub fn opacity_0<RetType, T: QWindow_opacity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacity_0(self);
    // return 1;
  }
}
pub trait QWindow_opacity_0<RetType> {
  fn opacity_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_opacity_0<f64> for () {
  fn opacity_0(self , rsthis: & QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow7opacityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:184
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMask(const QRegion &)

/*
Sets the mask of the window.

The mask is a hint to the windowing system that the application does not want to receive mouse or touch input outside the given region.

The window manager may or may not choose to display any areas of the window not included in the mask, thus it is the application's responsibility to clear to transparent the areas that are not part of the mask.

See also mask().
*/
impl /*struct*/ QWindow {
  pub fn setMask_0<RetType, T: QWindow_setMask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMask_0(self);
    // return 1;
  }
}
pub trait QWindow_setMask_0<RetType> {
  fn setMask_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMask_0<(/*void*/)> for (usize) {
  fn setMask_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow7setMaskERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:185
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegion mask() const

/*
Returns the mask set on the window.

The mask is a hint to the windowing system that the application does not want to receive mouse or touch input outside the given region.

See also setMask().
*/
impl /*struct*/ QWindow {
  pub fn mask_0<RetType, T: QWindow_mask_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mask_0(self);
    // return 1;
  }
}
pub trait QWindow_mask_0<RetType> {
  fn mask_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mask_0<usize> for () {
  fn mask_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow4maskEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:187
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if the window should appear active from a style perspective.

This is the case for the window that has input focus as well as windows that are in the same parent / transient parent chain as the focus window.

To get the window that currently has focus, use QGuiApplication::focusWindow().

Note: Getter function for property active.
*/
impl /*struct*/ QWindow {
  pub fn isActive_0<RetType, T: QWindow_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QWindow_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:189
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reportContentOrientationChange(Qt::ScreenOrientation)

/*

*/
impl /*struct*/ QWindow {
  pub fn reportContentOrientationChange_0<RetType, T: QWindow_reportContentOrientationChange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reportContentOrientationChange_0(self);
    // return 1;
  }
}
pub trait QWindow_reportContentOrientationChange_0<RetType> {
  fn reportContentOrientationChange_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_reportContentOrientationChange_0<(/*void*/)> for (i32) {
  fn reportContentOrientationChange_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow30reportContentOrientationChangeEN2Qt17ScreenOrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:190
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ScreenOrientation contentOrientation() const

/*

*/
impl /*struct*/ QWindow {
  pub fn contentOrientation_0<RetType, T: QWindow_contentOrientation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentOrientation_0(self);
    // return 1;
  }
}
pub trait QWindow_contentOrientation_0<RetType> {
  fn contentOrientation_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_contentOrientation_0<i32> for () {
  fn contentOrientation_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow18contentOrientationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:192
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal devicePixelRatio() const

/*
Returns the ratio between physical pixels and device-independent pixels for the window. This value is dependent on the screen the window is on, and may change when the window is moved.

Common values are 1.0 on normal displays and 2.0 on Apple "retina" displays.

Note: For windows not backed by a platform window, meaning that create() was not called, the function will fall back to the associated QScreen's device pixel ratio.

See also QScreen::devicePixelRatio().
*/
impl /*struct*/ QWindow {
  pub fn devicePixelRatio_0<RetType, T: QWindow_devicePixelRatio_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.devicePixelRatio_0(self);
    // return 1;
  }
}
pub trait QWindow_devicePixelRatio_0<RetType> {
  fn devicePixelRatio_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_devicePixelRatio_0<f64> for () {
  fn devicePixelRatio_0(self , rsthis: & QWindow) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow16devicePixelRatioEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:194
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowState windowState() const

/*
the screen-occupation state of the window

See also setWindowState() and windowStates().
*/
impl /*struct*/ QWindow {
  pub fn windowState_0<RetType, T: QWindow_windowState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowState_0(self);
    // return 1;
  }
}
pub trait QWindow_windowState_0<RetType> {
  fn windowState_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_windowState_0<i32> for () {
  fn windowState_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow11windowStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:195
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::WindowStates windowStates() const

/*
the screen-occupation state of the window

The window can be in a combination of several states. For example, if the window is both minimized and maximized, the window will appear minimized, but clicking on the task bar entry will restore it to the maximized state.

This function was introduced in  Qt 5.10.

See also setWindowStates().
*/
impl /*struct*/ QWindow {
  pub fn windowStates_0<RetType, T: QWindow_windowStates_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowStates_0(self);
    // return 1;
  }
}
pub trait QWindow_windowStates_0<RetType> {
  fn windowStates_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_windowStates_0<i32> for () {
  fn windowStates_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow12windowStatesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:196
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowState(Qt::WindowState)

/*
set the screen-occupation state of the window

The window state represents whether the window appears in the windowing system as maximized, minimized, fullscreen, or normal.

The enum value Qt::WindowActive is not an accepted parameter.

See also windowState(), showNormal(), showFullScreen(), showMinimized(), showMaximized(), and setWindowStates().
*/
impl /*struct*/ QWindow {
  pub fn setWindowState_0<RetType, T: QWindow_setWindowState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowState_0(self);
    // return 1;
  }
}
pub trait QWindow_setWindowState_0<RetType> {
  fn setWindowState_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setWindowState_0<(/*void*/)> for (i32) {
  fn setWindowState_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14setWindowStateEN2Qt11WindowStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:197
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWindowStates(Qt::WindowStates)

/*
set the screen-occupation state of the window

The window state represents whether the window appears in the windowing system as maximized, minimized and/or fullscreen.

The window can be in a combination of several states. For example, if the window is both minimized and maximized, the window will appear minimized, but clicking on the task bar entry will restore it to the maximized state.

The enum value Qt::WindowActive should not be set.

This function was introduced in  Qt 5.10.

See also windowStates(), showNormal(), showFullScreen(), showMinimized(), and showMaximized().
*/
impl /*struct*/ QWindow {
  pub fn setWindowStates_0<RetType, T: QWindow_setWindowStates_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindowStates_0(self);
    // return 1;
  }
}
pub trait QWindow_setWindowStates_0<RetType> {
  fn setWindowStates_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setWindowStates_0<(/*void*/)> for (i32) {
  fn setWindowStates_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow15setWindowStatesE6QFlagsIN2Qt11WindowStateEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:199
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransientParent(QWindow *)

/*
Sets the transient parent

This is a hint to the window manager that this window is a dialog or pop-up on behalf of the given window.

In order to cause the window to be centered above its transient parent by default, depending on the window manager, it may also be necessary to call setFlags() with a suitable Qt::WindowType (such as Qt::Dialog).

See also transientParent() and parent().
*/
impl /*struct*/ QWindow {
  pub fn setTransientParent_0<RetType, T: QWindow_setTransientParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransientParent_0(self);
    // return 1;
  }
}
pub trait QWindow_setTransientParent_0<RetType> {
  fn setTransientParent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setTransientParent_0<(/*void*/)> for (usize) {
  fn setTransientParent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow18setTransientParentEPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QWindow * transientParent() const

/*
Returns the transient parent of the window.

See also setTransientParent() and parent().
*/
impl /*struct*/ QWindow {
  pub fn transientParent_0<RetType, T: QWindow_transientParent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transientParent_0(self);
    // return 1;
  }
}
pub trait QWindow_transientParent_0<RetType> {
  fn transientParent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_transientParent_0<usize> for () {
  fn transientParent_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow15transientParentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:202
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAncestorOf(const QWindow *, QWindow::AncestorMode) const

/*
Returns true if the window is an ancestor of the given child. If mode is IncludeTransients, then transient parents are also considered ancestors.
*/
impl /*struct*/ QWindow {
  pub fn isAncestorOf_0<RetType, T: QWindow_isAncestorOf_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAncestorOf_0(self);
    // return 1;
  }
}
pub trait QWindow_isAncestorOf_0<RetType> {
  fn isAncestorOf_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_isAncestorOf_0<bool> for (usize,i32) {
  fn isAncestorOf_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow12isAncestorOfEPKS_NS_12AncestorModeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:204
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isExposed() const

/*
Returns if this window is exposed in the windowing system.

When the window is not exposed, it is shown by the application but it is still not showing in the windowing system, so the application should minimize rendering and other graphical activities.

An exposeEvent() is sent every time this value changes.

See also exposeEvent().
*/
impl /*struct*/ QWindow {
  pub fn isExposed_0<RetType, T: QWindow_isExposed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isExposed_0(self);
    // return 1;
  }
}
pub trait QWindow_isExposed_0<RetType> {
  fn isExposed_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_isExposed_0<bool> for () {
  fn isExposed_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow9isExposedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:206
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int minimumWidth() const

/*

*/
impl /*struct*/ QWindow {
  pub fn minimumWidth_0<RetType, T: QWindow_minimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth_0(self);
    // return 1;
  }
}
pub trait QWindow_minimumWidth_0<RetType> {
  fn minimumWidth_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_minimumWidth_0<i32> for () {
  fn minimumWidth_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow12minimumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:207
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int minimumHeight() const

/*

*/
impl /*struct*/ QWindow {
  pub fn minimumHeight_0<RetType, T: QWindow_minimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight_0(self);
    // return 1;
  }
}
pub trait QWindow_minimumHeight_0<RetType> {
  fn minimumHeight_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_minimumHeight_0<i32> for () {
  fn minimumHeight_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow13minimumHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:208
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int maximumWidth() const

/*

*/
impl /*struct*/ QWindow {
  pub fn maximumWidth_0<RetType, T: QWindow_maximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth_0(self);
    // return 1;
  }
}
pub trait QWindow_maximumWidth_0<RetType> {
  fn maximumWidth_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_maximumWidth_0<i32> for () {
  fn maximumWidth_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow12maximumWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:209
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int maximumHeight() const

/*

*/
impl /*struct*/ QWindow {
  pub fn maximumHeight_0<RetType, T: QWindow_maximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight_0(self);
    // return 1;
  }
}
pub trait QWindow_maximumHeight_0<RetType> {
  fn maximumHeight_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_maximumHeight_0<i32> for () {
  fn maximumHeight_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow13maximumHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:211
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize minimumSize() const

/*
Returns the minimum size of the window.

See also setMinimumSize().
*/
impl /*struct*/ QWindow {
  pub fn minimumSize_0<RetType, T: QWindow_minimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumSize_0(self);
    // return 1;
  }
}
pub trait QWindow_minimumSize_0<RetType> {
  fn minimumSize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_minimumSize_0<usize> for () {
  fn minimumSize_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow11minimumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:212
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize maximumSize() const

/*
Returns the maximum size of the window.

See also setMaximumSize().
*/
impl /*struct*/ QWindow {
  pub fn maximumSize_0<RetType, T: QWindow_maximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumSize_0(self);
    // return 1;
  }
}
pub trait QWindow_maximumSize_0<RetType> {
  fn maximumSize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_maximumSize_0<usize> for () {
  fn maximumSize_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow11maximumSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:213
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize baseSize() const

/*
Returns the base size of the window.

See also setBaseSize().
*/
impl /*struct*/ QWindow {
  pub fn baseSize_0<RetType, T: QWindow_baseSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.baseSize_0(self);
    // return 1;
  }
}
pub trait QWindow_baseSize_0<RetType> {
  fn baseSize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_baseSize_0<usize> for () {
  fn baseSize_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow8baseSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:214
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize sizeIncrement() const

/*
Returns the size increment of the window.

See also setSizeIncrement().
*/
impl /*struct*/ QWindow {
  pub fn sizeIncrement_0<RetType, T: QWindow_sizeIncrement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sizeIncrement_0(self);
    // return 1;
  }
}
pub trait QWindow_sizeIncrement_0<RetType> {
  fn sizeIncrement_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_sizeIncrement_0<usize> for () {
  fn sizeIncrement_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow13sizeIncrementEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:216
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumSize(const QSize &)

/*
Sets the minimum size of the window.

This is a hint to the window manager to prevent resizing below the specified size.

See also setMaximumSize() and minimumSize().
*/
impl /*struct*/ QWindow {
  pub fn setMinimumSize_0<RetType, T: QWindow_setMinimumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize_0(self);
    // return 1;
  }
}
pub trait QWindow_setMinimumSize_0<RetType> {
  fn setMinimumSize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMinimumSize_0<(/*void*/)> for (usize) {
  fn setMinimumSize_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14setMinimumSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:217
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumSize(const QSize &)

/*
Sets the maximum size of the window.

This is a hint to the window manager to prevent resizing above the specified size.

See also setMinimumSize() and maximumSize().
*/
impl /*struct*/ QWindow {
  pub fn setMaximumSize_0<RetType, T: QWindow_setMaximumSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize_0(self);
    // return 1;
  }
}
pub trait QWindow_setMaximumSize_0<RetType> {
  fn setMaximumSize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMaximumSize_0<(/*void*/)> for (usize) {
  fn setMaximumSize_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14setMaximumSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:218
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBaseSize(const QSize &)

/*
Sets the base size of the window.

The base size is used to calculate a proper window size if the window defines sizeIncrement().

See also setMinimumSize(), setMaximumSize(), setSizeIncrement(), and baseSize().
*/
impl /*struct*/ QWindow {
  pub fn setBaseSize_0<RetType, T: QWindow_setBaseSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBaseSize_0(self);
    // return 1;
  }
}
pub trait QWindow_setBaseSize_0<RetType> {
  fn setBaseSize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setBaseSize_0<(/*void*/)> for (usize) {
  fn setBaseSize_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setBaseSizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:219
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSizeIncrement(const QSize &)

/*
Sets the size increment (size) of the window.

When the user resizes the window, the size will move in steps of sizeIncrement().width() pixels horizontally and sizeIncrement().height() pixels vertically, with baseSize() as the basis.

By default, this property contains a size with zero width and height.

The windowing system might not support size increments.

See also sizeIncrement(), setBaseSize(), setMinimumSize(), and setMaximumSize().
*/
impl /*struct*/ QWindow {
  pub fn setSizeIncrement_0<RetType, T: QWindow_setSizeIncrement_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSizeIncrement_0(self);
    // return 1;
  }
}
pub trait QWindow_setSizeIncrement_0<RetType> {
  fn setSizeIncrement_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setSizeIncrement_0<(/*void*/)> for (usize) {
  fn setSizeIncrement_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow16setSizeIncrementERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:221
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect geometry() const

/*
Returns the geometry of the window, excluding its window frame.

The geometry is in relation to the virtualGeometry() of its screen.

See also setGeometry(), frameMargins(), and frameGeometry().
*/
impl /*struct*/ QWindow {
  pub fn geometry_0<RetType, T: QWindow_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QWindow_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:223
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins frameMargins() const

/*
Returns the window frame margins surrounding the window.

See also geometry() and frameGeometry().
*/
impl /*struct*/ QWindow {
  pub fn frameMargins_0<RetType, T: QWindow_frameMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameMargins_0(self);
    // return 1;
  }
}
pub trait QWindow_frameMargins_0<RetType> {
  fn frameMargins_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_frameMargins_0<usize> for () {
  fn frameMargins_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow12frameMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:224
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect frameGeometry() const

/*
Returns the geometry of the window, including its window frame.

The geometry is in relation to the virtualGeometry() of its screen.

See also geometry() and frameMargins().
*/
impl /*struct*/ QWindow {
  pub fn frameGeometry_0<RetType, T: QWindow_frameGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.frameGeometry_0(self);
    // return 1;
  }
}
pub trait QWindow_frameGeometry_0<RetType> {
  fn frameGeometry_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_frameGeometry_0<usize> for () {
  fn frameGeometry_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow13frameGeometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:226
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint framePosition() const

/*
Returns the top left position of the window, including its window frame.

This returns the same value as frameGeometry().topLeft().

See also setFramePosition(), geometry(), and frameGeometry().
*/
impl /*struct*/ QWindow {
  pub fn framePosition_0<RetType, T: QWindow_framePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.framePosition_0(self);
    // return 1;
  }
}
pub trait QWindow_framePosition_0<RetType> {
  fn framePosition_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_framePosition_0<usize> for () {
  fn framePosition_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow13framePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:227
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFramePosition(const QPoint &)

/*
Sets the upper left position of the window (point) including its window frame.

The position is in relation to the virtualGeometry() of its screen.

See also framePosition(), setGeometry(), and frameGeometry().
*/
impl /*struct*/ QWindow {
  pub fn setFramePosition_0<RetType, T: QWindow_setFramePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFramePosition_0(self);
    // return 1;
  }
}
pub trait QWindow_setFramePosition_0<RetType> {
  fn setFramePosition_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setFramePosition_0<(/*void*/)> for (usize) {
  fn setFramePosition_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow16setFramePositionERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:229
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int width() const

/*

*/
impl /*struct*/ QWindow {
  pub fn width_0<RetType, T: QWindow_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QWindow_width_0<RetType> {
  fn width_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_width_0<i32> for () {
  fn width_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:230
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int height() const

/*

*/
impl /*struct*/ QWindow {
  pub fn height_0<RetType, T: QWindow_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QWindow_height_0<RetType> {
  fn height_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_height_0<i32> for () {
  fn height_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:231
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int x() const

/*

*/
impl /*struct*/ QWindow {
  pub fn x_0<RetType, T: QWindow_x_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.x_0(self);
    // return 1;
  }
}
pub trait QWindow_x_0<RetType> {
  fn x_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_x_0<i32> for () {
  fn x_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow1xEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:232
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int y() const

/*

*/
impl /*struct*/ QWindow {
  pub fn y_0<RetType, T: QWindow_y_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.y_0(self);
    // return 1;
  }
}
pub trait QWindow_y_0<RetType> {
  fn y_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_y_0<i32> for () {
  fn y_0(self , rsthis: & QWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow1yEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:234
// index:0
// Public inline virtual Visibility=Default Availability=Available
// [8] QSize size() const

/*
Reimplemented from QSurface::size().

Returns the size of the window excluding any window frame

See also resize().
*/
impl /*struct*/ QWindow {
  pub fn size_0<RetType, T: QWindow_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QWindow_size_0<RetType> {
  fn size_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_size_0<usize> for () {
  fn size_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:235
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QPoint position() const

/*
Returns the position of the window on the desktop excluding any window frame

See also setPosition().
*/
impl /*struct*/ QWindow {
  pub fn position_0<RetType, T: QWindow_position_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.position_0(self);
    // return 1;
  }
}
pub trait QWindow_position_0<RetType> {
  fn position_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_position_0<usize> for () {
  fn position_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow8positionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:237
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPosition(const QPoint &)

/*
set the position of the window on the desktop to pt

The position is in relation to the virtualGeometry() of its screen.

See also position().
*/
impl /*struct*/ QWindow {
  pub fn setPosition_0<RetType, T: QWindow_setPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_0(self);
    // return 1;
  }
}
pub trait QWindow_setPosition_0<RetType> {
  fn setPosition_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setPosition_0<(/*void*/)> for (usize) {
  fn setPosition_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setPositionERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:238
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setPosition(int, int)

/*
set the position of the window on the desktop to pt

The position is in relation to the virtualGeometry() of its screen.

See also position().
*/
impl /*struct*/ QWindow {
  pub fn setPosition_1<RetType, T: QWindow_setPosition_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPosition_1(self);
    // return 1;
  }
}
pub trait QWindow_setPosition_1<RetType> {
  fn setPosition_1(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setPosition_1<(/*void*/)> for (i32,i32) {
  fn setPosition_1(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setPositionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:240
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resize(const QSize &)

/*
set the size of the window, excluding any window frame, to newSize

See also size() and geometry().
*/
impl /*struct*/ QWindow {
  pub fn resize_0<RetType, T: QWindow_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QWindow_resize_0<RetType> {
  fn resize_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_resize_0<(/*void*/)> for (usize) {
  fn resize_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow6resizeERK5QSize", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:241
// index:1
// Public Visibility=Default Availability=Available
// [-2] void resize(int, int)

/*
set the size of the window, excluding any window frame, to newSize

See also size() and geometry().
*/
impl /*struct*/ QWindow {
  pub fn resize_1<RetType, T: QWindow_resize_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_1(self);
    // return 1;
  }
}
pub trait QWindow_resize_1<RetType> {
  fn resize_1(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_resize_1<(/*void*/)> for (i32,i32) {
  fn resize_1(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow6resizeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:243
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilePath(const QString &)

/*
set the file name this window is representing.

The windowing system might use filePath to display the path of the document this window is representing in the tile bar.

See also filePath().
*/
impl /*struct*/ QWindow {
  pub fn setFilePath_0<RetType, T: QWindow_setFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilePath_0(self);
    // return 1;
  }
}
pub trait QWindow_setFilePath_0<RetType> {
  fn setFilePath_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setFilePath_0<(/*void*/)> for (usize) {
  fn setFilePath_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setFilePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:244
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath() const

/*
the file name this window is representing.

See also setFilePath().
*/
impl /*struct*/ QWindow {
  pub fn filePath_0<RetType, T: QWindow_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QWindow_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_filePath_0<usize> for () {
  fn filePath_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow8filePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:246
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*
Sets the window's icon in the windowing system

The window icon might be used by the windowing system for example to decorate the window, and/or in the task switcher.

Note: On macOS, the window title bar icon is meant for windows representing documents, and will only show up if a file path is also set.

See also icon() and setFilePath().
*/
impl /*struct*/ QWindow {
  pub fn setIcon_0<RetType, T: QWindow_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QWindow_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:247
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon icon() const

/*
Returns the window's icon in the windowing system

See also setIcon().
*/
impl /*struct*/ QWindow {
  pub fn icon_0<RetType, T: QWindow_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QWindow_icon_0<RetType> {
  fn icon_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:249
// index:0
// Public Visibility=Default Availability=Available
// [-2] void destroy()

/*
Releases the native platform resources associated with this window.

See also create().
*/
impl /*struct*/ QWindow {
  pub fn destroy_0<RetType, T: QWindow_destroy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroy_0(self);
    // return 1;
  }
}
pub trait QWindow_destroy_0<RetType> {
  fn destroy_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_destroy_0<(/*void*/)> for () {
  fn destroy_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow7destroyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:253
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setKeyboardGrabEnabled(bool)

/*
Sets whether keyboard grab should be enabled or not (grab).

If the return value is true, the window receives all key events until setKeyboardGrabEnabled(false) is called; other windows get no key events at all. Mouse events are not affected. Use setMouseGrabEnabled() if you want to grab that.

See also setMouseGrabEnabled().
*/
impl /*struct*/ QWindow {
  pub fn setKeyboardGrabEnabled_0<RetType, T: QWindow_setKeyboardGrabEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardGrabEnabled_0(self);
    // return 1;
  }
}
pub trait QWindow_setKeyboardGrabEnabled_0<RetType> {
  fn setKeyboardGrabEnabled_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setKeyboardGrabEnabled_0<bool> for (bool) {
  fn setKeyboardGrabEnabled_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWindow22setKeyboardGrabEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:254
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setMouseGrabEnabled(bool)

/*
Sets whether mouse grab should be enabled or not (grab).

If the return value is true, the window receives all mouse events until setMouseGrabEnabled(false) is called; other windows get no mouse events at all. Keyboard events are not affected. Use setKeyboardGrabEnabled() if you want to grab that.

See also setKeyboardGrabEnabled().
*/
impl /*struct*/ QWindow {
  pub fn setMouseGrabEnabled_0<RetType, T: QWindow_setMouseGrabEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMouseGrabEnabled_0(self);
    // return 1;
  }
}
pub trait QWindow_setMouseGrabEnabled_0<RetType> {
  fn setMouseGrabEnabled_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMouseGrabEnabled_0<bool> for (bool) {
  fn setMouseGrabEnabled_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWindow19setMouseGrabEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:256
// index:0
// Public Visibility=Default Availability=Available
// [8] QScreen * screen() const

/*
Returns the screen on which the window is shown, or null if there is none.

For child windows, this returns the screen of the corresponding top level window.

See also setScreen() and QScreen::virtualSiblings().
*/
impl /*struct*/ QWindow {
  pub fn screen_0<RetType, T: QWindow_screen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screen_0(self);
    // return 1;
  }
}
pub trait QWindow_screen_0<RetType> {
  fn screen_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_screen_0<usize> for () {
  fn screen_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow6screenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:257
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreen(QScreen *)

/*
Sets the screen on which the window should be shown.

If the window has been created, it will be recreated on the newScreen.

Note: If the screen is part of a virtual desktop of multiple screens, the window will not move automatically to newScreen. To place the window relative to the screen, use the screen's topLeft() position.

This function only works for top level windows.

See also screen() and QScreen::virtualSiblings().
*/
impl /*struct*/ QWindow {
  pub fn setScreen_0<RetType, T: QWindow_setScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreen_0(self);
    // return 1;
  }
}
pub trait QWindow_setScreen_0<RetType> {
  fn setScreen_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setScreen_0<(/*void*/)> for (usize) {
  fn setScreen_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9setScreenEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:259
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QAccessibleInterface * accessibleRoot() const

/*

*/
impl /*struct*/ QWindow {
  pub fn accessibleRoot_0<RetType, T: QWindow_accessibleRoot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accessibleRoot_0(self);
    // return 1;
  }
}
pub trait QWindow_accessibleRoot_0<RetType> {
  fn accessibleRoot_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_accessibleRoot_0<usize> for () {
  fn accessibleRoot_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow14accessibleRootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:260
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QObject * focusObject() const

/*
Returns the QObject that will be the final receiver of events tied focus, such as key events.
*/
impl /*struct*/ QWindow {
  pub fn focusObject_0<RetType, T: QWindow_focusObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusObject_0(self);
    // return 1;
  }
}
pub trait QWindow_focusObject_0<RetType> {
  fn focusObject_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_focusObject_0<usize> for () {
  fn focusObject_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow11focusObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:262
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapToGlobal(const QPoint &) const

/*
Translates the window coordinate pos to global screen coordinates. For example, mapToGlobal(QPoint(0,0)) would give the global coordinates of the top-left pixel of the window.

See also mapFromGlobal().
*/
impl /*struct*/ QWindow {
  pub fn mapToGlobal_0<RetType, T: QWindow_mapToGlobal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapToGlobal_0(self);
    // return 1;
  }
}
pub trait QWindow_mapToGlobal_0<RetType> {
  fn mapToGlobal_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mapToGlobal_0<usize> for (usize) {
  fn mapToGlobal_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow11mapToGlobalERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:263
// index:0
// Public Visibility=Default Availability=Available
// [8] QPoint mapFromGlobal(const QPoint &) const

/*
Translates the global screen coordinate pos to window coordinates.

See also mapToGlobal().
*/
impl /*struct*/ QWindow {
  pub fn mapFromGlobal_0<RetType, T: QWindow_mapFromGlobal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapFromGlobal_0(self);
    // return 1;
  }
}
pub trait QWindow_mapFromGlobal_0<RetType> {
  fn mapFromGlobal_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mapFromGlobal_0<usize> for (usize) {
  fn mapFromGlobal_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow13mapFromGlobalERK6QPoint", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:266
// index:0
// Public Visibility=Default Availability=Available
// [8] QCursor cursor() const

/*
the cursor shape for this window

See also setCursor() and unsetCursor().
*/
impl /*struct*/ QWindow {
  pub fn cursor_0<RetType, T: QWindow_cursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursor_0(self);
    // return 1;
  }
}
pub trait QWindow_cursor_0<RetType> {
  fn cursor_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_cursor_0<usize> for () {
  fn cursor_0(self , rsthis: & QWindow) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QWindow6cursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:267
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCursor(const QCursor &)

/*
set the cursor shape for this window

The mouse cursor will assume this shape when it is over this window, unless an override cursor is set. See the list of predefined cursor objects for a range of useful shapes.

If no cursor has been set, or after a call to unsetCursor(), the parent window's cursor is used.

By default, the cursor has the Qt::ArrowCursor shape.

Some underlying window implementations will reset the cursor if it leaves a window even if the mouse is grabbed. If you want to have a cursor set for all windows, even when outside the window, consider QGuiApplication::setOverrideCursor().

See also cursor() and QGuiApplication::setOverrideCursor().
*/
impl /*struct*/ QWindow {
  pub fn setCursor_0<RetType, T: QWindow_setCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursor_0(self);
    // return 1;
  }
}
pub trait QWindow_setCursor_0<RetType> {
  fn setCursor_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setCursor_0<(/*void*/)> for (usize) {
  fn setCursor_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9setCursorERK7QCursor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:268
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetCursor()

/*
Restores the default arrow cursor for this window.
*/
impl /*struct*/ QWindow {
  pub fn unsetCursor_0<RetType, T: QWindow_unsetCursor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor_0(self);
    // return 1;
  }
}
pub trait QWindow_unsetCursor_0<RetType> {
  fn unsetCursor_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_unsetCursor_0<(/*void*/)> for () {
  fn unsetCursor_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow11unsetCursorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:271
// index:0
// Public static Visibility=Default Availability=Available
// [8] QWindow * fromWinId(WId)

/*
Creates a local representation of a window created by another process or by using native libraries below Qt.

Given the handle id to a native window, this method creates a QWindow object which can be used to represent the window when invoking methods like setParent() and setTransientParent().

This can be used, on platforms which support it, to embed a QWindow inside a native window, or to embed a native window inside a QWindow.

If foreign windows are not supported or embedding the native window failed in the platform plugin, this function returns 0.

Note: The resulting QWindow should not be used to manipulate the underlying native window (besides re-parenting), or to observe state changes of the native window. Any support for these kind of operations is incidental, highly platform dependent and untested.

See also setParent() and setTransientParent().
*/
impl /*struct*/ QWindow {
  pub fn fromWinId_0<RetType, T: QWindow_fromWinId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromWinId_0();
    // return 1;
  }
}
pub trait QWindow_fromWinId_0<RetType> {
  fn fromWinId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QWindow_fromWinId_0<usize> for (u64) {
  fn fromWinId_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWindow9fromWinIdEy", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:279
// index:0
// Public Visibility=Default Availability=Available
// [-2] void requestActivate()

/*
Requests the window to be activated, i.e. receive keyboard focus.

See also isActive(), QGuiApplication::focusWindow(), and QWindowsWindowFunctions::setWindowActivationBehavior().
*/
impl /*struct*/ QWindow {
  pub fn requestActivate_0<RetType, T: QWindow_requestActivate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestActivate_0(self);
    // return 1;
  }
}
pub trait QWindow_requestActivate_0<RetType> {
  fn requestActivate_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_requestActivate_0<(/*void*/)> for () {
  fn requestActivate_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow15requestActivateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:281
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*

*/
impl /*struct*/ QWindow {
  pub fn setVisible_0<RetType, T: QWindow_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QWindow_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:283
// index:0
// Public Visibility=Default Availability=Available
// [-2] void show()

/*
Shows the window.

This is equivalent to calling showFullScreen(), showMaximized(), or showNormal(), depending on the platform's default behavior for the window type and flags.

See also showFullScreen(), showMaximized(), showNormal(), hide(), QStyleHints::showIsFullScreen(), and flags().
*/
impl /*struct*/ QWindow {
  pub fn show_0<RetType, T: QWindow_show_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.show_0(self);
    // return 1;
  }
}
pub trait QWindow_show_0<RetType> {
  fn show_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_show_0<(/*void*/)> for () {
  fn show_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow4showEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:284
// index:0
// Public Visibility=Default Availability=Available
// [-2] void hide()

/*
Hides the window.

Equivalent to calling setVisible(false).

See also show() and setVisible().
*/
impl /*struct*/ QWindow {
  pub fn hide_0<RetType, T: QWindow_hide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hide_0(self);
    // return 1;
  }
}
pub trait QWindow_hide_0<RetType> {
  fn hide_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_hide_0<(/*void*/)> for () {
  fn hide_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow4hideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:286
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMinimized()

/*
Shows the window as minimized.

Equivalent to calling setWindowStates(Qt::WindowMinimized) and then setVisible(true).

See also setWindowStates() and setVisible().
*/
impl /*struct*/ QWindow {
  pub fn showMinimized_0<RetType, T: QWindow_showMinimized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMinimized_0(self);
    // return 1;
  }
}
pub trait QWindow_showMinimized_0<RetType> {
  fn showMinimized_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_showMinimized_0<(/*void*/)> for () {
  fn showMinimized_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow13showMinimizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:287
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMaximized()

/*
Shows the window as maximized.

Equivalent to calling setWindowStates(Qt::WindowMaximized) and then setVisible(true).

See also setWindowStates() and setVisible().
*/
impl /*struct*/ QWindow {
  pub fn showMaximized_0<RetType, T: QWindow_showMaximized_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMaximized_0(self);
    // return 1;
  }
}
pub trait QWindow_showMaximized_0<RetType> {
  fn showMaximized_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_showMaximized_0<(/*void*/)> for () {
  fn showMaximized_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow13showMaximizedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:288
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showFullScreen()

/*
Shows the window as fullscreen.

Equivalent to calling setWindowStates(Qt::WindowFullScreen) and then setVisible(true).

See also setWindowStates() and setVisible().
*/
impl /*struct*/ QWindow {
  pub fn showFullScreen_0<RetType, T: QWindow_showFullScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showFullScreen_0(self);
    // return 1;
  }
}
pub trait QWindow_showFullScreen_0<RetType> {
  fn showFullScreen_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_showFullScreen_0<(/*void*/)> for () {
  fn showFullScreen_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow14showFullScreenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:289
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showNormal()

/*
Shows the window as normal, i.e. neither maximized, minimized, nor fullscreen.

Equivalent to calling setWindowStates(Qt::WindowNoState) and then setVisible(true).

See also setWindowStates() and setVisible().
*/
impl /*struct*/ QWindow {
  pub fn showNormal_0<RetType, T: QWindow_showNormal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showNormal_0(self);
    // return 1;
  }
}
pub trait QWindow_showNormal_0<RetType> {
  fn showNormal_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_showNormal_0<(/*void*/)> for () {
  fn showNormal_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow10showNormalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:291
// index:0
// Public Visibility=Default Availability=Available
// [1] bool close()

/*
Close the window.

This closes the window, effectively calling destroy(), and potentially quitting the application. Returns true on success, false if it has a parent window (in which case the top level window should be closed instead).

See also destroy() and QGuiApplication::quitOnLastWindowClosed().
*/
impl /*struct*/ QWindow {
  pub fn close_0<RetType, T: QWindow_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QWindow_close_0<RetType> {
  fn close_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_close_0<bool> for () {
  fn close_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWindow5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:292
// index:0
// Public Visibility=Default Availability=Available
// [-2] void raise()

/*
Raise the window in the windowing system.

Requests that the window be raised to appear above other windows.
*/
impl /*struct*/ QWindow {
  pub fn raise_0<RetType, T: QWindow_raise_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.raise_0(self);
    // return 1;
  }
}
pub trait QWindow_raise_0<RetType> {
  fn raise_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_raise_0<(/*void*/)> for () {
  fn raise_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow5raiseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:293
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lower()

/*
Lower the window in the windowing system.

Requests that the window be lowered to appear below other windows.
*/
impl /*struct*/ QWindow {
  pub fn lower_0<RetType, T: QWindow_lower_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lower_0(self);
    // return 1;
  }
}
pub trait QWindow_lower_0<RetType> {
  fn lower_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_lower_0<(/*void*/)> for () {
  fn lower_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow5lowerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:295
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTitle(const QString &)

/*

*/
impl /*struct*/ QWindow {
  pub fn setTitle_0<RetType, T: QWindow_setTitle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTitle_0(self);
    // return 1;
  }
}
pub trait QWindow_setTitle_0<RetType> {
  fn setTitle_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setTitle_0<(/*void*/)> for (usize) {
  fn setTitle_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow8setTitleERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:297
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setX(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setX_0<RetType, T: QWindow_setX_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setX_0(self);
    // return 1;
  }
}
pub trait QWindow_setX_0<RetType> {
  fn setX_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setX_0<(/*void*/)> for (i32) {
  fn setX_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow4setXEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:298
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setY(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setY_0<RetType, T: QWindow_setY_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setY_0(self);
    // return 1;
  }
}
pub trait QWindow_setY_0<RetType> {
  fn setY_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setY_0<(/*void*/)> for (i32) {
  fn setY_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow4setYEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidth(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setWidth_0<RetType, T: QWindow_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QWindow_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setWidth_0<(/*void*/)> for (i32) {
  fn setWidth_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow8setWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:300
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHeight(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setHeight_0<RetType, T: QWindow_setHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHeight_0(self);
    // return 1;
  }
}
pub trait QWindow_setHeight_0<RetType> {
  fn setHeight_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setHeight_0<(/*void*/)> for (i32) {
  fn setHeight_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9setHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGeometry(int, int, int, int)

/*
Sets the geometry of the window, excluding its window frame, to a rectangle constructed from posx, posy, w and h.

The geometry is in relation to the virtualGeometry() of its screen.

See also geometry().
*/
impl /*struct*/ QWindow {
  pub fn setGeometry_0<RetType, T: QWindow_setGeometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_0(self);
    // return 1;
  }
}
pub trait QWindow_setGeometry_0<RetType> {
  fn setGeometry_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setGeometry_0<(/*void*/)> for (i32,i32,i32,i32) {
  fn setGeometry_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setGeometryEiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:302
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setGeometry(const QRect &)

/*
Sets the geometry of the window, excluding its window frame, to a rectangle constructed from posx, posy, w and h.

The geometry is in relation to the virtualGeometry() of its screen.

See also geometry().
*/
impl /*struct*/ QWindow {
  pub fn setGeometry_1<RetType, T: QWindow_setGeometry_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGeometry_1(self);
    // return 1;
  }
}
pub trait QWindow_setGeometry_1<RetType> {
  fn setGeometry_1(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setGeometry_1<(/*void*/)> for (usize) {
  fn setGeometry_1(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11setGeometryERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:304
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumWidth(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setMinimumWidth_0<RetType, T: QWindow_setMinimumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth_0(self);
    // return 1;
  }
}
pub trait QWindow_setMinimumWidth_0<RetType> {
  fn setMinimumWidth_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMinimumWidth_0<(/*void*/)> for (i32) {
  fn setMinimumWidth_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow15setMinimumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:305
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimumHeight(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setMinimumHeight_0<RetType, T: QWindow_setMinimumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight_0(self);
    // return 1;
  }
}
pub trait QWindow_setMinimumHeight_0<RetType> {
  fn setMinimumHeight_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMinimumHeight_0<(/*void*/)> for (i32) {
  fn setMinimumHeight_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow16setMinimumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:306
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumWidth(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setMaximumWidth_0<RetType, T: QWindow_setMaximumWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth_0(self);
    // return 1;
  }
}
pub trait QWindow_setMaximumWidth_0<RetType> {
  fn setMaximumWidth_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMaximumWidth_0<(/*void*/)> for (i32) {
  fn setMaximumWidth_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow15setMaximumWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:307
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaximumHeight(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn setMaximumHeight_0<RetType, T: QWindow_setMaximumHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight_0(self);
    // return 1;
  }
}
pub trait QWindow_setMaximumHeight_0<RetType> {
  fn setMaximumHeight_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_setMaximumHeight_0<(/*void*/)> for (i32) {
  fn setMaximumHeight_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow16setMaximumHeightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:309
// index:0
// Public Visibility=Default Availability=Available
// [-2] void alert(int)

/*
Causes an alert to be shown for msec miliseconds. If msec is 0 (the default), then the alert is shown indefinitely until the window becomes active again. This function has no effect on an active window.

In alert state, the window indicates that it demands attention, for example by flashing or bouncing the taskbar entry.

This function was introduced in  Qt 5.1.
*/
impl /*struct*/ QWindow {
  pub fn alert_0<RetType, T: QWindow_alert_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alert_0(self);
    // return 1;
  }
}
pub trait QWindow_alert_0<RetType> {
  fn alert_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_alert_0<(/*void*/)> for (i32) {
  fn alert_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow5alertEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:311
// index:0
// Public Visibility=Default Availability=Available
// [-2] void requestUpdate()

/*
Schedules a QEvent::UpdateRequest event to be delivered to this window.

The event is delivered in sync with the display vsync on platforms where this is possible. Otherwise, the event is delivered after a delay of 5 ms. The additional time is there to give the event loop a bit of idle time to gather system events, and can be overridden using the QT_QPA_UPDATE_IDLE_TIME environment variable.

When driving animations, this function should be called once after drawing has completed. Calling this function multiple times will result in a single event being delivered to the window.

Subclasses of QWindow should reimplement event(), intercept the event and call the application's rendering code, then call the base class implementation.

Note: The subclass' reimplementation of event() must invoke the base class implementation, unless it is absolutely sure that the event does not need to be handled by the base class. For example, the default implementation of this function relies on QEvent::Timer events. Filtering them away would therefore break the delivery of the update events.

This function was introduced in  Qt 5.5.
*/
impl /*struct*/ QWindow {
  pub fn requestUpdate_0<RetType, T: QWindow_requestUpdate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestUpdate_0(self);
    // return 1;
  }
}
pub trait QWindow_requestUpdate_0<RetType> {
  fn requestUpdate_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_requestUpdate_0<(/*void*/)> for () {
  fn requestUpdate_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow13requestUpdateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:314
// index:0
// Public Visibility=Default Availability=Available
// [-2] void screenChanged(QScreen *)

/*
This signal is emitted when a window's screen changes, either by being set explicitly with setScreen(), or automatically when the window's screen is removed.
*/
impl /*struct*/ QWindow {
  pub fn screenChanged_0<RetType, T: QWindow_screenChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_screenChanged_0<RetType> {
  fn screenChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_screenChanged_0<(/*void*/)> for (usize) {
  fn screenChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow13screenChangedEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:315
// index:0
// Public Visibility=Default Availability=Available
// [-2] void modalityChanged(Qt::WindowModality)

/*
This signal is emitted when the Qwindow::modality property changes to modality.

Note: Notifier signal for property modality.
*/
impl /*struct*/ QWindow {
  pub fn modalityChanged_0<RetType, T: QWindow_modalityChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modalityChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_modalityChanged_0<RetType> {
  fn modalityChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_modalityChanged_0<(/*void*/)> for (i32) {
  fn modalityChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow15modalityChangedEN2Qt14WindowModalityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:316
// index:0
// Public Visibility=Default Availability=Available
// [-2] void windowStateChanged(Qt::WindowState)

/*
This signal is emitted when the windowState changes, either by being set explicitly with setWindowStates(), or automatically when the user clicks one of the titlebar buttons or by other means.
*/
impl /*struct*/ QWindow {
  pub fn windowStateChanged_0<RetType, T: QWindow_windowStateChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowStateChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_windowStateChanged_0<RetType> {
  fn windowStateChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_windowStateChanged_0<(/*void*/)> for (i32) {
  fn windowStateChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow18windowStateChangedEN2Qt11WindowStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:317
// index:0
// Public Visibility=Default Availability=Available
// [-2] void windowTitleChanged(const QString &)

/*

*/
impl /*struct*/ QWindow {
  pub fn windowTitleChanged_0<RetType, T: QWindow_windowTitleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.windowTitleChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_windowTitleChanged_0<RetType> {
  fn windowTitleChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_windowTitleChanged_0<(/*void*/)> for (usize) {
  fn windowTitleChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow18windowTitleChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:319
// index:0
// Public Visibility=Default Availability=Available
// [-2] void xChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn xChanged_0<RetType, T: QWindow_xChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_xChanged_0<RetType> {
  fn xChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_xChanged_0<(/*void*/)> for (i32) {
  fn xChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow8xChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:320
// index:0
// Public Visibility=Default Availability=Available
// [-2] void yChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn yChanged_0<RetType, T: QWindow_yChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.yChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_yChanged_0<RetType> {
  fn yChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_yChanged_0<(/*void*/)> for (i32) {
  fn yChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow8yChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:322
// index:0
// Public Visibility=Default Availability=Available
// [-2] void widthChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn widthChanged_0<RetType, T: QWindow_widthChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widthChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_widthChanged_0<RetType> {
  fn widthChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_widthChanged_0<(/*void*/)> for (i32) {
  fn widthChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow12widthChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:323
// index:0
// Public Visibility=Default Availability=Available
// [-2] void heightChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn heightChanged_0<RetType, T: QWindow_heightChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.heightChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_heightChanged_0<RetType> {
  fn heightChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_heightChanged_0<(/*void*/)> for (i32) {
  fn heightChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow13heightChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:325
// index:0
// Public Visibility=Default Availability=Available
// [-2] void minimumWidthChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn minimumWidthChanged_0<RetType, T: QWindow_minimumWidthChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumWidthChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_minimumWidthChanged_0<RetType> {
  fn minimumWidthChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_minimumWidthChanged_0<(/*void*/)> for (i32) {
  fn minimumWidthChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow19minimumWidthChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:326
// index:0
// Public Visibility=Default Availability=Available
// [-2] void minimumHeightChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn minimumHeightChanged_0<RetType, T: QWindow_minimumHeightChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minimumHeightChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_minimumHeightChanged_0<RetType> {
  fn minimumHeightChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_minimumHeightChanged_0<(/*void*/)> for (i32) {
  fn minimumHeightChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow20minimumHeightChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:327
// index:0
// Public Visibility=Default Availability=Available
// [-2] void maximumWidthChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn maximumWidthChanged_0<RetType, T: QWindow_maximumWidthChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumWidthChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_maximumWidthChanged_0<RetType> {
  fn maximumWidthChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_maximumWidthChanged_0<(/*void*/)> for (i32) {
  fn maximumWidthChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow19maximumWidthChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:328
// index:0
// Public Visibility=Default Availability=Available
// [-2] void maximumHeightChanged(int)

/*

*/
impl /*struct*/ QWindow {
  pub fn maximumHeightChanged_0<RetType, T: QWindow_maximumHeightChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maximumHeightChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_maximumHeightChanged_0<RetType> {
  fn maximumHeightChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_maximumHeightChanged_0<(/*void*/)> for (i32) {
  fn maximumHeightChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow20maximumHeightChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:330
// index:0
// Public Visibility=Default Availability=Available
// [-2] void visibleChanged(bool)

/*

*/
impl /*struct*/ QWindow {
  pub fn visibleChanged_0<RetType, T: QWindow_visibleChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibleChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_visibleChanged_0<RetType> {
  fn visibleChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_visibleChanged_0<(/*void*/)> for (bool) {
  fn visibleChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14visibleChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:331
// index:0
// Public Visibility=Default Availability=Available
// [-2] void visibilityChanged(QWindow::Visibility)

/*

*/
impl /*struct*/ QWindow {
  pub fn visibilityChanged_0<RetType, T: QWindow_visibilityChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.visibilityChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_visibilityChanged_0<RetType> {
  fn visibilityChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_visibilityChanged_0<(/*void*/)> for (i32) {
  fn visibilityChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow17visibilityChangedENS_10VisibilityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:332
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activeChanged()

/*

*/
impl /*struct*/ QWindow {
  pub fn activeChanged_0<RetType, T: QWindow_activeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_activeChanged_0<RetType> {
  fn activeChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_activeChanged_0<(/*void*/)> for () {
  fn activeChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QWindow13activeChangedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:333
// index:0
// Public Visibility=Default Availability=Available
// [-2] void contentOrientationChanged(Qt::ScreenOrientation)

/*

*/
impl /*struct*/ QWindow {
  pub fn contentOrientationChanged_0<RetType, T: QWindow_contentOrientationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentOrientationChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_contentOrientationChanged_0<RetType> {
  fn contentOrientationChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_contentOrientationChanged_0<(/*void*/)> for (i32) {
  fn contentOrientationChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow25contentOrientationChangedEN2Qt17ScreenOrientationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:335
// index:0
// Public Visibility=Default Availability=Available
// [-2] void focusObjectChanged(QObject *)

/*
This signal is emitted when the final receiver of events tied to focus is changed to object.

See also focusObject().
*/
impl /*struct*/ QWindow {
  pub fn focusObjectChanged_0<RetType, T: QWindow_focusObjectChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusObjectChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_focusObjectChanged_0<RetType> {
  fn focusObjectChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_focusObjectChanged_0<(/*void*/)> for (usize) {
  fn focusObjectChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow18focusObjectChangedEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:337
// index:0
// Public Visibility=Default Availability=Available
// [-2] void opacityChanged(qreal)

/*

*/
impl /*struct*/ QWindow {
  pub fn opacityChanged_0<RetType, T: QWindow_opacityChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.opacityChanged_0(self);
    // return 1;
  }
}
pub trait QWindow_opacityChanged_0<RetType> {
  fn opacityChanged_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_opacityChanged_0<(/*void*/)> for (f64) {
  fn opacityChanged_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14opacityChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:340
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void exposeEvent(QExposeEvent *)

/*
The expose event (ev) is sent by the window system whenever the window's exposure on screen changes.

The application can start rendering into the window with QBackingStore and QOpenGLContext as soon as it gets an exposeEvent() such that isExposed() is true.

If the window is moved off screen, is made totally obscured by another window, iconified or similar, this function might be called and the value of isExposed() might change to false. When this happens, an application should stop its rendering as it is no longer visible to the user.

A resize event will always be sent before the expose event the first time a window is shown.

See also isExposed().
*/
impl /*struct*/ QWindow {
  pub fn exposeEvent_0<RetType, T: QWindow_exposeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exposeEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_exposeEvent_0<RetType> {
  fn exposeEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_exposeEvent_0<(/*void*/)> for (usize) {
  fn exposeEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11exposeEventEP12QExposeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:341
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void resizeEvent(QResizeEvent *)

/*
Override this to handle resize events (ev).

The resize event is called whenever the window is resized in the windowing system, either directly through the windowing system acknowledging a setGeometry() or resize() request, or indirectly through the user resizing the window manually.
*/
impl /*struct*/ QWindow {
  pub fn resizeEvent_0<RetType, T: QWindow_resizeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resizeEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_resizeEvent_0<RetType> {
  fn resizeEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_resizeEvent_0<(/*void*/)> for (usize) {
  fn resizeEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11resizeEventEP12QResizeEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:342
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void moveEvent(QMoveEvent *)

/*
Override this to handle window move events (ev).
*/
impl /*struct*/ QWindow {
  pub fn moveEvent_0<RetType, T: QWindow_moveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.moveEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_moveEvent_0<RetType> {
  fn moveEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_moveEvent_0<(/*void*/)> for (usize) {
  fn moveEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9moveEventEP10QMoveEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:343
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusInEvent(QFocusEvent *)

/*
Override this to handle focus in events (ev).

Focus in events are sent when the window receives keyboard focus.

See also focusOutEvent().
*/
impl /*struct*/ QWindow {
  pub fn focusInEvent_0<RetType, T: QWindow_focusInEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusInEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_focusInEvent_0<RetType> {
  fn focusInEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_focusInEvent_0<(/*void*/)> for (usize) {
  fn focusInEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow12focusInEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:344
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void focusOutEvent(QFocusEvent *)

/*
Override this to handle focus out events (ev).

Focus out events are sent when the window loses keyboard focus.

See also focusInEvent().
*/
impl /*struct*/ QWindow {
  pub fn focusOutEvent_0<RetType, T: QWindow_focusOutEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.focusOutEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_focusOutEvent_0<RetType> {
  fn focusOutEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_focusOutEvent_0<(/*void*/)> for (usize) {
  fn focusOutEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow13focusOutEventEP11QFocusEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:346
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void showEvent(QShowEvent *)

/*
Override this to handle show events (ev).

The function is called when the window has requested becoming visible.

If the window is successfully shown by the windowing system, this will be followed by a resize and an expose event.
*/
impl /*struct*/ QWindow {
  pub fn showEvent_0<RetType, T: QWindow_showEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_showEvent_0<RetType> {
  fn showEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_showEvent_0<(/*void*/)> for (usize) {
  fn showEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9showEventEP10QShowEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:347
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void hideEvent(QHideEvent *)

/*
Override this to handle hide events (ev).

The function is called when the window has requested being hidden in the windowing system.
*/
impl /*struct*/ QWindow {
  pub fn hideEvent_0<RetType, T: QWindow_hideEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hideEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_hideEvent_0<RetType> {
  fn hideEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_hideEvent_0<(/*void*/)> for (usize) {
  fn hideEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow9hideEventEP10QHideEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:350
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().

Override this to handle any event (ev) sent to the window. Return true if the event was recognized and processed.

Remember to call the base class version if you wish for mouse events, key events, resize events, etc to be dispatched as usual.
*/
impl /*struct*/ QWindow {
  pub fn event_0<RetType, T: QWindow_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QWindow_event_0<RetType> {
  fn event_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWindow5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qwindow.h:351
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyPressEvent(QKeyEvent *)

/*
Override this to handle key press events (ev).

See also keyReleaseEvent().
*/
impl /*struct*/ QWindow {
  pub fn keyPressEvent_0<RetType, T: QWindow_keyPressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyPressEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_keyPressEvent_0<RetType> {
  fn keyPressEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_keyPressEvent_0<(/*void*/)> for (usize) {
  fn keyPressEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow13keyPressEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:352
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void keyReleaseEvent(QKeyEvent *)

/*
Override this to handle key release events (ev).

See also keyPressEvent().
*/
impl /*struct*/ QWindow {
  pub fn keyReleaseEvent_0<RetType, T: QWindow_keyReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keyReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_keyReleaseEvent_0<RetType> {
  fn keyReleaseEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_keyReleaseEvent_0<(/*void*/)> for (usize) {
  fn keyReleaseEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow15keyReleaseEventEP9QKeyEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:353
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mousePressEvent(QMouseEvent *)

/*
Override this to handle mouse press events (ev).

See also mouseReleaseEvent().
*/
impl /*struct*/ QWindow {
  pub fn mousePressEvent_0<RetType, T: QWindow_mousePressEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mousePressEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_mousePressEvent_0<RetType> {
  fn mousePressEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mousePressEvent_0<(/*void*/)> for (usize) {
  fn mousePressEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow15mousePressEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:354
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseReleaseEvent(QMouseEvent *)

/*
Override this to handle mouse release events (ev).

See also mousePressEvent().
*/
impl /*struct*/ QWindow {
  pub fn mouseReleaseEvent_0<RetType, T: QWindow_mouseReleaseEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseReleaseEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_mouseReleaseEvent_0<RetType> {
  fn mouseReleaseEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mouseReleaseEvent_0<(/*void*/)> for (usize) {
  fn mouseReleaseEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow17mouseReleaseEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:355
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseDoubleClickEvent(QMouseEvent *)

/*
Override this to handle mouse double click events (ev).

See also mousePressEvent() and QStyleHints::mouseDoubleClickInterval().
*/
impl /*struct*/ QWindow {
  pub fn mouseDoubleClickEvent_0<RetType, T: QWindow_mouseDoubleClickEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseDoubleClickEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_mouseDoubleClickEvent_0<RetType> {
  fn mouseDoubleClickEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mouseDoubleClickEvent_0<(/*void*/)> for (usize) {
  fn mouseDoubleClickEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow21mouseDoubleClickEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:356
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void mouseMoveEvent(QMouseEvent *)

/*
Override this to handle mouse move events (ev).
*/
impl /*struct*/ QWindow {
  pub fn mouseMoveEvent_0<RetType, T: QWindow_mouseMoveEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mouseMoveEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_mouseMoveEvent_0<RetType> {
  fn mouseMoveEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_mouseMoveEvent_0<(/*void*/)> for (usize) {
  fn mouseMoveEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow14mouseMoveEventEP11QMouseEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:358
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void wheelEvent(QWheelEvent *)

/*
Override this to handle mouse wheel or other wheel events (ev).
*/
impl /*struct*/ QWindow {
  pub fn wheelEvent_0<RetType, T: QWindow_wheelEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wheelEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_wheelEvent_0<RetType> {
  fn wheelEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_wheelEvent_0<(/*void*/)> for (usize) {
  fn wheelEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow10wheelEventEP11QWheelEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:360
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void touchEvent(QTouchEvent *)

/*
Override this to handle touch events (ev).
*/
impl /*struct*/ QWindow {
  pub fn touchEvent_0<RetType, T: QWindow_touchEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.touchEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_touchEvent_0<RetType> {
  fn touchEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_touchEvent_0<(/*void*/)> for (usize) {
  fn touchEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow10touchEventEP11QTouchEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:362
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void tabletEvent(QTabletEvent *)

/*
Override this to handle tablet press, move, and release events (ev).

Proximity enter and leave events are not sent to windows, they are delivered to the application instance.
*/
impl /*struct*/ QWindow {
  pub fn tabletEvent_0<RetType, T: QWindow_tabletEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tabletEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_tabletEvent_0<RetType> {
  fn tabletEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_tabletEvent_0<(/*void*/)> for (usize) {
  fn tabletEvent_0(self , rsthis: & QWindow) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QWindow11tabletEventEP12QTabletEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qwindow.h:364
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool nativeEvent(const QByteArray &, void *, long *)

/*
Override this to handle platform dependent events. Will be given eventType, message and result.

This might make your application non-portable.

Should return true only if the event was handled.
*/
impl /*struct*/ QWindow {
  pub fn nativeEvent_0<RetType, T: QWindow_nativeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeEvent_0(self);
    // return 1;
  }
}
pub trait QWindow_nativeEvent_0<RetType> {
  fn nativeEvent_0(self , rsthis: & QWindow) -> RetType;
}
impl<'a> /*trait*/ QWindow_nativeEvent_0<bool> for (usize,usize,usize) {
  fn nativeEvent_0(self , rsthis: & QWindow) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QWindow11nativeEventERK10QByteArrayPvPl", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes what part of the screen the window occupies or should occupy.



This enum was introduced or modified in  Qt 5.1.

*/
pub type QWindow__Visibility = i32;
// The window is not visible in any way, however it may remember a latent visibility which can be restored by setting AutomaticVisibility.
pub const QWindow__Hidden :QWindow__Visibility = 0;
// This means to give the window a default visible state, which might be fullscreen or windowed depending on the platform. It can be given as a parameter to setVisibility but will never be read back from the visibility accessor.
pub const QWindow__AutomaticVisibility :QWindow__Visibility = 1;
// The window occupies part of the screen, but not necessarily the entire screen. This state will occur only on windowing systems which support showing multiple windows simultaneously. In this state it is possible for the user to move and resize the window manually, if WindowFlags permit it and if it is supported by the windowing system.
pub const QWindow__Windowed :QWindow__Visibility = 2;
// The window is reduced to an entry or icon on the task bar, dock, task list or desktop, depending on how the windowing system handles minimized windows.
pub const QWindow__Minimized :QWindow__Visibility = 3;
// The window occupies one entire screen, and the titlebar is still visible. On most windowing systems this is the state achieved by clicking the maximize button on the toolbar.
pub const QWindow__Maximized :QWindow__Visibility = 4;
// The window occupies one entire screen, is not resizable, and there is no titlebar. On some platforms which do not support showing multiple simultaneous windows, this can be the usual visibility when the window is not hidden.
pub const QWindow__FullScreen :QWindow__Visibility = 5;
pub fn QWindow_VisibilityItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWindow", val);
}
pub fn QWindow_VisibilityItemName_s(val: i32) ->String {
  //var nilthis *QWindow
  //return nilthis.VisibilityItemName(val);
  return QWindow_VisibilityItemName(val);
}


/*
This enum is used to control whether or not transient parents should be considered ancestors.


*/
pub type QWindow__AncestorMode = i32;
// Transient parents are not considered ancestors.
pub const QWindow__ExcludeTransients :QWindow__AncestorMode = 0;
// Transient parents are considered ancestors.
pub const QWindow__IncludeTransients :QWindow__AncestorMode = 1;
pub fn QWindow_AncestorModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QWindow", val);
}
pub fn QWindow_AncestorModeItemName_s(val: i32) ->String {
  //var nilthis *QWindow
  //return nilthis.AncestorModeItemName(val);
  return QWindow_AncestorModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
