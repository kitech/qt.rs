

// mod ::gui::QAccessibleTextInsertEvent
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
#[derive(Default)] // class sizeof(QAccessibleTextInsertEvent)=48
pub struct QAccessibleTextInsertEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTextInsertEvent_ITF interface {
//    QAccessibleTextCursorEvent_ITF
//    QAccessibleTextInsertEvent_PTR() *QAccessibleTextInsertEvent
//}
//func (ptr *QAccessibleTextInsertEvent) QAccessibleTextInsertEvent_PTR() *QAccessibleTextInsertEvent { return ptr }

impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTextInsertEvent {
    return QAccessibleTextInsertEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTextInsertEvent {
//  type Target = QAccessibleTextInsertEventBASE;
//
//  fn deref(&self) -> &QAccessibleTextInsertEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTextInsertEventBASE> for QAccessibleTextInsertEvent {
//  fn as_ref(& self) -> & QAccessibleTextInsertEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:804
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextInsertEvent(QObject *, int, const QString &)

/*

*/
// QAccessibleTextInsertEvent(QObject *, int, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn QAccessibleTextInsertEvent_0<T: QAccessibleTextInsertEvent_QAccessibleTextInsertEvent_0>(value: T) -> QAccessibleTextInsertEvent {
    let rsthis = value.QAccessibleTextInsertEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_QAccessibleTextInsertEvent_0 {
  fn QAccessibleTextInsertEvent_0(self) -> QAccessibleTextInsertEvent;
}
// QAccessibleTextInsertEvent(QObject *, int, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextInsertEvent_QAccessibleTextInsertEvent_0 for (usize,i32,usize) {
  fn QAccessibleTextInsertEvent_0(self) -> QAccessibleTextInsertEvent {
    // unsafe{_ZN26QAccessibleTextInsertEventC2EP7QObjectiRK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextInsertEventC2EP7QObjectiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextInsertEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:810
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextInsertEvent(QAccessibleInterface *, int, const QString &)

/*

*/
// QAccessibleTextInsertEvent(QAccessibleInterface *, int, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn QAccessibleTextInsertEvent_1<T: QAccessibleTextInsertEvent_QAccessibleTextInsertEvent_1>(value: T) -> QAccessibleTextInsertEvent {
    let rsthis = value.QAccessibleTextInsertEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_QAccessibleTextInsertEvent_1 {
  fn QAccessibleTextInsertEvent_1(self) -> QAccessibleTextInsertEvent;
}
// QAccessibleTextInsertEvent(QAccessibleInterface *, int, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextInsertEvent_QAccessibleTextInsertEvent_1 for (usize,i32,usize) {
  fn QAccessibleTextInsertEvent_1(self) -> QAccessibleTextInsertEvent {
    // unsafe{_ZN26QAccessibleTextInsertEventC2EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextInsertEventC2EP20QAccessibleInterfaceiRK7QString", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextInsertEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:817
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTextInsertEvent()

/*

*/
pub fn DeleteQAccessibleTextInsertEvent(this :*mut QAccessibleTextInsertEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextInsertEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 48)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:819
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString textInserted() const

/*

*/
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn textInserted_0<RetType, T: QAccessibleTextInsertEvent_textInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInserted_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInsertEvent_textInserted_0<RetType> {
  fn textInserted_0(self , rsthis: & QAccessibleTextInsertEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInsertEvent_textInserted_0<usize> for () {
  fn textInserted_0(self , rsthis: & QAccessibleTextInsertEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextInsertEvent12textInsertedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:822
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int changePosition() const

/*

*/
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn changePosition_0<RetType, T: QAccessibleTextInsertEvent_changePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changePosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextInsertEvent_changePosition_0<RetType> {
  fn changePosition_0(self , rsthis: & QAccessibleTextInsertEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextInsertEvent_changePosition_0<i32> for () {
  fn changePosition_0(self , rsthis: & QAccessibleTextInsertEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextInsertEvent14changePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
