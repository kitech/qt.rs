

// mod ::core::QWriteLocker
// package qtcore
// /usr/include/qt/QtCore/qreadwritelock.h
// #include <qreadwritelock.h>
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



/*

*/
#[derive(Default)] // class sizeof(QWriteLocker)=8
pub struct QWriteLocker {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QWriteLocker_ITF interface {
//    QWriteLocker_PTR() *QWriteLocker
//}
//func (ptr *QWriteLocker) QWriteLocker_PTR() *QWriteLocker { return ptr }

impl /*struct*/ QWriteLocker {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QWriteLocker {
    return QWriteLocker{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QWriteLocker {
//  type Target = QWriteLockerBASE;
//
//  fn deref(&self) -> &QWriteLockerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QWriteLockerBASE> for QWriteLocker {
//  fn as_ref(& self) -> & QWriteLockerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qreadwritelock.h:131
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QWriteLocker(QReadWriteLock *)

/*

*/
// QWriteLocker(QReadWriteLock *) ctx.fn_proto_cpp
impl /*struct*/ QWriteLocker {
  pub fn QWriteLocker_0<T: QWriteLocker_QWriteLocker_0>(value: T) -> QWriteLocker {
    let rsthis = value.QWriteLocker_0();
    return rsthis;
    // return 1;
  }
}

pub trait QWriteLocker_QWriteLocker_0 {
  fn QWriteLocker_0(self) -> QWriteLocker;
}
// QWriteLocker(QReadWriteLock *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QWriteLocker_QWriteLocker_0 for (usize) {
  fn QWriteLocker_0(self) -> QWriteLocker {
    // unsafe{_ZN12QWriteLockerC2EP14QReadWriteLock()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QWriteLockerC2EP14QReadWriteLock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QWriteLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:133
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QWriteLocker()

/*

*/
pub fn DeleteQWriteLocker(this :*mut QWriteLocker) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QWriteLockerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qreadwritelock.h:136
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void unlock()

/*
Unlocks the lock.

Attempting to unlock a lock that is not locked is an error, and will result in program termination.

See also lockForRead(), lockForWrite(), tryLockForRead(), and tryLockForWrite().
*/
impl /*struct*/ QWriteLocker {
  pub fn unlock_0<RetType, T: QWriteLocker_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QWriteLocker_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QWriteLocker) -> RetType;
}
impl<'a> /*trait*/ QWriteLocker_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QWriteLocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QWriteLocker6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:146
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void relock()

/*

*/
impl /*struct*/ QWriteLocker {
  pub fn relock_0<RetType, T: QWriteLocker_relock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.relock_0(self);
    // return 1;
  }
}
pub trait QWriteLocker_relock_0<RetType> {
  fn relock_0(self , rsthis: & QWriteLocker) -> RetType;
}
impl<'a> /*trait*/ QWriteLocker_relock_0<(/*void*/)> for () {
  fn relock_0(self , rsthis: & QWriteLocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QWriteLocker6relockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:156
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QReadWriteLock * readWriteLock() const

/*

*/
impl /*struct*/ QWriteLocker {
  pub fn readWriteLock_0<RetType, T: QWriteLocker_readWriteLock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readWriteLock_0(self);
    // return 1;
  }
}
pub trait QWriteLocker_readWriteLock_0<RetType> {
  fn readWriteLock_0(self , rsthis: & QWriteLocker) -> RetType;
}
impl<'a> /*trait*/ QWriteLocker_readWriteLock_0<usize> for () {
  fn readWriteLock_0(self , rsthis: & QWriteLocker) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QWriteLocker13readWriteLockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
