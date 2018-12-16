

// mod ::core::QPauseAnimation
// package qtcore
// /usr/include/qt/QtCore/qpauseanimation.h
// #include <qpauseanimation.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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

// bool event(QEvent *)
// func (this *QPauseAnimation) InheritEvent(f func(e *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }

// void updateCurrentTime(int)
// func (this *QPauseAnimation) InheritUpdateCurrentTime(f func(arg0 int) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "updateCurrentTime", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QPauseAnimation)=16
pub struct QPauseAnimation {
  qbase: QAbstractAnimation,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPauseAnimation_ITF interface {
//    QAbstractAnimation_ITF
//    QPauseAnimation_PTR() *QPauseAnimation
//}
//func (ptr *QPauseAnimation) QPauseAnimation_PTR() *QPauseAnimation { return ptr }

impl /*struct*/ QPauseAnimation {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPauseAnimation {
    return QPauseAnimation{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPauseAnimation {
//  type Target = QPauseAnimationBASE;
//
//  fn deref(&self) -> &QPauseAnimationBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPauseAnimationBASE> for QPauseAnimation {
//  fn as_ref(& self) -> & QPauseAnimationBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qpauseanimation.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPauseAnimation {
  pub fn metaObject_0<RetType, T: QPauseAnimation_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPauseAnimation_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPauseAnimation) -> RetType;
}
impl<'a> /*trait*/ QPauseAnimation_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPauseAnimation) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QPauseAnimation10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpauseanimation.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPauseAnimation(QObject *)

/*
Constructs a QPauseAnimation. parent is passed to QObject's constructor. The default duration is 0.
*/
// QPauseAnimation(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPauseAnimation {
  pub fn QPauseAnimation_0<T: QPauseAnimation_QPauseAnimation_0>(value: T) -> QPauseAnimation {
    let rsthis = value.QPauseAnimation_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPauseAnimation_QPauseAnimation_0 {
  fn QPauseAnimation_0(self) -> QPauseAnimation;
}
// QPauseAnimation(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPauseAnimation_QPauseAnimation_0 for (usize) {
  fn QPauseAnimation_0(self) -> QPauseAnimation {
    // unsafe{_ZN15QPauseAnimationC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QPauseAnimationC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPauseAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpauseanimation.h:58
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPauseAnimation(int, QObject *)

/*
Constructs a QPauseAnimation. parent is passed to QObject's constructor. The default duration is 0.
*/
// QPauseAnimation(int, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPauseAnimation {
  pub fn QPauseAnimation_1<T: QPauseAnimation_QPauseAnimation_1>(value: T) -> QPauseAnimation {
    let rsthis = value.QPauseAnimation_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPauseAnimation_QPauseAnimation_1 {
  fn QPauseAnimation_1(self) -> QPauseAnimation;
}
// QPauseAnimation(int, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPauseAnimation_QPauseAnimation_1 for (i32,usize) {
  fn QPauseAnimation_1(self) -> QPauseAnimation {
    // unsafe{_ZN15QPauseAnimationC2EiP7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QPauseAnimationC2EiP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPauseAnimation{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpauseanimation.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPauseAnimation()

/*

*/
pub fn DeleteQPauseAnimation(this :*mut QPauseAnimation) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QPauseAnimationD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qpauseanimation.h:61
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] int duration() const

/*

*/
impl /*struct*/ QPauseAnimation {
  pub fn duration_0<RetType, T: QPauseAnimation_duration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.duration_0(self);
    // return 1;
  }
}
pub trait QPauseAnimation_duration_0<RetType> {
  fn duration_0(self , rsthis: & QPauseAnimation) -> RetType;
}
impl<'a> /*trait*/ QPauseAnimation_duration_0<i32> for () {
  fn duration_0(self , rsthis: & QPauseAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QPauseAnimation8durationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpauseanimation.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDuration(int)

/*

*/
impl /*struct*/ QPauseAnimation {
  pub fn setDuration_0<RetType, T: QPauseAnimation_setDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDuration_0(self);
    // return 1;
  }
}
pub trait QPauseAnimation_setDuration_0<RetType> {
  fn setDuration_0(self , rsthis: & QPauseAnimation) -> RetType;
}
impl<'a> /*trait*/ QPauseAnimation_setDuration_0<(/*void*/)> for (i32) {
  fn setDuration_0(self , rsthis: & QPauseAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QPauseAnimation11setDurationEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpauseanimation.h:65
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QPauseAnimation {
  pub fn event_0<RetType, T: QPauseAnimation_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QPauseAnimation_event_0<RetType> {
  fn event_0(self , rsthis: & QPauseAnimation) -> RetType;
}
impl<'a> /*trait*/ QPauseAnimation_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QPauseAnimation) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QPauseAnimation5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpauseanimation.h:66
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void updateCurrentTime(int)

/*
Reimplemented from QAbstractAnimation::updateCurrentTime().
*/
impl /*struct*/ QPauseAnimation {
  pub fn updateCurrentTime_0<RetType, T: QPauseAnimation_updateCurrentTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.updateCurrentTime_0(self);
    // return 1;
  }
}
pub trait QPauseAnimation_updateCurrentTime_0<RetType> {
  fn updateCurrentTime_0(self , rsthis: & QPauseAnimation) -> RetType;
}
impl<'a> /*trait*/ QPauseAnimation_updateCurrentTime_0<(/*void*/)> for (i32) {
  fn updateCurrentTime_0(self , rsthis: & QPauseAnimation) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QPauseAnimation17updateCurrentTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
