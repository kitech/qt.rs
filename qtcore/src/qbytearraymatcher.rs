

// mod ::core::QByteArrayMatcher
// package qtcore
// /usr/include/qt/QtCore/qbytearraymatcher.h
// #include <qbytearraymatcher.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 21
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
#[derive(Default)] // class sizeof(QByteArrayMatcher)=1040
pub struct QByteArrayMatcher {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QByteArrayMatcher_ITF interface {
//    QByteArrayMatcher_PTR() *QByteArrayMatcher
//}
//func (ptr *QByteArrayMatcher) QByteArrayMatcher_PTR() *QByteArrayMatcher { return ptr }

impl /*struct*/ QByteArrayMatcher {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QByteArrayMatcher {
    return QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QByteArrayMatcher {
//  type Target = QByteArrayMatcherBASE;
//
//  fn deref(&self) -> &QByteArrayMatcherBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QByteArrayMatcherBASE> for QByteArrayMatcher {
//  fn as_ref(& self) -> & QByteArrayMatcherBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbytearraymatcher.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QByteArrayMatcher()

/*
Constructs an empty byte array matcher that won't match anything. Call setPattern() to give it a pattern to match.
*/
// QByteArrayMatcher() ctx.fn_proto_cpp
impl /*struct*/ QByteArrayMatcher {
  pub fn QByteArrayMatcher_0<T: QByteArrayMatcher_QByteArrayMatcher_0>(value: T) -> QByteArrayMatcher {
    let rsthis = value.QByteArrayMatcher_0();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArrayMatcher_QByteArrayMatcher_0 {
  fn QByteArrayMatcher_0(self) -> QByteArrayMatcher;
}
// QByteArrayMatcher() ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArrayMatcher_QByteArrayMatcher_0 for () {
  fn QByteArrayMatcher_0(self) -> QByteArrayMatcher {
    // unsafe{_ZN17QByteArrayMatcherC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QByteArrayMatcherC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:54
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QByteArrayMatcher(const QByteArray &)

/*
Constructs an empty byte array matcher that won't match anything. Call setPattern() to give it a pattern to match.
*/
// QByteArrayMatcher(const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QByteArrayMatcher {
  pub fn QByteArrayMatcher_1<T: QByteArrayMatcher_QByteArrayMatcher_1>(value: T) -> QByteArrayMatcher {
    let rsthis = value.QByteArrayMatcher_1();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArrayMatcher_QByteArrayMatcher_1 {
  fn QByteArrayMatcher_1(self) -> QByteArrayMatcher;
}
// QByteArrayMatcher(const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArrayMatcher_QByteArrayMatcher_1 for (usize) {
  fn QByteArrayMatcher_1(self) -> QByteArrayMatcher {
    // unsafe{_ZN17QByteArrayMatcherC2ERK10QByteArray()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QByteArrayMatcherC2ERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:55
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QByteArrayMatcher(const char *, int)

/*
Constructs an empty byte array matcher that won't match anything. Call setPattern() to give it a pattern to match.
*/
// QByteArrayMatcher(const char *, int) ctx.fn_proto_cpp
impl /*struct*/ QByteArrayMatcher {
  pub fn QByteArrayMatcher_2<T: QByteArrayMatcher_QByteArrayMatcher_2>(value: T) -> QByteArrayMatcher {
    let rsthis = value.QByteArrayMatcher_2();
    return rsthis;
    // return 1;
  }
}

pub trait QByteArrayMatcher_QByteArrayMatcher_2 {
  fn QByteArrayMatcher_2(self) -> QByteArrayMatcher;
}
// QByteArrayMatcher(const char *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QByteArrayMatcher_QByteArrayMatcher_2 for (usize,i32) {
  fn QByteArrayMatcher_2(self) -> QByteArrayMatcher {
    // unsafe{_ZN17QByteArrayMatcherC2EPKci()};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QByteArrayMatcherC2EPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QByteArrayMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QByteArrayMatcher()

/*

*/
pub fn DeleteQByteArrayMatcher(this :*mut QByteArrayMatcher) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QByteArrayMatcherD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 1040)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qbytearraymatcher.h:59
// index:0
// Public Visibility=Default Availability=Available
// [1040] QByteArrayMatcher & operator=(const QByteArrayMatcher &)

/*

*/
impl /*struct*/ QByteArrayMatcher {
  pub fn operator_equal_0<RetType, T: QByteArrayMatcher_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QByteArrayMatcher_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QByteArrayMatcher) -> RetType;
}
impl<'a> /*trait*/ QByteArrayMatcher_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QByteArrayMatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QByteArrayMatcheraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPattern(const QByteArray &)

/*
Sets the byte array that this byte array matcher will search for to pattern.

See also pattern() and indexIn().
*/
impl /*struct*/ QByteArrayMatcher {
  pub fn setPattern_0<RetType, T: QByteArrayMatcher_setPattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPattern_0(self);
    // return 1;
  }
}
pub trait QByteArrayMatcher_setPattern_0<RetType> {
  fn setPattern_0(self , rsthis: & QByteArrayMatcher) -> RetType;
}
impl<'a> /*trait*/ QByteArrayMatcher_setPattern_0<(/*void*/)> for (usize) {
  fn setPattern_0(self , rsthis: & QByteArrayMatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QByteArrayMatcher10setPatternERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:63
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexIn(const QByteArray &, int) const

/*
Searches the byte array ba, from byte position from (default 0, i.e. from the first byte), for the byte array pattern() that was set in the constructor or in the most recent call to setPattern(). Returns the position where the pattern() matched in ba, or -1 if no match was found.
*/
impl /*struct*/ QByteArrayMatcher {
  pub fn indexIn_0<RetType, T: QByteArrayMatcher_indexIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexIn_0(self);
    // return 1;
  }
}
pub trait QByteArrayMatcher_indexIn_0<RetType> {
  fn indexIn_0(self , rsthis: & QByteArrayMatcher) -> RetType;
}
impl<'a> /*trait*/ QByteArrayMatcher_indexIn_0<i32> for (usize,i32) {
  fn indexIn_0(self , rsthis: & QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QByteArrayMatcher7indexInERK10QByteArrayi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:64
// index:1
// Public Visibility=Default Availability=Available
// [4] int indexIn(const char *, int, int) const

/*
Searches the byte array ba, from byte position from (default 0, i.e. from the first byte), for the byte array pattern() that was set in the constructor or in the most recent call to setPattern(). Returns the position where the pattern() matched in ba, or -1 if no match was found.
*/
impl /*struct*/ QByteArrayMatcher {
  pub fn indexIn_1<RetType, T: QByteArrayMatcher_indexIn_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexIn_1(self);
    // return 1;
  }
}
pub trait QByteArrayMatcher_indexIn_1<RetType> {
  fn indexIn_1(self , rsthis: & QByteArrayMatcher) -> RetType;
}
impl<'a> /*trait*/ QByteArrayMatcher_indexIn_1<i32> for (usize,i32,i32) {
  fn indexIn_1(self , rsthis: & QByteArrayMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QByteArrayMatcher7indexInEPKcii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbytearraymatcher.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QByteArray pattern() const

/*
Returns the byte array pattern that this byte array matcher will search for.

See also setPattern().
*/
impl /*struct*/ QByteArrayMatcher {
  pub fn pattern_0<RetType, T: QByteArrayMatcher_pattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pattern_0(self);
    // return 1;
  }
}
pub trait QByteArrayMatcher_pattern_0<RetType> {
  fn pattern_0(self , rsthis: & QByteArrayMatcher) -> RetType;
}
impl<'a> /*trait*/ QByteArrayMatcher_pattern_0<usize> for () {
  fn pattern_0(self , rsthis: & QByteArrayMatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QByteArrayMatcher7patternEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
