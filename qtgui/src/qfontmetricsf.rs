

// mod ::gui::QFontMetricsF
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
// extern C begin: 38
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
#[derive(Default)] // class sizeof(QFontMetricsF)=8
pub struct QFontMetricsF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFontMetricsF_ITF interface {
//    QFontMetricsF_PTR() *QFontMetricsF
//}
//func (ptr *QFontMetricsF) QFontMetricsF_PTR() *QFontMetricsF { return ptr }

impl /*struct*/ QFontMetricsF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFontMetricsF {
    return QFontMetricsF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFontMetricsF {
//  type Target = QFontMetricsFBASE;
//
//  fn deref(&self) -> &QFontMetricsFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFontMetricsFBASE> for QFontMetricsF {
//  fn as_ref(& self) -> & QFontMetricsFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qfontmetrics.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFontMetricsF(const QFont &)

/*

*/
// QFontMetricsF(const QFont &) ctx.fn_proto_cpp
impl /*struct*/ QFontMetricsF {
  pub fn QFontMetricsF_0<T: QFontMetricsF_QFontMetricsF_0>(value: T) -> QFontMetricsF {
    let rsthis = value.QFontMetricsF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetricsF_QFontMetricsF_0 {
  fn QFontMetricsF_0(self) -> QFontMetricsF;
}
// QFontMetricsF(const QFont &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontMetricsF_QFontMetricsF_0 for (usize) {
  fn QFontMetricsF_0(self) -> QFontMetricsF {
    // unsafe{_ZN13QFontMetricsFC2ERK5QFont()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFC2ERK5QFont", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontMetricsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:135
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFontMetricsF(const QFont &, QPaintDevice *)

/*

*/
// QFontMetricsF(const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl /*struct*/ QFontMetricsF {
  pub fn QFontMetricsF_1<T: QFontMetricsF_QFontMetricsF_1>(value: T) -> QFontMetricsF {
    let rsthis = value.QFontMetricsF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetricsF_QFontMetricsF_1 {
  fn QFontMetricsF_1(self) -> QFontMetricsF;
}
// QFontMetricsF(const QFont &, QPaintDevice *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontMetricsF_QFontMetricsF_1 for (usize,usize) {
  fn QFontMetricsF_1(self) -> QFontMetricsF {
    // unsafe{_ZN13QFontMetricsFC2ERK5QFontP12QPaintDevice()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFC2ERK5QFontP12QPaintDevice", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontMetricsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:136
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QFontMetricsF(const QFontMetrics &)

/*

*/
// QFontMetricsF(const QFontMetrics &) ctx.fn_proto_cpp
impl /*struct*/ QFontMetricsF {
  pub fn QFontMetricsF_2<T: QFontMetricsF_QFontMetricsF_2>(value: T) -> QFontMetricsF {
    let rsthis = value.QFontMetricsF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QFontMetricsF_QFontMetricsF_2 {
  fn QFontMetricsF_2(self) -> QFontMetricsF;
}
// QFontMetricsF(const QFontMetrics &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFontMetricsF_QFontMetricsF_2 for (usize) {
  fn QFontMetricsF_2(self) -> QFontMetricsF {
    // unsafe{_ZN13QFontMetricsFC2ERK12QFontMetrics()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFC2ERK12QFontMetrics", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFontMetricsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:138
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QFontMetricsF()

/*

*/
pub fn DeleteQFontMetricsF(this :*mut QFontMetricsF) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qfontmetrics.h:140
// index:0
// Public Visibility=Default Availability=Available
// [8] QFontMetricsF & operator=(const QFontMetricsF &)

/*

*/
impl /*struct*/ QFontMetricsF {
  pub fn operator_equal_0<RetType, T: QFontMetricsF_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:141
// index:1
// Public Visibility=Default Availability=Available
// [8] QFontMetricsF & operator=(const QFontMetrics &)

/*

*/
impl /*struct*/ QFontMetricsF {
  pub fn operator_equal_1<RetType, T: QFontMetricsF_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QFontMetricsF_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFaSERK12QFontMetrics", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:143
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QFontMetricsF & operator=(QFontMetricsF &&)

/*

*/
impl /*struct*/ QFontMetricsF {
  pub fn operator_equal_2<RetType, T: QFontMetricsF_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QFontMetricsF_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QFontMetricsFaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:147
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QFontMetricsF &)

/*
Swaps this font metrics instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QFontMetricsF {
  pub fn swap_0<RetType, T: QFontMetricsF_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_swap_0<RetType> {
  fn swap_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QFontMetricsF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QFontMetricsF4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:149
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal ascent() const

/*
Returns the ascent of the font.

The ascent of a font is the distance from the baseline to the highest position characters extend to. In practice, some font designers break this rule, e.g. when they put more than one accent on top of a character, or to accommodate an unusual character in an exotic language, so it is possible (though rare) that this value will be too small.

See also descent().
*/
impl /*struct*/ QFontMetricsF {
  pub fn ascent_0<RetType, T: QFontMetricsF_ascent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ascent_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_ascent_0<RetType> {
  fn ascent_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_ascent_0<f64> for () {
  fn ascent_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF6ascentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:150
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal capHeight() const

/*
Returns the cap height of the font.

The cap height of a font is the height of a capital letter above the baseline. It specifically is the height of capital letters that are flat - such as H or I - as opposed to round letters such as O, or pointed letters like A, both of which may display overshoot.

This function was introduced in  Qt 5.8.

See also ascent().
*/
impl /*struct*/ QFontMetricsF {
  pub fn capHeight_0<RetType, T: QFontMetricsF_capHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capHeight_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_capHeight_0<RetType> {
  fn capHeight_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_capHeight_0<f64> for () {
  fn capHeight_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF9capHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:151
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal descent() const

/*
Returns the descent of the font.

The descent is the distance from the base line to the lowest point characters extend to. In practice, some font designers break this rule, e.g. to accommodate an unusual character in an exotic language, so it is possible (though rare) that this value will be too small.

See also ascent().
*/
impl /*struct*/ QFontMetricsF {
  pub fn descent_0<RetType, T: QFontMetricsF_descent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.descent_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_descent_0<RetType> {
  fn descent_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_descent_0<f64> for () {
  fn descent_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF7descentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:152
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal height() const

/*
Returns the height of the font.

This is always equal to ascent()+descent().

See also leading() and lineSpacing().
*/
impl /*struct*/ QFontMetricsF {
  pub fn height_0<RetType, T: QFontMetricsF_height_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.height_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_height_0<RetType> {
  fn height_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_height_0<f64> for () {
  fn height_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF6heightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:153
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal leading() const

/*
Returns the leading of the font.

This is the natural inter-line spacing.

See also height() and lineSpacing().
*/
impl /*struct*/ QFontMetricsF {
  pub fn leading_0<RetType, T: QFontMetricsF_leading_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leading_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_leading_0<RetType> {
  fn leading_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_leading_0<f64> for () {
  fn leading_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF7leadingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:154
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal lineSpacing() const

/*
Returns the distance from one base line to the next.

This value is always equal to leading()+height().

See also height() and leading().
*/
impl /*struct*/ QFontMetricsF {
  pub fn lineSpacing_0<RetType, T: QFontMetricsF_lineSpacing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineSpacing_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_lineSpacing_0<RetType> {
  fn lineSpacing_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_lineSpacing_0<f64> for () {
  fn lineSpacing_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF11lineSpacingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:155
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal minLeftBearing() const

/*
Returns the minimum left bearing of the font.

This is the smallest leftBearing(char) of all characters in the font.

Note that this function can be very slow if the font is large.

See also minRightBearing() and leftBearing().
*/
impl /*struct*/ QFontMetricsF {
  pub fn minLeftBearing_0<RetType, T: QFontMetricsF_minLeftBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minLeftBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_minLeftBearing_0<RetType> {
  fn minLeftBearing_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_minLeftBearing_0<f64> for () {
  fn minLeftBearing_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF14minLeftBearingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:156
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal minRightBearing() const

/*
Returns the minimum right bearing of the font.

This is the smallest rightBearing(char) of all characters in the font.

Note that this function can be very slow if the font is large.

See also minLeftBearing() and rightBearing().
*/
impl /*struct*/ QFontMetricsF {
  pub fn minRightBearing_0<RetType, T: QFontMetricsF_minRightBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.minRightBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_minRightBearing_0<RetType> {
  fn minRightBearing_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_minRightBearing_0<f64> for () {
  fn minRightBearing_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF15minRightBearingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:157
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal maxWidth() const

/*
Returns the width of the widest character in the font.
*/
impl /*struct*/ QFontMetricsF {
  pub fn maxWidth_0<RetType, T: QFontMetricsF_maxWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_maxWidth_0<RetType> {
  fn maxWidth_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_maxWidth_0<f64> for () {
  fn maxWidth_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF8maxWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:159
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal xHeight() const

/*
Returns the 'x' height of the font. This is often but not always the same as the height of the character 'x'.
*/
impl /*struct*/ QFontMetricsF {
  pub fn xHeight_0<RetType, T: QFontMetricsF_xHeight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.xHeight_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_xHeight_0<RetType> {
  fn xHeight_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_xHeight_0<f64> for () {
  fn xHeight_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF7xHeightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:160
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal averageCharWidth() const

/*
Returns the average width of glyphs in the font.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QFontMetricsF {
  pub fn averageCharWidth_0<RetType, T: QFontMetricsF_averageCharWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.averageCharWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_averageCharWidth_0<RetType> {
  fn averageCharWidth_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_averageCharWidth_0<f64> for () {
  fn averageCharWidth_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF16averageCharWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:162
// index:0
// Public Visibility=Default Availability=Available
// [1] bool inFont(QChar) const

/*
Returns true if character ch is a valid character in the font; otherwise returns false.
*/
impl /*struct*/ QFontMetricsF {
  pub fn inFont_0<RetType, T: QFontMetricsF_inFont_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inFont_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_inFont_0<RetType> {
  fn inFont_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_inFont_0<bool> for (usize) {
  fn inFont_0(self , rsthis: & QFontMetricsF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF6inFontE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:163
// index:0
// Public Visibility=Default Availability=Available
// [1] bool inFontUcs4(uint) const

/*
Returns true if the character ucs4 encoded in UCS-4/UTF-32 is a valid character in the font; otherwise returns false.
*/
impl /*struct*/ QFontMetricsF {
  pub fn inFontUcs4_0<RetType, T: QFontMetricsF_inFontUcs4_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.inFontUcs4_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_inFontUcs4_0<RetType> {
  fn inFontUcs4_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_inFontUcs4_0<bool> for (u32) {
  fn inFontUcs4_0(self , rsthis: & QFontMetricsF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF10inFontUcs4Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:165
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal leftBearing(QChar) const

/*
Returns the left bearing of character ch in the font.

The left bearing is the right-ward distance of the left-most pixel of the character from the logical origin of the character. This value is negative if the pixels of the character extend to the left of the logical origin.

See width(QChar) for a graphical description of this metric.

See also rightBearing(), minLeftBearing(), and width().
*/
impl /*struct*/ QFontMetricsF {
  pub fn leftBearing_0<RetType, T: QFontMetricsF_leftBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.leftBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_leftBearing_0<RetType> {
  fn leftBearing_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_leftBearing_0<f64> for (usize) {
  fn leftBearing_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF11leftBearingE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:166
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal rightBearing(QChar) const

/*
Returns the right bearing of character ch in the font.

The right bearing is the left-ward distance of the right-most pixel of the character from the logical origin of a subsequent character. This value is negative if the pixels of the character extend to the right of the width() of the character.

See width() for a graphical description of this metric.

See also leftBearing(), minRightBearing(), and width().
*/
impl /*struct*/ QFontMetricsF {
  pub fn rightBearing_0<RetType, T: QFontMetricsF_rightBearing_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rightBearing_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_rightBearing_0<RetType> {
  fn rightBearing_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_rightBearing_0<f64> for (usize) {
  fn rightBearing_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF12rightBearingE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:167
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal width(const QString &) const

/*
Returns the width in pixels of the first len characters of text. If len is negative (the default), the entire string is used.

Note that this value is not equal to boundingRect().width(); boundingRect() returns a rectangle describing the pixels this string will cover whereas width() returns the distance to where the next string should be drawn.

See also boundingRect().
*/
impl /*struct*/ QFontMetricsF {
  pub fn width_0<RetType, T: QFontMetricsF_width_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_width_0<RetType> {
  fn width_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_width_0<f64> for (usize) {
  fn width_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF5widthERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:169
// index:1
// Public Visibility=Default Availability=Available
// [8] qreal width(QChar) const

/*
Returns the width in pixels of the first len characters of text. If len is negative (the default), the entire string is used.

Note that this value is not equal to boundingRect().width(); boundingRect() returns a rectangle describing the pixels this string will cover whereas width() returns the distance to where the next string should be drawn.

See also boundingRect().
*/
impl /*struct*/ QFontMetricsF {
  pub fn width_1<RetType, T: QFontMetricsF_width_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.width_1(self);
    // return 1;
  }
}
pub trait QFontMetricsF_width_1<RetType> {
  fn width_1(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_width_1<f64> for (usize) {
  fn width_1(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF5widthE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:171
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect(const QString &) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetricsF {
  pub fn boundingRect_0<RetType, T: QFontMetricsF_boundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_boundingRect_0<RetType> {
  fn boundingRect_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_boundingRect_0<usize> for (usize) {
  fn boundingRect_0(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF12boundingRectERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:172
// index:1
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect(QChar) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetricsF {
  pub fn boundingRect_1<RetType, T: QFontMetricsF_boundingRect_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_1(self);
    // return 1;
  }
}
pub trait QFontMetricsF_boundingRect_1<RetType> {
  fn boundingRect_1(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_boundingRect_1<usize> for (usize) {
  fn boundingRect_1(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF12boundingRectE5QChar", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:173
// index:2
// Public Visibility=Default Availability=Available
// [32] QRectF boundingRect(const QRectF &, int, const QString &, int, int *) const

/*
Returns the rectangle that is covered by ink if character ch were to be drawn at the origin of the coordinate system.

Note that the bounding rectangle may extend to the left of (0, 0) (e.g., for italicized fonts), and that the text output may cover all pixels in the bounding rectangle. For a space character the rectangle will usually be empty.

Note that the rectangle usually extends both above and below the base line.

Warning: The width of the returned rectangle is not the advance width of the character. Use boundingRect(const QString &) or width() instead.

See also width().
*/
impl /*struct*/ QFontMetricsF {
  pub fn boundingRect_2<RetType, T: QFontMetricsF_boundingRect_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.boundingRect_2(self);
    // return 1;
  }
}
pub trait QFontMetricsF_boundingRect_2<RetType> {
  fn boundingRect_2(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_boundingRect_2<usize> for (usize,i32,usize,i32,usize) {
  fn boundingRect_2(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let arg4 = (&self.4) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF12boundingRectERK6QRectFiRK7QStringiPi", 5,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:174
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF size(int, const QString &, int, int *) const

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
impl /*struct*/ QFontMetricsF {
  pub fn size_0<RetType, T: QFontMetricsF_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_size_0<RetType> {
  fn size_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_size_0<usize> for (i32,usize,i32,usize) {
  fn size_0(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF4sizeEiRK7QStringiPi", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:176
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF tightBoundingRect(const QString &) const

/*
Returns a tight bounding rectangle around the characters in the string specified by text. The bounding rectangle always covers at least the set of pixels the text would cover if drawn at (0, 0).

Note that the bounding rectangle may extend to the left of (0, 0), e.g. for italicized fonts, and that the width of the returned rectangle might be different than what the width() method returns.

If you want to know the advance width of the string (to lay out a set of strings next to each other), use width() instead.

Newline characters are processed as normal characters, not as linebreaks.

Warning: Calling this method is very slow on Windows.

This function was introduced in  Qt 4.3.

See also width(), height(), and boundingRect().
*/
impl /*struct*/ QFontMetricsF {
  pub fn tightBoundingRect_0<RetType, T: QFontMetricsF_tightBoundingRect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tightBoundingRect_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_tightBoundingRect_0<RetType> {
  fn tightBoundingRect_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_tightBoundingRect_0<usize> for (usize) {
  fn tightBoundingRect_0(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF17tightBoundingRectERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:178
// index:0
// Public Visibility=Default Availability=Available
// [8] QString elidedText(const QString &, Qt::TextElideMode, qreal, int) const

/*
If the string text is wider than width, returns an elided version of the string (i.e., a string with "..." in it). Otherwise, returns the original string.

The mode parameter specifies whether the text is elided on the left (e.g., "...tech"), in the middle (e.g., "Tr...ch"), or on the right (e.g., "Trol...").

The width is specified in pixels, not characters.

The flags argument is optional and currently only supports Qt::TextShowMnemonic as value.

The elide mark follows the layoutdirection. For example, it will be on the right side of the text for right-to-left layouts if the mode is Qt::ElideLeft, and on the left side of the text if the mode is Qt::ElideRight.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QFontMetricsF {
  pub fn elidedText_0<RetType, T: QFontMetricsF_elidedText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elidedText_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_elidedText_0<RetType> {
  fn elidedText_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_elidedText_0<usize> for (usize,i32,f64,i32) {
  fn elidedText_0(self , rsthis: & QFontMetricsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF10elidedTextERK7QStringN2Qt13TextElideModeEdi", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:180
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal underlinePos() const

/*
Returns the distance from the base line to where an underscore should be drawn.

See also overlinePos(), strikeOutPos(), and lineWidth().
*/
impl /*struct*/ QFontMetricsF {
  pub fn underlinePos_0<RetType, T: QFontMetricsF_underlinePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.underlinePos_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_underlinePos_0<RetType> {
  fn underlinePos_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_underlinePos_0<f64> for () {
  fn underlinePos_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF12underlinePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:181
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal overlinePos() const

/*
Returns the distance from the base line to where an overline should be drawn.

See also underlinePos(), strikeOutPos(), and lineWidth().
*/
impl /*struct*/ QFontMetricsF {
  pub fn overlinePos_0<RetType, T: QFontMetricsF_overlinePos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.overlinePos_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_overlinePos_0<RetType> {
  fn overlinePos_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_overlinePos_0<f64> for () {
  fn overlinePos_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF11overlinePosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:182
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal strikeOutPos() const

/*
Returns the distance from the base line to where the strikeout line should be drawn.

See also underlinePos(), overlinePos(), and lineWidth().
*/
impl /*struct*/ QFontMetricsF {
  pub fn strikeOutPos_0<RetType, T: QFontMetricsF_strikeOutPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.strikeOutPos_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_strikeOutPos_0<RetType> {
  fn strikeOutPos_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_strikeOutPos_0<f64> for () {
  fn strikeOutPos_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF12strikeOutPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:183
// index:0
// Public Visibility=Default Availability=Available
// [8] qreal lineWidth() const

/*
Returns the width of the underline and strikeout lines, adjusted for the point size of the font.

See also underlinePos(), overlinePos(), and strikeOutPos().
*/
impl /*struct*/ QFontMetricsF {
  pub fn lineWidth_0<RetType, T: QFontMetricsF_lineWidth_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lineWidth_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_lineWidth_0<RetType> {
  fn lineWidth_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_lineWidth_0<f64> for () {
  fn lineWidth_0(self , rsthis: & QFontMetricsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsF9lineWidthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:185
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QFontMetricsF &) const

/*

*/
impl /*struct*/ QFontMetricsF {
  pub fn operator_equal_equal_0<RetType, T: QFontMetricsF_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QFontMetricsF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsFeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qfontmetrics.h:186
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QFontMetricsF &) const

/*

*/
impl /*struct*/ QFontMetricsF {
  pub fn operator_not_equal_0<RetType, T: QFontMetricsF_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QFontMetricsF_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QFontMetricsF) -> RetType;
}
impl<'a> /*trait*/ QFontMetricsF_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QFontMetricsF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QFontMetricsFneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
