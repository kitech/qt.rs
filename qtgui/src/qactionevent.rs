

// mod ::gui::QActionEvent
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
// extern C begin: 3
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
#[derive(Default)] // class sizeof(QActionEvent)=40
pub struct QActionEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QActionEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QActionEvent_PTR() *QActionEvent
//}
//func (ptr *QActionEvent) QActionEvent_PTR() *QActionEvent { return ptr }

impl /*struct*/ QActionEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QActionEvent {
    return QActionEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QActionEvent {
//  type Target = QActionEventBASE;
//
//  fn deref(&self) -> &QActionEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QActionEventBASE> for QActionEvent {
//  fn as_ref(& self) -> & QActionEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:727
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QActionEvent(int, QAction *, QAction *)

/*

*/
// QActionEvent(int, QAction *, QAction *) ctx.fn_proto_cpp
impl /*struct*/ QActionEvent {
  pub fn QActionEvent_0<T: QActionEvent_QActionEvent_0>(value: T) -> QActionEvent {
    let rsthis = value.QActionEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QActionEvent_QActionEvent_0 {
  fn QActionEvent_0(self) -> QActionEvent;
}
// QActionEvent(int, QAction *, QAction *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QActionEvent_QActionEvent_0 for (i32,usize,usize) {
  fn QActionEvent_0(self) -> QActionEvent {
    // unsafe{_ZN12QActionEventC2EiP7QActionS1_()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QActionEventC2EiP7QActionS1_", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QActionEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:728
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QActionEvent()

/*

*/
pub fn DeleteQActionEvent(this :*mut QActionEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QActionEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:730
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAction * action() const

/*

*/
impl /*struct*/ QActionEvent {
  pub fn action_0<RetType, T: QActionEvent_action_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.action_0(self);
    // return 1;
  }
}
pub trait QActionEvent_action_0<RetType> {
  fn action_0(self , rsthis: & QActionEvent) -> RetType;
}
impl<'a> /*trait*/ QActionEvent_action_0<usize> for () {
  fn action_0(self , rsthis: & QActionEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionEvent6actionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:731
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QAction * before() const

/*

*/
impl /*struct*/ QActionEvent {
  pub fn before_0<RetType, T: QActionEvent_before_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.before_0(self);
    // return 1;
  }
}
pub trait QActionEvent_before_0<RetType> {
  fn before_0(self , rsthis: & QActionEvent) -> RetType;
}
impl<'a> /*trait*/ QActionEvent_before_0<usize> for () {
  fn before_0(self , rsthis: & QActionEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QActionEvent6beforeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
