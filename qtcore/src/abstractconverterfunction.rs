

// mod ::core::AbstractConverterFunction
// package qtcore
// /usr/include/qt/QtCore/qmetatype.h
// #include <qmetatype.h>
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
#[derive(Default)] // class sizeof(AbstractConverterFunction)=8
pub struct AbstractConverterFunction {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type AbstractConverterFunction_ITF interface {
//    AbstractConverterFunction_PTR() *AbstractConverterFunction
//}
//func (ptr *AbstractConverterFunction) AbstractConverterFunction_PTR() *AbstractConverterFunction { return ptr }

impl /*struct*/ AbstractConverterFunction {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> AbstractConverterFunction {
    return AbstractConverterFunction{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for AbstractConverterFunction {
//  type Target = AbstractConverterFunctionBASE;
//
//  fn deref(&self) -> &AbstractConverterFunctionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<AbstractConverterFunctionBASE> for AbstractConverterFunction {
//  fn as_ref(& self) -> & AbstractConverterFunctionBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteAbstractConverterFunction(this :*mut AbstractConverterFunction) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN25AbstractConverterFunctionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
