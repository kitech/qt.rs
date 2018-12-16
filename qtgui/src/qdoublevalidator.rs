

// mod ::gui::QDoubleValidator
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
// extern C begin: 13
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
#[derive(Default)] // class sizeof(QDoubleValidator)=40
pub struct QDoubleValidator {
  qbase: QValidator,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDoubleValidator_ITF interface {
//    QValidator_ITF
//    QDoubleValidator_PTR() *QDoubleValidator
//}
//func (ptr *QDoubleValidator) QDoubleValidator_PTR() *QDoubleValidator { return ptr }

impl /*struct*/ QDoubleValidator {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDoubleValidator {
    return QDoubleValidator{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDoubleValidator {
//  type Target = QDoubleValidatorBASE;
//
//  fn deref(&self) -> &QDoubleValidatorBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDoubleValidatorBASE> for QDoubleValidator {
//  fn as_ref(& self) -> & QDoubleValidatorBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qvalidator.h:126
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn metaObject_0<RetType, T: QDoubleValidator_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QDoubleValidator) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDoubleValidator10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:133
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDoubleValidator(QObject *)

/*

*/
// QDoubleValidator(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QDoubleValidator {
  pub fn QDoubleValidator_0<T: QDoubleValidator_QDoubleValidator_0>(value: T) -> QDoubleValidator {
    let rsthis = value.QDoubleValidator_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleValidator_QDoubleValidator_0 {
  fn QDoubleValidator_0(self) -> QDoubleValidator;
}
// QDoubleValidator(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDoubleValidator_QDoubleValidator_0 for (usize) {
  fn QDoubleValidator_0(self) -> QDoubleValidator {
    // unsafe{_ZN16QDoubleValidatorC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QDoubleValidatorC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDoubleValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:134
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDoubleValidator(double, double, int, QObject *)

/*

*/
// QDoubleValidator(double, double, int, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QDoubleValidator {
  pub fn QDoubleValidator_1<T: QDoubleValidator_QDoubleValidator_1>(value: T) -> QDoubleValidator {
    let rsthis = value.QDoubleValidator_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDoubleValidator_QDoubleValidator_1 {
  fn QDoubleValidator_1(self) -> QDoubleValidator;
}
// QDoubleValidator(double, double, int, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDoubleValidator_QDoubleValidator_1 for (f64,f64,i32,usize) {
  fn QDoubleValidator_1(self) -> QDoubleValidator {
    // unsafe{_ZN16QDoubleValidatorC2EddiP7QObject()};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QDoubleValidatorC2EddiP7QObject", 4,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDoubleValidator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:135
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDoubleValidator()

/*

*/
pub fn DeleteQDoubleValidator(this :*mut QDoubleValidator) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QDoubleValidatorD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qvalidator.h:142
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QValidator::State validate(QString &, int &) const

/*
This virtual function returns Invalid if input is invalid according to this validator's rules, Intermediate if it is likely that a little more editing will make the input acceptable (e.g. the user types "4" into a widget which accepts integers between 10 and 99), and Acceptable if the input is valid.

The function can change both input and pos (the cursor position) if required.
*/
impl /*struct*/ QDoubleValidator {
  pub fn validate_0<RetType, T: QDoubleValidator_validate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.validate_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_validate_0<RetType> {
  fn validate_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_validate_0<i32> for (usize,usize) {
  fn validate_0(self , rsthis: & QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDoubleValidator8validateER7QStringRi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:144
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void setRange(double, double, int)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn setRange_0<RetType, T: QDoubleValidator_setRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRange_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_setRange_0<RetType> {
  fn setRange_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_setRange_0<(/*void*/)> for (f64,f64,i32) {
  fn setRange_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const f64 as usize;
    let arg1 = (&self.1) as *const f64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator8setRangeEddi", 3,qtrt::FFITY_DOUBLE,qtrt::FFITY_DOUBLE,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setBottom(double)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn setBottom_0<RetType, T: QDoubleValidator_setBottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setBottom_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_setBottom_0<RetType> {
  fn setBottom_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_setBottom_0<(/*void*/)> for (f64) {
  fn setBottom_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator9setBottomEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTop(double)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn setTop_0<RetType, T: QDoubleValidator_setTop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTop_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_setTop_0<RetType> {
  fn setTop_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_setTop_0<(/*void*/)> for (f64) {
  fn setTop_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator6setTopEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDecimals(int)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn setDecimals_0<RetType, T: QDoubleValidator_setDecimals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDecimals_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_setDecimals_0<RetType> {
  fn setDecimals_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_setDecimals_0<(/*void*/)> for (i32) {
  fn setDecimals_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator11setDecimalsEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNotation(QDoubleValidator::Notation)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn setNotation_0<RetType, T: QDoubleValidator_setNotation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNotation_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_setNotation_0<RetType> {
  fn setNotation_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_setNotation_0<(/*void*/)> for (i32) {
  fn setNotation_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator11setNotationENS_8NotationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:150
// index:0
// Public inline Visibility=Default Availability=Available
// [8] double bottom() const

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn bottom_0<RetType, T: QDoubleValidator_bottom_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottom_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_bottom_0<RetType> {
  fn bottom_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_bottom_0<f64> for () {
  fn bottom_0(self , rsthis: & QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDoubleValidator6bottomEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:151
// index:0
// Public inline Visibility=Default Availability=Available
// [8] double top() const

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn top_0<RetType, T: QDoubleValidator_top_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.top_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_top_0<RetType> {
  fn top_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_top_0<f64> for () {
  fn top_0(self , rsthis: & QDoubleValidator) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDoubleValidator3topEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: f64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:152
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int decimals() const

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn decimals_0<RetType, T: QDoubleValidator_decimals_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decimals_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_decimals_0<RetType> {
  fn decimals_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_decimals_0<i32> for () {
  fn decimals_0(self , rsthis: & QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDoubleValidator8decimalsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:153
// index:0
// Public Visibility=Default Availability=Available
// [4] QDoubleValidator::Notation notation() const

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn notation_0<RetType, T: QDoubleValidator_notation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notation_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_notation_0<RetType> {
  fn notation_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_notation_0<i32> for () {
  fn notation_0(self , rsthis: & QDoubleValidator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QDoubleValidator8notationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void bottomChanged(double)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn bottomChanged_0<RetType, T: QDoubleValidator_bottomChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bottomChanged_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_bottomChanged_0<RetType> {
  fn bottomChanged_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_bottomChanged_0<(/*void*/)> for (f64) {
  fn bottomChanged_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator13bottomChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void topChanged(double)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn topChanged_0<RetType, T: QDoubleValidator_topChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.topChanged_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_topChanged_0<RetType> {
  fn topChanged_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_topChanged_0<(/*void*/)> for (f64) {
  fn topChanged_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const f64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator10topChangedEd", 1,qtrt::FFITY_DOUBLE,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void decimalsChanged(int)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn decimalsChanged_0<RetType, T: QDoubleValidator_decimalsChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.decimalsChanged_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_decimalsChanged_0<RetType> {
  fn decimalsChanged_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_decimalsChanged_0<(/*void*/)> for (i32) {
  fn decimalsChanged_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator15decimalsChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qvalidator.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void notationChanged(QDoubleValidator::Notation)

/*

*/
impl /*struct*/ QDoubleValidator {
  pub fn notationChanged_0<RetType, T: QDoubleValidator_notationChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notationChanged_0(self);
    // return 1;
  }
}
pub trait QDoubleValidator_notationChanged_0<RetType> {
  fn notationChanged_0(self , rsthis: & QDoubleValidator) -> RetType;
}
impl<'a> /*trait*/ QDoubleValidator_notationChanged_0<(/*void*/)> for (i32) {
  fn notationChanged_0(self , rsthis: & QDoubleValidator) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QDoubleValidator15notationChangedENS_8NotationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QDoubleValidator__Notation = i32;
// 
pub const QDoubleValidator__StandardNotation :QDoubleValidator__Notation = 0;
// 
pub const QDoubleValidator__ScientificNotation :QDoubleValidator__Notation = 1;
pub fn QDoubleValidator_NotationItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QDoubleValidator", val);
}
pub fn QDoubleValidator_NotationItemName_s(val: i32) ->String {
  //var nilthis *QDoubleValidator
  //return nilthis.NotationItemName(val);
  return QDoubleValidator_NotationItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
