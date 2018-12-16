

// mod ::core::QCollator
// package qtcore
// /usr/include/qt/QtCore/qcollator.h
// #include <qcollator.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QCollator)=8
pub struct QCollator {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QCollator_ITF interface {
//    QCollator_PTR() *QCollator
//}
//func (ptr *QCollator) QCollator_PTR() *QCollator { return ptr }

impl /*struct*/ QCollator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QCollator {
    return QCollator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QCollator {
//  type Target = QCollatorBASE;
//
//  fn deref(&self) -> &QCollatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QCollatorBASE> for QCollator {
//  fn as_ref(& self) -> & QCollatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcollator.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QCollator(const QLocale &)

/*
Constructs a QCollator from locale. If locale is not specified the system's default locale is used.

See also setLocale().
*/
// QCollator(const QLocale &) ctx.fn_proto_cpp
impl /*struct*/ QCollator {
  pub fn QCollator_0<T: QCollator_QCollator_0>(value: T) -> QCollator {
    let rsthis = value.QCollator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QCollator_QCollator_0 {
  fn QCollator_0(self) -> QCollator;
}
// QCollator(const QLocale &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QCollator_QCollator_0 for (usize) {
  fn QCollator_0(self) -> QCollator {
    // unsafe{_ZN9QCollatorC2ERK7QLocale()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QCollatorC2ERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QCollator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QCollator()

/*

*/
pub fn DeleteQCollator(this :*mut QCollator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QCollatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcollator.h:89
// index:0
// Public Visibility=Default Availability=Available
// [8] QCollator & operator=(const QCollator &)

/*

*/
impl /*struct*/ QCollator {
  pub fn operator_equal_0<RetType, T: QCollator_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QCollator_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QCollator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QCollatoraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:93
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QCollator & operator=(QCollator &&)

/*

*/
impl /*struct*/ QCollator {
  pub fn operator_equal_1<RetType, T: QCollator_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QCollator_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QCollator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QCollatoraSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:97
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QCollator &)

/*
Swaps this collator with other. This function is very fast and never fails.
*/
impl /*struct*/ QCollator {
  pub fn swap_0<RetType, T: QCollator_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QCollator_swap_0<RetType> {
  fn swap_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QCollator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QCollator4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:100
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLocale(const QLocale &)

/*
Sets the locale of the collator to locale.

See also locale().
*/
impl /*struct*/ QCollator {
  pub fn setLocale_0<RetType, T: QCollator_setLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLocale_0(self);
    // return 1;
  }
}
pub trait QCollator_setLocale_0<RetType> {
  fn setLocale_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_setLocale_0<(/*void*/)> for (usize) {
  fn setLocale_0(self , rsthis: & QCollator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QCollator9setLocaleERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale locale() const

/*
Returns the locale of the collator.

See also setLocale().
*/
impl /*struct*/ QCollator {
  pub fn locale_0<RetType, T: QCollator_locale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.locale_0(self);
    // return 1;
  }
}
pub trait QCollator_locale_0<RetType> {
  fn locale_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_locale_0<usize> for () {
  fn locale_0(self , rsthis: & QCollator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator6localeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::CaseSensitivity caseSensitivity() const

/*
Returns case sensitivity of the collator.

See also setCaseSensitivity().
*/
impl /*struct*/ QCollator {
  pub fn caseSensitivity_0<RetType, T: QCollator_caseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.caseSensitivity_0(self);
    // return 1;
  }
}
pub trait QCollator_caseSensitivity_0<RetType> {
  fn caseSensitivity_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_caseSensitivity_0<i32> for () {
  fn caseSensitivity_0(self , rsthis: & QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator15caseSensitivityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCaseSensitivity(Qt::CaseSensitivity)

/*
Sets the case sensitivity of the collator.

See also caseSensitivity().
*/
impl /*struct*/ QCollator {
  pub fn setCaseSensitivity_0<RetType, T: QCollator_setCaseSensitivity_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCaseSensitivity_0(self);
    // return 1;
  }
}
pub trait QCollator_setCaseSensitivity_0<RetType> {
  fn setCaseSensitivity_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_setCaseSensitivity_0<(/*void*/)> for (i32) {
  fn setCaseSensitivity_0(self , rsthis: & QCollator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QCollator18setCaseSensitivityEN2Qt15CaseSensitivityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:106
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNumericMode(bool)

/*
Enables numeric sorting mode when on is set to true.

This will enable proper sorting of numeric digits, so that e.g. 100 sorts after 99.

By default this mode is off.

Note: On Windows, this functionality makes use of the ICU library. If Qt was compiled without ICU support, it falls back to code using native Windows API, which only works from Windows 7 onwards. On older versions of Windows, it will not work and a warning will be emitted at runtime.

See also numericMode().
*/
impl /*struct*/ QCollator {
  pub fn setNumericMode_0<RetType, T: QCollator_setNumericMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNumericMode_0(self);
    // return 1;
  }
}
pub trait QCollator_setNumericMode_0<RetType> {
  fn setNumericMode_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_setNumericMode_0<(/*void*/)> for (bool) {
  fn setNumericMode_0(self , rsthis: & QCollator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QCollator14setNumericModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool numericMode() const

/*
Returns true if numeric sorting is enabled, false otherwise.

See also setNumericMode().
*/
impl /*struct*/ QCollator {
  pub fn numericMode_0<RetType, T: QCollator_numericMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.numericMode_0(self);
    // return 1;
  }
}
pub trait QCollator_numericMode_0<RetType> {
  fn numericMode_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_numericMode_0<bool> for () {
  fn numericMode_0(self , rsthis: & QCollator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator11numericModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIgnorePunctuation(bool)

/*
If on is set to true, punctuation characters and symbols are ignored when determining sort order.

The default is locale dependent.

Note: This method is not currently supported if Qt is configured to not use ICU on Linux.

See also ignorePunctuation().
*/
impl /*struct*/ QCollator {
  pub fn setIgnorePunctuation_0<RetType, T: QCollator_setIgnorePunctuation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIgnorePunctuation_0(self);
    // return 1;
  }
}
pub trait QCollator_setIgnorePunctuation_0<RetType> {
  fn setIgnorePunctuation_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_setIgnorePunctuation_0<(/*void*/)> for (bool) {
  fn setIgnorePunctuation_0(self , rsthis: & QCollator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QCollator20setIgnorePunctuationEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcollator.h:110
// index:0
// Public Visibility=Default Availability=Available
// [1] bool ignorePunctuation() const

/*
Returns true if punctuation characters and symbols are ignored when determining sort order.

See also setIgnorePunctuation().
*/
impl /*struct*/ QCollator {
  pub fn ignorePunctuation_0<RetType, T: QCollator_ignorePunctuation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ignorePunctuation_0(self);
    // return 1;
  }
}
pub trait QCollator_ignorePunctuation_0<RetType> {
  fn ignorePunctuation_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_ignorePunctuation_0<bool> for () {
  fn ignorePunctuation_0(self , rsthis: & QCollator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator17ignorePunctuationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:112
// index:0
// Public Visibility=Default Availability=Available
// [4] int compare(const QString &, const QString &) const

/*
Compares s1 with s2. Returns an integer less than, equal to, or greater than zero depending on whether s1 is smaller, equal or larger than s2.
*/
impl /*struct*/ QCollator {
  pub fn compare_0<RetType, T: QCollator_compare_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_0(self);
    // return 1;
  }
}
pub trait QCollator_compare_0<RetType> {
  fn compare_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_compare_0<i32> for (usize,usize) {
  fn compare_0(self , rsthis: & QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator7compareERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:113
// index:1
// Public Visibility=Default Availability=Available
// [4] int compare(const QStringRef &, const QStringRef &) const

/*
Compares s1 with s2. Returns an integer less than, equal to, or greater than zero depending on whether s1 is smaller, equal or larger than s2.
*/
impl /*struct*/ QCollator {
  pub fn compare_1<RetType, T: QCollator_compare_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_1(self);
    // return 1;
  }
}
pub trait QCollator_compare_1<RetType> {
  fn compare_1(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_compare_1<i32> for (usize,usize) {
  fn compare_1(self , rsthis: & QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator7compareERK10QStringRefS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:114
// index:2
// Public Visibility=Default Availability=Available
// [4] int compare(const QChar *, int, const QChar *, int) const

/*
Compares s1 with s2. Returns an integer less than, equal to, or greater than zero depending on whether s1 is smaller, equal or larger than s2.
*/
impl /*struct*/ QCollator {
  pub fn compare_2<RetType, T: QCollator_compare_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.compare_2(self);
    // return 1;
  }
}
pub trait QCollator_compare_2<RetType> {
  fn compare_2(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_compare_2<i32> for (usize,i32,usize,i32) {
  fn compare_2(self , rsthis: & QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator7compareEPK5QChariS2_i", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:116
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator()(const QString &, const QString &) const

/*

*/
impl /*struct*/ QCollator {
  pub fn operator_fncall_0<RetType, T: QCollator_operator_fncall_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_fncall_0(self);
    // return 1;
  }
}
pub trait QCollator_operator_fncall_0<RetType> {
  fn operator_fncall_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_operator_fncall_0<bool> for (usize,usize) {
  fn operator_fncall_0(self , rsthis: & QCollator) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollatorclERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcollator.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QCollatorSortKey sortKey(const QString &) const

/*
Returns a sortKey for string.

Creating the sort key is usually somewhat slower, than using the compare() methods directly. But if the string is compared repeatedly (e.g. when sorting a whole list of strings), it's usually faster to create the sort keys for each string and then sort using the keys.
*/
impl /*struct*/ QCollator {
  pub fn sortKey_0<RetType, T: QCollator_sortKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sortKey_0(self);
    // return 1;
  }
}
pub trait QCollator_sortKey_0<RetType> {
  fn sortKey_0(self , rsthis: & QCollator) -> RetType;
}
impl<'a> /*trait*/ QCollator_sortKey_0<usize> for (usize) {
  fn sortKey_0(self , rsthis: & QCollator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QCollator7sortKeyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
