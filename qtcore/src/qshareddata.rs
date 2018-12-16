

// mod ::core::QSharedData
// package qtcore
// /usr/include/qt/QtCore/qshareddata.h
// #include <qshareddata.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 2
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
#[derive(Default)] // class sizeof(QSharedData)=4
pub struct QSharedData {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSharedData_ITF interface {
//    QSharedData_PTR() *QSharedData
//}
//func (ptr *QSharedData) QSharedData_PTR() *QSharedData { return ptr }

impl /*struct*/ QSharedData {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSharedData {
    return QSharedData{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSharedData {
//  type Target = QSharedDataBASE;
//
//  fn deref(&self) -> &QSharedDataBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSharedDataBASE> for QSharedData {
//  fn as_ref(& self) -> & QSharedDataBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qshareddata.h:60
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSharedData()

/*
Constructs a QSharedData object with a reference count of 0.
*/
// QSharedData() ctx.fn_proto_cpp
impl /*struct*/ QSharedData {
  pub fn QSharedData_0<T: QSharedData_QSharedData_0>(value: T) -> QSharedData {
    let rsthis = value.QSharedData_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedData_QSharedData_0 {
  fn QSharedData_0(self) -> QSharedData;
}
// QSharedData() ctx.fn_proto_cpp
impl<'a> /*trait*/ QSharedData_QSharedData_0 for () {
  fn QSharedData_0(self) -> QSharedData {
    // unsafe{_ZN11QSharedDataC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QSharedDataC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSharedData{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQSharedData(this :*mut QSharedData) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QSharedDataD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
