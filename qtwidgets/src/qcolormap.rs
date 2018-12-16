

// mod ::widgets::QColormap
// package qtwidgets
// /usr/include/qt/QtWidgets/qcolormap.h
// #include <qcolormap.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 24
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



/*

*/
#[derive(Default)] // class sizeof(QColormap)=8
pub struct QColormap {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QColormap_ITF interface {
//    QColormap_PTR() *QColormap
//}
//func (ptr *QColormap) QColormap_PTR() *QColormap { return ptr }

impl /*struct*/ QColormap {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QColormap {
    return QColormap{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QColormap {
//  type Target = QColormapBASE;
//
//  fn deref(&self) -> &QColormapBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QColormapBASE> for QColormap {
//  fn as_ref(& self) -> & QColormapBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qcolormap.h:60
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void initialize()

/*

*/
impl /*struct*/ QColormap {
  pub fn initialize_0<RetType, T: QColormap_initialize_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.initialize_0();
    // return 1;
  }
}
pub trait QColormap_initialize_0<RetType> {
  fn initialize_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColormap_initialize_0<(/*void*/)> for () {
  fn initialize_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QColormap10initializeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:61
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void cleanup()

/*

*/
impl /*struct*/ QColormap {
  pub fn cleanup_0<RetType, T: QColormap_cleanup_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_0();
    // return 1;
  }
}
pub trait QColormap_cleanup_0<RetType> {
  fn cleanup_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColormap_cleanup_0<(/*void*/)> for () {
  fn cleanup_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QColormap7cleanupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:63
// index:0
// Public static Visibility=Default Availability=Available
// [8] QColormap instance(int)

/*
Returns the colormap for the specified screen. If screen is -1, this function returns the colormap for the default screen.
*/
impl /*struct*/ QColormap {
  pub fn instance_0<RetType, T: QColormap_instance_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.instance_0();
    // return 1;
  }
}
pub trait QColormap_instance_0<RetType> {
  fn instance_0(self ) -> RetType;
}
impl<'a> /*trait*/ QColormap_instance_0<usize> for (i32) {
  fn instance_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QColormap8instanceEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QColormap()

/*

*/
pub fn DeleteQColormap(this :*mut QColormap) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QColormapD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qcolormap.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QColormap & operator=(const QColormap &)

/*

*/
impl /*struct*/ QColormap {
  pub fn operator_equal_0<RetType, T: QColormap_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QColormap_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QColormap) -> RetType;
}
impl<'a> /*trait*/ QColormap_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QColormap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QColormapaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:70
// index:0
// Public Visibility=Default Availability=Available
// [4] QColormap::Mode mode() const

/*
Returns the mode of this colormap.

See also QColormap::Mode.
*/
impl /*struct*/ QColormap {
  pub fn mode_0<RetType, T: QColormap_mode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mode_0(self);
    // return 1;
  }
}
pub trait QColormap_mode_0<RetType> {
  fn mode_0(self , rsthis: & QColormap) -> RetType;
}
impl<'a> /*trait*/ QColormap_mode_0<i32> for () {
  fn mode_0(self , rsthis: & QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QColormap4modeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:72
// index:0
// Public Visibility=Default Availability=Available
// [4] int depth() const

/*
Returns the depth of the device.

See also size().
*/
impl /*struct*/ QColormap {
  pub fn depth_0<RetType, T: QColormap_depth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.depth_0(self);
    // return 1;
  }
}
pub trait QColormap_depth_0<RetType> {
  fn depth_0(self , rsthis: & QColormap) -> RetType;
}
impl<'a> /*trait*/ QColormap_depth_0<i32> for () {
  fn depth_0(self , rsthis: & QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QColormap5depthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the size of the colormap for Indexed and Gray modes; Returns -1 for Direct mode.

See also colormap().
*/
impl /*struct*/ QColormap {
  pub fn size_0<RetType, T: QColormap_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QColormap_size_0<RetType> {
  fn size_0(self , rsthis: & QColormap) -> RetType;
}
impl<'a> /*trait*/ QColormap_size_0<i32> for () {
  fn size_0(self , rsthis: & QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QColormap4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] uint pixel(const QColor &) const

/*
Returns a device dependent pixel value for the color.

See also colorAt().
*/
impl /*struct*/ QColormap {
  pub fn pixel_0<RetType, T: QColormap_pixel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pixel_0(self);
    // return 1;
  }
}
pub trait QColormap_pixel_0<RetType> {
  fn pixel_0(self , rsthis: & QColormap) -> RetType;
}
impl<'a> /*trait*/ QColormap_pixel_0<u32> for (usize) {
  fn pixel_0(self , rsthis: & QColormap) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QColormap5pixelERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qcolormap.h:76
// index:0
// Public Visibility=Default Availability=Available
// [16] const QColor colorAt(uint) const

/*
Returns a QColor for the pixel.

See also pixel().
*/
impl /*struct*/ QColormap {
  pub fn colorAt_0<RetType, T: QColormap_colorAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorAt_0(self);
    // return 1;
  }
}
pub trait QColormap_colorAt_0<RetType> {
  fn colorAt_0(self , rsthis: & QColormap) -> RetType;
}
impl<'a> /*trait*/ QColormap_colorAt_0<usize> for (u32) {
  fn colorAt_0(self , rsthis: & QColormap) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QColormap7colorAtEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum describes how QColormap maps device independent RGB values to device dependent pixel values.


*/
pub type QColormap__Mode = i32;
// Pixel values are derived directly from the RGB values, also known as "True Color."
pub const QColormap__Direct :QColormap__Mode = 0;
// Pixel values represent indexes into a vector of available colors, i.e. QColormap uses the index of the color that most closely matches an RGB value.
pub const QColormap__Indexed :QColormap__Mode = 1;
// Similar to Indexed, pixel values represent a vector of available gray tones. QColormap uses the index of the gray tone that most closely matches the computed gray tone of an RGB value.
pub const QColormap__Gray :QColormap__Mode = 2;
pub fn QColormap_ModeItemName(val: i32) ->String {
  match val {
     QColormap__Direct => // 0
     {return String::from("Direct");}
     QColormap__Indexed => // 1
     {return String::from("Indexed");}
     QColormap__Gray => // 2
     {return String::from("Gray");}
  _ => {return format!("{}", val);}
}
}
pub fn QColormap_ModeItemName_s(val: i32) ->String {
  //var nilthis *QColormap
  //return nilthis.ModeItemName(val);
  return QColormap_ModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
