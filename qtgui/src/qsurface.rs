

// mod ::gui::QSurface
// package qtgui
// /usr/include/qt/QtGui/qsurface.h
// #include <qsurface.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 44
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
#[derive(Default)] // class sizeof(QSurface)=24
pub struct QSurface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSurface_ITF interface {
//    QSurface_PTR() *QSurface
//}
//func (ptr *QSurface) QSurface_PTR() *QSurface { return ptr }

impl /*struct*/ QSurface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSurface {
    return QSurface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSurface {
//  type Target = QSurfaceBASE;
//
//  fn deref(&self) -> &QSurfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSurfaceBASE> for QSurface {
//  fn as_ref(& self) -> & QSurfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qsurface.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSurface()

/*

*/
pub fn DeleteQSurface(this :*mut QSurface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QSurfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qsurface.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] QSurface::SurfaceClass surfaceClass() const

/*
Returns the surface class of this surface.
*/
impl /*struct*/ QSurface {
  pub fn surfaceClass_0<RetType, T: QSurface_surfaceClass_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.surfaceClass_0(self);
    // return 1;
  }
}
pub trait QSurface_surfaceClass_0<RetType> {
  fn surfaceClass_0(self , rsthis: & QSurface) -> RetType;
}
impl<'a> /*trait*/ QSurface_surfaceClass_0<i32> for () {
  fn surfaceClass_0(self , rsthis: & QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSurface12surfaceClassEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurface.h:76
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSurfaceFormat format() const

/*
Returns the format of the surface.
*/
impl /*struct*/ QSurface {
  pub fn format_0<RetType, T: QSurface_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QSurface_format_0<RetType> {
  fn format_0(self , rsthis: & QSurface) -> RetType;
}
impl<'a> /*trait*/ QSurface_format_0<usize> for () {
  fn format_0(self , rsthis: & QSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSurface6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurface.h:79
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QSurface::SurfaceType surfaceType() const

/*
Returns the type of the surface.
*/
impl /*struct*/ QSurface {
  pub fn surfaceType_0<RetType, T: QSurface_surfaceType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.surfaceType_0(self);
    // return 1;
  }
}
pub trait QSurface_surfaceType_0<RetType> {
  fn surfaceType_0(self , rsthis: & QSurface) -> RetType;
}
impl<'a> /*trait*/ QSurface_surfaceType_0<i32> for () {
  fn surfaceType_0(self , rsthis: & QSurface) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSurface11surfaceTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurface.h:80
// index:0
// Public Visibility=Default Availability=Available
// [1] bool supportsOpenGL() const

/*
Returns true if the surface is OpenGL compatible and can be used in conjunction with QOpenGLContext; otherwise returns false.

This function was introduced in  Qt 5.3.
*/
impl /*struct*/ QSurface {
  pub fn supportsOpenGL_0<RetType, T: QSurface_supportsOpenGL_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.supportsOpenGL_0(self);
    // return 1;
  }
}
pub trait QSurface_supportsOpenGL_0<RetType> {
  fn supportsOpenGL_0(self , rsthis: & QSurface) -> RetType;
}
impl<'a> /*trait*/ QSurface_supportsOpenGL_0<bool> for () {
  fn supportsOpenGL_0(self , rsthis: & QSurface) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSurface14supportsOpenGLEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurface.h:82
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QSize size() const

/*
Returns the size of the surface in pixels.
*/
impl /*struct*/ QSurface {
  pub fn size_0<RetType, T: QSurface_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QSurface_size_0<RetType> {
  fn size_0(self , rsthis: & QSurface) -> RetType;
}
impl<'a> /*trait*/ QSurface_size_0<usize> for () {
  fn size_0(self , rsthis: & QSurface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QSurface4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurface.h:85
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QSurface(QSurface::SurfaceClass)

/*
Creates a surface with the given type.
*/
// QSurface(QSurface::SurfaceClass) ctx.fn_proto_cpp
impl /*struct*/ QSurface {
  pub fn QSurface_0<T: QSurface_QSurface_0>(value: T) -> QSurface {
    let rsthis = value.QSurface_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSurface_QSurface_0 {
  fn QSurface_0(self) -> QSurface;
}
// QSurface(QSurface::SurfaceClass) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSurface_QSurface_0 for (i32) {
  fn QSurface_0(self) -> QSurface {
    // unsafe{_ZN8QSurfaceC2ENS_12SurfaceClassE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QSurfaceC2ENS_12SurfaceClassE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSurface{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


/*
The SurfaceClass enum describes the actual subclass of the surface.


*/
pub type QSurface__SurfaceClass = i32;
// The surface is an instance of QWindow.
pub const QSurface__Window :QSurface__SurfaceClass = 0;
// The surface is an instance of QOffscreenSurface.
pub const QSurface__Offscreen :QSurface__SurfaceClass = 1;
pub fn QSurface_SurfaceClassItemName(val: i32) ->String {
  match val {
     QSurface__Window => // 0
     {return String::from("Window");}
     QSurface__Offscreen => // 1
     {return String::from("Offscreen");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurface_SurfaceClassItemName_s(val: i32) ->String {
  //var nilthis *QSurface
  //return nilthis.SurfaceClassItemName(val);
  return QSurface_SurfaceClassItemName(val);
}


/*
The SurfaceType enum describes what type of surface this is.


*/
pub type QSurface__SurfaceType = i32;
// The surface is is composed of pixels and can be rendered to using a software rasterizer like Qt's raster paint engine.
pub const QSurface__RasterSurface :QSurface__SurfaceType = 0;
// The surface is an OpenGL compatible surface and can be used in conjunction with QOpenGLContext.
pub const QSurface__OpenGLSurface :QSurface__SurfaceType = 1;
// The surface can be rendered to using a software rasterizer, and also supports OpenGL. This surface type is intended for internal Qt use, and requires the use of private API.
pub const QSurface__RasterGLSurface :QSurface__SurfaceType = 2;
// The surface is an OpenVG compatible surface and can be used in conjunction with OpenVG contexts.
pub const QSurface__OpenVGSurface :QSurface__SurfaceType = 3;
// The surface is a Vulkan compatible surface and can be used in conjunction with the Vulkan graphics API.
pub const QSurface__VulkanSurface :QSurface__SurfaceType = 4;
pub fn QSurface_SurfaceTypeItemName(val: i32) ->String {
  match val {
     QSurface__RasterSurface => // 0
     {return String::from("RasterSurface");}
     QSurface__OpenGLSurface => // 1
     {return String::from("OpenGLSurface");}
     QSurface__RasterGLSurface => // 2
     {return String::from("RasterGLSurface");}
     QSurface__OpenVGSurface => // 3
     {return String::from("OpenVGSurface");}
     QSurface__VulkanSurface => // 4
     {return String::from("VulkanSurface");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurface_SurfaceTypeItemName_s(val: i32) ->String {
  //var nilthis *QSurface
  //return nilthis.SurfaceTypeItemName(val);
  return QSurface_SurfaceTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
