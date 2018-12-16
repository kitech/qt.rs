

// mod ::core::QBitRef
// package qtcore
// /usr/include/qt/QtCore/qbitarray.h
// #include <qbitarray.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 33
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
#[derive(Default)] // class sizeof(QBitRef)=16
pub struct QBitRef {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBitRef_ITF interface {
//    QBitRef_PTR() *QBitRef
//}
//func (ptr *QBitRef) QBitRef_PTR() *QBitRef { return ptr }

impl /*struct*/ QBitRef {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBitRef {
    return QBitRef{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBitRef {
//  type Target = QBitRefBASE;
//
//  fn deref(&self) -> &QBitRefBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBitRefBASE> for QBitRef {
//  fn as_ref(& self) -> & QBitRefBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbitarray.h:152
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!() const

/*

*/
impl /*struct*/ QBitRef {
  pub fn operator_not_0<RetType, T: QBitRef_operator_not_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_0(self);
    // return 1;
  }
}
pub trait QBitRef_operator_not_0<RetType> {
  fn operator_not_0(self , rsthis: & QBitRef) -> RetType;
}
impl<'a> /*trait*/ QBitRef_operator_not_0<bool> for () {
  fn operator_not_0(self , rsthis: & QBitRef) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QBitRefntEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:153
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QBitRef & operator=(const QBitRef &)

/*

*/
impl /*struct*/ QBitRef {
  pub fn operator_equal_0<RetType, T: QBitRef_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QBitRef_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QBitRef) -> RetType;
}
impl<'a> /*trait*/ QBitRef_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QBitRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitRefaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbitarray.h:154
// index:1
// Public inline Visibility=Default Availability=Available
// [16] QBitRef & operator=(bool)

/*

*/
impl /*struct*/ QBitRef {
  pub fn operator_equal_1<RetType, T: QBitRef_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QBitRef_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QBitRef) -> RetType;
}
impl<'a> /*trait*/ QBitRef_operator_equal_1<usize> for (bool) {
  fn operator_equal_1(self , rsthis: & QBitRef) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QBitRefaSEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQBitRef(this :*mut QBitRef) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN7QBitRefD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
