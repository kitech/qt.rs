

// mod ::gui::QShortcutEvent
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
#[derive(Default)] // class sizeof(QShortcutEvent)=40
pub struct QShortcutEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QShortcutEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QShortcutEvent_PTR() *QShortcutEvent
//}
//func (ptr *QShortcutEvent) QShortcutEvent_PTR() *QShortcutEvent { return ptr }

impl /*struct*/ QShortcutEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QShortcutEvent {
    return QShortcutEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QShortcutEvent {
//  type Target = QShortcutEventBASE;
//
//  fn deref(&self) -> &QShortcutEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QShortcutEventBASE> for QShortcutEvent {
//  fn as_ref(& self) -> & QShortcutEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:767
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QShortcutEvent(const QKeySequence &, int, bool)

/*

*/
// QShortcutEvent(const QKeySequence &, int, bool) ctx.fn_proto_cpp
impl /*struct*/ QShortcutEvent {
  pub fn QShortcutEvent_0<T: QShortcutEvent_QShortcutEvent_0>(value: T) -> QShortcutEvent {
    let rsthis = value.QShortcutEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcutEvent_QShortcutEvent_0 {
  fn QShortcutEvent_0(self) -> QShortcutEvent;
}
// QShortcutEvent(const QKeySequence &, int, bool) ctx.fn_proto_cpp
impl<'a> /*trait*/ QShortcutEvent_QShortcutEvent_0 for (usize,i32,bool) {
  fn QShortcutEvent_0(self) -> QShortcutEvent {
    // unsafe{_ZN14QShortcutEventC2ERK12QKeySequenceib()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const bool as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QShortcutEventC2ERK12QKeySequenceib", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QShortcutEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:768
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QShortcutEvent()

/*

*/
pub fn DeleteQShortcutEvent(this :*mut QShortcutEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QShortcutEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:770
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QKeySequence & key() const

/*

*/
impl /*struct*/ QShortcutEvent {
  pub fn key_0<RetType, T: QShortcutEvent_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QShortcutEvent_key_0<RetType> {
  fn key_0(self , rsthis: & QShortcutEvent) -> RetType;
}
impl<'a> /*trait*/ QShortcutEvent_key_0<usize> for () {
  fn key_0(self , rsthis: & QShortcutEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QShortcutEvent3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:771
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int shortcutId() const

/*

*/
impl /*struct*/ QShortcutEvent {
  pub fn shortcutId_0<RetType, T: QShortcutEvent_shortcutId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.shortcutId_0(self);
    // return 1;
  }
}
pub trait QShortcutEvent_shortcutId_0<RetType> {
  fn shortcutId_0(self , rsthis: & QShortcutEvent) -> RetType;
}
impl<'a> /*trait*/ QShortcutEvent_shortcutId_0<i32> for () {
  fn shortcutId_0(self , rsthis: & QShortcutEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QShortcutEvent10shortcutIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:772
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAmbiguous() const

/*

*/
impl /*struct*/ QShortcutEvent {
  pub fn isAmbiguous_0<RetType, T: QShortcutEvent_isAmbiguous_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAmbiguous_0(self);
    // return 1;
  }
}
pub trait QShortcutEvent_isAmbiguous_0<RetType> {
  fn isAmbiguous_0(self , rsthis: & QShortcutEvent) -> RetType;
}
impl<'a> /*trait*/ QShortcutEvent_isAmbiguous_0<bool> for () {
  fn isAmbiguous_0(self , rsthis: & QShortcutEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QShortcutEvent11isAmbiguousEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
