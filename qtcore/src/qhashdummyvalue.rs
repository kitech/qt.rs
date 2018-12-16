

// mod ::core::QHashDummyValue
// package qtcore
// /usr/include/qt/QtCore/qhash.h
// #include <qhash.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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
#[derive(Default)] // class sizeof(QHashDummyValue)=1
pub struct QHashDummyValue {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHashDummyValue_ITF interface {
//    QHashDummyValue_PTR() *QHashDummyValue
//}
//func (ptr *QHashDummyValue) QHashDummyValue_PTR() *QHashDummyValue { return ptr }

impl /*struct*/ QHashDummyValue {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHashDummyValue {
    return QHashDummyValue{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHashDummyValue {
//  type Target = QHashDummyValueBASE;
//
//  fn deref(&self) -> &QHashDummyValueBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHashDummyValueBASE> for QHashDummyValue {
//  fn as_ref(& self) -> & QHashDummyValueBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQHashDummyValue(this :*mut QHashDummyValue) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QHashDummyValueD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
