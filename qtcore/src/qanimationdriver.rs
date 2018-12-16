

// mod ::core::QAnimationDriver
// package qtcore
// /usr/include/qt/QtCore/qabstractanimation.h
// #include <qabstractanimation.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 28
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

// void advanceAnimation(qint64)
// func (this *QAnimationDriver) InheritAdvanceAnimation(f func(timeStep int64) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "advanceAnimation", f)
// }

// void start()
// func (this *QAnimationDriver) InheritStart(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "start", f)
// }

// void stop()
// func (this *QAnimationDriver) InheritStop(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "stop", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QAnimationDriver)=16
pub struct QAnimationDriver {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAnimationDriver_ITF interface {
//    QObject_ITF
//    QAnimationDriver_PTR() *QAnimationDriver
//}
//func (ptr *QAnimationDriver) QAnimationDriver_PTR() *QAnimationDriver { return ptr }

impl /*struct*/ QAnimationDriver {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAnimationDriver {
    return QAnimationDriver{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAnimationDriver {
//  type Target = QAnimationDriverBASE;
//
//  fn deref(&self) -> &QAnimationDriverBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAnimationDriverBASE> for QAnimationDriver {
//  fn as_ref(& self) -> & QAnimationDriverBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstractanimation.h:135
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn metaObject_0<RetType, T: QAnimationDriver_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAnimationDriver) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAnimationDriver10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAnimationDriver(QObject *)

/*

*/
// QAnimationDriver(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAnimationDriver {
  pub fn QAnimationDriver_0<T: QAnimationDriver_QAnimationDriver_0>(value: T) -> QAnimationDriver {
    let rsthis = value.QAnimationDriver_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAnimationDriver_QAnimationDriver_0 {
  fn QAnimationDriver_0(self) -> QAnimationDriver;
}
// QAnimationDriver(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAnimationDriver_QAnimationDriver_0 for (usize) {
  fn QAnimationDriver_0(self) -> QAnimationDriver {
    // unsafe{_ZN16QAnimationDriverC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QAnimationDriverC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAnimationDriver{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:140
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAnimationDriver()

/*

*/
pub fn DeleteQAnimationDriver(this :*mut QAnimationDriver) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QAnimationDriverD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstractanimation.h:142
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void advance()

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn advance_0<RetType, T: QAnimationDriver_advance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.advance_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_advance_0<RetType> {
  fn advance_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_advance_0<(/*void*/)> for () {
  fn advance_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver7advanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void install()

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn install_0<RetType, T: QAnimationDriver_install_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.install_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_install_0<RetType> {
  fn install_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_install_0<(/*void*/)> for () {
  fn install_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver7installEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:145
// index:0
// Public Visibility=Default Availability=Available
// [-2] void uninstall()

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn uninstall_0<RetType, T: QAnimationDriver_uninstall_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.uninstall_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_uninstall_0<RetType> {
  fn uninstall_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_uninstall_0<(/*void*/)> for () {
  fn uninstall_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver9uninstallEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:147
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRunning() const

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn isRunning_0<RetType, T: QAnimationDriver_isRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRunning_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_isRunning_0<RetType> {
  fn isRunning_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_isRunning_0<bool> for () {
  fn isRunning_0(self , rsthis: & QAnimationDriver) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAnimationDriver9isRunningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:149
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 elapsed() const

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn elapsed_0<RetType, T: QAnimationDriver_elapsed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elapsed_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_elapsed_0<RetType> {
  fn elapsed_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_elapsed_0<i64> for () {
  fn elapsed_0(self , rsthis: & QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAnimationDriver7elapsedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStartTime(qint64)

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn setStartTime_0<RetType, T: QAnimationDriver_setStartTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStartTime_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_setStartTime_0<RetType> {
  fn setStartTime_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_setStartTime_0<(/*void*/)> for (i64) {
  fn setStartTime_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver12setStartTimeEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:153
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 startTime() const

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn startTime_0<RetType, T: QAnimationDriver_startTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startTime_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_startTime_0<RetType> {
  fn startTime_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_startTime_0<i64> for () {
  fn startTime_0(self , rsthis: & QAnimationDriver) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QAnimationDriver9startTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:156
// index:0
// Public Visibility=Default Availability=Available
// [-2] void started()

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn started_0<RetType, T: QAnimationDriver_started_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.started_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_started_0<RetType> {
  fn started_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_started_0<(/*void*/)> for () {
  fn started_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver7startedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stopped()

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn stopped_0<RetType, T: QAnimationDriver_stopped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stopped_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_stopped_0<RetType> {
  fn stopped_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_stopped_0<(/*void*/)> for () {
  fn stopped_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver7stoppedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:161
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void advanceAnimation(qint64)

/*

*/
impl /*struct*/ QAnimationDriver {
  pub fn advanceAnimation_0<RetType, T: QAnimationDriver_advanceAnimation_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.advanceAnimation_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_advanceAnimation_0<RetType> {
  fn advanceAnimation_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_advanceAnimation_0<(/*void*/)> for (i64) {
  fn advanceAnimation_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver16advanceAnimationEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:162
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void start()

/*
Starts the animation. The policy argument says whether or not the animation should be deleted when it's done. When the animation starts, the stateChanged() signal is emitted, and state() returns Running. When control reaches the event loop, the animation will run by itself, periodically calling updateCurrentTime() as the animation progresses.

If the animation is currently stopped or has already reached the end, calling start() will rewind the animation and start again from the beginning. When the animation reaches the end, the animation will either stop, or if the loop level is more than 1, it will rewind and continue from the beginning.

If the animation is already running, this function does nothing.

See also stop() and state().
*/
impl /*struct*/ QAnimationDriver {
  pub fn start_0<RetType, T: QAnimationDriver_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_start_0<RetType> {
  fn start_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_start_0<(/*void*/)> for () {
  fn start_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstractanimation.h:163
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the animation. When the animation is stopped, it emits the stateChanged() signal, and state() returns Stopped. The current time is not changed.

If the animation stops by itself after reaching the end (i.e., currentLoopTime() == duration() and currentLoop() > loopCount() - 1), the finished() signal is emitted.

See also start() and state().
*/
impl /*struct*/ QAnimationDriver {
  pub fn stop_0<RetType, T: QAnimationDriver_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QAnimationDriver_stop_0<RetType> {
  fn stop_0(self , rsthis: & QAnimationDriver) -> RetType;
}
impl<'a> /*trait*/ QAnimationDriver_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QAnimationDriver) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN16QAnimationDriver4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
