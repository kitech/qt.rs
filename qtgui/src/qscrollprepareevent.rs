

// mod ::gui::QScrollPrepareEvent
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
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QScrollPrepareEvent)=112
pub struct QScrollPrepareEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QScrollPrepareEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QScrollPrepareEvent_PTR() *QScrollPrepareEvent
//}
//func (ptr *QScrollPrepareEvent) QScrollPrepareEvent_PTR() *QScrollPrepareEvent { return ptr }

impl /*struct*/ QScrollPrepareEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QScrollPrepareEvent {
    return QScrollPrepareEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QScrollPrepareEvent {
//  type Target = QScrollPrepareEventBASE;
//
//  fn deref(&self) -> &QScrollPrepareEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QScrollPrepareEventBASE> for QScrollPrepareEvent {
//  fn as_ref(& self) -> & QScrollPrepareEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:990
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QScrollPrepareEvent(const QPointF &)

/*

*/
// QScrollPrepareEvent(const QPointF &) ctx.fn_proto_cpp
impl /*struct*/ QScrollPrepareEvent {
  pub fn QScrollPrepareEvent_0<T: QScrollPrepareEvent_QScrollPrepareEvent_0>(value: T) -> QScrollPrepareEvent {
    let rsthis = value.QScrollPrepareEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollPrepareEvent_QScrollPrepareEvent_0 {
  fn QScrollPrepareEvent_0(self) -> QScrollPrepareEvent;
}
// QScrollPrepareEvent(const QPointF &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QScrollPrepareEvent_QScrollPrepareEvent_0 for (usize) {
  fn QScrollPrepareEvent_0(self) -> QScrollPrepareEvent {
    // unsafe{_ZN19QScrollPrepareEventC2ERK7QPointF()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN19QScrollPrepareEventC2ERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QScrollPrepareEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:991
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QScrollPrepareEvent()

/*

*/
pub fn DeleteQScrollPrepareEvent(this :*mut QScrollPrepareEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN19QScrollPrepareEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 112)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:993
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF startPos() const

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn startPos_0<RetType, T: QScrollPrepareEvent_startPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startPos_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_startPos_0<RetType> {
  fn startPos_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_startPos_0<usize> for () {
  fn startPos_0(self , rsthis: & QScrollPrepareEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollPrepareEvent8startPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:995
// index:0
// Public Visibility=Default Availability=Available
// [16] QSizeF viewportSize() const

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn viewportSize_0<RetType, T: QScrollPrepareEvent_viewportSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.viewportSize_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_viewportSize_0<RetType> {
  fn viewportSize_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_viewportSize_0<usize> for () {
  fn viewportSize_0(self , rsthis: & QScrollPrepareEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollPrepareEvent12viewportSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:996
// index:0
// Public Visibility=Default Availability=Available
// [32] QRectF contentPosRange() const

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPosRange_0<RetType, T: QScrollPrepareEvent_contentPosRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentPosRange_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_contentPosRange_0<RetType> {
  fn contentPosRange_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_contentPosRange_0<usize> for () {
  fn contentPosRange_0(self , rsthis: & QScrollPrepareEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollPrepareEvent15contentPosRangeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:997
// index:0
// Public Visibility=Default Availability=Available
// [16] QPointF contentPos() const

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn contentPos_0<RetType, T: QScrollPrepareEvent_contentPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contentPos_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_contentPos_0<RetType> {
  fn contentPos_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_contentPos_0<usize> for () {
  fn contentPos_0(self , rsthis: & QScrollPrepareEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK19QScrollPrepareEvent10contentPosEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:999
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setViewportSize(const QSizeF &)

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn setViewportSize_0<RetType, T: QScrollPrepareEvent_setViewportSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setViewportSize_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_setViewportSize_0<RetType> {
  fn setViewportSize_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_setViewportSize_0<(/*void*/)> for (usize) {
  fn setViewportSize_0(self , rsthis: & QScrollPrepareEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QScrollPrepareEvent15setViewportSizeERK6QSizeF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:1000
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContentPosRange(const QRectF &)

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPosRange_0<RetType, T: QScrollPrepareEvent_setContentPosRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentPosRange_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_setContentPosRange_0<RetType> {
  fn setContentPosRange_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPosRange_0<(/*void*/)> for (usize) {
  fn setContentPosRange_0(self , rsthis: & QScrollPrepareEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QScrollPrepareEvent18setContentPosRangeERK6QRectF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:1001
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContentPos(const QPointF &)

/*

*/
impl /*struct*/ QScrollPrepareEvent {
  pub fn setContentPos_0<RetType, T: QScrollPrepareEvent_setContentPos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContentPos_0(self);
    // return 1;
  }
}
pub trait QScrollPrepareEvent_setContentPos_0<RetType> {
  fn setContentPos_0(self , rsthis: & QScrollPrepareEvent) -> RetType;
}
impl<'a> /*trait*/ QScrollPrepareEvent_setContentPos_0<(/*void*/)> for (usize) {
  fn setContentPos_0(self , rsthis: & QScrollPrepareEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN19QScrollPrepareEvent13setContentPosERK7QPointF", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
