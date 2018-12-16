

// mod ::core::QDeadlineTimer
// package qtcore
// /usr/include/qt/QtCore/qdeadlinetimer.h
// #include <qdeadlinetimer.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 15
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
#[derive(Default)] // class sizeof(QDeadlineTimer)=16
pub struct QDeadlineTimer {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDeadlineTimer_ITF interface {
//    QDeadlineTimer_PTR() *QDeadlineTimer
//}
//func (ptr *QDeadlineTimer) QDeadlineTimer_PTR() *QDeadlineTimer { return ptr }

impl /*struct*/ QDeadlineTimer {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDeadlineTimer {
    return QDeadlineTimer{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDeadlineTimer {
//  type Target = QDeadlineTimerBASE;
//
//  fn deref(&self) -> &QDeadlineTimerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDeadlineTimerBASE> for QDeadlineTimer {
//  fn as_ref(& self) -> & QDeadlineTimerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdeadlinetimer.h:65
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QDeadlineTimer(Qt::TimerType)

/*
Constructs an expired QDeadlineTimer object. For this object, remainingTime() will return 0.

The timer type timerType may be ignored, since the timer is already expired. Similarly, for optimization purposes, this function will not attempt to obtain the current time and will use a value known to be in the past. Therefore, deadline() may return an unexpected value and this object cannot be used in calculation of how long it is overdue. If that functionality is required, use QDeadlineTimer::current().

See also hasExpired(), remainingTime(), Qt::TimerType, and current().
*/
// QDeadlineTimer(Qt::TimerType) ctx.fn_proto_cpp
impl /*struct*/ QDeadlineTimer {
  pub fn QDeadlineTimer_0<T: QDeadlineTimer_QDeadlineTimer_0>(value: T) -> QDeadlineTimer {
    let rsthis = value.QDeadlineTimer_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDeadlineTimer_QDeadlineTimer_0 {
  fn QDeadlineTimer_0(self) -> QDeadlineTimer;
}
// QDeadlineTimer(Qt::TimerType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDeadlineTimer_QDeadlineTimer_0 for (i32) {
  fn QDeadlineTimer_0(self) -> QDeadlineTimer {
    // unsafe{_ZN14QDeadlineTimerC2EN2Qt9TimerTypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimerC2EN2Qt9TimerTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDeadlineTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:67
// index:1
// Public inline Visibility=Default Availability=Available
// [-2] void QDeadlineTimer(QDeadlineTimer::ForeverConstant, Qt::TimerType)

/*
Constructs an expired QDeadlineTimer object. For this object, remainingTime() will return 0.

The timer type timerType may be ignored, since the timer is already expired. Similarly, for optimization purposes, this function will not attempt to obtain the current time and will use a value known to be in the past. Therefore, deadline() may return an unexpected value and this object cannot be used in calculation of how long it is overdue. If that functionality is required, use QDeadlineTimer::current().

See also hasExpired(), remainingTime(), Qt::TimerType, and current().
*/
// QDeadlineTimer(QDeadlineTimer::ForeverConstant, Qt::TimerType) ctx.fn_proto_cpp
impl /*struct*/ QDeadlineTimer {
  pub fn QDeadlineTimer_1<T: QDeadlineTimer_QDeadlineTimer_1>(value: T) -> QDeadlineTimer {
    let rsthis = value.QDeadlineTimer_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDeadlineTimer_QDeadlineTimer_1 {
  fn QDeadlineTimer_1(self) -> QDeadlineTimer;
}
// QDeadlineTimer(QDeadlineTimer::ForeverConstant, Qt::TimerType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDeadlineTimer_QDeadlineTimer_1 for (i32,i32) {
  fn QDeadlineTimer_1(self) -> QDeadlineTimer {
    // unsafe{_ZN14QDeadlineTimerC2ENS_15ForeverConstantEN2Qt9TimerTypeE()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimerC2ENS_15ForeverConstantEN2Qt9TimerTypeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDeadlineTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:69
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QDeadlineTimer(qint64, Qt::TimerType)

/*
Constructs an expired QDeadlineTimer object. For this object, remainingTime() will return 0.

The timer type timerType may be ignored, since the timer is already expired. Similarly, for optimization purposes, this function will not attempt to obtain the current time and will use a value known to be in the past. Therefore, deadline() may return an unexpected value and this object cannot be used in calculation of how long it is overdue. If that functionality is required, use QDeadlineTimer::current().

See also hasExpired(), remainingTime(), Qt::TimerType, and current().
*/
// QDeadlineTimer(qint64, Qt::TimerType) ctx.fn_proto_cpp
impl /*struct*/ QDeadlineTimer {
  pub fn QDeadlineTimer_2<T: QDeadlineTimer_QDeadlineTimer_2>(value: T) -> QDeadlineTimer {
    let rsthis = value.QDeadlineTimer_2();
    return rsthis;
    // return 1;
  }
}

pub trait QDeadlineTimer_QDeadlineTimer_2 {
  fn QDeadlineTimer_2(self) -> QDeadlineTimer;
}
// QDeadlineTimer(qint64, Qt::TimerType) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDeadlineTimer_QDeadlineTimer_2 for (i64,i32) {
  fn QDeadlineTimer_2(self) -> QDeadlineTimer {
    // unsafe{_ZN14QDeadlineTimerC2ExN2Qt9TimerTypeE()};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimerC2ExN2Qt9TimerTypeE", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDeadlineTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QDeadlineTimer &)

/*
Swaps this deadline timer with the other deadline timer.
*/
impl /*struct*/ QDeadlineTimer {
  pub fn swap_0<RetType, T: QDeadlineTimer_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_swap_0<RetType> {
  fn swap_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QDeadlineTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isForever() const

/*
Returns true if this QDeadlineTimer object never expires, false otherwise. For timers that never expire, remainingTime() always returns -1 and deadline() returns the maximum value.

See also ForeverConstant, hasExpired(), and remainingTime().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn isForever_0<RetType, T: QDeadlineTimer_isForever_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isForever_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_isForever_0<RetType> {
  fn isForever_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_isForever_0<bool> for () {
  fn isForever_0(self , rsthis: & QDeadlineTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer9isForeverEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:76
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasExpired() const

/*
Returns true if this QDeadlineTimer object has expired, false if there remains time left. For objects that have expired, remainingTime() will return zero and deadline() will return a time point in the past.

QDeadlineTimer objects created with the ForeverConstant never expire and this function always returns false for them.

See also isForever() and remainingTime().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn hasExpired_0<RetType, T: QDeadlineTimer_hasExpired_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasExpired_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_hasExpired_0<RetType> {
  fn hasExpired_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_hasExpired_0<bool> for () {
  fn hasExpired_0(self , rsthis: & QDeadlineTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer10hasExpiredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [4] Qt::TimerType timerType() const

/*
Returns the timer type is active for this object.

See also setTimerType().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn timerType_0<RetType, T: QDeadlineTimer_timerType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.timerType_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_timerType_0<RetType> {
  fn timerType_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_timerType_0<i32> for () {
  fn timerType_0(self , rsthis: & QDeadlineTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer9timerTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setTimerType(Qt::TimerType)

/*
Changes the timer type for this object to timerType.

The behavior for each possible value of timerType is operating-system dependent. Qt::PreciseTimer will use the most precise timer that Qt can find, with resolution of 1 millisecond or better, whereas QDeadlineTimer will try to use a more coarse timer for Qt::CoarseTimer and Qt::VeryCoarseTimer.

See also timerType() and Qt::TimerType.
*/
impl /*struct*/ QDeadlineTimer {
  pub fn setTimerType_0<RetType, T: QDeadlineTimer_setTimerType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setTimerType_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_setTimerType_0<RetType> {
  fn setTimerType_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_setTimerType_0<(/*void*/)> for (i32) {
  fn setTimerType_0(self , rsthis: & QDeadlineTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer12setTimerTypeEN2Qt9TimerTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 remainingTime() const

/*
Returns the remaining time in this QDeadlineTimer object in milliseconds. If the timer has already expired, this function will return zero and it is not possible to obtain the amount of time overdue with this function (to do that, see deadline()). If the timer was set to never expire, this function returns -1.

This function is suitable for use in Qt APIs that take a millisecond timeout, such as the many QIODevice waitFor functions or the timed lock functions in QMutex, QWaitCondition, QSemaphore, or QReadWriteLock. For example:


  mutex.tryLock(deadline.remainingTime());



See also setRemainingTime(), remainingTimeNSecs(), isForever(), and hasExpired().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn remainingTime_0<RetType, T: QDeadlineTimer_remainingTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remainingTime_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_remainingTime_0<RetType> {
  fn remainingTime_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_remainingTime_0<i64> for () {
  fn remainingTime_0(self , rsthis: & QDeadlineTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer13remainingTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:83
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 remainingTimeNSecs() const

/*
Returns the remaining time in this QDeadlineTimer object in nanoseconds. If the timer has already expired, this function will return zero and it is not possible to obtain the amount of time overdue with this function. If the timer was set to never expire, this function returns -1.

See also remainingTime(), isForever(), and hasExpired().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn remainingTimeNSecs_0<RetType, T: QDeadlineTimer_remainingTimeNSecs_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remainingTimeNSecs_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_remainingTimeNSecs_0<RetType> {
  fn remainingTimeNSecs_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_remainingTimeNSecs_0<i64> for () {
  fn remainingTimeNSecs_0(self , rsthis: & QDeadlineTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer18remainingTimeNSecsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRemainingTime(qint64, Qt::TimerType)

/*
Sets the remaining time for this QDeadlineTimer object to msecs milliseconds from now, if msecs has a positive value. If msecs is zero, this QDeadlineTimer object will be marked as expired, whereas a value of -1 will set it to never expire.

The timer type for this QDeadlineTimer object will be set to the specified timerType.

See also setPreciseRemainingTime(), hasExpired(), isForever(), and remainingTime().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn setRemainingTime_0<RetType, T: QDeadlineTimer_setRemainingTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRemainingTime_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_setRemainingTime_0<RetType> {
  fn setRemainingTime_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_setRemainingTime_0<(/*void*/)> for (i64,i32) {
  fn setRemainingTime_0(self , rsthis: & QDeadlineTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer16setRemainingTimeExN2Qt9TimerTypeE", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreciseRemainingTime(qint64, qint64, Qt::TimerType)

/*
Sets the remaining time for this QDeadlineTimer object to secs seconds plus nsecs nanoseconds from now, if secs has a positive value. If secs is -1, this QDeadlineTimer will be set it to never expire. If both parameters are zero, this QDeadlineTimer will be marked as expired.

The timer type for this QDeadlineTimer object will be set to the specified timerType.

See also setRemainingTime(), hasExpired(), isForever(), and remainingTime().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn setPreciseRemainingTime_0<RetType, T: QDeadlineTimer_setPreciseRemainingTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreciseRemainingTime_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_setPreciseRemainingTime_0<RetType> {
  fn setPreciseRemainingTime_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_setPreciseRemainingTime_0<(/*void*/)> for (i64,i64,i32) {
  fn setPreciseRemainingTime_0(self , rsthis: & QDeadlineTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer23setPreciseRemainingTimeExxN2Qt9TimerTypeE", 3,qtrt::FFITY_SINT64,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 deadline() const

/*
Returns the absolute time point for the deadline stored in QDeadlineTimer object, calculated in milliseconds relative to the reference clock, the same as QElapsedTimer::msecsSinceReference(). The value will be in the past if this QDeadlineTimer has expired.

If this QDeadlineTimer never expires, this function returns std::numeric_limits<qint64>::max().

This function can be used to calculate the amount of time a timer is overdue, by subtracting QDeadlineTimer::current() or QElapsedTimer::msecsSinceReference(), as in the following example:


  qint64 realTimeLeft = deadline.deadline();
  if (realTimeLeft != (std::numeric_limits<qint64>::max)()) {
      realTimeLeft -= QDeadlineTimer::current().deadline();
      // or:
      //QElapsedTimer timer;
      //timer.start();
      //realTimeLeft -= timer.msecsSinceReference();
  }



Note: Timers that were created as expired have an indetermine time point in the past as their deadline, so the above calculation may not work.

See also remainingTime(), deadlineNSecs(), and setDeadline().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn deadline_0<RetType, T: QDeadlineTimer_deadline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deadline_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_deadline_0<RetType> {
  fn deadline_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_deadline_0<i64> for () {
  fn deadline_0(self , rsthis: & QDeadlineTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer8deadlineEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:89
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 deadlineNSecs() const

/*
Returns the absolute time point for the deadline stored in QDeadlineTimer object, calculated in nanoseconds relative to the reference clock, the same as QElapsedTimer::msecsSinceReference(). The value will be in the past if this QDeadlineTimer has expired.

If this QDeadlineTimer never expires, this function returns std::numeric_limits<qint64>::max().

This function can be used to calculate the amount of time a timer is overdue, by subtracting QDeadlineTimer::current(), as in the following example:


  qint64 realTimeLeft = deadline.deadlineNSecs();
  if (realTimeLeft != std::numeric_limits<qint64>::max())
      realTimeLeft -= QDeadlineTimer::current().deadlineNSecs();



Note: Timers that were created as expired have an indetermine time point in the past as their deadline, so the above calculation may not work.

See also remainingTime() and deadlineNSecs().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn deadlineNSecs_0<RetType, T: QDeadlineTimer_deadlineNSecs_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.deadlineNSecs_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_deadlineNSecs_0<RetType> {
  fn deadlineNSecs_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_deadlineNSecs_0<i64> for () {
  fn deadlineNSecs_0(self , rsthis: & QDeadlineTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer13deadlineNSecsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setDeadline(qint64, Qt::TimerType)

/*
Sets the deadline for this QDeadlineTimer object to be the msecs absolute time point, counted in milliseconds since the reference clock (the same as QElapsedTimer::msecsSinceReference()), and the timer type to timerType. If the value is in the past, this QDeadlineTimer will be marked as expired.

If msecs is std::numeric_limits<qint64>::max(), this QDeadlineTimer will be set to never expire.

See also setPreciseDeadline(), deadline(), deadlineNSecs(), and setRemainingTime().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn setDeadline_0<RetType, T: QDeadlineTimer_setDeadline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setDeadline_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_setDeadline_0<RetType> {
  fn setDeadline_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_setDeadline_0<(/*void*/)> for (i64,i32) {
  fn setDeadline_0(self , rsthis: & QDeadlineTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer11setDeadlineExN2Qt9TimerTypeE", 2,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPreciseDeadline(qint64, qint64, Qt::TimerType)

/*
Sets the deadline for this QDeadlineTimer object to be secs seconds and nsecs nanoseconds since the reference clock epoch (the same as QElapsedTimer::msecsSinceReference()), and the timer type to timerType. If the value is in the past, this QDeadlineTimer will be marked as expired.

If secs or nsecs is std::numeric_limits<qint64>::max(), this QDeadlineTimer will be set to never expire. If nsecs is more than 1 billion nanoseconds (1 second), then secs will be adjusted accordingly.

See also setDeadline(), deadline(), deadlineNSecs(), and setRemainingTime().
*/
impl /*struct*/ QDeadlineTimer {
  pub fn setPreciseDeadline_0<RetType, T: QDeadlineTimer_setPreciseDeadline_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPreciseDeadline_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_setPreciseDeadline_0<RetType> {
  fn setPreciseDeadline_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_setPreciseDeadline_0<(/*void*/)> for (i64,i64,i32) {
  fn setPreciseDeadline_0(self , rsthis: & QDeadlineTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer18setPreciseDeadlineExxN2Qt9TimerTypeE", 3,qtrt::FFITY_SINT64,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [16] QDeadlineTimer addNSecs(QDeadlineTimer, qint64)

/*
Returns a QDeadlineTimer object whose deadline is extended from dt's deadline by nsecs nanoseconds. If dt was set to never expire, this function returns a QDeadlineTimer that will not expire either.

Note: if dt was created as expired, its deadline is indeterminate and adding an amount of time may or may not cause it to become unexpired.
*/
impl /*struct*/ QDeadlineTimer {
  pub fn addNSecs_0<RetType, T: QDeadlineTimer_addNSecs_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addNSecs_0();
    // return 1;
  }
}
pub trait QDeadlineTimer_addNSecs_0<RetType> {
  fn addNSecs_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_addNSecs_0<usize> for (usize,i64) {
  fn addNSecs_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer8addNSecsES_x", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:95
// index:0
// Public static Visibility=Default Availability=Available
// [16] QDeadlineTimer current(Qt::TimerType)

/*
Returns a QDeadlineTimer that is expired but is guaranteed to contain the current time. Objects created by this function can participate in the calculation of how long a timer is overdue, using the deadline() function.

The QDeadlineTimer object will be constructed with the specified timerType.
*/
impl /*struct*/ QDeadlineTimer {
  pub fn current_0<RetType, T: QDeadlineTimer_current_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.current_0();
    // return 1;
  }
}
pub trait QDeadlineTimer_current_0<RetType> {
  fn current_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_current_0<usize> for (i32) {
  fn current_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimer7currentEN2Qt9TimerTypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:118
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QDeadlineTimer & operator+=(qint64)

/*

*/
impl /*struct*/ QDeadlineTimer {
  pub fn operator_add_equal_0<RetType, T: QDeadlineTimer_operator_add_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_add_equal_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_operator_add_equal_0<RetType> {
  fn operator_add_equal_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_operator_add_equal_0<usize> for (i64) {
  fn operator_add_equal_0(self , rsthis: & QDeadlineTimer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimerpLEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [16] QDeadlineTimer & operator-=(qint64)

/*

*/
impl /*struct*/ QDeadlineTimer {
  pub fn operator_minus_equal_0<RetType, T: QDeadlineTimer_operator_minus_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_minus_equal_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_operator_minus_equal_0<RetType> {
  fn operator_minus_equal_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_operator_minus_equal_0<usize> for (i64) {
  fn operator_minus_equal_0(self , rsthis: & QDeadlineTimer) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QDeadlineTimermIEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdeadlinetimer.h:162
// index:0
// Public inline Visibility=Default Availability=Available
// [8] std::chrono::nanoseconds remainingTimeAsDuration() const

/*
Returns the time remaining before the deadline.
*/
impl /*struct*/ QDeadlineTimer {
  pub fn remainingTimeAsDuration_0<RetType, T: QDeadlineTimer_remainingTimeAsDuration_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remainingTimeAsDuration_0(self);
    // return 1;
  }
}
pub trait QDeadlineTimer_remainingTimeAsDuration_0<RetType> {
  fn remainingTimeAsDuration_0(self , rsthis: & QDeadlineTimer) -> RetType;
}
impl<'a> /*trait*/ QDeadlineTimer_remainingTimeAsDuration_0<i32> for () {
  fn remainingTimeAsDuration_0(self , rsthis: & QDeadlineTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK14QDeadlineTimer23remainingTimeAsDurationEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQDeadlineTimer(this :*mut QDeadlineTimer) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN14QDeadlineTimerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*

*/
pub type QDeadlineTimer__ForeverConstant = i32;
// Used when creating a QDeadlineTimer to indicate the deadline should not expire
pub const QDeadlineTimer__Forever :QDeadlineTimer__ForeverConstant = 0;
pub fn QDeadlineTimer_ForeverConstantItemName(val: i32) ->String {
  match val {
     QDeadlineTimer__Forever => // 0
     {return String::from("Forever");}
  _ => {return format!("{}", val);}
}
}
pub fn QDeadlineTimer_ForeverConstantItemName_s(val: i32) ->String {
  //var nilthis *QDeadlineTimer
  //return nilthis.ForeverConstantItemName(val);
  return QDeadlineTimer_ForeverConstantItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
