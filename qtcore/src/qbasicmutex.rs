

// mod ::core::QBasicMutex
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
#[derive(Default)] // class sizeof(QBasicMutex)=8
pub struct QBasicMutex {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QBasicMutex_ITF interface {
//    QBasicMutex_PTR() *QBasicMutex
//}
//func (ptr *QBasicMutex) QBasicMutex_PTR() *QBasicMutex { return ptr }

impl /*struct*/ QBasicMutex {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QBasicMutex {
    return QBasicMutex{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QBasicMutex {
//  type Target = QBasicMutexBASE;
//
//  fn deref(&self) -> &QBasicMutexBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QBasicMutexBASE> for QBasicMutex {
//  fn as_ref(& self) -> & QBasicMutexBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmutex.h:71
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void QBasicMutex()

/*

*/
// QBasicMutex() ctx.fn_proto_cpp
impl /*struct*/ QBasicMutex {
  pub fn QBasicMutex_0<T: QBasicMutex_QBasicMutex_0>(value: T) -> QBasicMutex {
    let rsthis = value.QBasicMutex_0();
    return rsthis;
    // return 1;
  }
}

pub trait QBasicMutex_QBasicMutex_0 {
  fn QBasicMutex_0(self) -> QBasicMutex;
}
// QBasicMutex() ctx.fn_proto_cpp
impl<'a> /*trait*/ QBasicMutex_QBasicMutex_0 for () {
  fn QBasicMutex_0(self) -> QBasicMutex {
    // unsafe{_ZN11QBasicMutexC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QBasicMutexC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QBasicMutex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:77
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void lock()

/*
Locks the mutex. If another thread has locked the mutex then this call will block until that thread has unlocked it.

Calling this function multiple times on the same mutex from the same thread is allowed if this mutex is a recursive mutex. If this mutex is a non-recursive mutex, this function will dead-lock when the mutex is locked recursively.

See also unlock().
*/
impl /*struct*/ QBasicMutex {
  pub fn lock_0<RetType, T: QBasicMutex_lock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lock_0(self);
    // return 1;
  }
}
pub trait QBasicMutex_lock_0<RetType> {
  fn lock_0(self , rsthis: & QBasicMutex) -> RetType;
}
impl<'a> /*trait*/ QBasicMutex_lock_0<(/*void*/)> for () {
  fn lock_0(self , rsthis: & QBasicMutex) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QBasicMutex4lockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:83
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void unlock()

/*
Unlocks the mutex. Attempting to unlock a mutex in a different thread to the one that locked it results in an error. Unlocking a mutex that is not locked results in undefined behavior.

See also lock().
*/
impl /*struct*/ QBasicMutex {
  pub fn unlock_0<RetType, T: QBasicMutex_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QBasicMutex_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QBasicMutex) -> RetType;
}
impl<'a> /*trait*/ QBasicMutex_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QBasicMutex) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QBasicMutex6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:89
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool tryLock()

/*
Attempts to lock the mutex. This function returns true if the lock was obtained; otherwise it returns false. If another thread has locked the mutex, this function will wait for at most timeout milliseconds for the mutex to become available.

Note: Passing a negative number as the timeout is equivalent to calling lock(), i.e. this function will wait forever until mutex can be locked if timeout is negative.

If the lock was obtained, the mutex must be unlocked with unlock() before another thread can successfully lock it.

Calling this function multiple times on the same mutex from the same thread is allowed if this mutex is a recursive mutex. If this mutex is a non-recursive mutex, this function will always return false when attempting to lock the mutex recursively.

See also lock() and unlock().
*/
impl /*struct*/ QBasicMutex {
  pub fn tryLock_0<RetType, T: QBasicMutex_tryLock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLock_0(self);
    // return 1;
  }
}
pub trait QBasicMutex_tryLock_0<RetType> {
  fn tryLock_0(self , rsthis: & QBasicMutex) -> RetType;
}
impl<'a> /*trait*/ QBasicMutex_tryLock_0<bool> for () {
  fn tryLock_0(self , rsthis: & QBasicMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QBasicMutex7tryLockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmutex.h:94
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool try_lock()

/*
Attempts to lock the mutex. This function returns true if the lock was obtained; otherwise it returns false.

This function is provided for compatibility with the Standard Library concept Lockable. It is equivalent to tryLock().

The function returns true if the lock was obtained; otherwise it returns false

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QBasicMutex {
  pub fn try_lock_0<RetType, T: QBasicMutex_try_lock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.try_lock_0(self);
    // return 1;
  }
}
pub trait QBasicMutex_try_lock_0<RetType> {
  fn try_lock_0(self , rsthis: & QBasicMutex) -> RetType;
}
impl<'a> /*trait*/ QBasicMutex_try_lock_0<bool> for () {
  fn try_lock_0(self , rsthis: & QBasicMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QBasicMutex8try_lockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmutex.h:96
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRecursive()

/*
Returns true if the mutex is recursive.

This function was introduced in  Qt 5.7.
*/
impl /*struct*/ QBasicMutex {
  pub fn isRecursive_0<RetType, T: QBasicMutex_isRecursive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRecursive_0(self);
    // return 1;
  }
}
pub trait QBasicMutex_isRecursive_0<RetType> {
  fn isRecursive_0(self , rsthis: & QBasicMutex) -> RetType;
}
impl<'a> /*trait*/ QBasicMutex_isRecursive_0<bool> for () {
  fn isRecursive_0(self , rsthis: & QBasicMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QBasicMutex11isRecursiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmutex.h:97
// index:1
// Public Visibility=Default Availability=Available
// [1] bool isRecursive() const

/*
Returns true if the mutex is recursive.

This function was introduced in  Qt 5.7.
*/
impl /*struct*/ QBasicMutex {
  pub fn isRecursive_1<RetType, T: QBasicMutex_isRecursive_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRecursive_1(self);
    // return 1;
  }
}
pub trait QBasicMutex_isRecursive_1<RetType> {
  fn isRecursive_1(self , rsthis: & QBasicMutex) -> RetType;
}
impl<'a> /*trait*/ QBasicMutex_isRecursive_1<bool> for () {
  fn isRecursive_1(self , rsthis: & QBasicMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QBasicMutex11isRecursiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


pub fn DeleteQBasicMutex(this :*mut QBasicMutex) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN11QBasicMutexD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
