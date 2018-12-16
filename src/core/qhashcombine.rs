

// mod ::core::QHashCombine
// package qtcore
// /usr/include/qt/QtCore/qhashfunctions.h
// #include <qhashfunctions.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 93
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
#[derive(Default)] // class sizeof(QHashCombine)=1
pub struct QHashCombine {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHashCombine_ITF interface {
//    QHashCombine_PTR() *QHashCombine
//}
//func (ptr *QHashCombine) QHashCombine_PTR() *QHashCombine { return ptr }

impl /*struct*/ QHashCombine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHashCombine {
    return QHashCombine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHashCombine {
//  type Target = QHashCombineBASE;
//
//  fn deref(&self) -> &QHashCombineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHashCombineBASE> for QHashCombine {
//  fn as_ref(& self) -> & QHashCombineBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQHashCombine(this :*mut QHashCombine) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN12QHashCombineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
