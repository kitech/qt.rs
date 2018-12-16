

// mod ::gui::QRegularExpressionValidator
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
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QRegularExpressionValidator)=16
pub struct QRegularExpressionValidator {
  qbase: QValidator,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegularExpressionValidator_ITF interface {
//    QValidator_ITF
//    QRegularExpressionValidator_PTR() *QRegularExpressionValidator
//}
//func (ptr *QRegularExpressionValidator) QRegularExpressionValidator_PTR() *QRegularExpressionValidator { return ptr }

impl /*struct*/ QRegularExpressionValidator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegularExpressionValidator {
    return QRegularExpressionValidator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegularExpressionValidator {
//  type Target = QRegularExpressionValidatorBASE;
//
//  fn deref(&self) -> &QRegularExpressionValidatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegularExpressionValidatorBASE> for QRegularExpressionValidator {
//  fn as_ref(& self) -> & QRegularExpressionValidatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvalidator.h:203
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QRegularExpressionValidator {
  pub fn metaObject_0<RetType, T: QRegularExpressionValidator_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionValidator_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QRegularExpressionValidator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionValidator_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QRegularExpressionValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QRegularExpressionValidator10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:207
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegularExpressionValidator(QObject *)

/*

*/
// QRegularExpressionValidator(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QRegularExpressionValidator {
  pub fn QRegularExpressionValidator_0<T: QRegularExpressionValidator_QRegularExpressionValidator_0>(value: T) -> QRegularExpressionValidator {
    let rsthis = value.QRegularExpressionValidator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionValidator_QRegularExpressionValidator_0 {
  fn QRegularExpressionValidator_0(self) -> QRegularExpressionValidator;
}
// QRegularExpressionValidator(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegularExpressionValidator_QRegularExpressionValidator_0 for (usize) {
  fn QRegularExpressionValidator_0(self) -> QRegularExpressionValidator {
    // unsafe{_ZN27QRegularExpressionValidatorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QRegularExpressionValidatorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegularExpressionValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:208
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRegularExpressionValidator(const QRegularExpression &, QObject *)

/*

*/
// QRegularExpressionValidator(const QRegularExpression &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QRegularExpressionValidator {
  pub fn QRegularExpressionValidator_1<T: QRegularExpressionValidator_QRegularExpressionValidator_1>(value: T) -> QRegularExpressionValidator {
    let rsthis = value.QRegularExpressionValidator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionValidator_QRegularExpressionValidator_1 {
  fn QRegularExpressionValidator_1(self) -> QRegularExpressionValidator;
}
// QRegularExpressionValidator(const QRegularExpression &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegularExpressionValidator_QRegularExpressionValidator_1 for (usize,usize) {
  fn QRegularExpressionValidator_1(self) -> QRegularExpressionValidator {
    // unsafe{_ZN27QRegularExpressionValidatorC2ERK18QRegularExpressionP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN27QRegularExpressionValidatorC2ERK18QRegularExpressionP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegularExpressionValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:209
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QRegularExpressionValidator()

/*

*/
pub fn DeleteQRegularExpressionValidator(this :*mut QRegularExpressionValidator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN27QRegularExpressionValidatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qvalidator.h:211
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
This virtual function returns Invalid if input is invalid according to this validator's rules, Intermediate if it is likely that a little more editing will make the input acceptable (e.g. the user types "4" into a widget which accepts integers between 10 and 99), and Acceptable if the input is valid.

The function can change both input and pos (the cursor position) if required.
*/
impl /*struct*/ QRegularExpressionValidator {
  pub fn validate_0<RetType, T: QRegularExpressionValidator_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionValidator_validate_0<RetType> {
  fn validate_0(self , rsthis: & QRegularExpressionValidator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionValidator_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QRegularExpressionValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QRegularExpressionValidator8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:213
// index:0
// Public Visibility=Default Availability=Available
// [8] QRegularExpression regularExpression() const

/*

*/
impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpression_0<RetType, T: QRegularExpressionValidator_regularExpression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.regularExpression_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionValidator_regularExpression_0<RetType> {
  fn regularExpression_0(self , rsthis: & QRegularExpressionValidator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpression_0<usize> for () {
  fn regularExpression_0(self , rsthis: & QRegularExpressionValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK27QRegularExpressionValidator17regularExpressionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:216
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRegularExpression(const QRegularExpression &)

/*

*/
impl /*struct*/ QRegularExpressionValidator {
  pub fn setRegularExpression_0<RetType, T: QRegularExpressionValidator_setRegularExpression_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRegularExpression_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionValidator_setRegularExpression_0<RetType> {
  fn setRegularExpression_0(self , rsthis: & QRegularExpressionValidator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionValidator_setRegularExpression_0<(/*void*/)> for (usize) {
  fn setRegularExpression_0(self , rsthis: & QRegularExpressionValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QRegularExpressionValidator20setRegularExpressionERK18QRegularExpression", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:219
// index:0
// Public Visibility=Default Availability=Available
// [-2] void regularExpressionChanged(const QRegularExpression &)

/*

*/
impl /*struct*/ QRegularExpressionValidator {
  pub fn regularExpressionChanged_0<RetType, T: QRegularExpressionValidator_regularExpressionChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.regularExpressionChanged_0(self);
    // return 1;
  }
}
pub trait QRegularExpressionValidator_regularExpressionChanged_0<RetType> {
  fn regularExpressionChanged_0(self , rsthis: & QRegularExpressionValidator) -> RetType;
}
impl<'a> /*trait*/ QRegularExpressionValidator_regularExpressionChanged_0<(/*void*/)> for (usize) {
  fn regularExpressionChanged_0(self , rsthis: & QRegularExpressionValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN27QRegularExpressionValidator24regularExpressionChangedERK18QRegularExpression", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
