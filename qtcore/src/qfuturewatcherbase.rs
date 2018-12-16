

// mod ::core::QFutureWatcherBase
// package qtcore
// /usr/include/qt/QtCore/qfuturewatcher.h
// #include <qfuturewatcher.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 45
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

// void connectNotify(const QMetaMethod &)
// func (this *QFutureWatcherBase) InheritConnectNotify(f func(signal *QMetaMethod) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "connectNotify", f)
// }

// void disconnectNotify(const QMetaMethod &)
// func (this *QFutureWatcherBase) InheritDisconnectNotify(f func(signal *QMetaMethod) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "disconnectNotify", f)
// }

// void connectOutputInterface()
// func (this *QFutureWatcherBase) InheritConnectOutputInterface(f func() /*void*/) {
//  qtrt.SetAllInheritCallback(this, "connectOutputInterface", f)
// }

// void disconnectOutputInterface(bool)
// func (this *QFutureWatcherBase) InheritDisconnectOutputInterface(f func(pendingAssignment bool) /*void*/) {
//  qtrt.SetAllInheritCallback(this, "disconnectOutputInterface", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFutureWatcherBase)=16
pub struct QFutureWatcherBase {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFutureWatcherBase_ITF interface {
//    QObject_ITF
//    QFutureWatcherBase_PTR() *QFutureWatcherBase
//}
//func (ptr *QFutureWatcherBase) QFutureWatcherBase_PTR() *QFutureWatcherBase { return ptr }

impl /*struct*/ QFutureWatcherBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFutureWatcherBase {
    return QFutureWatcherBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFutureWatcherBase {
//  type Target = QFutureWatcherBaseBASE;
//
//  fn deref(&self) -> &QFutureWatcherBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFutureWatcherBaseBASE> for QFutureWatcherBase {
//  fn as_ref(& self) -> & QFutureWatcherBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfuturewatcher.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFutureWatcherBase {
  pub fn metaObject_0<RetType, T: QFutureWatcherBase_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFutureWatcherBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:61
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFutureWatcherBase(QObject *)

/*

*/
// QFutureWatcherBase(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFutureWatcherBase {
  pub fn QFutureWatcherBase_0<T: QFutureWatcherBase_QFutureWatcherBase_0>(value: T) -> QFutureWatcherBase {
    let rsthis = value.QFutureWatcherBase_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureWatcherBase_QFutureWatcherBase_0 {
  fn QFutureWatcherBase_0(self) -> QFutureWatcherBase;
}
// QFutureWatcherBase(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFutureWatcherBase_QFutureWatcherBase_0 for (usize) {
  fn QFutureWatcherBase_0(self) -> QFutureWatcherBase {
    // unsafe{_ZN18QFutureWatcherBaseC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBaseC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFutureWatcherBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:64
// index:0
// Public Visibility=Default Availability=Available
// [4] int progressValue() const

/*
Returns the current progress value, which is between the progressMinimum() and progressMaximum().

See also progressMinimum() and progressMaximum().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressValue_0<RetType, T: QFutureWatcherBase_progressValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressValue_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressValue_0<RetType> {
  fn progressValue_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressValue_0<i32> for () {
  fn progressValue_0(self , rsthis: & QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase13progressValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:65
// index:0
// Public Visibility=Default Availability=Available
// [4] int progressMinimum() const

/*
Returns the minimum progressValue().

See also progressValue() and progressMaximum().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressMinimum_0<RetType, T: QFutureWatcherBase_progressMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressMinimum_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressMinimum_0<RetType> {
  fn progressMinimum_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressMinimum_0<i32> for () {
  fn progressMinimum_0(self , rsthis: & QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase15progressMinimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:66
// index:0
// Public Visibility=Default Availability=Available
// [4] int progressMaximum() const

/*
Returns the maximum progressValue().

See also progressValue() and progressMinimum().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressMaximum_0<RetType, T: QFutureWatcherBase_progressMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressMaximum_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressMaximum_0<RetType> {
  fn progressMaximum_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressMaximum_0<i32> for () {
  fn progressMaximum_0(self , rsthis: & QFutureWatcherBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase15progressMaximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QString progressText() const

/*
Returns the (optional) textual representation of the progress as reported by the asynchronous computation.

Be aware that not all computations provide a textual representation of the progress, and as such, this function may return an empty string.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressText_0<RetType, T: QFutureWatcherBase_progressText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressText_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressText_0<RetType> {
  fn progressText_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressText_0<usize> for () {
  fn progressText_0(self , rsthis: & QFutureWatcherBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase12progressTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:69
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isStarted() const

/*
Returns true if the asynchronous computation represented by the future() has been started; otherwise returns false.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn isStarted_0<RetType, T: QFutureWatcherBase_isStarted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isStarted_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_isStarted_0<RetType> {
  fn isStarted_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_isStarted_0<bool> for () {
  fn isStarted_0(self , rsthis: & QFutureWatcherBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase9isStartedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:70
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFinished() const

/*
Returns true if the asynchronous computation represented by the future() has finished, or if no future has been set; otherwise returns false.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn isFinished_0<RetType, T: QFutureWatcherBase_isFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFinished_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_isFinished_0<RetType> {
  fn isFinished_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_isFinished_0<bool> for () {
  fn isFinished_0(self , rsthis: & QFutureWatcherBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase10isFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:71
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRunning() const

/*
Returns true if the asynchronous computation represented by the future() is currently running; otherwise returns false.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn isRunning_0<RetType, T: QFutureWatcherBase_isRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRunning_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_isRunning_0<RetType> {
  fn isRunning_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_isRunning_0<bool> for () {
  fn isRunning_0(self , rsthis: & QFutureWatcherBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase9isRunningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:72
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCanceled() const

/*
Returns true if the asynchronous computation has been canceled with the cancel() function; otherwise returns false.

Be aware that the computation may still be running even though this function returns true. See cancel() for more details.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn isCanceled_0<RetType, T: QFutureWatcherBase_isCanceled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCanceled_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_isCanceled_0<RetType> {
  fn isCanceled_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_isCanceled_0<bool> for () {
  fn isCanceled_0(self , rsthis: & QFutureWatcherBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase10isCanceledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPaused() const

/*
Returns true if the asynchronous computation has been paused with the pause() function; otherwise returns false.

Be aware that the computation may still be running even though this function returns true. See setPaused() for more details.

See also setPaused() and togglePaused().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn isPaused_0<RetType, T: QFutureWatcherBase_isPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPaused_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_isPaused_0<RetType> {
  fn isPaused_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_isPaused_0<bool> for () {
  fn isPaused_0(self , rsthis: & QFutureWatcherBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFutureWatcherBase8isPausedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void waitForFinished()

/*
Waits for the asynchronous computation to finish (including cancel()ed computations).
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn waitForFinished_0<RetType, T: QFutureWatcherBase_waitForFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_waitForFinished_0<RetType> {
  fn waitForFinished_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_waitForFinished_0<(/*void*/)> for () {
  fn waitForFinished_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase15waitForFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPendingResultsLimit(int)

/*
The setPendingResultsLimit() provides throttling control. When the number of pending resultReadyAt() or resultsReadyAt() signals exceeds the limit, the computation represented by the future will be throttled automatically. The computation will resume once the number of pending signals drops below the limit.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn setPendingResultsLimit_0<RetType, T: QFutureWatcherBase_setPendingResultsLimit_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPendingResultsLimit_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_setPendingResultsLimit_0<RetType> {
  fn setPendingResultsLimit_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_setPendingResultsLimit_0<(/*void*/)> for (i32) {
  fn setPendingResultsLimit_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase22setPendingResultsLimitEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*

*/
impl /*struct*/ QFutureWatcherBase {
  pub fn event_0<RetType, T: QFutureWatcherBase_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_event_0<RetType> {
  fn event_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QFutureWatcherBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void started()

/*
This signal is emitted when this QFutureWatcher starts watching the future set with setFuture().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn started_0<RetType, T: QFutureWatcherBase_started_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.started_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_started_0<RetType> {
  fn started_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_started_0<(/*void*/)> for () {
  fn started_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase7startedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void finished()

/*
This signal is emitted when the watched future finishes.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn finished_0<RetType, T: QFutureWatcherBase_finished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.finished_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_finished_0<RetType> {
  fn finished_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_finished_0<(/*void*/)> for () {
  fn finished_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase8finishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void canceled()

/*
This signal is emitted if the watched future is canceled.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn canceled_0<RetType, T: QFutureWatcherBase_canceled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canceled_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_canceled_0<RetType> {
  fn canceled_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_canceled_0<(/*void*/)> for () {
  fn canceled_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase8canceledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void paused()

/*
This signal is emitted when the watched future is paused.

See also setPaused().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn paused_0<RetType, T: QFutureWatcherBase_paused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.paused_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_paused_0<RetType> {
  fn paused_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_paused_0<(/*void*/)> for () {
  fn paused_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase6pausedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resumed()

/*
This signal is emitted when the watched future is resumed.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn resumed_0<RetType, T: QFutureWatcherBase_resumed_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resumed_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_resumed_0<RetType> {
  fn resumed_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_resumed_0<(/*void*/)> for () {
  fn resumed_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase7resumedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resultReadyAt(int)

/*
This signal is emitted when the watched future reports a ready result at index. If the future reports multiple results, the index will indicate which one it is. Results can be reported out-of-order. To get the result, call future().result(index);
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn resultReadyAt_0<RetType, T: QFutureWatcherBase_resultReadyAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resultReadyAt_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_resultReadyAt_0<RetType> {
  fn resultReadyAt_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_resultReadyAt_0<(/*void*/)> for (i32) {
  fn resultReadyAt_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase13resultReadyAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resultsReadyAt(int, int)

/*
This signal is emitted when the watched future reports ready results. The results are indexed from beginIndex to endIndex.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn resultsReadyAt_0<RetType, T: QFutureWatcherBase_resultsReadyAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resultsReadyAt_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_resultsReadyAt_0<RetType> {
  fn resultsReadyAt_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_resultsReadyAt_0<(/*void*/)> for (i32,i32) {
  fn resultsReadyAt_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase14resultsReadyAtEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void progressRangeChanged(int, int)

/*
The progress range for the watched future has changed to minimum and maximum
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressRangeChanged_0<RetType, T: QFutureWatcherBase_progressRangeChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressRangeChanged_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressRangeChanged_0<RetType> {
  fn progressRangeChanged_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressRangeChanged_0<(/*void*/)> for (i32,i32) {
  fn progressRangeChanged_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase20progressRangeChangedEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void progressValueChanged(int)

/*
This signal is emitted when the watched future reports progress, progressValue gives the current progress. In order to avoid overloading the GUI event loop, QFutureWatcher limits the progress signal emission rate. This means that listeners connected to this slot might not get all progress reports the future makes. The last progress update (where progressValue equals the maximum value) will always be delivered.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressValueChanged_0<RetType, T: QFutureWatcherBase_progressValueChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressValueChanged_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressValueChanged_0<RetType> {
  fn progressValueChanged_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressValueChanged_0<(/*void*/)> for (i32) {
  fn progressValueChanged_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase20progressValueChangedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void progressTextChanged(const QString &)

/*
This signal is emitted when the watched future reports textual progress information, progressText.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn progressTextChanged_0<RetType, T: QFutureWatcherBase_progressTextChanged_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressTextChanged_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_progressTextChanged_0<RetType> {
  fn progressTextChanged_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_progressTextChanged_0<(/*void*/)> for (usize) {
  fn progressTextChanged_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase19progressTextChangedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:94
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cancel()

/*
Cancels the asynchronous computation represented by the future(). Note that the cancelation is asynchronous. Use waitForFinished() after calling cancel() when you need synchronous cancelation.

Currently available results may still be accessed on a canceled QFuture, but new results will not become available after calling this function. Also, this QFutureWatcher will not deliver progress and result ready signals once canceled. This includes the progressValueChanged(), progressRangeChanged(), progressTextChanged(), resultReadyAt(), and resultsReadyAt() signals.

Be aware that not all asynchronous computations can be canceled. For example, the QFuture returned by QtConcurrent::run() cannot be canceled; but the QFuture returned by QtConcurrent::mappedReduced() can.
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn cancel_0<RetType, T: QFutureWatcherBase_cancel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancel_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_cancel_0<RetType> {
  fn cancel_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_cancel_0<(/*void*/)> for () {
  fn cancel_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase6cancelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaused(bool)

/*
If paused is true, this function pauses the asynchronous computation represented by the future(). If the computation is already paused, this function does nothing. This QFutureWatcher will stop delivering progress and result ready signals while the future is paused. Signal delivery will continue once the computation is resumed.

If paused is false, this function resumes the asynchronous computation. If the computation was not previously paused, this function does nothing.

Be aware that not all computations can be paused. For example, the QFuture returned by QtConcurrent::run() cannot be paused; but the QFuture returned by QtConcurrent::mappedReduced() can.

See also paused(), pause(), resume(), and togglePaused().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn setPaused_0<RetType, T: QFutureWatcherBase_setPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaused_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_setPaused_0<RetType> {
  fn setPaused_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_setPaused_0<(/*void*/)> for (bool) {
  fn setPaused_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase9setPausedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:96
// index:0
// Public Visibility=Default Availability=Available
// [-2] void pause()

/*
Pauses the asynchronous computation represented by the future(). This is a convenience method that simply calls setPaused(true).

See also resume().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn pause_0<RetType, T: QFutureWatcherBase_pause_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pause_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_pause_0<RetType> {
  fn pause_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_pause_0<(/*void*/)> for () {
  fn pause_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase5pauseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:97
// index:0
// Public Visibility=Default Availability=Available
// [-2] void resume()

/*
Resumes the asynchronous computation represented by the future(). This is a convenience method that simply calls setPaused(false).

See also pause().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn resume_0<RetType, T: QFutureWatcherBase_resume_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resume_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_resume_0<RetType> {
  fn resume_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_resume_0<(/*void*/)> for () {
  fn resume_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase6resumeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void togglePaused()

/*
Toggles the paused state of the asynchronous computation. In other words, if the computation is currently paused, calling this function resumes it; if the computation is running, it becomes paused. This is a convenience method for calling setPaused(!isPaused()).

See also setPaused(), pause(), and resume().
*/
impl /*struct*/ QFutureWatcherBase {
  pub fn togglePaused_0<RetType, T: QFutureWatcherBase_togglePaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.togglePaused_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_togglePaused_0<RetType> {
  fn togglePaused_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_togglePaused_0<(/*void*/)> for () {
  fn togglePaused_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase12togglePausedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:101
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void connectNotify(const QMetaMethod &)

/*

*/
impl /*struct*/ QFutureWatcherBase {
  pub fn connectNotify_0<RetType, T: QFutureWatcherBase_connectNotify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.connectNotify_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_connectNotify_0<RetType> {
  fn connectNotify_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_connectNotify_0<(/*void*/)> for (usize) {
  fn connectNotify_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase13connectNotifyERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:102
// index:0
// Protected virtual Visibility=Default Availability=Available
// [-2] void disconnectNotify(const QMetaMethod &)

/*

*/
impl /*struct*/ QFutureWatcherBase {
  pub fn disconnectNotify_0<RetType, T: QFutureWatcherBase_disconnectNotify_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.disconnectNotify_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_disconnectNotify_0<RetType> {
  fn disconnectNotify_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_disconnectNotify_0<(/*void*/)> for (usize) {
  fn disconnectNotify_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase16disconnectNotifyERK11QMetaMethod", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:105
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void connectOutputInterface()

/*

*/
impl /*struct*/ QFutureWatcherBase {
  pub fn connectOutputInterface_0<RetType, T: QFutureWatcherBase_connectOutputInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.connectOutputInterface_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_connectOutputInterface_0<RetType> {
  fn connectOutputInterface_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_connectOutputInterface_0<(/*void*/)> for () {
  fn connectOutputInterface_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase22connectOutputInterfaceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfuturewatcher.h:106
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void disconnectOutputInterface(bool)

/*

*/
impl /*struct*/ QFutureWatcherBase {
  pub fn disconnectOutputInterface_0<RetType, T: QFutureWatcherBase_disconnectOutputInterface_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.disconnectOutputInterface_0(self);
    // return 1;
  }
}
pub trait QFutureWatcherBase_disconnectOutputInterface_0<RetType> {
  fn disconnectOutputInterface_0(self , rsthis: & QFutureWatcherBase) -> RetType;
}
impl<'a> /*trait*/ QFutureWatcherBase_disconnectOutputInterface_0<(/*void*/)> for (bool) {
  fn disconnectOutputInterface_0(self , rsthis: & QFutureWatcherBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBase25disconnectOutputInterfaceEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


pub fn DeleteQFutureWatcherBase(this :*mut QFutureWatcherBase) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN18QFutureWatcherBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
