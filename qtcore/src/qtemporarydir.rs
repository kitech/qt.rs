

// mod ::core::QTemporaryDir
// package qtcore
// /usr/include/qt/QtCore/qtemporarydir.h
// #include <qtemporarydir.h>
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
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QTemporaryDir)=8
pub struct QTemporaryDir {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QTemporaryDir_ITF interface {
//    QTemporaryDir_PTR() *QTemporaryDir
//}
//func (ptr *QTemporaryDir) QTemporaryDir_PTR() *QTemporaryDir { return ptr }

impl /*struct*/ QTemporaryDir {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QTemporaryDir {
    return QTemporaryDir{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QTemporaryDir {
//  type Target = QTemporaryDirBASE;
//
//  fn deref(&self) -> &QTemporaryDirBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QTemporaryDirBASE> for QTemporaryDir {
//  fn as_ref(& self) -> & QTemporaryDirBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qtemporarydir.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QTemporaryDir()

/*
Constructs a QTemporaryDir using as template the application name returned by QCoreApplication::applicationName() (otherwise qt_temp). The directory is stored in the system's temporary directory, QDir::tempPath().

See also QDir::tempPath().
*/
// QTemporaryDir() ctx.fn_proto_cpp
impl /*struct*/ QTemporaryDir {
  pub fn QTemporaryDir_0<T: QTemporaryDir_QTemporaryDir_0>(value: T) -> QTemporaryDir {
    let rsthis = value.QTemporaryDir_0();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryDir_QTemporaryDir_0 {
  fn QTemporaryDir_0(self) -> QTemporaryDir;
}
// QTemporaryDir() ctx.fn_proto_cpp
impl<'a> /*trait*/ QTemporaryDir_QTemporaryDir_0 for () {
  fn QTemporaryDir_0(self) -> QTemporaryDir {
    // unsafe{_ZN13QTemporaryDirC2Ev()};
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QTemporaryDirC2Ev", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTemporaryDir{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:57
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QTemporaryDir(const QString &)

/*
Constructs a QTemporaryDir using as template the application name returned by QCoreApplication::applicationName() (otherwise qt_temp). The directory is stored in the system's temporary directory, QDir::tempPath().

See also QDir::tempPath().
*/
// QTemporaryDir(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QTemporaryDir {
  pub fn QTemporaryDir_1<T: QTemporaryDir_QTemporaryDir_1>(value: T) -> QTemporaryDir {
    let rsthis = value.QTemporaryDir_1();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryDir_QTemporaryDir_1 {
  fn QTemporaryDir_1(self) -> QTemporaryDir;
}
// QTemporaryDir(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QTemporaryDir_QTemporaryDir_1 for (usize) {
  fn QTemporaryDir_1(self) -> QTemporaryDir {
    // unsafe{_ZN13QTemporaryDirC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QTemporaryDirC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QTemporaryDir{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QTemporaryDir()

/*

*/
pub fn DeleteQTemporaryDir(this :*mut QTemporaryDir) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QTemporaryDirD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qtemporarydir.h:60
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isValid() const

/*
Returns true if the QTemporaryDir was created successfully.
*/
impl /*struct*/ QTemporaryDir {
  pub fn isValid_0<RetType, T: QTemporaryDir_isValid_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isValid_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_isValid_0<RetType> {
  fn isValid_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_isValid_0<bool> for () {
  fn isValid_0(self , rsthis: & QTemporaryDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTemporaryDir7isValidEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:61
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
If isValid() returns false, this function returns the error string that explains why the creation of the temporary directory failed. Otherwise, this function return an empty string.

This function was introduced in  Qt 5.6.
*/
impl /*struct*/ QTemporaryDir {
  pub fn errorString_0<RetType, T: QTemporaryDir_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QTemporaryDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTemporaryDir11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:63
// index:0
// Public Visibility=Default Availability=Available
// [1] bool autoRemove() const

/*
Returns true if the QTemporaryDir is in auto remove mode. Auto-remove mode will automatically delete the directory from disk upon destruction. This makes it very easy to create your QTemporaryDir object on the stack, fill it with files, do something with the files, and finally on function return it will automatically clean up after itself.

Auto-remove is on by default.

See also setAutoRemove() and remove().
*/
impl /*struct*/ QTemporaryDir {
  pub fn autoRemove_0<RetType, T: QTemporaryDir_autoRemove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.autoRemove_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_autoRemove_0<RetType> {
  fn autoRemove_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_autoRemove_0<bool> for () {
  fn autoRemove_0(self , rsthis: & QTemporaryDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTemporaryDir10autoRemoveEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:64
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAutoRemove(bool)

/*
Sets the QTemporaryDir into auto-remove mode if b is true.

Auto-remove is on by default.

See also autoRemove() and remove().
*/
impl /*struct*/ QTemporaryDir {
  pub fn setAutoRemove_0<RetType, T: QTemporaryDir_setAutoRemove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAutoRemove_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_setAutoRemove_0<RetType> {
  fn setAutoRemove_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_setAutoRemove_0<(/*void*/)> for (bool) {
  fn setAutoRemove_0(self , rsthis: & QTemporaryDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN13QTemporaryDir13setAutoRemoveEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:65
// index:0
// Public Visibility=Default Availability=Available
// [1] bool remove()

/*
Removes the temporary directory, including all its contents.

Returns true if removing was successful.
*/
impl /*struct*/ QTemporaryDir {
  pub fn remove_0<RetType, T: QTemporaryDir_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_remove_0<RetType> {
  fn remove_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_remove_0<bool> for () {
  fn remove_0(self , rsthis: & QTemporaryDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QTemporaryDir6removeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QString path() const

/*
Returns the path to the temporary directory. Empty if the QTemporaryDir could not be created.
*/
impl /*struct*/ QTemporaryDir {
  pub fn path_0<RetType, T: QTemporaryDir_path_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.path_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_path_0<RetType> {
  fn path_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_path_0<usize> for () {
  fn path_0(self , rsthis: & QTemporaryDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTemporaryDir4pathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qtemporarydir.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath(const QString &) const

/*
Returns the path name of a file in the temporary directory. Does not check if the file actually exists in the directory. Redundant multiple separators or "." and ".." directories in fileName are not removed (see QDir::cleanPath()). Absolute paths are not allowed.

This function was introduced in  Qt 5.9.
*/
impl /*struct*/ QTemporaryDir {
  pub fn filePath_0<RetType, T: QTemporaryDir_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QTemporaryDir_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QTemporaryDir) -> RetType;
}
impl<'a> /*trait*/ QTemporaryDir_filePath_0<usize> for (usize) {
  fn filePath_0(self , rsthis: & QTemporaryDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QTemporaryDir8filePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
