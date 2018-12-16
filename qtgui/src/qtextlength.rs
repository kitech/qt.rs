

// mod ::gui::QTextLength
// package qtgui
// /usr/include/qt/QtGui/qtextformat.h
// #include <qtextformat.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 18
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
#[derive(Default)] // class sizeof(QTextLength)=16
pub struct QTextLength {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextLength_ITF interface {
//    QTextLength_PTR() *QTextLength
//}
//func (ptr *QTextLength) QTextLength_PTR() *QTextLength { return ptr }

impl /*struct*/ QTextLength {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextLength {
    return QTextLength{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextLength {
//  type Target = QTextLengthBASE;
//
//  fn deref(&self) -> &QTextLengthBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextLengthBASE> for QTextLength {
//  fn as_ref(& self) -> & QTextLengthBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextformat.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QTextLength()

/*

*/
// QTextLength() ctx.fn_proto_cpp
impl /*struct*/ QTextLength {
  pub fn QTextLength_0<T: QTextLength_QTextLength_0>(value: T) -> QTextLength {
    let rsthis = value.QTextLength_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLength_QTextLength_0 {
  fn QTextLength_0(self) -> QTextLength;
}
// QTextLength() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLength_QTextLength_0 for () {
  fn QTextLength_0(self) -> QTextLength {
    // unsafe{_ZN11QTextLengthC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextLengthC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLength{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:91
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QTextLength(QTextLength::Type, qreal)

/*

*/
// QTextLength(QTextLength::Type, qreal) ctx.fn_proto_cpp
impl /*struct*/ QTextLength {
  pub fn QTextLength_1<T: QTextLength_QTextLength_1>(value: T) -> QTextLength {
    let rsthis = value.QTextLength_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLength_QTextLength_1 {
  fn QTextLength_1(self) -> QTextLength;
}
// QTextLength(QTextLength::Type, qreal) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTextLength_QTextLength_1 for (i32,f64) {
  fn QTextLength_1(self) -> QTextLength {
    // unsafe{_ZN11QTextLengthC2ENS_4TypeEd()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTextLengthC2ENS_4TypeEd", 2,qtrt::FFITY_INT,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTextLength{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:93
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QTextLength::Type type() const

/*
Returns the type of this format.

See also FormatType.
*/
impl /*struct*/ QTextLength {
  pub fn type__0<RetType, T: QTextLength_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QTextLength_type__0<RetType> {
  fn type__0(self , rsthis: & QTextLength) -> RetType;
}
impl<'a> /*trait*/ QTextLength_type__0<i32> for () {
  fn type__0(self , rsthis: & QTextLength) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLength4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal value(qreal) const

/*

*/
impl /*struct*/ QTextLength {
  pub fn value_0<RetType, T: QTextLength_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QTextLength_value_0<RetType> {
  fn value_0(self , rsthis: & QTextLength) -> RetType;
}
impl<'a> /*trait*/ QTextLength_value_0<f64> for (f64) {
  fn value_0(self , rsthis: & QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLength5valueEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:104
// index:0
// Public inline Visibility=Default Availability=Available
// [8] qreal rawValue() const

/*

*/
impl /*struct*/ QTextLength {
  pub fn rawValue_0<RetType, T: QTextLength_rawValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rawValue_0(self);
    // return 1;
  }
}
pub trait QTextLength_rawValue_0<RetType> {
  fn rawValue_0(self , rsthis: & QTextLength) -> RetType;
}
impl<'a> /*trait*/ QTextLength_rawValue_0<f64> for () {
  fn rawValue_0(self , rsthis: & QTextLength) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLength8rawValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:106
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QTextLength &) const

/*

*/
impl /*struct*/ QTextLength {
  pub fn operator_equal_equal_0<RetType, T: QTextLength_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QTextLength_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QTextLength) -> RetType;
}
impl<'a> /*trait*/ QTextLength_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QTextLength) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLengtheqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qtextformat.h:109
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QTextLength &) const

/*

*/
impl /*struct*/ QTextLength {
  pub fn operator_not_equal_0<RetType, T: QTextLength_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QTextLength_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QTextLength) -> RetType;
}
impl<'a> /*trait*/ QTextLength_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QTextLength) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTextLengthneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQTextLength(this :*mut QTextLength) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QTextLengthD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*


*/
pub type QTextLength__Type = i32;
// 
pub const QTextLength__VariableLength :QTextLength__Type = 0;
// 
pub const QTextLength__FixedLength :QTextLength__Type = 1;
// 
pub const QTextLength__PercentageLength :QTextLength__Type = 2;
pub fn QTextLength_TypeItemName(val: i32) ->String {
  match val {
     QTextLength__VariableLength => // 0
     {return String::from("VariableLength");}
     QTextLength__FixedLength => // 1
     {return String::from("FixedLength");}
     QTextLength__PercentageLength => // 2
     {return String::from("PercentageLength");}
  _ => {return format!("{}", val);}
}
}
pub fn QTextLength_TypeItemName_s(val: i32) ->String {
  //var nilthis *QTextLength
  //return nilthis.TypeItemName(val);
  return QTextLength_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
