

// mod ::core::QReadWriteLock
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
// extern C begin: 12
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
#[derive(Default)] // class sizeof(QReadWriteLock)=8
pub struct QReadWriteLock {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QReadWriteLock_ITF interface {
//    QReadWriteLock_PTR() *QReadWriteLock
//}
//func (ptr *QReadWriteLock) QReadWriteLock_PTR() *QReadWriteLock { return ptr }

impl /*struct*/ QReadWriteLock {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QReadWriteLock {
    return QReadWriteLock{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QReadWriteLock {
//  type Target = QReadWriteLockBASE;
//
//  fn deref(&self) -> &QReadWriteLockBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QReadWriteLockBASE> for QReadWriteLock {
//  fn as_ref(& self) -> & QReadWriteLockBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qreadwritelock.h:57
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QReadWriteLock(QReadWriteLock::RecursionMode)

/*
Constructs a QReadWriteLock object in the given recursionMode.

The default recursion mode is NonRecursive.

This function was introduced in  Qt 4.4.

See also lockForRead(), lockForWrite(), and RecursionMode.
*/
// QReadWriteLock(QReadWriteLock::RecursionMode) ctx.fn_proto_cpp
impl /*struct*/ QReadWriteLock {
  pub fn QReadWriteLock_0<T: QReadWriteLock_QReadWriteLock_0>(value: T) -> QReadWriteLock {
    let rsthis = value.QReadWriteLock_0();
    return rsthis;
    // return 1;
  }
}

pub trait QReadWriteLock_QReadWriteLock_0 {
  fn QReadWriteLock_0(self) -> QReadWriteLock;
}
// QReadWriteLock(QReadWriteLock::RecursionMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QReadWriteLock_QReadWriteLock_0 for (i32) {
  fn QReadWriteLock_0(self) -> QReadWriteLock {
    // unsafe{_ZN14QReadWriteLockC2ENS_13RecursionModeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN14QReadWriteLockC2ENS_13RecursionModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QReadWriteLock{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QReadWriteLock()

/*

*/
pub fn DeleteQReadWriteLock(this :*mut QReadWriteLock) {
    // let rv = qtrt::InvokeQtFunc6("_ZN14QReadWriteLockD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qreadwritelock.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lockForRead()

/*
Locks the lock for reading. This function will block the current thread if another thread has locked for writing.

It is not possible to lock for read if the thread already has locked for write.

See also unlock(), lockForWrite(), and tryLockForRead().
*/
impl /*struct*/ QReadWriteLock {
  pub fn lockForRead_0<RetType, T: QReadWriteLock_lockForRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lockForRead_0(self);
    // return 1;
  }
}
pub trait QReadWriteLock_lockForRead_0<RetType> {
  fn lockForRead_0(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_lockForRead_0<(/*void*/)> for () {
  fn lockForRead_0(self , rsthis: & QReadWriteLock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QReadWriteLock11lockForReadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:61
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryLockForRead()

/*
Attempts to lock for reading. If the lock was obtained, this function returns true, otherwise it returns false instead of waiting for the lock to become available, i.e. it does not block.

The lock attempt will fail if another thread has locked for writing.

If the lock was obtained, the lock must be unlocked with unlock() before another thread can successfully lock it for writing.

It is not possible to lock for read if the thread already has locked for write.

See also unlock() and lockForRead().
*/
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForRead_0<RetType, T: QReadWriteLock_tryLockForRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLockForRead_0(self);
    // return 1;
  }
}
pub trait QReadWriteLock_tryLockForRead_0<RetType> {
  fn tryLockForRead_0(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead_0<bool> for () {
  fn tryLockForRead_0(self , rsthis: & QReadWriteLock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QReadWriteLock14tryLockForReadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:62
// index:1
// Public Visibility=Default Availability=Available
// [1] bool tryLockForRead(int)

/*
Attempts to lock for reading. If the lock was obtained, this function returns true, otherwise it returns false instead of waiting for the lock to become available, i.e. it does not block.

The lock attempt will fail if another thread has locked for writing.

If the lock was obtained, the lock must be unlocked with unlock() before another thread can successfully lock it for writing.

It is not possible to lock for read if the thread already has locked for write.

See also unlock() and lockForRead().
*/
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForRead_1<RetType, T: QReadWriteLock_tryLockForRead_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLockForRead_1(self);
    // return 1;
  }
}
pub trait QReadWriteLock_tryLockForRead_1<RetType> {
  fn tryLockForRead_1(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_tryLockForRead_1<bool> for (i32) {
  fn tryLockForRead_1(self , rsthis: & QReadWriteLock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QReadWriteLock14tryLockForReadEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void lockForWrite()

/*
Locks the lock for writing. This function will block the current thread if another thread (including the current) has locked for reading or writing (unless the lock has been created using the QReadWriteLock::Recursive mode).

It is not possible to lock for write if the thread already has locked for read.

See also unlock(), lockForRead(), and tryLockForWrite().
*/
impl /*struct*/ QReadWriteLock {
  pub fn lockForWrite_0<RetType, T: QReadWriteLock_lockForWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lockForWrite_0(self);
    // return 1;
  }
}
pub trait QReadWriteLock_lockForWrite_0<RetType> {
  fn lockForWrite_0(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_lockForWrite_0<(/*void*/)> for () {
  fn lockForWrite_0(self , rsthis: & QReadWriteLock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QReadWriteLock12lockForWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:65
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryLockForWrite()

/*
Attempts to lock for writing. If the lock was obtained, this function returns true; otherwise, it returns false immediately.

The lock attempt will fail if another thread has locked for reading or writing.

If the lock was obtained, the lock must be unlocked with unlock() before another thread can successfully lock it.

It is not possible to lock for write if the thread already has locked for read.

See also unlock() and lockForWrite().
*/
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForWrite_0<RetType, T: QReadWriteLock_tryLockForWrite_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLockForWrite_0(self);
    // return 1;
  }
}
pub trait QReadWriteLock_tryLockForWrite_0<RetType> {
  fn tryLockForWrite_0(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite_0<bool> for () {
  fn tryLockForWrite_0(self , rsthis: & QReadWriteLock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QReadWriteLock15tryLockForWriteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:66
// index:1
// Public Visibility=Default Availability=Available
// [1] bool tryLockForWrite(int)

/*
Attempts to lock for writing. If the lock was obtained, this function returns true; otherwise, it returns false immediately.

The lock attempt will fail if another thread has locked for reading or writing.

If the lock was obtained, the lock must be unlocked with unlock() before another thread can successfully lock it.

It is not possible to lock for write if the thread already has locked for read.

See also unlock() and lockForWrite().
*/
impl /*struct*/ QReadWriteLock {
  pub fn tryLockForWrite_1<RetType, T: QReadWriteLock_tryLockForWrite_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLockForWrite_1(self);
    // return 1;
  }
}
pub trait QReadWriteLock_tryLockForWrite_1<RetType> {
  fn tryLockForWrite_1(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_tryLockForWrite_1<bool> for (i32) {
  fn tryLockForWrite_1(self , rsthis: & QReadWriteLock) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN14QReadWriteLock15tryLockForWriteEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qreadwritelock.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unlock()

/*
Unlocks the lock.

Attempting to unlock a lock that is not locked is an error, and will result in program termination.

See also lockForRead(), lockForWrite(), tryLockForRead(), and tryLockForWrite().
*/
impl /*struct*/ QReadWriteLock {
  pub fn unlock_0<RetType, T: QReadWriteLock_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QReadWriteLock_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QReadWriteLock) -> RetType;
}
impl<'a> /*trait*/ QReadWriteLock_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QReadWriteLock) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN14QReadWriteLock6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


This enum was introduced or modified in  Qt 4.4.

See also QReadWriteLock().

*/
pub type QReadWriteLock__RecursionMode = i32;
// In this mode, a thread may only lock a QReadWriteLock once.
pub const QReadWriteLock__NonRecursive :QReadWriteLock__RecursionMode = 0;
// In this mode, a thread can lock the same QReadWriteLock multiple times. The QReadWriteLock won't be unlocked until a corresponding number of unlock() calls have been made.
pub const QReadWriteLock__Recursive :QReadWriteLock__RecursionMode = 1;
pub fn QReadWriteLock_RecursionModeItemName(val: i32) ->String {
  match val {
     QReadWriteLock__NonRecursive => // 0
     {return String::from("NonRecursive");}
     QReadWriteLock__Recursive => // 1
     {return String::from("Recursive");}
  _ => {return format!("{}", val);}
}
}
pub fn QReadWriteLock_RecursionModeItemName_s(val: i32) ->String {
  //var nilthis *QReadWriteLock
  //return nilthis.RecursionModeItemName(val);
  return QReadWriteLock_RecursionModeItemName(val);
}


/*


*/
pub type QReadWriteLock__StateForWaitCondition = i32;
// 
pub const QReadWriteLock__LockedForRead :QReadWriteLock__StateForWaitCondition = 0;
// 
pub const QReadWriteLock__LockedForWrite :QReadWriteLock__StateForWaitCondition = 1;
// 
pub const QReadWriteLock__Unlocked :QReadWriteLock__StateForWaitCondition = 2;
// 
pub const QReadWriteLock__RecursivelyLocked :QReadWriteLock__StateForWaitCondition = 3;
pub fn QReadWriteLock_StateForWaitConditionItemName(val: i32) ->String {
  match val {
     QReadWriteLock__LockedForRead => // 0
     {return String::from("LockedForRead");}
     QReadWriteLock__LockedForWrite => // 1
     {return String::from("LockedForWrite");}
     QReadWriteLock__Unlocked => // 2
     {return String::from("Unlocked");}
     QReadWriteLock__RecursivelyLocked => // 3
     {return String::from("RecursivelyLocked");}
  _ => {return format!("{}", val);}
}
}
pub fn QReadWriteLock_StateForWaitConditionItemName_s(val: i32) ->String {
  //var nilthis *QReadWriteLock
  //return nilthis.StateForWaitConditionItemName(val);
  return QReadWriteLock_StateForWaitConditionItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
