

// mod ::core::QWaitCondition
// package qtcore
// /usr/include/qt/QtCore/qwaitcondition.h
// #include <qwaitcondition.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 23
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
#[derive(Default)] // class sizeof(QWaitCondition)=8
pub struct QWaitCondition {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWaitCondition_ITF interface {
//    QWaitCondition_PTR() *QWaitCondition
//}
//func (ptr *QWaitCondition) QWaitCondition_PTR() *QWaitCondition { return ptr }

impl /*struct*/ QWaitCondition {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWaitCondition {
    return QWaitCondition{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWaitCondition {
//  type Target = QWaitConditionBASE;
//
//  fn deref(&self) -> &QWaitConditionBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWaitConditionBASE> for QWaitCondition {
//  fn as_ref(& self) -> & QWaitConditionBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qwaitcondition.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QWaitCondition()

/*
Constructs a new wait condition object.
*/
// QWaitCondition() ctx.fn_proto_cpp
impl /*struct*/ QWaitCondition {
  pub fn QWaitCondition_0<T: QWaitCondition_QWaitCondition_0>(value: T) -> QWaitCondition {
    let rsthis = value.QWaitCondition_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWaitCondition_QWaitCondition_0 {
  fn QWaitCondition_0(self) -> QWaitCondition;
}
// QWaitCondition() ctx.fn_proto_cpp
impl<'a> /*trait*/ QWaitCondition_QWaitCondition_0 for () {
  fn QWaitCondition_0(self) -> QWaitCondition {
    // unsafe{_ZN14QWaitConditionC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QWaitConditionC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWaitCondition{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qwaitcondition.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QWaitCondition()

/*

*/
pub fn DeleteQWaitCondition(this :*mut QWaitCondition) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QWaitConditionD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qwaitcondition.h:62
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wait(QMutex *, unsigned long)

/*
Releases the lockedMutex and waits on the wait condition. The lockedMutex must be initially locked by the calling thread. If lockedMutex is not in a locked state, the behavior is undefined. If lockedMutex is a recursive mutex, this function returns immediately. The lockedMutex will be unlocked, and the calling thread will block until either of these conditions is met:


Another thread signals it using wakeOne() or wakeAll(). This function will return true in this case.
time milliseconds has elapsed. If time is ULONG_MAX (the default), then the wait will never timeout (the event must be signalled). This function will return false if the wait timed out.


The lockedMutex will be returned to the same locked state. This function is provided to allow the atomic transition from the locked state to the wait state.

See also wakeOne() and wakeAll().
*/
impl /*struct*/ QWaitCondition {
  pub fn wait_0<RetType, T: QWaitCondition_wait_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wait_0(self);
    // return 1;
  }
}
pub trait QWaitCondition_wait_0<RetType> {
  fn wait_0(self , rsthis: & QWaitCondition) -> RetType;
}
impl<'a> /*trait*/ QWaitCondition_wait_0<bool> for (usize,u64) {
  fn wait_0(self , rsthis: & QWaitCondition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QWaitCondition4waitEP6QMutexm", 2,qtrt::FFITY_POINTER,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qwaitcondition.h:63
// index:1
// Public Visibility=Default Availability=Available
// [1] bool wait(QReadWriteLock *, unsigned long)

/*
Releases the lockedMutex and waits on the wait condition. The lockedMutex must be initially locked by the calling thread. If lockedMutex is not in a locked state, the behavior is undefined. If lockedMutex is a recursive mutex, this function returns immediately. The lockedMutex will be unlocked, and the calling thread will block until either of these conditions is met:


Another thread signals it using wakeOne() or wakeAll(). This function will return true in this case.
time milliseconds has elapsed. If time is ULONG_MAX (the default), then the wait will never timeout (the event must be signalled). This function will return false if the wait timed out.


The lockedMutex will be returned to the same locked state. This function is provided to allow the atomic transition from the locked state to the wait state.

See also wakeOne() and wakeAll().
*/
impl /*struct*/ QWaitCondition {
  pub fn wait_1<RetType, T: QWaitCondition_wait_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wait_1(self);
    // return 1;
  }
}
pub trait QWaitCondition_wait_1<RetType> {
  fn wait_1(self , rsthis: & QWaitCondition) -> RetType;
}
impl<'a> /*trait*/ QWaitCondition_wait_1<bool> for (usize,u64) {
  fn wait_1(self , rsthis: & QWaitCondition) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QWaitCondition4waitEP14QReadWriteLockm", 2,qtrt::FFITY_POINTER,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qwaitcondition.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void wakeOne()

/*
Wakes one thread waiting on the wait condition. The thread that is woken up depends on the operating system's scheduling policies, and cannot be controlled or predicted.

If you want to wake up a specific thread, the solution is typically to use different wait conditions and have different threads wait on different conditions.

See also wakeAll().
*/
impl /*struct*/ QWaitCondition {
  pub fn wakeOne_0<RetType, T: QWaitCondition_wakeOne_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wakeOne_0(self);
    // return 1;
  }
}
pub trait QWaitCondition_wakeOne_0<RetType> {
  fn wakeOne_0(self , rsthis: & QWaitCondition) -> RetType;
}
impl<'a> /*trait*/ QWaitCondition_wakeOne_0<(/*void*/)> for () {
  fn wakeOne_0(self , rsthis: & QWaitCondition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QWaitCondition7wakeOneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qwaitcondition.h:66
// index:0
// Public Visibility=Default Availability=Available
// [-2] void wakeAll()

/*
Wakes all threads waiting on the wait condition. The order in which the threads are woken up depends on the operating system's scheduling policies and cannot be controlled or predicted.

See also wakeOne().
*/
impl /*struct*/ QWaitCondition {
  pub fn wakeAll_0<RetType, T: QWaitCondition_wakeAll_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wakeAll_0(self);
    // return 1;
  }
}
pub trait QWaitCondition_wakeAll_0<RetType> {
  fn wakeAll_0(self , rsthis: & QWaitCondition) -> RetType;
}
impl<'a> /*trait*/ QWaitCondition_wakeAll_0<(/*void*/)> for () {
  fn wakeAll_0(self , rsthis: & QWaitCondition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QWaitCondition7wakeAllEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qwaitcondition.h:68
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void notify_one()

/*
This function is provided for STL compatibility. It is equivalent to wakeOne().

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QWaitCondition {
  pub fn notify_one_0<RetType, T: QWaitCondition_notify_one_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notify_one_0(self);
    // return 1;
  }
}
pub trait QWaitCondition_notify_one_0<RetType> {
  fn notify_one_0(self , rsthis: & QWaitCondition) -> RetType;
}
impl<'a> /*trait*/ QWaitCondition_notify_one_0<(/*void*/)> for () {
  fn notify_one_0(self , rsthis: & QWaitCondition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QWaitCondition10notify_oneEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qwaitcondition.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void notify_all()

/*
This function is provided for STL compatibility. It is equivalent to wakeAll().

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QWaitCondition {
  pub fn notify_all_0<RetType, T: QWaitCondition_notify_all_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.notify_all_0(self);
    // return 1;
  }
}
pub trait QWaitCondition_notify_all_0<RetType> {
  fn notify_all_0(self , rsthis: & QWaitCondition) -> RetType;
}
impl<'a> /*trait*/ QWaitCondition_notify_all_0<(/*void*/)> for () {
  fn notify_all_0(self , rsthis: & QWaitCondition) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QWaitCondition10notify_allEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
