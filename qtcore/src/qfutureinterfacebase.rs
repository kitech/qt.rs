

// mod ::core::QFutureInterfaceBase
// package qtcore
// /usr/include/qt/QtCore/qfutureinterface.h
// #include <qfutureinterface.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
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

// bool refT()
// func (this *QFutureInterfaceBase) InheritRefT(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "refT", f)
// }

// bool derefT()
// func (this *QFutureInterfaceBase) InheritDerefT(f func() bool) {
//  qtrt.SetAllInheritCallback(this, "derefT", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFutureInterfaceBase)=16
pub struct QFutureInterfaceBase {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFutureInterfaceBase_ITF interface {
//    QFutureInterfaceBase_PTR() *QFutureInterfaceBase
//}
//func (ptr *QFutureInterfaceBase) QFutureInterfaceBase_PTR() *QFutureInterfaceBase { return ptr }

impl /*struct*/ QFutureInterfaceBase {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFutureInterfaceBase {
    return QFutureInterfaceBase{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFutureInterfaceBase {
//  type Target = QFutureInterfaceBaseBASE;
//
//  fn deref(&self) -> &QFutureInterfaceBaseBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFutureInterfaceBaseBASE> for QFutureInterfaceBase {
//  fn as_ref(& self) -> & QFutureInterfaceBaseBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfutureinterface.h:73
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFutureInterfaceBase(QFutureInterfaceBase::State)

/*

*/
// QFutureInterfaceBase(QFutureInterfaceBase::State) ctx.fn_proto_cpp
impl /*struct*/ QFutureInterfaceBase {
  pub fn QFutureInterfaceBase_0<T: QFutureInterfaceBase_QFutureInterfaceBase_0>(value: T) -> QFutureInterfaceBase {
    let rsthis = value.QFutureInterfaceBase_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFutureInterfaceBase_QFutureInterfaceBase_0 {
  fn QFutureInterfaceBase_0(self) -> QFutureInterfaceBase;
}
// QFutureInterfaceBase(QFutureInterfaceBase::State) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFutureInterfaceBase_QFutureInterfaceBase_0 for (i32) {
  fn QFutureInterfaceBase_0(self) -> QFutureInterfaceBase {
    // unsafe{_ZN20QFutureInterfaceBaseC2ENS_5StateE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBaseC2ENS_5StateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFutureInterfaceBase{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:75
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFutureInterfaceBase()

/*

*/
pub fn DeleteQFutureInterfaceBase(this :*mut QFutureInterfaceBase) {
    // let rv = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBaseD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfutureinterface.h:78
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reportStarted()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportStarted_0<RetType, T: QFutureInterfaceBase_reportStarted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reportStarted_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_reportStarted_0<RetType> {
  fn reportStarted_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_reportStarted_0<(/*void*/)> for () {
  fn reportStarted_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase13reportStartedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:79
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reportFinished()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportFinished_0<RetType, T: QFutureInterfaceBase_reportFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reportFinished_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_reportFinished_0<RetType> {
  fn reportFinished_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_reportFinished_0<(/*void*/)> for () {
  fn reportFinished_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase14reportFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reportCanceled()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportCanceled_0<RetType, T: QFutureInterfaceBase_reportCanceled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reportCanceled_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_reportCanceled_0<RetType> {
  fn reportCanceled_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_reportCanceled_0<(/*void*/)> for () {
  fn reportCanceled_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase14reportCanceledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:84
// index:0
// Public Visibility=Default Availability=Available
// [-2] void reportResultsReady(int, int)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn reportResultsReady_0<RetType, T: QFutureInterfaceBase_reportResultsReady_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.reportResultsReady_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_reportResultsReady_0<RetType> {
  fn reportResultsReady_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_reportResultsReady_0<(/*void*/)> for (i32,i32) {
  fn reportResultsReady_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase18reportResultsReadyEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:86
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setRunnable(QRunnable *)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setRunnable_0<RetType, T: QFutureInterfaceBase_setRunnable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setRunnable_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setRunnable_0<RetType> {
  fn setRunnable_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setRunnable_0<(/*void*/)> for (usize) {
  fn setRunnable_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase11setRunnableEP9QRunnable", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setThreadPool(QThreadPool *)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setThreadPool_0<RetType, T: QFutureInterfaceBase_setThreadPool_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setThreadPool_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setThreadPool_0<RetType> {
  fn setThreadPool_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setThreadPool_0<(/*void*/)> for (usize) {
  fn setThreadPool_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase13setThreadPoolEP11QThreadPool", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilterMode(bool)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setFilterMode_0<RetType, T: QFutureInterfaceBase_setFilterMode_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilterMode_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setFilterMode_0<RetType> {
  fn setFilterMode_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setFilterMode_0<(/*void*/)> for (bool) {
  fn setFilterMode_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase13setFilterModeEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:89
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProgressRange(int, int)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setProgressRange_0<RetType, T: QFutureInterfaceBase_setProgressRange_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProgressRange_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setProgressRange_0<RetType> {
  fn setProgressRange_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setProgressRange_0<(/*void*/)> for (i32,i32) {
  fn setProgressRange_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase16setProgressRangeEii", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:90
// index:0
// Public Visibility=Default Availability=Available
// [4] int progressMinimum() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressMinimum_0<RetType, T: QFutureInterfaceBase_progressMinimum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressMinimum_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_progressMinimum_0<RetType> {
  fn progressMinimum_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_progressMinimum_0<i32> for () {
  fn progressMinimum_0(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase15progressMinimumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:91
// index:0
// Public Visibility=Default Availability=Available
// [4] int progressMaximum() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressMaximum_0<RetType, T: QFutureInterfaceBase_progressMaximum_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressMaximum_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_progressMaximum_0<RetType> {
  fn progressMaximum_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_progressMaximum_0<i32> for () {
  fn progressMaximum_0(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase15progressMaximumEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:92
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isProgressUpdateNeeded() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isProgressUpdateNeeded_0<RetType, T: QFutureInterfaceBase_isProgressUpdateNeeded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isProgressUpdateNeeded_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isProgressUpdateNeeded_0<RetType> {
  fn isProgressUpdateNeeded_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isProgressUpdateNeeded_0<bool> for () {
  fn isProgressUpdateNeeded_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase22isProgressUpdateNeededEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:93
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProgressValue(int)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setProgressValue_0<RetType, T: QFutureInterfaceBase_setProgressValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProgressValue_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setProgressValue_0<RetType> {
  fn setProgressValue_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setProgressValue_0<(/*void*/)> for (i32) {
  fn setProgressValue_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase16setProgressValueEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:94
// index:0
// Public Visibility=Default Availability=Available
// [4] int progressValue() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressValue_0<RetType, T: QFutureInterfaceBase_progressValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressValue_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_progressValue_0<RetType> {
  fn progressValue_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_progressValue_0<i32> for () {
  fn progressValue_0(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase13progressValueEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:95
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setProgressValueAndText(int, const QString &)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setProgressValueAndText_0<RetType, T: QFutureInterfaceBase_setProgressValueAndText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setProgressValueAndText_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setProgressValueAndText_0<RetType> {
  fn setProgressValueAndText_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setProgressValueAndText_0<(/*void*/)> for (i32,usize) {
  fn setProgressValueAndText_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase23setProgressValueAndTextEiRK7QString", 2,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:96
// index:0
// Public Visibility=Default Availability=Available
// [8] QString progressText() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn progressText_0<RetType, T: QFutureInterfaceBase_progressText_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.progressText_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_progressText_0<RetType> {
  fn progressText_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_progressText_0<usize> for () {
  fn progressText_0(self , rsthis: & QFutureInterfaceBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase12progressTextEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:98
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setExpectedResultCount(int)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setExpectedResultCount_0<RetType, T: QFutureInterfaceBase_setExpectedResultCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setExpectedResultCount_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setExpectedResultCount_0<RetType> {
  fn setExpectedResultCount_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setExpectedResultCount_0<(/*void*/)> for (i32) {
  fn setExpectedResultCount_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase22setExpectedResultCountEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:99
// index:0
// Public Visibility=Default Availability=Available
// [4] int expectedResultCount()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn expectedResultCount_0<RetType, T: QFutureInterfaceBase_expectedResultCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.expectedResultCount_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_expectedResultCount_0<RetType> {
  fn expectedResultCount_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_expectedResultCount_0<i32> for () {
  fn expectedResultCount_0(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase19expectedResultCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] int resultCount() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn resultCount_0<RetType, T: QFutureInterfaceBase_resultCount_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resultCount_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_resultCount_0<RetType> {
  fn resultCount_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_resultCount_0<i32> for () {
  fn resultCount_0(self , rsthis: & QFutureInterfaceBase) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase11resultCountEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:102
// index:0
// Public Visibility=Default Availability=Available
// [1] bool queryState(QFutureInterfaceBase::State) const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn queryState_0<RetType, T: QFutureInterfaceBase_queryState_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.queryState_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_queryState_0<RetType> {
  fn queryState_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_queryState_0<bool> for (i32) {
  fn queryState_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase10queryStateENS_5StateE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:103
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRunning() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isRunning_0<RetType, T: QFutureInterfaceBase_isRunning_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRunning_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isRunning_0<RetType> {
  fn isRunning_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isRunning_0<bool> for () {
  fn isRunning_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase9isRunningEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:104
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isStarted() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isStarted_0<RetType, T: QFutureInterfaceBase_isStarted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isStarted_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isStarted_0<RetType> {
  fn isStarted_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isStarted_0<bool> for () {
  fn isStarted_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase9isStartedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isCanceled() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isCanceled_0<RetType, T: QFutureInterfaceBase_isCanceled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isCanceled_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isCanceled_0<RetType> {
  fn isCanceled_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isCanceled_0<bool> for () {
  fn isCanceled_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase10isCanceledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFinished() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isFinished_0<RetType, T: QFutureInterfaceBase_isFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFinished_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isFinished_0<RetType> {
  fn isFinished_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isFinished_0<bool> for () {
  fn isFinished_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase10isFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isPaused() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isPaused_0<RetType, T: QFutureInterfaceBase_isPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isPaused_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isPaused_0<RetType> {
  fn isPaused_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isPaused_0<bool> for () {
  fn isPaused_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase8isPausedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:108
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isThrottled() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isThrottled_0<RetType, T: QFutureInterfaceBase_isThrottled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isThrottled_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isThrottled_0<RetType> {
  fn isThrottled_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isThrottled_0<bool> for () {
  fn isThrottled_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase11isThrottledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isResultReadyAt(int) const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn isResultReadyAt_0<RetType, T: QFutureInterfaceBase_isResultReadyAt_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isResultReadyAt_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_isResultReadyAt_0<RetType> {
  fn isResultReadyAt_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_isResultReadyAt_0<bool> for (i32) {
  fn isResultReadyAt_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase15isResultReadyAtEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:111
// index:0
// Public Visibility=Default Availability=Available
// [-2] void cancel()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn cancel_0<RetType, T: QFutureInterfaceBase_cancel_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cancel_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_cancel_0<RetType> {
  fn cancel_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_cancel_0<(/*void*/)> for () {
  fn cancel_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase6cancelEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:112
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPaused(bool)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setPaused_0<RetType, T: QFutureInterfaceBase_setPaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPaused_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setPaused_0<RetType> {
  fn setPaused_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setPaused_0<(/*void*/)> for (bool) {
  fn setPaused_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase9setPausedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:113
// index:0
// Public Visibility=Default Availability=Available
// [-2] void togglePaused()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn togglePaused_0<RetType, T: QFutureInterfaceBase_togglePaused_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.togglePaused_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_togglePaused_0<RetType> {
  fn togglePaused_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_togglePaused_0<(/*void*/)> for () {
  fn togglePaused_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase12togglePausedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:114
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setThrottled(bool)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn setThrottled_0<RetType, T: QFutureInterfaceBase_setThrottled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setThrottled_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_setThrottled_0<RetType> {
  fn setThrottled_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_setThrottled_0<(/*void*/)> for (bool) {
  fn setThrottled_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase12setThrottledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void waitForFinished()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForFinished_0<RetType, T: QFutureInterfaceBase_waitForFinished_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForFinished_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_waitForFinished_0<RetType> {
  fn waitForFinished_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_waitForFinished_0<(/*void*/)> for () {
  fn waitForFinished_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase15waitForFinishedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool waitForNextResult()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForNextResult_0<RetType, T: QFutureInterfaceBase_waitForNextResult_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForNextResult_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_waitForNextResult_0<RetType> {
  fn waitForNextResult_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_waitForNextResult_0<bool> for () {
  fn waitForNextResult_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase17waitForNextResultEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:118
// index:0
// Public Visibility=Default Availability=Available
// [-2] void waitForResult(int)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForResult_0<RetType, T: QFutureInterfaceBase_waitForResult_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForResult_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_waitForResult_0<RetType> {
  fn waitForResult_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_waitForResult_0<(/*void*/)> for (i32) {
  fn waitForResult_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase13waitForResultEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:119
// index:0
// Public Visibility=Default Availability=Available
// [-2] void waitForResume()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn waitForResume_0<RetType, T: QFutureInterfaceBase_waitForResume_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.waitForResume_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_waitForResume_0<RetType> {
  fn waitForResume_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_waitForResume_0<(/*void*/)> for () {
  fn waitForResume_0(self , rsthis: & QFutureInterfaceBase) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase13waitForResumeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:121
// index:0
// Public Visibility=Default Availability=Available
// [8] QMutex * mutex() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn mutex_0<RetType, T: QFutureInterfaceBase_mutex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mutex_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_mutex_0<RetType> {
  fn mutex_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_mutex_0<usize> for () {
  fn mutex_0(self , rsthis: & QFutureInterfaceBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase5mutexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:122
// index:0
// Public Visibility=Default Availability=Available
// [1] QtPrivate::ExceptionStore & exceptionStore()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn exceptionStore_0<RetType, T: QFutureInterfaceBase_exceptionStore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exceptionStore_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_exceptionStore_0<RetType> {
  fn exceptionStore_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_exceptionStore_0<usize> for () {
  fn exceptionStore_0(self , rsthis: & QFutureInterfaceBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase14exceptionStoreEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:123
// index:0
// Public Visibility=Default Availability=Available
// [48] QtPrivate::ResultStoreBase & resultStoreBase()

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn resultStoreBase_0<RetType, T: QFutureInterfaceBase_resultStoreBase_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resultStoreBase_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_resultStoreBase_0<RetType> {
  fn resultStoreBase_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_resultStoreBase_0<usize> for () {
  fn resultStoreBase_0(self , rsthis: & QFutureInterfaceBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBase15resultStoreBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:124
// index:1
// Public Visibility=Default Availability=Available
// [48] const QtPrivate::ResultStoreBase & resultStoreBase() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn resultStoreBase_1<RetType, T: QFutureInterfaceBase_resultStoreBase_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resultStoreBase_1(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_resultStoreBase_1<RetType> {
  fn resultStoreBase_1(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_resultStoreBase_1<usize> for () {
  fn resultStoreBase_1(self , rsthis: & QFutureInterfaceBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase15resultStoreBaseEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:126
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator==(const QFutureInterfaceBase &) const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn operator_equal_equal_0<RetType, T: QFutureInterfaceBase_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBaseeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:127
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QFutureInterfaceBase &) const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn operator_not_equal_0<RetType, T: QFutureInterfaceBase_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBaseneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:128
// index:0
// Public Visibility=Default Availability=Available
// [16] QFutureInterfaceBase & operator=(const QFutureInterfaceBase &)

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn operator_equal_0<RetType, T: QFutureInterfaceBase_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QFutureInterfaceBase) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN20QFutureInterfaceBaseaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:131
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool refT() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn refT_0<RetType, T: QFutureInterfaceBase_refT_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refT_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_refT_0<RetType> {
  fn refT_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_refT_0<bool> for () {
  fn refT_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase4refTEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfutureinterface.h:132
// index:0
// Protected Visibility=Default Availability=Available
// [1] bool derefT() const

/*

*/
impl /*struct*/ QFutureInterfaceBase {
  pub fn derefT_0<RetType, T: QFutureInterfaceBase_derefT_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.derefT_0(self);
    // return 1;
  }
}
pub trait QFutureInterfaceBase_derefT_0<RetType> {
  fn derefT_0(self , rsthis: & QFutureInterfaceBase) -> RetType;
}
impl<'a> /*trait*/ QFutureInterfaceBase_derefT_0<bool> for () {
  fn derefT_0(self , rsthis: & QFutureInterfaceBase) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QFutureInterfaceBase6derefTEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


*/
pub type QFutureInterfaceBase__State = i32;
// 
pub const QFutureInterfaceBase__NoState :QFutureInterfaceBase__State = 0;
// 
pub const QFutureInterfaceBase__Running :QFutureInterfaceBase__State = 1;
// 
pub const QFutureInterfaceBase__Started :QFutureInterfaceBase__State = 2;
// 
pub const QFutureInterfaceBase__Finished :QFutureInterfaceBase__State = 4;
// 
pub const QFutureInterfaceBase__Canceled :QFutureInterfaceBase__State = 8;
// 
pub const QFutureInterfaceBase__Paused :QFutureInterfaceBase__State = 16;
// 
pub const QFutureInterfaceBase__Throttled :QFutureInterfaceBase__State = 32;
pub fn QFutureInterfaceBase_StateItemName(val: i32) ->String {
  match val {
     QFutureInterfaceBase__NoState => // 0
     {return String::from("NoState");}
     QFutureInterfaceBase__Running => // 1
     {return String::from("Running");}
     QFutureInterfaceBase__Started => // 2
     {return String::from("Started");}
     QFutureInterfaceBase__Finished => // 4
     {return String::from("Finished");}
     QFutureInterfaceBase__Canceled => // 8
     {return String::from("Canceled");}
     QFutureInterfaceBase__Paused => // 16
     {return String::from("Paused");}
     QFutureInterfaceBase__Throttled => // 32
     {return String::from("Throttled");}
  _ => {return format!("{}", val);}
}
}
pub fn QFutureInterfaceBase_StateItemName_s(val: i32) ->String {
  //var nilthis *QFutureInterfaceBase
  //return nilthis.StateItemName(val);
  return QFutureInterfaceBase_StateItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
