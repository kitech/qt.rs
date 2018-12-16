

// mod ::core::QRegExp
// package qtcore
// /usr/include/qt/QtCore/qregexp.h
// #include <qregexp.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QRegExp)=8
pub struct QRegExp {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegExp_ITF interface {
//    QRegExp_PTR() *QRegExp
//}
//func (ptr *QRegExp) QRegExp_PTR() *QRegExp { return ptr }

impl /*struct*/ QRegExp {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegExp {
    return QRegExp{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegExp {
//  type Target = QRegExpBASE;
//
//  fn deref(&self) -> &QRegExpBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegExpBASE> for QRegExp {
//  fn as_ref(& self) -> & QRegExpBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qregexp.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegExp()

/*
Constructs an empty regexp.

See also isValid() and errorString().
*/
// QRegExp() ctx.fn_proto_cpp
impl /*struct*/ QRegExp {
  pub fn QRegExp_0<T: QRegExp_QRegExp_0>(value: T) -> QRegExp {
    let rsthis = value.QRegExp_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExp_QRegExp_0 {
  fn QRegExp_0(self) -> QRegExp;
}
// QRegExp() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegExp_QRegExp_0 for () {
  fn QRegExp_0(self) -> QRegExp {
    // unsafe{_ZN7QRegExpC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegExpC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegExp{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:71
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRegExp(const QString &, Qt::CaseSensitivity, QRegExp::PatternSyntax)

/*
Constructs an empty regexp.

See also isValid() and errorString().
*/
// QRegExp(const QString &, Qt::CaseSensitivity, QRegExp::PatternSyntax) ctx.fn_proto_cpp
impl /*struct*/ QRegExp {
  pub fn QRegExp_1<T: QRegExp_QRegExp_1>(value: T) -> QRegExp {
    let rsthis = value.QRegExp_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExp_QRegExp_1 {
  fn QRegExp_1(self) -> QRegExp;
}
// QRegExp(const QString &, Qt::CaseSensitivity, QRegExp::PatternSyntax) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegExp_QRegExp_1 for (usize,i32,i32) {
  fn QRegExp_1(self) -> QRegExp {
    // unsafe{_ZN7QRegExpC2ERK7QStringN2Qt15CaseSensitivityENS_13PatternSyntaxE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QRegExpC2ERK7QStringN2Qt15CaseSensitivityENS_13PatternSyntaxE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegExp{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QRegExp()

/*

*/
pub fn DeleteQRegExp(this :*mut QRegExp) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QRegExpD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qregexp.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegExp & operator=(const QRegExp &)

/*

*/
impl /*struct*/ QRegExp {
  pub fn operator_equal_0<RetType, T: QRegExp_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRegExp_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExpaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:77
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QRegExp & operator=(QRegExp &&)

/*

*/
impl /*struct*/ QRegExp {
  pub fn operator_equal_1<RetType, T: QRegExp_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QRegExp_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExpaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:79
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QRegExp &)

/*
Swaps regular expression other with this regular expression. This operation is very fast and never fails.

This function was introduced in  Qt 4.8.
*/
impl /*struct*/ QRegExp {
  pub fn swap_0<RetType, T: QRegExp_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QRegExp_swap_0<RetType> {
  fn swap_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QRegExp) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegExp4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QRegExp &) const

/*

*/
impl /*struct*/ QRegExp {
  pub fn operator_equal_equal_0<RetType, T: QRegExp_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QRegExp_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QRegExp) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExpeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:82
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QRegExp &) const

/*

*/
impl /*struct*/ QRegExp {
  pub fn operator_not_equal_0<RetType, T: QRegExp_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QRegExp_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QRegExp) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExpneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:84
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty() const

/*
Returns true if the pattern string is empty; otherwise returns false.

If you call exactMatch() with an empty pattern on an empty string it will return true; otherwise it returns false since it operates over the whole string. If you call indexIn() with an empty pattern on any string it will return the start offset (0 by default) because the empty pattern matches the 'emptiness' at the start of the string. In this case the length of the match returned by matchedLength() will be 0.

See QString::isEmpty().
*/
impl /*struct*/ QRegExp {
  pub fn isEmpty_0<RetType, T: QRegExp_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QRegExp_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_isEmpty_0<bool> for () {
  fn isEmpty_0(self , rsthis: & QRegExp) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp7isEmptyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:85
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the regular expression is valid; otherwise returns false. An invalid regular expression never matches.

The pattern [a-z is an example of an invalid pattern, since it lacks a closing square bracket.

Note that the validity of a regexp may also depend on the setting of the wildcard flag, for example *.html is a valid wildcard regexp but an invalid full regexp.

See also errorString().
*/
impl /*struct*/ QRegExp {
  pub fn isValid_0<RetType, T: QRegExp_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRegExp_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRegExp) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QString pattern() const

/*
Returns the pattern string of the regular expression. The pattern has either regular expression syntax or wildcard syntax, depending on patternSyntax().

See also setPattern(), patternSyntax(), and caseSensitivity().
*/
impl /*struct*/ QRegExp {
  pub fn pattern_0<RetType, T: QRegExp_pattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pattern_0(self);
    // return 1;
  }
}
pub trait QRegExp_pattern_0<RetType> {
  fn pattern_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_pattern_0<usize> for () {
  fn pattern_0(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp7patternEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPattern(const QString &)

/*
Sets the pattern string to pattern. The case sensitivity, wildcard, and minimal matching options are not changed.

See also pattern(), setPatternSyntax(), and setCaseSensitivity().
*/
impl /*struct*/ QRegExp {
  pub fn setPattern_0<RetType, T: QRegExp_setPattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPattern_0(self);
    // return 1;
  }
}
pub trait QRegExp_setPattern_0<RetType> {
  fn setPattern_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_setPattern_0<(/*void*/)> for (usize) {
  fn setPattern_0(self , rsthis: & QRegExp) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegExp10setPatternERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:88
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity caseSensitivity() const

/*
Returns Qt::CaseSensitive if the regexp is matched case sensitively; otherwise returns Qt::CaseInsensitive.

See also setCaseSensitivity(), patternSyntax(), pattern(), and isMinimal().
*/
impl /*struct*/ QRegExp {
  pub fn caseSensitivity_0<RetType, T: QRegExp_caseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.caseSensitivity_0(self);
    // return 1;
  }
}
pub trait QRegExp_caseSensitivity_0<RetType> {
  fn caseSensitivity_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_caseSensitivity_0<i32> for () {
  fn caseSensitivity_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp15caseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCaseSensitivity(Qt::CaseSensitivity)

/*
Sets case sensitive matching to cs.

If cs is Qt::CaseSensitive, \.txt$ matches readme.txt but not README.TXT.

See also caseSensitivity(), setPatternSyntax(), setPattern(), and setMinimal().
*/
impl /*struct*/ QRegExp {
  pub fn setCaseSensitivity_0<RetType, T: QRegExp_setCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QRegExp_setCaseSensitivity_0<RetType> {
  fn setCaseSensitivity_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_setCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setCaseSensitivity_0(self , rsthis: & QRegExp) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegExp18setCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] QRegExp::PatternSyntax patternSyntax() const

/*
Returns the syntax used by the regular expression. The default is QRegExp::RegExp.

See also setPatternSyntax(), pattern(), and caseSensitivity().
*/
impl /*struct*/ QRegExp {
  pub fn patternSyntax_0<RetType, T: QRegExp_patternSyntax_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.patternSyntax_0(self);
    // return 1;
  }
}
pub trait QRegExp_patternSyntax_0<RetType> {
  fn patternSyntax_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_patternSyntax_0<i32> for () {
  fn patternSyntax_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp13patternSyntaxEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPatternSyntax(QRegExp::PatternSyntax)

/*
Sets the syntax mode for the regular expression. The default is QRegExp::RegExp.

Setting syntax to QRegExp::Wildcard enables simple shell-like QRegExp wildcard matching. For example, r*.txt matches the string readme.txt in wildcard mode, but does not match readme.

Setting syntax to QRegExp::FixedString means that the pattern is interpreted as a plain string. Special characters (e.g., backslash) don't need to be escaped then.

See also patternSyntax(), setPattern(), setCaseSensitivity(), and escape().
*/
impl /*struct*/ QRegExp {
  pub fn setPatternSyntax_0<RetType, T: QRegExp_setPatternSyntax_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPatternSyntax_0(self);
    // return 1;
  }
}
pub trait QRegExp_setPatternSyntax_0<RetType> {
  fn setPatternSyntax_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_setPatternSyntax_0<(/*void*/)> for (i32) {
  fn setPatternSyntax_0(self , rsthis: & QRegExp) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegExp16setPatternSyntaxENS_13PatternSyntaxE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:93
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isMinimal() const

/*
Returns true if minimal (non-greedy) matching is enabled; otherwise returns false.

See also caseSensitivity() and setMinimal().
*/
impl /*struct*/ QRegExp {
  pub fn isMinimal_0<RetType, T: QRegExp_isMinimal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isMinimal_0(self);
    // return 1;
  }
}
pub trait QRegExp_isMinimal_0<RetType> {
  fn isMinimal_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_isMinimal_0<bool> for () {
  fn isMinimal_0(self , rsthis: & QRegExp) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp9isMinimalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMinimal(bool)

/*
Enables or disables minimal matching. If minimal is false, matching is greedy (maximal) which is the default.

For example, suppose we have the input string "We must be <b>bold</b>, very <b>bold</b>!" and the pattern <b>.*</b>. With the default greedy (maximal) matching, the match is "We must be <b>bold</b>, very <b>bold</b>!". But with minimal (non-greedy) matching, the first match is: "We must be <b>bold</b>, very <b>bold</b>!" and the second match is "We must be <b>bold</b>, very <b>bold</b>!". In practice we might use the pattern <b>[^<]*</b> instead, although this will still fail for nested tags.

See also isMinimal() and setCaseSensitivity().
*/
impl /*struct*/ QRegExp {
  pub fn setMinimal_0<RetType, T: QRegExp_setMinimal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMinimal_0(self);
    // return 1;
  }
}
pub trait QRegExp_setMinimal_0<RetType> {
  fn setMinimal_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_setMinimal_0<(/*void*/)> for (bool) {
  fn setMinimal_0(self , rsthis: & QRegExp) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QRegExp10setMinimalEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregexp.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exactMatch(const QString &) const

/*
Returns true if str is matched exactly by this regular expression; otherwise returns false. You can determine how much of the string was matched by calling matchedLength().

For a given regexp string R, exactMatch("R") is the equivalent of indexIn("^R$") since exactMatch() effectively encloses the regexp in the start of string and end of string anchors, except that it sets matchedLength() differently.

For example, if the regular expression is blue, then exactMatch() returns true only for input blue. For inputs bluebell, blutak and lightblue, exactMatch() returns false and matchedLength() will return 4, 3 and 0 respectively.

Although const, this function sets matchedLength(), capturedTexts(), and pos().

See also indexIn() and lastIndexIn().
*/
impl /*struct*/ QRegExp {
  pub fn exactMatch_0<RetType, T: QRegExp_exactMatch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exactMatch_0(self);
    // return 1;
  }
}
pub trait QRegExp_exactMatch_0<RetType> {
  fn exactMatch_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_exactMatch_0<bool> for (usize) {
  fn exactMatch_0(self , rsthis: & QRegExp) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp10exactMatchERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:98
// index:0
// Public Visibility=Default Availability=Available
// [4] int indexIn(const QString &, int, QRegExp::CaretMode) const

/*
Attempts to find a match in str from position offset (0 by default). If offset is -1, the search starts at the last character; if -2, at the next to last character; etc.

Returns the position of the first match, or -1 if there was no match.

The caretMode parameter can be used to instruct whether ^ should match at index 0 or at offset.

You might prefer to use QString::indexOf(), QString::contains(), or even QStringList::filter(). To replace matches use QString::replace().

Example:


  QString str = "offsets: 1.23 .50 71.00 6.00";
  QRegExp rx("\\d*\\.\\d+");    // primitive floating point matching
  int count = 0;
  int pos = 0;
  while ((pos = rx.indexIn(str, pos)) != -1) {
      ++count;
      pos += rx.matchedLength();
  }
  // pos will be 9, 14, 18 and finally 24; count will end up as 4



Although const, this function sets matchedLength(), capturedTexts() and pos().

If the QRegExp is a wildcard expression (see setPatternSyntax()) and want to test a string against the whole wildcard expression, use exactMatch() instead of this function.

See also lastIndexIn() and exactMatch().
*/
impl /*struct*/ QRegExp {
  pub fn indexIn_0<RetType, T: QRegExp_indexIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.indexIn_0(self);
    // return 1;
  }
}
pub trait QRegExp_indexIn_0<RetType> {
  fn indexIn_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_indexIn_0<i32> for (usize,i32,i32) {
  fn indexIn_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp7indexInERK7QStringiNS_9CaretModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int lastIndexIn(const QString &, int, QRegExp::CaretMode) const

/*
Attempts to find a match backwards in str from position offset. If offset is -1 (the default), the search starts at the last character; if -2, at the next to last character; etc.

Returns the position of the first match, or -1 if there was no match.

The caretMode parameter can be used to instruct whether ^ should match at index 0 or at offset.

Although const, this function sets matchedLength(), capturedTexts() and pos().

Warning: Searching backwards is much slower than searching forwards.

See also indexIn() and exactMatch().
*/
impl /*struct*/ QRegExp {
  pub fn lastIndexIn_0<RetType, T: QRegExp_lastIndexIn_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastIndexIn_0(self);
    // return 1;
  }
}
pub trait QRegExp_lastIndexIn_0<RetType> {
  fn lastIndexIn_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_lastIndexIn_0<i32> for (usize,i32,i32) {
  fn lastIndexIn_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp11lastIndexInERK7QStringiNS_9CaretModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int matchedLength() const

/*
Returns the length of the last matched string, or -1 if there was no match.

See also exactMatch(), indexIn(), and lastIndexIn().
*/
impl /*struct*/ QRegExp {
  pub fn matchedLength_0<RetType, T: QRegExp_matchedLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matchedLength_0(self);
    // return 1;
  }
}
pub trait QRegExp_matchedLength_0<RetType> {
  fn matchedLength_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_matchedLength_0<i32> for () {
  fn matchedLength_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp13matchedLengthEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:102
// index:0
// Public Visibility=Default Availability=Available
// [4] int captureCount() const

/*
Returns the number of captures contained in the regular expression.

This function was introduced in  Qt 4.6.
*/
impl /*struct*/ QRegExp {
  pub fn captureCount_0<RetType, T: QRegExp_captureCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.captureCount_0(self);
    // return 1;
  }
}
pub trait QRegExp_captureCount_0<RetType> {
  fn captureCount_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_captureCount_0<i32> for () {
  fn captureCount_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp12captureCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList capturedTexts() const

/*
Returns a list of the captured text strings.

The first string in the list is the entire matched string. Each subsequent list element contains a string that matched a (capturing) subexpression of the regexp.

For example:


  QRegExp rx("(\\d+)(\\s*)(cm|inch(es)?)");
  int pos = rx.indexIn("Length: 36 inches");
  QStringList list = rx.capturedTexts();
  // list is now ("36 inches", "36", " ", "inches", "es")



The above example also captures elements that may be present but which we have no interest in. This problem can be solved by using non-capturing parentheses:


  QRegExp rx("(\\d+)(?:\\s*)(cm|inch(?:es)?)");
  int pos = rx.indexIn("Length: 36 inches");
  QStringList list = rx.capturedTexts();
  // list is now ("36 inches", "36", "inches")



Note that if you want to iterate over the list, you should iterate over a copy, e.g.


  QStringList list = rx.capturedTexts();
  QStringList::iterator it = list.begin();
  while (it != list.end()) {
      myProcessing(*it);
      ++it;
  }



Some regexps can match an indeterminate number of times. For example if the input string is "Offsets: 12 14 99 231 7" and the regexp, rx, is (\d+)+, we would hope to get a list of all the numbers matched. However, after calling rx.indexIn(str), capturedTexts() will return the list ("12", "12"), i.e. the entire match was "12" and the first subexpression matched was "12". The correct approach is to use cap() in a loop.

The order of elements in the string list is as follows. The first element is the entire matching string. Each subsequent element corresponds to the next capturing open left parentheses. Thus capturedTexts()[1] is the text of the first capturing parentheses, capturedTexts()[2] is the text of the second and so on (corresponding to $1, $2, etc., in some other regexp languages).

See also cap() and pos().
*/
impl /*struct*/ QRegExp {
  pub fn capturedTexts_0<RetType, T: QRegExp_capturedTexts_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedTexts_0(self);
    // return 1;
  }
}
pub trait QRegExp_capturedTexts_0<RetType> {
  fn capturedTexts_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_capturedTexts_0<usize> for () {
  fn capturedTexts_0(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp13capturedTextsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:104
// index:1
// Public Visibility=Default Availability=Available
// [8] QStringList capturedTexts()

/*
Returns a list of the captured text strings.

The first string in the list is the entire matched string. Each subsequent list element contains a string that matched a (capturing) subexpression of the regexp.

For example:


  QRegExp rx("(\\d+)(\\s*)(cm|inch(es)?)");
  int pos = rx.indexIn("Length: 36 inches");
  QStringList list = rx.capturedTexts();
  // list is now ("36 inches", "36", " ", "inches", "es")



The above example also captures elements that may be present but which we have no interest in. This problem can be solved by using non-capturing parentheses:


  QRegExp rx("(\\d+)(?:\\s*)(cm|inch(?:es)?)");
  int pos = rx.indexIn("Length: 36 inches");
  QStringList list = rx.capturedTexts();
  // list is now ("36 inches", "36", "inches")



Note that if you want to iterate over the list, you should iterate over a copy, e.g.


  QStringList list = rx.capturedTexts();
  QStringList::iterator it = list.begin();
  while (it != list.end()) {
      myProcessing(*it);
      ++it;
  }



Some regexps can match an indeterminate number of times. For example if the input string is "Offsets: 12 14 99 231 7" and the regexp, rx, is (\d+)+, we would hope to get a list of all the numbers matched. However, after calling rx.indexIn(str), capturedTexts() will return the list ("12", "12"), i.e. the entire match was "12" and the first subexpression matched was "12". The correct approach is to use cap() in a loop.

The order of elements in the string list is as follows. The first element is the entire matching string. Each subsequent element corresponds to the next capturing open left parentheses. Thus capturedTexts()[1] is the text of the first capturing parentheses, capturedTexts()[2] is the text of the second and so on (corresponding to $1, $2, etc., in some other regexp languages).

See also cap() and pos().
*/
impl /*struct*/ QRegExp {
  pub fn capturedTexts_1<RetType, T: QRegExp_capturedTexts_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedTexts_1(self);
    // return 1;
  }
}
pub trait QRegExp_capturedTexts_1<RetType> {
  fn capturedTexts_1(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_capturedTexts_1<usize> for () {
  fn capturedTexts_1(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExp13capturedTextsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QString cap(int) const

/*
Returns the text captured by the nth subexpression. The entire match has index 0 and the parenthesized subexpressions have indexes starting from 1 (excluding non-capturing parentheses).


  QRegExp rxlen("(\\d+)(?:\\s*)(cm|inch)");
  int pos = rxlen.indexIn("Length: 189cm");
  if (pos > -1) {
      QString value = rxlen.cap(1); // "189"
      QString unit = rxlen.cap(2);  // "cm"
      // ...
  }



The order of elements matched by cap() is as follows. The first element, cap(0), is the entire matching string. Each subsequent element corresponds to the next capturing open left parentheses. Thus cap(1) is the text of the first capturing parentheses, cap(2) is the text of the second, and so on.

See also capturedTexts() and pos().
*/
impl /*struct*/ QRegExp {
  pub fn cap_0<RetType, T: QRegExp_cap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cap_0(self);
    // return 1;
  }
}
pub trait QRegExp_cap_0<RetType> {
  fn cap_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_cap_0<usize> for (i32) {
  fn cap_0(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp3capEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:106
// index:1
// Public Visibility=Default Availability=Available
// [8] QString cap(int)

/*
Returns the text captured by the nth subexpression. The entire match has index 0 and the parenthesized subexpressions have indexes starting from 1 (excluding non-capturing parentheses).


  QRegExp rxlen("(\\d+)(?:\\s*)(cm|inch)");
  int pos = rxlen.indexIn("Length: 189cm");
  if (pos > -1) {
      QString value = rxlen.cap(1); // "189"
      QString unit = rxlen.cap(2);  // "cm"
      // ...
  }



The order of elements matched by cap() is as follows. The first element, cap(0), is the entire matching string. Each subsequent element corresponds to the next capturing open left parentheses. Thus cap(1) is the text of the first capturing parentheses, cap(2) is the text of the second, and so on.

See also capturedTexts() and pos().
*/
impl /*struct*/ QRegExp {
  pub fn cap_1<RetType, T: QRegExp_cap_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cap_1(self);
    // return 1;
  }
}
pub trait QRegExp_cap_1<RetType> {
  fn cap_1(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_cap_1<usize> for (i32) {
  fn cap_1(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExp3capEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] int pos(int) const

/*
Returns the position of the nth captured text in the searched string. If nth is 0 (the default), pos() returns the position of the whole match.

Example:


  QRegExp rx("/([a-z]+)/([a-z]+)");
  rx.indexIn("Output /dev/null");   // returns 7 (position of /dev/null)
  rx.pos(0);                        // returns 7 (position of /dev/null)
  rx.pos(1);                        // returns 8 (position of dev)
  rx.pos(2);                        // returns 12 (position of null)



For zero-length matches, pos() always returns -1. (For example, if cap(4) would return an empty string, pos(4) returns -1.) This is a feature of the implementation.

See also cap() and capturedTexts().
*/
impl /*struct*/ QRegExp {
  pub fn pos_0<RetType, T: QRegExp_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QRegExp_pos_0<RetType> {
  fn pos_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_pos_0<i32> for (i32) {
  fn pos_0(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp3posEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:108
// index:1
// Public Visibility=Default Availability=Available
// [4] int pos(int)

/*
Returns the position of the nth captured text in the searched string. If nth is 0 (the default), pos() returns the position of the whole match.

Example:


  QRegExp rx("/([a-z]+)/([a-z]+)");
  rx.indexIn("Output /dev/null");   // returns 7 (position of /dev/null)
  rx.pos(0);                        // returns 7 (position of /dev/null)
  rx.pos(1);                        // returns 8 (position of dev)
  rx.pos(2);                        // returns 12 (position of null)



For zero-length matches, pos() always returns -1. (For example, if cap(4) would return an empty string, pos(4) returns -1.) This is a feature of the implementation.

See also cap() and capturedTexts().
*/
impl /*struct*/ QRegExp {
  pub fn pos_1<RetType, T: QRegExp_pos_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_1(self);
    // return 1;
  }
}
pub trait QRegExp_pos_1<RetType> {
  fn pos_1(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_pos_1<i32> for (i32) {
  fn pos_1(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExp3posEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:109
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a text string that explains why a regexp pattern is invalid the case being; otherwise returns "no error occurred".

See also isValid().
*/
impl /*struct*/ QRegExp {
  pub fn errorString_0<RetType, T: QRegExp_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QRegExp_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QRegExp11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:110
// index:1
// Public Visibility=Default Availability=Available
// [8] QString errorString()

/*
Returns a text string that explains why a regexp pattern is invalid the case being; otherwise returns "no error occurred".

See also isValid().
*/
impl /*struct*/ QRegExp {
  pub fn errorString_1<RetType, T: QRegExp_errorString_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_1(self);
    // return 1;
  }
}
pub trait QRegExp_errorString_1<RetType> {
  fn errorString_1(self , rsthis: & QRegExp) -> RetType;
}
impl<'a> /*trait*/ QRegExp_errorString_1<usize> for () {
  fn errorString_1(self , rsthis: & QRegExp) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExp11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregexp.h:113
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString escape(const QString &)

/*
Returns the string str with every regexp special character escaped with a backslash. The special characters are $, (,), *, +, ., ?, [, ,], ^, {, | and }.

Example:


  s1 = QRegExp::escape("bingo");   // s1 == "bingo"
  s2 = QRegExp::escape("f(x)");    // s2 == "f\\(x\\)"



This function is useful to construct regexp patterns dynamically:


  QRegExp rx("(" + QRegExp::escape(name) +
             "|" + QRegExp::escape(alias) + ")");



See also setPatternSyntax().
*/
impl /*struct*/ QRegExp {
  pub fn escape_0<RetType, T: QRegExp_escape_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.escape_0();
    // return 1;
  }
}
pub trait QRegExp_escape_0<RetType> {
  fn escape_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRegExp_escape_0<usize> for (usize) {
  fn escape_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QRegExp6escapeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
The syntax used to interpret the meaning of the pattern.



See also setPatternSyntax().

*/
pub type QRegExp__PatternSyntax = i32;
// A rich Perl-like pattern matching syntax. This is the default.
pub const QRegExp__RegExp :QRegExp__PatternSyntax = 0;
// This provides a simple pattern matching syntax similar to that used by shells (command interpreters) for "file globbing". See QRegExp wildcard matching.
pub const QRegExp__Wildcard :QRegExp__PatternSyntax = 1;
// The pattern is a fixed string. This is equivalent to using the RegExp pattern on a string in which all metacharacters are escaped using escape().
pub const QRegExp__FixedString :QRegExp__PatternSyntax = 2;
// 
pub const QRegExp__RegExp2 :QRegExp__PatternSyntax = 3;
// This is similar to Wildcard but with the behavior of a Unix shell. The wildcard characters can be escaped with the character "\".
pub const QRegExp__WildcardUnix :QRegExp__PatternSyntax = 4;
// 
pub const QRegExp__W3CXmlSchema11 :QRegExp__PatternSyntax = 5;
pub fn QRegExp_PatternSyntaxItemName(val: i32) ->String {
  match val {
     QRegExp__RegExp => // 0
     {return String::from("RegExp");}
     QRegExp__Wildcard => // 1
     {return String::from("Wildcard");}
     QRegExp__FixedString => // 2
     {return String::from("FixedString");}
     QRegExp__RegExp2 => // 3
     {return String::from("RegExp2");}
     QRegExp__WildcardUnix => // 4
     {return String::from("WildcardUnix");}
     QRegExp__W3CXmlSchema11 => // 5
     {return String::from("W3CXmlSchema11");}
  _ => {return format!("{}", val);}
}
}
pub fn QRegExp_PatternSyntaxItemName_s(val: i32) ->String {
  //var nilthis *QRegExp
  //return nilthis.PatternSyntaxItemName(val);
  return QRegExp_PatternSyntaxItemName(val);
}


/*
The CaretMode enum defines the different meanings of the caret (^) in a regular expression. The possible values are:


*/
pub type QRegExp__CaretMode = i32;
// 
pub const QRegExp__CaretAtZero :QRegExp__CaretMode = 0;
// The caret corresponds to the start offset of the search.
pub const QRegExp__CaretAtOffset :QRegExp__CaretMode = 1;
// The caret never matches.
pub const QRegExp__CaretWontMatch :QRegExp__CaretMode = 2;
pub fn QRegExp_CaretModeItemName(val: i32) ->String {
  match val {
     QRegExp__CaretAtZero => // 0
     {return String::from("CaretAtZero");}
     QRegExp__CaretAtOffset => // 1
     {return String::from("CaretAtOffset");}
     QRegExp__CaretWontMatch => // 2
     {return String::from("CaretWontMatch");}
  _ => {return format!("{}", val);}
}
}
pub fn QRegExp_CaretModeItemName_s(val: i32) ->String {
  //var nilthis *QRegExp
  //return nilthis.CaretModeItemName(val);
  return QRegExp_CaretModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
