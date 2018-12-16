

// mod ::core::QStorageInfo
// package qtcore
// /usr/include/qt/QtCore/qstorageinfo.h
// #include <qstorageinfo.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 31
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
#[derive(Default)] // class sizeof(QStorageInfo)=8
pub struct QStorageInfo {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QStorageInfo_ITF interface {
//    QStorageInfo_PTR() *QStorageInfo
//}
//func (ptr *QStorageInfo) QStorageInfo_PTR() *QStorageInfo { return ptr }

impl /*struct*/ QStorageInfo {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QStorageInfo {
    return QStorageInfo{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QStorageInfo {
//  type Target = QStorageInfoBASE;
//
//  fn deref(&self) -> &QStorageInfoBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QStorageInfoBASE> for QStorageInfo {
//  fn as_ref(& self) -> & QStorageInfoBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qstorageinfo.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QStorageInfo()

/*
Constructs an empty QStorageInfo object.

Objects created with the default constructor will be invalid and therefore not ready for use.

See also setPath(), isReady(), and isValid().
*/
// QStorageInfo() ctx.fn_proto_cpp
impl /*struct*/ QStorageInfo {
  pub fn QStorageInfo_0<T: QStorageInfo_QStorageInfo_0>(value: T) -> QStorageInfo {
    let rsthis = value.QStorageInfo_0();
    return rsthis;
    // return 1;
  }
}

pub trait QStorageInfo_QStorageInfo_0 {
  fn QStorageInfo_0(self) -> QStorageInfo;
}
// QStorageInfo() ctx.fn_proto_cpp
impl<'a> /*trait*/ QStorageInfo_QStorageInfo_0 for () {
  fn QStorageInfo_0(self) -> QStorageInfo {
    // unsafe{_ZN12QStorageInfoC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QStorageInfoC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStorageInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QStorageInfo(const QString &)

/*
Constructs an empty QStorageInfo object.

Objects created with the default constructor will be invalid and therefore not ready for use.

See also setPath(), isReady(), and isValid().
*/
// QStorageInfo(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QStorageInfo {
  pub fn QStorageInfo_1<T: QStorageInfo_QStorageInfo_1>(value: T) -> QStorageInfo {
    let rsthis = value.QStorageInfo_1();
    return rsthis;
    // return 1;
  }
}

pub trait QStorageInfo_QStorageInfo_1 {
  fn QStorageInfo_1(self) -> QStorageInfo;
}
// QStorageInfo(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStorageInfo_QStorageInfo_1 for (usize) {
  fn QStorageInfo_1(self) -> QStorageInfo {
    // unsafe{_ZN12QStorageInfoC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QStorageInfoC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStorageInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:60
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QStorageInfo(const QDir &)

/*
Constructs an empty QStorageInfo object.

Objects created with the default constructor will be invalid and therefore not ready for use.

See also setPath(), isReady(), and isValid().
*/
// QStorageInfo(const QDir &) ctx.fn_proto_cpp
impl /*struct*/ QStorageInfo {
  pub fn QStorageInfo_2<T: QStorageInfo_QStorageInfo_2>(value: T) -> QStorageInfo {
    let rsthis = value.QStorageInfo_2();
    return rsthis;
    // return 1;
  }
}

pub trait QStorageInfo_QStorageInfo_2 {
  fn QStorageInfo_2(self) -> QStorageInfo;
}
// QStorageInfo(const QDir &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QStorageInfo_QStorageInfo_2 for (usize) {
  fn QStorageInfo_2(self) -> QStorageInfo {
    // unsafe{_ZN12QStorageInfoC2ERK4QDir()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN12QStorageInfoC2ERK4QDir", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QStorageInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QStorageInfo()

/*

*/
pub fn DeleteQStorageInfo(this :*mut QStorageInfo) {
    // let rv = qtrt::InvokeQtFunc6("_ZN12QStorageInfoD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qstorageinfo.h:64
// index:0
// Public Visibility=Default Availability=Available
// [8] QStorageInfo & operator=(const QStorageInfo &)

/*

*/
impl /*struct*/ QStorageInfo {
  pub fn operator_equal_0<RetType, T: QStorageInfo_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QStorageInfoaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:66
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QStorageInfo & operator=(QStorageInfo &&)

/*

*/
impl /*struct*/ QStorageInfo {
  pub fn operator_equal_1<RetType, T: QStorageInfo_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QStorageInfo_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QStorageInfoaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:69
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QStorageInfo &)

/*
Swaps this volume info with other. This function is very fast and never fails.
*/
impl /*struct*/ QStorageInfo {
  pub fn swap_0<RetType, T: QStorageInfo_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_swap_0<RetType> {
  fn swap_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QStorageInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QStorageInfo4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:72
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPath(const QString &)

/*
Sets this QStorageInfo object to the filesystem mounted where path is located.

path can either be a root path of the filesystem, a directory, or a file within that filesystem.

See also rootPath().
*/
impl /*struct*/ QStorageInfo {
  pub fn setPath_0<RetType, T: QStorageInfo_setPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPath_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_setPath_0<RetType> {
  fn setPath_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_setPath_0<(/*void*/)> for (usize) {
  fn setPath_0(self , rsthis: & QStorageInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN12QStorageInfo7setPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QString rootPath() const

/*
Returns the mount point of the filesystem this QStorageInfo object represents.

On Windows, it returns the volume letter in case the volume is not mounted to a directory.

Note that the value returned by rootPath() is the real mount point of a volume, and may not be equal to the value passed to the constructor or setPath() method. For example, if you have only the root volume in the system, and pass '/directory' to setPath(), then this method will return '/'.

See also setPath() and device().
*/
impl /*struct*/ QStorageInfo {
  pub fn rootPath_0<RetType, T: QStorageInfo_rootPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rootPath_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_rootPath_0<RetType> {
  fn rootPath_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_rootPath_0<usize> for () {
  fn rootPath_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo8rootPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:75
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray device() const

/*
Returns the device for this volume.

For example, on Unix filesystems (including macOS), this returns the devpath like /dev/sda0 for local storages. On Windows, it returns the UNC path starting with \\\\?\\ for local storages (in other words, the volume GUID).

See also rootPath() and subvolume().
*/
impl /*struct*/ QStorageInfo {
  pub fn device_0<RetType, T: QStorageInfo_device_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.device_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_device_0<RetType> {
  fn device_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_device_0<usize> for () {
  fn device_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo6deviceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:76
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray subvolume() const

/*
Returns the subvolume name for this volume.

Some filesystem types allow multiple subvolumes inside one device, which may be mounted in different paths. If the subvolume could be detected, it is returned here. The format of the subvolume name is specific to each filesystem type.

If this volume was not mounted from a subvolume of a larger filesystem or if the subvolume could not be detected, this function returns an empty byte array.

This function was introduced in  Qt 5.9.

See also device().
*/
impl /*struct*/ QStorageInfo {
  pub fn subvolume_0<RetType, T: QStorageInfo_subvolume_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.subvolume_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_subvolume_0<RetType> {
  fn subvolume_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_subvolume_0<usize> for () {
  fn subvolume_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo9subvolumeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QByteArray fileSystemType() const

/*
Returns the type name of the filesystem.

This is a platform-dependent function, and filesystem names can vary between different operating systems. For example, on Windows filesystems they can be named NTFS, and on Linux they can be named ntfs-3g or fuseblk.

See also name().
*/
impl /*struct*/ QStorageInfo {
  pub fn fileSystemType_0<RetType, T: QStorageInfo_fileSystemType_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileSystemType_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_fileSystemType_0<RetType> {
  fn fileSystemType_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_fileSystemType_0<usize> for () {
  fn fileSystemType_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo14fileSystemTypeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:78
// index:0
// Public Visibility=Default Availability=Available
// [8] QString name() const

/*
Returns the human-readable name of a filesystem, usually called label.

Not all filesystems support this feature. In this case, the value returned by this method could be empty. An empty string is returned if the file system does not support labels, or if no label is set.

On Linux, retrieving the volume's label requires udev to be present in the system.

See also fileSystemType().
*/
impl /*struct*/ QStorageInfo {
  pub fn name_0<RetType, T: QStorageInfo_name_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.name_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_name_0<RetType> {
  fn name_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_name_0<usize> for () {
  fn name_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo4nameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QString displayName() const

/*
Returns the volume's name, if available, or the root path if not.
*/
impl /*struct*/ QStorageInfo {
  pub fn displayName_0<RetType, T: QStorageInfo_displayName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.displayName_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_displayName_0<RetType> {
  fn displayName_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_displayName_0<usize> for () {
  fn displayName_0(self , rsthis: & QStorageInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo11displayNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:81
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 bytesTotal() const

/*
Returns the total volume size in bytes.

Returns -1 if QStorageInfo object is not valid.

See also bytesFree() and bytesAvailable().
*/
impl /*struct*/ QStorageInfo {
  pub fn bytesTotal_0<RetType, T: QStorageInfo_bytesTotal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesTotal_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_bytesTotal_0<RetType> {
  fn bytesTotal_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_bytesTotal_0<i64> for () {
  fn bytesTotal_0(self , rsthis: & QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo10bytesTotalEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:82
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 bytesFree() const

/*
Returns the number of free bytes in a volume. Note that if there are quotas on the filesystem, this value can be larger than the value returned by bytesAvailable().

Returns -1 if QStorageInfo object is not valid.

See also bytesTotal() and bytesAvailable().
*/
impl /*struct*/ QStorageInfo {
  pub fn bytesFree_0<RetType, T: QStorageInfo_bytesFree_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesFree_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_bytesFree_0<RetType> {
  fn bytesFree_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_bytesFree_0<i64> for () {
  fn bytesFree_0(self , rsthis: & QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo9bytesFreeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:83
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 bytesAvailable() const

/*
Returns the size (in bytes) available for the current user. It returns the total size available if the user is the root user or a system administrator.

This size can be less than or equal to the free size returned by bytesFree() function.

Returns -1 if QStorageInfo object is not valid.

See also bytesTotal() and bytesFree().
*/
impl /*struct*/ QStorageInfo {
  pub fn bytesAvailable_0<RetType, T: QStorageInfo_bytesAvailable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bytesAvailable_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_bytesAvailable_0<RetType> {
  fn bytesAvailable_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_bytesAvailable_0<i64> for () {
  fn bytesAvailable_0(self , rsthis: & QStorageInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo14bytesAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:84
// index:0
// Public Visibility=Default Availability=Available
// [4] int blockSize() const

/*
Returns the optimal transfer block size for this filesystem.

Returns -1 if QStorageInfo could not determine the size or if the QStorageInfo object is not valid.

This function was introduced in  Qt 5.6.
*/
impl /*struct*/ QStorageInfo {
  pub fn blockSize_0<RetType, T: QStorageInfo_blockSize_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.blockSize_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_blockSize_0<RetType> {
  fn blockSize_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_blockSize_0<i32> for () {
  fn blockSize_0(self , rsthis: & QStorageInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo9blockSizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:86
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isRoot() const

/*
Returns true if this QStorageInfo represents the system root volume; false otherwise.

On Unix filesystems, the root volume is a volume mounted on /. On Windows, the root volume is the volume where the OS is installed.

See also root().
*/
impl /*struct*/ QStorageInfo {
  pub fn isRoot_0<RetType, T: QStorageInfo_isRoot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRoot_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_isRoot_0<RetType> {
  fn isRoot_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_isRoot_0<bool> for () {
  fn isRoot_0(self , rsthis: & QStorageInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo6isRootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:87
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadOnly() const

/*
Returns true if the current filesystem is protected from writing; false otherwise.
*/
impl /*struct*/ QStorageInfo {
  pub fn isReadOnly_0<RetType, T: QStorageInfo_isReadOnly_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_isReadOnly_0<RetType> {
  fn isReadOnly_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_isReadOnly_0<bool> for () {
  fn isReadOnly_0(self , rsthis: & QStorageInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo10isReadOnlyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:88
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReady() const

/*
Returns true if the current filesystem is ready to work; false otherwise. For example, false is returned if the CD volume is not inserted.

Note that fileSystemType(), name(), bytesTotal(), bytesFree(), and bytesAvailable() will return invalid data until the volume is ready.

See also isValid().
*/
impl /*struct*/ QStorageInfo {
  pub fn isReady_0<RetType, T: QStorageInfo_isReady_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReady_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_isReady_0<RetType> {
  fn isReady_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_isReady_0<bool> for () {
  fn isReady_0(self , rsthis: & QStorageInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo7isReadyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:89
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the QStorageInfo specified by rootPath exists and is mounted correctly.

See also isReady().
*/
impl /*struct*/ QStorageInfo {
  pub fn isValid_0<RetType, T: QStorageInfo_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QStorageInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK12QStorageInfo7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:91
// index:0
// Public Visibility=Default Availability=Available
// [-2] void refresh()

/*
Resets QStorageInfo's internal cache.

QStorageInfo caches information about storage to speed up performance. QStorageInfo retrieves information during object construction and/or when calling the setPath() method. You have to manually reset the cache by calling this function to update storage information.
*/
impl /*struct*/ QStorageInfo {
  pub fn refresh_0<RetType, T: QStorageInfo_refresh_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refresh_0(self);
    // return 1;
  }
}
pub trait QStorageInfo_refresh_0<RetType> {
  fn refresh_0(self , rsthis: & QStorageInfo) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_refresh_0<(/*void*/)> for () {
  fn refresh_0(self , rsthis: & QStorageInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN12QStorageInfo7refreshEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qstorageinfo.h:94
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStorageInfo root()

/*
Returns a QStorageInfo object that represents the system root volume.

On Unix systems this call returns the root ('/') volume; in Windows the volume where the operating system is installed.

See also isRoot().
*/
impl /*struct*/ QStorageInfo {
  pub fn root_0<RetType, T: QStorageInfo_root_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.root_0();
    // return 1;
  }
}
pub trait QStorageInfo_root_0<RetType> {
  fn root_0(self ) -> RetType;
}
impl<'a> /*trait*/ QStorageInfo_root_0<usize> for () {
  fn root_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN12QStorageInfo4rootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
