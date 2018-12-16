

// mod ::core::QEventLoopLocker
// package qtcore
// /usr/include/qt/QtCore/qeventloop.h
// #include <qeventloop.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 11
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
#[derive(Default)] // class sizeof(QEventLoopLocker)=8
pub struct QEventLoopLocker {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QEventLoopLocker_ITF interface {
//    QEventLoopLocker_PTR() *QEventLoopLocker
//}
//func (ptr *QEventLoopLocker) QEventLoopLocker_PTR() *QEventLoopLocker { return ptr }

impl /*struct*/ QEventLoopLocker {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QEventLoopLocker {
    return QEventLoopLocker{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QEventLoopLocker {
//  type Target = QEventLoopLockerBASE;
//
//  fn deref(&self) -> &QEventLoopLockerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QEventLoopLockerBASE> for QEventLoopLocker {
//  fn as_ref(& self) -> & QEventLoopLockerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qeventloop.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QEventLoopLocker()

/*

*/
// QEventLoopLocker() ctx.fn_proto_cpp
impl /*struct*/ QEventLoopLocker {
  pub fn QEventLoopLocker_0<T: QEventLoopLocker_QEventLoopLocker_0>(value: T) -> QEventLoopLocker {
    let rsthis = value.QEventLoopLocker_0();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_QEventLoopLocker_0 {
  fn QEventLoopLocker_0(self) -> QEventLoopLocker;
}
// QEventLoopLocker() ctx.fn_proto_cpp
impl<'a> /*trait*/ QEventLoopLocker_QEventLoopLocker_0 for () {
  fn QEventLoopLocker_0(self) -> QEventLoopLocker {
    // unsafe{_ZN16QEventLoopLockerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QEventLoopLockerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEventLoopLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:94
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QEventLoopLocker(QEventLoop *)

/*

*/
// QEventLoopLocker(QEventLoop *) ctx.fn_proto_cpp
impl /*struct*/ QEventLoopLocker {
  pub fn QEventLoopLocker_1<T: QEventLoopLocker_QEventLoopLocker_1>(value: T) -> QEventLoopLocker {
    let rsthis = value.QEventLoopLocker_1();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_QEventLoopLocker_1 {
  fn QEventLoopLocker_1(self) -> QEventLoopLocker;
}
// QEventLoopLocker(QEventLoop *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEventLoopLocker_QEventLoopLocker_1 for (usize) {
  fn QEventLoopLocker_1(self) -> QEventLoopLocker {
    // unsafe{_ZN16QEventLoopLockerC2EP10QEventLoop()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QEventLoopLockerC2EP10QEventLoop", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEventLoopLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:95
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QEventLoopLocker(QThread *)

/*

*/
// QEventLoopLocker(QThread *) ctx.fn_proto_cpp
impl /*struct*/ QEventLoopLocker {
  pub fn QEventLoopLocker_2<T: QEventLoopLocker_QEventLoopLocker_2>(value: T) -> QEventLoopLocker {
    let rsthis = value.QEventLoopLocker_2();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_QEventLoopLocker_2 {
  fn QEventLoopLocker_2(self) -> QEventLoopLocker;
}
// QEventLoopLocker(QThread *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEventLoopLocker_QEventLoopLocker_2 for (usize) {
  fn QEventLoopLocker_2(self) -> QEventLoopLocker {
    // unsafe{_ZN16QEventLoopLockerC2EP7QThread()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QEventLoopLockerC2EP7QThread", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEventLoopLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QEventLoopLocker()

/*

*/
pub fn DeleteQEventLoopLocker(this :*mut QEventLoopLocker) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QEventLoopLockerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
