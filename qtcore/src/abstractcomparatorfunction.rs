

// mod ::core::AbstractComparatorFunction
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
#[derive(Default)] // class sizeof(AbstractComparatorFunction)=24
pub struct AbstractComparatorFunction {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type AbstractComparatorFunction_ITF interface {
//    AbstractComparatorFunction_PTR() *AbstractComparatorFunction
//}
//func (ptr *AbstractComparatorFunction) AbstractComparatorFunction_PTR() *AbstractComparatorFunction { return ptr }

impl /*struct*/ AbstractComparatorFunction {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> AbstractComparatorFunction {
    return AbstractComparatorFunction{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for AbstractComparatorFunction {
//  type Target = AbstractComparatorFunctionBASE;
//
//  fn deref(&self) -> &AbstractComparatorFunctionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<AbstractComparatorFunctionBASE> for AbstractComparatorFunction {
//  fn as_ref(& self) -> & AbstractComparatorFunctionBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteAbstractComparatorFunction(this :*mut AbstractComparatorFunction) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN26AbstractComparatorFunctionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
