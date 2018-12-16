

// mod ::core::QThreadStorageData
// package qtcore
// /usr/include/qt/QtCore/qthreadstorage.h
// #include <qthreadstorage.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QThreadStorageData)=4
pub struct QThreadStorageData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QThreadStorageData_ITF interface {
//    QThreadStorageData_PTR() *QThreadStorageData
//}
//func (ptr *QThreadStorageData) QThreadStorageData_PTR() *QThreadStorageData { return ptr }

impl /*struct*/ QThreadStorageData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QThreadStorageData {
    return QThreadStorageData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QThreadStorageData {
//  type Target = QThreadStorageDataBASE;
//
//  fn deref(&self) -> &QThreadStorageDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QThreadStorageDataBASE> for QThreadStorageData {
//  fn as_ref(& self) -> & QThreadStorageDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qthreadstorage.h:54
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QThreadStorageData()

/*

*/
pub fn DeleteQThreadStorageData(this :*mut QThreadStorageData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QThreadStorageDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 4)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qthreadstorage.h:56
// index:0
// Public Visibility=Default Availability=Available
// [8] void ** get() const

/*

*/
impl /*struct*/ QThreadStorageData {
  pub fn get_0<RetType, T: QThreadStorageData_get_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.get_0(self);
    // return 1;
  }
}
pub trait QThreadStorageData_get_0<RetType> {
  fn get_0(self , rsthis: & QThreadStorageData) -> RetType;
}
impl<'a> /*trait*/ QThreadStorageData_get_0<usize> for () {
  fn get_0(self , rsthis: & QThreadStorageData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QThreadStorageData3getEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadstorage.h:57
// index:0
// Public Visibility=Default Availability=Available
// [8] void ** set(void *)

/*

*/
impl /*struct*/ QThreadStorageData {
  pub fn set_0<RetType, T: QThreadStorageData_set_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.set_0(self);
    // return 1;
  }
}
pub trait QThreadStorageData_set_0<RetType> {
  fn set_0(self , rsthis: & QThreadStorageData) -> RetType;
}
impl<'a> /*trait*/ QThreadStorageData_set_0<usize> for (usize) {
  fn set_0(self , rsthis: & QThreadStorageData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QThreadStorageData3setEPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadstorage.h:59
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void finish(void **)

/*

*/
impl /*struct*/ QThreadStorageData {
  pub fn finish_0<RetType, T: QThreadStorageData_finish_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.finish_0();
    // return 1;
  }
}
pub trait QThreadStorageData_finish_0<RetType> {
  fn finish_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThreadStorageData_finish_0<(/*void*/)> for (usize) {
  fn finish_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QThreadStorageData6finishEPPv", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
