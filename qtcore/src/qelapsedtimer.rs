

// mod ::core::QElapsedTimer
// package qtcore
// /usr/include/qt/QtCore/qelapsedtimer.h
// #include <qelapsedtimer.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 67
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
#[derive(Default)] // class sizeof(QElapsedTimer)=16
pub struct QElapsedTimer {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QElapsedTimer_ITF interface {
//    QElapsedTimer_PTR() *QElapsedTimer
//}
//func (ptr *QElapsedTimer) QElapsedTimer_PTR() *QElapsedTimer { return ptr }

impl /*struct*/ QElapsedTimer {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QElapsedTimer {
    return QElapsedTimer{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QElapsedTimer {
//  type Target = QElapsedTimerBASE;
//
//  fn deref(&self) -> &QElapsedTimerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QElapsedTimerBASE> for QElapsedTimer {
//  fn as_ref(& self) -> & QElapsedTimerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qelapsedtimer.h:59
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QElapsedTimer()

/*
Constructs an invalid QElapsedTimer. A timer becomes valid once it has been started.

This function was introduced in  Qt 5.4.

See also isValid() and start().
*/
// QElapsedTimer() ctx.fn_proto_cpp
impl /*struct*/ QElapsedTimer {
  pub fn QElapsedTimer_0<T: QElapsedTimer_QElapsedTimer_0>(value: T) -> QElapsedTimer {
    let rsthis = value.QElapsedTimer_0();
    return rsthis;
    // return 1;
  }
}

pub trait QElapsedTimer_QElapsedTimer_0 {
  fn QElapsedTimer_0(self) -> QElapsedTimer;
}
// QElapsedTimer() ctx.fn_proto_cpp
impl<'a> /*trait*/ QElapsedTimer_QElapsedTimer_0 for () {
  fn QElapsedTimer_0(self) -> QElapsedTimer {
    // unsafe{_ZN13QElapsedTimerC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QElapsedTimerC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QElapsedTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:65
// index:0
// Public static Visibility=Default Availability=Available
// [4] QElapsedTimer::ClockType clockType()

/*
Returns the clock type that this QElapsedTimer implementation uses.

See also isMonotonic().
*/
impl /*struct*/ QElapsedTimer {
  pub fn clockType_0<RetType, T: QElapsedTimer_clockType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.clockType_0();
    // return 1;
  }
}
pub trait QElapsedTimer_clockType_0<RetType> {
  fn clockType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_clockType_0<i32> for () {
  fn clockType_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QElapsedTimer9clockTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:66
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isMonotonic()

/*
Returns true if this is a monotonic clock, false otherwise. See the information on the different clock types to understand which ones are monotonic.

See also clockType() and QElapsedTimer::ClockType.
*/
impl /*struct*/ QElapsedTimer {
  pub fn isMonotonic_0<RetType, T: QElapsedTimer_isMonotonic_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isMonotonic_0();
    // return 1;
  }
}
pub trait QElapsedTimer_isMonotonic_0<RetType> {
  fn isMonotonic_0(self ) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_isMonotonic_0<bool> for () {
  fn isMonotonic_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QElapsedTimer11isMonotonicEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start()

/*
Starts this timer. Once started, a timer value can be checked with elapsed() or msecsSinceReference().

Normally, a timer is started just before a lengthy operation, such as:


      QElapsedTimer timer;
      timer.start();

      slowOperation1();

      qDebug() << "The slow operation took" << timer.elapsed() << "milliseconds";



Also, starting a timer makes it valid again.

See also restart(), invalidate(), and elapsed().
*/
impl /*struct*/ QElapsedTimer {
  pub fn start_0<RetType, T: QElapsedTimer_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_start_0<RetType> {
  fn start_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_start_0<(/*void*/)> for () {
  fn start_0(self , rsthis: & QElapsedTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QElapsedTimer5startEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:69
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 restart()

/*
Restarts the timer and returns the time elapsed since the previous start. This function is equivalent to obtaining the elapsed time with elapsed() and then starting the timer again with start(), but it does so in one single operation, avoiding the need to obtain the clock value twice.

Calling this function on a QElapsedTimer that is invalid results in undefined behavior.

The following example illustrates how to use this function to calibrate a parameter to a slow operation (for example, an iteration count) so that this operation takes at least 250 milliseconds:


      QElapsedTimer timer;

      int count = 1;
      timer.start();
      do {
          count *= 2;
          slowOperation2(count);
      } while (timer.restart() < 250);

      return count;



See also start(), invalidate(), elapsed(), and isValid().
*/
impl /*struct*/ QElapsedTimer {
  pub fn restart_0<RetType, T: QElapsedTimer_restart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.restart_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_restart_0<RetType> {
  fn restart_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_restart_0<i64> for () {
  fn restart_0(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QElapsedTimer7restartEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void invalidate()

/*
Marks this QElapsedTimer object as invalid.

An invalid object can be checked with isValid(). Calculations of timer elapsed since invalid data are undefined and will likely produce bizarre results.

See also isValid(), start(), and restart().
*/
impl /*struct*/ QElapsedTimer {
  pub fn invalidate_0<RetType, T: QElapsedTimer_invalidate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.invalidate_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_invalidate_0<RetType> {
  fn invalidate_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_invalidate_0<(/*void*/)> for () {
  fn invalidate_0(self , rsthis: & QElapsedTimer) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QElapsedTimer10invalidateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns false if the timer has never been started or invalidated by a call to invalidate().

See also invalidate(), start(), and restart().
*/
impl /*struct*/ QElapsedTimer {
  pub fn isValid_0<RetType, T: QElapsedTimer_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QElapsedTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:73
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 nsecsElapsed() const

/*
Returns the number of nanoseconds since this QElapsedTimer was last started.

Calling this function on a QElapsedTimer that is invalid results in undefined behavior.

On platforms that do not provide nanosecond resolution, the value returned will be the best estimate available.

This function was introduced in  Qt 4.8.

See also start(), restart(), hasExpired(), and invalidate().
*/
impl /*struct*/ QElapsedTimer {
  pub fn nsecsElapsed_0<RetType, T: QElapsedTimer_nsecsElapsed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nsecsElapsed_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_nsecsElapsed_0<RetType> {
  fn nsecsElapsed_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_nsecsElapsed_0<i64> for () {
  fn nsecsElapsed_0(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer12nsecsElapsedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 elapsed() const

/*
Returns the number of milliseconds since this QElapsedTimer was last started.

Calling this function on a QElapsedTimer that is invalid results in undefined behavior.

See also start(), restart(), hasExpired(), isValid(), and invalidate().
*/
impl /*struct*/ QElapsedTimer {
  pub fn elapsed_0<RetType, T: QElapsedTimer_elapsed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.elapsed_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_elapsed_0<RetType> {
  fn elapsed_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_elapsed_0<i64> for () {
  fn elapsed_0(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer7elapsedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool hasExpired(qint64) const

/*
Returns true if this QElapsedTimer has already expired by timeout milliseconds (that is, more than timeout milliseconds have elapsed). The value of timeout can be -1 to indicate that this timer does not expire, in which case this function will always return false.

See also elapsed() and QDeadlineTimer.
*/
impl /*struct*/ QElapsedTimer {
  pub fn hasExpired_0<RetType, T: QElapsedTimer_hasExpired_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasExpired_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_hasExpired_0<RetType> {
  fn hasExpired_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_hasExpired_0<bool> for (i64) {
  fn hasExpired_0(self , rsthis: & QElapsedTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer10hasExpiredEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 msecsSinceReference() const

/*
Returns the number of milliseconds between last time this QElapsedTimer object was started and its reference clock's start.

This number is usually arbitrary for all clocks except the QElapsedTimer::SystemTime clock. For that clock type, this number is the number of milliseconds since January 1st, 1970 at 0:00 UTC (that is, it is the Unix time expressed in milliseconds).

On Linux, Windows and Apple platforms, this value is usually the time since the system boot, though it usually does not include the time the system has spent in sleep states.

See also clockType() and elapsed().
*/
impl /*struct*/ QElapsedTimer {
  pub fn msecsSinceReference_0<RetType, T: QElapsedTimer_msecsSinceReference_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.msecsSinceReference_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_msecsSinceReference_0<RetType> {
  fn msecsSinceReference_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_msecsSinceReference_0<i64> for () {
  fn msecsSinceReference_0(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer19msecsSinceReferenceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 msecsTo(const QElapsedTimer &) const

/*
Returns the number of milliseconds between this QElapsedTimer and other. If other was started before this object, the returned value will be negative. If it was started later, the returned value will be positive.

The return value is undefined if this object or other were invalidated.

See also secsTo() and elapsed().
*/
impl /*struct*/ QElapsedTimer {
  pub fn msecsTo_0<RetType, T: QElapsedTimer_msecsTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.msecsTo_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_msecsTo_0<RetType> {
  fn msecsTo_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_msecsTo_0<i64> for (usize) {
  fn msecsTo_0(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer7msecsToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 secsTo(const QElapsedTimer &) const

/*
Returns the number of seconds between this QElapsedTimer and other. If other was started before this object, the returned value will be negative. If it was started later, the returned value will be positive.

Calling this function on or with a QElapsedTimer that is invalid results in undefined behavior.

See also msecsTo() and elapsed().
*/
impl /*struct*/ QElapsedTimer {
  pub fn secsTo_0<RetType, T: QElapsedTimer_secsTo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.secsTo_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_secsTo_0<RetType> {
  fn secsTo_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_secsTo_0<i64> for (usize) {
  fn secsTo_0(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimer6secsToERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:81
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QElapsedTimer &) const

/*

*/
impl /*struct*/ QElapsedTimer {
  pub fn operator_equal_equal_0<RetType, T: QElapsedTimer_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QElapsedTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimereqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qelapsedtimer.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QElapsedTimer &) const

/*

*/
impl /*struct*/ QElapsedTimer {
  pub fn operator_not_equal_0<RetType, T: QElapsedTimer_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QElapsedTimer_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QElapsedTimer) -> RetType;
}
impl<'a> /*trait*/ QElapsedTimer_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QElapsedTimer) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QElapsedTimerneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQElapsedTimer(this :*mut QElapsedTimer) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN13QElapsedTimerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}

/*
This enum contains the different clock types that QElapsedTimer may use.

QElapsedTimer will always use the same clock type in a particular machine, so this value will not change during the lifetime of a program. It is provided so that QElapsedTimer can be used with other non-Qt implementations, to guarantee that the same reference clock is being used.


*/
pub type QElapsedTimer__ClockType = i32;
// The human-readable system time. This clock is not monotonic.
pub const QElapsedTimer__SystemTime :QElapsedTimer__ClockType = 0;
// The system's monotonic clock, usually found in Unix systems. This clock is monotonic and does not overflow.
pub const QElapsedTimer__MonotonicClock :QElapsedTimer__ClockType = 1;
// The system's tick counter, used on Windows systems. This clock may overflow.
pub const QElapsedTimer__TickCounter :QElapsedTimer__ClockType = 2;
// The Mach kernel's absolute time (macOS and iOS). This clock is monotonic and does not overflow.
pub const QElapsedTimer__MachAbsoluteTime :QElapsedTimer__ClockType = 3;
// The high-resolution performance counter provided by Windows. This clock is monotonic and does not overflow.
pub const QElapsedTimer__PerformanceCounter :QElapsedTimer__ClockType = 4;
pub fn QElapsedTimer_ClockTypeItemName(val: i32) ->String {
  match val {
     QElapsedTimer__SystemTime => // 0
     {return String::from("SystemTime");}
     QElapsedTimer__MonotonicClock => // 1
     {return String::from("MonotonicClock");}
     QElapsedTimer__TickCounter => // 2
     {return String::from("TickCounter");}
     QElapsedTimer__MachAbsoluteTime => // 3
     {return String::from("MachAbsoluteTime");}
     QElapsedTimer__PerformanceCounter => // 4
     {return String::from("PerformanceCounter");}
  _ => {return format!("{}", val);}
}
}
pub fn QElapsedTimer_ClockTypeItemName_s(val: i32) ->String {
  //var nilthis *QElapsedTimer
  //return nilthis.ClockTypeItemName(val);
  return QElapsedTimer_ClockTypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
