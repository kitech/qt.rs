

// mod ::core::QChildEvent
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QChildEvent)=32
pub struct QChildEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QChildEvent_ITF interface {
//    QEvent_ITF
//    QChildEvent_PTR() *QChildEvent
//}
//func (ptr *QChildEvent) QChildEvent_PTR() *QChildEvent { return ptr }

impl /*struct*/ QChildEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QChildEvent {
    return QChildEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QChildEvent {
//  type Target = QChildEventBASE;
//
//  fn deref(&self) -> &QChildEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QChildEventBASE> for QChildEvent {
//  fn as_ref(& self) -> & QChildEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcoreevent.h:352
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QChildEvent(QEvent::Type, QObject *)

/*

*/
// QChildEvent(QEvent::Type, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QChildEvent {
  pub fn QChildEvent_0<T: QChildEvent_QChildEvent_0>(value: T) -> QChildEvent {
    let rsthis = value.QChildEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QChildEvent_QChildEvent_0 {
  fn QChildEvent_0(self) -> QChildEvent;
}
// QChildEvent(QEvent::Type, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QChildEvent_QChildEvent_0 for (i32,usize) {
  fn QChildEvent_0(self) -> QChildEvent {
    // unsafe{_ZN11QChildEventC2EN6QEvent4TypeEP7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QChildEventC2EN6QEvent4TypeEP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QChildEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:353
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QChildEvent()

/*

*/
pub fn DeleteQChildEvent(this :*mut QChildEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QChildEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcoreevent.h:354
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QObject * child() const

/*

*/
impl /*struct*/ QChildEvent {
  pub fn child_0<RetType, T: QChildEvent_child_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.child_0(self);
    // return 1;
  }
}
pub trait QChildEvent_child_0<RetType> {
  fn child_0(self , rsthis: & QChildEvent) -> RetType;
}
impl<'a> /*trait*/ QChildEvent_child_0<usize> for () {
  fn child_0(self , rsthis: & QChildEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QChildEvent5childEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:355
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool added() const

/*

*/
impl /*struct*/ QChildEvent {
  pub fn added_0<RetType, T: QChildEvent_added_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.added_0(self);
    // return 1;
  }
}
pub trait QChildEvent_added_0<RetType> {
  fn added_0(self , rsthis: & QChildEvent) -> RetType;
}
impl<'a> /*trait*/ QChildEvent_added_0<bool> for () {
  fn added_0(self , rsthis: & QChildEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QChildEvent5addedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:356
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool polished() const

/*

*/
impl /*struct*/ QChildEvent {
  pub fn polished_0<RetType, T: QChildEvent_polished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.polished_0(self);
    // return 1;
  }
}
pub trait QChildEvent_polished_0<RetType> {
  fn polished_0(self , rsthis: & QChildEvent) -> RetType;
}
impl<'a> /*trait*/ QChildEvent_polished_0<bool> for () {
  fn polished_0(self , rsthis: & QChildEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QChildEvent8polishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:357
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool removed() const

/*

*/
impl /*struct*/ QChildEvent {
  pub fn removed_0<RetType, T: QChildEvent_removed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removed_0(self);
    // return 1;
  }
}
pub trait QChildEvent_removed_0<RetType> {
  fn removed_0(self , rsthis: & QChildEvent) -> RetType;
}
impl<'a> /*trait*/ QChildEvent_removed_0<bool> for () {
  fn removed_0(self , rsthis: & QChildEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QChildEvent7removedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
