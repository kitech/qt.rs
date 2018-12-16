

// mod ::core::QSignalTransition
// package qtcore
// /usr/include/qt/QtCore/qsignaltransition.h
// #include <qsignaltransition.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
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

// bool eventTest(QEvent *)
// func (this *QSignalTransition) InheritEventTest(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventTest", f)
// }

// void onTransition(QEvent *)
// func (this *QSignalTransition) InheritOnTransition(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onTransition", f)
// }

// bool event(QEvent *)
// func (this *QSignalTransition) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSignalTransition)=16
pub struct QSignalTransition {
  qbase: QAbstractTransition,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSignalTransition_ITF interface {
//    QAbstractTransition_ITF
//    QSignalTransition_PTR() *QSignalTransition
//}
//func (ptr *QSignalTransition) QSignalTransition_PTR() *QSignalTransition { return ptr }

impl /*struct*/ QSignalTransition {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSignalTransition {
    return QSignalTransition{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSignalTransition {
//  type Target = QSignalTransitionBASE;
//
//  fn deref(&self) -> &QSignalTransitionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSignalTransitionBASE> for QSignalTransition {
//  fn as_ref(& self) -> & QSignalTransitionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsignaltransition.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSignalTransition {
  pub fn metaObject_0<RetType, T: QSignalTransition_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSignalTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QSignalTransition10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSignalTransition(QState *)

/*
Constructs a new signal transition with the given sourceState.
*/
// QSignalTransition(QState *) ctx.fn_proto_cpp
impl /*struct*/ QSignalTransition {
  pub fn QSignalTransition_0<T: QSignalTransition_QSignalTransition_0>(value: T) -> QSignalTransition {
    let rsthis = value.QSignalTransition_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalTransition_QSignalTransition_0 {
  fn QSignalTransition_0(self) -> QSignalTransition;
}
// QSignalTransition(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSignalTransition_QSignalTransition_0 for (usize) {
  fn QSignalTransition_0(self) -> QSignalTransition {
    // unsafe{_ZN17QSignalTransitionC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QSignalTransitionC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSignalTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSignalTransition(const QObject *, const char *, QState *)

/*
Constructs a new signal transition with the given sourceState.
*/
// QSignalTransition(const QObject *, const char *, QState *) ctx.fn_proto_cpp
impl /*struct*/ QSignalTransition {
  pub fn QSignalTransition_1<T: QSignalTransition_QSignalTransition_1>(value: T) -> QSignalTransition {
    let rsthis = value.QSignalTransition_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalTransition_QSignalTransition_1 {
  fn QSignalTransition_1(self) -> QSignalTransition;
}
// QSignalTransition(const QObject *, const char *, QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSignalTransition_QSignalTransition_1 for (usize,usize,usize) {
  fn QSignalTransition_1(self) -> QSignalTransition {
    // unsafe{_ZN17QSignalTransitionC2EPK7QObjectPKcP6QState()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN17QSignalTransitionC2EPK7QObjectPKcP6QState", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSignalTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSignalTransition()

/*

*/
pub fn DeleteQSignalTransition(this :*mut QSignalTransition) {
    // let rv = qtrt::InvokeQtFunc6("_ZN17QSignalTransitionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsignaltransition.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * senderObject() const

/*
Returns the sender object associated with this signal transition.

Note: Getter function for property senderObject. 

See also setSenderObject().
*/
impl /*struct*/ QSignalTransition {
  pub fn senderObject_0<RetType, T: QSignalTransition_senderObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.senderObject_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_senderObject_0<RetType> {
  fn senderObject_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_senderObject_0<usize> for () {
  fn senderObject_0(self , rsthis: & QSignalTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QSignalTransition12senderObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSenderObject(const QObject *)

/*
Sets the sender object associated with this signal transition.

Note: Setter function for property senderObject. 

See also senderObject().
*/
impl /*struct*/ QSignalTransition {
  pub fn setSenderObject_0<RetType, T: QSignalTransition_setSenderObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSenderObject_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_setSenderObject_0<RetType> {
  fn setSenderObject_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_setSenderObject_0<(/*void*/)> for (usize) {
  fn setSenderObject_0(self , rsthis: & QSignalTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QSignalTransition15setSenderObjectEPK7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray signal() const

/*
Returns the signal associated with this signal transition.

Note: Getter function for property signal. 

See also setSignal().
*/
impl /*struct*/ QSignalTransition {
  pub fn signal_0<RetType, T: QSignalTransition_signal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.signal_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_signal_0<RetType> {
  fn signal_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_signal_0<usize> for () {
  fn signal_0(self , rsthis: & QSignalTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK17QSignalTransition6signalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSignal(const QByteArray &)

/*
Sets the signal associated with this signal transition.

Note: Setter function for property signal. 

See also signal().
*/
impl /*struct*/ QSignalTransition {
  pub fn setSignal_0<RetType, T: QSignalTransition_setSignal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSignal_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_setSignal_0<RetType> {
  fn setSignal_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_setSignal_0<(/*void*/)> for (usize) {
  fn setSignal_0(self , rsthis: & QSignalTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QSignalTransition9setSignalERK10QByteArray", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:83
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool eventTest(QEvent *)

/*
Reimplemented from QAbstractTransition::eventTest().

The default implementation returns true if the event is a QStateMachine::SignalEvent object and the event's sender and signal index match this transition, and returns false otherwise.
*/
impl /*struct*/ QSignalTransition {
  pub fn eventTest_0<RetType, T: QSignalTransition_eventTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventTest_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_eventTest_0<RetType> {
  fn eventTest_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_eventTest_0<bool> for (usize) {
  fn eventTest_0(self , rsthis: & QSignalTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QSignalTransition9eventTestEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:84
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onTransition(QEvent *)

/*
Reimplemented from QAbstractTransition::onTransition().
*/
impl /*struct*/ QSignalTransition {
  pub fn onTransition_0<RetType, T: QSignalTransition_onTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onTransition_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_onTransition_0<RetType> {
  fn onTransition_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_onTransition_0<(/*void*/)> for (usize) {
  fn onTransition_0(self , rsthis: & QSignalTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN17QSignalTransition12onTransitionEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignaltransition.h:86
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSignalTransition {
  pub fn event_0<RetType, T: QSignalTransition_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSignalTransition_event_0<RetType> {
  fn event_0(self , rsthis: & QSignalTransition) -> RetType;
}
impl<'a> /*trait*/ QSignalTransition_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSignalTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN17QSignalTransition5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
