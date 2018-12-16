

// mod ::core::QSystemSemaphore
// package qtcore
// /usr/include/qt/QtCore/qsystemsemaphore.h
// #include <qsystemsemaphore.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 14
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
#[derive(Default)] // class sizeof(QSystemSemaphore)=8
pub struct QSystemSemaphore {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSystemSemaphore_ITF interface {
//    QSystemSemaphore_PTR() *QSystemSemaphore
//}
//func (ptr *QSystemSemaphore) QSystemSemaphore_PTR() *QSystemSemaphore { return ptr }

impl /*struct*/ QSystemSemaphore {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSystemSemaphore {
    return QSystemSemaphore{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSystemSemaphore {
//  type Target = QSystemSemaphoreBASE;
//
//  fn deref(&self) -> &QSystemSemaphoreBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSystemSemaphoreBASE> for QSystemSemaphore {
//  fn as_ref(& self) -> & QSystemSemaphoreBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsystemsemaphore.h:74
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSystemSemaphore(const QString &, int, QSystemSemaphore::AccessMode)

/*
Requests a system semaphore for the specified key. The parameters initialValue and mode are used according to the following rules, which are system dependent.

In Unix, if the mode is Open and the system already has a semaphore identified by key, that semaphore is used, and the semaphore's resource count is not changed, i.e., initialValue is ignored. But if the system does not already have a semaphore identified by key, it creates a new semaphore for that key and sets its resource count to initialValue.

In Unix, if the mode is Create and the system already has a semaphore identified by key, that semaphore is used, and its resource count is set to initialValue. If the system does not already have a semaphore identified by key, it creates a new semaphore for that key and sets its resource count to initialValue.

In Windows, mode is ignored, and the system always tries to create a semaphore for the specified key. If the system does not already have a semaphore identified as key, it creates the semaphore and sets its resource count to initialValue. But if the system already has a semaphore identified as key it uses that semaphore and ignores initialValue.

The mode parameter is only used in Unix systems to handle the case where a semaphore survives a process crash. In that case, the next process to allocate a semaphore with the same key will get the semaphore that survived the crash, and unless mode is Create, the resource count will not be reset to initialValue but will retain the initial value it had been given by the crashed process.

See also acquire() and key().
*/
// QSystemSemaphore(const QString &, int, QSystemSemaphore::AccessMode) ctx.fn_proto_cpp
impl /*struct*/ QSystemSemaphore {
  pub fn QSystemSemaphore_0<T: QSystemSemaphore_QSystemSemaphore_0>(value: T) -> QSystemSemaphore {
    let rsthis = value.QSystemSemaphore_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemSemaphore_QSystemSemaphore_0 {
  fn QSystemSemaphore_0(self) -> QSystemSemaphore;
}
// QSystemSemaphore(const QString &, int, QSystemSemaphore::AccessMode) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSystemSemaphore_QSystemSemaphore_0 for (usize,i32,i32) {
  fn QSystemSemaphore_0(self) -> QSystemSemaphore {
    // unsafe{_ZN16QSystemSemaphoreC2ERK7QStringiNS_10AccessModeE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN16QSystemSemaphoreC2ERK7QStringiNS_10AccessModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSystemSemaphore{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsystemsemaphore.h:75
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QSystemSemaphore()

/*

*/
pub fn DeleteQSystemSemaphore(this :*mut QSystemSemaphore) {
    // let rv = qtrt::InvokeQtFunc6("_ZN16QSystemSemaphoreD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsystemsemaphore.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKey(const QString &, int, QSystemSemaphore::AccessMode)

/*
This function works the same as the constructor. It reconstructs this QSystemSemaphore object. If the new key is different from the old key, calling this function is like calling the destructor of the semaphore with the old key, then calling the constructor to create a new semaphore with the new key. The initialValue and mode parameters are as defined for the constructor.

See also QSystemSemaphore() and key().
*/
impl /*struct*/ QSystemSemaphore {
  pub fn setKey_0<RetType, T: QSystemSemaphore_setKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKey_0(self);
    // return 1;
  }
}
pub trait QSystemSemaphore_setKey_0<RetType> {
  fn setKey_0(self , rsthis: & QSystemSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSystemSemaphore_setKey_0<(/*void*/)> for (usize,i32,i32) {
  fn setKey_0(self , rsthis: & QSystemSemaphore) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN16QSystemSemaphore6setKeyERK7QStringiNS_10AccessModeE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsystemsemaphore.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QString key() const

/*
Returns the key assigned to this system semaphore. The key is the name by which the semaphore can be accessed from other processes.

See also setKey().
*/
impl /*struct*/ QSystemSemaphore {
  pub fn key_0<RetType, T: QSystemSemaphore_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QSystemSemaphore_key_0<RetType> {
  fn key_0(self , rsthis: & QSystemSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSystemSemaphore_key_0<usize> for () {
  fn key_0(self , rsthis: & QSystemSemaphore) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QSystemSemaphore3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsystemsemaphore.h:80
// index:0
// Public Visibility=Default Availability=Available
// [1] bool acquire()

/*
Acquires one of the resources guarded by this semaphore, if there is one available, and returns true. If all the resources guarded by this semaphore have already been acquired, the call blocks until one of them is released by another process or thread having a semaphore with the same key.

If false is returned, a system error has occurred. Call error() to get a value of QSystemSemaphore::SystemSemaphoreError that indicates which error occurred.

See also release().
*/
impl /*struct*/ QSystemSemaphore {
  pub fn acquire_0<RetType, T: QSystemSemaphore_acquire_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.acquire_0(self);
    // return 1;
  }
}
pub trait QSystemSemaphore_acquire_0<RetType> {
  fn acquire_0(self , rsthis: & QSystemSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSystemSemaphore_acquire_0<bool> for () {
  fn acquire_0(self , rsthis: & QSystemSemaphore) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QSystemSemaphore7acquireEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsystemsemaphore.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool release(int)

/*
Releases n resources guarded by the semaphore. Returns true unless there is a system error.

Example: Create a system semaphore having five resources; acquire them all and then release them all.


  QSystemSemaphore sem("market", 5, QSystemSemaphore::Create);
  for (int i = 0; i < 5; ++i)  // acquire all 5 resources
      sem.acquire();
  sem.release(5);              // release the 5 resources



This function can also "create" resources. For example, immediately following the sequence of statements above, suppose we add the statement:


  sem.release(10);          // "create" 10 new resources



Ten new resources are now guarded by the semaphore, in addition to the five that already existed. You would not normally use this function to create more resources.

See also acquire().
*/
impl /*struct*/ QSystemSemaphore {
  pub fn release_0<RetType, T: QSystemSemaphore_release_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.release_0(self);
    // return 1;
  }
}
pub trait QSystemSemaphore_release_0<RetType> {
  fn release_0(self , rsthis: & QSystemSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSystemSemaphore_release_0<bool> for (i32) {
  fn release_0(self , rsthis: & QSystemSemaphore) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN16QSystemSemaphore7releaseEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsystemsemaphore.h:83
// index:0
// Public Visibility=Default Availability=Available
// [4] QSystemSemaphore::SystemSemaphoreError error() const

/*
Returns a value indicating whether an error occurred, and, if so, which error it was.

See also errorString().
*/
impl /*struct*/ QSystemSemaphore {
  pub fn error_0<RetType, T: QSystemSemaphore_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QSystemSemaphore_error_0<RetType> {
  fn error_0(self , rsthis: & QSystemSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSystemSemaphore_error_0<i32> for () {
  fn error_0(self , rsthis: & QSystemSemaphore) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QSystemSemaphore5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsystemsemaphore.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a text description of the last error that occurred. If error() returns an error value, call this function to get a text string that describes the error.

See also error().
*/
impl /*struct*/ QSystemSemaphore {
  pub fn errorString_0<RetType, T: QSystemSemaphore_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QSystemSemaphore_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QSystemSemaphore) -> RetType;
}
impl<'a> /*trait*/ QSystemSemaphore_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QSystemSemaphore) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK16QSystemSemaphore11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*
This enum is used by the constructor and setKey(). Its purpose is to enable handling the problem in Unix implementations of semaphores that survive a crash. In Unix, when a semaphore survives a crash, we need a way to force it to reset its resource count, when the system reuses the semaphore. In Windows, where semaphores can't survive a crash, this enum has no effect.


*/
pub type QSystemSemaphore__AccessMode = i32;
// If the semaphore already exists, its initial resource count is not reset. If the semaphore does not already exist, it is created and its initial resource count set.
pub const QSystemSemaphore__Open :QSystemSemaphore__AccessMode = 0;
// QSystemSemaphore takes ownership of the semaphore and sets its resource count to the requested value, regardless of whether the semaphore already exists by having survived a crash. This value should be passed to the constructor, when the first semaphore for a particular key is constructed and you know that if the semaphore already exists it could only be because of a crash. In Windows, where a semaphore can't survive a crash, Create and Open have the same behavior.
pub const QSystemSemaphore__Create :QSystemSemaphore__AccessMode = 1;
pub fn QSystemSemaphore_AccessModeItemName(val: i32) ->String {
  match val {
     QSystemSemaphore__Open => // 0
     {return String::from("Open");}
     QSystemSemaphore__Create => // 1
     {return String::from("Create");}
  _ => {return format!("{}", val);}
}
}
pub fn QSystemSemaphore_AccessModeItemName_s(val: i32) ->String {
  //var nilthis *QSystemSemaphore
  //return nilthis.AccessModeItemName(val);
  return QSystemSemaphore_AccessModeItemName(val);
}


/*

*/
pub type QSystemSemaphore__SystemSemaphoreError = i32;
// No error occurred.
pub const QSystemSemaphore__NoError :QSystemSemaphore__SystemSemaphoreError = 0;
// The operation failed because the caller didn't have the required permissions.
pub const QSystemSemaphore__PermissionDenied :QSystemSemaphore__SystemSemaphoreError = 1;
// The operation failed because of an invalid key.
pub const QSystemSemaphore__KeyError :QSystemSemaphore__SystemSemaphoreError = 2;
// The operation failed because a system semaphore with the specified key already existed.
pub const QSystemSemaphore__AlreadyExists :QSystemSemaphore__SystemSemaphoreError = 3;
// The operation failed because a system semaphore with the specified key could not be found.
pub const QSystemSemaphore__NotFound :QSystemSemaphore__SystemSemaphoreError = 4;
// The operation failed because there was not enough memory available to fill the request.
pub const QSystemSemaphore__OutOfResources :QSystemSemaphore__SystemSemaphoreError = 5;
// Something else happened and it was bad.
pub const QSystemSemaphore__UnknownError :QSystemSemaphore__SystemSemaphoreError = 6;
pub fn QSystemSemaphore_SystemSemaphoreErrorItemName(val: i32) ->String {
  match val {
     QSystemSemaphore__NoError => // 0
     {return String::from("NoError");}
     QSystemSemaphore__PermissionDenied => // 1
     {return String::from("PermissionDenied");}
     QSystemSemaphore__KeyError => // 2
     {return String::from("KeyError");}
     QSystemSemaphore__AlreadyExists => // 3
     {return String::from("AlreadyExists");}
     QSystemSemaphore__NotFound => // 4
     {return String::from("NotFound");}
     QSystemSemaphore__OutOfResources => // 5
     {return String::from("OutOfResources");}
     QSystemSemaphore__UnknownError => // 6
     {return String::from("UnknownError");}
  _ => {return format!("{}", val);}
}
}
pub fn QSystemSemaphore_SystemSemaphoreErrorItemName_s(val: i32) ->String {
  //var nilthis *QSystemSemaphore
  //return nilthis.SystemSemaphoreErrorItemName(val);
  return QSystemSemaphore_SystemSemaphoreErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
