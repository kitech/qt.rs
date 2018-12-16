

// mod ::core::QMutexLocker
// package qtcore
// /usr/include/qt/QtCore/qmutex.h
// #include <qmutex.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 7
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
#[derive(Default)] // class sizeof(QMutexLocker)=8
pub struct QMutexLocker {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMutexLocker_ITF interface {
//    QMutexLocker_PTR() *QMutexLocker
//}
//func (ptr *QMutexLocker) QMutexLocker_PTR() *QMutexLocker { return ptr }

impl /*struct*/ QMutexLocker {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMutexLocker {
    return QMutexLocker{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMutexLocker {
//  type Target = QMutexLockerBASE;
//
//  fn deref(&self) -> &QMutexLockerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMutexLockerBASE> for QMutexLocker {
//  fn as_ref(& self) -> & QMutexLockerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmutex.h:199
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QMutexLocker(QBasicMutex *)

/*

*/
// QMutexLocker(QBasicMutex *) ctx.fn_proto_cpp
impl /*struct*/ QMutexLocker {
  pub fn QMutexLocker_0<T: QMutexLocker_QMutexLocker_0>(value: T) -> QMutexLocker {
    let rsthis = value.QMutexLocker_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMutexLocker_QMutexLocker_0 {
  fn QMutexLocker_0(self) -> QMutexLocker;
}
// QMutexLocker(QBasicMutex *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMutexLocker_QMutexLocker_0 for (usize) {
  fn QMutexLocker_0(self) -> QMutexLocker {
    // unsafe{_ZN12QMutexLockerC2EP11QBasicMutex()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QMutexLockerC2EP11QBasicMutex", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMutexLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:213
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QMutexLocker()

/*

*/
pub fn DeleteQMutexLocker(this :*mut QMutexLocker) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QMutexLockerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qmutex.h:215
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void unlock()

/*
Unlocks the mutex. Attempting to unlock a mutex in a different thread to the one that locked it results in an error. Unlocking a mutex that is not locked results in undefined behavior.

See also lock().
*/
impl /*struct*/ QMutexLocker {
  pub fn unlock_0<RetType, T: QMutexLocker_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QMutexLocker_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QMutexLocker) -> RetType;
}
impl<'a> /*trait*/ QMutexLocker_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QMutexLocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QMutexLocker6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:223
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void relock()

/*

*/
impl /*struct*/ QMutexLocker {
  pub fn relock_0<RetType, T: QMutexLocker_relock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.relock_0(self);
    // return 1;
  }
}
pub trait QMutexLocker_relock_0<RetType> {
  fn relock_0(self , rsthis: & QMutexLocker) -> RetType;
}
impl<'a> /*trait*/ QMutexLocker_relock_0<(/*void*/)> for () {
  fn relock_0(self , rsthis: & QMutexLocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QMutexLocker6relockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:238
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QMutex * mutex() const

/*

*/
impl /*struct*/ QMutexLocker {
  pub fn mutex_0<RetType, T: QMutexLocker_mutex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mutex_0(self);
    // return 1;
  }
}
pub trait QMutexLocker_mutex_0<RetType> {
  fn mutex_0(self , rsthis: & QMutexLocker) -> RetType;
}
impl<'a> /*trait*/ QMutexLocker_mutex_0<usize> for () {
  fn mutex_0(self , rsthis: & QMutexLocker) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QMutexLocker5mutexEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
