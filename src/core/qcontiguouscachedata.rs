

// mod ::core::QContiguousCacheData
// package qtcore
// /usr/include/qt/QtCore/qcontiguouscache.h
// #include <qcontiguouscache.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 27
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
#[derive(Default)] // class sizeof(QContiguousCacheData)=24
pub struct QContiguousCacheData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QContiguousCacheData_ITF interface {
//    QContiguousCacheData_PTR() *QContiguousCacheData
//}
//func (ptr *QContiguousCacheData) QContiguousCacheData_PTR() *QContiguousCacheData { return ptr }

impl /*struct*/ QContiguousCacheData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QContiguousCacheData {
    return QContiguousCacheData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QContiguousCacheData {
//  type Target = QContiguousCacheDataBASE;
//
//  fn deref(&self) -> &QContiguousCacheDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QContiguousCacheDataBASE> for QContiguousCacheData {
//  fn as_ref(& self) -> & QContiguousCacheDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcontiguouscache.h:67
// index:0
// Public static Visibility=Default Availability=Available
// [8] QContiguousCacheData * allocateData(int, int)

/*

*/
impl /*struct*/ QContiguousCacheData {
  pub fn allocateData_0<RetType, T: QContiguousCacheData_allocateData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.allocateData_0();
    // return 1;
  }
}
pub trait QContiguousCacheData_allocateData_0<RetType> {
  fn allocateData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QContiguousCacheData_allocateData_0<usize> for (i32,i32) {
  fn allocateData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QContiguousCacheData12allocateDataEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQContiguousCacheData(this :*mut QContiguousCacheData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QContiguousCacheDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
