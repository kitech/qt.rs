

// mod ::gui::QTextBlockUserData
// package qtgui
// /usr/include/qt/QtGui/qtextobject.h
// #include <qtextobject.h>
// #include <QtGui>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QTextBlockUserData)=8
pub struct QTextBlockUserData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextBlockUserData_ITF interface {
//    QTextBlockUserData_PTR() *QTextBlockUserData
//}
//func (ptr *QTextBlockUserData) QTextBlockUserData_PTR() *QTextBlockUserData { return ptr }

impl /*struct*/ QTextBlockUserData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextBlockUserData {
    return QTextBlockUserData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextBlockUserData {
//  type Target = QTextBlockUserDataBASE;
//
//  fn deref(&self) -> &QTextBlockUserDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextBlockUserDataBASE> for QTextBlockUserData {
//  fn as_ref(& self) -> & QTextBlockUserDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:198
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextBlockUserData()

/*

*/
pub fn DeleteQTextBlockUserData(this :*mut QTextBlockUserData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QTextBlockUserDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
