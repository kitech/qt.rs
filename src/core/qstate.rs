

// mod ::core::QState
// package qtcore
// /usr/include/qt/QtCore/qstate.h
// #include <qstate.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 9
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

// void onEntry(QEvent *)
// func (this *QState) InheritOnEntry(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onEntry", f)
// }

// void onExit(QEvent *)
// func (this *QState) InheritOnExit(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onExit", f)
// }

// bool event(QEvent *)
// func (this *QState) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QState)=16
pub struct QState {
  qbase: QAbstractState,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QState_ITF interface {
//    QAbstractState_ITF
//    QState_PTR() *QState
//}
//func (ptr *QState) QState_PTR() *QState { return ptr }

impl /*struct*/ QState {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QState {
    return QState{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QState {
//  type Target = QStateBASE;
//
//  fn deref(&self) -> &QStateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStateBASE> for QState {
//  fn as_ref(& self) -> & QStateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstate.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QState {
  pub fn metaObject_0<RetType, T: QState_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QState_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QState10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstate.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QState(QState *)

/*
Constructs a new state with the given parent state.
*/
// QState(QState *) ctx.fn_proto_cpp
impl /*struct*/ QState {
  pub fn QState_0<T: QState_QState_0>(value: T) -> QState {
    let rsthis = value.QState_0();
    return rsthis;
    // return 1;
  }
}

pub trait QState_QState_0 {
  fn QState_0(self) -> QState;
}
// QState(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QState_QState_0 for (usize) {
  fn QState_0(self) -> QState {
    // unsafe{_ZN6QStateC2EPS_()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QStateC2EPS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QState{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:75
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QState(QState::ChildMode, QState *)

/*
Constructs a new state with the given parent state.
*/
// QState(QState::ChildMode, QState *) ctx.fn_proto_cpp
impl /*struct*/ QState {
  pub fn QState_1<T: QState_QState_1>(value: T) -> QState {
    let rsthis = value.QState_1();
    return rsthis;
    // return 1;
  }
}

pub trait QState_QState_1 {
  fn QState_1(self) -> QState;
}
// QState(QState::ChildMode, QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QState_QState_1 for (i32,usize) {
  fn QState_1(self) -> QState {
    // unsafe{_ZN6QStateC2ENS_9ChildModeEPS_()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QStateC2ENS_9ChildModeEPS_", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QState{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QState()

/*

*/
pub fn DeleteQState(this :*mut QState) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QStateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qstate.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractState * errorState() const

/*
Returns this state's error state.

Note: Getter function for property errorState. 

See also setErrorState() and QStateMachine::error().
*/
impl /*struct*/ QState {
  pub fn errorState_0<RetType, T: QState_errorState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorState_0(self);
    // return 1;
  }
}
pub trait QState_errorState_0<RetType> {
  fn errorState_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_errorState_0<usize> for () {
  fn errorState_0(self , rsthis: & QState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QState10errorStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstate.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setErrorState(QAbstractState *)

/*
Sets this state's error state to be the given state. If the error state is not set, or if it is set to 0, the state will inherit its parent's error state recursively. If no error state is set for the state itself or any of its ancestors, an error will cause the machine to stop executing and an error will be printed to the console.

Note: Setter function for property errorState. 

See also errorState().
*/
impl /*struct*/ QState {
  pub fn setErrorState_0<RetType, T: QState_setErrorState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setErrorState_0(self);
    // return 1;
  }
}
pub trait QState_setErrorState_0<RetType> {
  fn setErrorState_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_setErrorState_0<(/*void*/)> for (usize) {
  fn setErrorState_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState13setErrorStateEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addTransition(QAbstractTransition *)

/*
Adds the given transition. The transition has this state as the source. This state takes ownership of the transition.
*/
impl /*struct*/ QState {
  pub fn addTransition_0<RetType, T: QState_addTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTransition_0(self);
    // return 1;
  }
}
pub trait QState_addTransition_0<RetType> {
  fn addTransition_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_addTransition_0<(/*void*/)> for (usize) {
  fn addTransition_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState13addTransitionEP19QAbstractTransition", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:82
// index:1
// Public Visibility=Default Availability=Available
// [8] QSignalTransition * addTransition(const QObject *, const char *, QAbstractState *)

/*
Adds the given transition. The transition has this state as the source. This state takes ownership of the transition.
*/
impl /*struct*/ QState {
  pub fn addTransition_1<RetType, T: QState_addTransition_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTransition_1(self);
    // return 1;
  }
}
pub trait QState_addTransition_1<RetType> {
  fn addTransition_1(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_addTransition_1<usize> for (usize,usize,usize) {
  fn addTransition_1(self , rsthis: & QState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QState13addTransitionEPK7QObjectPKcP14QAbstractState", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstate.h:96
// index:2
// Public Visibility=Default Availability=Available
// [8] QAbstractTransition * addTransition(QAbstractState *)

/*
Adds the given transition. The transition has this state as the source. This state takes ownership of the transition.
*/
impl /*struct*/ QState {
  pub fn addTransition_2<RetType, T: QState_addTransition_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addTransition_2(self);
    // return 1;
  }
}
pub trait QState_addTransition_2<RetType> {
  fn addTransition_2(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_addTransition_2<usize> for (usize) {
  fn addTransition_2(self , rsthis: & QState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QState13addTransitionEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstate.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeTransition(QAbstractTransition *)

/*
Removes the given transition from this state. The state releases ownership of the transition.

See also addTransition().
*/
impl /*struct*/ QState {
  pub fn removeTransition_0<RetType, T: QState_removeTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeTransition_0(self);
    // return 1;
  }
}
pub trait QState_removeTransition_0<RetType> {
  fn removeTransition_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_removeTransition_0<(/*void*/)> for (usize) {
  fn removeTransition_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState16removeTransitionEP19QAbstractTransition", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractState * initialState() const

/*
Returns this state's initial state, or 0 if the state has no initial state.

Note: Getter function for property initialState. 

See also setInitialState().
*/
impl /*struct*/ QState {
  pub fn initialState_0<RetType, T: QState_initialState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.initialState_0(self);
    // return 1;
  }
}
pub trait QState_initialState_0<RetType> {
  fn initialState_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_initialState_0<usize> for () {
  fn initialState_0(self , rsthis: & QState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QState12initialStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstate.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInitialState(QAbstractState *)

/*
Sets this state's initial state to be the given state. state has to be a child of this state.

Note: Setter function for property initialState. 

See also initialState().
*/
impl /*struct*/ QState {
  pub fn setInitialState_0<RetType, T: QState_setInitialState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInitialState_0(self);
    // return 1;
  }
}
pub trait QState_setInitialState_0<RetType> {
  fn setInitialState_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_setInitialState_0<(/*void*/)> for (usize) {
  fn setInitialState_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState15setInitialStateEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:103
// index:0
// Public Visibility=Default Availability=Available
// [4] QState::ChildMode childMode() const

/*
Returns the child mode of this state.

Note: Getter function for property childMode. 

See also setChildMode().
*/
impl /*struct*/ QState {
  pub fn childMode_0<RetType, T: QState_childMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childMode_0(self);
    // return 1;
  }
}
pub trait QState_childMode_0<RetType> {
  fn childMode_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_childMode_0<i32> for () {
  fn childMode_0(self , rsthis: & QState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QState9childModeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstate.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setChildMode(QState::ChildMode)

/*
Sets the child mode of this state.

Note: Setter function for property childMode. 

See also childMode().
*/
impl /*struct*/ QState {
  pub fn setChildMode_0<RetType, T: QState_setChildMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setChildMode_0(self);
    // return 1;
  }
}
pub trait QState_setChildMode_0<RetType> {
  fn setChildMode_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_setChildMode_0<(/*void*/)> for (i32) {
  fn setChildMode_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QState12setChildModeENS_9ChildModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:107
// index:0
// Public Visibility=Default Availability=Available
// [-2] void assignProperty(QObject *, const char *, const QVariant &)

/*
Instructs this state to set the property with the given name of the given object to the given value when the state is entered.

See also propertiesAssigned().
*/
impl /*struct*/ QState {
  pub fn assignProperty_0<RetType, T: QState_assignProperty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.assignProperty_0(self);
    // return 1;
  }
}
pub trait QState_assignProperty_0<RetType> {
  fn assignProperty_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_assignProperty_0<(/*void*/)> for (usize,usize,usize) {
  fn assignProperty_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:119
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onEntry(QEvent *)

/*
Reimplemented from QAbstractState::onEntry().
*/
impl /*struct*/ QState {
  pub fn onEntry_0<RetType, T: QState_onEntry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onEntry_0(self);
    // return 1;
  }
}
pub trait QState_onEntry_0<RetType> {
  fn onEntry_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_onEntry_0<(/*void*/)> for (usize) {
  fn onEntry_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState7onEntryEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:120
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onExit(QEvent *)

/*
Reimplemented from QAbstractState::onExit().
*/
impl /*struct*/ QState {
  pub fn onExit_0<RetType, T: QState_onExit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onExit_0(self);
    // return 1;
  }
}
pub trait QState_onExit_0<RetType> {
  fn onExit_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_onExit_0<(/*void*/)> for (usize) {
  fn onExit_0(self , rsthis: & QState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QState6onExitEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstate.h:122
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QState {
  pub fn event_0<RetType, T: QState_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QState_event_0<RetType> {
  fn event_0(self , rsthis: & QState) -> RetType;
}
impl<'a> /*trait*/ QState_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QState) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QState5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum specifies how a state's child states are treated.


*/
pub type QState__ChildMode = i32;
// The child states are mutually exclusive and an initial state must be set by calling QState::setInitialState().
pub const QState__ExclusiveStates :QState__ChildMode = 0;
// The child states are parallel. When the parent state is entered, all its child states are entered in parallel.
pub const QState__ParallelStates :QState__ChildMode = 1;
pub fn QState_ChildModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QState", val);
}
pub fn QState_ChildModeItemName_s(val: i32) ->String {
  //var nilthis *QState
  //return nilthis.ChildModeItemName(val);
  return QState_ChildModeItemName(val);
}


/*
This enum specifies the restore policy type. The restore policy takes effect when the machine enters a state which sets one or more properties. If the restore policy is set to RestoreProperties, the state machine will save the original value of the property before the new value is set.

Later, when the machine either enters a state which does not set a value for the given property, the property will automatically be restored to its initial value.

Only one initial value will be saved for any given property. If a value for a property has already been saved by the state machine, it will not be overwritten until the property has been successfully restored.



See also QStateMachine::globalRestorePolicy and QState::assignProperty().

*/
pub type QState__RestorePolicy = i32;
// The state machine should not save the initial values of properties and restore them later.
pub const QState__DontRestoreProperties :QState__RestorePolicy = 0;
// The state machine should save the initial values of properties and restore them later.
pub const QState__RestoreProperties :QState__RestorePolicy = 1;
pub fn QState_RestorePolicyItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QState", val);
}
pub fn QState_RestorePolicyItemName_s(val: i32) ->String {
  //var nilthis *QState
  //return nilthis.RestorePolicyItemName(val);
  return QState_RestorePolicyItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
