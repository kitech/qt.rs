

// mod ::gui::QPen
// package qtgui
// /usr/include/qt/QtGui/qpen.h
// #include <qpen.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QPen)=8
pub struct QPen {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPen_ITF interface {
//    QPen_PTR() *QPen
//}
//func (ptr *QPen) QPen_PTR() *QPen { return ptr }

impl /*struct*/ QPen {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPen {
    return QPen{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPen {
//  type Target = QPenBASE;
//
//  fn deref(&self) -> &QPenBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPenBASE> for QPen {
//  fn as_ref(& self) -> & QPenBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qpen.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPen()

/*
Constructs a default black solid line pen with 1 width.
*/
// QPen() ctx.fn_proto_cpp
impl /*struct*/ QPen {
  pub fn QPen_0<T: QPen_QPen_0>(value: T) -> QPen {
    let rsthis = value.QPen_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPen_QPen_0 {
  fn QPen_0(self) -> QPen;
}
// QPen() ctx.fn_proto_cpp
impl<'a> /*trait*/ QPen_QPen_0 for () {
  fn QPen_0(self) -> QPen {
    // unsafe{_ZN4QPenC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QPenC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:64
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPen(Qt::PenStyle)

/*
Constructs a default black solid line pen with 1 width.
*/
// QPen(Qt::PenStyle) ctx.fn_proto_cpp
impl /*struct*/ QPen {
  pub fn QPen_1<T: QPen_QPen_1>(value: T) -> QPen {
    let rsthis = value.QPen_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPen_QPen_1 {
  fn QPen_1(self) -> QPen;
}
// QPen(Qt::PenStyle) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPen_QPen_1 for (i32) {
  fn QPen_1(self) -> QPen {
    // unsafe{_ZN4QPenC2EN2Qt8PenStyleE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QPenC2EN2Qt8PenStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:65
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QPen(const QColor &)

/*
Constructs a default black solid line pen with 1 width.
*/
// QPen(const QColor &) ctx.fn_proto_cpp
impl /*struct*/ QPen {
  pub fn QPen_2<T: QPen_QPen_2>(value: T) -> QPen {
    let rsthis = value.QPen_2();
    return rsthis;
    // return 1;
  }
}

pub trait QPen_QPen_2 {
  fn QPen_2(self) -> QPen;
}
// QPen(const QColor &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPen_QPen_2 for (usize) {
  fn QPen_2(self) -> QPen {
    // unsafe{_ZN4QPenC2ERK6QColor()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QPenC2ERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:66
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QPen(const QBrush &, qreal, Qt::PenStyle, Qt::PenCapStyle, Qt::PenJoinStyle)

/*
Constructs a default black solid line pen with 1 width.
*/
// QPen(const QBrush &, qreal, Qt::PenStyle, Qt::PenCapStyle, Qt::PenJoinStyle) ctx.fn_proto_cpp
impl /*struct*/ QPen {
  pub fn QPen_3<T: QPen_QPen_3>(value: T) -> QPen {
    let rsthis = value.QPen_3();
    return rsthis;
    // return 1;
  }
}

pub trait QPen_QPen_3 {
  fn QPen_3(self) -> QPen;
}
// QPen(const QBrush &, qreal, Qt::PenStyle, Qt::PenCapStyle, Qt::PenJoinStyle) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPen_QPen_3 for (usize,f64,i32,i32,i32) {
  fn QPen_3(self) -> QPen {
    // unsafe{_ZN4QPenC2ERK6QBrushdN2Qt8PenStyleENS3_11PenCapStyleENS3_12PenJoinStyleE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QPenC2ERK6QBrushdN2Qt8PenStyleENS3_11PenCapStyleENS3_12PenJoinStyleE", 5,qtrt::FFITY_POINTER,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPen{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QPen()

/*

*/
pub fn DeleteQPen(this :*mut QPen) {
    // let rv = qtrt::InvokeQtFunc6("_ZN4QPenD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qpen.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QPen & operator=(const QPen &)

/*

*/
impl /*struct*/ QPen {
  pub fn operator_equal_0<RetType, T: QPen_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QPen_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QPen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QPenaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:76
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QPen & operator=(QPen &&)

/*

*/
impl /*struct*/ QPen {
  pub fn operator_equal_1<RetType, T: QPen_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QPen_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QPen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QPenaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QPen &)

/*
Swaps pen other with this pen. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QPen {
  pub fn swap_0<RetType, T: QPen_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QPen_swap_0<RetType> {
  fn swap_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::PenStyle style() const

/*
Returns the pen style.

See also setStyle() and Pen Style.
*/
impl /*struct*/ QPen {
  pub fn style_0<RetType, T: QPen_style_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.style_0(self);
    // return 1;
  }
}
pub trait QPen_style_0<RetType> {
  fn style_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_style_0<i32> for () {
  fn style_0(self , rsthis: & QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen5styleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStyle(Qt::PenStyle)

/*
Sets the pen style to the given style.

See the Qt::PenStyle documentation for a list of the available styles. Since Qt 4.1 it is also possible to specify a custom dash pattern using the setDashPattern() function which implicitly converts the style of the pen to Qt::CustomDashLine.

Note: This function resets the dash offset to zero.

See also style() and Pen Style.
*/
impl /*struct*/ QPen {
  pub fn setStyle_0<RetType, T: QPen_setStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStyle_0(self);
    // return 1;
  }
}
pub trait QPen_setStyle_0<RetType> {
  fn setStyle_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setStyle_0<(/*void*/)> for (i32) {
  fn setStyle_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen8setStyleEN2Qt8PenStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal dashOffset() const

/*
Returns the dash offset for the pen.

See also setDashOffset().
*/
impl /*struct*/ QPen {
  pub fn dashOffset_0<RetType, T: QPen_dashOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dashOffset_0(self);
    // return 1;
  }
}
pub trait QPen_dashOffset_0<RetType> {
  fn dashOffset_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_dashOffset_0<f64> for () {
  fn dashOffset_0(self , rsthis: & QPen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen10dashOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDashOffset(qreal)

/*
Sets the dash offset (the starting point on the dash pattern) for this pen to the offset specified. The offset is measured in terms of the units used to specify the dash pattern.


 For example, a pattern where each stroke is four units long, followed by a gap of two units, will begin with the stroke when drawn as a line.However, if the dash offset is set to 4.0, any line drawn will begin with the gap. Values of the offset up to 4.0 will cause part of the stroke to be drawn first, and values of the offset between 4.0 and 6.0 will cause the line to begin with part of the gap.



Note: This implicitly converts the style of the pen to Qt::CustomDashLine.

See also dashOffset().
*/
impl /*struct*/ QPen {
  pub fn setDashOffset_0<RetType, T: QPen_setDashOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDashOffset_0(self);
    // return 1;
  }
}
pub trait QPen_setDashOffset_0<RetType> {
  fn setDashOffset_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setDashOffset_0<(/*void*/)> for (f64) {
  fn setDashOffset_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen13setDashOffsetEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal miterLimit() const

/*
Returns the miter limit of the pen. The miter limit is only relevant when the join style is set to Qt::MiterJoin.

See also setMiterLimit() and Join Style.
*/
impl /*struct*/ QPen {
  pub fn miterLimit_0<RetType, T: QPen_miterLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.miterLimit_0(self);
    // return 1;
  }
}
pub trait QPen_miterLimit_0<RetType> {
  fn miterLimit_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_miterLimit_0<f64> for () {
  fn miterLimit_0(self , rsthis: & QPen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen10miterLimitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMiterLimit(qreal)

/*
Sets the miter limit of this pen to the given limit.



The miter limit describes how far a miter join can extend from the join point. This is used to reduce artifacts between line joins where the lines are close to parallel.

This value does only have effect when the pen style is set to Qt::MiterJoin. The value is specified in units of the pen's width, e.g. a miter limit of 5 in width 10 is 50 pixels long. The default miter limit is 2, i.e. twice the pen width in pixels.

See also miterLimit(), setJoinStyle(), and Join Style.
*/
impl /*struct*/ QPen {
  pub fn setMiterLimit_0<RetType, T: QPen_setMiterLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMiterLimit_0(self);
    // return 1;
  }
}
pub trait QPen_setMiterLimit_0<RetType> {
  fn setMiterLimit_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setMiterLimit_0<(/*void*/)> for (f64) {
  fn setMiterLimit_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen13setMiterLimitEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal widthF() const

/*
Returns the pen width with floating point precision.

See also setWidthF() and width().
*/
impl /*struct*/ QPen {
  pub fn widthF_0<RetType, T: QPen_widthF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.widthF_0(self);
    // return 1;
  }
}
pub trait QPen_widthF_0<RetType> {
  fn widthF_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_widthF_0<f64> for () {
  fn widthF_0(self , rsthis: & QPen) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen6widthFEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidthF(qreal)

/*
Sets the pen width to the given width in pixels with floating point precision.

A line width of zero indicates a cosmetic pen. This means that the pen width is always drawn one pixel wide, independent of the transformation on the painter.

Setting a pen width with a negative value is not supported.

See also setWidth() and widthF().
*/
impl /*struct*/ QPen {
  pub fn setWidthF_0<RetType, T: QPen_setWidthF_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidthF_0(self);
    // return 1;
  }
}
pub trait QPen_setWidthF_0<RetType> {
  fn setWidthF_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setWidthF_0<(/*void*/)> for (f64) {
  fn setWidthF_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen9setWidthFEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:96
// index:0
// Public Visibility=Default Availability=Available
// [4] int width() const

/*
Returns the pen width with integer precision.

See also setWidth() and widthF().
*/
impl /*struct*/ QPen {
  pub fn width_0<RetType, T: QPen_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QPen_width_0<RetType> {
  fn width_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_width_0<i32> for () {
  fn width_0(self , rsthis: & QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen5widthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setWidth(int)

/*
Sets the pen width to the given width in pixels with integer precision.

A line width of zero indicates a cosmetic pen. This means that the pen width is always drawn one pixel wide, independent of the transformation set on the painter.

Setting a pen width with a negative value is not supported.

See also setWidthF() and width().
*/
impl /*struct*/ QPen {
  pub fn setWidth_0<RetType, T: QPen_setWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWidth_0(self);
    // return 1;
  }
}
pub trait QPen_setWidth_0<RetType> {
  fn setWidth_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setWidth_0<(/*void*/)> for (i32) {
  fn setWidth_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen8setWidthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:99
// index:0
// Public Visibility=Default Availability=Available
// [16] QColor color() const

/*
Returns the color of this pen's brush.

See also brush() and setColor().
*/
impl /*struct*/ QPen {
  pub fn color_0<RetType, T: QPen_color_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.color_0(self);
    // return 1;
  }
}
pub trait QPen_color_0<RetType> {
  fn color_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_color_0<usize> for () {
  fn color_0(self , rsthis: & QPen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen5colorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setColor(const QColor &)

/*
Sets the color of this pen's brush to the given color.

See also setBrush() and color().
*/
impl /*struct*/ QPen {
  pub fn setColor_0<RetType, T: QPen_setColor_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setColor_0(self);
    // return 1;
  }
}
pub trait QPen_setColor_0<RetType> {
  fn setColor_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setColor_0<(/*void*/)> for (usize) {
  fn setColor_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen8setColorERK6QColor", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QBrush brush() const

/*
Returns the brush used to fill strokes generated with this pen.

See also setBrush().
*/
impl /*struct*/ QPen {
  pub fn brush_0<RetType, T: QPen_brush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.brush_0(self);
    // return 1;
  }
}
pub trait QPen_brush_0<RetType> {
  fn brush_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_brush_0<usize> for () {
  fn brush_0(self , rsthis: & QPen) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen5brushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBrush(const QBrush &)

/*
Sets the brush used to fill strokes generated with this pen to the given brush.

See also brush() and setColor().
*/
impl /*struct*/ QPen {
  pub fn setBrush_0<RetType, T: QPen_setBrush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBrush_0(self);
    // return 1;
  }
}
pub trait QPen_setBrush_0<RetType> {
  fn setBrush_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setBrush_0<(/*void*/)> for (usize) {
  fn setBrush_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen8setBrushERK6QBrush", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSolid() const

/*
Returns true if the pen has a solid fill, otherwise false.

See also style() and dashPattern().
*/
impl /*struct*/ QPen {
  pub fn isSolid_0<RetType, T: QPen_isSolid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSolid_0(self);
    // return 1;
  }
}
pub trait QPen_isSolid_0<RetType> {
  fn isSolid_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_isSolid_0<bool> for () {
  fn isSolid_0(self , rsthis: & QPen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen7isSolidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::PenCapStyle capStyle() const

/*
Returns the pen's cap style.

See also setCapStyle() and Cap Style.
*/
impl /*struct*/ QPen {
  pub fn capStyle_0<RetType, T: QPen_capStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capStyle_0(self);
    // return 1;
  }
}
pub trait QPen_capStyle_0<RetType> {
  fn capStyle_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_capStyle_0<i32> for () {
  fn capStyle_0(self , rsthis: & QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen8capStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCapStyle(Qt::PenCapStyle)

/*
Sets the pen's cap style to the given style. The default value is Qt::SquareCap.

See also capStyle() and Cap Style.
*/
impl /*struct*/ QPen {
  pub fn setCapStyle_0<RetType, T: QPen_setCapStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCapStyle_0(self);
    // return 1;
  }
}
pub trait QPen_setCapStyle_0<RetType> {
  fn setCapStyle_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setCapStyle_0<(/*void*/)> for (i32) {
  fn setCapStyle_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen11setCapStyleEN2Qt11PenCapStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:110
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::PenJoinStyle joinStyle() const

/*
Returns the pen's join style.

See also setJoinStyle() and Join Style.
*/
impl /*struct*/ QPen {
  pub fn joinStyle_0<RetType, T: QPen_joinStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.joinStyle_0(self);
    // return 1;
  }
}
pub trait QPen_joinStyle_0<RetType> {
  fn joinStyle_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_joinStyle_0<i32> for () {
  fn joinStyle_0(self , rsthis: & QPen) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen9joinStyleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setJoinStyle(Qt::PenJoinStyle)

/*
Sets the pen's join style to the given style. The default value is Qt::BevelJoin.

See also joinStyle() and Join Style.
*/
impl /*struct*/ QPen {
  pub fn setJoinStyle_0<RetType, T: QPen_setJoinStyle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setJoinStyle_0(self);
    // return 1;
  }
}
pub trait QPen_setJoinStyle_0<RetType> {
  fn setJoinStyle_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setJoinStyle_0<(/*void*/)> for (i32) {
  fn setJoinStyle_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen12setJoinStyleEN2Qt12PenJoinStyleE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCosmetic() const

/*
Returns true if the pen is cosmetic; otherwise returns false.

Cosmetic pens are used to draw strokes that have a constant width regardless of any transformations applied to the QPainter they are used with. Drawing a shape with a cosmetic pen ensures that its outline will have the same thickness at different scale factors.

A zero width pen is cosmetic by default.

See also setCosmetic() and widthF().
*/
impl /*struct*/ QPen {
  pub fn isCosmetic_0<RetType, T: QPen_isCosmetic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCosmetic_0(self);
    // return 1;
  }
}
pub trait QPen_isCosmetic_0<RetType> {
  fn isCosmetic_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_isCosmetic_0<bool> for () {
  fn isCosmetic_0(self , rsthis: & QPen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPen10isCosmeticEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCosmetic(bool)

/*
Sets this pen to cosmetic or non-cosmetic, depending on the value of cosmetic.

See also isCosmetic().
*/
impl /*struct*/ QPen {
  pub fn setCosmetic_0<RetType, T: QPen_setCosmetic_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCosmetic_0(self);
    // return 1;
  }
}
pub trait QPen_setCosmetic_0<RetType> {
  fn setCosmetic_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_setCosmetic_0<(/*void*/)> for (bool) {
  fn setCosmetic_0(self , rsthis: & QPen) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN4QPen11setCosmeticEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qpen.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QPen &) const

/*

*/
impl /*struct*/ QPen {
  pub fn operator_equal_equal_0<RetType, T: QPen_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QPen_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QPen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPeneqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:117
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QPen &) const

/*

*/
impl /*struct*/ QPen {
  pub fn operator_not_equal_0<RetType, T: QPen_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QPen_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QPen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QPenneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qpen.h:120
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDetached()

/*

*/
impl /*struct*/ QPen {
  pub fn isDetached_0<RetType, T: QPen_isDetached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDetached_0(self);
    // return 1;
  }
}
pub trait QPen_isDetached_0<RetType> {
  fn isDetached_0(self , rsthis: & QPen) -> RetType;
}
impl<'a> /*trait*/ QPen_isDetached_0<bool> for () {
  fn isDetached_0(self , rsthis: & QPen) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QPen10isDetachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
