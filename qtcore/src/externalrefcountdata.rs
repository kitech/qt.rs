

// mod ::core::ExternalRefCountData
// package qtcore
// /usr/include/qt/QtCore/qsharedpointer_impl.h
// #include <qsharedpointer_impl.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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
#[derive(Default)] // class sizeof(ExternalRefCountData)=16
pub struct ExternalRefCountData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type ExternalRefCountData_ITF interface {
//    ExternalRefCountData_PTR() *ExternalRefCountData
//}
//func (ptr *ExternalRefCountData) ExternalRefCountData_PTR() *ExternalRefCountData { return ptr }

impl /*struct*/ ExternalRefCountData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> ExternalRefCountData {
    return ExternalRefCountData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for ExternalRefCountData {
//  type Target = ExternalRefCountDataBASE;
//
//  fn deref(&self) -> &ExternalRefCountDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<ExternalRefCountDataBASE> for ExternalRefCountData {
//  fn as_ref(& self) -> & ExternalRefCountDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsharedpointer_impl.h:154
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ExternalRefCountData(Qt::Initialization)

/*

*/
// ExternalRefCountData(Qt::Initialization) ctx.fn_proto_cpp
impl /*struct*/ ExternalRefCountData {
  pub fn ExternalRefCountData_0<T: ExternalRefCountData_ExternalRefCountData_0>(value: T) -> ExternalRefCountData {
    let rsthis = value.ExternalRefCountData_0();
    return rsthis;
    // return 1;
  }
}

pub trait ExternalRefCountData_ExternalRefCountData_0 {
  fn ExternalRefCountData_0(self) -> ExternalRefCountData;
}
// ExternalRefCountData(Qt::Initialization) ctx.fn_proto_cpp
impl<'a> /*trait*/ ExternalRefCountData_ExternalRefCountData_0 for (i32) {
  fn ExternalRefCountData_0(self) -> ExternalRefCountData {
    // unsafe{_ZN15QtSharedPointer20ExternalRefCountDataC2EN2Qt14InitializationE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountDataC2EN2Qt14InitializationE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = ExternalRefCountData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:155
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~ExternalRefCountData()

/*

*/
pub fn DeleteExternalRefCountData(this :*mut ExternalRefCountData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsharedpointer_impl.h:157
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void destroy()

/*

*/
impl /*struct*/ ExternalRefCountData {
  pub fn destroy_0<RetType, T: ExternalRefCountData_destroy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.destroy_0(self);
    // return 1;
  }
}
pub trait ExternalRefCountData_destroy_0<RetType> {
  fn destroy_0(self , rsthis: & ExternalRefCountData) -> RetType;
}
impl<'a> /*trait*/ ExternalRefCountData_destroy_0<(/*void*/)> for () {
  fn destroy_0(self , rsthis: & ExternalRefCountData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountData7destroyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:160
// index:0
// Public static Visibility=Default Availability=Available
// [8] QtSharedPointer::ExternalRefCountData * getAndRef(const QObject *)

/*

*/
impl /*struct*/ ExternalRefCountData {
  pub fn getAndRef_0<RetType, T: ExternalRefCountData_getAndRef_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.getAndRef_0();
    // return 1;
  }
}
pub trait ExternalRefCountData_getAndRef_0<RetType> {
  fn getAndRef_0(self ) -> RetType;
}
impl<'a> /*trait*/ ExternalRefCountData_getAndRef_0<usize> for (usize) {
  fn getAndRef_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountData9getAndRefEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setQObjectShared(const QObject *, bool)

/*

*/
impl /*struct*/ ExternalRefCountData {
  pub fn setQObjectShared_0<RetType, T: ExternalRefCountData_setQObjectShared_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setQObjectShared_0(self);
    // return 1;
  }
}
pub trait ExternalRefCountData_setQObjectShared_0<RetType> {
  fn setQObjectShared_0(self , rsthis: & ExternalRefCountData) -> RetType;
}
impl<'a> /*trait*/ ExternalRefCountData_setQObjectShared_0<(/*void*/)> for (usize,bool) {
  fn setQObjectShared_0(self , rsthis: & ExternalRefCountData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountData16setQObjectSharedEPK7QObjectb", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:162
// index:0
// Public Visibility=Default Availability=Available
// [-2] void checkQObjectShared(const QObject *)

/*

*/
impl /*struct*/ ExternalRefCountData {
  pub fn checkQObjectShared_0<RetType, T: ExternalRefCountData_checkQObjectShared_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.checkQObjectShared_0(self);
    // return 1;
  }
}
pub trait ExternalRefCountData_checkQObjectShared_0<RetType> {
  fn checkQObjectShared_0(self , rsthis: & ExternalRefCountData) -> RetType;
}
impl<'a> /*trait*/ ExternalRefCountData_checkQObjectShared_0<(/*void*/)> for (usize) {
  fn checkQObjectShared_0(self , rsthis: & ExternalRefCountData) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountData18checkQObjectSharedEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:167
// index:0
// Public static inline Visibility=Default Availability=Available
// [-2] void operator delete(void *)

/*

*/
impl /*struct*/ ExternalRefCountData {
  pub fn operator_delete_0<RetType, T: ExternalRefCountData_operator_delete_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.operator_delete_0();
    // return 1;
  }
}
pub trait ExternalRefCountData_operator_delete_0<RetType> {
  fn operator_delete_0(self ) -> RetType;
}
impl<'a> /*trait*/ ExternalRefCountData_operator_delete_0<(/*void*/)> for (usize) {
  fn operator_delete_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountDatadlEPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedpointer_impl.h:168
// index:1
// Public static inline Visibility=Default Availability=Available
// [-2] void operator delete(void *, void *)

/*

*/
impl /*struct*/ ExternalRefCountData {
  pub fn operator_delete_1<RetType, T: ExternalRefCountData_operator_delete_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.operator_delete_1();
    // return 1;
  }
}
pub trait ExternalRefCountData_operator_delete_1<RetType> {
  fn operator_delete_1(self ) -> RetType;
}
impl<'a> /*trait*/ ExternalRefCountData_operator_delete_1<(/*void*/)> for (usize,usize) {
  fn operator_delete_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QtSharedPointer20ExternalRefCountDatadlEPvS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
