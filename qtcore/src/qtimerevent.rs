

// mod ::core::QTimerEvent
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



/*

*/
#[derive(Default)] // class sizeof(QTimerEvent)=24
pub struct QTimerEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTimerEvent_ITF interface {
//    QEvent_ITF
//    QTimerEvent_PTR() *QTimerEvent
//}
//func (ptr *QTimerEvent) QTimerEvent_PTR() *QTimerEvent { return ptr }

impl /*struct*/ QTimerEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTimerEvent {
    return QTimerEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTimerEvent {
//  type Target = QTimerEventBASE;
//
//  fn deref(&self) -> &QTimerEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTimerEventBASE> for QTimerEvent {
//  fn as_ref(& self) -> & QTimerEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcoreevent.h:340
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTimerEvent(int)

/*

*/
// QTimerEvent(int) ctx.fn_proto_cpp
impl /*struct*/ QTimerEvent {
  pub fn QTimerEvent_0<T: QTimerEvent_QTimerEvent_0>(value: T) -> QTimerEvent {
    let rsthis = value.QTimerEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTimerEvent_QTimerEvent_0 {
  fn QTimerEvent_0(self) -> QTimerEvent;
}
// QTimerEvent(int) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimerEvent_QTimerEvent_0 for (i32) {
  fn QTimerEvent_0(self) -> QTimerEvent {
    // unsafe{_ZN11QTimerEventC2Ei()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QTimerEventC2Ei", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimerEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:341
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTimerEvent()

/*

*/
pub fn DeleteQTimerEvent(this :*mut QTimerEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTimerEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcoreevent.h:342
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int timerId() const

/*

*/
impl /*struct*/ QTimerEvent {
  pub fn timerId_0<RetType, T: QTimerEvent_timerId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerId_0(self);
    // return 1;
  }
}
pub trait QTimerEvent_timerId_0<RetType> {
  fn timerId_0(self , rsthis: & QTimerEvent) -> RetType;
}
impl<'a> /*trait*/ QTimerEvent_timerId_0<i32> for () {
  fn timerId_0(self , rsthis: & QTimerEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTimerEvent7timerIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
