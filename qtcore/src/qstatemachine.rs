

// mod ::core::QStateMachine
// package qtcore
// /usr/include/qt/QtCore/qstatemachine.h
// #include <qstatemachine.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 18
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
// func (this *QStateMachine) InheritOnEntry(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onEntry", f)
// }

// void onExit(QEvent *)
// func (this *QStateMachine) InheritOnExit(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onExit", f)
// }

// void beginSelectTransitions(QEvent *)
// func (this *QStateMachine) InheritBeginSelectTransitions(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginSelectTransitions", f)
// }

// void endSelectTransitions(QEvent *)
// func (this *QStateMachine) InheritEndSelectTransitions(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endSelectTransitions", f)
// }

// void beginMicrostep(QEvent *)
// func (this *QStateMachine) InheritBeginMicrostep(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "beginMicrostep", f)
// }

// void endMicrostep(QEvent *)
// func (this *QStateMachine) InheritEndMicrostep(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "endMicrostep", f)
// }

// bool event(QEvent *)
// func (this *QStateMachine) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QStateMachine)=16
pub struct QStateMachine {
  qbase: QState,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStateMachine_ITF interface {
//    QState_ITF
//    QStateMachine_PTR() *QStateMachine
//}
//func (ptr *QStateMachine) QStateMachine_PTR() *QStateMachine { return ptr }

impl /*struct*/ QStateMachine {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStateMachine {
    return QStateMachine{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStateMachine {
//  type Target = QStateMachineBASE;
//
//  fn deref(&self) -> &QStateMachineBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStateMachineBASE> for QStateMachine {
//  fn as_ref(& self) -> & QStateMachineBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstatemachine.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QStateMachine {
  pub fn metaObject_0<RetType, T: QStateMachine_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QStateMachine_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QStateMachine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStateMachine10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStateMachine(QObject *)

/*
Constructs a new state machine with the given parent.
*/
// QStateMachine(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStateMachine {
  pub fn QStateMachine_0<T: QStateMachine_QStateMachine_0>(value: T) -> QStateMachine {
    let rsthis = value.QStateMachine_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStateMachine_QStateMachine_0 {
  fn QStateMachine_0(self) -> QStateMachine;
}
// QStateMachine(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStateMachine_QStateMachine_0 for (usize) {
  fn QStateMachine_0(self) -> QStateMachine {
    // unsafe{_ZN13QStateMachineC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStateMachineC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStateMachine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:113
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStateMachine(QState::ChildMode, QObject *)

/*
Constructs a new state machine with the given parent.
*/
// QStateMachine(QState::ChildMode, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QStateMachine {
  pub fn QStateMachine_1<T: QStateMachine_QStateMachine_1>(value: T) -> QStateMachine {
    let rsthis = value.QStateMachine_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStateMachine_QStateMachine_1 {
  fn QStateMachine_1(self) -> QStateMachine;
}
// QStateMachine(QState::ChildMode, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStateMachine_QStateMachine_1 for (i32,usize) {
  fn QStateMachine_1(self) -> QStateMachine {
    // unsafe{_ZN13QStateMachineC2EN6QState9ChildModeEP7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QStateMachineC2EN6QState9ChildModeEP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStateMachine{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:114
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QStateMachine()

/*

*/
pub fn DeleteQStateMachine(this :*mut QStateMachine) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QStateMachineD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qstatemachine.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addState(QAbstractState *)

/*
Adds the given state to this state machine. The state becomes a top-level state.

If the state is already in a different machine, it will first be removed from its old machine, and then added to this machine.

See also removeState() and setInitialState().
*/
impl /*struct*/ QStateMachine {
  pub fn addState_0<RetType, T: QStateMachine_addState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addState_0(self);
    // return 1;
  }
}
pub trait QStateMachine_addState_0<RetType> {
  fn addState_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_addState_0<(/*void*/)> for (usize) {
  fn addState_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine8addStateEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:117
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeState(QAbstractState *)

/*
Removes the given state from this state machine. The state machine releases ownership of the state.

See also addState().
*/
impl /*struct*/ QStateMachine {
  pub fn removeState_0<RetType, T: QStateMachine_removeState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeState_0(self);
    // return 1;
  }
}
pub trait QStateMachine_removeState_0<RetType> {
  fn removeState_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_removeState_0<(/*void*/)> for (usize) {
  fn removeState_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine11removeStateEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:119
// index:0
// Public Visibility=Default Availability=Available
// [4] QStateMachine::Error error() const

/*
Returns the error code of the last error that occurred in the state machine.
*/
impl /*struct*/ QStateMachine {
  pub fn error_0<RetType, T: QStateMachine_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QStateMachine_error_0<RetType> {
  fn error_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_error_0<i32> for () {
  fn error_0(self , rsthis: & QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStateMachine5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:120
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns the error string of the last error that occurred in the state machine.

Note: Getter function for property errorString.
*/
impl /*struct*/ QStateMachine {
  pub fn errorString_0<RetType, T: QStateMachine_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QStateMachine_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QStateMachine) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStateMachine11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:121
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clearError()

/*
Clears the error string and error code of the state machine.
*/
impl /*struct*/ QStateMachine {
  pub fn clearError_0<RetType, T: QStateMachine_clearError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clearError_0(self);
    // return 1;
  }
}
pub trait QStateMachine_clearError_0<RetType> {
  fn clearError_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_clearError_0<(/*void*/)> for () {
  fn clearError_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QStateMachine10clearErrorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:123
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRunning() const

/*

*/
impl /*struct*/ QStateMachine {
  pub fn isRunning_0<RetType, T: QStateMachine_isRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRunning_0(self);
    // return 1;
  }
}
pub trait QStateMachine_isRunning_0<RetType> {
  fn isRunning_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_isRunning_0<bool> for () {
  fn isRunning_0(self , rsthis: & QStateMachine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStateMachine9isRunningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:126
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAnimated() const

/*
Returns whether animations are enabled for this state machine.

Note: Getter function for property animated.
*/
impl /*struct*/ QStateMachine {
  pub fn isAnimated_0<RetType, T: QStateMachine_isAnimated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAnimated_0(self);
    // return 1;
  }
}
pub trait QStateMachine_isAnimated_0<RetType> {
  fn isAnimated_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_isAnimated_0<bool> for () {
  fn isAnimated_0(self , rsthis: & QStateMachine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStateMachine10isAnimatedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAnimated(bool)

/*
Sets whether animations are enabled for this state machine.

Note: Setter function for property animated. 

See also isAnimated().
*/
impl /*struct*/ QStateMachine {
  pub fn setAnimated_0<RetType, T: QStateMachine_setAnimated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAnimated_0(self);
    // return 1;
  }
}
pub trait QStateMachine_setAnimated_0<RetType> {
  fn setAnimated_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_setAnimated_0<(/*void*/)> for (bool) {
  fn setAnimated_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine11setAnimatedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:129
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addDefaultAnimation(QAbstractAnimation *)

/*
Adds a default animation to be considered for any transition.
*/
impl /*struct*/ QStateMachine {
  pub fn addDefaultAnimation_0<RetType, T: QStateMachine_addDefaultAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addDefaultAnimation_0(self);
    // return 1;
  }
}
pub trait QStateMachine_addDefaultAnimation_0<RetType> {
  fn addDefaultAnimation_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_addDefaultAnimation_0<(/*void*/)> for (usize) {
  fn addDefaultAnimation_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine19addDefaultAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeDefaultAnimation(QAbstractAnimation *)

/*
Removes animation from the list of default animations.
*/
impl /*struct*/ QStateMachine {
  pub fn removeDefaultAnimation_0<RetType, T: QStateMachine_removeDefaultAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeDefaultAnimation_0(self);
    // return 1;
  }
}
pub trait QStateMachine_removeDefaultAnimation_0<RetType> {
  fn removeDefaultAnimation_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_removeDefaultAnimation_0<(/*void*/)> for (usize) {
  fn removeDefaultAnimation_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine22removeDefaultAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:134
// index:0
// Public Visibility=Default Availability=Available
// [4] QState::RestorePolicy globalRestorePolicy() const

/*
Returns the restore policy of the state machine.

Note: Getter function for property globalRestorePolicy. 

See also setGlobalRestorePolicy().
*/
impl /*struct*/ QStateMachine {
  pub fn globalRestorePolicy_0<RetType, T: QStateMachine_globalRestorePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.globalRestorePolicy_0(self);
    // return 1;
  }
}
pub trait QStateMachine_globalRestorePolicy_0<RetType> {
  fn globalRestorePolicy_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_globalRestorePolicy_0<i32> for () {
  fn globalRestorePolicy_0(self , rsthis: & QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QStateMachine19globalRestorePolicyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setGlobalRestorePolicy(QState::RestorePolicy)

/*
Sets the restore policy of the state machine to restorePolicy. The default restore policy is QState::DontRestoreProperties.

Note: Setter function for property globalRestorePolicy. 

See also globalRestorePolicy().
*/
impl /*struct*/ QStateMachine {
  pub fn setGlobalRestorePolicy_0<RetType, T: QStateMachine_setGlobalRestorePolicy_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setGlobalRestorePolicy_0(self);
    // return 1;
  }
}
pub trait QStateMachine_setGlobalRestorePolicy_0<RetType> {
  fn setGlobalRestorePolicy_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_setGlobalRestorePolicy_0<(/*void*/)> for (i32) {
  fn setGlobalRestorePolicy_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine22setGlobalRestorePolicyEN6QState13RestorePolicyE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void postEvent(QEvent *, QStateMachine::EventPriority)

/*
Posts the given event of the given priority for processing by this state machine.

This function returns immediately. The event is added to the state machine's event queue. Events are processed in the order posted. The state machine takes ownership of the event and deletes it once it has been processed.

You can only post events when the state machine is running or when it is starting up.

Note: This function is thread-safe.

See also postDelayedEvent().
*/
impl /*struct*/ QStateMachine {
  pub fn postEvent_0<RetType, T: QStateMachine_postEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.postEvent_0(self);
    // return 1;
  }
}
pub trait QStateMachine_postEvent_0<RetType> {
  fn postEvent_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_postEvent_0<(/*void*/)> for (usize,i32) {
  fn postEvent_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine9postEventEP6QEventNS_13EventPriorityE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:138
// index:0
// Public Visibility=Default Availability=Available
// [4] int postDelayedEvent(QEvent *, int)

/*
Posts the given event for processing by this state machine, with the given delay in milliseconds. Returns an identifier associated with the delayed event, or -1 if the event could not be posted.

This function returns immediately. When the delay has expired, the event will be added to the state machine's event queue for processing. The state machine takes ownership of the event and deletes it once it has been processed.

You can only post events when the state machine is running.

Note: This function is thread-safe.

See also cancelDelayedEvent() and postEvent().
*/
impl /*struct*/ QStateMachine {
  pub fn postDelayedEvent_0<RetType, T: QStateMachine_postDelayedEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.postDelayedEvent_0(self);
    // return 1;
  }
}
pub trait QStateMachine_postDelayedEvent_0<RetType> {
  fn postDelayedEvent_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_postDelayedEvent_0<i32> for (usize,i32) {
  fn postDelayedEvent_0(self , rsthis: & QStateMachine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStateMachine16postDelayedEventEP6QEventi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:139
// index:0
// Public Visibility=Default Availability=Available
// [1] bool cancelDelayedEvent(int)

/*
Cancels the delayed event identified by the given id. The id should be a value returned by a call to postDelayedEvent(). Returns true if the event was successfully cancelled, otherwise returns false.

Note: This function is thread-safe.

See also postDelayedEvent().
*/
impl /*struct*/ QStateMachine {
  pub fn cancelDelayedEvent_0<RetType, T: QStateMachine_cancelDelayedEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancelDelayedEvent_0(self);
    // return 1;
  }
}
pub trait QStateMachine_cancelDelayedEvent_0<RetType> {
  fn cancelDelayedEvent_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_cancelDelayedEvent_0<bool> for (i32) {
  fn cancelDelayedEvent_0(self , rsthis: & QStateMachine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStateMachine18cancelDelayedEventEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:144
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool eventFilter(QObject *, QEvent *)

/*
Reimplemented from QObject::eventFilter().
*/
impl /*struct*/ QStateMachine {
  pub fn eventFilter_0<RetType, T: QStateMachine_eventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventFilter_0(self);
    // return 1;
  }
}
pub trait QStateMachine_eventFilter_0<RetType> {
  fn eventFilter_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_eventFilter_0<bool> for (usize,usize) {
  fn eventFilter_0(self , rsthis: & QStateMachine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStateMachine11eventFilterEP7QObjectP6QEvent", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:148
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start()

/*
Starts this state machine. The machine will reset its configuration and transition to the initial state. When a final top-level state (QFinalState) is entered, the machine will emit the finished() signal.

Note: A state machine will not run without a running event loop, such as the main application event loop started with QCoreApplication::exec() or QApplication::exec().

See also started(), finished(), stop(), initialState(), and setRunning().
*/
impl /*struct*/ QStateMachine {
  pub fn start_0<RetType, T: QStateMachine_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QStateMachine_start_0<RetType> {
  fn start_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_start_0<(/*void*/)> for () {
  fn start_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QStateMachine5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:149
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops this state machine. The state machine will stop processing events and then emit the stopped() signal.

See also stopped(), start(), and setRunning().
*/
impl /*struct*/ QStateMachine {
  pub fn stop_0<RetType, T: QStateMachine_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QStateMachine_stop_0<RetType> {
  fn stop_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QStateMachine4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRunning(bool)

/*

*/
impl /*struct*/ QStateMachine {
  pub fn setRunning_0<RetType, T: QStateMachine_setRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRunning_0(self);
    // return 1;
  }
}
pub trait QStateMachine_setRunning_0<RetType> {
  fn setRunning_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_setRunning_0<(/*void*/)> for (bool) {
  fn setRunning_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine10setRunningEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:155
// index:0
// Public Visibility=Default Availability=Available
// [-2] void runningChanged(bool)

/*
This signal is emitted when the running property is changed with running as argument.

This function was introduced in  Qt 5.4.

Note: Notifier signal for property running. 

See also QStateMachine::running.
*/
impl /*struct*/ QStateMachine {
  pub fn runningChanged_0<RetType, T: QStateMachine_runningChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.runningChanged_0(self);
    // return 1;
  }
}
pub trait QStateMachine_runningChanged_0<RetType> {
  fn runningChanged_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_runningChanged_0<(/*void*/)> for (bool) {
  fn runningChanged_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine14runningChangedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:159
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onEntry(QEvent *)

/*
Reimplemented from QAbstractState::onEntry().

This function will call start() to start the state machine.
*/
impl /*struct*/ QStateMachine {
  pub fn onEntry_0<RetType, T: QStateMachine_onEntry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onEntry_0(self);
    // return 1;
  }
}
pub trait QStateMachine_onEntry_0<RetType> {
  fn onEntry_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_onEntry_0<(/*void*/)> for (usize) {
  fn onEntry_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine7onEntryEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:160
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onExit(QEvent *)

/*
Reimplemented from QAbstractState::onExit().

This function will call stop() to stop the state machine and subsequently emit the stopped() signal.
*/
impl /*struct*/ QStateMachine {
  pub fn onExit_0<RetType, T: QStateMachine_onExit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onExit_0(self);
    // return 1;
  }
}
pub trait QStateMachine_onExit_0<RetType> {
  fn onExit_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_onExit_0<(/*void*/)> for (usize) {
  fn onExit_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine6onExitEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:162
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void beginSelectTransitions(QEvent *)

/*

*/
impl /*struct*/ QStateMachine {
  pub fn beginSelectTransitions_0<RetType, T: QStateMachine_beginSelectTransitions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginSelectTransitions_0(self);
    // return 1;
  }
}
pub trait QStateMachine_beginSelectTransitions_0<RetType> {
  fn beginSelectTransitions_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_beginSelectTransitions_0<(/*void*/)> for (usize) {
  fn beginSelectTransitions_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine22beginSelectTransitionsEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:163
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void endSelectTransitions(QEvent *)

/*

*/
impl /*struct*/ QStateMachine {
  pub fn endSelectTransitions_0<RetType, T: QStateMachine_endSelectTransitions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endSelectTransitions_0(self);
    // return 1;
  }
}
pub trait QStateMachine_endSelectTransitions_0<RetType> {
  fn endSelectTransitions_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_endSelectTransitions_0<(/*void*/)> for (usize) {
  fn endSelectTransitions_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine20endSelectTransitionsEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:165
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void beginMicrostep(QEvent *)

/*

*/
impl /*struct*/ QStateMachine {
  pub fn beginMicrostep_0<RetType, T: QStateMachine_beginMicrostep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginMicrostep_0(self);
    // return 1;
  }
}
pub trait QStateMachine_beginMicrostep_0<RetType> {
  fn beginMicrostep_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_beginMicrostep_0<(/*void*/)> for (usize) {
  fn beginMicrostep_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine14beginMicrostepEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:166
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void endMicrostep(QEvent *)

/*

*/
impl /*struct*/ QStateMachine {
  pub fn endMicrostep_0<RetType, T: QStateMachine_endMicrostep_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endMicrostep_0(self);
    // return 1;
  }
}
pub trait QStateMachine_endMicrostep_0<RetType> {
  fn endMicrostep_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_endMicrostep_0<(/*void*/)> for (usize) {
  fn endMicrostep_0(self , rsthis: & QStateMachine) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QStateMachine12endMicrostepEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstatemachine.h:168
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QStateMachine {
  pub fn event_0<RetType, T: QStateMachine_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QStateMachine_event_0<RetType> {
  fn event_0(self , rsthis: & QStateMachine) -> RetType;
}
impl<'a> /*trait*/ QStateMachine_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QStateMachine) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QStateMachine5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum type specifies the priority of an event posted to the state machine using postEvent().

Events of high priority are processed before events of normal priority.


*/
pub type QStateMachine__EventPriority = i32;
// The event has normal priority.
pub const QStateMachine__NormalPriority :QStateMachine__EventPriority = 0;
// The event has high priority.
pub const QStateMachine__HighPriority :QStateMachine__EventPriority = 1;
pub fn QStateMachine_EventPriorityItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStateMachine", val);
}
pub fn QStateMachine_EventPriorityItemName_s(val: i32) ->String {
  //var nilthis *QStateMachine
  //return nilthis.EventPriorityItemName(val);
  return QStateMachine_EventPriorityItemName(val);
}


/*
This enum type defines errors that can occur in the state machine at run time. When the state machine encounters an unrecoverable error at run time, it will set the error code returned by error(), the error message returned by errorString(), and enter an error state based on the context of the error.



See also setErrorState().

*/
pub type QStateMachine__Error = i32;
// No error has occurred.
pub const QStateMachine__NoError :QStateMachine__Error = 0;
// The machine has entered a QState with children which does not have an initial state set. The context of this error is the state which is missing an initial state.
pub const QStateMachine__NoInitialStateError :QStateMachine__Error = 1;
// The machine has entered a QHistoryState which does not have a default state set. The context of this error is the QHistoryState which is missing a default state.
pub const QStateMachine__NoDefaultStateInHistoryStateError :QStateMachine__Error = 2;
// The machine has selected a transition whose source and targets are not part of the same tree of states, and thus are not part of the same state machine. Commonly, this could mean that one of the states has not been given any parent or added to any machine. The context of this error is the source state of the transition.
pub const QStateMachine__NoCommonAncestorForTransitionError :QStateMachine__Error = 3;
pub fn QStateMachine_ErrorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QStateMachine", val);
}
pub fn QStateMachine_ErrorItemName_s(val: i32) ->String {
  //var nilthis *QStateMachine
  //return nilthis.ErrorItemName(val);
  return QStateMachine_ErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
