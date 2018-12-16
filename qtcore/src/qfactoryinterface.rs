

// mod ::core::QFactoryInterface
// package qtcore
// /usr/include/qt/QtCore/qfactoryinterface.h
// #include <qfactoryinterface.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QFactoryInterface)=8
pub struct QFactoryInterface {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFactoryInterface_ITF interface {
//    QFactoryInterface_PTR() *QFactoryInterface
//}
//func (ptr *QFactoryInterface) QFactoryInterface_PTR() *QFactoryInterface { return ptr }

impl /*struct*/ QFactoryInterface {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFactoryInterface {
    return QFactoryInterface{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFactoryInterface {
//  type Target = QFactoryInterfaceBASE;
//
//  fn deref(&self) -> &QFactoryInterfaceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFactoryInterfaceBASE> for QFactoryInterface {
//  fn as_ref(& self) -> & QFactoryInterfaceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfactoryinterface.h:51
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFactoryInterface()

/*

*/
pub fn DeleteQFactoryInterface(this :*mut QFactoryInterface) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QFactoryInterfaceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfactoryinterface.h:52
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [8] QStringList keys() const

/*

*/
impl /*struct*/ QFactoryInterface {
  pub fn keys_0<RetType, T: QFactoryInterface_keys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.keys_0(self);
    // return 1;
  }
}
pub trait QFactoryInterface_keys_0<RetType> {
  fn keys_0(self , rsthis: & QFactoryInterface) -> RetType;
}
impl<'a> /*trait*/ QFactoryInterface_keys_0<usize> for () {
  fn keys_0(self , rsthis: & QFactoryInterface) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QFactoryInterface4keysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
