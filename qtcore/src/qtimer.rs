

// mod ::core::QTimer
// package qtcore
// /usr/include/qt/QtCore/qtimer.h
// #include <qtimer.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 33
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

// void timerEvent(QTimerEvent *)
// func (this *QTimer) InheritTimerEvent(f func(arg0 *QTimerEvent/*777 QTimerEvent **/) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "timerEvent", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QTimer)=32
pub struct QTimer {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTimer_ITF interface {
//    QObject_ITF
//    QTimer_PTR() *QTimer
//}
//func (ptr *QTimer) QTimer_PTR() *QTimer { return ptr }

impl /*struct*/ QTimer {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTimer {
    return QTimer{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTimer {
//  type Target = QTimerBASE;
//
//  fn deref(&self) -> &QTimerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTimerBASE> for QTimer {
//  fn as_ref(& self) -> & QTimerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtimer.h:59
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QTimer {
  pub fn metaObject_0<RetType, T: QTimer_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QTimer_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QTimer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTimer(QObject *)

/*
Constructs a timer with the given parent.
*/
// QTimer(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QTimer {
  pub fn QTimer_0<T: QTimer_QTimer_0>(value: T) -> QTimer {
    let rsthis = value.QTimer_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTimer_QTimer_0 {
  fn QTimer_0(self) -> QTimer;
}
// QTimer(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTimer_QTimer_0 for (usize) {
  fn QTimer_0(self) -> QTimer {
    // unsafe{_ZN6QTimerC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QTimerC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:67
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QTimer()

/*

*/
pub fn DeleteQTimer(this :*mut QTimer) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QTimerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtimer.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isActive() const

/*
Returns true if the timer is running (pending); otherwise returns false.

Note: Getter function for property active.
*/
impl /*struct*/ QTimer {
  pub fn isActive_0<RetType, T: QTimer_isActive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isActive_0(self);
    // return 1;
  }
}
pub trait QTimer_isActive_0<RetType> {
  fn isActive_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_isActive_0<bool> for () {
  fn isActive_0(self , rsthis: & QTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer8isActiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:70
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int timerId() const

/*
Returns the ID of the timer if the timer is running; otherwise returns -1.
*/
impl /*struct*/ QTimer {
  pub fn timerId_0<RetType, T: QTimer_timerId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerId_0(self);
    // return 1;
  }
}
pub trait QTimer_timerId_0<RetType> {
  fn timerId_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_timerId_0<i32> for () {
  fn timerId_0(self , rsthis: & QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer7timerIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setInterval(int)

/*

*/
impl /*struct*/ QTimer {
  pub fn setInterval_0<RetType, T: QTimer_setInterval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setInterval_0(self);
    // return 1;
  }
}
pub trait QTimer_setInterval_0<RetType> {
  fn setInterval_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_setInterval_0<(/*void*/)> for (i32) {
  fn setInterval_0(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer11setIntervalEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:73
// index:0
// Public inline Visibility=Default Availability=Available
// [4] int interval() const

/*

*/
impl /*struct*/ QTimer {
  pub fn interval_0<RetType, T: QTimer_interval_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interval_0(self);
    // return 1;
  }
}
pub trait QTimer_interval_0<RetType> {
  fn interval_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_interval_0<i32> for () {
  fn interval_0(self , rsthis: & QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer8intervalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:75
// index:0
// Public Visibility=Default Availability=Available
// [4] int remainingTime() const

/*

*/
impl /*struct*/ QTimer {
  pub fn remainingTime_0<RetType, T: QTimer_remainingTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remainingTime_0(self);
    // return 1;
  }
}
pub trait QTimer_remainingTime_0<RetType> {
  fn remainingTime_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_remainingTime_0<i32> for () {
  fn remainingTime_0(self , rsthis: & QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer13remainingTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setTimerType(Qt::TimerType)

/*

*/
impl /*struct*/ QTimer {
  pub fn setTimerType_0<RetType, T: QTimer_setTimerType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimerType_0(self);
    // return 1;
  }
}
pub trait QTimer_setTimerType_0<RetType> {
  fn setTimerType_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_setTimerType_0<(/*void*/)> for (i32) {
  fn setTimerType_0(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer12setTimerTypeEN2Qt9TimerTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::TimerType timerType() const

/*

*/
impl /*struct*/ QTimer {
  pub fn timerType_0<RetType, T: QTimer_timerType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerType_0(self);
    // return 1;
  }
}
pub trait QTimer_timerType_0<RetType> {
  fn timerType_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_timerType_0<i32> for () {
  fn timerType_0(self , rsthis: & QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer9timerTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:80
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setSingleShot(bool)

/*

*/
impl /*struct*/ QTimer {
  pub fn setSingleShot_0<RetType, T: QTimer_setSingleShot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSingleShot_0(self);
    // return 1;
  }
}
pub trait QTimer_setSingleShot_0<RetType> {
  fn setSingleShot_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_setSingleShot_0<(/*void*/)> for (bool) {
  fn setSingleShot_0(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer13setSingleShotEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isSingleShot() const

/*

*/
impl /*struct*/ QTimer {
  pub fn isSingleShot_0<RetType, T: QTimer_isSingleShot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSingleShot_0(self);
    // return 1;
  }
}
pub trait QTimer_isSingleShot_0<RetType> {
  fn isSingleShot_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_isSingleShot_0<bool> for () {
  fn isSingleShot_0(self , rsthis: & QTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer12isSingleShotEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:83
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void singleShot(int, const QObject *, const char *)

/*
This static function calls a slot after a given time interval.

It is very convenient to use this function because you do not need to bother with a timerEvent or create a local QTimer object.

Example:


  #include <QApplication>
  #include <QTimer>

  int main(int argc, char *argv[])
  {
      QApplication app(argc, argv);
      QTimer::singleShot(600000, &app, SLOT(quit()));
      ...
      return app.exec();
  }



This sample program automatically terminates after 10 minutes (600,000 milliseconds).

The receiver is the receiving object and the member is the slot. The time interval is msec milliseconds.

Note: This function is reentrant.

See also setSingleShot() and start().
*/
impl /*struct*/ QTimer {
  pub fn singleShot_0<RetType, T: QTimer_singleShot_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.singleShot_0();
    // return 1;
  }
}
pub trait QTimer_singleShot_0<RetType> {
  fn singleShot_0(self ) -> RetType;
}
impl<'a> /*trait*/ QTimer_singleShot_0<(/*void*/)> for (i32,usize,usize) {
  fn singleShot_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer10singleShotEiPK7QObjectPKc", 3,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:84
// index:1
// Public static Visibility=Default Availability=Available
// [-2] void singleShot(int, Qt::TimerType, const QObject *, const char *)

/*
This static function calls a slot after a given time interval.

It is very convenient to use this function because you do not need to bother with a timerEvent or create a local QTimer object.

Example:


  #include <QApplication>
  #include <QTimer>

  int main(int argc, char *argv[])
  {
      QApplication app(argc, argv);
      QTimer::singleShot(600000, &app, SLOT(quit()));
      ...
      return app.exec();
  }



This sample program automatically terminates after 10 minutes (600,000 milliseconds).

The receiver is the receiving object and the member is the slot. The time interval is msec milliseconds.

Note: This function is reentrant.

See also setSingleShot() and start().
*/
impl /*struct*/ QTimer {
  pub fn singleShot_1<RetType, T: QTimer_singleShot_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.singleShot_1();
    // return 1;
  }
}
pub trait QTimer_singleShot_1<RetType> {
  fn singleShot_1(self ) -> RetType;
}
impl<'a> /*trait*/ QTimer_singleShot_1<(/*void*/)> for (i32,i32,usize,usize) {
  fn singleShot_1(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (self.3) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer10singleShotEiN2Qt9TimerTypeEPK7QObjectPKc", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start(int)

/*
Starts or restarts the timer with a timeout interval of msec milliseconds.

If the timer is already running, it will be stopped and restarted.

If singleShot is true, the timer will be activated only once.
*/
impl /*struct*/ QTimer {
  pub fn start_0<RetType, T: QTimer_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QTimer_start_0<RetType> {
  fn start_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_start_0<(/*void*/)> for (i32) {
  fn start_0(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer5startEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:160
// index:1
// Public Visibility=Default Availability=Available
// [-2] void start()

/*
Starts or restarts the timer with a timeout interval of msec milliseconds.

If the timer is already running, it will be stopped and restarted.

If singleShot is true, the timer will be activated only once.
*/
impl /*struct*/ QTimer {
  pub fn start_1<RetType, T: QTimer_start_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_1(self);
    // return 1;
  }
}
pub trait QTimer_start_1<RetType> {
  fn start_1(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_start_1<(/*void*/)> for () {
  fn start_1(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QTimer5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:161
// index:0
// Public Visibility=Default Availability=Available
// [-2] void stop()

/*
Stops the timer.

See also start().
*/
impl /*struct*/ QTimer {
  pub fn stop_0<RetType, T: QTimer_stop_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stop_0(self);
    // return 1;
  }
}
pub trait QTimer_stop_0<RetType> {
  fn stop_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_stop_0<(/*void*/)> for () {
  fn stop_0(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QTimer4stopEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtimer.h:173
// index:0
// Public inline Visibility=Default Availability=Available
// [8] std::chrono::milliseconds intervalAsDuration() const

/*
Returns the interval of this timer as a std::chrono::milliseconds object.

This function was introduced in  Qt 5.8.

See also interval.
*/
impl /*struct*/ QTimer {
  pub fn intervalAsDuration_0<RetType, T: QTimer_intervalAsDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.intervalAsDuration_0(self);
    // return 1;
  }
}
pub trait QTimer_intervalAsDuration_0<RetType> {
  fn intervalAsDuration_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_intervalAsDuration_0<i32> for () {
  fn intervalAsDuration_0(self , rsthis: & QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer18intervalAsDurationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:178
// index:0
// Public inline Visibility=Default Availability=Available
// [8] std::chrono::milliseconds remainingTimeAsDuration() const

/*
Returns the time remaining in this timer object as a std::chrono::milliseconds object. If this timer is due or overdue, the returned value is std::chrono::milliseconds::zero(). If the remaining time could not be found or the timer is not active, this function returns a negative duration.

This function was introduced in  Qt 5.8.

See also remainingTime().
*/
impl /*struct*/ QTimer {
  pub fn remainingTimeAsDuration_0<RetType, T: QTimer_remainingTimeAsDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remainingTimeAsDuration_0(self);
    // return 1;
  }
}
pub trait QTimer_remainingTimeAsDuration_0<RetType> {
  fn remainingTimeAsDuration_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_remainingTimeAsDuration_0<i32> for () {
  fn remainingTimeAsDuration_0(self , rsthis: & QTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QTimer23remainingTimeAsDurationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtimer.h:200
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void timerEvent(QTimerEvent *)

/*
Reimplemented from QObject::timerEvent().
*/
impl /*struct*/ QTimer {
  pub fn timerEvent_0<RetType, T: QTimer_timerEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerEvent_0(self);
    // return 1;
  }
}
pub trait QTimer_timerEvent_0<RetType> {
  fn timerEvent_0(self , rsthis: & QTimer) -> RetType;
}
impl<'a> /*trait*/ QTimer_timerEvent_0<(/*void*/)> for (usize) {
  fn timerEvent_0(self , rsthis: & QTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN6QTimer10timerEventEP11QTimerEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
