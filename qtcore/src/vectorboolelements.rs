

// mod ::core::VectorBoolElements
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
#[derive(Default)] // class sizeof(VectorBoolElements)=1
pub struct VectorBoolElements {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type VectorBoolElements_ITF interface {
//    VectorBoolElements_PTR() *VectorBoolElements
//}
//func (ptr *VectorBoolElements) VectorBoolElements_PTR() *VectorBoolElements { return ptr }

impl /*struct*/ VectorBoolElements {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> VectorBoolElements {
    return VectorBoolElements{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for VectorBoolElements {
//  type Target = VectorBoolElementsBASE;
//
//  fn deref(&self) -> &VectorBoolElementsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<VectorBoolElementsBASE> for VectorBoolElements {
//  fn as_ref(& self) -> & VectorBoolElementsBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteVectorBoolElements(this :*mut VectorBoolElements) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18VectorBoolElementsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
