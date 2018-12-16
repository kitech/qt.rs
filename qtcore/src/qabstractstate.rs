

// mod ::core::QAbstractState
// package qtcore
// /usr/include/qt/QtCore/qabstractstate.h
// #include <qabstractstate.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 29
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

// void onEntry(QEvent *)
// func (this *QAbstractState) InheritOnEntry(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onEntry", f)
// }

// void onExit(QEvent *)
// func (this *QAbstractState) InheritOnExit(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onExit", f)
// }

// bool event(QEvent *)
// func (this *QAbstractState) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractState)=16
pub struct QAbstractState {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractState_ITF interface {
//    QObject_ITF
//    QAbstractState_PTR() *QAbstractState
//}
//func (ptr *QAbstractState) QAbstractState_PTR() *QAbstractState { return ptr }

impl /*struct*/ QAbstractState {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractState {
    return QAbstractState{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractState {
//  type Target = QAbstractStateBASE;
//
//  fn deref(&self) -> &QAbstractStateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractStateBASE> for QAbstractState {
//  fn as_ref(& self) -> & QAbstractStateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractstate.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractState {
  pub fn metaObject_0<RetType, T: QAbstractState_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractState_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QAbstractState10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractState()

/*

*/
pub fn DeleteQAbstractState(this :*mut QAbstractState) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QAbstractStateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractstate.h:60
// index:0
// Public Visibility=Default Availability=Available
// [8] QState * parentState() const

/*
Returns this state's parent state, or 0 if the state has no parent state.
*/
impl /*struct*/ QAbstractState {
  pub fn parentState_0<RetType, T: QAbstractState_parentState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.parentState_0(self);
    // return 1;
  }
}
pub trait QAbstractState_parentState_0<RetType> {
  fn parentState_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_parentState_0<usize> for () {
  fn parentState_0(self , rsthis: & QAbstractState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QAbstractState11parentStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QStateMachine * machine() const

/*
Returns the state machine that this state is part of, or 0 if the state is not part of a state machine.
*/
impl /*struct*/ QAbstractState {
  pub fn machine_0<RetType, T: QAbstractState_machine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.machine_0(self);
    // return 1;
  }
}
pub trait QAbstractState_machine_0<RetType> {
  fn machine_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_machine_0<usize> for () {
  fn machine_0(self , rsthis: & QAbstractState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QAbstractState7machineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:63
// index:0
// Public Visibility=Default Availability=Available
// [1] bool active() const

/*
Returns whether this state is active.

Note: Getter function for property active. 

See also activeChanged(bool), entered(), and exited().
*/
impl /*struct*/ QAbstractState {
  pub fn active_0<RetType, T: QAbstractState_active_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.active_0(self);
    // return 1;
  }
}
pub trait QAbstractState_active_0<RetType> {
  fn active_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_active_0<bool> for () {
  fn active_0(self , rsthis: & QAbstractState) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QAbstractState6activeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activeChanged(bool)

/*
This signal is emitted when the active property is changed with active as argument.

This function was introduced in  Qt 5.4.

Note: Notifier signal for property active. 

See also QAbstractState::active, entered(), and exited().
*/
impl /*struct*/ QAbstractState {
  pub fn activeChanged_0<RetType, T: QAbstractState_activeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeChanged_0(self);
    // return 1;
  }
}
pub trait QAbstractState_activeChanged_0<RetType> {
  fn activeChanged_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_activeChanged_0<(/*void*/)> for (bool) {
  fn activeChanged_0(self , rsthis: & QAbstractState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN14QAbstractState13activeChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:71
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QAbstractState(QState *)

/*
Constructs a new state with the given parent state.
*/
// QAbstractState(QState *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractState {
  pub fn QAbstractState_0<T: QAbstractState_QAbstractState_0>(value: T) -> QAbstractState {
    let rsthis = value.QAbstractState_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractState_QAbstractState_0 {
  fn QAbstractState_0(self) -> QAbstractState;
}
// QAbstractState(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractState_QAbstractState_0 for (usize) {
  fn QAbstractState_0(self) -> QAbstractState {
    // unsafe{_ZN14QAbstractStateC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QAbstractStateC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractState{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:73
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void onEntry(QEvent *)

/*
This function is called when the state is entered. The given event is what caused the state to be entered. Reimplement this function to perform custom processing when the state is entered.
*/
impl /*struct*/ QAbstractState {
  pub fn onEntry_0<RetType, T: QAbstractState_onEntry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onEntry_0(self);
    // return 1;
  }
}
pub trait QAbstractState_onEntry_0<RetType> {
  fn onEntry_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_onEntry_0<(/*void*/)> for (usize) {
  fn onEntry_0(self , rsthis: & QAbstractState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QAbstractState7onEntryEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:74
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void onExit(QEvent *)

/*
This function is called when the state is exited. The given event is what caused the state to be exited. Reimplement this function to perform custom processing when the state is exited.
*/
impl /*struct*/ QAbstractState {
  pub fn onExit_0<RetType, T: QAbstractState_onExit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onExit_0(self);
    // return 1;
  }
}
pub trait QAbstractState_onExit_0<RetType> {
  fn onExit_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_onExit_0<(/*void*/)> for (usize) {
  fn onExit_0(self , rsthis: & QAbstractState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QAbstractState6onExitEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractstate.h:76
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractState {
  pub fn event_0<RetType, T: QAbstractState_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractState_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractState) -> RetType;
}
impl<'a> /*trait*/ QAbstractState_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractState) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QAbstractState5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
