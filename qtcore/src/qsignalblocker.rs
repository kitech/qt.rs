

// mod ::core::QSignalBlocker
// package qtcore
// /usr/include/qt/QtCore/qobject.h
// #include <qobject.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
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
#[derive(Default)] // class sizeof(QSignalBlocker)=16
pub struct QSignalBlocker {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSignalBlocker_ITF interface {
//    QSignalBlocker_PTR() *QSignalBlocker
//}
//func (ptr *QSignalBlocker) QSignalBlocker_PTR() *QSignalBlocker { return ptr }

impl /*struct*/ QSignalBlocker {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSignalBlocker {
    return QSignalBlocker{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSignalBlocker {
//  type Target = QSignalBlockerBASE;
//
//  fn deref(&self) -> &QSignalBlockerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSignalBlockerBASE> for QSignalBlocker {
//  fn as_ref(& self) -> & QSignalBlockerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qobject.h:548
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QSignalBlocker(QObject *)

/*

*/
// QSignalBlocker(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSignalBlocker {
  pub fn QSignalBlocker_0<T: QSignalBlocker_QSignalBlocker_0>(value: T) -> QSignalBlocker {
    let rsthis = value.QSignalBlocker_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalBlocker_QSignalBlocker_0 {
  fn QSignalBlocker_0(self) -> QSignalBlocker;
}
// QSignalBlocker(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSignalBlocker_QSignalBlocker_0 for (usize) {
  fn QSignalBlocker_0(self) -> QSignalBlocker {
    // unsafe{_ZN14QSignalBlockerC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QSignalBlockerC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSignalBlocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:549
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QSignalBlocker(QObject &)

/*

*/
// QSignalBlocker(QObject &) ctx.fn_proto_cpp
impl /*struct*/ QSignalBlocker {
  pub fn QSignalBlocker_1<T: QSignalBlocker_QSignalBlocker_1>(value: T) -> QSignalBlocker {
    let rsthis = value.QSignalBlocker_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalBlocker_QSignalBlocker_1 {
  fn QSignalBlocker_1(self) -> QSignalBlocker;
}
// QSignalBlocker(QObject &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSignalBlocker_QSignalBlocker_1 for (usize) {
  fn QSignalBlocker_1(self) -> QSignalBlocker {
    // unsafe{_ZN14QSignalBlockerC2ER7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QSignalBlockerC2ER7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSignalBlocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:550
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QSignalBlocker()

/*

*/
pub fn DeleteQSignalBlocker(this :*mut QSignalBlocker) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QSignalBlockerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qobject.h:557
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void reblock()

/*

*/
impl /*struct*/ QSignalBlocker {
  pub fn reblock_0<RetType, T: QSignalBlocker_reblock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reblock_0(self);
    // return 1;
  }
}
pub trait QSignalBlocker_reblock_0<RetType> {
  fn reblock_0(self , rsthis: & QSignalBlocker) -> RetType;
}
impl<'a> /*trait*/ QSignalBlocker_reblock_0<(/*void*/)> for () {
  fn reblock_0(self , rsthis: & QSignalBlocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QSignalBlocker7reblockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qobject.h:558
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void unblock()

/*

*/
impl /*struct*/ QSignalBlocker {
  pub fn unblock_0<RetType, T: QSignalBlocker_unblock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unblock_0(self);
    // return 1;
  }
}
pub trait QSignalBlocker_unblock_0<RetType> {
  fn unblock_0(self , rsthis: & QSignalBlocker) -> RetType;
}
impl<'a> /*trait*/ QSignalBlocker_unblock_0<(/*void*/)> for () {
  fn unblock_0(self , rsthis: & QSignalBlocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QSignalBlocker7unblockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
