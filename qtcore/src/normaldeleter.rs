

// mod ::core::NormalDeleter
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
#[derive(Default)] // class sizeof(NormalDeleter)=1
pub struct NormalDeleter {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type NormalDeleter_ITF interface {
//    NormalDeleter_PTR() *NormalDeleter
//}
//func (ptr *NormalDeleter) NormalDeleter_PTR() *NormalDeleter { return ptr }

impl /*struct*/ NormalDeleter {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> NormalDeleter {
    return NormalDeleter{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for NormalDeleter {
//  type Target = NormalDeleterBASE;
//
//  fn deref(&self) -> &NormalDeleterBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<NormalDeleterBASE> for NormalDeleter {
//  fn as_ref(& self) -> & NormalDeleterBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteNormalDeleter(this :*mut NormalDeleter) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13NormalDeleterD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
