

// mod ::gui::QAccessibleTextCursorEvent
// package qtgui
// /usr/include/qt/QtGui/qaccessible.h
// #include <qaccessible.h>
// #include <QtGui>

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
#[derive(Default)] // class sizeof(QAccessibleTextCursorEvent)=32
pub struct QAccessibleTextCursorEvent {
  qbase: QAccessibleEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTextCursorEvent_ITF interface {
//    QAccessibleEvent_ITF
//    QAccessibleTextCursorEvent_PTR() *QAccessibleTextCursorEvent
//}
//func (ptr *QAccessibleTextCursorEvent) QAccessibleTextCursorEvent_PTR() *QAccessibleTextCursorEvent { return ptr }

impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTextCursorEvent {
    return QAccessibleTextCursorEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTextCursorEvent {
//  type Target = QAccessibleTextCursorEventBASE;
//
//  fn deref(&self) -> &QAccessibleTextCursorEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTextCursorEventBASE> for QAccessibleTextCursorEvent {
//  fn as_ref(& self) -> & QAccessibleTextCursorEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:747
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextCursorEvent(QObject *, int)

/*

*/
// QAccessibleTextCursorEvent(QObject *, int) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn QAccessibleTextCursorEvent_0<T: QAccessibleTextCursorEvent_QAccessibleTextCursorEvent_0>(value: T) -> QAccessibleTextCursorEvent {
    let rsthis = value.QAccessibleTextCursorEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextCursorEvent_QAccessibleTextCursorEvent_0 {
  fn QAccessibleTextCursorEvent_0(self) -> QAccessibleTextCursorEvent;
}
// QAccessibleTextCursorEvent(QObject *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextCursorEvent_QAccessibleTextCursorEvent_0 for (usize,i32) {
  fn QAccessibleTextCursorEvent_0(self) -> QAccessibleTextCursorEvent {
    // unsafe{_ZN26QAccessibleTextCursorEventC2EP7QObjecti()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextCursorEventC2EP7QObjecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextCursorEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:753
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextCursorEvent(QAccessibleInterface *, int)

/*

*/
// QAccessibleTextCursorEvent(QAccessibleInterface *, int) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn QAccessibleTextCursorEvent_1<T: QAccessibleTextCursorEvent_QAccessibleTextCursorEvent_1>(value: T) -> QAccessibleTextCursorEvent {
    let rsthis = value.QAccessibleTextCursorEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextCursorEvent_QAccessibleTextCursorEvent_1 {
  fn QAccessibleTextCursorEvent_1(self) -> QAccessibleTextCursorEvent;
}
// QAccessibleTextCursorEvent(QAccessibleInterface *, int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextCursorEvent_QAccessibleTextCursorEvent_1 for (usize,i32) {
  fn QAccessibleTextCursorEvent_1(self) -> QAccessibleTextCursorEvent {
    // unsafe{_ZN26QAccessibleTextCursorEventC2EP20QAccessibleInterfacei()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextCursorEventC2EP20QAccessibleInterfacei", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextCursorEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:760
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTextCursorEvent()

/*

*/
pub fn DeleteQAccessibleTextCursorEvent(this :*mut QAccessibleTextCursorEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextCursorEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:762
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setCursorPosition(int)

/*

*/
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn setCursorPosition_0<RetType, T: QAccessibleTextCursorEvent_setCursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCursorPosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextCursorEvent_setCursorPosition_0<RetType> {
  fn setCursorPosition_0(self , rsthis: & QAccessibleTextCursorEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextCursorEvent_setCursorPosition_0<(/*void*/)> for (i32) {
  fn setCursorPosition_0(self , rsthis: & QAccessibleTextCursorEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN26QAccessibleTextCursorEvent17setCursorPositionEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:763
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int cursorPosition() const

/*

*/
impl /*struct*/ QAccessibleTextCursorEvent {
  pub fn cursorPosition_0<RetType, T: QAccessibleTextCursorEvent_cursorPosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cursorPosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextCursorEvent_cursorPosition_0<RetType> {
  fn cursorPosition_0(self , rsthis: & QAccessibleTextCursorEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextCursorEvent_cursorPosition_0<i32> for () {
  fn cursorPosition_0(self , rsthis: & QAccessibleTextCursorEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextCursorEvent14cursorPositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
