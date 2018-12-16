

// mod ::core::QAtomicInt
// package qtcore
// /usr/include/qt/QtCore/qatomic.h
// #include <qatomic.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QAtomicInt)=4
pub struct QAtomicInt {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAtomicInt_ITF interface {
//    QAtomicInt_PTR() *QAtomicInt
//}
//func (ptr *QAtomicInt) QAtomicInt_PTR() *QAtomicInt { return ptr }

impl /*struct*/ QAtomicInt {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAtomicInt {
    return QAtomicInt{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAtomicInt {
//  type Target = QAtomicIntBASE;
//
//  fn deref(&self) -> &QAtomicIntBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAtomicIntBASE> for QAtomicInt {
//  fn as_ref(& self) -> & QAtomicIntBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qatomic.h:162
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAtomicInt(int)

/*

*/
// QAtomicInt(int) ctx.fn_proto_cpp
impl /*struct*/ QAtomicInt {
  pub fn QAtomicInt_0<T: QAtomicInt_QAtomicInt_0>(value: T) -> QAtomicInt {
    let rsthis = value.QAtomicInt_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAtomicInt_QAtomicInt_0 {
  fn QAtomicInt_0(self) -> QAtomicInt;
}
// QAtomicInt(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAtomicInt_QAtomicInt_0 for (i32) {
  fn QAtomicInt_0(self) -> QAtomicInt {
    // unsafe{_ZN10QAtomicIntC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QAtomicIntC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAtomicInt{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQAtomicInt(this :*mut QAtomicInt) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN10QAtomicIntD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
