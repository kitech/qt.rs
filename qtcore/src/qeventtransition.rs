

// mod ::core::QEventTransition
// package qtcore
// /usr/include/qt/QtCore/qeventtransition.h
// #include <qeventtransition.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 19
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

// bool eventTest(QEvent *)
// func (this *QEventTransition) InheritEventTest(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventTest", f)
// }

// void onTransition(QEvent *)
// func (this *QEventTransition) InheritOnTransition(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onTransition", f)
// }

// bool event(QEvent *)
// func (this *QEventTransition) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QEventTransition)=16
pub struct QEventTransition {
  qbase: QAbstractTransition,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QEventTransition_ITF interface {
//    QAbstractTransition_ITF
//    QEventTransition_PTR() *QEventTransition
//}
//func (ptr *QEventTransition) QEventTransition_PTR() *QEventTransition { return ptr }

impl /*struct*/ QEventTransition {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QEventTransition {
    return QEventTransition{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QEventTransition {
//  type Target = QEventTransitionBASE;
//
//  fn deref(&self) -> &QEventTransitionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QEventTransitionBASE> for QEventTransition {
//  fn as_ref(& self) -> & QEventTransitionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qeventtransition.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QEventTransition {
  pub fn metaObject_0<RetType, T: QEventTransition_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QEventTransition_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QEventTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QEventTransition10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QEventTransition(QState *)

/*
Constructs a new QEventTransition object with the given sourceState.
*/
// QEventTransition(QState *) ctx.fn_proto_cpp
impl /*struct*/ QEventTransition {
  pub fn QEventTransition_0<T: QEventTransition_QEventTransition_0>(value: T) -> QEventTransition {
    let rsthis = value.QEventTransition_0();
    return rsthis;
    // return 1;
  }
}

pub trait QEventTransition_QEventTransition_0 {
  fn QEventTransition_0(self) -> QEventTransition;
}
// QEventTransition(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEventTransition_QEventTransition_0 for (usize) {
  fn QEventTransition_0(self) -> QEventTransition {
    // unsafe{_ZN16QEventTransitionC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QEventTransitionC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEventTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QEventTransition(QObject *, QEvent::Type, QState *)

/*
Constructs a new QEventTransition object with the given sourceState.
*/
// QEventTransition(QObject *, QEvent::Type, QState *) ctx.fn_proto_cpp
impl /*struct*/ QEventTransition {
  pub fn QEventTransition_1<T: QEventTransition_QEventTransition_1>(value: T) -> QEventTransition {
    let rsthis = value.QEventTransition_1();
    return rsthis;
    // return 1;
  }
}

pub trait QEventTransition_QEventTransition_1 {
  fn QEventTransition_1(self) -> QEventTransition;
}
// QEventTransition(QObject *, QEvent::Type, QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEventTransition_QEventTransition_1 for (usize,i32,usize) {
  fn QEventTransition_1(self) -> QEventTransition {
    // unsafe{_ZN16QEventTransitionC2EP7QObjectN6QEvent4TypeEP6QState()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QEventTransitionC2EP7QObjectN6QEvent4TypeEP6QState", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEventTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QEventTransition()

/*

*/
pub fn DeleteQEventTransition(this :*mut QEventTransition) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QEventTransitionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qeventtransition.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * eventSource() const

/*
Returns the event source associated with this event transition.

Note: Getter function for property eventSource. 

See also setEventSource().
*/
impl /*struct*/ QEventTransition {
  pub fn eventSource_0<RetType, T: QEventTransition_eventSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventSource_0(self);
    // return 1;
  }
}
pub trait QEventTransition_eventSource_0<RetType> {
  fn eventSource_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_eventSource_0<usize> for () {
  fn eventSource_0(self , rsthis: & QEventTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QEventTransition11eventSourceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEventSource(QObject *)

/*
Sets the event source associated with this event transition to be the given object.

Note: Setter function for property eventSource. 

See also eventSource().
*/
impl /*struct*/ QEventTransition {
  pub fn setEventSource_0<RetType, T: QEventTransition_setEventSource_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEventSource_0(self);
    // return 1;
  }
}
pub trait QEventTransition_setEventSource_0<RetType> {
  fn setEventSource_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_setEventSource_0<(/*void*/)> for (usize) {
  fn setEventSource_0(self , rsthis: & QEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QEventTransition14setEventSourceEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:64
// index:0
// Public Visibility=Default Availability=Available
// [4] QEvent::Type eventType() const

/*
Returns the event type that this event transition is associated with.

Note: Getter function for property eventType. 

See also setEventType().
*/
impl /*struct*/ QEventTransition {
  pub fn eventType_0<RetType, T: QEventTransition_eventType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventType_0(self);
    // return 1;
  }
}
pub trait QEventTransition_eventType_0<RetType> {
  fn eventType_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_eventType_0<i32> for () {
  fn eventType_0(self , rsthis: & QEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QEventTransition9eventTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEventType(QEvent::Type)

/*
Sets the event type that this event transition is associated with.

Note: Setter function for property eventType. 

See also eventType().
*/
impl /*struct*/ QEventTransition {
  pub fn setEventType_0<RetType, T: QEventTransition_setEventType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEventType_0(self);
    // return 1;
  }
}
pub trait QEventTransition_setEventType_0<RetType> {
  fn setEventType_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_setEventType_0<(/*void*/)> for (i32) {
  fn setEventType_0(self , rsthis: & QEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QEventTransition12setEventTypeEN6QEvent4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:68
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventTest(QEvent *)

/*
Reimplemented from QAbstractTransition::eventTest().
*/
impl /*struct*/ QEventTransition {
  pub fn eventTest_0<RetType, T: QEventTransition_eventTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventTest_0(self);
    // return 1;
  }
}
pub trait QEventTransition_eventTest_0<RetType> {
  fn eventTest_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_eventTest_0<bool> for (usize) {
  fn eventTest_0(self , rsthis: & QEventTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QEventTransition9eventTestEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:69
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onTransition(QEvent *)

/*
Reimplemented from QAbstractTransition::onTransition().
*/
impl /*struct*/ QEventTransition {
  pub fn onTransition_0<RetType, T: QEventTransition_onTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onTransition_0(self);
    // return 1;
  }
}
pub trait QEventTransition_onTransition_0<RetType> {
  fn onTransition_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_onTransition_0<(/*void*/)> for (usize) {
  fn onTransition_0(self , rsthis: & QEventTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN16QEventTransition12onTransitionEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventtransition.h:71
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QEventTransition {
  pub fn event_0<RetType, T: QEventTransition_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QEventTransition_event_0<RetType> {
  fn event_0(self , rsthis: & QEventTransition) -> RetType;
}
impl<'a> /*trait*/ QEventTransition_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QEventTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QEventTransition5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
