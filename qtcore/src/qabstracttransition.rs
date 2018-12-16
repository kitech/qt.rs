

// mod ::core::QAbstractTransition
// package qtcore
// /usr/include/qt/QtCore/qabstracttransition.h
// #include <qabstracttransition.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 10
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
// func (this *QAbstractTransition) InheritEventTest(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "eventTest", f)
// }

// void onTransition(QEvent *)
// func (this *QAbstractTransition) InheritOnTransition(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onTransition", f)
// }

// bool event(QEvent *)
// func (this *QAbstractTransition) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAbstractTransition)=16
pub struct QAbstractTransition {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractTransition_ITF interface {
//    QObject_ITF
//    QAbstractTransition_PTR() *QAbstractTransition
//}
//func (ptr *QAbstractTransition) QAbstractTransition_PTR() *QAbstractTransition { return ptr }

impl /*struct*/ QAbstractTransition {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractTransition {
    return QAbstractTransition{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractTransition {
//  type Target = QAbstractTransitionBASE;
//
//  fn deref(&self) -> &QAbstractTransitionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractTransitionBASE> for QAbstractTransition {
//  fn as_ref(& self) -> & QAbstractTransitionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstracttransition.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractTransition {
  pub fn metaObject_0<RetType, T: QAbstractTransition_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTransition10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractTransition(QState *)

/*
Constructs a new QAbstractTransition object with the given sourceState.
*/
// QAbstractTransition(QState *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractTransition {
  pub fn QAbstractTransition_0<T: QAbstractTransition_QAbstractTransition_0>(value: T) -> QAbstractTransition {
    let rsthis = value.QAbstractTransition_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTransition_QAbstractTransition_0 {
  fn QAbstractTransition_0(self) -> QAbstractTransition;
}
// QAbstractTransition(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractTransition_QAbstractTransition_0 for (usize) {
  fn QAbstractTransition_0(self) -> QAbstractTransition {
    // unsafe{_ZN19QAbstractTransitionC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QAbstractTransitionC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractTransition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:76
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractTransition()

/*

*/
pub fn DeleteQAbstractTransition(this :*mut QAbstractTransition) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QAbstractTransitionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstracttransition.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QState * sourceState() const

/*
Returns the source state of this transition, or 0 if this transition has no source state.

Note: Getter function for property sourceState.
*/
impl /*struct*/ QAbstractTransition {
  pub fn sourceState_0<RetType, T: QAbstractTransition_sourceState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sourceState_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_sourceState_0<RetType> {
  fn sourceState_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_sourceState_0<usize> for () {
  fn sourceState_0(self , rsthis: & QAbstractTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTransition11sourceStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractState * targetState() const

/*
Returns the target state of this transition, or 0 if the transition has no target.

Note: Getter function for property targetState. 

See also setTargetState().
*/
impl /*struct*/ QAbstractTransition {
  pub fn targetState_0<RetType, T: QAbstractTransition_targetState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.targetState_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_targetState_0<RetType> {
  fn targetState_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_targetState_0<usize> for () {
  fn targetState_0(self , rsthis: & QAbstractTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTransition11targetStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTargetState(QAbstractState *)

/*
Sets the target state of this transition.

Note: Setter function for property targetState. 

See also targetState().
*/
impl /*struct*/ QAbstractTransition {
  pub fn setTargetState_0<RetType, T: QAbstractTransition_setTargetState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTargetState_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_setTargetState_0<RetType> {
  fn setTargetState_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_setTargetState_0<(/*void*/)> for (usize) {
  fn setTargetState_0(self , rsthis: & QAbstractTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractTransition14setTargetStateEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] QAbstractTransition::TransitionType transitionType() const

/*
Returns the type of the transition.

Note: Getter function for property transitionType. 

See also setTransitionType().
*/
impl /*struct*/ QAbstractTransition {
  pub fn transitionType_0<RetType, T: QAbstractTransition_transitionType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.transitionType_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_transitionType_0<RetType> {
  fn transitionType_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_transitionType_0<i32> for () {
  fn transitionType_0(self , rsthis: & QAbstractTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTransition14transitionTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTransitionType(QAbstractTransition::TransitionType)

/*
Sets the type of the transition to type.

Note: Setter function for property transitionType. 

See also transitionType().
*/
impl /*struct*/ QAbstractTransition {
  pub fn setTransitionType_0<RetType, T: QAbstractTransition_setTransitionType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTransitionType_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_setTransitionType_0<RetType> {
  fn setTransitionType_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_setTransitionType_0<(/*void*/)> for (i32) {
  fn setTransitionType_0(self , rsthis: & QAbstractTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractTransition17setTransitionTypeENS_14TransitionTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] QStateMachine * machine() const

/*
Returns the state machine that this transition is part of, or 0 if the transition is not part of a state machine.
*/
impl /*struct*/ QAbstractTransition {
  pub fn machine_0<RetType, T: QAbstractTransition_machine_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.machine_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_machine_0<RetType> {
  fn machine_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_machine_0<usize> for () {
  fn machine_0(self , rsthis: & QAbstractTransition) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QAbstractTransition7machineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void addAnimation(QAbstractAnimation *)

/*
Adds the given animation to this transition. The transition does not take ownership of the animation.

See also removeAnimation() and animations().
*/
impl /*struct*/ QAbstractTransition {
  pub fn addAnimation_0<RetType, T: QAbstractTransition_addAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addAnimation_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_addAnimation_0<RetType> {
  fn addAnimation_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_addAnimation_0<(/*void*/)> for (usize) {
  fn addAnimation_0(self , rsthis: & QAbstractTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractTransition12addAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeAnimation(QAbstractAnimation *)

/*
Removes the given animation from this transition.

See also addAnimation().
*/
impl /*struct*/ QAbstractTransition {
  pub fn removeAnimation_0<RetType, T: QAbstractTransition_removeAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeAnimation_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_removeAnimation_0<RetType> {
  fn removeAnimation_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_removeAnimation_0<(/*void*/)> for (usize) {
  fn removeAnimation_0(self , rsthis: & QAbstractTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractTransition15removeAnimationEP18QAbstractAnimation", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:101
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [1] bool eventTest(QEvent *)

/*
This function is called to determine whether the given event should cause this transition to trigger. Reimplement this function and return true if the event should trigger the transition, otherwise return false.
*/
impl /*struct*/ QAbstractTransition {
  pub fn eventTest_0<RetType, T: QAbstractTransition_eventTest_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventTest_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_eventTest_0<RetType> {
  fn eventTest_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_eventTest_0<bool> for (usize) {
  fn eventTest_0(self , rsthis: & QAbstractTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractTransition9eventTestEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:103
// index:0
// Protected purevirtual virtual Visibility=Default Availability=Available
// [-2] void onTransition(QEvent *)

/*
This function is called when the transition is triggered. The given event is what caused the transition to trigger. Reimplement this function to perform custom processing when the transition is triggered.
*/
impl /*struct*/ QAbstractTransition {
  pub fn onTransition_0<RetType, T: QAbstractTransition_onTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onTransition_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_onTransition_0<RetType> {
  fn onTransition_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_onTransition_0<(/*void*/)> for (usize) {
  fn onTransition_0(self , rsthis: & QAbstractTransition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QAbstractTransition12onTransitionEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracttransition.h:105
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QAbstractTransition {
  pub fn event_0<RetType, T: QAbstractTransition_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QAbstractTransition_event_0<RetType> {
  fn event_0(self , rsthis: & QAbstractTransition) -> RetType;
}
impl<'a> /*trait*/ QAbstractTransition_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QAbstractTransition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN19QAbstractTransition5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum specifies the kind of transition. By default, the type is an external transition.



See also QAbstractTransition::transitionType.

*/
pub type QAbstractTransition__TransitionType = i32;
// Any state that is the source state of a transition (which is not a target-less transition) is left, and re-entered when necessary.
pub const QAbstractTransition__ExternalTransition :QAbstractTransition__TransitionType = 0;
// If the target state of a transition is a sub-state of a compound state, and that compound state is the source state, an internal transition will not leave the source state.
pub const QAbstractTransition__InternalTransition :QAbstractTransition__TransitionType = 1;
pub fn QAbstractTransition_TransitionTypeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QAbstractTransition", val);
}
pub fn QAbstractTransition_TransitionTypeItemName_s(val: i32) ->String {
  //var nilthis *QAbstractTransition
  //return nilthis.TransitionTypeItemName(val);
  return QAbstractTransition_TransitionTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
