

// mod ::gui::QPaintEvent
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
#[derive(Default)] // class sizeof(QPaintEvent)=56
pub struct QPaintEvent {
  qbase: QEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPaintEvent_ITF interface {
//    qtcore.QEvent_ITF
//    QPaintEvent_PTR() *QPaintEvent
//}
//func (ptr *QPaintEvent) QPaintEvent_PTR() *QPaintEvent { return ptr }

impl /*struct*/ QPaintEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPaintEvent {
    return QPaintEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPaintEvent {
//  type Target = QPaintEventBASE;
//
//  fn deref(&self) -> &QPaintEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPaintEventBASE> for QPaintEvent {
//  fn as_ref(& self) -> & QPaintEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:405
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPaintEvent(const QRegion &)

/*

*/
// QPaintEvent(const QRegion &) ctx.fn_proto_cpp
impl /*struct*/ QPaintEvent {
  pub fn QPaintEvent_0<T: QPaintEvent_QPaintEvent_0>(value: T) -> QPaintEvent {
    let rsthis = value.QPaintEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEvent_QPaintEvent_0 {
  fn QPaintEvent_0(self) -> QPaintEvent;
}
// QPaintEvent(const QRegion &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPaintEvent_QPaintEvent_0 for (usize) {
  fn QPaintEvent_0(self) -> QPaintEvent {
    // unsafe{_ZN11QPaintEventC2ERK7QRegion()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPaintEventC2ERK7QRegion", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPaintEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:406
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPaintEvent(const QRect &)

/*

*/
// QPaintEvent(const QRect &) ctx.fn_proto_cpp
impl /*struct*/ QPaintEvent {
  pub fn QPaintEvent_1<T: QPaintEvent_QPaintEvent_1>(value: T) -> QPaintEvent {
    let rsthis = value.QPaintEvent_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEvent_QPaintEvent_1 {
  fn QPaintEvent_1(self) -> QPaintEvent;
}
// QPaintEvent(const QRect &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPaintEvent_QPaintEvent_1 for (usize) {
  fn QPaintEvent_1(self) -> QPaintEvent {
    // unsafe{_ZN11QPaintEventC2ERK5QRect()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QPaintEventC2ERK5QRect", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPaintEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:407
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPaintEvent()

/*

*/
pub fn DeleteQPaintEvent(this :*mut QPaintEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QPaintEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 56)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:409
// index:0
// Public inline Visibility=Default Availability=Available
// [16] const QRect & rect() const

/*

*/
impl /*struct*/ QPaintEvent {
  pub fn rect_0<RetType, T: QPaintEvent_rect_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rect_0(self);
    // return 1;
  }
}
pub trait QPaintEvent_rect_0<RetType> {
  fn rect_0(self , rsthis: & QPaintEvent) -> RetType;
}
impl<'a> /*trait*/ QPaintEvent_rect_0<usize> for () {
  fn rect_0(self , rsthis: & QPaintEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPaintEvent4rectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:410
// index:0
// Public inline Visibility=Default Availability=Available
// [8] const QRegion & region() const

/*

*/
impl /*struct*/ QPaintEvent {
  pub fn region_0<RetType, T: QPaintEvent_region_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.region_0(self);
    // return 1;
  }
}
pub trait QPaintEvent_region_0<RetType> {
  fn region_0(self , rsthis: & QPaintEvent) -> RetType;
}
impl<'a> /*trait*/ QPaintEvent_region_0<usize> for () {
  fn region_0(self , rsthis: & QPaintEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QPaintEvent6regionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
