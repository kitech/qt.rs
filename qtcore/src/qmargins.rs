

// mod ::core::QMargins
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
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QMargins)=16
pub struct QMargins {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMargins_ITF interface {
//    QMargins_PTR() *QMargins
//}
//func (ptr *QMargins) QMargins_PTR() *QMargins { return ptr }

impl /*struct*/ QMargins {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMargins {
    return QMargins{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMargins {
//  type Target = QMarginsBASE;
//
//  fn deref(&self) -> &QMarginsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMarginsBASE> for QMargins {
//  fn as_ref(& self) -> & QMarginsBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmargins.h:54
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMargins()

/*
Constructs a margins object with all margins set to 0.

See also isNull().
*/
// QMargins() ctx.fn_proto_cpp
impl /*struct*/ QMargins {
  pub fn QMargins_0<T: QMargins_QMargins_0>(value: T) -> QMargins {
    let rsthis = value.QMargins_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMargins_QMargins_0 {
  fn QMargins_0(self) -> QMargins;
}
// QMargins() ctx.fn_proto_cpp
impl<'a> /*trait*/ QMargins_QMargins_0 for () {
  fn QMargins_0(self) -> QMargins {
    // unsafe{_ZN8QMarginsC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QMarginsC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMargins{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:55
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QMargins(int, int, int, int)

/*
Constructs a margins object with all margins set to 0.

See also isNull().
*/
// QMargins(int, int, int, int) ctx.fn_proto_cpp
impl /*struct*/ QMargins {
  pub fn QMargins_1<T: QMargins_QMargins_1>(value: T) -> QMargins {
    let rsthis = value.QMargins_1();
    return rsthis;
    // return 1;
  }
}

pub trait QMargins_QMargins_1 {
  fn QMargins_1(self) -> QMargins;
}
// QMargins(int, int, int, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMargins_QMargins_1 for (i32,i32,i32,i32) {
  fn QMargins_1(self) -> QMargins {
    // unsafe{_ZN8QMarginsC2Eiiii()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QMarginsC2Eiiii", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMargins{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isNull() const

/*
Returns true if all margins are is 0; otherwise returns false.
*/
impl /*struct*/ QMargins {
  pub fn isNull_0<RetType, T: QMargins_isNull_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNull_0(self);
    // return 1;
  }
}
pub trait QMargins_isNull_0<RetType> {
  fn isNull_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_isNull_0<bool> for () {
  fn isNull_0(self , rsthis: & QMargins) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMargins6isNullEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int left() const

/*
Returns the left margin.

See also setLeft().
*/
impl /*struct*/ QMargins {
  pub fn left_0<RetType, T: QMargins_left_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.left_0(self);
    // return 1;
  }
}
pub trait QMargins_left_0<RetType> {
  fn left_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_left_0<i32> for () {
  fn left_0(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMargins4leftEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int top() const

/*
Returns the top margin.

See also setTop().
*/
impl /*struct*/ QMargins {
  pub fn top_0<RetType, T: QMargins_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QMargins_top_0<RetType> {
  fn top_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_top_0<i32> for () {
  fn top_0(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMargins3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:61
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int right() const

/*
Returns the right margin.

See also setRight().
*/
impl /*struct*/ QMargins {
  pub fn right_0<RetType, T: QMargins_right_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.right_0(self);
    // return 1;
  }
}
pub trait QMargins_right_0<RetType> {
  fn right_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_right_0<i32> for () {
  fn right_0(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMargins5rightEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:62
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int bottom() const

/*
Returns the bottom margin.

See also setBottom().
*/
impl /*struct*/ QMargins {
  pub fn bottom_0<RetType, T: QMargins_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QMargins_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_bottom_0<i32> for () {
  fn bottom_0(self , rsthis: & QMargins) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QMargins6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLeft(int)

/*
Sets the left margin to left.

See also left().
*/
impl /*struct*/ QMargins {
  pub fn setLeft_0<RetType, T: QMargins_setLeft_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLeft_0(self);
    // return 1;
  }
}
pub trait QMargins_setLeft_0<RetType> {
  fn setLeft_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_setLeft_0<(/*void*/)> for (i32) {
  fn setLeft_0(self , rsthis: & QMargins) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMargins7setLeftEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTop(int)

/*
Sets the Top margin to Top.

See also top().
*/
impl /*struct*/ QMargins {
  pub fn setTop_0<RetType, T: QMargins_setTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTop_0(self);
    // return 1;
  }
}
pub trait QMargins_setTop_0<RetType> {
  fn setTop_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_setTop_0<(/*void*/)> for (i32) {
  fn setTop_0(self , rsthis: & QMargins) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMargins6setTopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRight(int)

/*
Sets the right margin to right.

See also right().
*/
impl /*struct*/ QMargins {
  pub fn setRight_0<RetType, T: QMargins_setRight_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRight_0(self);
    // return 1;
  }
}
pub trait QMargins_setRight_0<RetType> {
  fn setRight_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_setRight_0<(/*void*/)> for (i32) {
  fn setRight_0(self , rsthis: & QMargins) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMargins8setRightEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBottom(int)

/*
Sets the bottom margin to bottom.

See also bottom().
*/
impl /*struct*/ QMargins {
  pub fn setBottom_0<RetType, T: QMargins_setBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottom_0(self);
    // return 1;
  }
}
pub trait QMargins_setBottom_0<RetType> {
  fn setBottom_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_setBottom_0<(/*void*/)> for (i32) {
  fn setBottom_0(self , rsthis: & QMargins) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QMargins9setBottomEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmargins.h:69
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins & operator+=(const QMargins &)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_add_equal_0<RetType, T: QMargins_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QMargins_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_add_equal_0<usize> for (usize) {
  fn operator_add_equal_0(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginspLERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:71
// index:1
// Public Visibility=Default Availability=Available
// [16] QMargins & operator+=(int)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_add_equal_1<RetType, T: QMargins_operator_add_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_1(self);
    // return 1;
  }
}
pub trait QMargins_operator_add_equal_1<RetType> {
  fn operator_add_equal_1(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_add_equal_1<usize> for (i32) {
  fn operator_add_equal_1(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginspLEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:70
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins & operator-=(const QMargins &)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_minus_equal_0<RetType, T: QMargins_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QMargins_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_minus_equal_0<usize> for (usize) {
  fn operator_minus_equal_0(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginsmIERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:72
// index:1
// Public Visibility=Default Availability=Available
// [16] QMargins & operator-=(int)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_minus_equal_1<RetType, T: QMargins_operator_minus_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_1(self);
    // return 1;
  }
}
pub trait QMargins_operator_minus_equal_1<RetType> {
  fn operator_minus_equal_1(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_minus_equal_1<usize> for (i32) {
  fn operator_minus_equal_1(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginsmIEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:73
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins & operator*=(int)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_mul_equal_0<RetType, T: QMargins_operator_mul_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_0(self);
    // return 1;
  }
}
pub trait QMargins_operator_mul_equal_0<RetType> {
  fn operator_mul_equal_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_mul_equal_0<usize> for (i32) {
  fn operator_mul_equal_0(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginsmLEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:75
// index:1
// Public Visibility=Default Availability=Available
// [16] QMargins & operator*=(qreal)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_mul_equal_1<RetType, T: QMargins_operator_mul_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_equal_1(self);
    // return 1;
  }
}
pub trait QMargins_operator_mul_equal_1<RetType> {
  fn operator_mul_equal_1(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_mul_equal_1<usize> for (f64) {
  fn operator_mul_equal_1(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginsmLEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:74
// index:0
// Public Visibility=Default Availability=Available
// [16] QMargins & operator/=(int)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_div_equal_0<RetType, T: QMargins_operator_div_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_0(self);
    // return 1;
  }
}
pub trait QMargins_operator_div_equal_0<RetType> {
  fn operator_div_equal_0(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_div_equal_0<usize> for (i32) {
  fn operator_div_equal_0(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginsdVEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmargins.h:76
// index:1
// Public Visibility=Default Availability=Available
// [16] QMargins & operator/=(qreal)

/*

*/
impl /*struct*/ QMargins {
  pub fn operator_div_equal_1<RetType, T: QMargins_operator_div_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_div_equal_1(self);
    // return 1;
  }
}
pub trait QMargins_operator_div_equal_1<RetType> {
  fn operator_div_equal_1(self , rsthis: & QMargins) -> RetType;
}
impl<'a> /*trait*/ QMargins_operator_div_equal_1<usize> for (f64) {
  fn operator_div_equal_1(self , rsthis: & QMargins) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QMarginsdVEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMargins(this :*mut QMargins) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN8QMarginsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
