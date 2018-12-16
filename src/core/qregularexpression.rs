

// mod ::core::QRegularExpression
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
// extern C begin: 74
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
#[derive(Default)] // class sizeof(QRegularExpression)=8
pub struct QRegularExpression {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegularExpression_ITF interface {
//    QRegularExpression_PTR() *QRegularExpression
//}
//func (ptr *QRegularExpression) QRegularExpression_PTR() *QRegularExpression { return ptr }

impl /*struct*/ QRegularExpression {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegularExpression {
    return QRegularExpression{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegularExpression {
//  type Target = QRegularExpressionBASE;
//
//  fn deref(&self) -> &QRegularExpressionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegularExpressionBASE> for QRegularExpression {
//  fn as_ref(& self) -> & QRegularExpressionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qregularexpression.h:81
// index:0
// Public Visibility=Default Availability=Available
// [4] QRegularExpression::PatternOptions patternOptions() const

/*
Returns the pattern options for the regular expression.

See also setPatternOptions() and pattern().
*/
impl /*struct*/ QRegularExpression {
  pub fn patternOptions_0<RetType, T: QRegularExpression_patternOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.patternOptions_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_patternOptions_0<RetType> {
  fn patternOptions_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_patternOptions_0<i32> for () {
  fn patternOptions_0(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression14patternOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPatternOptions(QRegularExpression::PatternOptions)

/*
Sets the given options as the pattern options of the regular expression. The pattern string is left unchanged.

See also patternOptions() and setPattern().
*/
impl /*struct*/ QRegularExpression {
  pub fn setPatternOptions_0<RetType, T: QRegularExpression_setPatternOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPatternOptions_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_setPatternOptions_0<RetType> {
  fn setPatternOptions_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_setPatternOptions_0<(/*void*/)> for (i32) {
  fn setPatternOptions_0(self , rsthis: & QRegularExpression) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QRegularExpression17setPatternOptionsE6QFlagsINS_13PatternOptionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegularExpression()

/*
Constructs a QRegularExpression object with an empty pattern and no pattern options.

See also setPattern() and setPatternOptions().
*/
// QRegularExpression() ctx.fn_proto_cpp
impl /*struct*/ QRegularExpression {
  pub fn QRegularExpression_0<T: QRegularExpression_QRegularExpression_0>(value: T) -> QRegularExpression {
    let rsthis = value.QRegularExpression_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpression_QRegularExpression_0 {
  fn QRegularExpression_0(self) -> QRegularExpression;
}
// QRegularExpression() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegularExpression_QRegularExpression_0 for () {
  fn QRegularExpression_0(self) -> QRegularExpression {
    // unsafe{_ZN18QRegularExpressionC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QRegularExpressionC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegularExpression{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:85
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRegularExpression(const QString &, QRegularExpression::PatternOptions)

/*
Constructs a QRegularExpression object with an empty pattern and no pattern options.

See also setPattern() and setPatternOptions().
*/
// QRegularExpression(const QString &, QRegularExpression::PatternOptions) ctx.fn_proto_cpp
impl /*struct*/ QRegularExpression {
  pub fn QRegularExpression_1<T: QRegularExpression_QRegularExpression_1>(value: T) -> QRegularExpression {
    let rsthis = value.QRegularExpression_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpression_QRegularExpression_1 {
  fn QRegularExpression_1(self) -> QRegularExpression;
}
// QRegularExpression(const QString &, QRegularExpression::PatternOptions) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegularExpression_QRegularExpression_1 for (usize,i32) {
  fn QRegularExpression_1(self) -> QRegularExpression {
    // unsafe{_ZN18QRegularExpressionC2ERK7QString6QFlagsINS_13PatternOptionEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QRegularExpressionC2ERK7QString6QFlagsINS_13PatternOptionEE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegularExpression{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QRegularExpression()

/*

*/
pub fn DeleteQRegularExpression(this :*mut QRegularExpression) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QRegularExpressionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qregularexpression.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpression & operator=(const QRegularExpression &)

/*

*/
impl /*struct*/ QRegularExpression {
  pub fn operator_equal_0<RetType, T: QRegularExpression_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRegularExpressionaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:91
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QRegularExpression & operator=(QRegularExpression &&)

/*

*/
impl /*struct*/ QRegularExpression {
  pub fn operator_equal_1<RetType, T: QRegularExpression_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QRegularExpression_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRegularExpressionaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:95
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QRegularExpression &)

/*
Swaps the regular expression other with this regular expression. This operation is very fast and never fails.
*/
impl /*struct*/ QRegularExpression {
  pub fn swap_0<RetType, T: QRegularExpression_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_swap_0<RetType> {
  fn swap_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QRegularExpression) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QRegularExpression4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QString pattern() const

/*
Returns the pattern string of the regular expression.

See also setPattern() and patternOptions().
*/
impl /*struct*/ QRegularExpression {
  pub fn pattern_0<RetType, T: QRegularExpression_pattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pattern_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_pattern_0<RetType> {
  fn pattern_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_pattern_0<usize> for () {
  fn pattern_0(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression7patternEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPattern(const QString &)

/*
Sets the pattern string of the regular expression to pattern. The pattern options are left unchanged.

See also pattern() and setPatternOptions().
*/
impl /*struct*/ QRegularExpression {
  pub fn setPattern_0<RetType, T: QRegularExpression_setPattern_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPattern_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_setPattern_0<RetType> {
  fn setPattern_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_setPattern_0<(/*void*/)> for (usize) {
  fn setPattern_0(self , rsthis: & QRegularExpression) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QRegularExpression10setPatternERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the regular expression is a valid regular expression (that is, it contains no syntax errors, etc.), or false otherwise. Use errorString() to obtain a textual description of the error.

See also errorString() and patternErrorOffset().
*/
impl /*struct*/ QRegularExpression {
  pub fn isValid_0<RetType, T: QRegularExpression_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRegularExpression) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:101
// index:0
// Public Visibility=Default Availability=Available
// [4] int patternErrorOffset() const

/*
Returns the offset, inside the pattern string, at which an error was found when checking the validity of the regular expression. If no error was found, then -1 is returned.

See also pattern(), isValid(), and errorString().
*/
impl /*struct*/ QRegularExpression {
  pub fn patternErrorOffset_0<RetType, T: QRegularExpression_patternErrorOffset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.patternErrorOffset_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_patternErrorOffset_0<RetType> {
  fn patternErrorOffset_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_patternErrorOffset_0<i32> for () {
  fn patternErrorOffset_0(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression18patternErrorOffsetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:102
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a textual description of the error found when checking the validity of the regular expression, or "no error" if no error was found.

See also isValid() and patternErrorOffset().
*/
impl /*struct*/ QRegularExpression {
  pub fn errorString_0<RetType, T: QRegularExpression_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:104
// index:0
// Public Visibility=Default Availability=Available
// [4] int captureCount() const

/*
Returns the number of capturing groups inside the pattern string, or -1 if the regular expression is not valid.

Note: The implicit capturing group 0 is not included in the returned number.

See also isValid().
*/
impl /*struct*/ QRegularExpression {
  pub fn captureCount_0<RetType, T: QRegularExpression_captureCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.captureCount_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_captureCount_0<RetType> {
  fn captureCount_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_captureCount_0<i32> for () {
  fn captureCount_0(self , rsthis: & QRegularExpression) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression12captureCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:105
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList namedCaptureGroups() const

/*
Returns a list of captureCount() + 1 elements, containing the names of the named capturing groups in the pattern string. The list is sorted such that the element of the list at position i is the name of the i-th capturing group, if it has a name, or an empty string if that capturing group is unnamed.

For instance, given the regular expression


  (?<day>\d\d)-(?<month>\d\d)-(?<year>\d\d\d\d) (\w+) (?<name>\w+)



namedCaptureGroups() will return the following list:


  ("", "day", "month", "year", "", "name")



which corresponds to the fact that the capturing group #0 (corresponding to the whole match) has no name, the capturing group #1 has name "day", the capturing group #2 has name "month", etc.

If the regular expression is not valid, returns an empty list.

This function was introduced in  Qt 5.1.

See also isValid(), QRegularExpressionMatch::captured(), and QString::isEmpty().
*/
impl /*struct*/ QRegularExpression {
  pub fn namedCaptureGroups_0<RetType, T: QRegularExpression_namedCaptureGroups_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.namedCaptureGroups_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_namedCaptureGroups_0<RetType> {
  fn namedCaptureGroups_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_namedCaptureGroups_0<usize> for () {
  fn namedCaptureGroups_0(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression18namedCaptureGroupsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:121
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatch match(const QString &, int, QRegularExpression::MatchType, QRegularExpression::MatchOptions) const

/*
Attempts to match the regular expression against the given subject string, starting at the position offset inside the subject, using a match of type matchType and honoring the given matchOptions.

The returned QRegularExpressionMatch object contains the results of the match.

See also QRegularExpressionMatch and normal matching.
*/
impl /*struct*/ QRegularExpression {
  pub fn match__0<RetType, T: QRegularExpression_match__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.match__0(self);
    // return 1;
  }
}
pub trait QRegularExpression_match__0<RetType> {
  fn match__0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_match__0<usize> for (usize,i32,i32,i32) {
  fn match__0(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression5matchERK7QStringiNS_9MatchTypeE6QFlagsINS_11MatchOptionEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:126
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatch match(const QStringRef &, int, QRegularExpression::MatchType, QRegularExpression::MatchOptions) const

/*
Attempts to match the regular expression against the given subject string, starting at the position offset inside the subject, using a match of type matchType and honoring the given matchOptions.

The returned QRegularExpressionMatch object contains the results of the match.

See also QRegularExpressionMatch and normal matching.
*/
impl /*struct*/ QRegularExpression {
  pub fn match__1<RetType, T: QRegularExpression_match__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.match__1(self);
    // return 1;
  }
}
pub trait QRegularExpression_match__1<RetType> {
  fn match__1(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_match__1<usize> for (usize,i32,i32,i32) {
  fn match__1(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression5matchERK10QStringRefiNS_9MatchTypeE6QFlagsINS_11MatchOptionEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:131
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatchIterator globalMatch(const QString &, int, QRegularExpression::MatchType, QRegularExpression::MatchOptions) const

/*
Attempts to perform a global match of the regular expression against the given subject string, starting at the position offset inside the subject, using a match of type matchType and honoring the given matchOptions.

The returned QRegularExpressionMatchIterator is positioned before the first match result (if any).

See also QRegularExpressionMatchIterator and global matching.
*/
impl /*struct*/ QRegularExpression {
  pub fn globalMatch_0<RetType, T: QRegularExpression_globalMatch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalMatch_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_globalMatch_0<RetType> {
  fn globalMatch_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_globalMatch_0<usize> for (usize,i32,i32,i32) {
  fn globalMatch_0(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression11globalMatchERK7QStringiNS_9MatchTypeE6QFlagsINS_11MatchOptionEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:136
// index:1
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatchIterator globalMatch(const QStringRef &, int, QRegularExpression::MatchType, QRegularExpression::MatchOptions) const

/*
Attempts to perform a global match of the regular expression against the given subject string, starting at the position offset inside the subject, using a match of type matchType and honoring the given matchOptions.

The returned QRegularExpressionMatchIterator is positioned before the first match result (if any).

See also QRegularExpressionMatchIterator and global matching.
*/
impl /*struct*/ QRegularExpression {
  pub fn globalMatch_1<RetType, T: QRegularExpression_globalMatch_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalMatch_1(self);
    // return 1;
  }
}
pub trait QRegularExpression_globalMatch_1<RetType> {
  fn globalMatch_1(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_globalMatch_1<usize> for (usize,i32,i32,i32) {
  fn globalMatch_1(self , rsthis: & QRegularExpression) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpression11globalMatchERK10QStringRefiNS_9MatchTypeE6QFlagsINS_11MatchOptionEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:141
// index:0
// Public Visibility=Default Availability=Available
// [-2] void optimize() const

/*
Forces an immediate optimization of the pattern, including JIT-compiling it (if the JIT compiler is enabled).

Patterns are normally optimized only after a certain number of usages. If you can predict that this QRegularExpression object is going to be used for several matches, it may be convenient to optimize it in advance by calling this function.

This function was introduced in  Qt 5.4.

See also QRegularExpression::OptimizeOnFirstUsageOption.
*/
impl /*struct*/ QRegularExpression {
  pub fn optimize_0<RetType, T: QRegularExpression_optimize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.optimize_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_optimize_0<RetType> {
  fn optimize_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_optimize_0<(/*void*/)> for () {
  fn optimize_0(self , rsthis: & QRegularExpression) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZNK18QRegularExpression8optimizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:143
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString escape(const QString &)

/*
Escapes all characters of str so that they no longer have any special meaning when used as a regular expression pattern string, and returns the escaped string. For instance:


  QString escaped = QRegularExpression::escape("a(x) = f(x) + g(x)");
  // escaped == "a\\(x\\)\\ \\=\\ f\\(x\\)\\ \\+\\ g\\(x\\)"



This is very convenient in order to build patterns from arbitrary strings:


  QString pattern = "(" + QRegularExpression::escape(name) +
                    "|" + QRegularExpression::escape(nickname) + ")";
  QRegularExpression re(pattern);



Note: This function implements Perl's quotemeta algorithm and escapes with a backslash all characters in str, except for the characters in the [A-Z], [a-z] and [0-9] ranges, as well as the underscore (_) character. The only difference with Perl is that a literal NUL inside str is escaped with the sequence "\\0" (backslash + '0'), instead of "\\\0" (backslash + NUL).
*/
impl /*struct*/ QRegularExpression {
  pub fn escape_0<RetType, T: QRegularExpression_escape_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.escape_0();
    // return 1;
  }
}
pub trait QRegularExpression_escape_0<RetType> {
  fn escape_0(self ) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_escape_0<usize> for (usize) {
  fn escape_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QRegularExpression6escapeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:145
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QRegularExpression &) const

/*

*/
impl /*struct*/ QRegularExpression {
  pub fn operator_equal_equal_0<RetType, T: QRegularExpression_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QRegularExpression) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpressioneqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:146
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QRegularExpression &) const

/*

*/
impl /*struct*/ QRegularExpression {
  pub fn operator_not_equal_0<RetType, T: QRegularExpression_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QRegularExpression_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QRegularExpression) -> RetType;
}
impl<'a> /*trait*/ QRegularExpression_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QRegularExpression) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QRegularExpressionneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QRegularExpression__PatternOption = i32;
// 
pub const QRegularExpression__NoPatternOption :QRegularExpression__PatternOption = 0;
// 
pub const QRegularExpression__CaseInsensitiveOption :QRegularExpression__PatternOption = 1;
// 
pub const QRegularExpression__DotMatchesEverythingOption :QRegularExpression__PatternOption = 2;
// 
pub const QRegularExpression__MultilineOption :QRegularExpression__PatternOption = 4;
// 
pub const QRegularExpression__ExtendedPatternSyntaxOption :QRegularExpression__PatternOption = 8;
// 
pub const QRegularExpression__InvertedGreedinessOption :QRegularExpression__PatternOption = 16;
// 
pub const QRegularExpression__DontCaptureOption :QRegularExpression__PatternOption = 32;
// 
pub const QRegularExpression__UseUnicodePropertiesOption :QRegularExpression__PatternOption = 64;
// 
pub const QRegularExpression__OptimizeOnFirstUsageOption :QRegularExpression__PatternOption = 128;
// 
pub const QRegularExpression__DontAutomaticallyOptimizeOption :QRegularExpression__PatternOption = 256;
pub fn QRegularExpression_PatternOptionItemName(val: i32) ->String {
  match val {
     QRegularExpression__NoPatternOption => // 0
     {return String::from("NoPatternOption");}
     QRegularExpression__CaseInsensitiveOption => // 1
     {return String::from("CaseInsensitiveOption");}
     QRegularExpression__DotMatchesEverythingOption => // 2
     {return String::from("DotMatchesEverythingOption");}
     QRegularExpression__MultilineOption => // 4
     {return String::from("MultilineOption");}
     QRegularExpression__ExtendedPatternSyntaxOption => // 8
     {return String::from("ExtendedPatternSyntaxOption");}
     QRegularExpression__InvertedGreedinessOption => // 16
     {return String::from("InvertedGreedinessOption");}
     QRegularExpression__DontCaptureOption => // 32
     {return String::from("DontCaptureOption");}
     QRegularExpression__UseUnicodePropertiesOption => // 64
     {return String::from("UseUnicodePropertiesOption");}
     QRegularExpression__OptimizeOnFirstUsageOption => // 128
     {return String::from("OptimizeOnFirstUsageOption");}
     QRegularExpression__DontAutomaticallyOptimizeOption => // 256
     {return String::from("DontAutomaticallyOptimizeOption");}
  _ => {return format!("{}", val);}
}
}
pub fn QRegularExpression_PatternOptionItemName_s(val: i32) ->String {
  //var nilthis *QRegularExpression
  //return nilthis.PatternOptionItemName(val);
  return QRegularExpression_PatternOptionItemName(val);
}


/*
The MatchType enum defines the type of the match that should be attempted against the subject string.


*/
pub type QRegularExpression__MatchType = i32;
// A normal match is done.
pub const QRegularExpression__NormalMatch :QRegularExpression__MatchType = 0;
// The pattern string is matched partially against the subject string. If a partial match is found, then it is recorded, and other matching alternatives are tried as usual. If a complete match is then found, then it's preferred to the partial match; in this case only the complete match is reported. If instead no complete match is found (but only the partial one), then the partial one is reported.
pub const QRegularExpression__PartialPreferCompleteMatch :QRegularExpression__MatchType = 1;
// The pattern string is matched partially against the subject string. If a partial match is found, then matching stops and the partial match is reported. In this case, other matching alternatives (potentially leading to a complete match) are not tried. Moreover, this match type assumes that the subject string only a substring of a larger text, and that (in this text) there are other characters beyond the end of the subject string. This can lead to surprising results; see the discussion in the partial matching section for more details.
pub const QRegularExpression__PartialPreferFirstMatch :QRegularExpression__MatchType = 2;
// 
pub const QRegularExpression__NoMatch :QRegularExpression__MatchType = 3;
pub fn QRegularExpression_MatchTypeItemName(val: i32) ->String {
  match val {
     QRegularExpression__NormalMatch => // 0
     {return String::from("NormalMatch");}
     QRegularExpression__PartialPreferCompleteMatch => // 1
     {return String::from("PartialPreferCompleteMatch");}
     QRegularExpression__PartialPreferFirstMatch => // 2
     {return String::from("PartialPreferFirstMatch");}
     QRegularExpression__NoMatch => // 3
     {return String::from("NoMatch");}
  _ => {return format!("{}", val);}
}
}
pub fn QRegularExpression_MatchTypeItemName_s(val: i32) ->String {
  //var nilthis *QRegularExpression
  //return nilthis.MatchTypeItemName(val);
  return QRegularExpression_MatchTypeItemName(val);
}


/*


*/
pub type QRegularExpression__MatchOption = i32;
// 
pub const QRegularExpression__NoMatchOption :QRegularExpression__MatchOption = 0;
// 
pub const QRegularExpression__AnchoredMatchOption :QRegularExpression__MatchOption = 1;
// 
pub const QRegularExpression__DontCheckSubjectStringMatchOption :QRegularExpression__MatchOption = 2;
pub fn QRegularExpression_MatchOptionItemName(val: i32) ->String {
  match val {
     QRegularExpression__NoMatchOption => // 0
     {return String::from("NoMatchOption");}
     QRegularExpression__AnchoredMatchOption => // 1
     {return String::from("AnchoredMatchOption");}
     QRegularExpression__DontCheckSubjectStringMatchOption => // 2
     {return String::from("DontCheckSubjectStringMatchOption");}
  _ => {return format!("{}", val);}
}
}
pub fn QRegularExpression_MatchOptionItemName_s(val: i32) ->String {
  //var nilthis *QRegularExpression
  //return nilthis.MatchOptionItemName(val);
  return QRegularExpression_MatchOptionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
