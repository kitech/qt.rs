

// mod ::gui::QAccessibleTextUpdateEvent
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
#[derive(Default)] // class sizeof(QAccessibleTextUpdateEvent)=56
pub struct QAccessibleTextUpdateEvent {
  qbase: QAccessibleTextCursorEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAccessibleTextUpdateEvent_ITF interface {
//    QAccessibleTextCursorEvent_ITF
//    QAccessibleTextUpdateEvent_PTR() *QAccessibleTextUpdateEvent
//}
//func (ptr *QAccessibleTextUpdateEvent) QAccessibleTextUpdateEvent_PTR() *QAccessibleTextUpdateEvent { return ptr }

impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAccessibleTextUpdateEvent {
    return QAccessibleTextUpdateEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAccessibleTextUpdateEvent {
//  type Target = QAccessibleTextUpdateEventBASE;
//
//  fn deref(&self) -> &QAccessibleTextUpdateEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAccessibleTextUpdateEventBASE> for QAccessibleTextUpdateEvent {
//  fn as_ref(& self) -> & QAccessibleTextUpdateEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qaccessible.h:864
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextUpdateEvent(QObject *, int, const QString &, const QString &)

/*

*/
// QAccessibleTextUpdateEvent(QObject *, int, const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn QAccessibleTextUpdateEvent_0<T: QAccessibleTextUpdateEvent_QAccessibleTextUpdateEvent_0>(value: T) -> QAccessibleTextUpdateEvent {
    let rsthis = value.QAccessibleTextUpdateEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_QAccessibleTextUpdateEvent_0 {
  fn QAccessibleTextUpdateEvent_0(self) -> QAccessibleTextUpdateEvent;
}
// QAccessibleTextUpdateEvent(QObject *, int, const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_QAccessibleTextUpdateEvent_0 for (usize,i32,usize,usize) {
  fn QAccessibleTextUpdateEvent_0(self) -> QAccessibleTextUpdateEvent {
    // unsafe{_ZN26QAccessibleTextUpdateEventC2EP7QObjectiRK7QStringS4_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextUpdateEventC2EP7QObjectiRK7QStringS4_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextUpdateEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:870
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QAccessibleTextUpdateEvent(QAccessibleInterface *, int, const QString &, const QString &)

/*

*/
// QAccessibleTextUpdateEvent(QAccessibleInterface *, int, const QString &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn QAccessibleTextUpdateEvent_1<T: QAccessibleTextUpdateEvent_QAccessibleTextUpdateEvent_1>(value: T) -> QAccessibleTextUpdateEvent {
    let rsthis = value.QAccessibleTextUpdateEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextUpdateEvent_QAccessibleTextUpdateEvent_1 {
  fn QAccessibleTextUpdateEvent_1(self) -> QAccessibleTextUpdateEvent;
}
// QAccessibleTextUpdateEvent(QAccessibleInterface *, int, const QString &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_QAccessibleTextUpdateEvent_1 for (usize,i32,usize,usize) {
  fn QAccessibleTextUpdateEvent_1(self) -> QAccessibleTextUpdateEvent {
    // unsafe{_ZN26QAccessibleTextUpdateEventC2EP20QAccessibleInterfaceiRK7QStringS4_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextUpdateEventC2EP20QAccessibleInterfaceiRK7QStringS4_", 4,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAccessibleTextUpdateEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:877
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAccessibleTextUpdateEvent()

/*

*/
pub fn DeleteQAccessibleTextUpdateEvent(this :*mut QAccessibleTextUpdateEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN26QAccessibleTextUpdateEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 56)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qaccessible.h:879
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString textRemoved() const

/*

*/
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textRemoved_0<RetType, T: QAccessibleTextUpdateEvent_textRemoved_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textRemoved_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextUpdateEvent_textRemoved_0<RetType> {
  fn textRemoved_0(self , rsthis: & QAccessibleTextUpdateEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textRemoved_0<usize> for () {
  fn textRemoved_0(self , rsthis: & QAccessibleTextUpdateEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextUpdateEvent11textRemovedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:882
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString textInserted() const

/*

*/
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn textInserted_0<RetType, T: QAccessibleTextUpdateEvent_textInserted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.textInserted_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextUpdateEvent_textInserted_0<RetType> {
  fn textInserted_0(self , rsthis: & QAccessibleTextUpdateEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_textInserted_0<usize> for () {
  fn textInserted_0(self , rsthis: & QAccessibleTextUpdateEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextUpdateEvent12textInsertedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qaccessible.h:885
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int changePosition() const

/*

*/
impl /*struct*/ QAccessibleTextUpdateEvent {
  pub fn changePosition_0<RetType, T: QAccessibleTextUpdateEvent_changePosition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.changePosition_0(self);
    // return 1;
  }
}
pub trait QAccessibleTextUpdateEvent_changePosition_0<RetType> {
  fn changePosition_0(self , rsthis: & QAccessibleTextUpdateEvent) -> RetType;
}
impl<'a> /*trait*/ QAccessibleTextUpdateEvent_changePosition_0<i32> for () {
  fn changePosition_0(self , rsthis: & QAccessibleTextUpdateEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK26QAccessibleTextUpdateEvent14changePositionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
