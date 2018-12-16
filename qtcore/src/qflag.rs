

// mod ::core::QFlag
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QFlag)=4
pub struct QFlag {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFlag_ITF interface {
//    QFlag_PTR() *QFlag
//}
//func (ptr *QFlag) QFlag_PTR() *QFlag { return ptr }

impl /*struct*/ QFlag {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFlag {
    return QFlag{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFlag {
//  type Target = QFlagBASE;
//
//  fn deref(&self) -> &QFlagBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFlagBASE> for QFlag {
//  fn as_ref(& self) -> & QFlagBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qflags.h:57
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QFlag(int)

/*

*/
// QFlag(int) ctx.fn_proto_cpp
impl /*struct*/ QFlag {
  pub fn QFlag_0<T: QFlag_QFlag_0>(value: T) -> QFlag {
    let rsthis = value.QFlag_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFlag_QFlag_0 {
  fn QFlag_0(self) -> QFlag;
}
// QFlag(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFlag_QFlag_0 for (i32) {
  fn QFlag_0(self) -> QFlag {
    // unsafe{_ZN5QFlagC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFlagC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qflags.h:68
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QFlag(uint)

/*

*/
// QFlag(uint) ctx.fn_proto_cpp
impl /*struct*/ QFlag {
  pub fn QFlag_1<T: QFlag_QFlag_1>(value: T) -> QFlag {
    let rsthis = value.QFlag_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFlag_QFlag_1 {
  fn QFlag_1(self) -> QFlag;
}
// QFlag(uint) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFlag_QFlag_1 for (u32) {
  fn QFlag_1(self) -> QFlag {
    // unsafe{_ZN5QFlagC2Ej()};
    let arg0 = (&self) as *const u32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFlagC2Ej", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qflags.h:69
// index:2
// Public inline Visibility=Default Availability=Available
// [-2] void QFlag(short)

/*

*/
// QFlag(short) ctx.fn_proto_cpp
impl /*struct*/ QFlag {
  pub fn QFlag_2<T: QFlag_QFlag_2>(value: T) -> QFlag {
    let rsthis = value.QFlag_2();
    return rsthis;
    // return 1;
  }
}

pub trait QFlag_QFlag_2 {
  fn QFlag_2(self) -> QFlag;
}
// QFlag(short) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFlag_QFlag_2 for (i16) {
  fn QFlag_2(self) -> QFlag {
    // unsafe{_ZN5QFlagC2Es()};
    let arg0 = (&self) as *const i16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFlagC2Es", 1,qtrt::FFITY_SINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qflags.h:70
// index:3
// Public inline Visibility=Default Availability=Available
// [-2] void QFlag(ushort)

/*

*/
// QFlag(ushort) ctx.fn_proto_cpp
impl /*struct*/ QFlag {
  pub fn QFlag_3<T: QFlag_QFlag_3>(value: T) -> QFlag {
    let rsthis = value.QFlag_3();
    return rsthis;
    // return 1;
  }
}

pub trait QFlag_QFlag_3 {
  fn QFlag_3(self) -> QFlag;
}
// QFlag(ushort) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFlag_QFlag_3 for (u16) {
  fn QFlag_3(self) -> QFlag {
    // unsafe{_ZN5QFlagC2Et()};
    let arg0 = (&self) as *const u16 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN5QFlagC2Et", 1,qtrt::FFITY_UINT16,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFlag{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}


pub fn DeleteQFlag(this :*mut QFlag) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN5QFlagD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
