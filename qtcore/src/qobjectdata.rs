

// mod ::core::QObjectData
// package qtcore
// /usr/include/qt/QtCore/qobject.h
// #include <qobject.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
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
#[derive(Default)] // class sizeof(QObjectData)=48
pub struct QObjectData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QObjectData_ITF interface {
//    QObjectData_PTR() *QObjectData
//}
//func (ptr *QObjectData) QObjectData_PTR() *QObjectData { return ptr }

impl /*struct*/ QObjectData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QObjectData {
    return QObjectData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QObjectData {
//  type Target = QObjectDataBASE;
//
//  fn deref(&self) -> &QObjectDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QObjectDataBASE> for QObjectData {
//  fn as_ref(& self) -> & QObjectDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobject.h:97
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void ~QObjectData()

/*

*/
pub fn DeleteQObjectData(this :*mut QObjectData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QObjectDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qobject.h:113
// index:0
// Public Visibility=Default Availability=Available
// [8] QMetaObject * dynamicMetaObject() const

/*

*/
impl /*struct*/ QObjectData {
  pub fn dynamicMetaObject_0<RetType, T: QObjectData_dynamicMetaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dynamicMetaObject_0(self);
    // return 1;
  }
}
pub trait QObjectData_dynamicMetaObject_0<RetType> {
  fn dynamicMetaObject_0(self , rsthis: & QObjectData) -> RetType;
}
impl<'a> /*trait*/ QObjectData_dynamicMetaObject_0<usize> for () {
  fn dynamicMetaObject_0(self , rsthis: & QObjectData) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QObjectData17dynamicMetaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
