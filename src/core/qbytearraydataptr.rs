

// mod ::core::QByteArrayDataPtr
// package qtcore
// /usr/include/qt/QtCore/qbytearray.h
// #include <qbytearray.h>
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
#[derive(Default)] // class sizeof(QByteArrayDataPtr)=8
pub struct QByteArrayDataPtr {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QByteArrayDataPtr_ITF interface {
//    QByteArrayDataPtr_PTR() *QByteArrayDataPtr
//}
//func (ptr *QByteArrayDataPtr) QByteArrayDataPtr_PTR() *QByteArrayDataPtr { return ptr }

impl /*struct*/ QByteArrayDataPtr {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QByteArrayDataPtr {
    return QByteArrayDataPtr{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QByteArrayDataPtr {
//  type Target = QByteArrayDataPtrBASE;
//
//  fn deref(&self) -> &QByteArrayDataPtrBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QByteArrayDataPtrBASE> for QByteArrayDataPtr {
//  fn as_ref(& self) -> & QByteArrayDataPtrBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQByteArrayDataPtr(this :*mut QByteArrayDataPtr) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN17QByteArrayDataPtrD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
