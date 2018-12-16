

// mod ::core::QFileDevice
// package qtcore
// /usr/include/qt/QtCore/qfiledevice.h
// #include <qfiledevice.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 8
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

// long long readData(char *, qint64)
// func (this *QFileDevice) InheritReadData(f func(data string, maxlen int64) int64) {
//  qtrt.SetAllInheritCallback(this, "readData", f)
// }

// long long writeData(const char *, qint64)
// func (this *QFileDevice) InheritWriteData(f func(data string, len_ int64) int64) {
//  qtrt.SetAllInheritCallback(this, "writeData", f)
// }

// long long readLineData(char *, qint64)
// func (this *QFileDevice) InheritReadLineData(f func(data string, maxlen int64) int64) {
//  qtrt.SetAllInheritCallback(this, "readLineData", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QFileDevice)=16
pub struct QFileDevice {
  qbase: QIODevice,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileDevice_ITF interface {
//    QIODevice_ITF
//    QFileDevice_PTR() *QFileDevice
//}
//func (ptr *QFileDevice) QFileDevice_PTR() *QFileDevice { return ptr }

impl /*struct*/ QFileDevice {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileDevice {
    return QFileDevice{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileDevice {
//  type Target = QFileDeviceBASE;
//
//  fn deref(&self) -> &QFileDeviceBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileDeviceBASE> for QFileDevice {
//  fn as_ref(& self) -> & QFileDeviceBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfiledevice.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFileDevice {
  pub fn metaObject_0<RetType, T: QFileDevice_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFileDevice_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFileDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:98
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileDevice()

/*

*/
pub fn DeleteQFileDevice(this :*mut QFileDevice) {
    // let rv = qtrt::InvokeQtFunc6("_ZN11QFileDeviceD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfiledevice.h:100
// index:0
// Public Visibility=Default Availability=Available
// [4] QFileDevice::FileError error() const

/*
Returns the file error status.

The I/O device status returns an error code. For example, if open() returns false, or a read/write operation returns -1, this function can be called to find out the reason why the operation failed.

See also unsetError().
*/
impl /*struct*/ QFileDevice {
  pub fn error_0<RetType, T: QFileDevice_error_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.error_0(self);
    // return 1;
  }
}
pub trait QFileDevice_error_0<RetType> {
  fn error_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_error_0<i32> for () {
  fn error_0(self , rsthis: & QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice5errorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void unsetError()

/*
Sets the file's error to QFileDevice::NoError.

See also error().
*/
impl /*struct*/ QFileDevice {
  pub fn unsetError_0<RetType, T: QFileDevice_unsetError_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unsetError_0(self);
    // return 1;
  }
}
pub trait QFileDevice_unsetError_0<RetType> {
  fn unsetError_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_unsetError_0<(/*void*/)> for () {
  fn unsetError_0(self , rsthis: & QFileDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QFileDevice10unsetErrorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:103
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void close()

/*
Reimplemented from QIODevice::close().

Calls QFileDevice::flush() and closes the file. Errors from flush are ignored.

See also QIODevice::close().
*/
impl /*struct*/ QFileDevice {
  pub fn close_0<RetType, T: QFileDevice_close_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.close_0(self);
    // return 1;
  }
}
pub trait QFileDevice_close_0<RetType> {
  fn close_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_close_0<(/*void*/)> for () {
  fn close_0(self , rsthis: & QFileDevice) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN11QFileDevice5closeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:105
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool isSequential() const

/*
Reimplemented from QIODevice::isSequential().

Returns true if the file can only be manipulated sequentially; otherwise returns false.

Most files support random-access, but some special files may not.

See also QIODevice::isSequential().
*/
impl /*struct*/ QFileDevice {
  pub fn isSequential_0<RetType, T: QFileDevice_isSequential_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSequential_0(self);
    // return 1;
  }
}
pub trait QFileDevice_isSequential_0<RetType> {
  fn isSequential_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_isSequential_0<bool> for () {
  fn isSequential_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice12isSequentialEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:107
// index:0
// Public Visibility=Default Availability=Available
// [4] int handle() const

/*
Returns the file handle of the file.

This is a small positive integer, suitable for use with C library functions such as fdopen() and fcntl(). On systems that use file descriptors for sockets (i.e. Unix systems, but not Windows) the handle can be used with QSocketNotifier as well.

If the file is not open, or there is an error, handle() returns -1.

See also QSocketNotifier.
*/
impl /*struct*/ QFileDevice {
  pub fn handle_0<RetType, T: QFileDevice_handle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.handle_0(self);
    // return 1;
  }
}
pub trait QFileDevice_handle_0<RetType> {
  fn handle_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_handle_0<i32> for () {
  fn handle_0(self , rsthis: & QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice6handleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:108
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Returns the name of the file. The default implementation in QFileDevice returns a null string.
*/
impl /*struct*/ QFileDevice {
  pub fn fileName_0<RetType, T: QFileDevice_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QFileDevice_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QFileDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:110
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 pos() const

/*
Reimplemented from QIODevice::pos().
*/
impl /*struct*/ QFileDevice {
  pub fn pos_0<RetType, T: QFileDevice_pos_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.pos_0(self);
    // return 1;
  }
}
pub trait QFileDevice_pos_0<RetType> {
  fn pos_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_pos_0<i64> for () {
  fn pos_0(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice3posEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:111
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool seek(qint64)

/*
Reimplemented from QIODevice::seek().

For random-access devices, this function sets the current position to pos, returning true on success, or false if an error occurred. For sequential devices, the default behavior is to do nothing and return false.

Seeking beyond the end of a file: If the position is beyond the end of a file, then seek() will not immediately extend the file. If a write is performed at this position, then the file will be extended. The content of the file between the previous end of file and the newly written data is UNDEFINED and varies between platforms and file systems.
*/
impl /*struct*/ QFileDevice {
  pub fn seek_0<RetType, T: QFileDevice_seek_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.seek_0(self);
    // return 1;
  }
}
pub trait QFileDevice_seek_0<RetType> {
  fn seek_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_seek_0<bool> for (i64) {
  fn seek_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice4seekEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:112
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool atEnd() const

/*
Reimplemented from QIODevice::atEnd().

Returns true if the end of the file has been reached; otherwise returns false.

For regular empty files on Unix (e.g. those in /proc), this function returns true, since the file system reports that the size of such a file is 0. Therefore, you should not depend on atEnd() when reading data from such a file, but rather call read() until no more data can be read.
*/
impl /*struct*/ QFileDevice {
  pub fn atEnd_0<RetType, T: QFileDevice_atEnd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.atEnd_0(self);
    // return 1;
  }
}
pub trait QFileDevice_atEnd_0<RetType> {
  fn atEnd_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_atEnd_0<bool> for () {
  fn atEnd_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice5atEndEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool flush()

/*
Flushes any buffered data to the file. Returns true if successful; otherwise returns false.
*/
impl /*struct*/ QFileDevice {
  pub fn flush_0<RetType, T: QFileDevice_flush_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.flush_0(self);
    // return 1;
  }
}
pub trait QFileDevice_flush_0<RetType> {
  fn flush_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_flush_0<bool> for () {
  fn flush_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice5flushEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:115
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] qint64 size() const

/*
Reimplemented from QIODevice::size().

Returns the size of the file.

For regular empty files on Unix (e.g. those in /proc), this function returns 0; the contents of such a file are generated on demand in response to you calling read().
*/
impl /*struct*/ QFileDevice {
  pub fn size_0<RetType, T: QFileDevice_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QFileDevice_size_0<RetType> {
  fn size_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_size_0<i64> for () {
  fn size_0(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:117
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool resize(qint64)

/*
Sets the file size (in bytes) sz. Returns true if the resize succeeds; false otherwise. If sz is larger than the file currently is, the new bytes will be set to 0; if sz is smaller, the file is simply truncated.

Warning: This function can fail if the file doesn't exist.

See also size().
*/
impl /*struct*/ QFileDevice {
  pub fn resize_0<RetType, T: QFileDevice_resize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resize_0(self);
    // return 1;
  }
}
pub trait QFileDevice_resize_0<RetType> {
  fn resize_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_resize_0<bool> for (i64) {
  fn resize_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice6resizeEx", 1,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:118
// index:0
// Public virtual Visibility=Default Availability=Available
// [4] QFileDevice::Permissions permissions() const

/*
Returns the complete OR-ed together combination of QFile::Permission for the file.

See also setPermissions().
*/
impl /*struct*/ QFileDevice {
  pub fn permissions_0<RetType, T: QFileDevice_permissions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.permissions_0(self);
    // return 1;
  }
}
pub trait QFileDevice_permissions_0<RetType> {
  fn permissions_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_permissions_0<i32> for () {
  fn permissions_0(self , rsthis: & QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice11permissionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:119
// index:0
// Public virtual Visibility=Default Availability=Available
// [1] bool setPermissions(QFileDevice::Permissions)

/*
Sets the permissions for the file to the permissions specified. Returns true if successful, or false if the permissions cannot be modified.

Warning: This function does not manipulate ACLs, which may limit its effectiveness.

See also permissions().
*/
impl /*struct*/ QFileDevice {
  pub fn setPermissions_0<RetType, T: QFileDevice_setPermissions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPermissions_0(self);
    // return 1;
  }
}
pub trait QFileDevice_setPermissions_0<RetType> {
  fn setPermissions_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_setPermissions_0<bool> for (i32) {
  fn setPermissions_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice14setPermissionsE6QFlagsINS_10PermissionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] uchar * map(qint64, qint64, QFileDevice::MemoryMapFlags)

/*
Maps size bytes of the file into memory starting at offset. A file should be open for a map to succeed but the file does not need to stay open after the memory has been mapped. When the QFile is destroyed or a new file is opened with this object, any maps that have not been unmapped will automatically be unmapped.

The mapping will have the same open mode as the file (read and/or write), except when using MapPrivateOption, in which case it is always possible to write to the mapped memory.

Any mapping options can be passed through flags.

Returns a pointer to the memory or 0 if there is an error.

See also unmap().
*/
impl /*struct*/ QFileDevice {
  pub fn map__0<RetType, T: QFileDevice_map__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__0(self);
    // return 1;
  }
}
pub trait QFileDevice_map__0<RetType> {
  fn map__0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_map__0<usize> for (i64,i64,i32) {
  fn map__0(self , rsthis: & QFileDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i64 as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice3mapExxNS_14MemoryMapFlagsE", 3,qtrt::FFITY_SINT64,qtrt::FFITY_SINT64,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:128
// index:0
// Public Visibility=Default Availability=Available
// [1] bool unmap(uchar *)

/*
Unmaps the memory address.

Returns true if the unmap succeeds; false otherwise.

See also map().
*/
impl /*struct*/ QFileDevice {
  pub fn unmap_0<RetType, T: QFileDevice_unmap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unmap_0(self);
    // return 1;
  }
}
pub trait QFileDevice_unmap_0<RetType> {
  fn unmap_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_unmap_0<bool> for (usize) {
  fn unmap_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice5unmapEPh", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime fileTime(QFileDevice::FileTime) const

/*
Returns the file time specified by time. If the time cannot be determined return QDateTime() (an invalid date time).

This function was introduced in  Qt 5.10.

See also setFileTime(), FileTime, and QDateTime::isValid().
*/
impl /*struct*/ QFileDevice {
  pub fn fileTime_0<RetType, T: QFileDevice_fileTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileTime_0(self);
    // return 1;
  }
}
pub trait QFileDevice_fileTime_0<RetType> {
  fn fileTime_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_fileTime_0<usize> for (i32) {
  fn fileTime_0(self , rsthis: & QFileDevice) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK11QFileDevice8fileTimeENS_8FileTimeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:131
// index:0
// Public Visibility=Default Availability=Available
// [1] bool setFileTime(const QDateTime &, QFileDevice::FileTime)

/*
Sets the file time specified by fileTime to newDate, returning true if successful; otherwise returns false.

Note: The file must be open to use this function.

This function was introduced in  Qt 5.10.

See also fileTime() and FileTime.
*/
impl /*struct*/ QFileDevice {
  pub fn setFileTime_0<RetType, T: QFileDevice_setFileTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileTime_0(self);
    // return 1;
  }
}
pub trait QFileDevice_setFileTime_0<RetType> {
  fn setFileTime_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_setFileTime_0<bool> for (usize,i32) {
  fn setFileTime_0(self , rsthis: & QFileDevice) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice11setFileTimeERK9QDateTimeNS_8FileTimeE", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:134
// index:0
// Protected Visibility=Default Availability=Available
// [-2] void QFileDevice()

/*

*/
// QFileDevice() ctx.fn_proto_cpp
impl /*struct*/ QFileDevice {
  pub fn QFileDevice_0<T: QFileDevice_QFileDevice_0>(value: T) -> QFileDevice {
    let rsthis = value.QFileDevice_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDevice_QFileDevice_0 {
  fn QFileDevice_0(self) -> QFileDevice;
}
// QFileDevice() ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileDevice_QFileDevice_0 for () {
  fn QFileDevice_0(self) -> QFileDevice {
    // unsafe{_ZN11QFileDeviceC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFileDeviceC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:138
// index:1
// Protected Visibility=Default Availability=Available
// [-2] void QFileDevice(QObject *)

/*

*/
// QFileDevice(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFileDevice {
  pub fn QFileDevice_1<T: QFileDevice_QFileDevice_1>(value: T) -> QFileDevice {
    let rsthis = value.QFileDevice_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDevice_QFileDevice_1 {
  fn QFileDevice_1(self) -> QFileDevice;
}
// QFileDevice(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileDevice_QFileDevice_1 for (usize) {
  fn QFileDevice_1(self) -> QFileDevice {
    // unsafe{_ZN11QFileDeviceC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN11QFileDeviceC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileDevice{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:142
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 readData(char *, qint64)

/*
Reimplemented from QIODevice::readData().
*/
impl /*struct*/ QFileDevice {
  pub fn readData_0<RetType, T: QFileDevice_readData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readData_0(self);
    // return 1;
  }
}
pub trait QFileDevice_readData_0<RetType> {
  fn readData_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_readData_0<i64> for (usize,i64) {
  fn readData_0(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice8readDataEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:143
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 writeData(const char *, qint64)

/*
Reimplemented from QIODevice::writeData().
*/
impl /*struct*/ QFileDevice {
  pub fn writeData_0<RetType, T: QFileDevice_writeData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.writeData_0(self);
    // return 1;
  }
}
pub trait QFileDevice_writeData_0<RetType> {
  fn writeData_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_writeData_0<i64> for (usize,i64) {
  fn writeData_0(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice9writeDataEPKcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfiledevice.h:144
// index:0
// Protected virtual Visibility=Default Availability=Available
// [8] qint64 readLineData(char *, qint64)

/*
Reimplemented from QIODevice::readLineData().
*/
impl /*struct*/ QFileDevice {
  pub fn readLineData_0<RetType, T: QFileDevice_readLineData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLineData_0(self);
    // return 1;
  }
}
pub trait QFileDevice_readLineData_0<RetType> {
  fn readLineData_0(self , rsthis: & QFileDevice) -> RetType;
}
impl<'a> /*trait*/ QFileDevice_readLineData_0<i64> for (usize,i64) {
  fn readLineData_0(self , rsthis: & QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self.0) as *const usize as usize;
    let arg1 = (&self.1) as *const i64 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN11QFileDevice12readLineDataEPcx", 2,qtrt::FFITY_POINTER,qtrt::FFITY_SINT64,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}


/*
This enum describes the errors that may be returned by the error() function.


*/
pub type QFileDevice__FileError = i32;
// No error occurred.
pub const QFileDevice__NoError :QFileDevice__FileError = 0;
// An error occurred when reading from the file.
pub const QFileDevice__ReadError :QFileDevice__FileError = 1;
// An error occurred when writing to the file.
pub const QFileDevice__WriteError :QFileDevice__FileError = 2;
// A fatal error occurred.
pub const QFileDevice__FatalError :QFileDevice__FileError = 3;
// Out of resources (e.g., too many open files, out of memory, etc.)
pub const QFileDevice__ResourceError :QFileDevice__FileError = 4;
// The file could not be opened.
pub const QFileDevice__OpenError :QFileDevice__FileError = 5;
// The operation was aborted.
pub const QFileDevice__AbortError :QFileDevice__FileError = 6;
// A timeout occurred.
pub const QFileDevice__TimeOutError :QFileDevice__FileError = 7;
// An unspecified error occurred.
pub const QFileDevice__UnspecifiedError :QFileDevice__FileError = 8;
// The file could not be removed.
pub const QFileDevice__RemoveError :QFileDevice__FileError = 9;
// 
pub const QFileDevice__RenameError :QFileDevice__FileError = 10;
// 
pub const QFileDevice__PositionError :QFileDevice__FileError = 11;
// 
pub const QFileDevice__ResizeError :QFileDevice__FileError = 12;
// 
pub const QFileDevice__PermissionsError :QFileDevice__FileError = 13;
// 
pub const QFileDevice__CopyError :QFileDevice__FileError = 14;
pub fn QFileDevice_FileErrorItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDevice", val);
}
pub fn QFileDevice_FileErrorItemName_s(val: i32) ->String {
  //var nilthis *QFileDevice
  //return nilthis.FileErrorItemName(val);
  return QFileDevice_FileErrorItemName(val);
}


/*
This enum is used by the fileTime() and setFileTime() functions.



This enum was introduced or modified in  Qt 5.10.

See also setFileTime(), fileTime(), and QFileInfo::fileTime().

*/
pub type QFileDevice__FileTime = i32;
// When the file was most recently accessed (e.g. read or written to).
pub const QFileDevice__FileAccessTime :QFileDevice__FileTime = 0;
// When the file was created (may not be not supported on UNIX).
pub const QFileDevice__FileBirthTime :QFileDevice__FileTime = 1;
// When the file's metadata was last changed.
pub const QFileDevice__FileMetadataChangeTime :QFileDevice__FileTime = 2;
// When the file was most recently modified.
pub const QFileDevice__FileModificationTime :QFileDevice__FileTime = 3;
pub fn QFileDevice_FileTimeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDevice", val);
}
pub fn QFileDevice_FileTimeItemName_s(val: i32) ->String {
  //var nilthis *QFileDevice
  //return nilthis.FileTimeItemName(val);
  return QFileDevice_FileTimeItemName(val);
}


/*


*/
pub type QFileDevice__Permission = i32;
// 
pub const QFileDevice__ReadOwner :QFileDevice__Permission = 16384;
// 
pub const QFileDevice__WriteOwner :QFileDevice__Permission = 8192;
// 
pub const QFileDevice__ExeOwner :QFileDevice__Permission = 4096;
// 
pub const QFileDevice__ReadUser :QFileDevice__Permission = 1024;
// 
pub const QFileDevice__WriteUser :QFileDevice__Permission = 512;
// 
pub const QFileDevice__ExeUser :QFileDevice__Permission = 256;
// 
pub const QFileDevice__ReadGroup :QFileDevice__Permission = 64;
// 
pub const QFileDevice__WriteGroup :QFileDevice__Permission = 32;
// 
pub const QFileDevice__ExeGroup :QFileDevice__Permission = 16;
// 
pub const QFileDevice__ReadOther :QFileDevice__Permission = 4;
// 
pub const QFileDevice__WriteOther :QFileDevice__Permission = 2;
// 
pub const QFileDevice__ExeOther :QFileDevice__Permission = 1;
pub fn QFileDevice_PermissionItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDevice", val);
}
pub fn QFileDevice_PermissionItemName_s(val: i32) ->String {
  //var nilthis *QFileDevice
  //return nilthis.PermissionItemName(val);
  return QFileDevice_PermissionItemName(val);
}


/*


*/
pub type QFileDevice__FileHandleFlag = i32;
// 
pub const QFileDevice__AutoCloseHandle :QFileDevice__FileHandleFlag = 1;
// 
pub const QFileDevice__DontCloseHandle :QFileDevice__FileHandleFlag = 0;
pub fn QFileDevice_FileHandleFlagItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDevice", val);
}
pub fn QFileDevice_FileHandleFlagItemName_s(val: i32) ->String {
  //var nilthis *QFileDevice
  //return nilthis.FileHandleFlagItemName(val);
  return QFileDevice_FileHandleFlagItemName(val);
}


/*
This enum describes special options that may be used by the map() function.



This enum was introduced or modified in  Qt 4.4.

*/
pub type QFileDevice__MemoryMapFlags = i32;
// No options.
pub const QFileDevice__NoOptions :QFileDevice__MemoryMapFlags = 0;
// 
pub const QFileDevice__MapPrivateOption :QFileDevice__MemoryMapFlags = 1;
pub fn QFileDevice_MemoryMapFlagsItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QFileDevice", val);
}
pub fn QFileDevice_MemoryMapFlagsItemName_s(val: i32) ->String {
  //var nilthis *QFileDevice
  //return nilthis.MemoryMapFlagsItemName(val);
  return QFileDevice_MemoryMapFlagsItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
