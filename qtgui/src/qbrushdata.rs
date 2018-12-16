

// mod ::gui::QBrushData
// package qtgui
// /usr/include/qt/QtGui/qbrush.h
// #include <qbrush.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QBrushData)=112
pub struct QBrushData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBrushData_ITF interface {
//    QBrushData_PTR() *QBrushData
//}
//func (ptr *QBrushData) QBrushData_PTR() *QBrushData { return ptr }

impl /*struct*/ QBrushData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBrushData {
    return QBrushData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBrushData {
//  type Target = QBrushDataBASE;
//
//  fn deref(&self) -> &QBrushDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBrushDataBASE> for QBrushData {
//  fn as_ref(& self) -> & QBrushDataBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQBrushData(this :*mut QBrushData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QBrushDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
