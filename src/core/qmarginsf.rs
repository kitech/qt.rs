

// mod ::core::QMarginsF
// package qtcore
// /usr/include/qt/QtCore/qmargins.h
// #include <qmargins.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QMarginsF)=32
pub struct QMarginsF {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMarginsF_ITF interface {
//    QMarginsF_PTR() *QMarginsF
//}
//func (ptr *QMarginsF) QMarginsF_PTR() *QMarginsF { return ptr }

impl /*struct*/ QMarginsF {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMarginsF {
    return QMarginsF{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMarginsF {
//  type Target = QMarginsFBASE;
//
//  fn deref(&self) -> &QMarginsFBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMarginsFBASE> for QMarginsF {
//  fn as_ref(& self) -> & QMarginsFBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmargins.h:288
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMarginsF()

/*

*/
// QMarginsF() ctx.fn_proto_cpp
impl /*struct*/ QMarginsF {
  pub fn QMarginsF_0<T: QMarginsF_QMarginsF_0>(value: T) -> QMarginsF {
    let rsthis = value.QMarginsF_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMarginsF_QMarginsF_0 {
  fn QMarginsF_0(self) -> QMarginsF;
}
// QMarginsF() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMarginsF_QMarginsF_0 for () {
  fn QMarginsF_0(self) -> QMarginsF {
    // unsafe{_ZN9QMarginsFC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QMarginsFC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMarginsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:289
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QMarginsF(qreal, qreal, qreal, qreal)

/*

*/
// QMarginsF(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl /*struct*/ QMarginsF {
  pub fn QMarginsF_1<T: QMarginsF_QMarginsF_1>(value: T) -> QMarginsF {
    let rsthis = value.QMarginsF_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMarginsF_QMarginsF_1 {
  fn QMarginsF_1(self) -> QMarginsF;
}
// QMarginsF(qreal, qreal, qreal, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMarginsF_QMarginsF_1 for (f64,f64,f64,f64) {
  fn QMarginsF_1(self) -> QMarginsF {
    // unsafe{_ZN9QMarginsFC2Edddd()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const f64 as usize;
    let arg3 = (&self.3) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QMarginsFC2Edddd", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMarginsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:290
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QMarginsF(const QMargins &)

/*

*/
// QMarginsF(const QMargins &) ctx.fn_proto_cpp
impl /*struct*/ QMarginsF {
  pub fn QMarginsF_2<T: QMarginsF_QMarginsF_2>(value: T) -> QMarginsF {
    let rsthis = value.QMarginsF_2();
    return rsthis;
    // return 1;
  }
}

pub trait QMarginsF_QMarginsF_2 {
  fn QMarginsF_2(self) -> QMarginsF;
}
// QMarginsF(const QMargins &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMarginsF_QMarginsF_2 for (usize) {
  fn QMarginsF_2(self) -> QMarginsF {
    // unsafe{_ZN9QMarginsFC2ERK8QMargins()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QMarginsFC2ERK8QMargins", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMarginsF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:292
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if all margins are is 0; otherwise returns false.
*/
impl /*struct*/ QMarginsF {
  pub fn isNull_0<RetType, T: QMarginsF_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QMarginsF_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QMarginsF) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMarginsF6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:294
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal left() const

/*
Returns the left margin.

See also setLeft().
*/
impl /*struct*/ QMarginsF {
  pub fn left_0<RetType, T: QMarginsF_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QMarginsF_left_0<RetType> {
  fn left_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_left_0<f64> for () {
  fn left_0(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMarginsF4leftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:295
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal top() const

/*
Returns the top margin.

See also setTop().
*/
impl /*struct*/ QMarginsF {
  pub fn top_0<RetType, T: QMarginsF_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QMarginsF_top_0<RetType> {
  fn top_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_top_0<f64> for () {
  fn top_0(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMarginsF3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:296
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal right() const

/*
Returns the right margin.

See also setRight().
*/
impl /*struct*/ QMarginsF {
  pub fn right_0<RetType, T: QMarginsF_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QMarginsF_right_0<RetType> {
  fn right_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_right_0<f64> for () {
  fn right_0(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMarginsF5rightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:297
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal bottom() const

/*
Returns the bottom margin.

See also setBottom().
*/
impl /*struct*/ QMarginsF {
  pub fn bottom_0<RetType, T: QMarginsF_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QMarginsF_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_bottom_0<f64> for () {
  fn bottom_0(self , rsthis: & QMarginsF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMarginsF6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:299
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLeft(qreal)

/*
Sets the left margin to left.

See also left().
*/
impl /*struct*/ QMarginsF {
  pub fn setLeft_0<RetType, T: QMarginsF_setLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeft_0(self);
    // return 1;
  }
}
pub trait QMarginsF_setLeft_0<RetType> {
  fn setLeft_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_setLeft_0<(/*void*/)> for (f64) {
  fn setLeft_0(self , rsthis: & QMarginsF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QMarginsF7setLeftEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:300
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTop(qreal)

/*
Sets the Top margin to Top.

See also top().
*/
impl /*struct*/ QMarginsF {
  pub fn setTop_0<RetType, T: QMarginsF_setTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTop_0(self);
    // return 1;
  }
}
pub trait QMarginsF_setTop_0<RetType> {
  fn setTop_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_setTop_0<(/*void*/)> for (f64) {
  fn setTop_0(self , rsthis: & QMarginsF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QMarginsF6setTopEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:301
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRight(qreal)

/*
Sets the right margin to right.

See also right().
*/
impl /*struct*/ QMarginsF {
  pub fn setRight_0<RetType, T: QMarginsF_setRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRight_0(self);
    // return 1;
  }
}
pub trait QMarginsF_setRight_0<RetType> {
  fn setRight_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_setRight_0<(/*void*/)> for (f64) {
  fn setRight_0(self , rsthis: & QMarginsF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QMarginsF8setRightEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:302
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBottom(qreal)

/*
Sets the bottom margin to bottom.

See also bottom().
*/
impl /*struct*/ QMarginsF {
  pub fn setBottom_0<RetType, T: QMarginsF_setBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottom_0(self);
    // return 1;
  }
}
pub trait QMarginsF_setBottom_0<RetType> {
  fn setBottom_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_setBottom_0<(/*void*/)> for (f64) {
  fn setBottom_0(self , rsthis: & QMarginsF) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN9QMarginsF9setBottomEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:304
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF & operator+=(const QMarginsF &)

/*

*/
impl /*struct*/ QMarginsF {
  pub fn operator_add_equal_0<RetType, T: QMarginsF_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QMarginsF_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMarginsFpLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:306
// index:1
// Public Visibility=Default Availability=Available
// [32] QMarginsF & operator+=(qreal)

/*

*/
impl /*struct*/ QMarginsF {
  pub fn operator_add_equal_1<RetType, T: QMarginsF_operator_add_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_1(self);
    // return 1;
  }
}
pub trait QMarginsF_operator_add_equal_1<RetType> {
  fn operator_add_equal_1(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_operator_add_equal_1<usize> for (f64) {
  fn operator_add_equal_1(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMarginsFpLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:305
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF & operator-=(const QMarginsF &)

/*

*/
impl /*struct*/ QMarginsF {
  pub fn operator_minus_equal_0<RetType, T: QMarginsF_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QMarginsF_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMarginsFmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:307
// index:1
// Public Visibility=Default Availability=Available
// [32] QMarginsF & operator-=(qreal)

/*

*/
impl /*struct*/ QMarginsF {
  pub fn operator_minus_equal_1<RetType, T: QMarginsF_operator_minus_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_1(self);
    // return 1;
  }
}
pub trait QMarginsF_operator_minus_equal_1<RetType> {
  fn operator_minus_equal_1(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_operator_minus_equal_1<usize> for (f64) {
  fn operator_minus_equal_1(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMarginsFmIEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:308
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF & operator*=(qreal)

/*

*/
impl /*struct*/ QMarginsF {
  pub fn operator_mul_equal_0<RetType, T: QMarginsF_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QMarginsF_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_operator_mul_equal_0<usize> for (f64) {
  fn operator_mul_equal_0(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMarginsFmLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:309
// index:0
// Public Visibility=Default Availability=Available
// [32] QMarginsF & operator/=(qreal)

/*

*/
impl /*struct*/ QMarginsF {
  pub fn operator_div_equal_0<RetType, T: QMarginsF_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QMarginsF_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_operator_div_equal_0<usize> for (f64) {
  fn operator_div_equal_0(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QMarginsFdVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:311
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QMargins toMargins() const

/*

*/
impl /*struct*/ QMarginsF {
  pub fn toMargins_0<RetType, T: QMarginsF_toMargins_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toMargins_0(self);
    // return 1;
  }
}
pub trait QMarginsF_toMargins_0<RetType> {
  fn toMargins_0(self , rsthis: & QMarginsF) -> RetType;
}
impl<'a> /*trait*/ QMarginsF_toMargins_0<usize> for () {
  fn toMargins_0(self , rsthis: & QMarginsF) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QMarginsF9toMarginsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMarginsF(this :*mut QMarginsF) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN9QMarginsFD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
