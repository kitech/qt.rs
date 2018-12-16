

// mod ::core::QMutex
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
#[derive(Default)] // class sizeof(QMutex)=8
pub struct QMutex {
  qbase: QBasicMutex,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QMutex_ITF interface {
//    QBasicMutex_ITF
//    QMutex_PTR() *QMutex
//}
//func (ptr *QMutex) QMutex_PTR() *QMutex { return ptr }

impl /*struct*/ QMutex {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QMutex {
    return QMutex{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QMutex {
//  type Target = QMutexBASE;
//
//  fn deref(&self) -> &QMutexBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QMutexBASE> for QMutex {
//  fn as_ref(& self) -> & QMutexBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qmutex.h:130
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QMutex(QMutex::RecursionMode)

/*
Constructs a new mutex. The mutex is created in an unlocked state.

If mode is QMutex::Recursive, a thread can lock the same mutex multiple times and the mutex won't be unlocked until a corresponding number of unlock() calls have been made. Otherwise a thread may only lock a mutex once. The default is QMutex::NonRecursive.

Recursive mutexes are slower and take more memory than non-recursive ones.

See also lock() and unlock().
*/
// QMutex(QMutex::RecursionMode) ctx.fn_proto_cpp
impl /*struct*/ QMutex {
  pub fn QMutex_0<T: QMutex_QMutex_0>(value: T) -> QMutex {
    let rsthis = value.QMutex_0();
    return rsthis;
    // return 1;
  }
}

pub trait QMutex_QMutex_0 {
  fn QMutex_0(self) -> QMutex;
}
// QMutex(QMutex::RecursionMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QMutex_QMutex_0 for (i32) {
  fn QMutex_0(self) -> QMutex {
    // unsafe{_ZN6QMutexC2ENS_13RecursionModeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QMutexC2ENS_13RecursionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QMutex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:131
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QMutex()

/*

*/
pub fn DeleteQMutex(this :*mut QMutex) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QMutexD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qmutex.h:134
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lock()

/*
Locks the mutex. If another thread has locked the mutex then this call will block until that thread has unlocked it.

Calling this function multiple times on the same mutex from the same thread is allowed if this mutex is a recursive mutex. If this mutex is a non-recursive mutex, this function will dead-lock when the mutex is locked recursively.

See also unlock().
*/
impl /*struct*/ QMutex {
  pub fn lock_0<RetType, T: QMutex_lock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lock_0(self);
    // return 1;
  }
}
pub trait QMutex_lock_0<RetType> {
  fn lock_0(self , rsthis: & QMutex) -> RetType;
}
impl<'a> /*trait*/ QMutex_lock_0<(/*void*/)> for () {
  fn lock_0(self , rsthis: & QMutex) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QMutex4lockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:135
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryLock(int)

/*
Attempts to lock the mutex. This function returns true if the lock was obtained; otherwise it returns false. If another thread has locked the mutex, this function will wait for at most timeout milliseconds for the mutex to become available.

Note: Passing a negative number as the timeout is equivalent to calling lock(), i.e. this function will wait forever until mutex can be locked if timeout is negative.

If the lock was obtained, the mutex must be unlocked with unlock() before another thread can successfully lock it.

Calling this function multiple times on the same mutex from the same thread is allowed if this mutex is a recursive mutex. If this mutex is a non-recursive mutex, this function will always return false when attempting to lock the mutex recursively.

See also lock() and unlock().
*/
impl /*struct*/ QMutex {
  pub fn tryLock_0<RetType, T: QMutex_tryLock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLock_0(self);
    // return 1;
  }
}
pub trait QMutex_tryLock_0<RetType> {
  fn tryLock_0(self , rsthis: & QMutex) -> RetType;
}
impl<'a> /*trait*/ QMutex_tryLock_0<bool> for (i32) {
  fn tryLock_0(self , rsthis: & QMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QMutex7tryLockEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmutex.h:137
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unlock()

/*
Unlocks the mutex. Attempting to unlock a mutex in a different thread to the one that locked it results in an error. Unlocking a mutex that is not locked results in undefined behavior.

See also lock().
*/
impl /*struct*/ QMutex {
  pub fn unlock_0<RetType, T: QMutex_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QMutex_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QMutex) -> RetType;
}
impl<'a> /*trait*/ QMutex_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QMutex) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QMutex6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qmutex.h:140
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool try_lock()

/*
Attempts to lock the mutex. This function returns true if the lock was obtained; otherwise it returns false.

This function is provided for compatibility with the Standard Library concept Lockable. It is equivalent to tryLock().

The function returns true if the lock was obtained; otherwise it returns false

This function was introduced in  Qt 5.8.
*/
impl /*struct*/ QMutex {
  pub fn try_lock_0<RetType, T: QMutex_try_lock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.try_lock_0(self);
    // return 1;
  }
}
pub trait QMutex_try_lock_0<RetType> {
  fn try_lock_0(self , rsthis: & QMutex) -> RetType;
}
impl<'a> /*trait*/ QMutex_try_lock_0<bool> for () {
  fn try_lock_0(self , rsthis: & QMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QMutex8try_lockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qmutex.h:161
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isRecursive() const

/*
Returns true if the mutex is recursive.

This function was introduced in  Qt 5.7.
*/
impl /*struct*/ QMutex {
  pub fn isRecursive_0<RetType, T: QMutex_isRecursive_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRecursive_0(self);
    // return 1;
  }
}
pub trait QMutex_isRecursive_0<RetType> {
  fn isRecursive_0(self , rsthis: & QMutex) -> RetType;
}
impl<'a> /*trait*/ QMutex_isRecursive_0<bool> for () {
  fn isRecursive_0(self , rsthis: & QMutex) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QMutex11isRecursiveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*


See also QMutex().

*/
pub type QMutex__RecursionMode = i32;
// In this mode, a thread may only lock a mutex once.
pub const QMutex__NonRecursive :QMutex__RecursionMode = 0;
// In this mode, a thread can lock the same mutex multiple times and the mutex won't be unlocked until a corresponding number of unlock() calls have been made.
pub const QMutex__Recursive :QMutex__RecursionMode = 1;
pub fn QMutex_RecursionModeItemName(val: i32) ->String {
  match val {
     QMutex__NonRecursive => // 0
     {return String::from("NonRecursive");}
     QMutex__Recursive => // 1
     {return String::from("Recursive");}
  _ => {return format!("{}", val);}
}
}
pub fn QMutex_RecursionModeItemName_s(val: i32) ->String {
  //var nilthis *QMutex
  //return nilthis.RecursionModeItemName(val);
  return QMutex_RecursionModeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
