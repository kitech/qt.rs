

// mod ::core::QHistoryState
// package qtcore
// /usr/include/qt/QtCore/qhistorystate.h
// #include <qhistorystate.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 33
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
// func (this *QHistoryState) InheritOnEntry(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onEntry", f)
// }

// void onExit(QEvent *)
// func (this *QHistoryState) InheritOnExit(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onExit", f)
// }

// bool event(QEvent *)
// func (this *QHistoryState) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QHistoryState)=16
pub struct QHistoryState {
  qbase: QAbstractState,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QHistoryState_ITF interface {
//    QAbstractState_ITF
//    QHistoryState_PTR() *QHistoryState
//}
//func (ptr *QHistoryState) QHistoryState_PTR() *QHistoryState { return ptr }

impl /*struct*/ QHistoryState {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QHistoryState {
    return QHistoryState{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QHistoryState {
//  type Target = QHistoryStateBASE;
//
//  fn deref(&self) -> &QHistoryStateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QHistoryStateBASE> for QHistoryState {
//  fn as_ref(& self) -> & QHistoryStateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qhistorystate.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QHistoryState {
  pub fn metaObject_0<RetType, T: QHistoryState_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QHistoryState_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QHistoryState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QHistoryState10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QHistoryState(QState *)

/*
Constructs a new shallow history state with the given parent state.
*/
// QHistoryState(QState *) ctx.fn_proto_cpp
impl /*struct*/ QHistoryState {
  pub fn QHistoryState_0<T: QHistoryState_QHistoryState_0>(value: T) -> QHistoryState {
    let rsthis = value.QHistoryState_0();
    return rsthis;
    // return 1;
  }
}

pub trait QHistoryState_QHistoryState_0 {
  fn QHistoryState_0(self) -> QHistoryState;
}
// QHistoryState(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QHistoryState_QHistoryState_0 for (usize) {
  fn QHistoryState_0(self) -> QHistoryState {
    // unsafe{_ZN13QHistoryStateC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QHistoryStateC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHistoryState{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:65
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QHistoryState(QHistoryState::HistoryType, QState *)

/*
Constructs a new shallow history state with the given parent state.
*/
// QHistoryState(QHistoryState::HistoryType, QState *) ctx.fn_proto_cpp
impl /*struct*/ QHistoryState {
  pub fn QHistoryState_1<T: QHistoryState_QHistoryState_1>(value: T) -> QHistoryState {
    let rsthis = value.QHistoryState_1();
    return rsthis;
    // return 1;
  }
}

pub trait QHistoryState_QHistoryState_1 {
  fn QHistoryState_1(self) -> QHistoryState;
}
// QHistoryState(QHistoryState::HistoryType, QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QHistoryState_QHistoryState_1 for (i32,usize) {
  fn QHistoryState_1(self) -> QHistoryState {
    // unsafe{_ZN13QHistoryStateC2ENS_11HistoryTypeEP6QState()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QHistoryStateC2ENS_11HistoryTypeEP6QState", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QHistoryState{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QHistoryState()

/*

*/
pub fn DeleteQHistoryState(this :*mut QHistoryState) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QHistoryStateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qhistorystate.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractTransition * defaultTransition() const

/*
Returns this history state's default transition. The default transition is taken when the history state has never been entered before. The target states of the default transition therefore make up the default state.

This function was introduced in  Qt 5.6.

Note: Getter function for property defaultTransition. 

See also setDefaultTransition().
*/
impl /*struct*/ QHistoryState {
  pub fn defaultTransition_0<RetType, T: QHistoryState_defaultTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultTransition_0(self);
    // return 1;
  }
}
pub trait QHistoryState_defaultTransition_0<RetType> {
  fn defaultTransition_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_defaultTransition_0<usize> for () {
  fn defaultTransition_0(self , rsthis: & QHistoryState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QHistoryState17defaultTransitionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultTransition(QAbstractTransition *)

/*
Sets this history state's default transition to be the given transition. This will set the source state of the transition to the history state.

Note that the eventTest method of the transition will never be called.

This function was introduced in  Qt 5.6.

Note: Setter function for property defaultTransition. 

See also defaultTransition().
*/
impl /*struct*/ QHistoryState {
  pub fn setDefaultTransition_0<RetType, T: QHistoryState_setDefaultTransition_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultTransition_0(self);
    // return 1;
  }
}
pub trait QHistoryState_setDefaultTransition_0<RetType> {
  fn setDefaultTransition_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_setDefaultTransition_0<(/*void*/)> for (usize) {
  fn setDefaultTransition_0(self , rsthis: & QHistoryState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QHistoryState20setDefaultTransitionEP19QAbstractTransition", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:71
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractState * defaultState() const

/*
Returns this history state's default state. The default state indicates the state to transition to if the parent state has never been entered before.

Note: Getter function for property defaultState. 

See also setDefaultState().
*/
impl /*struct*/ QHistoryState {
  pub fn defaultState_0<RetType, T: QHistoryState_defaultState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.defaultState_0(self);
    // return 1;
  }
}
pub trait QHistoryState_defaultState_0<RetType> {
  fn defaultState_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_defaultState_0<usize> for () {
  fn defaultState_0(self , rsthis: & QHistoryState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QHistoryState12defaultStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDefaultState(QAbstractState *)

/*
Sets this history state's default state to be the given state. state must be a sibling of this history state.

Note that this function does not set state as the initial state of its parent.

Note: Setter function for property defaultState. 

See also defaultState().
*/
impl /*struct*/ QHistoryState {
  pub fn setDefaultState_0<RetType, T: QHistoryState_setDefaultState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDefaultState_0(self);
    // return 1;
  }
}
pub trait QHistoryState_setDefaultState_0<RetType> {
  fn setDefaultState_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_setDefaultState_0<(/*void*/)> for (usize) {
  fn setDefaultState_0(self , rsthis: & QHistoryState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QHistoryState15setDefaultStateEP14QAbstractState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:74
// index:0
// Public Visibility=Default Availability=Available
// [4] QHistoryState::HistoryType historyType() const

/*
Returns the type of history that this history state records.

Note: Getter function for property historyType. 

See also setHistoryType().
*/
impl /*struct*/ QHistoryState {
  pub fn historyType_0<RetType, T: QHistoryState_historyType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.historyType_0(self);
    // return 1;
  }
}
pub trait QHistoryState_historyType_0<RetType> {
  fn historyType_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_historyType_0<i32> for () {
  fn historyType_0(self , rsthis: & QHistoryState) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QHistoryState11historyTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setHistoryType(QHistoryState::HistoryType)

/*
Sets the type of history that this history state records.

Note: Setter function for property historyType. 

See also historyType().
*/
impl /*struct*/ QHistoryState {
  pub fn setHistoryType_0<RetType, T: QHistoryState_setHistoryType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setHistoryType_0(self);
    // return 1;
  }
}
pub trait QHistoryState_setHistoryType_0<RetType> {
  fn setHistoryType_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_setHistoryType_0<(/*void*/)> for (i32) {
  fn setHistoryType_0(self , rsthis: & QHistoryState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QHistoryState14setHistoryTypeENS_11HistoryTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:83
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onEntry(QEvent *)

/*
Reimplemented from QAbstractState::onEntry().
*/
impl /*struct*/ QHistoryState {
  pub fn onEntry_0<RetType, T: QHistoryState_onEntry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onEntry_0(self);
    // return 1;
  }
}
pub trait QHistoryState_onEntry_0<RetType> {
  fn onEntry_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_onEntry_0<(/*void*/)> for (usize) {
  fn onEntry_0(self , rsthis: & QHistoryState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QHistoryState7onEntryEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:84
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onExit(QEvent *)

/*
Reimplemented from QAbstractState::onExit().
*/
impl /*struct*/ QHistoryState {
  pub fn onExit_0<RetType, T: QHistoryState_onExit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onExit_0(self);
    // return 1;
  }
}
pub trait QHistoryState_onExit_0<RetType> {
  fn onExit_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_onExit_0<(/*void*/)> for (usize) {
  fn onExit_0(self , rsthis: & QHistoryState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QHistoryState6onExitEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qhistorystate.h:86
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QHistoryState {
  pub fn event_0<RetType, T: QHistoryState_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QHistoryState_event_0<RetType> {
  fn event_0(self , rsthis: & QHistoryState) -> RetType;
}
impl<'a> /*trait*/ QHistoryState_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QHistoryState) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QHistoryState5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum specifies the type of history that a QHistoryState records.


*/
pub type QHistoryState__HistoryType = i32;
// Only the immediate child states of the parent state are recorded. In this case a transition with the history state as its target will end up in the immediate child state that the parent was in the last time it was exited. This is the default.
pub const QHistoryState__ShallowHistory :QHistoryState__HistoryType = 0;
// Nested states are recorded. In this case a transition with the history state as its target will end up in the most deeply nested descendant state the parent was in the last time it was exited.
pub const QHistoryState__DeepHistory :QHistoryState__HistoryType = 1;
pub fn QHistoryState_HistoryTypeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QHistoryState", val);
}
pub fn QHistoryState_HistoryTypeItemName_s(val: i32) ->String {
  //var nilthis *QHistoryState
  //return nilthis.HistoryTypeItemName(val);
  return QHistoryState_HistoryTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
