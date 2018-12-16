

// mod ::core::QBasicTimer
// package qtcore
// /usr/include/qt/QtCore/qbasictimer.h
// #include <qbasictimer.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
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



/*

*/
#[derive(Default)] // class sizeof(QBasicTimer)=4
pub struct QBasicTimer {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBasicTimer_ITF interface {
//    QBasicTimer_PTR() *QBasicTimer
//}
//func (ptr *QBasicTimer) QBasicTimer_PTR() *QBasicTimer { return ptr }

impl /*struct*/ QBasicTimer {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBasicTimer {
    return QBasicTimer{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBasicTimer {
//  type Target = QBasicTimerBASE;
//
//  fn deref(&self) -> &QBasicTimerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBasicTimerBASE> for QBasicTimer {
//  fn as_ref(& self) -> & QBasicTimerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qbasictimer.h:55
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QBasicTimer()

/*
Contructs a basic timer.

See also start().
*/
// QBasicTimer() ctx.fn_proto_cpp
impl /*struct*/ QBasicTimer {
  pub fn QBasicTimer_0<T: QBasicTimer_QBasicTimer_0>(value: T) -> QBasicTimer {
    let rsthis = value.QBasicTimer_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBasicTimer_QBasicTimer_0 {
  fn QBasicTimer_0(self) -> QBasicTimer;
}
// QBasicTimer() ctx.fn_proto_cpp
impl<'a> /*trait*/ QBasicTimer_QBasicTimer_0 for () {
  fn QBasicTimer_0(self) -> QBasicTimer {
    // unsafe{_ZN11QBasicTimerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QBasicTimerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBasicTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbasictimer.h:56
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QBasicTimer()

/*

*/
pub fn DeleteQBasicTimer(this :*mut QBasicTimer) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QBasicTimerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 4)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qbasictimer.h:58
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if the timer is running and has not been stopped; otherwise returns false.

See also start() and stop().
*/
impl /*struct*/ QBasicTimer {
  pub fn isActive_0<RetType, T: QBasicTimer_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QBasicTimer_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QBasicTimer) -> RetType;
}
impl<'a> /*trait*/ QBasicTimer_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QBasicTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QBasicTimer8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbasictimer.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int timerId() const

/*
Returns the timer's ID.

See also QTimerEvent::timerId().
*/
impl /*struct*/ QBasicTimer {
  pub fn timerId_0<RetType, T: QBasicTimer_timerId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerId_0(self);
    // return 1;
  }
}
pub trait QBasicTimer_timerId_0<RetType> {
  fn timerId_0(self , rsthis: & QBasicTimer) -> RetType;
}
impl<'a> /*trait*/ QBasicTimer_timerId_0<i32> for () {
  fn timerId_0(self , rsthis: & QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QBasicTimer7timerIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qbasictimer.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start(int, QObject *)

/*
Starts (or restarts) the timer with a msec milliseconds timeout. The timer will be a Qt::CoarseTimer. See Qt::TimerType for information on the different timer types.

The given object will receive timer events.

See also stop(), isActive(), QObject::timerEvent(), and Qt::CoarseTimer.
*/
impl /*struct*/ QBasicTimer {
  pub fn start_0<RetType, T: QBasicTimer_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QBasicTimer_start_0<RetType> {
  fn start_0(self , rsthis: & QBasicTimer) -> RetType;
}
impl<'a> /*trait*/ QBasicTimer_start_0<(/*void*/)> for (i32,usize) {
  fn start_0(self , rsthis: & QBasicTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QBasicTimer5startEiP7QObject", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbasictimer.h:62
// index:1
// Public Visibility=Default Availability=Available
// [-2] void start(int, Qt::TimerType, QObject *)

/*
Starts (or restarts) the timer with a msec milliseconds timeout. The timer will be a Qt::CoarseTimer. See Qt::TimerType for information on the different timer types.

The given object will receive timer events.

See also stop(), isActive(), QObject::timerEvent(), and Qt::CoarseTimer.
*/
impl /*struct*/ QBasicTimer {
  pub fn start_1<RetType, T: QBasicTimer_start_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_1(self);
    // return 1;
  }
}
pub trait QBasicTimer_start_1<RetType> {
  fn start_1(self , rsthis: & QBasicTimer) -> RetType;
}
impl<'a> /*trait*/ QBasicTimer_start_1<(/*void*/)> for (i32,i32,usize) {
  fn start_1(self , rsthis: & QBasicTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QBasicTimer5startEiN2Qt9TimerTypeEP7QObject", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qbasictimer.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the timer.

See also start() and isActive().
*/
impl /*struct*/ QBasicTimer {
  pub fn stop_0<RetType, T: QBasicTimer_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QBasicTimer_stop_0<RetType> {
  fn stop_0(self , rsthis: & QBasicTimer) -> RetType;
}
impl<'a> /*trait*/ QBasicTimer_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QBasicTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QBasicTimer4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
