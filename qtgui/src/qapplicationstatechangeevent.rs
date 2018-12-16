

// mod ::gui::QApplicationStateChangeEvent
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
// extern C begin: 4
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
#[derive(Default)] // class sizeof(QApplicationStateChangeEvent)=24
pub struct QApplicationStateChangeEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QApplicationStateChangeEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QApplicationStateChangeEvent_PTR() *QApplicationStateChangeEvent
//}
//func (ptr *QApplicationStateChangeEvent) QApplicationStateChangeEvent_PTR() *QApplicationStateChangeEvent { return ptr }

impl /*struct*/ QApplicationStateChangeEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QApplicationStateChangeEvent {
    return QApplicationStateChangeEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QApplicationStateChangeEvent {
//  type Target = QApplicationStateChangeEventBASE;
//
//  fn deref(&self) -> &QApplicationStateChangeEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QApplicationStateChangeEventBASE> for QApplicationStateChangeEvent {
//  fn as_ref(& self) -> & QApplicationStateChangeEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:1052
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QApplicationStateChangeEvent(Qt::ApplicationState)

/*

*/
// QApplicationStateChangeEvent(Qt::ApplicationState) ctx.fn_proto_cpp
impl /*struct*/ QApplicationStateChangeEvent {
  pub fn QApplicationStateChangeEvent_0<T: QApplicationStateChangeEvent_QApplicationStateChangeEvent_0>(value: T) -> QApplicationStateChangeEvent {
    let rsthis = value.QApplicationStateChangeEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QApplicationStateChangeEvent_QApplicationStateChangeEvent_0 {
  fn QApplicationStateChangeEvent_0(self) -> QApplicationStateChangeEvent;
}
// QApplicationStateChangeEvent(Qt::ApplicationState) ctx.fn_proto_cpp
impl<'a> /*trait*/ QApplicationStateChangeEvent_QApplicationStateChangeEvent_0 for (i32) {
  fn QApplicationStateChangeEvent_0(self) -> QApplicationStateChangeEvent {
    // unsafe{_ZN28QApplicationStateChangeEventC2EN2Qt16ApplicationStateE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN28QApplicationStateChangeEventC2EN2Qt16ApplicationStateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QApplicationStateChangeEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:1053
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::ApplicationState applicationState() const

/*

*/
impl /*struct*/ QApplicationStateChangeEvent {
  pub fn applicationState_0<RetType, T: QApplicationStateChangeEvent_applicationState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationState_0(self);
    // return 1;
  }
}
pub trait QApplicationStateChangeEvent_applicationState_0<RetType> {
  fn applicationState_0(self , rsthis: & QApplicationStateChangeEvent) -> RetType;
}
impl<'a> /*trait*/ QApplicationStateChangeEvent_applicationState_0<i32> for () {
  fn applicationState_0(self , rsthis: & QApplicationStateChangeEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK28QApplicationStateChangeEvent16applicationStateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQApplicationStateChangeEvent(this :*mut QApplicationStateChangeEvent) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN28QApplicationStateChangeEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
