

// mod ::core::QAbstractEventDispatcher
// package qtcore
// /usr/include/qt/QtCore/qabstracteventdispatcher.h
// #include <qabstracteventdispatcher.h>
// #include <QtCore>

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
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QAbstractEventDispatcher)=16
pub struct QAbstractEventDispatcher {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAbstractEventDispatcher_ITF interface {
//    QObject_ITF
//    QAbstractEventDispatcher_PTR() *QAbstractEventDispatcher
//}
//func (ptr *QAbstractEventDispatcher) QAbstractEventDispatcher_PTR() *QAbstractEventDispatcher { return ptr }

impl /*struct*/ QAbstractEventDispatcher {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAbstractEventDispatcher {
    return QAbstractEventDispatcher{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAbstractEventDispatcher {
//  type Target = QAbstractEventDispatcherBASE;
//
//  fn deref(&self) -> &QAbstractEventDispatcherBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAbstractEventDispatcherBASE> for QAbstractEventDispatcher {
//  fn as_ref(& self) -> & QAbstractEventDispatcherBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qabstracteventdispatcher.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn metaObject_0<RetType, T: QAbstractEventDispatcher_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QAbstractEventDispatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK24QAbstractEventDispatcher10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QAbstractEventDispatcher(QObject *)

/*
Constructs a new event dispatcher with the given parent.
*/
// QAbstractEventDispatcher(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QAbstractEventDispatcher {
  pub fn QAbstractEventDispatcher_0<T: QAbstractEventDispatcher_QAbstractEventDispatcher_0>(value: T) -> QAbstractEventDispatcher {
    let rsthis = value.QAbstractEventDispatcher_0();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractEventDispatcher_QAbstractEventDispatcher_0 {
  fn QAbstractEventDispatcher_0(self) -> QAbstractEventDispatcher;
}
// QAbstractEventDispatcher(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QAbstractEventDispatcher_QAbstractEventDispatcher_0 for (usize) {
  fn QAbstractEventDispatcher_0(self) -> QAbstractEventDispatcher {
    // unsafe{_ZN24QAbstractEventDispatcherC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcherC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QAbstractEventDispatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:74
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QAbstractEventDispatcher()

/*

*/
pub fn DeleteQAbstractEventDispatcher(this :*mut QAbstractEventDispatcher) {
    // let rv = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcherD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qabstracteventdispatcher.h:76
// index:0
// Public static Visibility=Default Availability=Available
// [8] QAbstractEventDispatcher * instance(QThread *)

/*
Returns a pointer to the event dispatcher object for the specified thread. If thread is zero, the current thread is used. If no event dispatcher exists for the specified thread, this function returns 0.

Note: If Qt is built without thread support, the thread argument is ignored.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn instance_0<RetType, T: QAbstractEventDispatcher_instance_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.instance_0();
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_instance_0<RetType> {
  fn instance_0(self ) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_instance_0<usize> for (usize) {
  fn instance_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher8instanceEP7QThread", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:78
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool processEvents(QEventLoop::ProcessEventsFlags)

/*
Processes pending events that match flags until there are no more events to process. Returns true if an event was processed; otherwise returns false.

This function is especially useful if you have a long running operation, and want to show its progress without allowing user input by using the QEventLoop::ExcludeUserInputEvents flag.

If the QEventLoop::WaitForMoreEvents flag is set in flags, the behavior of this function is as follows:


If events are available, this function returns after processing them.
If no events are available, this function will wait until more are available and return after processing newly available events.


If the QEventLoop::WaitForMoreEvents flag is not set in flags, and no events are available, this function will return immediately.

Note: This function does not process events continuously; it returns after all available events are processed.

See also hasPendingEvents().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn processEvents_0<RetType, T: QAbstractEventDispatcher_processEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processEvents_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_processEvents_0<RetType> {
  fn processEvents_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_processEvents_0<bool> for (i32) {
  fn processEvents_0(self , rsthis: & QAbstractEventDispatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher13processEventsE6QFlagsIN10QEventLoop17ProcessEventsFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:79
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool hasPendingEvents()

/*

*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn hasPendingEvents_0<RetType, T: QAbstractEventDispatcher_hasPendingEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hasPendingEvents_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_hasPendingEvents_0<RetType> {
  fn hasPendingEvents_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_hasPendingEvents_0<bool> for () {
  fn hasPendingEvents_0(self , rsthis: & QAbstractEventDispatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher16hasPendingEventsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:81
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void registerSocketNotifier(QSocketNotifier *)

/*
Registers notifier with the event loop. Subclasses must implement this method to tie a socket notifier into another event loop.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn registerSocketNotifier_0<RetType, T: QAbstractEventDispatcher_registerSocketNotifier_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.registerSocketNotifier_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_registerSocketNotifier_0<RetType> {
  fn registerSocketNotifier_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_registerSocketNotifier_0<(/*void*/)> for (usize) {
  fn registerSocketNotifier_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher22registerSocketNotifierEP15QSocketNotifier", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:82
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void unregisterSocketNotifier(QSocketNotifier *)

/*
Unregisters notifier from the event dispatcher. Subclasses must reimplement this method to tie a socket notifier into another event loop. Reimplementations must call the base implementation.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn unregisterSocketNotifier_0<RetType, T: QAbstractEventDispatcher_unregisterSocketNotifier_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unregisterSocketNotifier_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_unregisterSocketNotifier_0<RetType> {
  fn unregisterSocketNotifier_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_unregisterSocketNotifier_0<(/*void*/)> for (usize) {
  fn unregisterSocketNotifier_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher24unregisterSocketNotifierEP15QSocketNotifier", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int registerTimer(int, Qt::TimerType, QObject *)

/*
Registers a timer with the specified interval and timerType for the given object and returns the timer id.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn registerTimer_0<RetType, T: QAbstractEventDispatcher_registerTimer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.registerTimer_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_registerTimer_0<RetType> {
  fn registerTimer_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_registerTimer_0<i32> for (i32,i32,usize) {
  fn registerTimer_0(self , rsthis: & QAbstractEventDispatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher13registerTimerEiN2Qt9TimerTypeEP7QObject", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:91
// index:1
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void registerTimer(int, int, Qt::TimerType, QObject *)

/*
Registers a timer with the specified interval and timerType for the given object and returns the timer id.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn registerTimer_1<RetType, T: QAbstractEventDispatcher_registerTimer_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.registerTimer_1(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_registerTimer_1<RetType> {
  fn registerTimer_1(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_registerTimer_1<(/*void*/)> for (i32,i32,i32,usize) {
  fn registerTimer_1(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher13registerTimerEiiN2Qt9TimerTypeEP7QObject", 4,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:92
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool unregisterTimer(int)

/*
Unregisters the timer with the given timerId. Returns true if successful; otherwise returns false.

See also registerTimer() and unregisterTimers().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn unregisterTimer_0<RetType, T: QAbstractEventDispatcher_unregisterTimer_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unregisterTimer_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_unregisterTimer_0<RetType> {
  fn unregisterTimer_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_unregisterTimer_0<bool> for (i32) {
  fn unregisterTimer_0(self , rsthis: & QAbstractEventDispatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher15unregisterTimerEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:93
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [1] bool unregisterTimers(QObject *)

/*
Unregisters all the timers associated with the given object. Returns true if all timers were successful removed; otherwise returns false.

See also unregisterTimer() and registeredTimers().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn unregisterTimers_0<RetType, T: QAbstractEventDispatcher_unregisterTimers_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unregisterTimers_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_unregisterTimers_0<RetType> {
  fn unregisterTimers_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_unregisterTimers_0<bool> for (usize) {
  fn unregisterTimers_0(self , rsthis: & QAbstractEventDispatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher16unregisterTimersEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:96
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [4] int remainingTime(int)

/*
Returns the remaining time in milliseconds with the given timerId. If the timer is inactive, the returned value will be -1. If the timer is overdue, the returned value will be 0.

See also Qt::TimerType.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn remainingTime_0<RetType, T: QAbstractEventDispatcher_remainingTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remainingTime_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_remainingTime_0<RetType> {
  fn remainingTime_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_remainingTime_0<i32> for (i32) {
  fn remainingTime_0(self , rsthis: & QAbstractEventDispatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher13remainingTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:103
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void wakeUp()

/*
Wakes up the event loop.

Note: This function is thread-safe.

See also awake().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn wakeUp_0<RetType, T: QAbstractEventDispatcher_wakeUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wakeUp_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_wakeUp_0<RetType> {
  fn wakeUp_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_wakeUp_0<(/*void*/)> for () {
  fn wakeUp_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher6wakeUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:104
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void interrupt()

/*
Interrupts event dispatching. The event dispatcher will return from processEvents() as soon as possible.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn interrupt_0<RetType, T: QAbstractEventDispatcher_interrupt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.interrupt_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_interrupt_0<RetType> {
  fn interrupt_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_interrupt_0<(/*void*/)> for () {
  fn interrupt_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher9interruptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:105
// index:0
// Public purevirtual virtual Visibility=Default Availability=Available
// [-2] void flush()

/*

*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn flush_0<RetType, T: QAbstractEventDispatcher_flush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flush_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_flush_0<RetType> {
  fn flush_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_flush_0<(/*void*/)> for () {
  fn flush_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher5flushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:107
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void startingUp()

/*

*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn startingUp_0<RetType, T: QAbstractEventDispatcher_startingUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.startingUp_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_startingUp_0<RetType> {
  fn startingUp_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_startingUp_0<(/*void*/)> for () {
  fn startingUp_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher10startingUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:108
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void closingDown()

/*

*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn closingDown_0<RetType, T: QAbstractEventDispatcher_closingDown_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.closingDown_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_closingDown_0<RetType> {
  fn closingDown_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_closingDown_0<(/*void*/)> for () {
  fn closingDown_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher11closingDownEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void installNativeEventFilter(QAbstractNativeEventFilter *)

/*
Installs an event filter filterObj for all native events received by the application.

The event filter filterObj receives events via its nativeEventFilter() function, which is called for all events received by all threads.

The nativeEventFilter() function should return true if the event should be filtered, (in this case, stopped). It should return false to allow normal Qt processing to continue: the native event can then be translated into a QEvent and handled by the standard Qt event filtering, e.g. QObject::installEventFilter().

If multiple event filters are installed, the filter that was installed last is activated first.

Note: The filter function set here receives native messages, that is, MSG or XEvent structs.

For maximum portability, you should always try to use QEvent objects and QObject::installEventFilter() whenever possible.

This function was introduced in  Qt 5.0.

See also QObject::installEventFilter().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn installNativeEventFilter_0<RetType, T: QAbstractEventDispatcher_installNativeEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.installNativeEventFilter_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_installNativeEventFilter_0<RetType> {
  fn installNativeEventFilter_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_installNativeEventFilter_0<(/*void*/)> for (usize) {
  fn installNativeEventFilter_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher24installNativeEventFilterEP26QAbstractNativeEventFilter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeNativeEventFilter(QAbstractNativeEventFilter *)

/*
Removes the event filter filter from this object. The request is ignored if such an event filter has not been installed.

All event filters for this object are automatically removed when this object is destroyed.

It is always safe to remove an event filter, even during event filter filter activation (that is, even from within the nativeEventFilter() function).

This function was introduced in  Qt 5.0.

See also installNativeEventFilter() and QAbstractNativeEventFilter.
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn removeNativeEventFilter_0<RetType, T: QAbstractEventDispatcher_removeNativeEventFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeNativeEventFilter_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_removeNativeEventFilter_0<RetType> {
  fn removeNativeEventFilter_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_removeNativeEventFilter_0<(/*void*/)> for (usize) {
  fn removeNativeEventFilter_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher23removeNativeEventFilterEP26QAbstractNativeEventFilter", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:112
// index:0
// Public Visibility=Default Availability=Available
// [1] bool filterNativeEvent(const QByteArray &, void *, long *)

/*
Sends message through the event filters that were set by installNativeEventFilter(). This function returns true as soon as an event filter returns true, and false otherwise to indicate that the processing of the event should continue.

Subclasses of QAbstractEventDispatcher must call this function for all messages received from the system to ensure compatibility with any extensions that may be used in the application. The type of event eventType is specific to the platform plugin chosen at run-time, and can be used to cast message to the right type. The result pointer is only used on Windows, and corresponds to the LRESULT pointer.

Note that the type of message is platform dependent. See QAbstractNativeEventFilter for details.

This function was introduced in  Qt 5.0.

See also installNativeEventFilter() and QAbstractNativeEventFilter::nativeEventFilter().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn filterNativeEvent_0<RetType, T: QAbstractEventDispatcher_filterNativeEvent_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filterNativeEvent_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_filterNativeEvent_0<RetType> {
  fn filterNativeEvent_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_filterNativeEvent_0<bool> for (usize,usize,usize) {
  fn filterNativeEvent_0(self , rsthis: & QAbstractEventDispatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const usize as usize;
    let arg2 = (&self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher17filterNativeEventERK10QByteArrayPvPl", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void aboutToBlock()

/*
This signal is emitted before the event loop calls a function that could block.

See also awake().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn aboutToBlock_0<RetType, T: QAbstractEventDispatcher_aboutToBlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.aboutToBlock_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_aboutToBlock_0<RetType> {
  fn aboutToBlock_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_aboutToBlock_0<(/*void*/)> for () {
  fn aboutToBlock_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher12aboutToBlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qabstracteventdispatcher.h:120
// index:0
// Public Visibility=Default Availability=Available
// [-2] void awake()

/*
This signal is emitted after the event loop returns from a function that could block.

See also wakeUp() and aboutToBlock().
*/
impl /*struct*/ QAbstractEventDispatcher {
  pub fn awake_0<RetType, T: QAbstractEventDispatcher_awake_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.awake_0(self);
    // return 1;
  }
}
pub trait QAbstractEventDispatcher_awake_0<RetType> {
  fn awake_0(self , rsthis: & QAbstractEventDispatcher) -> RetType;
}
impl<'a> /*trait*/ QAbstractEventDispatcher_awake_0<(/*void*/)> for () {
  fn awake_0(self , rsthis: & QAbstractEventDispatcher) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN24QAbstractEventDispatcher5awakeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
