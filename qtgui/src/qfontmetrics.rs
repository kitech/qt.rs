

// mod ::gui::QFontMetrics
// package qtgui
// /usr/include/qt/QtGui/qfontmetrics.h
// #include <qfontmetrics.h>
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



/*

*/
#[derive(Default)] // class sizeof(QFontMetrics)=8
pub struct QFontMetrics {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFontMetrics_ITF interface {
//    QFontMetrics_PTR() *QFontMetrics
//}
//func (ptr *QFontMetrics) QFontMetrics_PTR() *QFontMetrics { return ptr }

impl /*struct*/ QFontMetrics {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFontMetrics {
    return QFontMetrics{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFontMetrics {
//  type Target = QFontMetricsBASE;
//
//  fn deref(&self) -> &QFontMetricsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontMetricsBASE> for QFontMetrics {
//  fn as_ref(& self) -> & QFontMetricsBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qfontmetrics.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFontMetrics(const QFont &)

/*
Constructs a font metrics object for font.

The font metrics will be compatible with the paintdevice used to create font.

The font metrics object holds the information for the font that is passed in the constructor at the time it is created, and is not updated if the font's attributes are changed later.

Use QFontMetrics(const QFont &, QPaintDevice *) to get the font metrics that are compatible with a certain paint device.
*/
// QFontMetrics(const QFont &) ctx.fn_proto_cpp
impl /*struct*/ QFontMetrics {
  pub fn QFontMetrics_0<T: QFontMetrics_QFontMetrics_0>(value: T) -> QFontMetrics {
    let rsthis = value.QFontMetrics_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetrics_QFontMetrics_0 {
  fn QFontMetrics_0(self) -> QFontMetrics;
}
// QFontMetrics(const QFont &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontMetrics_QFontMetrics_0 for (usize) {
  fn QFontMetrics_0(self) -> QFontMetrics {
    // unsafe{_ZN12QFontMetricsC2ERK5QFont()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QFontMetricsC2ERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontMetrics{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:62
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFontMetrics(const QFont &, QPaintDevice *)

/*
Constructs a font metrics object for font.

The font metrics will be compatible with the paintdevice used to create font.

The font metrics object holds the information for the font that is passed in the constructor at the time it is created, and is not updated if the font's attributes are changed later.

Use QFontMetrics(const QFont &, QPaintDevice *) to get the font metrics that are compatible with a certain paint device.
*/
// QFontMetrics(const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl /*struct*/ QFontMetrics {
  pub fn QFontMetrics_1<T: QFontMetrics_QFontMetrics_1>(value: T) -> QFontMetrics {
    let rsthis = value.QFontMetrics_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetrics_QFontMetrics_1 {
  fn QFontMetrics_1(self) -> QFontMetrics;
}
// QFontMetrics(const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontMetrics_QFontMetrics_1 for (usize,usize) {
  fn QFontMetrics_1(self) -> QFontMetrics {
    // unsafe{_ZN12QFontMetricsC2ERK5QFontP12QPaintDevice()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QFontMetricsC2ERK5QFontP12QPaintDevice", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontMetrics{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QFontMetrics()

/*

*/
pub fn DeleteQFontMetrics(this :*mut QFontMetrics) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QFontMetricsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qfontmetrics.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontMetrics & operator=(const QFontMetrics &)

/*

*/
impl /*struct*/ QFontMetrics {
  pub fn operator_equal_0<RetType, T: QFontMetrics_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QFontMetricsaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:68
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QFontMetrics & operator=(QFontMetrics &&)

/*

*/
impl /*struct*/ QFontMetrics {
  pub fn operator_equal_1<RetType, T: QFontMetrics_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QFontMetrics_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QFontMetricsaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:72
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QFontMetrics &)

/*
Swaps this font metrics instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QFontMetrics {
  pub fn swap_0<RetType, T: QFontMetrics_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_swap_0<RetType> {
  fn swap_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QFontMetrics) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QFontMetrics4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int ascent() const

/*
Returns the ascent of the font.

The ascent of a font is the distance from the baseline to the highest position characters extend to. In practice, some font designers break this rule, e.g. when they put more than one accent on top of a character, or to accommodate an unusual character in an exotic language, so it is possible (though rare) that this value will be too small.

See also descent().
*/
impl /*struct*/ QFontMetrics {
  pub fn ascent_0<RetType, T: QFontMetrics_ascent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ascent_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_ascent_0<RetType> {
  fn ascent_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_ascent_0<i32> for () {
  fn ascent_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics6ascentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int capHeight() const

/*
Returns the cap height of the font.

The cap height of a font is the height of a capital letter above the baseline. It specifically is the height of capital letters that are flat - such as H or I - as opposed to round letters such as O, or pointed letters like A, both of which may display overshoot.

This function was introduced in  Qt 5.8.

See also ascent().
*/
impl /*struct*/ QFontMetrics {
  pub fn capHeight_0<RetType, T: QFontMetrics_capHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capHeight_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_capHeight_0<RetType> {
  fn capHeight_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_capHeight_0<i32> for () {
  fn capHeight_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics9capHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:77
// index:0
// Public Visibility=Default Availability=Available
// [4] int descent() const

/*
Returns the descent of the font.

The descent is the distance from the base line to the lowest point characters extend to. In practice, some font designers break this rule, e.g. to accommodate an unusual character in an exotic language, so it is possible (though rare) that this value will be too small.

See also ascent().
*/
impl /*struct*/ QFontMetrics {
  pub fn descent_0<RetType, T: QFontMetrics_descent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.descent_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_descent_0<RetType> {
  fn descent_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_descent_0<i32> for () {
  fn descent_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics7descentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:78
// index:0
// Public Visibility=Default Availability=Available
// [4] int height() const

/*
Returns the height of the font.

This is always equal to ascent()+descent().

See also leading() and lineSpacing().
*/
impl /*struct*/ QFontMetrics {
  pub fn height_0<RetType, T: QFontMetrics_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_height_0<RetType> {
  fn height_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_height_0<i32> for () {
  fn height_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] int leading() const

/*
Returns the leading of the font.

This is the natural inter-line spacing.

See also height() and lineSpacing().
*/
impl /*struct*/ QFontMetrics {
  pub fn leading_0<RetType, T: QFontMetrics_leading_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leading_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_leading_0<RetType> {
  fn leading_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_leading_0<i32> for () {
  fn leading_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics7leadingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:80
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineSpacing() const

/*
Returns the distance from one base line to the next.

This value is always equal to leading()+height().

See also height() and leading().
*/
impl /*struct*/ QFontMetrics {
  pub fn lineSpacing_0<RetType, T: QFontMetrics_lineSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineSpacing_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_lineSpacing_0<RetType> {
  fn lineSpacing_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_lineSpacing_0<i32> for () {
  fn lineSpacing_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics11lineSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] int minLeftBearing() const

/*
Returns the minimum left bearing of the font.

This is the smallest leftBearing(char) of all characters in the font.

Note that this function can be very slow if the font is large.

See also minRightBearing() and leftBearing().
*/
impl /*struct*/ QFontMetrics {
  pub fn minLeftBearing_0<RetType, T: QFontMetrics_minLeftBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minLeftBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_minLeftBearing_0<RetType> {
  fn minLeftBearing_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_minLeftBearing_0<i32> for () {
  fn minLeftBearing_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics14minLeftBearingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] int minRightBearing() const

/*
Returns the minimum right bearing of the font.

This is the smallest rightBearing(char) of all characters in the font.

Note that this function can be very slow if the font is large.

See also minLeftBearing() and rightBearing().
*/
impl /*struct*/ QFontMetrics {
  pub fn minRightBearing_0<RetType, T: QFontMetrics_minRightBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minRightBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_minRightBearing_0<RetType> {
  fn minRightBearing_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_minRightBearing_0<i32> for () {
  fn minRightBearing_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics15minRightBearingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] int maxWidth() const

/*
Returns the width of the widest character in the font.
*/
impl /*struct*/ QFontMetrics {
  pub fn maxWidth_0<RetType, T: QFontMetrics_maxWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_maxWidth_0<RetType> {
  fn maxWidth_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_maxWidth_0<i32> for () {
  fn maxWidth_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics8maxWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:85
// index:0
// Public Visibility=Default Availability=Available
// [4] int xHeight() const

/*
Returns the 'x' height of the font. This is often but not always the same as the height of the character 'x'.
*/
impl /*struct*/ QFontMetrics {
  pub fn xHeight_0<RetType, T: QFontMetrics_xHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xHeight_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_xHeight_0<RetType> {
  fn xHeight_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_xHeight_0<i32> for () {
  fn xHeight_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics7xHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:86
// index:0
// Public Visibility=Default Availability=Available
// [4] int averageCharWidth() const

/*
Returns the average width of glyphs in the font.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QFontMetrics {
  pub fn averageCharWidth_0<RetType, T: QFontMetrics_averageCharWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.averageCharWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_averageCharWidth_0<RetType> {
  fn averageCharWidth_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_averageCharWidth_0<i32> for () {
  fn averageCharWidth_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics16averageCharWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool inFont(QChar) const

/*
Returns true if character ch is a valid character in the font; otherwise returns false.
*/
impl /*struct*/ QFontMetrics {
  pub fn inFont_0<RetType, T: QFontMetrics_inFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inFont_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_inFont_0<RetType> {
  fn inFont_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_inFont_0<bool> for (usize) {
  fn inFont_0(self , rsthis: & QFontMetrics) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics6inFontE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:89
// index:0
// Public Visibility=Default Availability=Available
// [1] bool inFontUcs4(uint) const

/*
Returns true if the character ucs4 encoded in UCS-4/UTF-32 is a valid character in the font; otherwise returns false.
*/
impl /*struct*/ QFontMetrics {
  pub fn inFontUcs4_0<RetType, T: QFontMetrics_inFontUcs4_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inFontUcs4_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_inFontUcs4_0<RetType> {
  fn inFontUcs4_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_inFontUcs4_0<bool> for (u32) {
  fn inFontUcs4_0(self , rsthis: & QFontMetrics) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics10inFontUcs4Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] int leftBearing(QChar) const

/*
Returns the left bearing of character ch in the font.

The left bearing is the right-ward distance of the left-most pixel of the character from the logical origin of the character. This value is negative if the pixels of the character extend to the left of the logical origin.

See width(QChar) for a graphical description of this metric.

See also rightBearing(), minLeftBearing(), and width().
*/
impl /*struct*/ QFontMetrics {
  pub fn leftBearing_0<RetType, T: QFontMetrics_leftBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_leftBearing_0<RetType> {
  fn leftBearing_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_leftBearing_0<i32> for (usize) {
  fn leftBearing_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics11leftBearingE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:92
// index:0
// Public Visibility=Default Availability=Available
// [4] int rightBearing(QChar) const

/*
Returns the right bearing of character ch in the font.

The right bearing is the left-ward distance of the right-most pixel of the character from the logical origin of a subsequent character. This value is negative if the pixels of the character extend to the right of the width() of the character.

See width() for a graphical description of this metric.

See also leftBearing(), minRightBearing(), and width().
*/
impl /*struct*/ QFontMetrics {
  pub fn rightBearing_0<RetType, T: QFontMetrics_rightBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_rightBearing_0<RetType> {
  fn rightBearing_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_rightBearing_0<i32> for (usize) {
  fn rightBearing_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12rightBearingE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] int width(const QString &, int) const

/*
Returns the width in pixels of the first len characters of text. If len is negative (the default), the entire string is used.

Note that this value is not equal to boundingRect().width(); boundingRect() returns a rectangle describing the pixels this string will cover whereas width() returns the distance to where the next string should be drawn.

See also boundingRect().
*/
impl /*struct*/ QFontMetrics {
  pub fn width_0<RetType, T: QFontMetrics_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_width_0<RetType> {
  fn width_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_width_0<i32> for (usize,i32) {
  fn width_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics5widthERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:94
// index:1
// Public Visibility=Default Availability=Available
// [4] int width(const QString &, int, int) const

/*
Returns the width in pixels of the first len characters of text. If len is negative (the default), the entire string is used.

Note that this value is not equal to boundingRect().width(); boundingRect() returns a rectangle describing the pixels this string will cover whereas width() returns the distance to where the next string should be drawn.

See also boundingRect().
*/
impl /*struct*/ QFontMetrics {
  pub fn width_1<RetType, T: QFontMetrics_width_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_1(self);
    // return 1;
  }
}
pub trait QFontMetrics_width_1<RetType> {
  fn width_1(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_width_1<i32> for (usize,i32,i32) {
  fn width_1(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics5widthERK7QStringii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:96
// index:2
// Public Visibility=Default Availability=Available
// [4] int width(QChar) const

/*
Returns the width in pixels of the first len characters of text. If len is negative (the default), the entire string is used.

Note that this value is not equal to boundingRect().width(); boundingRect() returns a rectangle describing the pixels this string will cover whereas width() returns the distance to where the next string should be drawn.

See also boundingRect().
*/
impl /*struct*/ QFontMetrics {
  pub fn width_2<RetType, T: QFontMetrics_width_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_2(self);
    // return 1;
  }
}
pub trait QFontMetrics_width_2<RetType> {
  fn width_2(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_width_2<i32> for (usize) {
  fn width_2(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics5widthE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] int charWidth(const QString &, int) const

/*

*/
impl /*struct*/ QFontMetrics {
  pub fn charWidth_0<RetType, T: QFontMetrics_charWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.charWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_charWidth_0<RetType> {
  fn charWidth_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_charWidth_0<i32> for (usize,i32) {
  fn charWidth_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics9charWidthERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:101
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect boundingRect(QChar) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetrics {
  pub fn boundingRect_0<RetType, T: QFontMetrics_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_boundingRect_0<usize> for (usize) {
  fn boundingRect_0(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12boundingRectE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:103
// index:1
// Public Visibility=Default Availability=Available
// [16] QRect boundingRect(const QString &) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetrics {
  pub fn boundingRect_1<RetType, T: QFontMetrics_boundingRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_1(self);
    // return 1;
  }
}
pub trait QFontMetrics_boundingRect_1<RetType> {
  fn boundingRect_1(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_boundingRect_1<usize> for (usize) {
  fn boundingRect_1(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12boundingRectERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:104
// index:2
// Public Visibility=Default Availability=Available
// [16] QRect boundingRect(const QRect &, int, const QString &, int, int *) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetrics {
  pub fn boundingRect_2<RetType, T: QFontMetrics_boundingRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_2(self);
    // return 1;
  }
}
pub trait QFontMetrics_boundingRect_2<RetType> {
  fn boundingRect_2(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_boundingRect_2<usize> for (usize,i32,usize,i32,usize) {
  fn boundingRect_2(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12boundingRectERK5QRectiRK7QStringiPi", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:105
// index:3
// Public inline Visibility=Default Availability=Available
// [16] QRect boundingRect(int, int, int, int, int, const QString &, int, int *) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetrics {
  pub fn boundingRect_3<RetType, T: QFontMetrics_boundingRect_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_3(self);
    // return 1;
  }
}
pub trait QFontMetrics_boundingRect_3<RetType> {
  fn boundingRect_3(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_boundingRect_3<usize> for (i32,i32,i32,i32,i32,usize,i32,usize) {
  fn boundingRect_3(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const i32 as usize;
    let arg5 = (&self.5/*.qclsinst*/) as *const usize as usize;
    let arg6 = (&self.6) as *const i32 as usize;
    let arg7 = (&self.7) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12boundingRectEiiiiiRK7QStringiPi", 8,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,arg5,arg6,arg7,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:108
// index:0
// Public Visibility=Default Availability=Available
// [8] QSize size(int, const QString &, int, int *) const

/*
Returns the size in pixels of text.

The flags argument is the bitwise OR of the following flags:


Qt::TextSingleLine ignores newline characters.
Qt::TextExpandTabs expands tabs (see below)
Qt::TextShowMnemonic interprets "&x" as x; i.e., underlined.
Qt::TextWordWrap breaks the text to fit the rectangle.


If Qt::TextExpandTabs is set in flags, then: if tabArray is non-null, it specifies a 0-terminated sequence of pixel-positions for tabs; otherwise if tabStops is non-zero, it is used as the tab spacing (in pixels).

Newline characters are processed as linebreaks.

Despite the different actual character heights, the heights of the bounding rectangles of "Yes" and "yes" are the same.

See also boundingRect().
*/
impl /*struct*/ QFontMetrics {
  pub fn size_0<RetType, T: QFontMetrics_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_size_0<RetType> {
  fn size_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_size_0<usize> for (i32,usize,i32,usize) {
  fn size_0(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics4sizeEiRK7QStringiPi", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:110
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect tightBoundingRect(const QString &) const

/*
Returns a tight bounding rectangle around the characters in the string specified by text. The bounding rectangle always covers at least the set of pixels the text would cover if drawn at (0, 0).

Note that the bounding rectangle may extend to the left of (0, 0), e.g. for italicized fonts, and that the width of the returned rectangle might be different than what the width() method returns.

If you want to know the advance width of the string (to lay out a set of strings next to each other), use width() instead.

Newline characters are processed as normal characters, not as linebreaks.

Warning: Calling this method is very slow on Windows.

This function was introduced in  Qt 4.3.

See also width(), height(), and boundingRect().
*/
impl /*struct*/ QFontMetrics {
  pub fn tightBoundingRect_0<RetType, T: QFontMetrics_tightBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tightBoundingRect_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_tightBoundingRect_0<RetType> {
  fn tightBoundingRect_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_tightBoundingRect_0<usize> for (usize) {
  fn tightBoundingRect_0(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics17tightBoundingRectERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:112
// index:0
// Public Visibility=Default Availability=Available
// [8] QString elidedText(const QString &, Qt::TextElideMode, int, int) const

/*
If the string text is wider than width, returns an elided version of the string (i.e., a string with "..." in it). Otherwise, returns the original string.

The mode parameter specifies whether the text is elided on the left (e.g., "...tech"), in the middle (e.g., "Tr...ch"), or on the right (e.g., "Trol...").

The width is specified in pixels, not characters.

The flags argument is optional and currently only supports Qt::TextShowMnemonic as value.

The elide mark follows the layoutdirection. For example, it will be on the right side of the text for right-to-left layouts if the mode is Qt::ElideLeft, and on the left side of the text if the mode is Qt::ElideRight.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QFontMetrics {
  pub fn elidedText_0<RetType, T: QFontMetrics_elidedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elidedText_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_elidedText_0<RetType> {
  fn elidedText_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_elidedText_0<usize> for (usize,i32,i32,i32) {
  fn elidedText_0(self , rsthis: & QFontMetrics) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics10elidedTextERK7QStringN2Qt13TextElideModeEii", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:114
// index:0
// Public Visibility=Default Availability=Available
// [4] int underlinePos() const

/*
Returns the distance from the base line to where an underscore should be drawn.

See also overlinePos(), strikeOutPos(), and lineWidth().
*/
impl /*struct*/ QFontMetrics {
  pub fn underlinePos_0<RetType, T: QFontMetrics_underlinePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underlinePos_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_underlinePos_0<RetType> {
  fn underlinePos_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_underlinePos_0<i32> for () {
  fn underlinePos_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12underlinePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:115
// index:0
// Public Visibility=Default Availability=Available
// [4] int overlinePos() const

/*
Returns the distance from the base line to where an overline should be drawn.

See also underlinePos(), strikeOutPos(), and lineWidth().
*/
impl /*struct*/ QFontMetrics {
  pub fn overlinePos_0<RetType, T: QFontMetrics_overlinePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overlinePos_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_overlinePos_0<RetType> {
  fn overlinePos_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_overlinePos_0<i32> for () {
  fn overlinePos_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics11overlinePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:116
// index:0
// Public Visibility=Default Availability=Available
// [4] int strikeOutPos() const

/*
Returns the distance from the base line to where the strikeout line should be drawn.

See also underlinePos(), overlinePos(), and lineWidth().
*/
impl /*struct*/ QFontMetrics {
  pub fn strikeOutPos_0<RetType, T: QFontMetrics_strikeOutPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strikeOutPos_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_strikeOutPos_0<RetType> {
  fn strikeOutPos_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_strikeOutPos_0<i32> for () {
  fn strikeOutPos_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics12strikeOutPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:117
// index:0
// Public Visibility=Default Availability=Available
// [4] int lineWidth() const

/*
Returns the width of the underline and strikeout lines, adjusted for the point size of the font.

See also underlinePos(), overlinePos(), and strikeOutPos().
*/
impl /*struct*/ QFontMetrics {
  pub fn lineWidth_0<RetType, T: QFontMetrics_lineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_lineWidth_0<RetType> {
  fn lineWidth_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_lineWidth_0<i32> for () {
  fn lineWidth_0(self , rsthis: & QFontMetrics) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetrics9lineWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:119
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QFontMetrics &) const

/*

*/
impl /*struct*/ QFontMetrics {
  pub fn operator_equal_equal_0<RetType, T: QFontMetrics_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QFontMetrics) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetricseqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QFontMetrics &) const

/*

*/
impl /*struct*/ QFontMetrics {
  pub fn operator_not_equal_0<RetType, T: QFontMetrics_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QFontMetrics_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QFontMetrics) -> RetType;
}
impl<'a> /*trait*/ QFontMetrics_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QFontMetrics) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QFontMetricsneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
