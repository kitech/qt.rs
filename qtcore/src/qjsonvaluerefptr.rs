

// mod ::core::QJsonValueRefPtr
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QJsonValueRefPtr)=16
pub struct QJsonValueRefPtr {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QJsonValueRefPtr_ITF interface {
//    QJsonValueRefPtr_PTR() *QJsonValueRefPtr
//}
//func (ptr *QJsonValueRefPtr) QJsonValueRefPtr_PTR() *QJsonValueRefPtr { return ptr }

impl /*struct*/ QJsonValueRefPtr {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QJsonValueRefPtr {
    return QJsonValueRefPtr{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QJsonValueRefPtr {
//  type Target = QJsonValueRefPtrBASE;
//
//  fn deref(&self) -> &QJsonValueRefPtrBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QJsonValueRefPtrBASE> for QJsonValueRefPtr {
//  fn as_ref(& self) -> & QJsonValueRefPtrBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qjsonvalue.h:237
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QJsonValueRefPtr(QJsonArray *, int)

/*

*/
// QJsonValueRefPtr(QJsonArray *, int) ctx.fn_proto_cpp
impl /*struct*/ QJsonValueRefPtr {
  pub fn QJsonValueRefPtr_0<T: QJsonValueRefPtr_QJsonValueRefPtr_0>(value: T) -> QJsonValueRefPtr {
    let rsthis = value.QJsonValueRefPtr_0();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValueRefPtr_QJsonValueRefPtr_0 {
  fn QJsonValueRefPtr_0(self) -> QJsonValueRefPtr;
}
// QJsonValueRefPtr(QJsonArray *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValueRefPtr_QJsonValueRefPtr_0 for (usize,i32) {
  fn QJsonValueRefPtr_0(self) -> QJsonValueRefPtr {
    // unsafe{_ZN16QJsonValueRefPtrC2EP10QJsonArrayi()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QJsonValueRefPtrC2EP10QJsonArrayi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValueRefPtr{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:239
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QJsonValueRefPtr(QJsonObject *, int)

/*

*/
// QJsonValueRefPtr(QJsonObject *, int) ctx.fn_proto_cpp
impl /*struct*/ QJsonValueRefPtr {
  pub fn QJsonValueRefPtr_1<T: QJsonValueRefPtr_QJsonValueRefPtr_1>(value: T) -> QJsonValueRefPtr {
    let rsthis = value.QJsonValueRefPtr_1();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonValueRefPtr_QJsonValueRefPtr_1 {
  fn QJsonValueRefPtr_1(self) -> QJsonValueRefPtr;
}
// QJsonValueRefPtr(QJsonObject *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QJsonValueRefPtr_QJsonValueRefPtr_1 for (usize,i32) {
  fn QJsonValueRefPtr_1(self) -> QJsonValueRefPtr {
    // unsafe{_ZN16QJsonValueRefPtrC2EP11QJsonObjecti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QJsonValueRefPtrC2EP11QJsonObjecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QJsonValueRefPtr{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:242
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QJsonValueRef & operator*()

/*

*/
impl /*struct*/ QJsonValueRefPtr {
  pub fn operator_mul_0<RetType, T: QJsonValueRefPtr_operator_mul_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_mul_0(self);
    // return 1;
  }
}
pub trait QJsonValueRefPtr_operator_mul_0<RetType> {
  fn operator_mul_0(self , rsthis: & QJsonValueRefPtr) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRefPtr_operator_mul_0<usize> for () {
  fn operator_mul_0(self , rsthis: & QJsonValueRefPtr) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QJsonValueRefPtrdeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qjsonvalue.h:243
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QJsonValueRef * operator->()

/*

*/
impl /*struct*/ QJsonValueRefPtr {
  pub fn operator_minus_greater_0<RetType, T: QJsonValueRefPtr_operator_minus_greater_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_greater_0(self);
    // return 1;
  }
}
pub trait QJsonValueRefPtr_operator_minus_greater_0<RetType> {
  fn operator_minus_greater_0(self , rsthis: & QJsonValueRefPtr) -> RetType;
}
impl<'a> /*trait*/ QJsonValueRefPtr_operator_minus_greater_0<usize> for () {
  fn operator_minus_greater_0(self , rsthis: & QJsonValueRefPtr) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QJsonValueRefPtrptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQJsonValueRefPtr(this :*mut QJsonValueRefPtr) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN16QJsonValueRefPtrD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
