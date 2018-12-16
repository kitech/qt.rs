

// mod ::gui::QOffscreenSurface
// package qtgui
// /usr/include/qt/QtGui/qoffscreensurface.h
// #include <qoffscreensurface.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 42
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



/*

*/
#[derive(Default)] // class sizeof(QOffscreenSurface)=40
pub struct QOffscreenSurface {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QOffscreenSurface_ITF interface {
//    qtcore.QObject_ITF
//    QSurface_ITF
//    QOffscreenSurface_PTR() *QOffscreenSurface
//}
//func (ptr *QOffscreenSurface) QOffscreenSurface_PTR() *QOffscreenSurface { return ptr }

impl /*struct*/ QOffscreenSurface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QOffscreenSurface {
    return QOffscreenSurface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QOffscreenSurface {
//  type Target = QOffscreenSurfaceBASE;
//
//  fn deref(&self) -> &QOffscreenSurfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QOffscreenSurfaceBASE> for QOffscreenSurface {
//  fn as_ref(& self) -> & QOffscreenSurfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qoffscreensurface.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QOffscreenSurface {
  pub fn metaObject_0<RetType, T: QOffscreenSurface_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QOffscreenSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QOffscreenSurface(QScreen *, QObject *)

/*
Creates an offscreen surface for the targetScreen with the given parent.

The underlying platform surface is not created until create() is called.

This function was introduced in  Qt 5.10.

See also setScreen() and create().
*/
// QOffscreenSurface(QScreen *, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QOffscreenSurface {
  pub fn QOffscreenSurface_0<T: QOffscreenSurface_QOffscreenSurface_0>(value: T) -> QOffscreenSurface {
    let rsthis = value.QOffscreenSurface_0();
    return rsthis;
    // return 1;
  }
}

pub trait QOffscreenSurface_QOffscreenSurface_0 {
  fn QOffscreenSurface_0(self) -> QOffscreenSurface;
}
// QOffscreenSurface(QScreen *, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QOffscreenSurface_QOffscreenSurface_0 for (usize,usize) {
  fn QOffscreenSurface_0(self) -> QOffscreenSurface {
    // unsafe{_ZN17QOffscreenSurfaceC2EP7QScreenP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QOffscreenSurfaceC2EP7QScreenP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QOffscreenSurface{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:62
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QOffscreenSurface(QScreen *)

/*
Creates an offscreen surface for the targetScreen with the given parent.

The underlying platform surface is not created until create() is called.

This function was introduced in  Qt 5.10.

See also setScreen() and create().
*/
// QOffscreenSurface(QScreen *) ctx.fn_proto_cpp
impl /*struct*/ QOffscreenSurface {
  pub fn QOffscreenSurface_1<T: QOffscreenSurface_QOffscreenSurface_1>(value: T) -> QOffscreenSurface {
    let rsthis = value.QOffscreenSurface_1();
    return rsthis;
    // return 1;
  }
}

pub trait QOffscreenSurface_QOffscreenSurface_1 {
  fn QOffscreenSurface_1(self) -> QOffscreenSurface;
}
// QOffscreenSurface(QScreen *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QOffscreenSurface_QOffscreenSurface_1 for (usize) {
  fn QOffscreenSurface_1(self) -> QOffscreenSurface {
    // unsafe{_ZN17QOffscreenSurfaceC2EP7QScreen()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QOffscreenSurfaceC2EP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QOffscreenSurface{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QOffscreenSurface()

/*

*/
pub fn DeleteQOffscreenSurface(this :*mut QOffscreenSurface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QOffscreenSurfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qoffscreensurface.h:65
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QSurface::SurfaceType surfaceType() const

/*
Reimplemented from QSurface::surfaceType().

Returns the surface type of the offscreen surface.

The surface type of an offscreen surface is always QSurface::OpenGLSurface.
*/
impl /*struct*/ QOffscreenSurface {
  pub fn surfaceType_0<RetType, T: QOffscreenSurface_surfaceType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.surfaceType_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_surfaceType_0<RetType> {
  fn surfaceType_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_surfaceType_0<i32> for () {
  fn surfaceType_0(self , rsthis: & QOffscreenSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface11surfaceTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void create()

/*
Allocates the platform resources associated with the offscreen surface.

It is at this point that the surface format set using setFormat() gets resolved into an actual native surface.

Call destroy() to free the platform resources if necessary.

Note: Some platforms require this function to be called on the main (GUI) thread.

See also destroy().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn create_0<RetType, T: QOffscreenSurface_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_create_0<RetType> {
  fn create_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_create_0<(/*void*/)> for () {
  fn create_0(self , rsthis: & QOffscreenSurface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QOffscreenSurface6createEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void destroy()

/*
Releases the native platform resources associated with this offscreen surface.

See also create().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn destroy_0<RetType, T: QOffscreenSurface_destroy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroy_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_destroy_0<RetType> {
  fn destroy_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_destroy_0<(/*void*/)> for () {
  fn destroy_0(self , rsthis: & QOffscreenSurface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN17QOffscreenSurface7destroyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if this offscreen surface is valid; otherwise returns false.

The offscreen surface is valid if the platform resources have been successfuly allocated.

See also create().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn isValid_0<RetType, T: QOffscreenSurface_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QOffscreenSurface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFormat(const QSurfaceFormat &)

/*
Sets the offscreen surface format.

The surface format will be resolved in the create() function. Calling this function after create() will not re-resolve the surface format of the native surface.

See also format(), create(), and destroy().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn setFormat_0<RetType, T: QOffscreenSurface_setFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFormat_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_setFormat_0<RetType> {
  fn setFormat_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_setFormat_0<(/*void*/)> for (usize) {
  fn setFormat_0(self , rsthis: & QOffscreenSurface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:73
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSurfaceFormat format() const

/*
Reimplemented from QSurface::format().

Returns the actual format of this offscreen surface.

After the offscreen surface has been created, this function will return the actual surface format of the surface. It might differ from the requested format if the requested format could not be fulfilled by the platform.

See also setFormat(), create(), and requestedFormat().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn format_0<RetType, T: QOffscreenSurface_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_format_0<RetType> {
  fn format_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_format_0<usize> for () {
  fn format_0(self , rsthis: & QOffscreenSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QSurfaceFormat requestedFormat() const

/*
Returns the requested surfaceformat of this offscreen surface.

If the requested format was not supported by the platform implementation, the requestedFormat will differ from the actual offscreen surface format.

This is the value set with setFormat().

See also setFormat() and format().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn requestedFormat_0<RetType, T: QOffscreenSurface_requestedFormat_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestedFormat_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_requestedFormat_0<RetType> {
  fn requestedFormat_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_requestedFormat_0<usize> for () {
  fn requestedFormat_0(self , rsthis: & QOffscreenSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface15requestedFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QSize size() const

/*
Reimplemented from QSurface::size().

Returns the size of the offscreen surface.
*/
impl /*struct*/ QOffscreenSurface {
  pub fn size_0<RetType, T: QOffscreenSurface_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_size_0<RetType> {
  fn size_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_size_0<usize> for () {
  fn size_0(self , rsthis: & QOffscreenSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QScreen * screen() const

/*
Returns the screen to which the offscreen surface is connected.

See also setScreen().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn screen_0<RetType, T: QOffscreenSurface_screen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screen_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_screen_0<RetType> {
  fn screen_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_screen_0<usize> for () {
  fn screen_0(self , rsthis: & QOffscreenSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface6screenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setScreen(QScreen *)

/*
Sets the screen to which the offscreen surface is connected.

If the offscreen surface has been created, it will be recreated on the newScreen.

See also screen().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn setScreen_0<RetType, T: QOffscreenSurface_setScreen_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setScreen_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_setScreen_0<RetType> {
  fn setScreen_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_setScreen_0<(/*void*/)> for (usize) {
  fn setScreen_0(self , rsthis: & QOffscreenSurface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QOffscreenSurface9setScreenEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:83
// index:0
// Public Visibility=Default Availability=Available
// [8] void * nativeHandle() const

/*
Returns an optional native handle to which the offscreen surface is connected.

This function was introduced in  Qt 5.9.

See also setNativeHandle().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn nativeHandle_0<RetType, T: QOffscreenSurface_nativeHandle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeHandle_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_nativeHandle_0<RetType> {
  fn nativeHandle_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_nativeHandle_0<usize> for () {
  fn nativeHandle_0(self , rsthis: & QOffscreenSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QOffscreenSurface12nativeHandleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNativeHandle(void *)

/*
Sets the native handle to which the offscreen surface is connected to handle.

The native handle will be resolved in the create() function. Calling this function after create() will not re-create a native surface.

Note: The interpretation of the native handle is platform specific. Only some platforms will support adopting native handles of offscreen surfaces and platforms that do not implement this support will ignore the handle.

This function was introduced in  Qt 5.9.

See also nativeHandle().
*/
impl /*struct*/ QOffscreenSurface {
  pub fn setNativeHandle_0<RetType, T: QOffscreenSurface_setNativeHandle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNativeHandle_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_setNativeHandle_0<RetType> {
  fn setNativeHandle_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_setNativeHandle_0<(/*void*/)> for (usize) {
  fn setNativeHandle_0(self , rsthis: & QOffscreenSurface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QOffscreenSurface15setNativeHandleEPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qoffscreensurface.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void screenChanged(QScreen *)

/*
This signal is emitted when an offscreen surface's screen changes, either by being set explicitly with setScreen(), or automatically when the window's screen is removed.
*/
impl /*struct*/ QOffscreenSurface {
  pub fn screenChanged_0<RetType, T: QOffscreenSurface_screenChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.screenChanged_0(self);
    // return 1;
  }
}
pub trait QOffscreenSurface_screenChanged_0<RetType> {
  fn screenChanged_0(self , rsthis: & QOffscreenSurface) -> RetType;
}
impl<'a> /*trait*/ QOffscreenSurface_screenChanged_0<(/*void*/)> for (usize) {
  fn screenChanged_0(self , rsthis: & QOffscreenSurface) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QOffscreenSurface13screenChangedEP7QScreen", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
