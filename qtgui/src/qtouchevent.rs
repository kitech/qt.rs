

// mod ::gui::QTouchEvent
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
#[derive(Default)] // class sizeof(QTouchEvent)=72
pub struct QTouchEvent {
  qbase: QInputEvent,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTouchEvent_ITF interface {
//    QInputEvent_ITF
//    QTouchEvent_PTR() *QTouchEvent
//}
//func (ptr *QTouchEvent) QTouchEvent_PTR() *QTouchEvent { return ptr }

impl /*struct*/ QTouchEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTouchEvent {
    return QTouchEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTouchEvent {
//  type Target = QTouchEventBASE;
//
//  fn deref(&self) -> &QTouchEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTouchEventBASE> for QTouchEvent {
//  fn as_ref(& self) -> & QTouchEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtGui/qevent.h:947
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTouchEvent()

/*

*/
pub fn DeleteQTouchEvent(this :*mut QTouchEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QTouchEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 72)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtGui/qevent.h:949
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QWindow * window() const

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn window_0<RetType, T: QTouchEvent_window_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.window_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_window_0<RetType> {
  fn window_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_window_0<usize> for () {
  fn window_0(self , rsthis: & QTouchEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTouchEvent6windowEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:950
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QObject * target() const

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn target_0<RetType, T: QTouchEvent_target_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.target_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_target_0<RetType> {
  fn target_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_target_0<usize> for () {
  fn target_0(self , rsthis: & QTouchEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTouchEvent6targetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:954
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::TouchPointStates touchPointStates() const

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn touchPointStates_0<RetType, T: QTouchEvent_touchPointStates_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.touchPointStates_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_touchPointStates_0<RetType> {
  fn touchPointStates_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_touchPointStates_0<i32> for () {
  fn touchPointStates_0(self , rsthis: & QTouchEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTouchEvent16touchPointStatesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:956
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QTouchDevice * device() const

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn device_0<RetType, T: QTouchEvent_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_device_0<RetType> {
  fn device_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_device_0<usize> for () {
  fn device_0(self , rsthis: & QTouchEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QTouchEvent6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtGui/qevent.h:959
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setWindow(QWindow *)

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn setWindow_0<RetType, T: QTouchEvent_setWindow_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setWindow_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_setWindow_0<RetType> {
  fn setWindow_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_setWindow_0<(/*void*/)> for (usize) {
  fn setWindow_0(self , rsthis: & QTouchEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTouchEvent9setWindowEP7QWindow", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:960
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTarget(QObject *)

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn setTarget_0<RetType, T: QTouchEvent_setTarget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTarget_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_setTarget_0<RetType> {
  fn setTarget_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_setTarget_0<(/*void*/)> for (usize) {
  fn setTarget_0(self , rsthis: & QTouchEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTouchEvent9setTargetEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:961
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTouchPointStates(Qt::TouchPointStates)

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn setTouchPointStates_0<RetType, T: QTouchEvent_setTouchPointStates_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTouchPointStates_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_setTouchPointStates_0<RetType> {
  fn setTouchPointStates_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_setTouchPointStates_0<(/*void*/)> for (i32) {
  fn setTouchPointStates_0(self , rsthis: & QTouchEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QTouchEvent19setTouchPointStatesE6QFlagsIN2Qt15TouchPointStateEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtGui/qevent.h:963
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setDevice(QTouchDevice *)

/*

*/
impl /*struct*/ QTouchEvent {
  pub fn setDevice_0<RetType, T: QTouchEvent_setDevice_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDevice_0(self);
    // return 1;
  }
}
pub trait QTouchEvent_setDevice_0<RetType> {
  fn setDevice_0(self , rsthis: & QTouchEvent) -> RetType;
}
impl<'a> /*trait*/ QTouchEvent_setDevice_0<(/*void*/)> for (usize) {
  fn setDevice_0(self , rsthis: & QTouchEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QTouchEvent9setDeviceEP12QTouchDevice", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
