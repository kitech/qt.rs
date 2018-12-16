

// mod ::core::QLinkedListData
// package qtcore
// /usr/include/qt/QtCore/qlinkedlist.h
// #include <qlinkedlist.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 36
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
#[derive(Default)] // class sizeof(QLinkedListData)=32
pub struct QLinkedListData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLinkedListData_ITF interface {
//    QLinkedListData_PTR() *QLinkedListData
//}
//func (ptr *QLinkedListData) QLinkedListData_PTR() *QLinkedListData { return ptr }

impl /*struct*/ QLinkedListData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLinkedListData {
    return QLinkedListData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLinkedListData {
//  type Target = QLinkedListDataBASE;
//
//  fn deref(&self) -> &QLinkedListDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLinkedListDataBASE> for QLinkedListData {
//  fn as_ref(& self) -> & QLinkedListDataBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQLinkedListData(this :*mut QLinkedListData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN15QLinkedListDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
