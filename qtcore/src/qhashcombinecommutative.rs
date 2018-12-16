

// mod ::core::QHashCombineCommutative
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
#[derive(Default)] // class sizeof(QHashCombineCommutative)=1
pub struct QHashCombineCommutative {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHashCombineCommutative_ITF interface {
//    QHashCombineCommutative_PTR() *QHashCombineCommutative
//}
//func (ptr *QHashCombineCommutative) QHashCombineCommutative_PTR() *QHashCombineCommutative { return ptr }

impl /*struct*/ QHashCombineCommutative {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHashCombineCommutative {
    return QHashCombineCommutative{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHashCombineCommutative {
//  type Target = QHashCombineCommutativeBASE;
//
//  fn deref(&self) -> &QHashCombineCommutativeBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHashCombineCommutativeBASE> for QHashCombineCommutative {
//  fn as_ref(& self) -> & QHashCombineCommutativeBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQHashCombineCommutative(this :*mut QHashCombineCommutative) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN23QHashCombineCommutativeD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
