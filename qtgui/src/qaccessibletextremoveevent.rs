

// mod ::gui::QAccessibleTextRemoveEvent
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
// extern C begin: 5
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
#[derive(Default)] // class sizeof(QAccessibleTextRemoveEvent)=48
pub struct QAccessibleTextRemoveEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTextRemoveEvent_ITF interface {
//    QAccessibleTextCursorEvent_ITF
//    QAccessibleTextRemoveEvent_PTR() *QAccessibleTextRemoveEvent
//}
//func (ptr *QAccessibleTextRemoveEvent) QAccessibleTextRemoveEvent_PTR() *QAccessibleTextRemoveEvent { return ptr }

impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTextRemoveEvent {
    return QAccessibleTextRemoveEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTextRemoveEvent {
//  type Target = QAccessibleTextRemoveEventBASE;
//
//  fn deref(&self) -> &QAccessibleTextRemoveEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTextRemoveEventBASE> for QAccessibleTextRemoveEvent {
//  fn as_ref(& self) -> & QAccessibleTextRemoveEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:834
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextRemoveEvent(QObject *, int, const QString &)

/*

*/
// QAccessibleTextRemoveEvent(QObject *, int, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn QAccessibleTextRemoveEvent_0<T: QAccessibleTextRemoveEvent_QAccessibleTextRemoveEvent_0>(value: T) -> QAccessibleTextRemoveEvent {
    let rsthis = value.QAccessibleTextRemoveEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_QAccessibleTextRemoveEvent_0 {
  fn QAccessibleTextRemoveEvent_0(self) -> QAccessibleTextRemoveEvent;
}
// QAccessibleTextRemoveEvent(QObject *, int, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_QAccessibleTextRemoveEvent_0 for (usize,i32,usize) {
  fn QAccessibleTextRemoveEvent_0(self) -> QAccessibleTextRemoveEvent {
    // unsafe{_ZN26QAccessibleTextRemoveEventC2EP7QObjectiRK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextRemoveEventC2EP7QObjectiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextRemoveEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:840
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextRemoveEvent(QAccessibleInterface *, int, const QString &)

/*

*/
// QAccessibleTextRemoveEvent(QAccessibleInterface *, int, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn QAccessibleTextRemoveEvent_1<T: QAccessibleTextRemoveEvent_QAccessibleTextRemoveEvent_1>(value: T) -> QAccessibleTextRemoveEvent {
    let rsthis = value.QAccessibleTextRemoveEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextRemoveEvent_QAccessibleTextRemoveEvent_1 {
  fn QAccessibleTextRemoveEvent_1(self) -> QAccessibleTextRemoveEvent;
}
// QAccessibleTextRemoveEvent(QAccessibleInterface *, int, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_QAccessibleTextRemoveEvent_1 for (usize,i32,usize) {
  fn QAccessibleTextRemoveEvent_1(self) -> QAccessibleTextRemoveEvent {
    // unsafe{_ZN26QAccessibleTextRemoveEventC2EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextRemoveEventC2EP20QAccessibleInterfaceiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextRemoveEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:847
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTextRemoveEvent()

/*

*/
pub fn DeleteQAccessibleTextRemoveEvent(this :*mut QAccessibleTextRemoveEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextRemoveEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:849
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString textRemoved() const

/*

*/
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn textRemoved_0<RetType, T: QAccessibleTextRemoveEvent_textRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textRemoved_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextRemoveEvent_textRemoved_0<RetType> {
  fn textRemoved_0(self , rsthis: & QAccessibleTextRemoveEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_textRemoved_0<usize> for () {
  fn textRemoved_0(self , rsthis: & QAccessibleTextRemoveEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextRemoveEvent11textRemovedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:852
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int changePosition() const

/*

*/
impl /*struct*/ QAccessibleTextRemoveEvent {
  pub fn changePosition_0<RetType, T: QAccessibleTextRemoveEvent_changePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changePosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextRemoveEvent_changePosition_0<RetType> {
  fn changePosition_0(self , rsthis: & QAccessibleTextRemoveEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextRemoveEvent_changePosition_0<i32> for () {
  fn changePosition_0(self , rsthis: & QAccessibleTextRemoveEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextRemoveEvent14changePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
