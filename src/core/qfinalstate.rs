

// mod ::core::QFinalState
// package qtcore
// /usr/include/qt/QtCore/qfinalstate.h
// #include <qfinalstate.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// void onEntry(QEvent *)
// func (this *QFinalState) InheritOnEntry(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onEntry", f)
// }

// void onExit(QEvent *)
// func (this *QFinalState) InheritOnExit(f func(event *QEvent/*777 QEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "onExit", f)
// }

// bool event(QEvent *)
// func (this *QFinalState) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFinalState)=16
pub struct QFinalState {
  qbase: QAbstractState,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFinalState_ITF interface {
//    QAbstractState_ITF
//    QFinalState_PTR() *QFinalState
//}
//func (ptr *QFinalState) QFinalState_PTR() *QFinalState { return ptr }

impl /*struct*/ QFinalState {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFinalState {
    return QFinalState{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFinalState {
//  type Target = QFinalStateBASE;
//
//  fn deref(&self) -> &QFinalStateBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFinalStateBASE> for QFinalState {
//  fn as_ref(& self) -> & QFinalStateBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfinalstate.h:52
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFinalState {
  pub fn metaObject_0<RetType, T: QFinalState_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFinalState_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFinalState) -> RetType;
}
impl<'a> /*trait*/ QFinalState_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFinalState) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFinalState10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfinalstate.h:54
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFinalState(QState *)

/*
Constructs a new QFinalState object with the given parent state.
*/
// QFinalState(QState *) ctx.fn_proto_cpp
impl /*struct*/ QFinalState {
  pub fn QFinalState_0<T: QFinalState_QFinalState_0>(value: T) -> QFinalState {
    let rsthis = value.QFinalState_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFinalState_QFinalState_0 {
  fn QFinalState_0(self) -> QFinalState;
}
// QFinalState(QState *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFinalState_QFinalState_0 for (usize) {
  fn QFinalState_0(self) -> QFinalState {
    // unsafe{_ZN11QFinalStateC2EP6QState()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFinalStateC2EP6QState", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFinalState{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfinalstate.h:55
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFinalState()

/*

*/
pub fn DeleteQFinalState(this :*mut QFinalState) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFinalStateD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfinalstate.h:58
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onEntry(QEvent *)

/*
Reimplemented from QAbstractState::onEntry().
*/
impl /*struct*/ QFinalState {
  pub fn onEntry_0<RetType, T: QFinalState_onEntry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onEntry_0(self);
    // return 1;
  }
}
pub trait QFinalState_onEntry_0<RetType> {
  fn onEntry_0(self , rsthis: & QFinalState) -> RetType;
}
impl<'a> /*trait*/ QFinalState_onEntry_0<(/*void*/)> for (usize) {
  fn onEntry_0(self , rsthis: & QFinalState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFinalState7onEntryEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfinalstate.h:59
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void onExit(QEvent *)

/*
Reimplemented from QAbstractState::onExit().
*/
impl /*struct*/ QFinalState {
  pub fn onExit_0<RetType, T: QFinalState_onExit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.onExit_0(self);
    // return 1;
  }
}
pub trait QFinalState_onExit_0<RetType> {
  fn onExit_0(self , rsthis: & QFinalState) -> RetType;
}
impl<'a> /*trait*/ QFinalState_onExit_0<(/*void*/)> for (usize) {
  fn onExit_0(self , rsthis: & QFinalState) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QFinalState6onExitEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfinalstate.h:61
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QFinalState {
  pub fn event_0<RetType, T: QFinalState_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QFinalState_event_0<RetType> {
  fn event_0(self , rsthis: & QFinalState) -> RetType;
}
impl<'a> /*trait*/ QFinalState_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QFinalState) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFinalState5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
