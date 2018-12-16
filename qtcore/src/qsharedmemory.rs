

// mod ::core::QSharedMemory
// package qtcore
// /usr/include/qt/QtCore/qsharedmemory.h
// #include <qsharedmemory.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 43
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
#[derive(Default)] // class sizeof(QSharedMemory)=16
pub struct QSharedMemory {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSharedMemory_ITF interface {
//    QObject_ITF
//    QSharedMemory_PTR() *QSharedMemory
//}
//func (ptr *QSharedMemory) QSharedMemory_PTR() *QSharedMemory { return ptr }

impl /*struct*/ QSharedMemory {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSharedMemory {
    return QSharedMemory{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSharedMemory {
//  type Target = QSharedMemoryBASE;
//
//  fn deref(&self) -> &QSharedMemoryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSharedMemoryBASE> for QSharedMemory {
//  fn as_ref(& self) -> & QSharedMemoryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsharedmemory.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSharedMemory {
  pub fn metaObject_0<RetType, T: QSharedMemory_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSharedMemory(QObject *)

/*
Constructs a shared memory object with the given parent and with its key set to key. Because its key is set, its create() and attach() functions can be called.

See also setKey(), create(), and attach().
*/
// QSharedMemory(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSharedMemory {
  pub fn QSharedMemory_0<T: QSharedMemory_QSharedMemory_0>(value: T) -> QSharedMemory {
    let rsthis = value.QSharedMemory_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedMemory_QSharedMemory_0 {
  fn QSharedMemory_0(self) -> QSharedMemory;
}
// QSharedMemory(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSharedMemory_QSharedMemory_0 for (usize) {
  fn QSharedMemory_0(self) -> QSharedMemory {
    // unsafe{_ZN13QSharedMemoryC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QSharedMemoryC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSharedMemory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:78
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSharedMemory(const QString &, QObject *)

/*
Constructs a shared memory object with the given parent and with its key set to key. Because its key is set, its create() and attach() functions can be called.

See also setKey(), create(), and attach().
*/
// QSharedMemory(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSharedMemory {
  pub fn QSharedMemory_1<T: QSharedMemory_QSharedMemory_1>(value: T) -> QSharedMemory {
    let rsthis = value.QSharedMemory_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedMemory_QSharedMemory_1 {
  fn QSharedMemory_1(self) -> QSharedMemory;
}
// QSharedMemory(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSharedMemory_QSharedMemory_1 for (usize,usize) {
  fn QSharedMemory_1(self) -> QSharedMemory {
    // unsafe{_ZN13QSharedMemoryC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QSharedMemoryC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSharedMemory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:79
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSharedMemory()

/*

*/
pub fn DeleteQSharedMemory(this :*mut QSharedMemory) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QSharedMemoryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsharedmemory.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setKey(const QString &)

/*
Sets the platform independent key for this shared memory object. If key is the same as the current key, the function returns without doing anything.

You can call key() to retrieve the platform independent key. Internally, QSharedMemory converts this key into a platform specific key. If you instead call nativeKey(), you will get the platform specific, converted key.

If the shared memory object is attached to an underlying shared memory segment, it will detach from it before setting the new key. This function does not do an attach().

See also key(), nativeKey(), and isAttached().
*/
impl /*struct*/ QSharedMemory {
  pub fn setKey_0<RetType, T: QSharedMemory_setKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setKey_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_setKey_0<RetType> {
  fn setKey_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_setKey_0<(/*void*/)> for (usize) {
  fn setKey_0(self , rsthis: & QSharedMemory) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSharedMemory6setKeyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] QString key() const

/*
Returns the key assigned with setKey() to this shared memory, or a null key if no key has been assigned, or if the segment is using a nativeKey(). The key is the identifier used by Qt applications to identify the shared memory segment.

You can find the native, platform specific, key used by the operating system by calling nativeKey().

See also setKey() and setNativeKey().
*/
impl /*struct*/ QSharedMemory {
  pub fn key_0<RetType, T: QSharedMemory_key_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.key_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_key_0<RetType> {
  fn key_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_key_0<usize> for () {
  fn key_0(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory3keyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:83
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNativeKey(const QString &)

/*
Sets the native, platform specific, key for this shared memory object. If key is the same as the current native key, the function returns without doing anything. If all you want is to assign a key to a segment, you should call setKey() instead.

You can call nativeKey() to retrieve the native key. If a native key has been assigned, calling key() will return a null string.

If the shared memory object is attached to an underlying shared memory segment, it will detach from it before setting the new key. This function does not do an attach().

The application will not be portable if you set a native key.

This function was introduced in  Qt 4.8.

See also nativeKey(), key(), and isAttached().
*/
impl /*struct*/ QSharedMemory {
  pub fn setNativeKey_0<RetType, T: QSharedMemory_setNativeKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNativeKey_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_setNativeKey_0<RetType> {
  fn setNativeKey_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_setNativeKey_0<(/*void*/)> for (usize) {
  fn setNativeKey_0(self , rsthis: & QSharedMemory) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSharedMemory12setNativeKeyERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:84
// index:0
// Public Visibility=Default Availability=Available
// [8] QString nativeKey() const

/*
Returns the native, platform specific, key for this shared memory object. The native key is the identifier used by the operating system to identify the shared memory segment.

You can use the native key to access shared memory segments that have not been created by Qt, or to grant shared memory access to non-Qt applications.

This function was introduced in  Qt 4.8.

See also setKey() and setNativeKey().
*/
impl /*struct*/ QSharedMemory {
  pub fn nativeKey_0<RetType, T: QSharedMemory_nativeKey_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nativeKey_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_nativeKey_0<RetType> {
  fn nativeKey_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_nativeKey_0<usize> for () {
  fn nativeKey_0(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory9nativeKeyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:86
// index:0
// Public Visibility=Default Availability=Available
// [1] bool create(int, QSharedMemory::AccessMode)

/*
Creates a shared memory segment of size bytes with the key passed to the constructor, set with setKey() or set with setNativeKey(), then attaches to the new shared memory segment with the given access mode and returns true. If a shared memory segment identified by the key already exists, the attach operation is not performed and false is returned. When the return value is false, call error() to determine which error occurred.

See also error().
*/
impl /*struct*/ QSharedMemory {
  pub fn create_0<RetType, T: QSharedMemory_create_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.create_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_create_0<RetType> {
  fn create_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_create_0<bool> for (i32,i32) {
  fn create_0(self , rsthis: & QSharedMemory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSharedMemory6createEiNS_10AccessModeE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:87
// index:0
// Public Visibility=Default Availability=Available
// [4] int size() const

/*
Returns the size of the attached shared memory segment. If no shared memory segment is attached, 0 is returned.

Note: The size of the segment may be larger than the requested size that was passed to create().

See also create() and attach().
*/
impl /*struct*/ QSharedMemory {
  pub fn size_0<RetType, T: QSharedMemory_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_size_0<RetType> {
  fn size_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_size_0<i32> for () {
  fn size_0(self , rsthis: & QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:89
// index:0
// Public Visibility=Default Availability=Available
// [1] bool attach(QSharedMemory::AccessMode)

/*
Attempts to attach the process to the shared memory segment identified by the key that was passed to the constructor or to a call to setKey() or setNativeKey(). The access mode is ReadWrite by default. It can also be ReadOnly. Returns true if the attach operation is successful. If false is returned, call error() to determine which error occurred. After attaching the shared memory segment, a pointer to the shared memory can be obtained by calling data().

See also isAttached(), detach(), and create().
*/
impl /*struct*/ QSharedMemory {
  pub fn attach_0<RetType, T: QSharedMemory_attach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.attach_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_attach_0<RetType> {
  fn attach_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_attach_0<bool> for (i32) {
  fn attach_0(self , rsthis: & QSharedMemory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSharedMemory6attachENS_10AccessModeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:90
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAttached() const

/*
Returns true if this process is attached to the shared memory segment.

See also attach() and detach().
*/
impl /*struct*/ QSharedMemory {
  pub fn isAttached_0<RetType, T: QSharedMemory_isAttached_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAttached_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_isAttached_0<RetType> {
  fn isAttached_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_isAttached_0<bool> for () {
  fn isAttached_0(self , rsthis: & QSharedMemory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory10isAttachedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:91
// index:0
// Public Visibility=Default Availability=Available
// [1] bool detach()

/*
Detaches the process from the shared memory segment. If this was the last process attached to the shared memory segment, then the shared memory segment is released by the system, i.e., the contents are destroyed. The function returns true if it detaches the shared memory segment. If it returns false, it usually means the segment either isn't attached, or it is locked by another process.

See also attach() and isAttached().
*/
impl /*struct*/ QSharedMemory {
  pub fn detach_0<RetType, T: QSharedMemory_detach_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.detach_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_detach_0<RetType> {
  fn detach_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_detach_0<bool> for () {
  fn detach_0(self , rsthis: & QSharedMemory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSharedMemory6detachEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] void * data()

/*
Returns a pointer to the contents of the shared memory segment, if one is attached. Otherwise it returns null. Remember to lock the shared memory with lock() before reading from or writing to the shared memory, and remember to release the lock with unlock() after you are done.

See also attach().
*/
impl /*struct*/ QSharedMemory {
  pub fn data_0<RetType, T: QSharedMemory_data_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_data_0<RetType> {
  fn data_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_data_0<usize> for () {
  fn data_0(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSharedMemory4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:95
// index:1
// Public Visibility=Default Availability=Available
// [8] const void * data() const

/*
Returns a pointer to the contents of the shared memory segment, if one is attached. Otherwise it returns null. Remember to lock the shared memory with lock() before reading from or writing to the shared memory, and remember to release the lock with unlock() after you are done.

See also attach().
*/
impl /*struct*/ QSharedMemory {
  pub fn data_1<RetType, T: QSharedMemory_data_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.data_1(self);
    // return 1;
  }
}
pub trait QSharedMemory_data_1<RetType> {
  fn data_1(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_data_1<usize> for () {
  fn data_1(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory4dataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] const void * constData() const

/*
Returns a const pointer to the contents of the shared memory segment, if one is attached. Otherwise it returns null. Remember to lock the shared memory with lock() before reading from or writing to the shared memory, and remember to release the lock with unlock() after you are done.

See also attach() and create().
*/
impl /*struct*/ QSharedMemory {
  pub fn constData_0<RetType, T: QSharedMemory_constData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.constData_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_constData_0<RetType> {
  fn constData_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_constData_0<usize> for () {
  fn constData_0(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory9constDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool lock()

/*
This is a semaphore that locks the shared memory segment for access by this process and returns true. If another process has locked the segment, this function blocks until the lock is released. Then it acquires the lock and returns true. If this function returns false, it means that you have ignored a false return from create() or attach(), that you have set the key with setNativeKey() or that QSystemSemaphore::acquire() failed due to an unknown system error.

See also unlock(), data(), and QSystemSemaphore::acquire().
*/
impl /*struct*/ QSharedMemory {
  pub fn lock_0<RetType, T: QSharedMemory_lock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lock_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_lock_0<RetType> {
  fn lock_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_lock_0<bool> for () {
  fn lock_0(self , rsthis: & QSharedMemory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSharedMemory4lockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:99
// index:0
// Public Visibility=Default Availability=Available
// [1] bool unlock()

/*
Releases the lock on the shared memory segment and returns true, if the lock is currently held by this process. If the segment is not locked, or if the lock is held by another process, nothing happens and false is returned.

See also lock().
*/
impl /*struct*/ QSharedMemory {
  pub fn unlock_0<RetType, T: QSharedMemory_unlock_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unlock_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_unlock_0<RetType> {
  fn unlock_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_unlock_0<bool> for () {
  fn unlock_0(self , rsthis: & QSharedMemory) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QSharedMemory6unlockEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:102
// index:0
// Public Visibility=Default Availability=Available
// [4] QSharedMemory::SharedMemoryError error() const

/*
Returns a value indicating whether an error occurred, and, if so, which error it was.

See also errorString().
*/
impl /*struct*/ QSharedMemory {
  pub fn error_0<RetType, T: QSharedMemory_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_error_0<RetType> {
  fn error_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_error_0<i32> for () {
  fn error_0(self , rsthis: & QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsharedmemory.h:103
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a text description of the last error that occurred. If error() returns an error value, call this function to get a text string that describes the error.

See also error().
*/
impl /*struct*/ QSharedMemory {
  pub fn errorString_0<RetType, T: QSharedMemory_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QSharedMemory_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QSharedMemory) -> RetType;
}
impl<'a> /*trait*/ QSharedMemory_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QSharedMemory) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSharedMemory11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}


/*

*/
pub type QSharedMemory__AccessMode = i32;
// The shared memory segment is read-only. Writing to the shared memory segment is not allowed. An attempt to write to a shared memory segment created with ReadOnly causes the program to abort.
pub const QSharedMemory__ReadOnly :QSharedMemory__AccessMode = 0;
// Reading and writing the shared memory segment are both allowed.
pub const QSharedMemory__ReadWrite :QSharedMemory__AccessMode = 1;
pub fn QSharedMemory_AccessModeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSharedMemory", val);
}
pub fn QSharedMemory_AccessModeItemName_s(val: i32) ->String {
  //var nilthis *QSharedMemory
  //return nilthis.AccessModeItemName(val);
  return QSharedMemory_AccessModeItemName(val);
}


/*

*/
pub type QSharedMemory__SharedMemoryError = i32;
// No error occurred.
pub const QSharedMemory__NoError :QSharedMemory__SharedMemoryError = 0;
// The operation failed because the caller didn't have the required permissions.
pub const QSharedMemory__PermissionDenied :QSharedMemory__SharedMemoryError = 1;
// A create operation failed because the requested size was invalid.
pub const QSharedMemory__InvalidSize :QSharedMemory__SharedMemoryError = 2;
// The operation failed because of an invalid key.
pub const QSharedMemory__KeyError :QSharedMemory__SharedMemoryError = 3;
// A create() operation failed because a shared memory segment with the specified key already existed.
pub const QSharedMemory__AlreadyExists :QSharedMemory__SharedMemoryError = 4;
// An attach() failed because a shared memory segment with the specified key could not be found.
pub const QSharedMemory__NotFound :QSharedMemory__SharedMemoryError = 5;
// The attempt to lock() the shared memory segment failed because create() or attach() failed and returned false, or because a system error occurred in QSystemSemaphore::acquire().
pub const QSharedMemory__LockError :QSharedMemory__SharedMemoryError = 6;
// A create() operation failed because there was not enough memory available to fill the request.
pub const QSharedMemory__OutOfResources :QSharedMemory__SharedMemoryError = 7;
// Something else happened and it was bad.
pub const QSharedMemory__UnknownError :QSharedMemory__SharedMemoryError = 8;
pub fn QSharedMemory_SharedMemoryErrorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSharedMemory", val);
}
pub fn QSharedMemory_SharedMemoryErrorItemName_s(val: i32) ->String {
  //var nilthis *QSharedMemory
  //return nilthis.SharedMemoryErrorItemName(val);
  return QSharedMemory_SharedMemoryErrorItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
