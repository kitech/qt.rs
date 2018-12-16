

// mod ::core::AbstractDebugStreamFunction
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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(AbstractDebugStreamFunction)=16
pub struct AbstractDebugStreamFunction {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type AbstractDebugStreamFunction_ITF interface {
//    AbstractDebugStreamFunction_PTR() *AbstractDebugStreamFunction
//}
//func (ptr *AbstractDebugStreamFunction) AbstractDebugStreamFunction_PTR() *AbstractDebugStreamFunction { return ptr }

impl /*struct*/ AbstractDebugStreamFunction {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> AbstractDebugStreamFunction {
    return AbstractDebugStreamFunction{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for AbstractDebugStreamFunction {
//  type Target = AbstractDebugStreamFunctionBASE;
//
//  fn deref(&self) -> &AbstractDebugStreamFunctionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<AbstractDebugStreamFunctionBASE> for AbstractDebugStreamFunction {
//  fn as_ref(& self) -> & AbstractDebugStreamFunctionBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteAbstractDebugStreamFunction(this :*mut AbstractDebugStreamFunction) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN27AbstractDebugStreamFunctionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
