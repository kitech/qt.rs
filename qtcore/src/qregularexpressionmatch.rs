

// mod ::core::QRegularExpressionMatch
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
// extern C begin: 23
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
#[derive(Default)] // class sizeof(QRegularExpressionMatch)=8
pub struct QRegularExpressionMatch {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegularExpressionMatch_ITF interface {
//    QRegularExpressionMatch_PTR() *QRegularExpressionMatch
//}
//func (ptr *QRegularExpressionMatch) QRegularExpressionMatch_PTR() *QRegularExpressionMatch { return ptr }

impl /*struct*/ QRegularExpressionMatch {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegularExpressionMatch {
    return QRegularExpressionMatch{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegularExpressionMatch {
//  type Target = QRegularExpressionMatchBASE;
//
//  fn deref(&self) -> &QRegularExpressionMatchBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegularExpressionMatchBASE> for QRegularExpressionMatch {
//  fn as_ref(& self) -> & QRegularExpressionMatchBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qregularexpression.h:178
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegularExpressionMatch()

/*

*/
// QRegularExpressionMatch() ctx.fn_proto_cpp
impl /*struct*/ QRegularExpressionMatch {
  pub fn QRegularExpressionMatch_0<T: QRegularExpressionMatch_QRegularExpressionMatch_0>(value: T) -> QRegularExpressionMatch {
    let rsthis = value.QRegularExpressionMatch_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatch_QRegularExpressionMatch_0 {
  fn QRegularExpressionMatch_0(self) -> QRegularExpressionMatch;
}
// QRegularExpressionMatch() ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegularExpressionMatch_QRegularExpressionMatch_0 for () {
  fn QRegularExpressionMatch_0(self) -> QRegularExpressionMatch {
    // unsafe{_ZN23QRegularExpressionMatchC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QRegularExpressionMatchC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegularExpressionMatch{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:179
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QRegularExpressionMatch()

/*

*/
pub fn DeleteQRegularExpressionMatch(this :*mut QRegularExpressionMatch) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QRegularExpressionMatchD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qregularexpression.h:181
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpressionMatch & operator=(const QRegularExpressionMatch &)

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn operator_equal_0<RetType, T: QRegularExpressionMatch_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QRegularExpressionMatchaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:184
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QRegularExpressionMatch & operator=(QRegularExpressionMatch &&)

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn operator_equal_1<RetType, T: QRegularExpressionMatch_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN23QRegularExpressionMatchaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:187
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QRegularExpressionMatch &)

/*
Swaps the regular expression other with this regular expression. This operation is very fast and never fails.
*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn swap_0<RetType, T: QRegularExpressionMatch_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_swap_0<RetType> {
  fn swap_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QRegularExpressionMatch) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN23QRegularExpressionMatch4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:189
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpression regularExpression() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn regularExpression_0<RetType, T: QRegularExpressionMatch_regularExpression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.regularExpression_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_regularExpression_0<RetType> {
  fn regularExpression_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_regularExpression_0<usize> for () {
  fn regularExpression_0(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch17regularExpressionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:190
// index:0
// Public Visibility=Default Availability=Available
// [4] QRegularExpression::MatchType matchType() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn matchType_0<RetType, T: QRegularExpressionMatch_matchType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matchType_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_matchType_0<RetType> {
  fn matchType_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_matchType_0<i32> for () {
  fn matchType_0(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch9matchTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:191
// index:0
// Public Visibility=Default Availability=Available
// [4] QRegularExpression::MatchOptions matchOptions() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn matchOptions_0<RetType, T: QRegularExpressionMatch_matchOptions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.matchOptions_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_matchOptions_0<RetType> {
  fn matchOptions_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_matchOptions_0<i32> for () {
  fn matchOptions_0(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch12matchOptionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:193
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasMatch() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasMatch_0<RetType, T: QRegularExpressionMatch_hasMatch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasMatch_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_hasMatch_0<RetType> {
  fn hasMatch_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_hasMatch_0<bool> for () {
  fn hasMatch_0(self , rsthis: & QRegularExpressionMatch) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch8hasMatchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:194
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasPartialMatch() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn hasPartialMatch_0<RetType, T: QRegularExpressionMatch_hasPartialMatch_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasPartialMatch_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_hasPartialMatch_0<RetType> {
  fn hasPartialMatch_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_hasPartialMatch_0<bool> for () {
  fn hasPartialMatch_0(self , rsthis: & QRegularExpressionMatch) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch15hasPartialMatchEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:196
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the regular expression is a valid regular expression (that is, it contains no syntax errors, etc.), or false otherwise. Use errorString() to obtain a textual description of the error.

See also errorString() and patternErrorOffset().
*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn isValid_0<RetType, T: QRegularExpressionMatch_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QRegularExpressionMatch) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:198
// index:0
// Public Visibility=Default Availability=Available
// [4] int lastCapturedIndex() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn lastCapturedIndex_0<RetType, T: QRegularExpressionMatch_lastCapturedIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastCapturedIndex_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_lastCapturedIndex_0<RetType> {
  fn lastCapturedIndex_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_lastCapturedIndex_0<i32> for () {
  fn lastCapturedIndex_0(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch17lastCapturedIndexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:200
// index:0
// Public Visibility=Default Availability=Available
// [8] QString captured(int) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn captured_0<RetType, T: QRegularExpressionMatch_captured_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.captured_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_captured_0<RetType> {
  fn captured_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_captured_0<usize> for (i32) {
  fn captured_0(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch8capturedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:205
// index:1
// Public Visibility=Default Availability=Available
// [8] QString captured(const QString &) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn captured_1<RetType, T: QRegularExpressionMatch_captured_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.captured_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_captured_1<RetType> {
  fn captured_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_captured_1<usize> for (usize) {
  fn captured_1(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch8capturedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:209
// index:2
// Public Visibility=Default Availability=Available
// [8] QString captured(QStringView) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn captured_2<RetType, T: QRegularExpressionMatch_captured_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.captured_2(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_captured_2<RetType> {
  fn captured_2(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_captured_2<usize> for (usize) {
  fn captured_2(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch8capturedE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:201
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringRef capturedRef(int) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedRef_0<RetType, T: QRegularExpressionMatch_capturedRef_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedRef_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedRef_0<RetType> {
  fn capturedRef_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef_0<usize> for (i32) {
  fn capturedRef_0(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch11capturedRefEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:206
// index:1
// Public Visibility=Default Availability=Available
// [16] QStringRef capturedRef(const QString &) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedRef_1<RetType, T: QRegularExpressionMatch_capturedRef_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedRef_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedRef_1<RetType> {
  fn capturedRef_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef_1<usize> for (usize) {
  fn capturedRef_1(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch11capturedRefERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:210
// index:2
// Public Visibility=Default Availability=Available
// [16] QStringRef capturedRef(QStringView) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedRef_2<RetType, T: QRegularExpressionMatch_capturedRef_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedRef_2(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedRef_2<RetType> {
  fn capturedRef_2(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedRef_2<usize> for (usize) {
  fn capturedRef_2(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch11capturedRefE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:202
// index:0
// Public Visibility=Default Availability=Available
// [16] QStringView capturedView(int) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedView_0<RetType, T: QRegularExpressionMatch_capturedView_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedView_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedView_0<RetType> {
  fn capturedView_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedView_0<usize> for (i32) {
  fn capturedView_0(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch12capturedViewEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:211
// index:1
// Public Visibility=Default Availability=Available
// [16] QStringView capturedView(QStringView) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedView_1<RetType, T: QRegularExpressionMatch_capturedView_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedView_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedView_1<RetType> {
  fn capturedView_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedView_1<usize> for (usize) {
  fn capturedView_1(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch12capturedViewE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:213
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList capturedTexts() const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedTexts_0<RetType, T: QRegularExpressionMatch_capturedTexts_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedTexts_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedTexts_0<RetType> {
  fn capturedTexts_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedTexts_0<usize> for () {
  fn capturedTexts_0(self , rsthis: & QRegularExpressionMatch) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch13capturedTextsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:215
// index:0
// Public Visibility=Default Availability=Available
// [4] int capturedStart(int) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart_0<RetType, T: QRegularExpressionMatch_capturedStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedStart_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedStart_0<RetType> {
  fn capturedStart_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart_0<i32> for (i32) {
  fn capturedStart_0(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch13capturedStartEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:220
// index:1
// Public Visibility=Default Availability=Available
// [4] int capturedStart(const QString &) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart_1<RetType, T: QRegularExpressionMatch_capturedStart_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedStart_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedStart_1<RetType> {
  fn capturedStart_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart_1<i32> for (usize) {
  fn capturedStart_1(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch13capturedStartERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:225
// index:2
// Public Visibility=Default Availability=Available
// [4] int capturedStart(QStringView) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedStart_2<RetType, T: QRegularExpressionMatch_capturedStart_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedStart_2(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedStart_2<RetType> {
  fn capturedStart_2(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedStart_2<i32> for (usize) {
  fn capturedStart_2(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch13capturedStartE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:216
// index:0
// Public Visibility=Default Availability=Available
// [4] int capturedLength(int) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength_0<RetType, T: QRegularExpressionMatch_capturedLength_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedLength_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedLength_0<RetType> {
  fn capturedLength_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength_0<i32> for (i32) {
  fn capturedLength_0(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch14capturedLengthEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:221
// index:1
// Public Visibility=Default Availability=Available
// [4] int capturedLength(const QString &) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength_1<RetType, T: QRegularExpressionMatch_capturedLength_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedLength_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedLength_1<RetType> {
  fn capturedLength_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength_1<i32> for (usize) {
  fn capturedLength_1(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch14capturedLengthERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:226
// index:2
// Public Visibility=Default Availability=Available
// [4] int capturedLength(QStringView) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedLength_2<RetType, T: QRegularExpressionMatch_capturedLength_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedLength_2(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedLength_2<RetType> {
  fn capturedLength_2(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedLength_2<i32> for (usize) {
  fn capturedLength_2(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch14capturedLengthE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:217
// index:0
// Public Visibility=Default Availability=Available
// [4] int capturedEnd(int) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd_0<RetType, T: QRegularExpressionMatch_capturedEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedEnd_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedEnd_0<RetType> {
  fn capturedEnd_0(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd_0<i32> for (i32) {
  fn capturedEnd_0(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch11capturedEndEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:222
// index:1
// Public Visibility=Default Availability=Available
// [4] int capturedEnd(const QString &) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd_1<RetType, T: QRegularExpressionMatch_capturedEnd_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedEnd_1(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedEnd_1<RetType> {
  fn capturedEnd_1(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd_1<i32> for (usize) {
  fn capturedEnd_1(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch11capturedEndERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qregularexpression.h:227
// index:2
// Public Visibility=Default Availability=Available
// [4] int capturedEnd(QStringView) const

/*

*/
impl /*struct*/ QRegularExpressionMatch {
  pub fn capturedEnd_2<RetType, T: QRegularExpressionMatch_capturedEnd_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.capturedEnd_2(self);
    // return 1;
  }
}
pub trait QRegularExpressionMatch_capturedEnd_2<RetType> {
  fn capturedEnd_2(self , rsthis: & QRegularExpressionMatch) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionMatch_capturedEnd_2<i32> for (usize) {
  fn capturedEnd_2(self , rsthis: & QRegularExpressionMatch) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QRegularExpressionMatch11capturedEndE11QStringView", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
