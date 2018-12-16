

// mod ::widgets::QWidgetData
// package qtwidgets
// /usr/include/qt/QtWidgets/qwidget.h
// #include <qwidget.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 23
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QWidgetData)=88
pub struct QWidgetData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWidgetData_ITF interface {
//    QWidgetData_PTR() *QWidgetData
//}
//func (ptr *QWidgetData) QWidgetData_PTR() *QWidgetData { return ptr }

impl /*struct*/ QWidgetData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWidgetData {
    return QWidgetData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWidgetData {
//  type Target = QWidgetDataBASE;
//
//  fn deref(&self) -> &QWidgetDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWidgetDataBASE> for QWidgetData {
//  fn as_ref(& self) -> & QWidgetDataBASE {
//    return & self.qbase;
//  }
//}

pub fn DeleteQWidgetData(this :*mut QWidgetData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QWidgetDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
