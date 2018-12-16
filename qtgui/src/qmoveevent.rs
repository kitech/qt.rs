

// mod ::gui::QMoveEvent
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
#[derive(Default)] // class sizeof(QMoveEvent)=40
pub struct QMoveEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMoveEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QMoveEvent_PTR() *QMoveEvent
//}
//func (ptr *QMoveEvent) QMoveEvent_PTR() *QMoveEvent { return ptr }

impl /*struct*/ QMoveEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMoveEvent {
    return QMoveEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMoveEvent {
//  type Target = QMoveEventBASE;
//
//  fn deref(&self) -> &QMoveEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMoveEventBASE> for QMoveEvent {
//  fn as_ref(& self) -> & QMoveEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:421
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMoveEvent(const QPoint &, const QPoint &)

/*

*/
// QMoveEvent(const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl /*struct*/ QMoveEvent {
  pub fn QMoveEvent_0<T: QMoveEvent_QMoveEvent_0>(value: T) -> QMoveEvent {
    let rsthis = value.QMoveEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMoveEvent_QMoveEvent_0 {
  fn QMoveEvent_0(self) -> QMoveEvent;
}
// QMoveEvent(const QPoint &, const QPoint &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMoveEvent_QMoveEvent_0 for (usize,usize) {
  fn QMoveEvent_0(self) -> QMoveEvent {
    // unsafe{_ZN10QMoveEventC2ERK6QPointS2_()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QMoveEventC2ERK6QPointS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMoveEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:422
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QMoveEvent()

/*

*/
pub fn DeleteQMoveEvent(this :*mut QMoveEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QMoveEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 40)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:424
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint & pos() const

/*

*/
impl /*struct*/ QMoveEvent {
  pub fn pos_0<RetType, T: QMoveEvent_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QMoveEvent_pos_0<RetType> {
  fn pos_0(self , rsthis: & QMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QMoveEvent_pos_0<usize> for () {
  fn pos_0(self , rsthis: & QMoveEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMoveEvent3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:425
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QPoint & oldPos() const

/*

*/
impl /*struct*/ QMoveEvent {
  pub fn oldPos_0<RetType, T: QMoveEvent_oldPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.oldPos_0(self);
    // return 1;
  }
}
pub trait QMoveEvent_oldPos_0<RetType> {
  fn oldPos_0(self , rsthis: & QMoveEvent) -> RetType;
}
impl<'a> /*trait*/ QMoveEvent_oldPos_0<usize> for () {
  fn oldPos_0(self , rsthis: & QMoveEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QMoveEvent6oldPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
