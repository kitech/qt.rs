

// mod ::core::QIncompatibleFlag
// package qtcore
// /usr/include/qt/QtCore/qflags.h
// #include <qflags.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QIncompatibleFlag)=4
pub struct QIncompatibleFlag {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QIncompatibleFlag_ITF interface {
//    QIncompatibleFlag_PTR() *QIncompatibleFlag
//}
//func (ptr *QIncompatibleFlag) QIncompatibleFlag_PTR() *QIncompatibleFlag { return ptr }

impl /*struct*/ QIncompatibleFlag {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QIncompatibleFlag {
    return QIncompatibleFlag{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QIncompatibleFlag {
//  type Target = QIncompatibleFlagBASE;
//
//  fn deref(&self) -> &QIncompatibleFlagBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QIncompatibleFlagBASE> for QIncompatibleFlag {
//  fn as_ref(& self) -> & QIncompatibleFlagBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qflags.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QIncompatibleFlag(int)

/*

*/
// QIncompatibleFlag(int) ctx.fn_proto_cpp
impl /*struct*/ QIncompatibleFlag {
  pub fn QIncompatibleFlag_0<T: QIncompatibleFlag_QIncompatibleFlag_0>(value: T) -> QIncompatibleFlag {
    let rsthis = value.QIncompatibleFlag_0();
    return rsthis;
    // return 1;
  }
}

pub trait QIncompatibleFlag_QIncompatibleFlag_0 {
  fn QIncompatibleFlag_0(self) -> QIncompatibleFlag;
}
// QIncompatibleFlag(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QIncompatibleFlag_QIncompatibleFlag_0 for (i32) {
  fn QIncompatibleFlag_0(self) -> QIncompatibleFlag {
    // unsafe{_ZN17QIncompatibleFlagC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QIncompatibleFlagC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QIncompatibleFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQIncompatibleFlag(this :*mut QIncompatibleFlag) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN17QIncompatibleFlagD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
