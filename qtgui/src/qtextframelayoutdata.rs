

// mod ::gui::QTextFrameLayoutData
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
// extern C begin: 6
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
#[derive(Default)] // class sizeof(QTextFrameLayoutData)=8
pub struct QTextFrameLayoutData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTextFrameLayoutData_ITF interface {
//    QTextFrameLayoutData_PTR() *QTextFrameLayoutData
//}
//func (ptr *QTextFrameLayoutData) QTextFrameLayoutData_PTR() *QTextFrameLayoutData { return ptr }

impl /*struct*/ QTextFrameLayoutData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTextFrameLayoutData {
    return QTextFrameLayoutData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTextFrameLayoutData {
//  type Target = QTextFrameLayoutDataBASE;
//
//  fn deref(&self) -> &QTextFrameLayoutDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTextFrameLayoutDataBASE> for QTextFrameLayoutData {
//  fn as_ref(& self) -> & QTextFrameLayoutDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qtextobject.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTextFrameLayoutData()

/*

*/
pub fn DeleteQTextFrameLayoutData(this :*mut QTextFrameLayoutData) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QTextFrameLayoutDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
