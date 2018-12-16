

// mod ::core::QStringMatcher
// package qtcore
// /usr/include/qt/QtCore/qstringmatcher.h
// #include <qstringmatcher.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 32
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
#[derive(Default)] // class sizeof(QStringMatcher)=1048
pub struct QStringMatcher {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStringMatcher_ITF interface {
//    QStringMatcher_PTR() *QStringMatcher
//}
//func (ptr *QStringMatcher) QStringMatcher_PTR() *QStringMatcher { return ptr }

impl /*struct*/ QStringMatcher {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStringMatcher {
    return QStringMatcher{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStringMatcher {
//  type Target = QStringMatcherBASE;
//
//  fn deref(&self) -> &QStringMatcherBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringMatcherBASE> for QStringMatcher {
//  fn as_ref(& self) -> & QStringMatcherBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstringmatcher.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStringMatcher()

/*
Constructs an empty string matcher that won't match anything. Call setPattern() to give it a pattern to match.
*/
// QStringMatcher() ctx.fn_proto_cpp
impl /*struct*/ QStringMatcher {
  pub fn QStringMatcher_0<T: QStringMatcher_QStringMatcher_0>(value: T) -> QStringMatcher {
    let rsthis = value.QStringMatcher_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStringMatcher_QStringMatcher_0 {
  fn QStringMatcher_0(self) -> QStringMatcher;
}
// QStringMatcher() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringMatcher_QStringMatcher_0 for () {
  fn QStringMatcher_0(self) -> QStringMatcher {
    // unsafe{_ZN14QStringMatcherC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStringMatcherC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:54
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStringMatcher(const QString &, Qt::CaseSensitivity)

/*
Constructs an empty string matcher that won't match anything. Call setPattern() to give it a pattern to match.
*/
// QStringMatcher(const QString &, Qt::CaseSensitivity) ctx.fn_proto_cpp
impl /*struct*/ QStringMatcher {
  pub fn QStringMatcher_1<T: QStringMatcher_QStringMatcher_1>(value: T) -> QStringMatcher {
    let rsthis = value.QStringMatcher_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStringMatcher_QStringMatcher_1 {
  fn QStringMatcher_1(self) -> QStringMatcher;
}
// QStringMatcher(const QString &, Qt::CaseSensitivity) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringMatcher_QStringMatcher_1 for (usize,i32) {
  fn QStringMatcher_1(self) -> QStringMatcher {
    // unsafe{_ZN14QStringMatcherC2ERK7QStringN2Qt15CaseSensitivityE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStringMatcherC2ERK7QStringN2Qt15CaseSensitivityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:56
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QStringMatcher(const QChar *, int, Qt::CaseSensitivity)

/*
Constructs an empty string matcher that won't match anything. Call setPattern() to give it a pattern to match.
*/
// QStringMatcher(const QChar *, int, Qt::CaseSensitivity) ctx.fn_proto_cpp
impl /*struct*/ QStringMatcher {
  pub fn QStringMatcher_2<T: QStringMatcher_QStringMatcher_2>(value: T) -> QStringMatcher {
    let rsthis = value.QStringMatcher_2();
    return rsthis;
    // return 1;
  }
}

pub trait QStringMatcher_QStringMatcher_2 {
  fn QStringMatcher_2(self) -> QStringMatcher;
}
// QStringMatcher(const QChar *, int, Qt::CaseSensitivity) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStringMatcher_QStringMatcher_2 for (usize,i32,i32) {
  fn QStringMatcher_2(self) -> QStringMatcher {
    // unsafe{_ZN14QStringMatcherC2EPK5QChariN2Qt15CaseSensitivityE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QStringMatcherC2EPK5QChariN2Qt15CaseSensitivityE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStringMatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStringMatcher()

/*

*/
pub fn DeleteQStringMatcher(this :*mut QStringMatcher) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QStringMatcherD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 1048)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qstringmatcher.h:61
// index:0
// Public Visibility=Default Availability=Available
// [1048] QStringMatcher & operator=(const QStringMatcher &)

/*

*/
impl /*struct*/ QStringMatcher {
  pub fn operator_equal_0<RetType, T: QStringMatcher_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QStringMatcher_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QStringMatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QStringMatcheraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPattern(const QString &)

/*
Sets the string that this string matcher will search for to pattern.

See also pattern(), setCaseSensitivity(), and indexIn().
*/
impl /*struct*/ QStringMatcher {
  pub fn setPattern_0<RetType, T: QStringMatcher_setPattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPattern_0(self);
    // return 1;
  }
}
pub trait QStringMatcher_setPattern_0<RetType> {
  fn setPattern_0(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_setPattern_0<(/*void*/)> for (usize) {
  fn setPattern_0(self , rsthis: & QStringMatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QStringMatcher10setPatternERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCaseSensitivity(Qt::CaseSensitivity)

/*
Sets the case sensitivity setting of this string matcher to cs.

See also caseSensitivity(), setPattern(), and indexIn().
*/
impl /*struct*/ QStringMatcher {
  pub fn setCaseSensitivity_0<RetType, T: QStringMatcher_setCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QStringMatcher_setCaseSensitivity_0<RetType> {
  fn setCaseSensitivity_0(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_setCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setCaseSensitivity_0(self , rsthis: & QStringMatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QStringMatcher18setCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:66
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexIn(const QString &, int) const

/*
Searches the string str from character position from (default 0, i.e. from the first character), for the string pattern() that was set in the constructor or in the most recent call to setPattern(). Returns the position where the pattern() matched in str, or -1 if no match was found.

See also setPattern() and setCaseSensitivity().
*/
impl /*struct*/ QStringMatcher {
  pub fn indexIn_0<RetType, T: QStringMatcher_indexIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexIn_0(self);
    // return 1;
  }
}
pub trait QStringMatcher_indexIn_0<RetType> {
  fn indexIn_0(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_indexIn_0<i32> for (usize,i32) {
  fn indexIn_0(self , rsthis: & QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStringMatcher7indexInERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:67
// index:1
// Public Visibility=Default Availability=Available
// [4] int indexIn(const QChar *, int, int) const

/*
Searches the string str from character position from (default 0, i.e. from the first character), for the string pattern() that was set in the constructor or in the most recent call to setPattern(). Returns the position where the pattern() matched in str, or -1 if no match was found.

See also setPattern() and setCaseSensitivity().
*/
impl /*struct*/ QStringMatcher {
  pub fn indexIn_1<RetType, T: QStringMatcher_indexIn_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexIn_1(self);
    // return 1;
  }
}
pub trait QStringMatcher_indexIn_1<RetType> {
  fn indexIn_1(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_indexIn_1<i32> for (usize,i32,i32) {
  fn indexIn_1(self , rsthis: & QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStringMatcher7indexInEPK5QCharii", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QString pattern() const

/*
Returns the string pattern that this string matcher will search for.

See also setPattern().
*/
impl /*struct*/ QStringMatcher {
  pub fn pattern_0<RetType, T: QStringMatcher_pattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pattern_0(self);
    // return 1;
  }
}
pub trait QStringMatcher_pattern_0<RetType> {
  fn pattern_0(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_pattern_0<usize> for () {
  fn pattern_0(self , rsthis: & QStringMatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStringMatcher7patternEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstringmatcher.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity caseSensitivity() const

/*
Returns the case sensitivity setting for this string matcher.

See also setCaseSensitivity().
*/
impl /*struct*/ QStringMatcher {
  pub fn caseSensitivity_0<RetType, T: QStringMatcher_caseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.caseSensitivity_0(self);
    // return 1;
  }
}
pub trait QStringMatcher_caseSensitivity_0<RetType> {
  fn caseSensitivity_0(self , rsthis: & QStringMatcher) -> RetType;
}
impl<'a> /*trait*/ QStringMatcher_caseSensitivity_0<i32> for () {
  fn caseSensitivity_0(self , rsthis: & QStringMatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QStringMatcher15caseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
