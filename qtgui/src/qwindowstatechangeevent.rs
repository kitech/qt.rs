

// mod ::gui::QWindowStateChangeEvent
// package qtgui
// /usr/include/qt/QtGui/qevent.h
// #include <qevent.h>
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
#[derive(Default)] // class sizeof(QWindowStateChangeEvent)=32
pub struct QWindowStateChangeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWindowStateChangeEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QWindowStateChangeEvent_PTR() *QWindowStateChangeEvent
//}
//func (ptr *QWindowStateChangeEvent) QWindowStateChangeEvent_PTR() *QWindowStateChangeEvent { return ptr }

impl /*struct*/ QWindowStateChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWindowStateChangeEvent {
    return QWindowStateChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWindowStateChangeEvent {
//  type Target = QWindowStateChangeEventBASE;
//
//  fn deref(&self) -> &QWindowStateChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWindowStateChangeEventBASE> for QWindowStateChangeEvent {
//  fn as_ref(& self) -> & QWindowStateChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:783
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWindowStateChangeEvent(Qt::WindowStates, bool)

/*

*/
// QWindowStateChangeEvent(Qt::WindowStates, bool) ctx.fn_proto_cpp
impl /*struct*/ QWindowStateChangeEvent {
  pub fn QWindowStateChangeEvent_0<T: QWindowStateChangeEvent_QWindowStateChangeEvent_0>(value: T) -> QWindowStateChangeEvent {
    let rsthis = value.QWindowStateChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWindowStateChangeEvent_QWindowStateChangeEvent_0 {
  fn QWindowStateChangeEvent_0(self) -> QWindowStateChangeEvent;
}
// QWindowStateChangeEvent(Qt::WindowStates, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWindowStateChangeEvent_QWindowStateChangeEvent_0 for (i32,bool) {
  fn QWindowStateChangeEvent_0(self) -> QWindowStateChangeEvent {
    // unsafe{_ZN23QWindowStateChangeEventC2E6QFlagsIN2Qt11WindowStateEEb()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN23QWindowStateChangeEventC2E6QFlagsIN2Qt11WindowStateEEb", 2,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWindowStateChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:784
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QWindowStateChangeEvent()

/*

*/
pub fn DeleteQWindowStateChangeEvent(this :*mut QWindowStateChangeEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN23QWindowStateChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:786
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::WindowStates oldState() const

/*

*/
impl /*struct*/ QWindowStateChangeEvent {
  pub fn oldState_0<RetType, T: QWindowStateChangeEvent_oldState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldState_0(self);
    // return 1;
  }
}
pub trait QWindowStateChangeEvent_oldState_0<RetType> {
  fn oldState_0(self , rsthis: & QWindowStateChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QWindowStateChangeEvent_oldState_0<i32> for () {
  fn oldState_0(self , rsthis: & QWindowStateChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QWindowStateChangeEvent8oldStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:787
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isOverride() const

/*

*/
impl /*struct*/ QWindowStateChangeEvent {
  pub fn isOverride_0<RetType, T: QWindowStateChangeEvent_isOverride_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isOverride_0(self);
    // return 1;
  }
}
pub trait QWindowStateChangeEvent_isOverride_0<RetType> {
  fn isOverride_0(self , rsthis: & QWindowStateChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QWindowStateChangeEvent_isOverride_0<bool> for () {
  fn isOverride_0(self , rsthis: & QWindowStateChangeEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK23QWindowStateChangeEvent10isOverrideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
