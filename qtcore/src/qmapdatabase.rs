

// mod ::core::QMapDataBase
// package qtcore
// /usr/include/qt/QtCore/qmap.h
// #include <qmap.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QMapDataBase)=40
pub struct QMapDataBase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMapDataBase_ITF interface {
//    QMapDataBase_PTR() *QMapDataBase
//}
//func (ptr *QMapDataBase) QMapDataBase_PTR() *QMapDataBase { return ptr }

impl /*struct*/ QMapDataBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMapDataBase {
    return QMapDataBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMapDataBase {
//  type Target = QMapDataBaseBASE;
//
//  fn deref(&self) -> &QMapDataBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMapDataBaseBASE> for QMapDataBase {
//  fn as_ref(& self) -> & QMapDataBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmap.h:194
// index:0
// Public Visibility=Default Availability=Available
// [-2] void recalcMostLeftNode()

/*

*/
impl /*struct*/ QMapDataBase {
  pub fn recalcMostLeftNode_0<RetType, T: QMapDataBase_recalcMostLeftNode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.recalcMostLeftNode_0(self);
    // return 1;
  }
}
pub trait QMapDataBase_recalcMostLeftNode_0<RetType> {
  fn recalcMostLeftNode_0(self , rsthis: & QMapDataBase) -> RetType;
}
impl<'a> /*trait*/ QMapDataBase_recalcMostLeftNode_0<(/*void*/)> for () {
  fn recalcMostLeftNode_0(self , rsthis: & QMapDataBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QMapDataBase18recalcMostLeftNodeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmap.h:201
// index:0
// Public static Visibility=Default Availability=Available
// [8] QMapDataBase * createData()

/*

*/
impl /*struct*/ QMapDataBase {
  pub fn createData_0<RetType, T: QMapDataBase_createData_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.createData_0();
    // return 1;
  }
}
pub trait QMapDataBase_createData_0<RetType> {
  fn createData_0(self ) -> RetType;
}
impl<'a> /*trait*/ QMapDataBase_createData_0<usize> for () {
  fn createData_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QMapDataBase10createDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


pub fn DeleteQMapDataBase(this :*mut QMapDataBase) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN12QMapDataBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
