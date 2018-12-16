

// mod ::gui::QSurfaceFormat
// package qtgui
// /usr/include/qt/QtGui/qsurfaceformat.h
// #include <qsurfaceformat.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QSurfaceFormat)=8
pub struct QSurfaceFormat {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSurfaceFormat_ITF interface {
//    QSurfaceFormat_PTR() *QSurfaceFormat
//}
//func (ptr *QSurfaceFormat) QSurfaceFormat_PTR() *QSurfaceFormat { return ptr }

impl /*struct*/ QSurfaceFormat {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSurfaceFormat {
    return QSurfaceFormat{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSurfaceFormat {
//  type Target = QSurfaceFormatBASE;
//
//  fn deref(&self) -> &QSurfaceFormatBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSurfaceFormatBASE> for QSurfaceFormat {
//  fn as_ref(& self) -> & QSurfaceFormatBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qsurfaceformat.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSurfaceFormat()

/*
Constructs a default initialized QSurfaceFormat.

Note: By default OpenGL 2.0 is requested since this provides the highest grade of portability between platforms and OpenGL implementations.
*/
// QSurfaceFormat() ctx.fn_proto_cpp
impl /*struct*/ QSurfaceFormat {
  pub fn QSurfaceFormat_0<T: QSurfaceFormat_QSurfaceFormat_0>(value: T) -> QSurfaceFormat {
    let rsthis = value.QSurfaceFormat_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSurfaceFormat_QSurfaceFormat_0 {
  fn QSurfaceFormat_0(self) -> QSurfaceFormat;
}
// QSurfaceFormat() ctx.fn_proto_cpp
impl<'a> /*trait*/ QSurfaceFormat_QSurfaceFormat_0 for () {
  fn QSurfaceFormat_0(self) -> QSurfaceFormat {
    // unsafe{_ZN14QSurfaceFormatC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QSurfaceFormatC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSurfaceFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:95
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSurfaceFormat(QSurfaceFormat::FormatOptions)

/*
Constructs a default initialized QSurfaceFormat.

Note: By default OpenGL 2.0 is requested since this provides the highest grade of portability between platforms and OpenGL implementations.
*/
// QSurfaceFormat(QSurfaceFormat::FormatOptions) ctx.fn_proto_cpp
impl /*struct*/ QSurfaceFormat {
  pub fn QSurfaceFormat_1<T: QSurfaceFormat_QSurfaceFormat_1>(value: T) -> QSurfaceFormat {
    let rsthis = value.QSurfaceFormat_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSurfaceFormat_QSurfaceFormat_1 {
  fn QSurfaceFormat_1(self) -> QSurfaceFormat;
}
// QSurfaceFormat(QSurfaceFormat::FormatOptions) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSurfaceFormat_QSurfaceFormat_1 for (i32) {
  fn QSurfaceFormat_1(self) -> QSurfaceFormat {
    // unsafe{_ZN14QSurfaceFormatC2E6QFlagsINS_12FormatOptionEE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QSurfaceFormatC2E6QFlagsINS_12FormatOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSurfaceFormat{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QSurfaceFormat & operator=(const QSurfaceFormat &)

/*

*/
impl /*struct*/ QSurfaceFormat {
  pub fn operator_equal_0<RetType, T: QSurfaceFormat_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QSurfaceFormat) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QSurfaceFormataSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QSurfaceFormat()

/*

*/
pub fn DeleteQSurfaceFormat(this :*mut QSurfaceFormat) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QSurfaceFormatD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qsurfaceformat.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDepthBufferSize(int)

/*
Set the minimum depth buffer size to size.

See also depthBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setDepthBufferSize_0<RetType, T: QSurfaceFormat_setDepthBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDepthBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setDepthBufferSize_0<RetType> {
  fn setDepthBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setDepthBufferSize_0<(/*void*/)> for (i32) {
  fn setDepthBufferSize_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat18setDepthBufferSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int depthBufferSize() const

/*
Returns the depth buffer size.

See also setDepthBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn depthBufferSize_0<RetType, T: QSurfaceFormat_depthBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.depthBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_depthBufferSize_0<RetType> {
  fn depthBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_depthBufferSize_0<i32> for () {
  fn depthBufferSize_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat15depthBufferSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStencilBufferSize(int)

/*
Set the preferred stencil buffer size to size bits.

See also stencilBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setStencilBufferSize_0<RetType, T: QSurfaceFormat_setStencilBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStencilBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setStencilBufferSize_0<RetType> {
  fn setStencilBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setStencilBufferSize_0<(/*void*/)> for (i32) {
  fn setStencilBufferSize_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat20setStencilBufferSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int stencilBufferSize() const

/*
Returns the stencil buffer size in bits.

See also setStencilBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn stencilBufferSize_0<RetType, T: QSurfaceFormat_stencilBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stencilBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_stencilBufferSize_0<RetType> {
  fn stencilBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_stencilBufferSize_0<i32> for () {
  fn stencilBufferSize_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat17stencilBufferSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRedBufferSize(int)

/*
Set the desired size in bits of the red channel of the color buffer.

Note: On Mac OSX, be sure to set the buffer size of all color channels, otherwise this setting will have no effect. If one of the buffer sizes is not set, the current bit-depth of the screen is used.

See also redBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setRedBufferSize_0<RetType, T: QSurfaceFormat_setRedBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRedBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setRedBufferSize_0<RetType> {
  fn setRedBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setRedBufferSize_0<(/*void*/)> for (i32) {
  fn setRedBufferSize_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat16setRedBufferSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] int redBufferSize() const

/*
Get the size in bits of the red channel of the color buffer.

See also setRedBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn redBufferSize_0<RetType, T: QSurfaceFormat_redBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.redBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_redBufferSize_0<RetType> {
  fn redBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_redBufferSize_0<i32> for () {
  fn redBufferSize_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat13redBufferSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGreenBufferSize(int)

/*
Set the desired size in bits of the green channel of the color buffer.

Note: On Mac OSX, be sure to set the buffer size of all color channels, otherwise this setting will have no effect. If one of the buffer sizes is not set, the current bit-depth of the screen is used.

See also greenBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setGreenBufferSize_0<RetType, T: QSurfaceFormat_setGreenBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGreenBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setGreenBufferSize_0<RetType> {
  fn setGreenBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setGreenBufferSize_0<(/*void*/)> for (i32) {
  fn setGreenBufferSize_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat18setGreenBufferSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:109
// index:0
// Public Visibility=Default Availability=Available
// [4] int greenBufferSize() const

/*
Get the size in bits of the green channel of the color buffer.

See also setGreenBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn greenBufferSize_0<RetType, T: QSurfaceFormat_greenBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.greenBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_greenBufferSize_0<RetType> {
  fn greenBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_greenBufferSize_0<i32> for () {
  fn greenBufferSize_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat15greenBufferSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBlueBufferSize(int)

/*
Set the desired size in bits of the blue channel of the color buffer.

Note: On Mac OSX, be sure to set the buffer size of all color channels, otherwise this setting will have no effect. If one of the buffer sizes is not set, the current bit-depth of the screen is used.

See also blueBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setBlueBufferSize_0<RetType, T: QSurfaceFormat_setBlueBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBlueBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setBlueBufferSize_0<RetType> {
  fn setBlueBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setBlueBufferSize_0<(/*void*/)> for (i32) {
  fn setBlueBufferSize_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat17setBlueBufferSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:111
// index:0
// Public Visibility=Default Availability=Available
// [4] int blueBufferSize() const

/*
Get the size in bits of the blue channel of the color buffer.

See also setBlueBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn blueBufferSize_0<RetType, T: QSurfaceFormat_blueBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blueBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_blueBufferSize_0<RetType> {
  fn blueBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_blueBufferSize_0<i32> for () {
  fn blueBufferSize_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat14blueBufferSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAlphaBufferSize(int)

/*
Set the desired size in bits of the alpha channel of the color buffer.

See also alphaBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setAlphaBufferSize_0<RetType, T: QSurfaceFormat_setAlphaBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAlphaBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setAlphaBufferSize_0<RetType> {
  fn setAlphaBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setAlphaBufferSize_0<(/*void*/)> for (i32) {
  fn setAlphaBufferSize_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat18setAlphaBufferSizeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:113
// index:0
// Public Visibility=Default Availability=Available
// [4] int alphaBufferSize() const

/*
Get the size in bits of the alpha channel of the color buffer.

See also setAlphaBufferSize().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn alphaBufferSize_0<RetType, T: QSurfaceFormat_alphaBufferSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.alphaBufferSize_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_alphaBufferSize_0<RetType> {
  fn alphaBufferSize_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_alphaBufferSize_0<i32> for () {
  fn alphaBufferSize_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat15alphaBufferSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:115
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSamples(int)

/*
Set the preferred number of samples per pixel when multisampling is enabled to numSamples. By default, multisampling is disabled.

See also samples().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setSamples_0<RetType, T: QSurfaceFormat_setSamples_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSamples_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setSamples_0<RetType> {
  fn setSamples_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setSamples_0<(/*void*/)> for (i32) {
  fn setSamples_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat10setSamplesEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:116
// index:0
// Public Visibility=Default Availability=Available
// [4] int samples() const

/*
Returns the number of samples per pixel when multisampling is enabled. By default, multisampling is disabled.

See also setSamples().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn samples_0<RetType, T: QSurfaceFormat_samples_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.samples_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_samples_0<RetType> {
  fn samples_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_samples_0<i32> for () {
  fn samples_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat7samplesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSwapBehavior(QSurfaceFormat::SwapBehavior)

/*
Set the swap behavior of the surface.

The swap behavior specifies whether single, double, or triple buffering is desired. The default, DefaultSwapBehavior, gives the default swap behavior of the platform.

See also swapBehavior().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setSwapBehavior_0<RetType, T: QSurfaceFormat_setSwapBehavior_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSwapBehavior_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setSwapBehavior_0<RetType> {
  fn setSwapBehavior_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setSwapBehavior_0<(/*void*/)> for (i32) {
  fn setSwapBehavior_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat15setSwapBehaviorENS_12SwapBehaviorE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:119
// index:0
// Public Visibility=Default Availability=Available
// [4] QSurfaceFormat::SwapBehavior swapBehavior() const

/*
Returns the configured swap behaviour.

See also setSwapBehavior().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn swapBehavior_0<RetType, T: QSurfaceFormat_swapBehavior_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swapBehavior_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_swapBehavior_0<RetType> {
  fn swapBehavior_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_swapBehavior_0<i32> for () {
  fn swapBehavior_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat12swapBehaviorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:121
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasAlpha() const

/*
Returns true if the alpha buffer size is greater than zero.

This means that the surface might be used with per pixel translucency effects.
*/
impl /*struct*/ QSurfaceFormat {
  pub fn hasAlpha_0<RetType, T: QSurfaceFormat_hasAlpha_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasAlpha_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_hasAlpha_0<RetType> {
  fn hasAlpha_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_hasAlpha_0<bool> for () {
  fn hasAlpha_0(self , rsthis: & QSurfaceFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat8hasAlphaEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:123
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProfile(QSurfaceFormat::OpenGLContextProfile)

/*
Sets the desired OpenGL context profile.

This setting is ignored if the requested OpenGL version is less than 3.2.

See also profile().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setProfile_0<RetType, T: QSurfaceFormat_setProfile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProfile_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setProfile_0<RetType> {
  fn setProfile_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setProfile_0<(/*void*/)> for (i32) {
  fn setProfile_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat10setProfileENS_20OpenGLContextProfileE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:124
// index:0
// Public Visibility=Default Availability=Available
// [4] QSurfaceFormat::OpenGLContextProfile profile() const

/*
Get the configured OpenGL context profile.

This setting is ignored if the requested OpenGL version is less than 3.2.

See also setProfile().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn profile_0<RetType, T: QSurfaceFormat_profile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.profile_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_profile_0<RetType> {
  fn profile_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_profile_0<i32> for () {
  fn profile_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat7profileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:126
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRenderableType(QSurfaceFormat::RenderableType)

/*
Sets the desired renderable type.

Chooses between desktop OpenGL, OpenGL ES, and OpenVG.

See also renderableType().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setRenderableType_0<RetType, T: QSurfaceFormat_setRenderableType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRenderableType_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setRenderableType_0<RetType> {
  fn setRenderableType_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setRenderableType_0<(/*void*/)> for (i32) {
  fn setRenderableType_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat17setRenderableTypeENS_14RenderableTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:127
// index:0
// Public Visibility=Default Availability=Available
// [4] QSurfaceFormat::RenderableType renderableType() const

/*
Gets the renderable type.

Chooses between desktop OpenGL, OpenGL ES, and OpenVG.

See also setRenderableType().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn renderableType_0<RetType, T: QSurfaceFormat_renderableType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.renderableType_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_renderableType_0<RetType> {
  fn renderableType_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_renderableType_0<i32> for () {
  fn renderableType_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat14renderableTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMajorVersion(int)

/*
Sets the desired major OpenGL version.

See also majorVersion().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setMajorVersion_0<RetType, T: QSurfaceFormat_setMajorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMajorVersion_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setMajorVersion_0<RetType> {
  fn setMajorVersion_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setMajorVersion_0<(/*void*/)> for (i32) {
  fn setMajorVersion_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat15setMajorVersionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:130
// index:0
// Public Visibility=Default Availability=Available
// [4] int majorVersion() const

/*
Returns the major OpenGL version.

The default version is 2.0.

See also setMajorVersion().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn majorVersion_0<RetType, T: QSurfaceFormat_majorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.majorVersion_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_majorVersion_0<RetType> {
  fn majorVersion_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_majorVersion_0<i32> for () {
  fn majorVersion_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat12majorVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:132
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinorVersion(int)

/*
Sets the desired minor OpenGL version.

The default version is 2.0.

See also minorVersion().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setMinorVersion_0<RetType, T: QSurfaceFormat_setMinorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinorVersion_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setMinorVersion_0<RetType> {
  fn setMinorVersion_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setMinorVersion_0<(/*void*/)> for (i32) {
  fn setMinorVersion_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat15setMinorVersionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:133
// index:0
// Public Visibility=Default Availability=Available
// [4] int minorVersion() const

/*
Returns the minor OpenGL version.

See also setMinorVersion().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn minorVersion_0<RetType, T: QSurfaceFormat_minorVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minorVersion_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_minorVersion_0<RetType> {
  fn minorVersion_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_minorVersion_0<i32> for () {
  fn minorVersion_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat12minorVersionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVersion(int, int)

/*
Sets the desired major and minor OpenGL versions.

The default version is 2.0.

See also version().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setVersion_0<RetType, T: QSurfaceFormat_setVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVersion_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setVersion_0<RetType> {
  fn setVersion_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setVersion_0<(/*void*/)> for (i32,i32) {
  fn setVersion_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat10setVersionEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:138
// index:0
// Public Visibility=Default Availability=Available
// [1] bool stereo() const

/*
Returns true if stereo buffering is enabled; otherwise returns false. Stereo buffering is disabled by default.

See also setStereo().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn stereo_0<RetType, T: QSurfaceFormat_stereo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stereo_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_stereo_0<RetType> {
  fn stereo_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_stereo_0<bool> for () {
  fn stereo_0(self , rsthis: & QSurfaceFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat6stereoEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStereo(bool)

/*
If enable is true enables stereo buffering; otherwise disables stereo buffering.

Stereo buffering is disabled by default.

Stereo buffering provides extra color buffers to generate left-eye and right-eye images.

See also stereo().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setStereo_0<RetType, T: QSurfaceFormat_setStereo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStereo_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setStereo_0<RetType> {
  fn setStereo_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setStereo_0<(/*void*/)> for (bool) {
  fn setStereo_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat9setStereoEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOption(QSurfaceFormat::FormatOptions)

/*
Sets the format option option if on is true; otherwise, clears the option.

This function was introduced in  Qt 5.3.

See also setOptions(), options(), and testOption().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setOption_0<RetType, T: QSurfaceFormat_setOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setOption_0<RetType> {
  fn setOption_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setOption_0<(/*void*/)> for (i32) {
  fn setOption_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat9setOptionE6QFlagsINS_12FormatOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:147
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setOption(QSurfaceFormat::FormatOption, bool)

/*
Sets the format option option if on is true; otherwise, clears the option.

This function was introduced in  Qt 5.3.

See also setOptions(), options(), and testOption().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setOption_1<RetType, T: QSurfaceFormat_setOption_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOption_1(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setOption_1<RetType> {
  fn setOption_1(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setOption_1<(/*void*/)> for (i32,bool) {
  fn setOption_1(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat9setOptionENS_12FormatOptionEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool testOption(QSurfaceFormat::FormatOptions) const

/*
Returns true if the format option option is set; otherwise returns false.

This function was introduced in  Qt 5.3.

See also options().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn testOption_0<RetType, T: QSurfaceFormat_testOption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_testOption_0<RetType> {
  fn testOption_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_testOption_0<bool> for (i32) {
  fn testOption_0(self , rsthis: & QSurfaceFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat10testOptionE6QFlagsINS_12FormatOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:148
// index:1
// Public Visibility=Default Availability=Available
// [1] bool testOption(QSurfaceFormat::FormatOption) const

/*
Returns true if the format option option is set; otherwise returns false.

This function was introduced in  Qt 5.3.

See also options().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn testOption_1<RetType, T: QSurfaceFormat_testOption_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.testOption_1(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_testOption_1<RetType> {
  fn testOption_1(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_testOption_1<bool> for (i32) {
  fn testOption_1(self , rsthis: & QSurfaceFormat) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat10testOptionENS_12FormatOptionE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setOptions(QSurfaceFormat::FormatOptions)

/*
Sets the format options to options.

This function was introduced in  Qt 5.3.

See also options() and testOption().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setOptions_0<RetType, T: QSurfaceFormat_setOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setOptions_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setOptions_0<RetType> {
  fn setOptions_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setOptions_0<(/*void*/)> for (i32) {
  fn setOptions_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat10setOptionsE6QFlagsINS_12FormatOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:149
// index:0
// Public Visibility=Default Availability=Available
// [4] QSurfaceFormat::FormatOptions options() const

/*
Returns the currently set format options.

This function was introduced in  Qt 5.3.

See also setOption(), setOptions(), and testOption().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn options_0<RetType, T: QSurfaceFormat_options_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.options_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_options_0<RetType> {
  fn options_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_options_0<i32> for () {
  fn options_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat7optionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:151
// index:0
// Public Visibility=Default Availability=Available
// [4] int swapInterval() const

/*
Returns the swap interval.

This function was introduced in  Qt 5.3.

See also setSwapInterval().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn swapInterval_0<RetType, T: QSurfaceFormat_swapInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swapInterval_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_swapInterval_0<RetType> {
  fn swapInterval_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_swapInterval_0<i32> for () {
  fn swapInterval_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat12swapIntervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSwapInterval(int)

/*
Sets the preferred swap interval. The swap interval specifies the minimum number of video frames that are displayed before a buffer swap occurs. This can be used to sync the GL drawing into a window to the vertical refresh of the screen.

Setting an interval value of 0 will turn the vertical refresh syncing off, any value higher than 0 will turn the vertical syncing on. Setting interval to a higher value, for example 10, results in having 10 vertical retraces between every buffer swap.

The default interval is 1.

Changing the swap interval may not be supported by the underlying platform. In this case, the request will be silently ignored.

This function was introduced in  Qt 5.3.

See also swapInterval().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setSwapInterval_0<RetType, T: QSurfaceFormat_setSwapInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSwapInterval_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setSwapInterval_0<RetType> {
  fn setSwapInterval_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setSwapInterval_0<(/*void*/)> for (i32) {
  fn setSwapInterval_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat15setSwapIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:154
// index:0
// Public Visibility=Default Availability=Available
// [4] QSurfaceFormat::ColorSpace colorSpace() const

/*
Returns the color space.

This function was introduced in  Qt 5.10.

See also setColorSpace().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn colorSpace_0<RetType, T: QSurfaceFormat_colorSpace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.colorSpace_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_colorSpace_0<RetType> {
  fn colorSpace_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_colorSpace_0<i32> for () {
  fn colorSpace_0(self , rsthis: & QSurfaceFormat) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QSurfaceFormat10colorSpaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColorSpace(QSurfaceFormat::ColorSpace)

/*
Sets the preferred colorSpace.

For example, this allows requesting windows with default framebuffers that are sRGB-capable on platforms that support it.

Note: When the requested color space is not supported by the platform, the request is ignored. Query the QSurfaceFormat after window creation to verify if the color space request could be honored or not.

Note: This setting controls if the default framebuffer of the window is capable of updates and blending in a given color space. It does not change applications' output by itself. The applications' rendering code will still have to opt in via the appropriate OpenGL calls to enable updates and blending to be performed in the given color space instead of using the standard linear operations.

This function was introduced in  Qt 5.10.

See also colorSpace().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setColorSpace_0<RetType, T: QSurfaceFormat_setColorSpace_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColorSpace_0(self);
    // return 1;
  }
}
pub trait QSurfaceFormat_setColorSpace_0<RetType> {
  fn setColorSpace_0(self , rsthis: & QSurfaceFormat) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setColorSpace_0<(/*void*/)> for (i32) {
  fn setColorSpace_0(self , rsthis: & QSurfaceFormat) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat13setColorSpaceENS_10ColorSpaceE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:157
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDefaultFormat(const QSurfaceFormat &)

/*
Sets the global default surface format.

This format is used by default in QOpenGLContext, QWindow, QOpenGLWidget and similar classes.

It can always be overridden on a per-instance basis by using the class in question's own setFormat() function. However, it is often more convenient to set the format for all windows once at the start of the application. It also guarantees proper behavior in cases where shared contexts are required, because settings the format via this function guarantees that all contexts and surfaces, even the ones created internally by Qt, will use the same format.

Note: When setting Qt::AA_ShareOpenGLContexts, it is strongly recommended to place the call to this function before the construction of the QGuiApplication or QApplication. Otherwise format will not be applied to the global share context and therefore issues may arise with context sharing afterwards.

This function was introduced in  Qt 5.4.

See also defaultFormat().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn setDefaultFormat_0<RetType, T: QSurfaceFormat_setDefaultFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultFormat_0();
    // return 1;
  }
}
pub trait QSurfaceFormat_setDefaultFormat_0<RetType> {
  fn setDefaultFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_setDefaultFormat_0<(/*void*/)> for (usize) {
  fn setDefaultFormat_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat16setDefaultFormatERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qsurfaceformat.h:158
// index:0
// Public static Visibility=Default Availability=Available
// [8] QSurfaceFormat defaultFormat()

/*
Returns the global default surface format.

When setDefaultFormat() is not called, this is a default-constructed QSurfaceFormat.

This function was introduced in  Qt 5.4.

See also setDefaultFormat().
*/
impl /*struct*/ QSurfaceFormat {
  pub fn defaultFormat_0<RetType, T: QSurfaceFormat_defaultFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultFormat_0();
    // return 1;
  }
}
pub trait QSurfaceFormat_defaultFormat_0<RetType> {
  fn defaultFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSurfaceFormat_defaultFormat_0<usize> for () {
  fn defaultFormat_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QSurfaceFormat13defaultFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*


*/
pub type QSurfaceFormat__FormatOption = i32;
// 
pub const QSurfaceFormat__StereoBuffers :QSurfaceFormat__FormatOption = 1;
// 
pub const QSurfaceFormat__DebugContext :QSurfaceFormat__FormatOption = 2;
// 
pub const QSurfaceFormat__DeprecatedFunctions :QSurfaceFormat__FormatOption = 4;
// 
pub const QSurfaceFormat__ResetNotification :QSurfaceFormat__FormatOption = 8;
pub fn QSurfaceFormat_FormatOptionItemName(val: i32) ->String {
  match val {
     QSurfaceFormat__StereoBuffers => // 1
     {return String::from("StereoBuffers");}
     QSurfaceFormat__DebugContext => // 2
     {return String::from("DebugContext");}
     QSurfaceFormat__DeprecatedFunctions => // 4
     {return String::from("DeprecatedFunctions");}
     QSurfaceFormat__ResetNotification => // 8
     {return String::from("ResetNotification");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurfaceFormat_FormatOptionItemName_s(val: i32) ->String {
  //var nilthis *QSurfaceFormat
  //return nilthis.FormatOptionItemName(val);
  return QSurfaceFormat_FormatOptionItemName(val);
}


/*
This enum is used by QSurfaceFormat to specify the swap behaviour of a surface. The swap behaviour is mostly transparent to the application, but it affects factors such as rendering latency and throughput.


*/
pub type QSurfaceFormat__SwapBehavior = i32;
// The default, unspecified swap behaviour of the platform.
pub const QSurfaceFormat__DefaultSwapBehavior :QSurfaceFormat__SwapBehavior = 0;
// Used to request single buffering, which might result in flickering when OpenGL rendering is done directly to screen without an intermediate offscreen buffer.
pub const QSurfaceFormat__SingleBuffer :QSurfaceFormat__SwapBehavior = 1;
// This is typically the default swap behaviour on desktop platforms, consisting of one back buffer and one front buffer. Rendering is done to the back buffer, and then the back buffer and front buffer are swapped, or the contents of the back buffer are copied to the front buffer, depending on the implementation.
pub const QSurfaceFormat__DoubleBuffer :QSurfaceFormat__SwapBehavior = 2;
// This swap behaviour is sometimes used in order to decrease the risk of skipping a frame when the rendering rate is just barely keeping up with the screen refresh rate. Depending on the platform it might also lead to slightly more efficient use of the GPU due to improved pipelining behaviour. Triple buffering comes at the cost of an extra frame of memory usage and latency, and might not be supported depending on the underlying platform.
pub const QSurfaceFormat__TripleBuffer :QSurfaceFormat__SwapBehavior = 3;
pub fn QSurfaceFormat_SwapBehaviorItemName(val: i32) ->String {
  match val {
     QSurfaceFormat__DefaultSwapBehavior => // 0
     {return String::from("DefaultSwapBehavior");}
     QSurfaceFormat__SingleBuffer => // 1
     {return String::from("SingleBuffer");}
     QSurfaceFormat__DoubleBuffer => // 2
     {return String::from("DoubleBuffer");}
     QSurfaceFormat__TripleBuffer => // 3
     {return String::from("TripleBuffer");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurfaceFormat_SwapBehaviorItemName_s(val: i32) ->String {
  //var nilthis *QSurfaceFormat
  //return nilthis.SwapBehaviorItemName(val);
  return QSurfaceFormat_SwapBehaviorItemName(val);
}


/*
This enum specifies the rendering backend for the surface.


*/
pub type QSurfaceFormat__RenderableType = i32;
// 
pub const QSurfaceFormat__DefaultRenderableType :QSurfaceFormat__RenderableType = 0;
// 
pub const QSurfaceFormat__OpenGL :QSurfaceFormat__RenderableType = 1;
// 
pub const QSurfaceFormat__OpenGLES :QSurfaceFormat__RenderableType = 2;
// 
pub const QSurfaceFormat__OpenVG :QSurfaceFormat__RenderableType = 4;
pub fn QSurfaceFormat_RenderableTypeItemName(val: i32) ->String {
  match val {
     QSurfaceFormat__DefaultRenderableType => // 0
     {return String::from("DefaultRenderableType");}
     QSurfaceFormat__OpenGL => // 1
     {return String::from("OpenGL");}
     QSurfaceFormat__OpenGLES => // 2
     {return String::from("OpenGLES");}
     QSurfaceFormat__OpenVG => // 4
     {return String::from("OpenVG");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurfaceFormat_RenderableTypeItemName_s(val: i32) ->String {
  //var nilthis *QSurfaceFormat
  //return nilthis.RenderableTypeItemName(val);
  return QSurfaceFormat_RenderableTypeItemName(val);
}


/*
This enum is used to specify the OpenGL context profile, in conjunction with QSurfaceFormat::setMajorVersion() and QSurfaceFormat::setMinorVersion().

Profiles are exposed in OpenGL 3.2 and above, and are used to choose between a restricted core profile, and a compatibility profile which might contain deprecated support functionality.

Note that the core profile might still contain functionality that is deprecated and scheduled for removal in a higher version. To get access to the deprecated functionality for the core profile in the set OpenGL version you can use the QSurfaceFormat format option QSurfaceFormat::DeprecatedFunctions.


*/
pub type QSurfaceFormat__OpenGLContextProfile = i32;
// 
pub const QSurfaceFormat__NoProfile :QSurfaceFormat__OpenGLContextProfile = 0;
// 
pub const QSurfaceFormat__CoreProfile :QSurfaceFormat__OpenGLContextProfile = 1;
// Functionality from earlier OpenGL versions is available.
pub const QSurfaceFormat__CompatibilityProfile :QSurfaceFormat__OpenGLContextProfile = 2;
pub fn QSurfaceFormat_OpenGLContextProfileItemName(val: i32) ->String {
  match val {
     QSurfaceFormat__NoProfile => // 0
     {return String::from("NoProfile");}
     QSurfaceFormat__CoreProfile => // 1
     {return String::from("CoreProfile");}
     QSurfaceFormat__CompatibilityProfile => // 2
     {return String::from("CompatibilityProfile");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurfaceFormat_OpenGLContextProfileItemName_s(val: i32) ->String {
  //var nilthis *QSurfaceFormat
  //return nilthis.OpenGLContextProfileItemName(val);
  return QSurfaceFormat_OpenGLContextProfileItemName(val);
}


/*
This enum is used to specify the preferred color space, controlling if the window's associated default framebuffer is able to do updates and blending in a given encoding instead of the standard linear operations.


*/
pub type QSurfaceFormat__ColorSpace = i32;
// The default, unspecified color space.
pub const QSurfaceFormat__DefaultColorSpace :QSurfaceFormat__ColorSpace = 0;
// When GL_ARB_framebuffer_sRGB or GL_EXT_framebuffer_sRGB is supported by the platform and this value is set, the window will be created with an sRGB-capable default framebuffer. Note that some platforms may return windows with a sRGB-capable default framebuffer even when not requested explicitly.
pub const QSurfaceFormat__sRGBColorSpace :QSurfaceFormat__ColorSpace = 1;
pub fn QSurfaceFormat_ColorSpaceItemName(val: i32) ->String {
  match val {
     QSurfaceFormat__DefaultColorSpace => // 0
     {return String::from("DefaultColorSpace");}
     QSurfaceFormat__sRGBColorSpace => // 1
     {return String::from("sRGBColorSpace");}
  _ => {return format!("{}", val);}
}
}
pub fn QSurfaceFormat_ColorSpaceItemName_s(val: i32) ->String {
  //var nilthis *QSurfaceFormat
  //return nilthis.ColorSpaceItemName(val);
  return QSurfaceFormat_ColorSpaceItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
