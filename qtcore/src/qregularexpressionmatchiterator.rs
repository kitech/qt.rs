

// mod ::core::QRegularExpressionMatchIterator
// package qtcore
// /usr/include/qt/QtCore/qregularexpression.h
// #include <qregularexpression.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 30
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
#[derive(Default)] // class sizeof(QRegularExpressionMatchIterator)=8
pub struct QRegularExpressionMatchIterator {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegularExpressionMatchIterator_ITF interface {
//    QRegularExpressionMatchIterator_PTR() *QRegularExpressionMatchIterator
//}
//func (ptr *QRegularExpressionMatchIterator) QRegularExpressionMatchIterator_PTR() *QRegularExpressionMatchIterator { return ptr }

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegularExpressionMatchIterator {
    return QRegularExpressionMatchIterator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegularExpressionMatchIterator {
//  type Target = QRegularExpressionMatchIteratorBASE;
//
//  fn deref(&self) -> &QRegularExpressionMatchIteratorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegularExpressionMatchIteratorBASE> for QRegularExpressionMatchIterator {
//  fn as_ref(& self) -> & QRegularExpressionMatchIteratorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qregularexpression.h:249
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegularExpressionMatchIterator()

/*

*/
// QRegularExpressionMatchIterator() ctx.fn_proto_cpp
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn QRegularExpressionMatchIterator_0<T: QRegularExpressionMatchIterator_QRegularExpressionMatchIterator_0>(value: T) -> QRegularExpressionMatchIterator {
    let rsthis = value.QRegularExpressionMatchIterator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_QRegularExpressionMatchIterator_0 {
  fn QRegularExpressionMatchIterator_0(self) -> QRegularExpressionMatchIterator;
}
// QRegularExpressionMatchIterator() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegularExpressionMatchIterator_QRegularExpressionMatchIterator_0 for () {
  fn QRegularExpressionMatchIterator_0(self) -> QRegularExpressionMatchIterator {
    // unsafe{_ZN31QRegularExpressionMatchIteratorC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN31QRegularExpressionMatchIteratorC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:250
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QRegularExpressionMatchIterator()

/*

*/
pub fn DeleteQRegularExpressionMatchIterator(this :*mut QRegularExpressionMatchIterator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN31QRegularExpressionMatchIteratorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qregularexpression.h:252
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatchIterator & operator=(const QRegularExpressionMatchIterator &)

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn operator_equal_0<RetType, T: QRegularExpressionMatchIterator_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRegularExpressionMatchIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN31QRegularExpressionMatchIteratoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:254
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QRegularExpressionMatchIterator & operator=(QRegularExpressionMatchIterator &&)

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn operator_equal_1<RetType, T: QRegularExpressionMatchIterator_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QRegularExpressionMatchIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN31QRegularExpressionMatchIteratoraSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:257
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QRegularExpressionMatchIterator &)

/*
Swaps the regular expression other with this regular expression. This operation is very fast and never fails.
*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn swap_0<RetType, T: QRegularExpressionMatchIterator_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_swap_0<RetType> {
  fn swap_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QRegularExpressionMatchIterator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN31QRegularExpressionMatchIterator4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:259
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the regular expression is a valid regular expression (that is, it contains no syntax errors, etc.), or false otherwise. Use errorString() to obtain a textual description of the error.

See also errorString() and patternErrorOffset().
*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn isValid_0<RetType, T: QRegularExpressionMatchIterator_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRegularExpressionMatchIterator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK31QRegularExpressionMatchIterator7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:261
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasNext() const

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn hasNext_0<RetType, T: QRegularExpressionMatchIterator_hasNext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasNext_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_hasNext_0<RetType> {
  fn hasNext_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_hasNext_0<bool> for () {
  fn hasNext_0(self , rsthis: & QRegularExpressionMatchIterator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK31QRegularExpressionMatchIterator7hasNextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:262
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatch next()

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn next_0<RetType, T: QRegularExpressionMatchIterator_next_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.next_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_next_0<RetType> {
  fn next_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_next_0<usize> for () {
  fn next_0(self , rsthis: & QRegularExpressionMatchIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN31QRegularExpressionMatchIterator4nextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:263
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatch peekNext() const

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn peekNext_0<RetType, T: QRegularExpressionMatchIterator_peekNext_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.peekNext_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_peekNext_0<RetType> {
  fn peekNext_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_peekNext_0<usize> for () {
  fn peekNext_0(self , rsthis: & QRegularExpressionMatchIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK31QRegularExpressionMatchIterator8peekNextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:265
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpression regularExpression() const

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn regularExpression_0<RetType, T: QRegularExpressionMatchIterator_regularExpression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.regularExpression_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_regularExpression_0<RetType> {
  fn regularExpression_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_regularExpression_0<usize> for () {
  fn regularExpression_0(self , rsthis: & QRegularExpressionMatchIterator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK31QRegularExpressionMatchIterator17regularExpressionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:266
// index:0
// Public Visibility=Default Availability=Available
// [4] QRegularExpression::MatchType matchType() const

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn matchType_0<RetType, T: QRegularExpressionMatchIterator_matchType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matchType_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_matchType_0<RetType> {
  fn matchType_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_matchType_0<i32> for () {
  fn matchType_0(self , rsthis: & QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK31QRegularExpressionMatchIterator9matchTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:267
// index:0
// Public Visibility=Default Availability=Available
// [4] QRegularExpression::MatchOptions matchOptions() const

/*

*/
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn matchOptions_0<RetType, T: QRegularExpressionMatchIterator_matchOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matchOptions_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatchIterator_matchOptions_0<RetType> {
  fn matchOptions_0(self , rsthis: & QRegularExpressionMatchIterator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatchIterator_matchOptions_0<i32> for () {
  fn matchOptions_0(self , rsthis: & QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK31QRegularExpressionMatchIterator12matchOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
