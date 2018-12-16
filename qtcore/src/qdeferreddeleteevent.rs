

// mod ::core::QDeferredDeleteEvent
// package qtcore
// /usr/include/qt/QtCore/qcoreevent.h
// #include <qcoreevent.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QDeferredDeleteEvent)=24
pub struct QDeferredDeleteEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDeferredDeleteEvent_ITF interface {
//    QEvent_ITF
//    QDeferredDeleteEvent_PTR() *QDeferredDeleteEvent
//}
//func (ptr *QDeferredDeleteEvent) QDeferredDeleteEvent_PTR() *QDeferredDeleteEvent { return ptr }

impl /*struct*/ QDeferredDeleteEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDeferredDeleteEvent {
    return QDeferredDeleteEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDeferredDeleteEvent {
//  type Target = QDeferredDeleteEventBASE;
//
//  fn deref(&self) -> &QDeferredDeleteEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDeferredDeleteEventBASE> for QDeferredDeleteEvent {
//  fn as_ref(& self) -> & QDeferredDeleteEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcoreevent.h:377
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDeferredDeleteEvent()

/*

*/
// QDeferredDeleteEvent() ctx.fn_proto_cpp
impl /*struct*/ QDeferredDeleteEvent {
  pub fn QDeferredDeleteEvent_0<T: QDeferredDeleteEvent_QDeferredDeleteEvent_0>(value: T) -> QDeferredDeleteEvent {
    let rsthis = value.QDeferredDeleteEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDeferredDeleteEvent_QDeferredDeleteEvent_0 {
  fn QDeferredDeleteEvent_0(self) -> QDeferredDeleteEvent;
}
// QDeferredDeleteEvent() ctx.fn_proto_cpp
impl<'a> /*trait*/ QDeferredDeleteEvent_QDeferredDeleteEvent_0 for () {
  fn QDeferredDeleteEvent_0(self) -> QDeferredDeleteEvent {
    // unsafe{_ZN20QDeferredDeleteEventC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QDeferredDeleteEventC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDeferredDeleteEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:378
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QDeferredDeleteEvent()

/*

*/
pub fn DeleteQDeferredDeleteEvent(this :*mut QDeferredDeleteEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QDeferredDeleteEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcoreevent.h:379
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int loopLevel() const

/*

*/
impl /*struct*/ QDeferredDeleteEvent {
  pub fn loopLevel_0<RetType, T: QDeferredDeleteEvent_loopLevel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopLevel_0(self);
    // return 1;
  }
}
pub trait QDeferredDeleteEvent_loopLevel_0<RetType> {
  fn loopLevel_0(self , rsthis: & QDeferredDeleteEvent) -> RetType;
}
impl<'a> /*trait*/ QDeferredDeleteEvent_loopLevel_0<i32> for () {
  fn loopLevel_0(self , rsthis: & QDeferredDeleteEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QDeferredDeleteEvent9loopLevelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
