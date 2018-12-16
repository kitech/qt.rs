

// mod ::core::QThread
// package qtcore
// /usr/include/qt/QtCore/qthread.h
// #include <qthread.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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

// void run()
// func (this *QThread) InheritRun(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "run", f)
// }

// int exec()
// func (this *QThread) InheritExec(f func() int) {
//  qtrt.SetAllInheritCallback(this, "exec", f)
// }

// void setTerminationEnabled(bool)
// func (this *QThread) InheritSetTerminationEnabled(f func(enabled bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "setTerminationEnabled", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QThread)=16
pub struct QThread {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QThread_ITF interface {
//    QObject_ITF
//    QThread_PTR() *QThread
//}
//func (ptr *QThread) QThread_PTR() *QThread { return ptr }

impl /*struct*/ QThread {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QThread {
    return QThread{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QThread {
//  type Target = QThreadBASE;
//
//  fn deref(&self) -> &QThreadBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QThreadBASE> for QThread {
//  fn as_ref(& self) -> & QThreadBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qthread.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QThread {
  pub fn metaObject_0<RetType, T: QThread_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QThread_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QThread) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:74
// index:0
// Public static Visibility=Default Availability=Available
// [8] Qt::HANDLE currentThreadId()

/*
Returns the thread handle of the currently executing thread.

Warning: The handle returned by this function is used for internal purposes and should not be used in any application code.

Warning: On Windows, the returned value is a pseudo-handle for the current thread. It can't be used for numerical comparison. i.e., this function returns the DWORD (Windows-Thread ID) returned by the Win32 function getCurrentThreadId(), not the HANDLE (Windows-Thread HANDLE) returned by the Win32 function getCurrentThread().
*/
impl /*struct*/ QThread {
  pub fn currentThreadId_0<RetType, T: QThread_currentThreadId_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentThreadId_0();
    // return 1;
  }
}
pub trait QThread_currentThreadId_0<RetType> {
  fn currentThreadId_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_currentThreadId_0<i32> for () {
  fn currentThreadId_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QThread15currentThreadIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:75
// index:0
// Public static Visibility=Default Availability=Available
// [8] QThread * currentThread()

/*
Returns a pointer to a QThread which manages the currently executing thread.
*/
impl /*struct*/ QThread {
  pub fn currentThread_0<RetType, T: QThread_currentThread_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentThread_0();
    // return 1;
  }
}
pub trait QThread_currentThread_0<RetType> {
  fn currentThread_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_currentThread_0<usize> for () {
  fn currentThread_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QThread13currentThreadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:76
// index:0
// Public static Visibility=Default Availability=Available
// [4] int idealThreadCount()

/*
Returns the ideal number of threads that can be run on the system. This is done querying the number of processor cores, both real and logical, in the system. This function returns 1 if the number of processor cores could not be detected.
*/
impl /*struct*/ QThread {
  pub fn idealThreadCount_0<RetType, T: QThread_idealThreadCount_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.idealThreadCount_0();
    // return 1;
  }
}
pub trait QThread_idealThreadCount_0<RetType> {
  fn idealThreadCount_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_idealThreadCount_0<i32> for () {
  fn idealThreadCount_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QThread16idealThreadCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:77
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void yieldCurrentThread()

/*
Yields execution of the current thread to another runnable thread, if any. Note that the operating system decides to which thread to switch.
*/
impl /*struct*/ QThread {
  pub fn yieldCurrentThread_0<RetType, T: QThread_yieldCurrentThread_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.yieldCurrentThread_0();
    // return 1;
  }
}
pub trait QThread_yieldCurrentThread_0<RetType> {
  fn yieldCurrentThread_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_yieldCurrentThread_0<(/*void*/)> for () {
  fn yieldCurrentThread_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QThread18yieldCurrentThreadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QThread(QObject *)

/*
Constructs a new QThread to manage a new thread. The parent takes ownership of the QThread. The thread does not begin executing until start() is called.

See also start().
*/
// QThread(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QThread {
  pub fn QThread_0<T: QThread_QThread_0>(value: T) -> QThread {
    let rsthis = value.QThread_0();
    return rsthis;
    // return 1;
  }
}

pub trait QThread_QThread_0 {
  fn QThread_0(self) -> QThread;
}
// QThread(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QThread_QThread_0 for (usize) {
  fn QThread_0(self) -> QThread {
    // unsafe{_ZN7QThreadC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN7QThreadC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QThread{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:80
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QThread()

/*

*/
pub fn DeleteQThread(this :*mut QThread) {
    // let rv = qtrt::InvokeQtFunc6("_ZN7QThreadD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qthread.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPriority(QThread::Priority)

/*
This function sets the priority for a running thread. If the thread is not running, this function does nothing and returns immediately. Use start() to start a thread with a specific priority.

The priority argument can be any value in the QThread::Priority enum except for InheritPriorty.

The effect of the priority parameter is dependent on the operating system's scheduling policy. In particular, the priority will be ignored on systems that do not support thread priorities (such as on Linux, see http://linux.die.net/man/2/sched_setscheduler for more details).

This function was introduced in  Qt 4.1.

See also Priority, priority(), and start().
*/
impl /*struct*/ QThread {
  pub fn setPriority_0<RetType, T: QThread_setPriority_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPriority_0(self);
    // return 1;
  }
}
pub trait QThread_setPriority_0<RetType> {
  fn setPriority_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_setPriority_0<(/*void*/)> for (i32) {
  fn setPriority_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread11setPriorityENS_8PriorityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:97
// index:0
// Public Visibility=Default Availability=Available
// [4] QThread::Priority priority() const

/*
Returns the priority for a running thread. If the thread is not running, this function returns InheritPriority.

This function was introduced in  Qt 4.1.

See also Priority, setPriority(), and start().
*/
impl /*struct*/ QThread {
  pub fn priority_0<RetType, T: QThread_priority_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.priority_0(self);
    // return 1;
  }
}
pub trait QThread_priority_0<RetType> {
  fn priority_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_priority_0<i32> for () {
  fn priority_0(self , rsthis: & QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread8priorityEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:99
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFinished() const

/*
Returns true if the thread is finished; otherwise returns false.

See also isRunning().
*/
impl /*struct*/ QThread {
  pub fn isFinished_0<RetType, T: QThread_isFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFinished_0(self);
    // return 1;
  }
}
pub trait QThread_isFinished_0<RetType> {
  fn isFinished_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_isFinished_0<bool> for () {
  fn isFinished_0(self , rsthis: & QThread) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread10isFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:100
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRunning() const

/*
Returns true if the thread is running; otherwise returns false.

See also isFinished().
*/
impl /*struct*/ QThread {
  pub fn isRunning_0<RetType, T: QThread_isRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRunning_0(self);
    // return 1;
  }
}
pub trait QThread_isRunning_0<RetType> {
  fn isRunning_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_isRunning_0<bool> for () {
  fn isRunning_0(self , rsthis: & QThread) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread9isRunningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void requestInterruption()

/*
Request the interruption of the thread. That request is advisory and it is up to code running on the thread to decide if and how it should act upon such request. This function does not stop any event loop running on the thread and does not terminate it in any way.

This function was introduced in  Qt 5.2.

See also isInterruptionRequested().
*/
impl /*struct*/ QThread {
  pub fn requestInterruption_0<RetType, T: QThread_requestInterruption_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.requestInterruption_0(self);
    // return 1;
  }
}
pub trait QThread_requestInterruption_0<RetType> {
  fn requestInterruption_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_requestInterruption_0<(/*void*/)> for () {
  fn requestInterruption_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QThread19requestInterruptionEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:103
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isInterruptionRequested() const

/*
Return true if the task running on this thread should be stopped. An interruption can be requested by requestInterruption().

This function can be used to make long running tasks cleanly interruptible. Never checking or acting on the value returned by this function is safe, however it is advisable do so regularly in long running functions. Take care not to call it too often, to keep the overhead low.


  void long_task() {
       forever {
          if ( QThread::currentThread()->isInterruptionRequested() ) {
              return;
          }
      }
  }



This function was introduced in  Qt 5.2.

See also currentThread() and requestInterruption().
*/
impl /*struct*/ QThread {
  pub fn isInterruptionRequested_0<RetType, T: QThread_isInterruptionRequested_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isInterruptionRequested_0(self);
    // return 1;
  }
}
pub trait QThread_isInterruptionRequested_0<RetType> {
  fn isInterruptionRequested_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_isInterruptionRequested_0<bool> for () {
  fn isInterruptionRequested_0(self , rsthis: & QThread) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread23isInterruptionRequestedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStackSize(uint)

/*
Sets the maximum stack size for the thread to stackSize. If stackSize is greater than zero, the maximum stack size is set to stackSize bytes, otherwise the maximum stack size is automatically determined by the operating system.

Warning: Most operating systems place minimum and maximum limits on thread stack sizes. The thread will fail to start if the stack size is outside these limits.

See also stackSize().
*/
impl /*struct*/ QThread {
  pub fn setStackSize_0<RetType, T: QThread_setStackSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStackSize_0(self);
    // return 1;
  }
}
pub trait QThread_setStackSize_0<RetType> {
  fn setStackSize_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_setStackSize_0<(/*void*/)> for (u32) {
  fn setStackSize_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread12setStackSizeEj", 1,qtrt::FFITY_UINT32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:106
// index:0
// Public Visibility=Default Availability=Available
// [4] uint stackSize() const

/*
Returns the maximum stack size for the thread (if set with setStackSize()); otherwise returns zero.

See also setStackSize().
*/
impl /*struct*/ QThread {
  pub fn stackSize_0<RetType, T: QThread_stackSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.stackSize_0(self);
    // return 1;
  }
}
pub trait QThread_stackSize_0<RetType> {
  fn stackSize_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_stackSize_0<u32> for () {
  fn stackSize_0(self , rsthis: & QThread) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread9stackSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:108
// index:0
// Public Visibility=Default Availability=Available
// [-2] void exit(int)

/*
Tells the thread's event loop to exit with a return code.

After calling this function, the thread leaves the event loop and returns from the call to QEventLoop::exec(). The QEventLoop::exec() function returns returnCode.

By convention, a returnCode of 0 means success, any non-zero value indicates an error.

Note that unlike the C library function of the same name, this function does return to the caller -- it is event processing that stops.

No QEventLoops will be started anymore in this thread until QThread::exec() has been called again. If the eventloop in QThread::exec() is not running then the next call to QThread::exec() will also return immediately.

See also quit() and QEventLoop.
*/
impl /*struct*/ QThread {
  pub fn exit_0<RetType, T: QThread_exit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exit_0(self);
    // return 1;
  }
}
pub trait QThread_exit_0<RetType> {
  fn exit_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_exit_0<(/*void*/)> for (i32) {
  fn exit_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread4exitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:110
// index:0
// Public Visibility=Default Availability=Available
// [8] QAbstractEventDispatcher * eventDispatcher() const

/*
Returns a pointer to the event dispatcher object for the thread. If no event dispatcher exists for the thread, this function returns 0.

This function was introduced in  Qt 5.0.

See also setEventDispatcher().
*/
impl /*struct*/ QThread {
  pub fn eventDispatcher_0<RetType, T: QThread_eventDispatcher_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.eventDispatcher_0(self);
    // return 1;
  }
}
pub trait QThread_eventDispatcher_0<RetType> {
  fn eventDispatcher_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_eventDispatcher_0<usize> for () {
  fn eventDispatcher_0(self , rsthis: & QThread) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread15eventDispatcherEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setEventDispatcher(QAbstractEventDispatcher *)

/*
Sets the event dispatcher for the thread to eventDispatcher. This is only possible as long as there is no event dispatcher installed for the thread yet. That is, before the thread has been started with start() or, in case of the main thread, before QCoreApplication has been instantiated. This method takes ownership of the object.

This function was introduced in  Qt 5.0.

See also eventDispatcher().
*/
impl /*struct*/ QThread {
  pub fn setEventDispatcher_0<RetType, T: QThread_setEventDispatcher_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setEventDispatcher_0(self);
    // return 1;
  }
}
pub trait QThread_setEventDispatcher_0<RetType> {
  fn setEventDispatcher_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_setEventDispatcher_0<(/*void*/)> for (usize) {
  fn setEventDispatcher_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread18setEventDispatcherEP24QAbstractEventDispatcher", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:113
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QThread {
  pub fn event_0<RetType, T: QThread_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QThread_event_0<RetType> {
  fn event_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QThread) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QThread5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:114
// index:0
// Public Visibility=Default Availability=Available
// [4] int loopLevel() const

/*
Returns the current event loop level for the thread.

Note: This can only be called within the thread itself, i.e. when it is the current thread.

This function was introduced in  Qt 5.5.
*/
impl /*struct*/ QThread {
  pub fn loopLevel_0<RetType, T: QThread_loopLevel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loopLevel_0(self);
    // return 1;
  }
}
pub trait QThread_loopLevel_0<RetType> {
  fn loopLevel_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_loopLevel_0<i32> for () {
  fn loopLevel_0(self , rsthis: & QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK7QThread9loopLevelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void start(QThread::Priority)

/*
Begins execution of the thread by calling run(). The operating system will schedule the thread according to the priority parameter. If the thread is already running, this function does nothing.

The effect of the priority parameter is dependent on the operating system's scheduling policy. In particular, the priority will be ignored on systems that do not support thread priorities (such as on Linux, see the sched_setscheduler documentation for more details).

See also run() and terminate().
*/
impl /*struct*/ QThread {
  pub fn start_0<RetType, T: QThread_start_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.start_0(self);
    // return 1;
  }
}
pub trait QThread_start_0<RetType> {
  fn start_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_start_0<(/*void*/)> for (i32) {
  fn start_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread5startENS_8PriorityE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:135
// index:0
// Public Visibility=Default Availability=Available
// [-2] void terminate()

/*
Terminates the execution of the thread. The thread may or may not be terminated immediately, depending on the operating system's scheduling policies. Use QThread::wait() after terminate(), to be sure.

When the thread is terminated, all threads waiting for the thread to finish will be woken up.

Warning: This function is dangerous and its use is discouraged. The thread can be terminated at any point in its code path. Threads can be terminated while modifying data. There is no chance for the thread to clean up after itself, unlock any held mutexes, etc. In short, use this function only if absolutely necessary.

Termination can be explicitly enabled or disabled by calling QThread::setTerminationEnabled(). Calling this function while termination is disabled results in the termination being deferred, until termination is re-enabled. See the documentation of QThread::setTerminationEnabled() for more information.

See also setTerminationEnabled().
*/
impl /*struct*/ QThread {
  pub fn terminate_0<RetType, T: QThread_terminate_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.terminate_0(self);
    // return 1;
  }
}
pub trait QThread_terminate_0<RetType> {
  fn terminate_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_terminate_0<(/*void*/)> for () {
  fn terminate_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QThread9terminateEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:136
// index:0
// Public Visibility=Default Availability=Available
// [-2] void quit()

/*
Tells the thread's event loop to exit with return code 0 (success). Equivalent to calling QThread::exit(0).

This function does nothing if the thread does not have an event loop.

See also exit() and QEventLoop.
*/
impl /*struct*/ QThread {
  pub fn quit_0<RetType, T: QThread_quit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quit_0(self);
    // return 1;
  }
}
pub trait QThread_quit_0<RetType> {
  fn quit_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_quit_0<(/*void*/)> for () {
  fn quit_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QThread4quitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:140
// index:0
// Public Visibility=Default Availability=Available
// [1] bool wait(unsigned long)

/*
Blocks the thread until either of these conditions is met:


The thread associated with this QThread object has finished execution (i.e. when it returns from run()). This function will return true if the thread has finished. It also returns true if the thread has not been started yet.
time milliseconds has elapsed. If time is ULONG_MAX (the default), then the wait will never timeout (the thread must return from run()). This function will return false if the wait timed out.


This provides similar functionality to the POSIX pthread_join() function.

See also sleep() and terminate().
*/
impl /*struct*/ QThread {
  pub fn wait_0<RetType, T: QThread_wait_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wait_0(self);
    // return 1;
  }
}
pub trait QThread_wait_0<RetType> {
  fn wait_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_wait_0<bool> for (u64) {
  fn wait_0(self , rsthis: & QThread) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QThread4waitEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:142
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void sleep(unsigned long)

/*
Forces the current thread to sleep for secs seconds.

See also msleep() and usleep().
*/
impl /*struct*/ QThread {
  pub fn sleep_0<RetType, T: QThread_sleep_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.sleep_0();
    // return 1;
  }
}
pub trait QThread_sleep_0<RetType> {
  fn sleep_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_sleep_0<(/*void*/)> for (u64) {
  fn sleep_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread5sleepEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:143
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void msleep(unsigned long)

/*
Forces the current thread to sleep for msecs milliseconds.

See also sleep() and usleep().
*/
impl /*struct*/ QThread {
  pub fn msleep_0<RetType, T: QThread_msleep_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.msleep_0();
    // return 1;
  }
}
pub trait QThread_msleep_0<RetType> {
  fn msleep_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_msleep_0<(/*void*/)> for (u64) {
  fn msleep_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread6msleepEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:144
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void usleep(unsigned long)

/*
Forces the current thread to sleep for usecs microseconds.

See also sleep() and msleep().
*/
impl /*struct*/ QThread {
  pub fn usleep_0<RetType, T: QThread_usleep_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.usleep_0();
    // return 1;
  }
}
pub trait QThread_usleep_0<RetType> {
  fn usleep_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_usleep_0<(/*void*/)> for (u64) {
  fn usleep_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const u64 as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread6usleepEm", 1,qtrt::FFITY_UINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:151
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void run()

/*
The starting point for the thread. After calling start(), the newly created thread calls this function. The default implementation simply calls exec().

You can reimplement this function to facilitate advanced thread management. Returning from this method will end the execution of the thread.

See also start() and wait().
*/
impl /*struct*/ QThread {
  pub fn run_0<RetType, T: QThread_run_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.run_0(self);
    // return 1;
  }
}
pub trait QThread_run_0<RetType> {
  fn run_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_run_0<(/*void*/)> for () {
  fn run_0(self , rsthis: & QThread) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN7QThread3runEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qthread.h:152
// index:0
// Protected Visibility=Default Availability=Available
// [4] int exec()

/*
Enters the event loop and waits until exit() is called, returning the value that was passed to exit(). The value returned is 0 if exit() is called via quit().

This function is meant to be called from within run(). It is necessary to call this function to start event handling.

See also quit() and exit().
*/
impl /*struct*/ QThread {
  pub fn exec_0<RetType, T: QThread_exec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_0(self);
    // return 1;
  }
}
pub trait QThread_exec_0<RetType> {
  fn exec_0(self , rsthis: & QThread) -> RetType;
}
impl<'a> /*trait*/ QThread_exec_0<i32> for () {
  fn exec_0(self , rsthis: & QThread) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN7QThread4execEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qthread.h:154
// index:0
// Protected static Visibility=Default Availability=Available
// [-2] void setTerminationEnabled(bool)

/*
Enables or disables termination of the current thread based on the enabled parameter. The thread must have been started by QThread.

When enabled is false, termination is disabled. Future calls to QThread::terminate() will return immediately without effect. Instead, the termination is deferred until termination is enabled.

When enabled is true, termination is enabled. Future calls to QThread::terminate() will terminate the thread normally. If termination has been deferred (i.e. QThread::terminate() was called with termination disabled), this function will terminate the calling thread immediately. Note that this function will not return in this case.

See also terminate().
*/
impl /*struct*/ QThread {
  pub fn setTerminationEnabled_0<RetType, T: QThread_setTerminationEnabled_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTerminationEnabled_0();
    // return 1;
  }
}
pub trait QThread_setTerminationEnabled_0<RetType> {
  fn setTerminationEnabled_0(self ) -> RetType;
}
impl<'a> /*trait*/ QThread_setTerminationEnabled_0<(/*void*/)> for (bool) {
  fn setTerminationEnabled_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN7QThread21setTerminationEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*
This enum type indicates how the operating system should schedule newly created threads.


*/
pub type QThread__Priority = i32;
// scheduled only when no other threads are running.
pub const QThread__IdlePriority :QThread__Priority = 0;
// scheduled less often than LowPriority.
pub const QThread__LowestPriority :QThread__Priority = 1;
// scheduled less often than NormalPriority.
pub const QThread__LowPriority :QThread__Priority = 2;
// the default priority of the operating system.
pub const QThread__NormalPriority :QThread__Priority = 3;
// scheduled more often than NormalPriority.
pub const QThread__HighPriority :QThread__Priority = 4;
// scheduled more often than HighPriority.
pub const QThread__HighestPriority :QThread__Priority = 5;
// scheduled as often as possible.
pub const QThread__TimeCriticalPriority :QThread__Priority = 6;
// use the same priority as the creating thread. This is the default.
pub const QThread__InheritPriority :QThread__Priority = 7;
pub fn QThread_PriorityItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QThread", val);
}
pub fn QThread_PriorityItemName_s(val: i32) ->String {
  //var nilthis *QThread
  //return nilthis.PriorityItemName(val);
  return QThread_PriorityItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
