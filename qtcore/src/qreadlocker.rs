

// mod ::core::QReadLocker
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
// extern C begin: 9
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
#[derive(Default)] // class sizeof(QReadLocker)=8
pub struct QReadLocker {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QReadLocker_ITF interface {
//    QReadLocker_PTR() *QReadLocker
//}
//func (ptr *QReadLocker) QReadLocker_PTR() *QReadLocker { return ptr }

impl /*struct*/ QReadLocker {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QReadLocker {
    return QReadLocker{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QReadLocker {
//  type Target = QReadLockerBASE;
//
//  fn deref(&self) -> &QReadLockerBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QReadLockerBASE> for QReadLocker {
//  fn as_ref(& self) -> & QReadLockerBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qreadwritelock.h:87
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QReadLocker(QReadWriteLock *)

/*

*/
// QReadLocker(QReadWriteLock *) ctx.fn_proto_cpp
impl /*struct*/ QReadLocker {
  pub fn QReadLocker_0<T: QReadLocker_QReadLocker_0>(value: T) -> QReadLocker {
    let rsthis = value.QReadLocker_0();
    return rsthis;
    // return 1;
  }
}

pub trait QReadLocker_QReadLocker_0 {
  fn QReadLocker_0(self) -> QReadLocker;
}
// QReadLocker(QReadWriteLock *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QReadLocker_QReadLocker_0 for (usize) {
  fn QReadLocker_0(self) -> QReadLocker {
    // unsafe{_ZN11QReadLockerC2EP14QReadWriteLock()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QReadLockerC2EP14QReadWriteLock", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QReadLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ~QReadLocker()

/*

*/
pub fn DeleteQReadLocker(this :*mut QReadLocker) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QReadLockerD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qreadwritelock.h:92
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void unlock()

/*
Unlocks the lock.

Attempting to unlock a lock that is not locked is an error, and will result in program termination.

See also lockForRead(), lockForWrite(), tryLockForRead(), and tryLockForWrite().
*/
impl /*struct*/ QReadLocker {
  pub fn unlock_0<RetType, T: QReadLocker_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QReadLocker_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QReadLocker) -> RetType;
}
impl<'a> /*trait*/ QReadLocker_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QReadLocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QReadLocker6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void relock()

/*

*/
impl /*struct*/ QReadLocker {
  pub fn relock_0<RetType, T: QReadLocker_relock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.relock_0(self);
    // return 1;
  }
}
pub trait QReadLocker_relock_0<RetType> {
  fn relock_0(self , rsthis: & QReadLocker) -> RetType;
}
impl<'a> /*trait*/ QReadLocker_relock_0<(/*void*/)> for () {
  fn relock_0(self , rsthis: & QReadLocker) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QReadLocker6relockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:112
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QReadWriteLock * readWriteLock() const

/*

*/
impl /*struct*/ QReadLocker {
  pub fn readWriteLock_0<RetType, T: QReadLocker_readWriteLock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readWriteLock_0(self);
    // return 1;
  }
}
pub trait QReadLocker_readWriteLock_0<RetType> {
  fn readWriteLock_0(self , rsthis: & QReadLocker) -> RetType;
}
impl<'a> /*trait*/ QReadLocker_readWriteLock_0<usize> for () {
  fn readWriteLock_0(self , rsthis: & QReadLocker) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QReadLocker13readWriteLockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
