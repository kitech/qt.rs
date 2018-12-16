

// mod ::gui::QInputEvent
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
// extern C begin: 10
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
#[derive(Default)] // class sizeof(QInputEvent)=32
pub struct QInputEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QInputEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QInputEvent_PTR() *QInputEvent
//}
//func (ptr *QInputEvent) QInputEvent_PTR() *QInputEvent { return ptr }

impl /*struct*/ QInputEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QInputEvent {
    return QInputEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QInputEvent {
//  type Target = QInputEventBASE;
//
//  fn deref(&self) -> &QInputEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QInputEventBASE> for QInputEvent {
//  fn as_ref(& self) -> & QInputEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QInputEvent(QEvent::Type, Qt::KeyboardModifiers)

/*

*/
// QInputEvent(QEvent::Type, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl /*struct*/ QInputEvent {
  pub fn QInputEvent_0<T: QInputEvent_QInputEvent_0>(value: T) -> QInputEvent {
    let rsthis = value.QInputEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QInputEvent_QInputEvent_0 {
  fn QInputEvent_0(self) -> QInputEvent;
}
// QInputEvent(QEvent::Type, Qt::KeyboardModifiers) ctx.fn_proto_cpp
impl<'a> /*trait*/ QInputEvent_QInputEvent_0 for (i32,i32) {
  fn QInputEvent_0(self) -> QInputEvent {
    // unsafe{_ZN11QInputEventC2EN6QEvent4TypeE6QFlagsIN2Qt16KeyboardModifierEE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QInputEventC2EN6QEvent4TypeE6QFlagsIN2Qt16KeyboardModifierEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QInputEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QInputEvent()

/*

*/
pub fn DeleteQInputEvent(this :*mut QInputEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QInputEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::KeyboardModifiers modifiers() const

/*

*/
impl /*struct*/ QInputEvent {
  pub fn modifiers_0<RetType, T: QInputEvent_modifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.modifiers_0(self);
    // return 1;
  }
}
pub trait QInputEvent_modifiers_0<RetType> {
  fn modifiers_0(self , rsthis: & QInputEvent) -> RetType;
}
impl<'a> /*trait*/ QInputEvent_modifiers_0<i32> for () {
  fn modifiers_0(self , rsthis: & QInputEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QInputEvent9modifiersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setModifiers(Qt::KeyboardModifiers)

/*

*/
impl /*struct*/ QInputEvent {
  pub fn setModifiers_0<RetType, T: QInputEvent_setModifiers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setModifiers_0(self);
    // return 1;
  }
}
pub trait QInputEvent_setModifiers_0<RetType> {
  fn setModifiers_0(self , rsthis: & QInputEvent) -> RetType;
}
impl<'a> /*trait*/ QInputEvent_setModifiers_0<(/*void*/)> for (i32) {
  fn setModifiers_0(self , rsthis: & QInputEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QInputEvent12setModifiersE6QFlagsIN2Qt16KeyboardModifierEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:75
// index:0
// Public inline Visibility=Default Availability=Available
// [8] ulong timestamp() const

/*

*/
impl /*struct*/ QInputEvent {
  pub fn timestamp_0<RetType, T: QInputEvent_timestamp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timestamp_0(self);
    // return 1;
  }
}
pub trait QInputEvent_timestamp_0<RetType> {
  fn timestamp_0(self , rsthis: & QInputEvent) -> RetType;
}
impl<'a> /*trait*/ QInputEvent_timestamp_0<u64> for () {
  fn timestamp_0(self , rsthis: & QInputEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QInputEvent9timestampEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:76
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTimestamp(ulong)

/*

*/
impl /*struct*/ QInputEvent {
  pub fn setTimestamp_0<RetType, T: QInputEvent_setTimestamp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimestamp_0(self);
    // return 1;
  }
}
pub trait QInputEvent_setTimestamp_0<RetType> {
  fn setTimestamp_0(self , rsthis: & QInputEvent) -> RetType;
}
impl<'a> /*trait*/ QInputEvent_setTimestamp_0<(/*void*/)> for (u64) {
  fn setTimestamp_0(self , rsthis: & QInputEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
     qtrt::InvokeQtFunc6("_ZN11QInputEvent12setTimestampEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
