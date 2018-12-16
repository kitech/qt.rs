

// mod ::core::QEventLoop
// package qtcore
// /usr/include/qt/QtCore/qeventloop.h
// #include <qeventloop.h>
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
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QEventLoop)=16
pub struct QEventLoop {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QEventLoop_ITF interface {
//    QObject_ITF
//    QEventLoop_PTR() *QEventLoop
//}
//func (ptr *QEventLoop) QEventLoop_PTR() *QEventLoop { return ptr }

impl /*struct*/ QEventLoop {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QEventLoop {
    return QEventLoop{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QEventLoop {
//  type Target = QEventLoopBASE;
//
//  fn deref(&self) -> &QEventLoopBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QEventLoopBASE> for QEventLoop {
//  fn as_ref(& self) -> & QEventLoopBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qeventloop.h:52
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QEventLoop {
  pub fn metaObject_0<RetType, T: QEventLoop_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QEventLoop_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QEventLoop) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QEventLoop10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QEventLoop(QObject *)

/*
Constructs an event loop object with the given parent.
*/
// QEventLoop(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QEventLoop {
  pub fn QEventLoop_0<T: QEventLoop_QEventLoop_0>(value: T) -> QEventLoop {
    let rsthis = value.QEventLoop_0();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoop_QEventLoop_0 {
  fn QEventLoop_0(self) -> QEventLoop;
}
// QEventLoop(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEventLoop_QEventLoop_0 for (usize) {
  fn QEventLoop_0(self) -> QEventLoop {
    // unsafe{_ZN10QEventLoopC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN10QEventLoopC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEventLoop{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QEventLoop()

/*

*/
pub fn DeleteQEventLoop(this :*mut QEventLoop) {
    // let rv = qtrt::InvokeQtFunc6("_ZN10QEventLoopD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qeventloop.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool processEvents(QEventLoop::ProcessEventsFlags)

/*
Processes pending events that match flags until there are no more events to process. Returns true if pending events were handled; otherwise returns false.

This function is especially useful if you have a long running operation and want to show its progress without allowing user input; i.e. by using the ExcludeUserInputEvents flag.

This function is simply a wrapper for QAbstractEventDispatcher::processEvents(). See the documentation for that function for details.
*/
impl /*struct*/ QEventLoop {
  pub fn processEvents_0<RetType, T: QEventLoop_processEvents_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processEvents_0(self);
    // return 1;
  }
}
pub trait QEventLoop_processEvents_0<RetType> {
  fn processEvents_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_processEvents_0<bool> for (i32) {
  fn processEvents_0(self , rsthis: & QEventLoop) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:71
// index:1
// Public Visibility=Default Availability=Available
// [-2] void processEvents(QEventLoop::ProcessEventsFlags, int)

/*
Processes pending events that match flags until there are no more events to process. Returns true if pending events were handled; otherwise returns false.

This function is especially useful if you have a long running operation and want to show its progress without allowing user input; i.e. by using the ExcludeUserInputEvents flag.

This function is simply a wrapper for QAbstractEventDispatcher::processEvents(). See the documentation for that function for details.
*/
impl /*struct*/ QEventLoop {
  pub fn processEvents_1<RetType, T: QEventLoop_processEvents_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.processEvents_1(self);
    // return 1;
  }
}
pub trait QEventLoop_processEvents_1<RetType> {
  fn processEvents_1(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_processEvents_1<(/*void*/)> for (i32,i32) {
  fn processEvents_1(self , rsthis: & QEventLoop) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QEventLoop13processEventsE6QFlagsINS_17ProcessEventsFlagEEi", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] int exec(QEventLoop::ProcessEventsFlags)

/*
Enters the main event loop and waits until exit() is called. Returns the value that was passed to exit().

If flags are specified, only events of the types allowed by the flags will be processed.

It is necessary to call this function to start event handling. The main event loop receives events from the window system and dispatches these to the application widgets.

Generally speaking, no user interaction can take place before calling exec(). As a special case, modal widgets like QMessageBox can be used before calling exec(), because modal widgets use their own local event loop.

To make your application perform idle processing (i.e. executing a special function whenever there are no pending events), use a QTimer with 0 timeout. More sophisticated idle processing schemes can be achieved using processEvents().

See also QCoreApplication::quit(), exit(), and processEvents().
*/
impl /*struct*/ QEventLoop {
  pub fn exec_0<RetType, T: QEventLoop_exec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exec_0(self);
    // return 1;
  }
}
pub trait QEventLoop_exec_0<RetType> {
  fn exec_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_exec_0<i32> for (i32) {
  fn exec_0(self , rsthis: & QEventLoop) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QEventLoop4execE6QFlagsINS_17ProcessEventsFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void exit(int)

/*
Tells the event loop to exit with a return code.

After this function has been called, the event loop returns from the call to exec(). The exec() function returns returnCode.

By convention, a returnCode of 0 means success, and any non-zero value indicates an error.

Note that unlike the C library function of the same name, this function does return to the caller -- it is event processing that stops.

See also QCoreApplication::quit(), quit(), and exec().
*/
impl /*struct*/ QEventLoop {
  pub fn exit_0<RetType, T: QEventLoop_exit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exit_0(self);
    // return 1;
  }
}
pub trait QEventLoop_exit_0<RetType> {
  fn exit_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_exit_0<(/*void*/)> for (i32) {
  fn exit_0(self , rsthis: & QEventLoop) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN10QEventLoop4exitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:75
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRunning() const

/*
Returns true if the event loop is running; otherwise returns false. The event loop is considered running from the time when exec() is called until exit() is called.

See also exec() and exit().
*/
impl /*struct*/ QEventLoop {
  pub fn isRunning_0<RetType, T: QEventLoop_isRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRunning_0(self);
    // return 1;
  }
}
pub trait QEventLoop_isRunning_0<RetType> {
  fn isRunning_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_isRunning_0<bool> for () {
  fn isRunning_0(self , rsthis: & QEventLoop) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK10QEventLoop9isRunningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void wakeUp()

/*
Wakes up the event loop.

See also QAbstractEventDispatcher::wakeUp().
*/
impl /*struct*/ QEventLoop {
  pub fn wakeUp_0<RetType, T: QEventLoop_wakeUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.wakeUp_0(self);
    // return 1;
  }
}
pub trait QEventLoop_wakeUp_0<RetType> {
  fn wakeUp_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_wakeUp_0<(/*void*/)> for () {
  fn wakeUp_0(self , rsthis: & QEventLoop) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QEventLoop6wakeUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QEventLoop {
  pub fn event_0<RetType, T: QEventLoop_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QEventLoop_event_0<RetType> {
  fn event_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QEventLoop) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN10QEventLoop5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qeventloop.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void quit()

/*
Tells the event loop to exit normally.

Same as exit(0).

See also QCoreApplication::quit() and exit().
*/
impl /*struct*/ QEventLoop {
  pub fn quit_0<RetType, T: QEventLoop_quit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.quit_0(self);
    // return 1;
  }
}
pub trait QEventLoop_quit_0<RetType> {
  fn quit_0(self , rsthis: & QEventLoop) -> RetType;
}
impl<'a> /*trait*/ QEventLoop_quit_0<(/*void*/)> for () {
  fn quit_0(self , rsthis: & QEventLoop) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN10QEventLoop4quitEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QEventLoop__ProcessEventsFlag = i32;
// 
pub const QEventLoop__AllEvents :QEventLoop__ProcessEventsFlag = 0;
// 
pub const QEventLoop__ExcludeUserInputEvents :QEventLoop__ProcessEventsFlag = 1;
// 
pub const QEventLoop__ExcludeSocketNotifiers :QEventLoop__ProcessEventsFlag = 2;
// 
pub const QEventLoop__WaitForMoreEvents :QEventLoop__ProcessEventsFlag = 4;
// 
pub const QEventLoop__X11ExcludeTimers :QEventLoop__ProcessEventsFlag = 8;
// 
pub const QEventLoop__EventLoopExec :QEventLoop__ProcessEventsFlag = 32;
// 
pub const QEventLoop__DialogExec :QEventLoop__ProcessEventsFlag = 64;
pub fn QEventLoop_ProcessEventsFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QEventLoop", val);
}
pub fn QEventLoop_ProcessEventsFlagItemName_s(val: i32) ->String {
  //var nilthis *QEventLoop
  //return nilthis.ProcessEventsFlagItemName(val);
  return QEventLoop_ProcessEventsFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
