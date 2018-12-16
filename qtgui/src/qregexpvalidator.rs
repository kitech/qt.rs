

// mod ::gui::QRegExpValidator
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
#[derive(Default)] // class sizeof(QRegExpValidator)=24
pub struct QRegExpValidator {
  qbase: QValidator,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QRegExpValidator_ITF interface {
//    QValidator_ITF
//    QRegExpValidator_PTR() *QRegExpValidator
//}
//func (ptr *QRegExpValidator) QRegExpValidator_PTR() *QRegExpValidator { return ptr }

impl /*struct*/ QRegExpValidator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QRegExpValidator {
    return QRegExpValidator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QRegExpValidator {
//  type Target = QRegExpValidatorBASE;
//
//  fn deref(&self) -> &QRegExpValidatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QRegExpValidatorBASE> for QRegExpValidator {
//  fn as_ref(& self) -> & QRegExpValidatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvalidator.h:173
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QRegExpValidator {
  pub fn metaObject_0<RetType, T: QRegExpValidator_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QRegExpValidator_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QRegExpValidator) -> RetType;
}
impl<'a> /*trait*/ QRegExpValidator_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QRegExpValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QRegExpValidator10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:177
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QRegExpValidator(QObject *)

/*

*/
// QRegExpValidator(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QRegExpValidator {
  pub fn QRegExpValidator_0<T: QRegExpValidator_QRegExpValidator_0>(value: T) -> QRegExpValidator {
    let rsthis = value.QRegExpValidator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExpValidator_QRegExpValidator_0 {
  fn QRegExpValidator_0(self) -> QRegExpValidator;
}
// QRegExpValidator(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegExpValidator_QRegExpValidator_0 for (usize) {
  fn QRegExpValidator_0(self) -> QRegExpValidator {
    // unsafe{_ZN16QRegExpValidatorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QRegExpValidatorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegExpValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:178
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QRegExpValidator(const QRegExp &, QObject *)

/*

*/
// QRegExpValidator(const QRegExp &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QRegExpValidator {
  pub fn QRegExpValidator_1<T: QRegExpValidator_QRegExpValidator_1>(value: T) -> QRegExpValidator {
    let rsthis = value.QRegExpValidator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExpValidator_QRegExpValidator_1 {
  fn QRegExpValidator_1(self) -> QRegExpValidator;
}
// QRegExpValidator(const QRegExp &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QRegExpValidator_QRegExpValidator_1 for (usize,usize) {
  fn QRegExpValidator_1(self) -> QRegExpValidator {
    // unsafe{_ZN16QRegExpValidatorC2ERK7QRegExpP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QRegExpValidatorC2ERK7QRegExpP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QRegExpValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:179
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QRegExpValidator()

/*

*/
pub fn DeleteQRegExpValidator(this :*mut QRegExpValidator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QRegExpValidatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qvalidator.h:181
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
This virtual function returns Invalid if input is invalid according to this validator's rules, Intermediate if it is likely that a little more editing will make the input acceptable (e.g. the user types "4" into a widget which accepts integers between 10 and 99), and Acceptable if the input is valid.

The function can change both input and pos (the cursor position) if required.
*/
impl /*struct*/ QRegExpValidator {
  pub fn validate_0<RetType, T: QRegExpValidator_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QRegExpValidator_validate_0<RetType> {
  fn validate_0(self , rsthis: & QRegExpValidator) -> RetType;
}
impl<'a> /*trait*/ QRegExpValidator_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QRegExpValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QRegExpValidator8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:183
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRegExp(const QRegExp &)

/*

*/
impl /*struct*/ QRegExpValidator {
  pub fn setRegExp_0<RetType, T: QRegExpValidator_setRegExp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRegExp_0(self);
    // return 1;
  }
}
pub trait QRegExpValidator_setRegExp_0<RetType> {
  fn setRegExp_0(self , rsthis: & QRegExpValidator) -> RetType;
}
impl<'a> /*trait*/ QRegExpValidator_setRegExp_0<(/*void*/)> for (usize) {
  fn setRegExp_0(self , rsthis: & QRegExpValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QRegExpValidator9setRegExpERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:184
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QRegExp & regExp() const

/*

*/
impl /*struct*/ QRegExpValidator {
  pub fn regExp_0<RetType, T: QRegExpValidator_regExp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.regExp_0(self);
    // return 1;
  }
}
pub trait QRegExpValidator_regExp_0<RetType> {
  fn regExp_0(self , rsthis: & QRegExpValidator) -> RetType;
}
impl<'a> /*trait*/ QRegExpValidator_regExp_0<usize> for () {
  fn regExp_0(self , rsthis: & QRegExpValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QRegExpValidator6regExpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:187
// index:0
// Public Visibility=Default Availability=Available
// [-2] void regExpChanged(const QRegExp &)

/*

*/
impl /*struct*/ QRegExpValidator {
  pub fn regExpChanged_0<RetType, T: QRegExpValidator_regExpChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.regExpChanged_0(self);
    // return 1;
  }
}
pub trait QRegExpValidator_regExpChanged_0<RetType> {
  fn regExpChanged_0(self , rsthis: & QRegExpValidator) -> RetType;
}
impl<'a> /*trait*/ QRegExpValidator_regExpChanged_0<(/*void*/)> for (usize) {
  fn regExpChanged_0(self , rsthis: & QRegExpValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QRegExpValidator13regExpChangedERK7QRegExp", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
