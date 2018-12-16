

// mod ::core::QStringDataPtr
// package qtcore
// /usr/include/qt/QtCore/qstringliteral.h
// #include <qstringliteral.h>
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
#[derive(Default)] // class sizeof(QStringDataPtr)=8
pub struct QStringDataPtr {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStringDataPtr_ITF interface {
//    QStringDataPtr_PTR() *QStringDataPtr
//}
//func (ptr *QStringDataPtr) QStringDataPtr_PTR() *QStringDataPtr { return ptr }

impl /*struct*/ QStringDataPtr {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStringDataPtr {
    return QStringDataPtr{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStringDataPtr {
//  type Target = QStringDataPtrBASE;
//
//  fn deref(&self) -> &QStringDataPtrBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStringDataPtrBASE> for QStringDataPtr {
//  fn as_ref(& self) -> & QStringDataPtrBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQStringDataPtr(this :*mut QStringDataPtr) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QStringDataPtrD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
