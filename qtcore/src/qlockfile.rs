

// mod ::core::QLockFile
// package qtcore
// /usr/include/qt/QtCore/qlockfile.h
// #include <qlockfile.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 0
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
#[derive(Default)] // class sizeof(QLockFile)=8
pub struct QLockFile {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLockFile_ITF interface {
//    QLockFile_PTR() *QLockFile
//}
//func (ptr *QLockFile) QLockFile_PTR() *QLockFile { return ptr }

impl /*struct*/ QLockFile {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLockFile {
    return QLockFile{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLockFile {
//  type Target = QLockFileBASE;
//
//  fn deref(&self) -> &QLockFileBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLockFileBASE> for QLockFile {
//  fn as_ref(& self) -> & QLockFileBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qlockfile.h:53
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLockFile(const QString &)

/*
Constructs a new lock file object. The object is created in an unlocked state. When calling lock() or tryLock(), a lock file named fileName will be created, if it doesn't already exist.

See also lock() and unlock().
*/
// QLockFile(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QLockFile {
  pub fn QLockFile_0<T: QLockFile_QLockFile_0>(value: T) -> QLockFile {
    let rsthis = value.QLockFile_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLockFile_QLockFile_0 {
  fn QLockFile_0(self) -> QLockFile;
}
// QLockFile(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLockFile_QLockFile_0 for (usize) {
  fn QLockFile_0(self) -> QLockFile {
    // unsafe{_ZN9QLockFileC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QLockFileC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLockFile{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:54
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QLockFile()

/*

*/
pub fn DeleteQLockFile(this :*mut QLockFile) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QLockFileD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qlockfile.h:56
// index:0
// Public Visibility=Default Availability=Available
// [1] bool lock()

/*
Creates the lock file.

If another process (or another thread) has created the lock file already, this function will block until that process (or thread) releases it.

Calling this function multiple times on the same lock from the same thread without unlocking first is not allowed. This function will dead-lock when the file is locked recursively.

Returns true if the lock was acquired, false if it could not be acquired due to an unrecoverable error, such as no permissions in the parent directory.

See also unlock() and tryLock().
*/
impl /*struct*/ QLockFile {
  pub fn lock_0<RetType, T: QLockFile_lock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lock_0(self);
    // return 1;
  }
}
pub trait QLockFile_lock_0<RetType> {
  fn lock_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_lock_0<bool> for () {
  fn lock_0(self , rsthis: & QLockFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLockFile4lockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:57
// index:0
// Public Visibility=Default Availability=Available
// [1] bool tryLock(int)

/*
Attempts to create the lock file. This function returns true if the lock was obtained; otherwise it returns false. If another process (or another thread) has created the lock file already, this function will wait for at most timeout milliseconds for the lock file to become available.

Note: Passing a negative number as the timeout is equivalent to calling lock(), i.e. this function will wait forever until the lock file can be locked if timeout is negative.

If the lock was obtained, it must be released with unlock() before another process (or thread) can successfully lock it.

Calling this function multiple times on the same lock from the same thread without unlocking first is not allowed, this function will always return false when attempting to lock the file recursively.

See also lock() and unlock().
*/
impl /*struct*/ QLockFile {
  pub fn tryLock_0<RetType, T: QLockFile_tryLock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.tryLock_0(self);
    // return 1;
  }
}
pub trait QLockFile_tryLock_0<RetType> {
  fn tryLock_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_tryLock_0<bool> for (i32) {
  fn tryLock_0(self , rsthis: & QLockFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLockFile7tryLockEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unlock()

/*
Releases the lock, by deleting the lock file.

Calling unlock() without locking the file first, does nothing.

See also lock() and tryLock().
*/
impl /*struct*/ QLockFile {
  pub fn unlock_0<RetType, T: QLockFile_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QLockFile_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_unlock_0<(/*void*/)> for () {
  fn unlock_0(self , rsthis: & QLockFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QLockFile6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:60
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setStaleLockTime(int)

/*
Sets staleLockTime to be the time in milliseconds after which a lock file is considered stale. The default value is 30000, i.e. 30 seconds. If your application typically keeps the file locked for more than 30 seconds (for instance while saving megabytes of data for 2 minutes), you should set a bigger value using setStaleLockTime().

The value of staleLockTime is used by lock() and tryLock() in order to determine when an existing lock file is considered stale, i.e. left over by a crashed process. This is useful for the case where the PID got reused meanwhile, so one way to detect a stale lock file is by the fact that it has been around for a long time.

See also staleLockTime().
*/
impl /*struct*/ QLockFile {
  pub fn setStaleLockTime_0<RetType, T: QLockFile_setStaleLockTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setStaleLockTime_0(self);
    // return 1;
  }
}
pub trait QLockFile_setStaleLockTime_0<RetType> {
  fn setStaleLockTime_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_setStaleLockTime_0<(/*void*/)> for (i32) {
  fn setStaleLockTime_0(self , rsthis: & QLockFile) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QLockFile16setStaleLockTimeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:61
// index:0
// Public Visibility=Default Availability=Available
// [4] int staleLockTime() const

/*
Returns the time in milliseconds after which a lock file is considered stale.

See also setStaleLockTime().
*/
impl /*struct*/ QLockFile {
  pub fn staleLockTime_0<RetType, T: QLockFile_staleLockTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.staleLockTime_0(self);
    // return 1;
  }
}
pub trait QLockFile_staleLockTime_0<RetType> {
  fn staleLockTime_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_staleLockTime_0<i32> for () {
  fn staleLockTime_0(self , rsthis: & QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLockFile13staleLockTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:63
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isLocked() const

/*
Returns true if the lock was acquired by this QLockFile instance, otherwise returns false.

See also lock(), unlock(), and tryLock().
*/
impl /*struct*/ QLockFile {
  pub fn isLocked_0<RetType, T: QLockFile_isLocked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLocked_0(self);
    // return 1;
  }
}
pub trait QLockFile_isLocked_0<RetType> {
  fn isLocked_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_isLocked_0<bool> for () {
  fn isLocked_0(self , rsthis: & QLockFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLockFile8isLockedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:64
// index:0
// Public Visibility=Default Availability=Available
// [1] bool getLockInfo(qint64 *, QString *, QString *) const

/*
Retrieves information about the current owner of the lock file.

If tryLock() returns false, and error() returns LockFailedError, this function can be called to find out more information about the existing lock file:


the PID of the application (returned in pid)
the hostname it's running on (useful in case of networked filesystems),
the name of the application which created it (returned in appname),


Note that tryLock() automatically deleted the file if there is no running application with this PID, so LockFailedError can only happen if there is an application with this PID (it could be unrelated though).

This can be used to inform users about the existing lock file and give them the choice to delete it. After removing the file using removeStaleLockFile(), the application can call tryLock() again.

This function returns true if the information could be successfully retrieved, false if the lock file doesn't exist or doesn't contain the expected data. This can happen if the lock file was deleted between the time where tryLock() failed and the call to this function. Simply call tryLock() again if this happens.
*/
impl /*struct*/ QLockFile {
  pub fn getLockInfo_0<RetType, T: QLockFile_getLockInfo_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.getLockInfo_0(self);
    // return 1;
  }
}
pub trait QLockFile_getLockInfo_0<RetType> {
  fn getLockInfo_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_getLockInfo_0<bool> for (usize,usize,usize) {
  fn getLockInfo_0(self , rsthis: & QLockFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLockFile11getLockInfoEPxP7QStringS2_", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:65
// index:0
// Public Visibility=Default Availability=Available
// [1] bool removeStaleLockFile()

/*
Attempts to forcefully remove an existing lock file.

Calling this is not recommended when protecting a short-lived operation: QLockFile already takes care of removing lock files after they are older than staleLockTime().

This method should only be called when protecting a resource for a long time, i.e. with staleLockTime(0), and after tryLock() returned LockFailedError, and the user agreed on removing the lock file.

Returns true on success, false if the lock file couldn't be removed. This happens on Windows, when the application owning the lock is still running.
*/
impl /*struct*/ QLockFile {
  pub fn removeStaleLockFile_0<RetType, T: QLockFile_removeStaleLockFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeStaleLockFile_0(self);
    // return 1;
  }
}
pub trait QLockFile_removeStaleLockFile_0<RetType> {
  fn removeStaleLockFile_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_removeStaleLockFile_0<bool> for () {
  fn removeStaleLockFile_0(self , rsthis: & QLockFile) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QLockFile19removeStaleLockFileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlockfile.h:73
// index:0
// Public Visibility=Default Availability=Available
// [4] QLockFile::LockError error() const

/*
Returns the lock file error status.

If tryLock() returns false, this function can be called to find out the reason why the locking failed.
*/
impl /*struct*/ QLockFile {
  pub fn error_0<RetType, T: QLockFile_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QLockFile_error_0<RetType> {
  fn error_0(self , rsthis: & QLockFile) -> RetType;
}
impl<'a> /*trait*/ QLockFile_error_0<i32> for () {
  fn error_0(self , rsthis: & QLockFile) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QLockFile5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*
This enum describes the result of the last call to lock() or tryLock().


*/
pub type QLockFile__LockError = i32;
// The lock was acquired successfully.
pub const QLockFile__NoError :QLockFile__LockError = 0;
// The lock could not be acquired because another process holds it.
pub const QLockFile__LockFailedError :QLockFile__LockError = 1;
// The lock file could not be created, for lack of permissions in the parent directory.
pub const QLockFile__PermissionError :QLockFile__LockError = 2;
// Another error happened, for instance a full partition prevented writing out the lock file.
pub const QLockFile__UnknownError :QLockFile__LockError = 3;
pub fn QLockFile_LockErrorItemName(val: i32) ->String {
  match val {
     QLockFile__NoError => // 0
     {return String::from("NoError");}
     QLockFile__LockFailedError => // 1
     {return String::from("LockFailedError");}
     QLockFile__PermissionError => // 2
     {return String::from("PermissionError");}
     QLockFile__UnknownError => // 3
     {return String::from("UnknownError");}
  _ => {return format!("{}", val);}
}
}
pub fn QLockFile_LockErrorItemName_s(val: i32) ->String {
  //var nilthis *QLockFile
  //return nilthis.LockErrorItemName(val);
  return QLockFile_LockErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
