

// mod ::gui::QFocusEvent
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
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QFocusEvent)=24
pub struct QFocusEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFocusEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QFocusEvent_PTR() *QFocusEvent
//}
//func (ptr *QFocusEvent) QFocusEvent_PTR() *QFocusEvent { return ptr }

impl /*struct*/ QFocusEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFocusEvent {
    return QFocusEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFocusEvent {
//  type Target = QFocusEventBASE;
//
//  fn deref(&self) -> &QFocusEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFocusEventBASE> for QFocusEvent {
//  fn as_ref(& self) -> & QFocusEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:389
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFocusEvent(QEvent::Type, Qt::FocusReason)

/*

*/
// QFocusEvent(QEvent::Type, Qt::FocusReason) ctx.fn_proto_cpp
impl /*struct*/ QFocusEvent {
  pub fn QFocusEvent_0<T: QFocusEvent_QFocusEvent_0>(value: T) -> QFocusEvent {
    let rsthis = value.QFocusEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFocusEvent_QFocusEvent_0 {
  fn QFocusEvent_0(self) -> QFocusEvent;
}
// QFocusEvent(QEvent::Type, Qt::FocusReason) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFocusEvent_QFocusEvent_0 for (i32,i32) {
  fn QFocusEvent_0(self) -> QFocusEvent {
    // unsafe{_ZN11QFocusEventC2EN6QEvent4TypeEN2Qt11FocusReasonE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFocusEventC2EN6QEvent4TypeEN2Qt11FocusReasonE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFocusEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:390
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFocusEvent()

/*

*/
pub fn DeleteQFocusEvent(this :*mut QFocusEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFocusEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:392
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool gotFocus() const

/*

*/
impl /*struct*/ QFocusEvent {
  pub fn gotFocus_0<RetType, T: QFocusEvent_gotFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.gotFocus_0(self);
    // return 1;
  }
}
pub trait QFocusEvent_gotFocus_0<RetType> {
  fn gotFocus_0(self , rsthis: & QFocusEvent) -> RetType;
}
impl<'a> /*trait*/ QFocusEvent_gotFocus_0<bool> for () {
  fn gotFocus_0(self , rsthis: & QFocusEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFocusEvent8gotFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:393
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool lostFocus() const

/*

*/
impl /*struct*/ QFocusEvent {
  pub fn lostFocus_0<RetType, T: QFocusEvent_lostFocus_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lostFocus_0(self);
    // return 1;
  }
}
pub trait QFocusEvent_lostFocus_0<RetType> {
  fn lostFocus_0(self , rsthis: & QFocusEvent) -> RetType;
}
impl<'a> /*trait*/ QFocusEvent_lostFocus_0<bool> for () {
  fn lostFocus_0(self , rsthis: & QFocusEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFocusEvent9lostFocusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:395
// index:0
// Public Visibility=Default Availability=Available
// [4] Qt::FocusReason reason() const

/*

*/
impl /*struct*/ QFocusEvent {
  pub fn reason_0<RetType, T: QFocusEvent_reason_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reason_0(self);
    // return 1;
  }
}
pub trait QFocusEvent_reason_0<RetType> {
  fn reason_0(self , rsthis: & QFocusEvent) -> RetType;
}
impl<'a> /*trait*/ QFocusEvent_reason_0<i32> for () {
  fn reason_0(self , rsthis: & QFocusEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFocusEvent6reasonEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
