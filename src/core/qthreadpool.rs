

// mod ::core::QThreadPool
// package qtcore
// /usr/include/qt/QtCore/qthreadpool.h
// #include <qthreadpool.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 30
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
#[derive(Default)] // class sizeof(QThreadPool)=16
pub struct QThreadPool {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QThreadPool_ITF interface {
//    QObject_ITF
//    QThreadPool_PTR() *QThreadPool
//}
//func (ptr *QThreadPool) QThreadPool_PTR() *QThreadPool { return ptr }

impl /*struct*/ QThreadPool {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QThreadPool {
    return QThreadPool{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QThreadPool {
//  type Target = QThreadPoolBASE;
//
//  fn deref(&self) -> &QThreadPoolBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QThreadPoolBASE> for QThreadPool {
//  fn as_ref(& self) -> & QThreadPoolBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qthreadpool.h:56
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QThreadPool {
  pub fn metaObject_0<RetType, T: QThreadPool_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QThreadPool_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QThreadPool) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QThreadPool10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:65
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QThreadPool(QObject *)

/*
Constructs a thread pool with the given parent.
*/
// QThreadPool(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QThreadPool {
  pub fn QThreadPool_0<T: QThreadPool_QThreadPool_0>(value: T) -> QThreadPool {
    let rsthis = value.QThreadPool_0();
    return rsthis;
    // return 1;
  }
}

pub trait QThreadPool_QThreadPool_0 {
  fn QThreadPool_0(self) -> QThreadPool;
}
// QThreadPool(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QThreadPool_QThreadPool_0 for (usize) {
  fn QThreadPool_0(self) -> QThreadPool {
    // unsafe{_ZN11QThreadPoolC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QThreadPoolC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QThreadPool{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:66
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QThreadPool()

/*

*/
pub fn DeleteQThreadPool(this :*mut QThreadPool) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QThreadPoolD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qthreadpool.h:68
// index:0
// Public static Visibility=Default Availability=Available
// [8] QThreadPool * globalInstance()

/*
Returns the global QThreadPool instance.
*/
impl /*struct*/ QThreadPool {
  pub fn globalInstance_0<RetType, T: QThreadPool_globalInstance_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalInstance_0();
    // return 1;
  }
}
pub trait QThreadPool_globalInstance_0<RetType> {
  fn globalInstance_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_globalInstance_0<usize> for () {
  fn globalInstance_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QThreadPool14globalInstanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:70
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start(QRunnable *, int)

/*
Reserves a thread and uses it to run runnable, unless this thread will make the current thread count exceed maxThreadCount(). In that case, runnable is added to a run queue instead. The priority argument can be used to control the run queue's order of execution.

Note that the thread pool takes ownership of the runnable if runnable->autoDelete() returns true, and the runnable will be deleted automatically by the thread pool after the runnable->run() returns. If runnable->autoDelete() returns false, ownership of runnable remains with the caller. Note that changing the auto-deletion on runnable after calling this functions results in undefined behavior.
*/
impl /*struct*/ QThreadPool {
  pub fn start_0<RetType, T: QThreadPool_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QThreadPool_start_0<RetType> {
  fn start_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_start_0<(/*void*/)> for (usize,i32) {
  fn start_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QThreadPool5startEP9QRunnablei", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryStart(QRunnable *)

/*
Attempts to reserve a thread to run runnable.

If no threads are available at the time of calling, then this function does nothing and returns false. Otherwise, runnable is run immediately using one available thread and this function returns true.

Note that the thread pool takes ownership of the runnable if runnable->autoDelete() returns true, and the runnable will be deleted automatically by the thread pool after the runnable->run() returns. If runnable->autoDelete() returns false, ownership of runnable remains with the caller. Note that changing the auto-deletion on runnable after calling this function results in undefined behavior.
*/
impl /*struct*/ QThreadPool {
  pub fn tryStart_0<RetType, T: QThreadPool_tryStart_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryStart_0(self);
    // return 1;
  }
}
pub trait QThreadPool_tryStart_0<RetType> {
  fn tryStart_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_tryStart_0<bool> for (usize) {
  fn tryStart_0(self , rsthis: & QThreadPool) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QThreadPool8tryStartEP9QRunnable", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] int expiryTimeout() const

/*

*/
impl /*struct*/ QThreadPool {
  pub fn expiryTimeout_0<RetType, T: QThreadPool_expiryTimeout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expiryTimeout_0(self);
    // return 1;
  }
}
pub trait QThreadPool_expiryTimeout_0<RetType> {
  fn expiryTimeout_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_expiryTimeout_0<i32> for () {
  fn expiryTimeout_0(self , rsthis: & QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QThreadPool13expiryTimeoutEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExpiryTimeout(int)

/*

*/
impl /*struct*/ QThreadPool {
  pub fn setExpiryTimeout_0<RetType, T: QThreadPool_setExpiryTimeout_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExpiryTimeout_0(self);
    // return 1;
  }
}
pub trait QThreadPool_setExpiryTimeout_0<RetType> {
  fn setExpiryTimeout_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_setExpiryTimeout_0<(/*void*/)> for (i32) {
  fn setExpiryTimeout_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QThreadPool16setExpiryTimeoutEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:76
// index:0
// Public Visibility=Default Availability=Available
// [4] int maxThreadCount() const

/*

*/
impl /*struct*/ QThreadPool {
  pub fn maxThreadCount_0<RetType, T: QThreadPool_maxThreadCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.maxThreadCount_0(self);
    // return 1;
  }
}
pub trait QThreadPool_maxThreadCount_0<RetType> {
  fn maxThreadCount_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_maxThreadCount_0<i32> for () {
  fn maxThreadCount_0(self , rsthis: & QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QThreadPool14maxThreadCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMaxThreadCount(int)

/*

*/
impl /*struct*/ QThreadPool {
  pub fn setMaxThreadCount_0<RetType, T: QThreadPool_setMaxThreadCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMaxThreadCount_0(self);
    // return 1;
  }
}
pub trait QThreadPool_setMaxThreadCount_0<RetType> {
  fn setMaxThreadCount_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_setMaxThreadCount_0<(/*void*/)> for (i32) {
  fn setMaxThreadCount_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QThreadPool17setMaxThreadCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:79
// index:0
// Public Visibility=Default Availability=Available
// [4] int activeThreadCount() const

/*

*/
impl /*struct*/ QThreadPool {
  pub fn activeThreadCount_0<RetType, T: QThreadPool_activeThreadCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activeThreadCount_0(self);
    // return 1;
  }
}
pub trait QThreadPool_activeThreadCount_0<RetType> {
  fn activeThreadCount_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_activeThreadCount_0<i32> for () {
  fn activeThreadCount_0(self , rsthis: & QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QThreadPool17activeThreadCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStackSize(uint)

/*

*/
impl /*struct*/ QThreadPool {
  pub fn setStackSize_0<RetType, T: QThreadPool_setStackSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStackSize_0(self);
    // return 1;
  }
}
pub trait QThreadPool_setStackSize_0<RetType> {
  fn setStackSize_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_setStackSize_0<(/*void*/)> for (u32) {
  fn setStackSize_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN11QThreadPool12setStackSizeEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] uint stackSize() const

/*

*/
impl /*struct*/ QThreadPool {
  pub fn stackSize_0<RetType, T: QThreadPool_stackSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stackSize_0(self);
    // return 1;
  }
}
pub trait QThreadPool_stackSize_0<RetType> {
  fn stackSize_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_stackSize_0<u32> for () {
  fn stackSize_0(self , rsthis: & QThreadPool) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QThreadPool9stackSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reserveThread()

/*
Reserves one thread, disregarding activeThreadCount() and maxThreadCount().

Once you are done with the thread, call releaseThread() to allow it to be reused.

Note: This function will always increase the number of active threads. This means that by using this function, it is possible for activeThreadCount() to return a value greater than maxThreadCount() .

See also releaseThread().
*/
impl /*struct*/ QThreadPool {
  pub fn reserveThread_0<RetType, T: QThreadPool_reserveThread_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reserveThread_0(self);
    // return 1;
  }
}
pub trait QThreadPool_reserveThread_0<RetType> {
  fn reserveThread_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_reserveThread_0<(/*void*/)> for () {
  fn reserveThread_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QThreadPool13reserveThreadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void releaseThread()

/*
Releases a thread previously reserved by a call to reserveThread().

Note: Calling this function without previously reserving a thread temporarily increases maxThreadCount(). This is useful when a thread goes to sleep waiting for more work, allowing other threads to continue. Be sure to call reserveThread() when done waiting, so that the thread pool can correctly maintain the activeThreadCount().

See also reserveThread().
*/
impl /*struct*/ QThreadPool {
  pub fn releaseThread_0<RetType, T: QThreadPool_releaseThread_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.releaseThread_0(self);
    // return 1;
  }
}
pub trait QThreadPool_releaseThread_0<RetType> {
  fn releaseThread_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_releaseThread_0<(/*void*/)> for () {
  fn releaseThread_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QThreadPool13releaseThreadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:87
// index:0
// Public Visibility=Default Availability=Available
// [1] bool waitForDone(int)

/*
Waits up to msecs milliseconds for all threads to exit and removes all threads from the thread pool. Returns true if all threads were removed; otherwise it returns false. If msecs is -1 (the default), the timeout is ignored (waits for the last thread to exit).
*/
impl /*struct*/ QThreadPool {
  pub fn waitForDone_0<RetType, T: QThreadPool_waitForDone_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForDone_0(self);
    // return 1;
  }
}
pub trait QThreadPool_waitForDone_0<RetType> {
  fn waitForDone_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_waitForDone_0<bool> for (i32) {
  fn waitForDone_0(self , rsthis: & QThreadPool) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QThreadPool11waitForDoneEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes the runnables that are not yet started from the queue. The runnables for which runnable->autoDelete() returns true are deleted.

This function was introduced in  Qt 5.2.

See also start().
*/
impl /*struct*/ QThreadPool {
  pub fn clear_0<RetType, T: QThreadPool_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QThreadPool_clear_0<RetType> {
  fn clear_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QThreadPool5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cancel(QRunnable *)

/*

*/
impl /*struct*/ QThreadPool {
  pub fn cancel_0<RetType, T: QThreadPool_cancel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancel_0(self);
    // return 1;
  }
}
pub trait QThreadPool_cancel_0<RetType> {
  fn cancel_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_cancel_0<(/*void*/)> for (usize) {
  fn cancel_0(self , rsthis: & QThreadPool) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN11QThreadPool6cancelEP9QRunnable", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthreadpool.h:95
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryTake(QRunnable *)

/*
Attempts to remove the specified runnable from the queue if it is not yet started. If the runnable had not been started, returns true, and ownership of runnable is transferred to the caller (even when runnable->autoDelete() == true). Otherwise returns false.

Note: If runnable->autoDelete() == true, this function may remove the wrong runnable. This is known as the ABA problem: the original runnable may already have executed and has since been deleted. The memory is re-used for another runnable, which then gets removed instead of the intended one. For this reason, we recommend calling this function only for runnables that are not auto-deleting.

This function was introduced in  Qt 5.9.

See also start() and QRunnable::autoDelete().
*/
impl /*struct*/ QThreadPool {
  pub fn tryTake_0<RetType, T: QThreadPool_tryTake_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryTake_0(self);
    // return 1;
  }
}
pub trait QThreadPool_tryTake_0<RetType> {
  fn tryTake_0(self , rsthis: & QThreadPool) -> RetType;
}
impl<'a> /*trait*/ QThreadPool_tryTake_0<bool> for (usize) {
  fn tryTake_0(self , rsthis: & QThreadPool) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QThreadPool7tryTakeEP9QRunnable", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
