

// mod ::core::QMessageAuthenticationCode
// package qtcore
// /usr/include/qt/QtCore/qmessageauthenticationcode.h
// #include <qmessageauthenticationcode.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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
#[derive(Default)] // class sizeof(QMessageAuthenticationCode)=8
pub struct QMessageAuthenticationCode {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMessageAuthenticationCode_ITF interface {
//    QMessageAuthenticationCode_PTR() *QMessageAuthenticationCode
//}
//func (ptr *QMessageAuthenticationCode) QMessageAuthenticationCode_PTR() *QMessageAuthenticationCode { return ptr }

impl /*struct*/ QMessageAuthenticationCode {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMessageAuthenticationCode {
    return QMessageAuthenticationCode{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMessageAuthenticationCode {
//  type Target = QMessageAuthenticationCodeBASE;
//
//  fn deref(&self) -> &QMessageAuthenticationCodeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMessageAuthenticationCodeBASE> for QMessageAuthenticationCode {
//  fn as_ref(& self) -> & QMessageAuthenticationCodeBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmessageauthenticationcode.h:54
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMessageAuthenticationCode(QCryptographicHash::Algorithm, const QByteArray &)

/*
Constructs an object that can be used to create a cryptographic hash from data using method method and key key.
*/
// QMessageAuthenticationCode(QCryptographicHash::Algorithm, const QByteArray &) ctx.fn_proto_cpp
impl /*struct*/ QMessageAuthenticationCode {
  pub fn QMessageAuthenticationCode_0<T: QMessageAuthenticationCode_QMessageAuthenticationCode_0>(value: T) -> QMessageAuthenticationCode {
    let rsthis = value.QMessageAuthenticationCode_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_QMessageAuthenticationCode_0 {
  fn QMessageAuthenticationCode_0(self) -> QMessageAuthenticationCode;
}
// QMessageAuthenticationCode(QCryptographicHash::Algorithm, const QByteArray &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMessageAuthenticationCode_QMessageAuthenticationCode_0 for (i32,usize) {
  fn QMessageAuthenticationCode_0(self) -> QMessageAuthenticationCode {
    // unsafe{_ZN26QMessageAuthenticationCodeC2EN18QCryptographicHash9AlgorithmERK10QByteArray()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCodeC2EN18QCryptographicHash9AlgorithmERK10QByteArray", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMessageAuthenticationCode{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QMessageAuthenticationCode()

/*

*/
pub fn DeleteQMessageAuthenticationCode(this :*mut QMessageAuthenticationCode) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCodeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qmessageauthenticationcode.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reset()

/*
Resets message data. Calling this method doesn't affect the key.
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn reset_0<RetType, T: QMessageAuthenticationCode_reset_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reset_0(self);
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_reset_0<RetType> {
  fn reset_0(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_reset_0<(/*void*/)> for () {
  fn reset_0(self , rsthis: & QMessageAuthenticationCode) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCode5resetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKey(const QByteArray &)

/*
Sets secret key. Calling this method automatically resets the object state.
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn setKey_0<RetType, T: QMessageAuthenticationCode_setKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKey_0(self);
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_setKey_0<RetType> {
  fn setKey_0(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_setKey_0<(/*void*/)> for (usize) {
  fn setKey_0(self , rsthis: & QMessageAuthenticationCode) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addData(const char *, int)

/*
Adds the first length chars of data to the message.
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn addData_0<RetType, T: QMessageAuthenticationCode_addData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_0(self);
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_addData_0<RetType> {
  fn addData_0(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_addData_0<(/*void*/)> for (usize,i32) {
  fn addData_0(self , rsthis: & QMessageAuthenticationCode) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCode7addDataEPKci", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:63
// index:1
// Public Visibility=Default Availability=Available
// [-2] void addData(const QByteArray &)

/*
Adds the first length chars of data to the message.
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn addData_1<RetType, T: QMessageAuthenticationCode_addData_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_1(self);
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_addData_1<RetType> {
  fn addData_1(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_addData_1<(/*void*/)> for (usize) {
  fn addData_1(self , rsthis: & QMessageAuthenticationCode) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCode7addDataERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:64
// index:2
// Public Visibility=Default Availability=Available
// [1] bool addData(QIODevice *)

/*
Adds the first length chars of data to the message.
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn addData_2<RetType, T: QMessageAuthenticationCode_addData_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addData_2(self);
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_addData_2<RetType> {
  fn addData_2(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_addData_2<bool> for (usize) {
  fn addData_2(self , rsthis: & QMessageAuthenticationCode) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCode7addDataEP9QIODevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray result() const

/*
Returns the final authentication code.

See also QByteArray::toHex().
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn result_0<RetType, T: QMessageAuthenticationCode_result_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.result_0(self);
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_result_0<RetType> {
  fn result_0(self , rsthis: & QMessageAuthenticationCode) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_result_0<usize> for () {
  fn result_0(self , rsthis: & QMessageAuthenticationCode) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QMessageAuthenticationCode6resultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmessageauthenticationcode.h:68
// index:0
// Public static Visibility=Default Availability=Available
// [8] QByteArray hash(const QByteArray &, const QByteArray &, QCryptographicHash::Algorithm)

/*
Returns the authentication code for the message message using the key key and the method method.
*/
impl /*struct*/ QMessageAuthenticationCode {
  pub fn hash_0<RetType, T: QMessageAuthenticationCode_hash_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.hash_0();
    // return 1;
  }
}
pub trait QMessageAuthenticationCode_hash_0<RetType> {
  fn hash_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMessageAuthenticationCode_hash_0<usize> for (usize,usize,i32) {
  fn hash_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN26QMessageAuthenticationCode4hashERK10QByteArrayS2_N18QCryptographicHash9AlgorithmE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
