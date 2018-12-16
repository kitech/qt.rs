

// mod ::core::QObjectUserData
// package qtcore
// /usr/include/qt/QtCore/qobject.h
// #include <qobject.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 48
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
#[derive(Default)] // class sizeof(QObjectUserData)=8
pub struct QObjectUserData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QObjectUserData_ITF interface {
//    QObjectUserData_PTR() *QObjectUserData
//}
//func (ptr *QObjectUserData) QObjectUserData_PTR() *QObjectUserData { return ptr }

impl /*struct*/ QObjectUserData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QObjectUserData {
    return QObjectUserData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QObjectUserData {
//  type Target = QObjectUserDataBASE;
//
//  fn deref(&self) -> &QObjectUserDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QObjectUserDataBASE> for QObjectUserData {
//  fn as_ref(& self) -> & QObjectUserDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobject.h:479
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QObjectUserData()

/*

*/
pub fn DeleteQObjectUserData(this :*mut QObjectUserData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QObjectUserDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
