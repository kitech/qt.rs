

// mod ::core::QJsonValuePtr
// package qtcore
// /usr/include/qt/QtCore/qjsonvalue.h
// #include <qjsonvalue.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 25
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
#[derive(Default)] // class sizeof(QJsonValuePtr)=24
pub struct QJsonValuePtr {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonValuePtr_ITF interface {
//    QJsonValuePtr_PTR() *QJsonValuePtr
//}
//func (ptr *QJsonValuePtr) QJsonValuePtr_PTR() *QJsonValuePtr { return ptr }

impl /*struct*/ QJsonValuePtr {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonValuePtr {
    return QJsonValuePtr{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonValuePtr {
//  type Target = QJsonValuePtrBASE;
//
//  fn deref(&self) -> &QJsonValuePtrBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonValuePtrBASE> for QJsonValuePtr {
//  fn as_ref(& self) -> & QJsonValuePtrBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsonvalue.h:226
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QJsonValuePtr(const QJsonValue &)

/*

*/
// QJsonValuePtr(const QJsonValue &) ctx.fn_proto_cpp
impl /*struct*/ QJsonValuePtr {
  pub fn QJsonValuePtr_0<T: QJsonValuePtr_QJsonValuePtr_0>(value: T) -> QJsonValuePtr {
    let rsthis = value.QJsonValuePtr_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValuePtr_QJsonValuePtr_0 {
  fn QJsonValuePtr_0(self) -> QJsonValuePtr;
}
// QJsonValuePtr(const QJsonValue &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValuePtr_QJsonValuePtr_0 for (usize) {
  fn QJsonValuePtr_0(self) -> QJsonValuePtr {
    // unsafe{_ZN13QJsonValuePtrC2ERK10QJsonValue()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QJsonValuePtrC2ERK10QJsonValue", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValuePtr{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:229
// index:0
// Public inline Visibility=Default Availability=Available
// [24] QJsonValue & operator*()

/*

*/
impl /*struct*/ QJsonValuePtr {
  pub fn operator_mul_0<RetType, T: QJsonValuePtr_operator_mul_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_0(self);
    // return 1;
  }
}
pub trait QJsonValuePtr_operator_mul_0<RetType> {
  fn operator_mul_0(self , rsthis: & QJsonValuePtr) -> RetType;
}
impl<'a> /*trait*/ QJsonValuePtr_operator_mul_0<usize> for () {
  fn operator_mul_0(self , rsthis: & QJsonValuePtr) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonValuePtrdeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:230
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QJsonValue * operator->()

/*

*/
impl /*struct*/ QJsonValuePtr {
  pub fn operator_minus_greater_0<RetType, T: QJsonValuePtr_operator_minus_greater_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_greater_0(self);
    // return 1;
  }
}
pub trait QJsonValuePtr_operator_minus_greater_0<RetType> {
  fn operator_minus_greater_0(self , rsthis: & QJsonValuePtr) -> RetType;
}
impl<'a> /*trait*/ QJsonValuePtr_operator_minus_greater_0<usize> for () {
  fn operator_minus_greater_0(self , rsthis: & QJsonValuePtr) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QJsonValuePtrptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQJsonValuePtr(this :*mut QJsonValuePtr) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QJsonValuePtrD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
