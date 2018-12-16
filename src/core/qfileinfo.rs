

// mod ::core::QFileInfo
// package qtcore
// /usr/include/qt/QtCore/qfileinfo.h
// #include <qfileinfo.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 34
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
#[derive(Default)] // class sizeof(QFileInfo)=8
pub struct QFileInfo {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileInfo_ITF interface {
//    QFileInfo_PTR() *QFileInfo
//}
//func (ptr *QFileInfo) QFileInfo_PTR() *QFileInfo { return ptr }

impl /*struct*/ QFileInfo {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileInfo {
    return QFileInfo{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileInfo {
//  type Target = QFileInfoBASE;
//
//  fn deref(&self) -> &QFileInfoBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileInfoBASE> for QFileInfo {
//  fn as_ref(& self) -> & QFileInfoBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfileinfo.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileInfo()

/*
Constructs an empty QFileInfo object.

Note that an empty QFileInfo object contain no file reference.

See also setFile().
*/
// QFileInfo() ctx.fn_proto_cpp
impl /*struct*/ QFileInfo {
  pub fn QFileInfo_0<T: QFileInfo_QFileInfo_0>(value: T) -> QFileInfo {
    let rsthis = value.QFileInfo_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_QFileInfo_0 {
  fn QFileInfo_0(self) -> QFileInfo;
}
// QFileInfo() ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileInfo_QFileInfo_0 for () {
  fn QFileInfo_0(self) -> QFileInfo {
    // unsafe{_ZN9QFileInfoC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QFileInfoC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:63
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFileInfo(const QString &)

/*
Constructs an empty QFileInfo object.

Note that an empty QFileInfo object contain no file reference.

See also setFile().
*/
// QFileInfo(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QFileInfo {
  pub fn QFileInfo_1<T: QFileInfo_QFileInfo_1>(value: T) -> QFileInfo {
    let rsthis = value.QFileInfo_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_QFileInfo_1 {
  fn QFileInfo_1(self) -> QFileInfo;
}
// QFileInfo(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileInfo_QFileInfo_1 for (usize) {
  fn QFileInfo_1(self) -> QFileInfo {
    // unsafe{_ZN9QFileInfoC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QFileInfoC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:64
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QFileInfo(const QFile &)

/*
Constructs an empty QFileInfo object.

Note that an empty QFileInfo object contain no file reference.

See also setFile().
*/
// QFileInfo(const QFile &) ctx.fn_proto_cpp
impl /*struct*/ QFileInfo {
  pub fn QFileInfo_2<T: QFileInfo_QFileInfo_2>(value: T) -> QFileInfo {
    let rsthis = value.QFileInfo_2();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_QFileInfo_2 {
  fn QFileInfo_2(self) -> QFileInfo;
}
// QFileInfo(const QFile &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileInfo_QFileInfo_2 for (usize) {
  fn QFileInfo_2(self) -> QFileInfo {
    // unsafe{_ZN9QFileInfoC2ERK5QFile()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QFileInfoC2ERK5QFile", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:65
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QFileInfo(const QDir &, const QString &)

/*
Constructs an empty QFileInfo object.

Note that an empty QFileInfo object contain no file reference.

See also setFile().
*/
// QFileInfo(const QDir &, const QString &) ctx.fn_proto_cpp
impl /*struct*/ QFileInfo {
  pub fn QFileInfo_3<T: QFileInfo_QFileInfo_3>(value: T) -> QFileInfo {
    let rsthis = value.QFileInfo_3();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_QFileInfo_3 {
  fn QFileInfo_3(self) -> QFileInfo;
}
// QFileInfo(const QDir &, const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileInfo_QFileInfo_3 for (usize,usize) {
  fn QFileInfo_3(self) -> QFileInfo {
    // unsafe{_ZN9QFileInfoC2ERK4QDirRK7QString()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QFileInfoC2ERK4QDirRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileInfo{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:67
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QFileInfo()

/*

*/
pub fn DeleteQFileInfo(this :*mut QFileInfo) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QFileInfoD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfileinfo.h:69
// index:0
// Public Visibility=Default Availability=Available
// [8] QFileInfo & operator=(const QFileInfo &)

/*

*/
impl /*struct*/ QFileInfo {
  pub fn operator_equal_0<RetType, T: QFileInfo_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QFileInfo_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QFileInfoaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:71
// index:1
// Public inline Visibility=Default Availability=Available
// [8] QFileInfo & operator=(QFileInfo &&)

/*

*/
impl /*struct*/ QFileInfo {
  pub fn operator_equal_1<RetType, T: QFileInfo_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QFileInfo_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QFileInfoaSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:74
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QFileInfo &)

/*
Swaps this file info with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QFileInfo {
  pub fn swap_0<RetType, T: QFileInfo_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QFileInfo_swap_0<RetType> {
  fn swap_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QFileInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QFileInfo4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:77
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QFileInfo &) const

/*

*/
impl /*struct*/ QFileInfo {
  pub fn operator_equal_equal_0<RetType, T: QFileInfo_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QFileInfo_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfoeqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:78
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QFileInfo &) const

/*

*/
impl /*struct*/ QFileInfo {
  pub fn operator_not_equal_0<RetType, T: QFileInfo_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QFileInfo_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfoneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:80
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFile(const QString &)

/*
Sets the file that the QFileInfo provides information about to file.

The file can also include an absolute or relative file path. Absolute paths begin with the directory separator (e.g. "/" under Unix) or a drive specification (under Windows). Relative file names begin with a directory name or a file name and specify a path relative to the current directory.

Example:


  QString absolute = "/local/bin";
  QString relative = "local/bin";
  QFileInfo absFile(absolute);
  QFileInfo relFile(relative);

  QDir::setCurrent(QDir::rootPath());
  // absFile and relFile now point to the same file

  QDir::setCurrent("/tmp");
  // absFile now points to "/local/bin",
  // while relFile points to "/tmp/local/bin"



See also isFile(), isRelative(), QDir::setCurrent(), and QDir::isRelativePath().
*/
impl /*struct*/ QFileInfo {
  pub fn setFile_0<RetType, T: QFileInfo_setFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFile_0(self);
    // return 1;
  }
}
pub trait QFileInfo_setFile_0<RetType> {
  fn setFile_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_setFile_0<(/*void*/)> for (usize) {
  fn setFile_0(self , rsthis: & QFileInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QFileInfo7setFileERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:81
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFile(const QFile &)

/*
Sets the file that the QFileInfo provides information about to file.

The file can also include an absolute or relative file path. Absolute paths begin with the directory separator (e.g. "/" under Unix) or a drive specification (under Windows). Relative file names begin with a directory name or a file name and specify a path relative to the current directory.

Example:


  QString absolute = "/local/bin";
  QString relative = "local/bin";
  QFileInfo absFile(absolute);
  QFileInfo relFile(relative);

  QDir::setCurrent(QDir::rootPath());
  // absFile and relFile now point to the same file

  QDir::setCurrent("/tmp");
  // absFile now points to "/local/bin",
  // while relFile points to "/tmp/local/bin"



See also isFile(), isRelative(), QDir::setCurrent(), and QDir::isRelativePath().
*/
impl /*struct*/ QFileInfo {
  pub fn setFile_1<RetType, T: QFileInfo_setFile_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFile_1(self);
    // return 1;
  }
}
pub trait QFileInfo_setFile_1<RetType> {
  fn setFile_1(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_setFile_1<(/*void*/)> for (usize) {
  fn setFile_1(self , rsthis: & QFileInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QFileInfo7setFileERK5QFile", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:82
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setFile(const QDir &, const QString &)

/*
Sets the file that the QFileInfo provides information about to file.

The file can also include an absolute or relative file path. Absolute paths begin with the directory separator (e.g. "/" under Unix) or a drive specification (under Windows). Relative file names begin with a directory name or a file name and specify a path relative to the current directory.

Example:


  QString absolute = "/local/bin";
  QString relative = "local/bin";
  QFileInfo absFile(absolute);
  QFileInfo relFile(relative);

  QDir::setCurrent(QDir::rootPath());
  // absFile and relFile now point to the same file

  QDir::setCurrent("/tmp");
  // absFile now points to "/local/bin",
  // while relFile points to "/tmp/local/bin"



See also isFile(), isRelative(), QDir::setCurrent(), and QDir::isRelativePath().
*/
impl /*struct*/ QFileInfo {
  pub fn setFile_2<RetType, T: QFileInfo_setFile_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFile_2(self);
    // return 1;
  }
}
pub trait QFileInfo_setFile_2<RetType> {
  fn setFile_2(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_setFile_2<(/*void*/)> for (usize,usize) {
  fn setFile_2(self , rsthis: & QFileInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QFileInfo7setFileERK4QDirRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:83
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exists() const

/*
Returns true if the file exists; otherwise returns false.

Note: If the file is a symlink that points to a non-existing file, false is returned.
*/
impl /*struct*/ QFileInfo {
  pub fn exists_0<RetType, T: QFileInfo_exists_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exists_0(self);
    // return 1;
  }
}
pub trait QFileInfo_exists_0<RetType> {
  fn exists_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_exists_0<bool> for () {
  fn exists_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo6existsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:84
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool exists(const QString &)

/*
Returns true if the file exists; otherwise returns false.

Note: If the file is a symlink that points to a non-existing file, false is returned.
*/
impl /*struct*/ QFileInfo {
  pub fn exists_1<RetType, T: QFileInfo_exists_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.exists_1();
    // return 1;
  }
}
pub trait QFileInfo_exists_1<RetType> {
  fn exists_1(self ) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_exists_1<bool> for (usize) {
  fn exists_1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QFileInfo6existsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void refresh()

/*
Refreshes the information about the file, i.e. reads in information from the file system the next time a cached property is fetched.
*/
impl /*struct*/ QFileInfo {
  pub fn refresh_0<RetType, T: QFileInfo_refresh_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refresh_0(self);
    // return 1;
  }
}
pub trait QFileInfo_refresh_0<RetType> {
  fn refresh_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_refresh_0<(/*void*/)> for () {
  fn refresh_0(self , rsthis: & QFileInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QFileInfo7refreshEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:87
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath() const

/*
Returns the file name, including the path (which may be absolute or relative).

See also absoluteFilePath(), canonicalFilePath(), and isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn filePath_0<RetType, T: QFileInfo_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QFileInfo_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_filePath_0<usize> for () {
  fn filePath_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8filePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:88
// index:0
// Public Visibility=Default Availability=Available
// [8] QString absoluteFilePath() const

/*
Returns an absolute path including the file name.

The absolute path name consists of the full path and the file name. On Unix this will always begin with the root, '/', directory. On Windows this will always begin 'D:/' where D is a drive letter, except for network shares that are not mapped to a drive letter, in which case the path will begin '//sharename/'. QFileInfo will uppercase drive letters. Note that QDir does not do this. The code snippet below shows this.


      QFileInfo fi("c:/temp/foo"); => fi.absoluteFilePath() => "C:/temp/foo"



This function returns the same as filePath(), unless isRelative() is true. In contrast to canonicalFilePath(), symbolic links or redundant "." or ".." elements are not necessarily removed.

Warning: If filePath() is empty the behavior of this function is undefined.

See also filePath(), canonicalFilePath(), and isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn absoluteFilePath_0<RetType, T: QFileInfo_absoluteFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.absoluteFilePath_0(self);
    // return 1;
  }
}
pub trait QFileInfo_absoluteFilePath_0<RetType> {
  fn absoluteFilePath_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_absoluteFilePath_0<usize> for () {
  fn absoluteFilePath_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo16absoluteFilePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:89
// index:0
// Public Visibility=Default Availability=Available
// [8] QString canonicalFilePath() const

/*
Returns the canonical path including the file name, i.e. an absolute path without symbolic links or redundant "." or ".." elements.

If the file does not exist, canonicalFilePath() returns an empty string.

See also filePath(), absoluteFilePath(), and dir().
*/
impl /*struct*/ QFileInfo {
  pub fn canonicalFilePath_0<RetType, T: QFileInfo_canonicalFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canonicalFilePath_0(self);
    // return 1;
  }
}
pub trait QFileInfo_canonicalFilePath_0<RetType> {
  fn canonicalFilePath_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_canonicalFilePath_0<usize> for () {
  fn canonicalFilePath_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo17canonicalFilePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Returns the name of the file, excluding the path.

Example:


  QFileInfo fi("/tmp/archive.tar.gz");
  QString name = fi.fileName();                // name = "archive.tar.gz"



Note that, if this QFileInfo object is given a path ending in a slash, the name of the file is considered empty.

See also isRelative(), filePath(), baseName(), and suffix().
*/
impl /*struct*/ QFileInfo {
  pub fn fileName_0<RetType, T: QFileInfo_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QFileInfo_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:91
// index:0
// Public Visibility=Default Availability=Available
// [8] QString baseName() const

/*
Returns the base name of the file without the path.

The base name consists of all characters in the file up to (but not including) the first '.' character.

Example:


  QFileInfo fi("/tmp/archive.tar.gz");
  QString base = fi.baseName();  // base = "archive"



The base name of a file is computed equally on all platforms, independent of file naming conventions (e.g., ".bashrc" on Unix has an empty base name, and the suffix is "bashrc").

See also fileName(), suffix(), completeSuffix(), and completeBaseName().
*/
impl /*struct*/ QFileInfo {
  pub fn baseName_0<RetType, T: QFileInfo_baseName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.baseName_0(self);
    // return 1;
  }
}
pub trait QFileInfo_baseName_0<RetType> {
  fn baseName_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_baseName_0<usize> for () {
  fn baseName_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8baseNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:92
// index:0
// Public Visibility=Default Availability=Available
// [8] QString completeBaseName() const

/*
Returns the complete base name of the file without the path.

The complete base name consists of all characters in the file up to (but not including) the last '.' character.

Example:


  QFileInfo fi("/tmp/archive.tar.gz");
  QString base = fi.completeBaseName();  // base = "archive.tar"



See also fileName(), suffix(), completeSuffix(), and baseName().
*/
impl /*struct*/ QFileInfo {
  pub fn completeBaseName_0<RetType, T: QFileInfo_completeBaseName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completeBaseName_0(self);
    // return 1;
  }
}
pub trait QFileInfo_completeBaseName_0<RetType> {
  fn completeBaseName_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_completeBaseName_0<usize> for () {
  fn completeBaseName_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo16completeBaseNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:93
// index:0
// Public Visibility=Default Availability=Available
// [8] QString suffix() const

/*
Returns the suffix (extension) of the file.

The suffix consists of all characters in the file after (but not including) the last '.'.

Example:


  QFileInfo fi("/tmp/archive.tar.gz");
  QString ext = fi.suffix();  // ext = "gz"



The suffix of a file is computed equally on all platforms, independent of file naming conventions (e.g., ".bashrc" on Unix has an empty base name, and the suffix is "bashrc").

See also fileName(), completeSuffix(), baseName(), and completeBaseName().
*/
impl /*struct*/ QFileInfo {
  pub fn suffix_0<RetType, T: QFileInfo_suffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.suffix_0(self);
    // return 1;
  }
}
pub trait QFileInfo_suffix_0<RetType> {
  fn suffix_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_suffix_0<usize> for () {
  fn suffix_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo6suffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:94
// index:0
// Public Visibility=Default Availability=Available
// [8] QString bundleName() const

/*
Returns the name of the bundle.

On macOS and iOS this returns the proper localized name for a bundle if the path isBundle(). On all other platforms an empty QString is returned.

Example:


  QFileInfo fi("/Applications/Safari.app");
  QString bundle = fi.bundleName();                // name = "Safari"



This function was introduced in  Qt 4.3.

See also isBundle(), filePath(), baseName(), and suffix().
*/
impl /*struct*/ QFileInfo {
  pub fn bundleName_0<RetType, T: QFileInfo_bundleName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.bundleName_0(self);
    // return 1;
  }
}
pub trait QFileInfo_bundleName_0<RetType> {
  fn bundleName_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_bundleName_0<usize> for () {
  fn bundleName_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo10bundleNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:95
// index:0
// Public Visibility=Default Availability=Available
// [8] QString completeSuffix() const

/*
Returns the complete suffix (extension) of the file.

The complete suffix consists of all characters in the file after (but not including) the first '.'.

Example:


  QFileInfo fi("/tmp/archive.tar.gz");
  QString ext = fi.completeSuffix();  // ext = "tar.gz"



See also fileName(), suffix(), baseName(), and completeBaseName().
*/
impl /*struct*/ QFileInfo {
  pub fn completeSuffix_0<RetType, T: QFileInfo_completeSuffix_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.completeSuffix_0(self);
    // return 1;
  }
}
pub trait QFileInfo_completeSuffix_0<RetType> {
  fn completeSuffix_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_completeSuffix_0<usize> for () {
  fn completeSuffix_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo14completeSuffixEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:97
// index:0
// Public Visibility=Default Availability=Available
// [8] QString path() const

/*
Returns the file's path. This doesn't include the file name.

Note that, if this QFileInfo object is given a path ending in a slash, the name of the file is considered empty and this function will return the entire path.

See also filePath(), absolutePath(), canonicalPath(), dir(), fileName(), and isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn path_0<RetType, T: QFileInfo_path_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.path_0(self);
    // return 1;
  }
}
pub trait QFileInfo_path_0<RetType> {
  fn path_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_path_0<usize> for () {
  fn path_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo4pathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:98
// index:0
// Public Visibility=Default Availability=Available
// [8] QString absolutePath() const

/*
Returns a file's path absolute path. This doesn't include the file name.

On Unix the absolute path will always begin with the root, '/', directory. On Windows this will always begin 'D:/' where D is a drive letter, except for network shares that are not mapped to a drive letter, in which case the path will begin '//sharename/'.

In contrast to canonicalPath() symbolic links or redundant "." or ".." elements are not necessarily removed.

Warning: If filePath() is empty the behavior of this function is undefined.

See also absoluteFilePath(), path(), canonicalPath(), fileName(), and isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn absolutePath_0<RetType, T: QFileInfo_absolutePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.absolutePath_0(self);
    // return 1;
  }
}
pub trait QFileInfo_absolutePath_0<RetType> {
  fn absolutePath_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_absolutePath_0<usize> for () {
  fn absolutePath_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo12absolutePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:99
// index:0
// Public Visibility=Default Availability=Available
// [8] QString canonicalPath() const

/*
Returns the file's path canonical path (excluding the file name), i.e. an absolute path without symbolic links or redundant "." or ".." elements.

If the file does not exist, canonicalPath() returns an empty string.

See also path() and absolutePath().
*/
impl /*struct*/ QFileInfo {
  pub fn canonicalPath_0<RetType, T: QFileInfo_canonicalPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canonicalPath_0(self);
    // return 1;
  }
}
pub trait QFileInfo_canonicalPath_0<RetType> {
  fn canonicalPath_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_canonicalPath_0<usize> for () {
  fn canonicalPath_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo13canonicalPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:100
// index:0
// Public Visibility=Default Availability=Available
// [8] QDir dir() const

/*
Returns the path of the object's parent directory as a QDir object.

Note: The QDir returned always corresponds to the object's parent directory, even if the QFileInfo represents a directory.

For each of the following, dir() returns the QDir "~/examples/191697".


      QFileInfo fileInfo1("~/examples/191697/.");
      QFileInfo fileInfo2("~/examples/191697/..");
      QFileInfo fileInfo3("~/examples/191697/main.cpp");



For each of the following, dir() returns the QDir ".".


      QFileInfo fileInfo4(".");
      QFileInfo fileInfo5("..");
      QFileInfo fileInfo6("main.cpp");



See also absolutePath(), filePath(), fileName(), isRelative(), and absoluteDir().
*/
impl /*struct*/ QFileInfo {
  pub fn dir_0<RetType, T: QFileInfo_dir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dir_0(self);
    // return 1;
  }
}
pub trait QFileInfo_dir_0<RetType> {
  fn dir_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_dir_0<usize> for () {
  fn dir_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo3dirEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:101
// index:0
// Public Visibility=Default Availability=Available
// [8] QDir absoluteDir() const

/*
Returns the file's absolute path as a QDir object.

See also dir(), filePath(), fileName(), and isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn absoluteDir_0<RetType, T: QFileInfo_absoluteDir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.absoluteDir_0(self);
    // return 1;
  }
}
pub trait QFileInfo_absoluteDir_0<RetType> {
  fn absoluteDir_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_absoluteDir_0<usize> for () {
  fn absoluteDir_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo11absoluteDirEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:103
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadable() const

/*
Returns true if the user can read the file; otherwise returns false.

Note: If the NTFS permissions check has not been enabled, the result on Windows will merely reflect whether the file exists.

See also isWritable(), isExecutable(), and permission().
*/
impl /*struct*/ QFileInfo {
  pub fn isReadable_0<RetType, T: QFileInfo_isReadable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadable_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isReadable_0<RetType> {
  fn isReadable_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isReadable_0<bool> for () {
  fn isReadable_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo10isReadableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:104
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWritable() const

/*
Returns true if the user can write to the file; otherwise returns false.

Note: If the NTFS permissions check has not been enabled, the result on Windows will merely reflect whether the file is marked as Read Only.

See also isReadable(), isExecutable(), and permission().
*/
impl /*struct*/ QFileInfo {
  pub fn isWritable_0<RetType, T: QFileInfo_isWritable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWritable_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isWritable_0<RetType> {
  fn isWritable_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isWritable_0<bool> for () {
  fn isWritable_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo10isWritableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:105
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isExecutable() const

/*
Returns true if the file is executable; otherwise returns false.

See also isReadable(), isWritable(), and permission().
*/
impl /*struct*/ QFileInfo {
  pub fn isExecutable_0<RetType, T: QFileInfo_isExecutable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isExecutable_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isExecutable_0<RetType> {
  fn isExecutable_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isExecutable_0<bool> for () {
  fn isExecutable_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo12isExecutableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:106
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isHidden() const

/*
Returns true if this is a `hidden' file; otherwise returns false.

Note: This function returns true for the special entries "." and ".." on Unix, even though QDir::entryList threats them as shown.
*/
impl /*struct*/ QFileInfo {
  pub fn isHidden_0<RetType, T: QFileInfo_isHidden_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isHidden_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isHidden_0<RetType> {
  fn isHidden_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isHidden_0<bool> for () {
  fn isHidden_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8isHiddenEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:107
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isNativePath() const

/*
Returns true if the file path can be used directly with native APIs. Returns false if the file is otherwise supported by a virtual file system inside Qt, such as the Qt Resource System.

Note: Native paths may still require conversion of path separators and character encoding, depending on platform and input requirements of the native API.

This function was introduced in  Qt 5.0.

See also QDir::toNativeSeparators(), QFile::encodeName(), filePath(), absoluteFilePath(), and canonicalFilePath().
*/
impl /*struct*/ QFileInfo {
  pub fn isNativePath_0<RetType, T: QFileInfo_isNativePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isNativePath_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isNativePath_0<RetType> {
  fn isNativePath_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isNativePath_0<bool> for () {
  fn isNativePath_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo12isNativePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:109
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRelative() const

/*
Returns true if the file path name is relative, otherwise returns false if the path is absolute (e.g. under Unix a path is absolute if it begins with a "/").

See also isAbsolute().
*/
impl /*struct*/ QFileInfo {
  pub fn isRelative_0<RetType, T: QFileInfo_isRelative_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRelative_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isRelative_0<RetType> {
  fn isRelative_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isRelative_0<bool> for () {
  fn isRelative_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo10isRelativeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:110
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAbsolute() const

/*
Returns true if the file path name is absolute, otherwise returns false if the path is relative.

See also isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn isAbsolute_0<RetType, T: QFileInfo_isAbsolute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAbsolute_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isAbsolute_0<RetType> {
  fn isAbsolute_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isAbsolute_0<bool> for () {
  fn isAbsolute_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo10isAbsoluteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:111
// index:0
// Public Visibility=Default Availability=Available
// [1] bool makeAbsolute()

/*
Converts the file's path to an absolute path if it is not already in that form. Returns true to indicate that the path was converted; otherwise returns false to indicate that the path was already absolute.

See also filePath() and isRelative().
*/
impl /*struct*/ QFileInfo {
  pub fn makeAbsolute_0<RetType, T: QFileInfo_makeAbsolute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.makeAbsolute_0(self);
    // return 1;
  }
}
pub trait QFileInfo_makeAbsolute_0<RetType> {
  fn makeAbsolute_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_makeAbsolute_0<bool> for () {
  fn makeAbsolute_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QFileInfo12makeAbsoluteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:113
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isFile() const

/*
Returns true if this object points to a file or to a symbolic link to a file. Returns false if the object points to something which isn't a file, such as a directory.

See also isDir(), isSymLink(), and isBundle().
*/
impl /*struct*/ QFileInfo {
  pub fn isFile_0<RetType, T: QFileInfo_isFile_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isFile_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isFile_0<RetType> {
  fn isFile_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isFile_0<bool> for () {
  fn isFile_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo6isFileEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:114
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isDir() const

/*
Returns true if this object points to a directory or to a symbolic link to a directory; otherwise returns false.

See also isFile(), isSymLink(), and isBundle().
*/
impl /*struct*/ QFileInfo {
  pub fn isDir_0<RetType, T: QFileInfo_isDir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isDir_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isDir_0<RetType> {
  fn isDir_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isDir_0<bool> for () {
  fn isDir_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo5isDirEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:115
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isSymLink() const

/*
Returns true if this object points to a symbolic link; otherwise returns false.

Symbolic links exist on Unix (including macOS and iOS) and Windows and are typically created by the ln -s or mklink commands, respectively. Opening a symbolic link effectively opens the link's target.

In addition, true will be returned for shortcuts (*.lnk files) on Windows. Opening those will open the .lnk file itself.

Example:


  QFileInfo info(fileName);
  if (info.isSymLink())
      fileName = info.symLinkTarget();



Note: If the symlink points to a non existing file, exists() returns false.

See also isFile(), isDir(), and symLinkTarget().
*/
impl /*struct*/ QFileInfo {
  pub fn isSymLink_0<RetType, T: QFileInfo_isSymLink_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isSymLink_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isSymLink_0<RetType> {
  fn isSymLink_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isSymLink_0<bool> for () {
  fn isSymLink_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo9isSymLinkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:116
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRoot() const

/*
Returns true if the object points to a directory or to a symbolic link to a directory, and that directory is the root directory; otherwise returns false.
*/
impl /*struct*/ QFileInfo {
  pub fn isRoot_0<RetType, T: QFileInfo_isRoot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRoot_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isRoot_0<RetType> {
  fn isRoot_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isRoot_0<bool> for () {
  fn isRoot_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo6isRootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:117
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isBundle() const

/*
Returns true if this object points to a bundle or to a symbolic link to a bundle on macOS and iOS; otherwise returns false.

This function was introduced in  Qt 4.3.

See also isDir(), isSymLink(), and isFile().
*/
impl /*struct*/ QFileInfo {
  pub fn isBundle_0<RetType, T: QFileInfo_isBundle_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isBundle_0(self);
    // return 1;
  }
}
pub trait QFileInfo_isBundle_0<RetType> {
  fn isBundle_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_isBundle_0<bool> for () {
  fn isBundle_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8isBundleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QString readLink() const

/*

*/
impl /*struct*/ QFileInfo {
  pub fn readLink_0<RetType, T: QFileInfo_readLink_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.readLink_0(self);
    // return 1;
  }
}
pub trait QFileInfo_readLink_0<RetType> {
  fn readLink_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_readLink_0<usize> for () {
  fn readLink_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8readLinkEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:120
// index:0
// Public inline Visibility=Default Availability=Available
// [8] QString symLinkTarget() const

/*
Returns the absolute path to the file or directory a symbolic link points to, or an empty string if the object isn't a symbolic link.

This name may not represent an existing file; it is only a string. QFileInfo::exists() returns true if the symlink points to an existing file.

This function was introduced in  Qt 4.2.

See also exists(), isSymLink(), isDir(), and isFile().
*/
impl /*struct*/ QFileInfo {
  pub fn symLinkTarget_0<RetType, T: QFileInfo_symLinkTarget_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.symLinkTarget_0(self);
    // return 1;
  }
}
pub trait QFileInfo_symLinkTarget_0<RetType> {
  fn symLinkTarget_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_symLinkTarget_0<usize> for () {
  fn symLinkTarget_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo13symLinkTargetEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:122
// index:0
// Public Visibility=Default Availability=Available
// [8] QString owner() const

/*
Returns the owner of the file. On systems where files do not have owners, or if an error occurs, an empty string is returned.

This function can be time consuming under Unix (in the order of milliseconds). On Windows, it will return an empty string unless the NTFS permissions check has been enabled.

See also ownerId(), group(), and groupId().
*/
impl /*struct*/ QFileInfo {
  pub fn owner_0<RetType, T: QFileInfo_owner_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.owner_0(self);
    // return 1;
  }
}
pub trait QFileInfo_owner_0<RetType> {
  fn owner_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_owner_0<usize> for () {
  fn owner_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo5ownerEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:123
// index:0
// Public Visibility=Default Availability=Available
// [4] uint ownerId() const

/*
Returns the id of the owner of the file.

On Windows and on systems where files do not have owners this function returns ((uint) -2).

See also owner(), group(), and groupId().
*/
impl /*struct*/ QFileInfo {
  pub fn ownerId_0<RetType, T: QFileInfo_ownerId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ownerId_0(self);
    // return 1;
  }
}
pub trait QFileInfo_ownerId_0<RetType> {
  fn ownerId_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_ownerId_0<u32> for () {
  fn ownerId_0(self , rsthis: & QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo7ownerIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:124
// index:0
// Public Visibility=Default Availability=Available
// [8] QString group() const

/*
Returns the group of the file. On Windows, on systems where files do not have groups, or if an error occurs, an empty string is returned.

This function can be time consuming under Unix (in the order of milliseconds).

See also groupId(), owner(), and ownerId().
*/
impl /*struct*/ QFileInfo {
  pub fn group_0<RetType, T: QFileInfo_group_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.group_0(self);
    // return 1;
  }
}
pub trait QFileInfo_group_0<RetType> {
  fn group_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_group_0<usize> for () {
  fn group_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo5groupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:125
// index:0
// Public Visibility=Default Availability=Available
// [4] uint groupId() const

/*
Returns the id of the group the file belongs to.

On Windows and on systems where files do not have groups this function always returns (uint) -2.

See also group(), owner(), and ownerId().
*/
impl /*struct*/ QFileInfo {
  pub fn groupId_0<RetType, T: QFileInfo_groupId_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.groupId_0(self);
    // return 1;
  }
}
pub trait QFileInfo_groupId_0<RetType> {
  fn groupId_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_groupId_0<u32> for () {
  fn groupId_0(self , rsthis: & QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo7groupIdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:127
// index:0
// Public Visibility=Default Availability=Available
// [1] bool permission(QFile::Permissions) const

/*
Tests for file permissions. The permissions argument can be several flags of type QFile::Permissions OR-ed together to check for permission combinations.

On systems where files do not have permissions this function always returns true.

Note: The result might be inaccurate on Windows if the NTFS permissions check has not been enabled.

Example:


  QFileInfo fi("/tmp/archive.tar.gz");
  if (fi.permission(QFile::WriteUser | QFile::ReadGroup))
      qWarning("I can change the file; my group can read the file");
  if (fi.permission(QFile::WriteGroup | QFile::WriteOther))
      qWarning("The group or others can change the file");



See also isReadable(), isWritable(), and isExecutable().
*/
impl /*struct*/ QFileInfo {
  pub fn permission_0<RetType, T: QFileInfo_permission_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.permission_0(self);
    // return 1;
  }
}
pub trait QFileInfo_permission_0<RetType> {
  fn permission_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_permission_0<bool> for (i32) {
  fn permission_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo10permissionE6QFlagsIN11QFileDevice10PermissionEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:128
// index:0
// Public Visibility=Default Availability=Available
// [4] QFile::Permissions permissions() const

/*
Returns the complete OR-ed together combination of QFile::Permissions for the file.

Note: The result might be inaccurate on Windows if the NTFS permissions check has not been enabled.
*/
impl /*struct*/ QFileInfo {
  pub fn permissions_0<RetType, T: QFileInfo_permissions_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.permissions_0(self);
    // return 1;
  }
}
pub trait QFileInfo_permissions_0<RetType> {
  fn permissions_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_permissions_0<i32> for () {
  fn permissions_0(self , rsthis: & QFileInfo) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo11permissionsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] qint64 size() const

/*
Returns the file size in bytes. If the file does not exist or cannot be fetched, 0 is returned.

See also exists().
*/
impl /*struct*/ QFileInfo {
  pub fn size_0<RetType, T: QFileInfo_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QFileInfo_size_0<RetType> {
  fn size_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_size_0<i64> for () {
  fn size_0(self , rsthis: & QFileInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i64 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:135
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime created() const

/*

*/
impl /*struct*/ QFileInfo {
  pub fn created_0<RetType, T: QFileInfo_created_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.created_0(self);
    // return 1;
  }
}
pub trait QFileInfo_created_0<RetType> {
  fn created_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_created_0<usize> for () {
  fn created_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo7createdEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:137
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime birthTime() const

/*
Returns the date and time when the file was created / born.

If the file birth time is not available, this function returns an invalid QDateTime.

This function was introduced in  Qt 5.10.

See also lastModified(), lastRead(), and metadataChangeTime().
*/
impl /*struct*/ QFileInfo {
  pub fn birthTime_0<RetType, T: QFileInfo_birthTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.birthTime_0(self);
    // return 1;
  }
}
pub trait QFileInfo_birthTime_0<RetType> {
  fn birthTime_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_birthTime_0<usize> for () {
  fn birthTime_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo9birthTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime metadataChangeTime() const

/*
Returns the date and time when the file metadata was changed. A metadata change occurs when the file is created, but it also occurs whenever the user writes or sets inode information (for example, changing the file permissions).

This function was introduced in  Qt 5.10.

See also lastModified() and lastRead().
*/
impl /*struct*/ QFileInfo {
  pub fn metadataChangeTime_0<RetType, T: QFileInfo_metadataChangeTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metadataChangeTime_0(self);
    // return 1;
  }
}
pub trait QFileInfo_metadataChangeTime_0<RetType> {
  fn metadataChangeTime_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_metadataChangeTime_0<usize> for () {
  fn metadataChangeTime_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo18metadataChangeTimeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:139
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime lastModified() const

/*
Returns the date and local time when the file was last modified.

See also birthTime(), lastRead(), metadataChangeTime(), and fileTime().
*/
impl /*struct*/ QFileInfo {
  pub fn lastModified_0<RetType, T: QFileInfo_lastModified_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastModified_0(self);
    // return 1;
  }
}
pub trait QFileInfo_lastModified_0<RetType> {
  fn lastModified_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_lastModified_0<usize> for () {
  fn lastModified_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo12lastModifiedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:140
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime lastRead() const

/*
Returns the date and local time when the file was last read (accessed).

On platforms where this information is not available, returns the same as lastModified().

See also birthTime(), lastModified(), metadataChangeTime(), and fileTime().
*/
impl /*struct*/ QFileInfo {
  pub fn lastRead_0<RetType, T: QFileInfo_lastRead_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.lastRead_0(self);
    // return 1;
  }
}
pub trait QFileInfo_lastRead_0<RetType> {
  fn lastRead_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_lastRead_0<usize> for () {
  fn lastRead_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8lastReadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:141
// index:0
// Public Visibility=Default Availability=Available
// [8] QDateTime fileTime(QFile::FileTime) const

/*
Returns the file time specified by time. If the time cannot be determined, an invalid date time is returned.

This function was introduced in  Qt 5.10.

See also QFile::FileTime and QDateTime::isValid().
*/
impl /*struct*/ QFileInfo {
  pub fn fileTime_0<RetType, T: QFileInfo_fileTime_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileTime_0(self);
    // return 1;
  }
}
pub trait QFileInfo_fileTime_0<RetType> {
  fn fileTime_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_fileTime_0<usize> for (i32) {
  fn fileTime_0(self , rsthis: & QFileInfo) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo8fileTimeEN11QFileDevice8FileTimeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:143
// index:0
// Public Visibility=Default Availability=Available
// [1] bool caching() const

/*
Returns true if caching is enabled; otherwise returns false.

See also setCaching() and refresh().
*/
impl /*struct*/ QFileInfo {
  pub fn caching_0<RetType, T: QFileInfo_caching_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.caching_0(self);
    // return 1;
  }
}
pub trait QFileInfo_caching_0<RetType> {
  fn caching_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_caching_0<bool> for () {
  fn caching_0(self , rsthis: & QFileInfo) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QFileInfo7cachingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfileinfo.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setCaching(bool)

/*
If enable is true, enables caching of file information. If enable is false caching is disabled.

When caching is enabled, QFileInfo reads the file information from the file system the first time it's needed, but generally not later.

Caching is enabled by default.

See also refresh() and caching().
*/
impl /*struct*/ QFileInfo {
  pub fn setCaching_0<RetType, T: QFileInfo_setCaching_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setCaching_0(self);
    // return 1;
  }
}
pub trait QFileInfo_setCaching_0<RetType> {
  fn setCaching_0(self , rsthis: & QFileInfo) -> RetType;
}
impl<'a> /*trait*/ QFileInfo_setCaching_0<(/*void*/)> for (bool) {
  fn setCaching_0(self , rsthis: & QFileInfo) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QFileInfo10setCachingEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
