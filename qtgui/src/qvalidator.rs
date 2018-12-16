

// mod ::gui::QValidator
// package qtgui
// /usr/include/qt/QtGui/qvalidator.h
// #include <qvalidator.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 22
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
#[derive(Default)] // class sizeof(QValidator)=16
pub struct QValidator {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QValidator_ITF interface {
//    qtcore.QObject_ITF
//    QValidator_PTR() *QValidator
//}
//func (ptr *QValidator) QValidator_PTR() *QValidator { return ptr }

impl /*struct*/ QValidator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QValidator {
    return QValidator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QValidator {
//  type Target = QValidatorBASE;
//
//  fn deref(&self) -> &QValidatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QValidatorBASE> for QValidator {
//  fn as_ref(& self) -> & QValidatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvalidator.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QValidator {
  pub fn metaObject_0<RetType, T: QValidator_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QValidator_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QValidator) -> RetType;
}
impl<'a> /*trait*/ QValidator_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QValidator10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QValidator(QObject *)

/*
Sets up the validator. The parent parameter is passed on to the QObject constructor.
*/
// QValidator(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QValidator {
  pub fn QValidator_0<T: QValidator_QValidator_0>(value: T) -> QValidator {
    let rsthis = value.QValidator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QValidator_QValidator_0 {
  fn QValidator_0(self) -> QValidator;
}
// QValidator(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QValidator_QValidator_0 for (usize) {
  fn QValidator_0(self) -> QValidator {
    // unsafe{_ZN10QValidatorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QValidatorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QValidator()

/*

*/
pub fn DeleteQValidator(this :*mut QValidator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QValidatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qvalidator.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLocale(const QLocale &)

/*
Sets the locale that will be used for the validator. Unless setLocale has been called, the validator will use the default locale set with QLocale::setDefault(). If a default locale has not been set, it is the operating system's locale.

See also locale() and QLocale::setDefault().
*/
impl /*struct*/ QValidator {
  pub fn setLocale_0<RetType, T: QValidator_setLocale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLocale_0(self);
    // return 1;
  }
}
pub trait QValidator_setLocale_0<RetType> {
  fn setLocale_0(self , rsthis: & QValidator) -> RetType;
}
impl<'a> /*trait*/ QValidator_setLocale_0<(/*void*/)> for (usize) {
  fn setLocale_0(self , rsthis: & QValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN10QValidator9setLocaleERK7QLocale", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:72
// index:0
// Public Visibility=Default Availability=Available
// [8] QLocale locale() const

/*
Returns the locale for the validator. The locale is by default initialized to the same as QLocale().

See also setLocale() and QLocale::QLocale().
*/
impl /*struct*/ QValidator {
  pub fn locale_0<RetType, T: QValidator_locale_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.locale_0(self);
    // return 1;
  }
}
pub trait QValidator_locale_0<RetType> {
  fn locale_0(self , rsthis: & QValidator) -> RetType;
}
impl<'a> /*trait*/ QValidator_locale_0<usize> for () {
  fn locale_0(self , rsthis: & QValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QValidator6localeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:74
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
This virtual function returns Invalid if input is invalid according to this validator's rules, Intermediate if it is likely that a little more editing will make the input acceptable (e.g. the user types "4" into a widget which accepts integers between 10 and 99), and Acceptable if the input is valid.

The function can change both input and pos (the cursor position) if required.
*/
impl /*struct*/ QValidator {
  pub fn validate_0<RetType, T: QValidator_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QValidator_validate_0<RetType> {
  fn validate_0(self , rsthis: & QValidator) -> RetType;
}
impl<'a> /*trait*/ QValidator_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QValidator8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fixup(QString &) const

/*
This function attempts to change input to be valid according to this validator's rules. It need not result in a valid string: callers of this function must re-test afterwards; the default does nothing.

Reimplementations of this function can change input even if they do not produce a valid string. For example, an ISBN validator might want to delete every character except digits and "-", even if the result is still not a valid ISBN; a surname validator might want to remove whitespace from the start and end of the string, even if the resulting string is not in the list of accepted surnames.
*/
impl /*struct*/ QValidator {
  pub fn fixup_0<RetType, T: QValidator_fixup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixup_0(self);
    // return 1;
  }
}
pub trait QValidator_fixup_0<RetType> {
  fn fixup_0(self , rsthis: & QValidator) -> RetType;
}
impl<'a> /*trait*/ QValidator_fixup_0<(/*void*/)> for (usize) {
  fn fixup_0(self , rsthis: & QValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK10QValidator5fixupER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void changed()

/*
This signal is emitted when any property that may affect the validity of a string has changed.
*/
impl /*struct*/ QValidator {
  pub fn changed_0<RetType, T: QValidator_changed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changed_0(self);
    // return 1;
  }
}
pub trait QValidator_changed_0<RetType> {
  fn changed_0(self , rsthis: & QValidator) -> RetType;
}
impl<'a> /*trait*/ QValidator_changed_0<(/*void*/)> for () {
  fn changed_0(self , rsthis: & QValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QValidator7changedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type defines the states in which a validated string can exist.


*/
pub type QValidator__State = i32;
// The string is clearly invalid.
pub const QValidator__Invalid :QValidator__State = 0;
// The string is a plausible intermediate value.
pub const QValidator__Intermediate :QValidator__State = 1;
// The string is acceptable as a final result; i.e. it is valid.
pub const QValidator__Acceptable :QValidator__State = 2;
pub fn QValidator_StateItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QValidator", val);
}
pub fn QValidator_StateItemName_s(val: i32) ->String {
  //var nilthis *QValidator
  //return nilthis.StateItemName(val);
  return QValidator_StateItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
