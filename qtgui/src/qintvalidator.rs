

// mod ::gui::QIntValidator
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
#[derive(Default)] // class sizeof(QIntValidator)=24
pub struct QIntValidator {
  qbase: QValidator,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIntValidator_ITF interface {
//    QValidator_ITF
//    QIntValidator_PTR() *QIntValidator
//}
//func (ptr *QIntValidator) QIntValidator_PTR() *QIntValidator { return ptr }

impl /*struct*/ QIntValidator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIntValidator {
    return QIntValidator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIntValidator {
//  type Target = QIntValidatorBASE;
//
//  fn deref(&self) -> &QIntValidatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIntValidatorBASE> for QIntValidator {
//  fn as_ref(& self) -> & QIntValidatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvalidator.h:91
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QIntValidator {
  pub fn metaObject_0<RetType, T: QIntValidator_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QIntValidator_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QIntValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QIntValidator10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QIntValidator(QObject *)

/*

*/
// QIntValidator(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QIntValidator {
  pub fn QIntValidator_0<T: QIntValidator_QIntValidator_0>(value: T) -> QIntValidator {
    let rsthis = value.QIntValidator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIntValidator_QIntValidator_0 {
  fn QIntValidator_0(self) -> QIntValidator;
}
// QIntValidator(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIntValidator_QIntValidator_0 for (usize) {
  fn QIntValidator_0(self) -> QIntValidator {
    // unsafe{_ZN13QIntValidatorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QIntValidatorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIntValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:97
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QIntValidator(int, int, QObject *)

/*

*/
// QIntValidator(int, int, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QIntValidator {
  pub fn QIntValidator_1<T: QIntValidator_QIntValidator_1>(value: T) -> QIntValidator {
    let rsthis = value.QIntValidator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QIntValidator_QIntValidator_1 {
  fn QIntValidator_1(self) -> QIntValidator;
}
// QIntValidator(int, int, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIntValidator_QIntValidator_1 for (i32,i32,usize) {
  fn QIntValidator_1(self) -> QIntValidator {
    // unsafe{_ZN13QIntValidatorC2EiiP7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QIntValidatorC2EiiP7QObject", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIntValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QIntValidator()

/*

*/
pub fn DeleteQIntValidator(this :*mut QIntValidator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QIntValidatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qvalidator.h:100
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
This virtual function returns Invalid if input is invalid according to this validator's rules, Intermediate if it is likely that a little more editing will make the input acceptable (e.g. the user types "4" into a widget which accepts integers between 10 and 99), and Acceptable if the input is valid.

The function can change both input and pos (the cursor position) if required.
*/
impl /*struct*/ QIntValidator {
  pub fn validate_0<RetType, T: QIntValidator_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QIntValidator_validate_0<RetType> {
  fn validate_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QIntValidator8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:101
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void fixup(QString &) const

/*
This function attempts to change input to be valid according to this validator's rules. It need not result in a valid string: callers of this function must re-test afterwards; the default does nothing.

Reimplementations of this function can change input even if they do not produce a valid string. For example, an ISBN validator might want to delete every character except digits and "-", even if the result is still not a valid ISBN; a surname validator might want to remove whitespace from the start and end of the string, even if the resulting string is not in the list of accepted surnames.
*/
impl /*struct*/ QIntValidator {
  pub fn fixup_0<RetType, T: QIntValidator_fixup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fixup_0(self);
    // return 1;
  }
}
pub trait QIntValidator_fixup_0<RetType> {
  fn fixup_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_fixup_0<(/*void*/)> for (usize) {
  fn fixup_0(self , rsthis: & QIntValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZNK13QIntValidator5fixupER7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:103
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBottom(int)

/*

*/
impl /*struct*/ QIntValidator {
  pub fn setBottom_0<RetType, T: QIntValidator_setBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottom_0(self);
    // return 1;
  }
}
pub trait QIntValidator_setBottom_0<RetType> {
  fn setBottom_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_setBottom_0<(/*void*/)> for (i32) {
  fn setBottom_0(self , rsthis: & QIntValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QIntValidator9setBottomEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTop(int)

/*

*/
impl /*struct*/ QIntValidator {
  pub fn setTop_0<RetType, T: QIntValidator_setTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTop_0(self);
    // return 1;
  }
}
pub trait QIntValidator_setTop_0<RetType> {
  fn setTop_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_setTop_0<(/*void*/)> for (i32) {
  fn setTop_0(self , rsthis: & QIntValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QIntValidator6setTopEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRange(int, int)

/*

*/
impl /*struct*/ QIntValidator {
  pub fn setRange_0<RetType, T: QIntValidator_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QIntValidator_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_setRange_0<(/*void*/)> for (i32,i32) {
  fn setRange_0(self , rsthis: & QIntValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QIntValidator8setRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:107
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int bottom() const

/*

*/
impl /*struct*/ QIntValidator {
  pub fn bottom_0<RetType, T: QIntValidator_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QIntValidator_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_bottom_0<i32> for () {
  fn bottom_0(self , rsthis: & QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QIntValidator6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:108
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int top() const

/*

*/
impl /*struct*/ QIntValidator {
  pub fn top_0<RetType, T: QIntValidator_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QIntValidator_top_0<RetType> {
  fn top_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_top_0<i32> for () {
  fn top_0(self , rsthis: & QIntValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QIntValidator3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void bottomChanged(int)

/*

*/
impl /*struct*/ QIntValidator {
  pub fn bottomChanged_0<RetType, T: QIntValidator_bottomChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomChanged_0(self);
    // return 1;
  }
}
pub trait QIntValidator_bottomChanged_0<RetType> {
  fn bottomChanged_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_bottomChanged_0<(/*void*/)> for (i32) {
  fn bottomChanged_0(self , rsthis: & QIntValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QIntValidator13bottomChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void topChanged(int)

/*

*/
impl /*struct*/ QIntValidator {
  pub fn topChanged_0<RetType, T: QIntValidator_topChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topChanged_0(self);
    // return 1;
  }
}
pub trait QIntValidator_topChanged_0<RetType> {
  fn topChanged_0(self , rsthis: & QIntValidator) -> RetType;
}
impl<'a> /*trait*/ QIntValidator_topChanged_0<(/*void*/)> for (i32) {
  fn topChanged_0(self , rsthis: & QIntValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QIntValidator10topChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
