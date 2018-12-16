

// mod ::core::QDir
// package qtcore
// /usr/include/qt/QtCore/qdir.h
// #include <qdir.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 60
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
#[derive(Default)] // class sizeof(QDir)=8
pub struct QDir {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QDir_ITF interface {
//    QDir_PTR() *QDir
//}
//func (ptr *QDir) QDir_PTR() *QDir { return ptr }

impl /*struct*/ QDir {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QDir {
    return QDir{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QDir {
//  type Target = QDirBASE;
//
//  fn deref(&self) -> &QDirBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QDirBASE> for QDir {
//  fn as_ref(& self) -> & QDirBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qdir.h:102
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QDir(const QString &)

/*
Constructs a QDir object that is a copy of the QDir object for directory dir.

See also operator=().
*/
// QDir(const QString &) ctx.fn_proto_cpp
impl /*struct*/ QDir {
  pub fn QDir_0<T: QDir_QDir_0>(value: T) -> QDir {
    let rsthis = value.QDir_0();
    return rsthis;
    // return 1;
  }
}

pub trait QDir_QDir_0 {
  fn QDir_0(self) -> QDir;
}
// QDir(const QString &) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDir_QDir_0 for (usize) {
  fn QDir_0(self) -> QDir {
    // unsafe{_ZN4QDirC2ERK7QString()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QDirC2ERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDir{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:103
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QDir(const QString &, const QString &, QDir::SortFlags, QDir::Filters)

/*
Constructs a QDir object that is a copy of the QDir object for directory dir.

See also operator=().
*/
// QDir(const QString &, const QString &, QDir::SortFlags, QDir::Filters) ctx.fn_proto_cpp
impl /*struct*/ QDir {
  pub fn QDir_1<T: QDir_QDir_1>(value: T) -> QDir {
    let rsthis = value.QDir_1();
    return rsthis;
    // return 1;
  }
}

pub trait QDir_QDir_1 {
  fn QDir_1(self) -> QDir;
}
// QDir(const QString &, const QString &, QDir::SortFlags, QDir::Filters) ctx.fn_proto_cpp
impl<'a> /*trait*/ QDir_QDir_1 for (usize,usize,i32,i32) {
  fn QDir_1(self) -> QDir {
    // unsafe{_ZN4QDirC2ERK7QStringS2_6QFlagsINS_8SortFlagEES3_INS_6FilterEE()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN4QDirC2ERK7QStringS2_6QFlagsINS_8SortFlagEES3_INS_6FilterEE", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QDir{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:105
// index:0
// Public Visibility=Default Availability=Available
// [-2] void ~QDir()

/*

*/
pub fn DeleteQDir(this :*mut QDir) {
    // let rv = qtrt::InvokeQtFunc6("_ZN4QDirD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 8)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qdir.h:107
// index:0
// Public Visibility=Default Availability=Available
// [8] QDir & operator=(const QDir &)

/*

*/
impl /*struct*/ QDir {
  pub fn operator_equal_0<RetType, T: QDir_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QDir_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDiraSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:108
// index:1
// Public Visibility=Default Availability=Available
// [8] QDir & operator=(const QString &)

/*

*/
impl /*struct*/ QDir {
  pub fn operator_equal_1<RetType, T: QDir_operator_equal_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_1(self);
    // return 1;
  }
}
pub trait QDir_operator_equal_1<RetType> {
  fn operator_equal_1(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_operator_equal_1<usize> for (usize) {
  fn operator_equal_1(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDiraSERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:110
// index:2
// Public inline Visibility=Default Availability=Available
// [8] QDir & operator=(QDir &&)

/*

*/
impl /*struct*/ QDir {
  pub fn operator_equal_2<RetType, T: QDir_operator_equal_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_2(self);
    // return 1;
  }
}
pub trait QDir_operator_equal_2<RetType> {
  fn operator_equal_2(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_operator_equal_2<usize> for (usize) {
  fn operator_equal_2(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDiraSEOS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:113
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void swap(QDir &)

/*
Swaps this QDir instance with other. This function is very fast and never fails.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QDir {
  pub fn swap_0<RetType, T: QDir_swap_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.swap_0(self);
    // return 1;
  }
}
pub trait QDir_swap_0<RetType> {
  fn swap_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_swap_0<(/*void*/)> for (usize) {
  fn swap_0(self , rsthis: & QDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir4swapERS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:116
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setPath(const QString &)

/*
Sets the path of the directory to path. The path is cleaned of redundant ".", ".." and of multiple separators. No check is made to see whether a directory with this path actually exists; but you can check for yourself using exists().

The path can be either absolute or relative. Absolute paths begin with the directory separator "/" (optionally preceded by a drive specification under Windows). Relative file names begin with a directory name or a file name and specify a path relative to the current directory. An example of an absolute path is the string "/tmp/quartz", a relative path might look like "src/fatlib".

See also path(), absolutePath(), exists(), cleanPath(), dirName(), absoluteFilePath(), isRelative(), and makeAbsolute().
*/
impl /*struct*/ QDir {
  pub fn setPath_0<RetType, T: QDir_setPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setPath_0(self);
    // return 1;
  }
}
pub trait QDir_setPath_0<RetType> {
  fn setPath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_setPath_0<(/*void*/)> for (usize) {
  fn setPath_0(self , rsthis: & QDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir7setPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:117
// index:0
// Public Visibility=Default Availability=Available
// [8] QString path() const

/*
Returns the path. This may contain symbolic links, but never contains redundant ".", ".." or multiple separators.

The returned path can be either absolute or relative (see setPath()).

See also setPath(), absolutePath(), exists(), cleanPath(), dirName(), absoluteFilePath(), toNativeSeparators(), and makeAbsolute().
*/
impl /*struct*/ QDir {
  pub fn path_0<RetType, T: QDir_path_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.path_0(self);
    // return 1;
  }
}
pub trait QDir_path_0<RetType> {
  fn path_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_path_0<usize> for () {
  fn path_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir4pathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:118
// index:0
// Public Visibility=Default Availability=Available
// [8] QString absolutePath() const

/*
Returns the absolute path (a path that starts with "/" or with a drive specification), which may contain symbolic links, but never contains redundant ".", ".." or multiple separators.

See also setPath(), canonicalPath(), exists(), cleanPath(), dirName(), and absoluteFilePath().
*/
impl /*struct*/ QDir {
  pub fn absolutePath_0<RetType, T: QDir_absolutePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.absolutePath_0(self);
    // return 1;
  }
}
pub trait QDir_absolutePath_0<RetType> {
  fn absolutePath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_absolutePath_0<usize> for () {
  fn absolutePath_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir12absolutePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:119
// index:0
// Public Visibility=Default Availability=Available
// [8] QString canonicalPath() const

/*
Returns the canonical path, i.e. a path without symbolic links or redundant "." or ".." elements.

On systems that do not have symbolic links this function will always return the same string that absolutePath() returns. If the canonical path does not exist (normally due to dangling symbolic links) canonicalPath() returns an empty string.

Example:


  QString bin = "/local/bin";         // where /local/bin is a symlink to /usr/bin
  QDir binDir(bin);
  QString canonicalBin = binDir.canonicalPath();
  // canonicalBin now equals "/usr/bin"

  QString ls = "/local/bin/ls";       // where ls is the executable "ls"
  QDir lsDir(ls);
  QString canonicalLs = lsDir.canonicalPath();
  // canonicalLS now equals "/usr/bin/ls".



See also path(), absolutePath(), exists(), cleanPath(), dirName(), and absoluteFilePath().
*/
impl /*struct*/ QDir {
  pub fn canonicalPath_0<RetType, T: QDir_canonicalPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.canonicalPath_0(self);
    // return 1;
  }
}
pub trait QDir_canonicalPath_0<RetType> {
  fn canonicalPath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_canonicalPath_0<usize> for () {
  fn canonicalPath_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir13canonicalPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:121
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void addResourceSearchPath(const QString &)

/*

*/
impl /*struct*/ QDir {
  pub fn addResourceSearchPath_0<RetType, T: QDir_addResourceSearchPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addResourceSearchPath_0();
    // return 1;
  }
}
pub trait QDir_addResourceSearchPath_0<RetType> {
  fn addResourceSearchPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_addResourceSearchPath_0<(/*void*/)> for (usize) {
  fn addResourceSearchPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir21addResourceSearchPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:123
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setSearchPaths(const QString &, const QStringList &)

/*
Sets or replaces Qt's search paths for file names with the prefix prefix to searchPaths.

To specify a prefix for a file name, prepend the prefix followed by a single colon (e.g., "images:undo.png", "xmldocs:books.xml"). prefix can only contain letters or numbers (e.g., it cannot contain a colon, nor a slash).

Qt uses this search path to locate files with a known prefix. The search path entries are tested in order, starting with the first entry.


  QDir::setSearchPaths("icons", QStringList(QDir::homePath() + "/images"));
  QDir::setSearchPaths("docs", QStringList(":/embeddedDocuments"));
  ...
  QPixmap pixmap("icons:undo.png"); // will look for undo.png in QDir::homePath() + "/images"
  QFile file("docs:design.odf"); // will look in the :/embeddedDocuments resource path



File name prefix must be at least 2 characters long to avoid conflicts with Windows drive letters.

Search paths may contain paths to The Qt Resource System.

This function was introduced in  Qt 4.3.

See also searchPaths().
*/
impl /*struct*/ QDir {
  pub fn setSearchPaths_0<RetType, T: QDir_setSearchPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setSearchPaths_0();
    // return 1;
  }
}
pub trait QDir_setSearchPaths_0<RetType> {
  fn setSearchPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_setSearchPaths_0<(/*void*/)> for (usize,usize) {
  fn setSearchPaths_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir14setSearchPathsERK7QStringRK11QStringList", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:124
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void addSearchPath(const QString &, const QString &)

/*
Adds path to the search path for prefix.

This function was introduced in  Qt 4.3.

See also setSearchPaths().
*/
impl /*struct*/ QDir {
  pub fn addSearchPath_0<RetType, T: QDir_addSearchPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.addSearchPath_0();
    // return 1;
  }
}
pub trait QDir_addSearchPath_0<RetType> {
  fn addSearchPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_addSearchPath_0<(/*void*/)> for (usize,usize) {
  fn addSearchPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir13addSearchPathERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:125
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList searchPaths(const QString &)

/*
Returns the search paths for prefix.

This function was introduced in  Qt 4.3.

See also setSearchPaths() and addSearchPath().
*/
impl /*struct*/ QDir {
  pub fn searchPaths_0<RetType, T: QDir_searchPaths_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.searchPaths_0();
    // return 1;
  }
}
pub trait QDir_searchPaths_0<RetType> {
  fn searchPaths_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_searchPaths_0<usize> for (usize) {
  fn searchPaths_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir11searchPathsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:127
// index:0
// Public Visibility=Default Availability=Available
// [8] QString dirName() const

/*
Returns the name of the directory; this is not the same as the path, e.g. a directory with the name "mail", might have the path "/var/spool/mail". If the directory has no name (e.g. it is the root directory) an empty string is returned.

No check is made to ensure that a directory with this name actually exists; but see exists().

See also path(), filePath(), absolutePath(), and absoluteFilePath().
*/
impl /*struct*/ QDir {
  pub fn dirName_0<RetType, T: QDir_dirName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.dirName_0(self);
    // return 1;
  }
}
pub trait QDir_dirName_0<RetType> {
  fn dirName_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_dirName_0<usize> for () {
  fn dirName_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir7dirNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:128
// index:0
// Public Visibility=Default Availability=Available
// [8] QString filePath(const QString &) const

/*
Returns the path name of a file in the directory. Does not check if the file actually exists in the directory; but see exists(). If the QDir is relative the returned path name will also be relative. Redundant multiple separators or "." and ".." directories in fileName are not removed (see cleanPath()).

See also dirName(), absoluteFilePath(), isRelative(), and canonicalPath().
*/
impl /*struct*/ QDir {
  pub fn filePath_0<RetType, T: QDir_filePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filePath_0(self);
    // return 1;
  }
}
pub trait QDir_filePath_0<RetType> {
  fn filePath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_filePath_0<usize> for (usize) {
  fn filePath_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir8filePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:129
// index:0
// Public Visibility=Default Availability=Available
// [8] QString absoluteFilePath(const QString &) const

/*
Returns the absolute path name of a file in the directory. Does not check if the file actually exists in the directory; but see exists(). Redundant multiple separators or "." and ".." directories in fileName are not removed (see cleanPath()).

See also relativeFilePath(), filePath(), and canonicalPath().
*/
impl /*struct*/ QDir {
  pub fn absoluteFilePath_0<RetType, T: QDir_absoluteFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.absoluteFilePath_0(self);
    // return 1;
  }
}
pub trait QDir_absoluteFilePath_0<RetType> {
  fn absoluteFilePath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_absoluteFilePath_0<usize> for (usize) {
  fn absoluteFilePath_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir16absoluteFilePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:130
// index:0
// Public Visibility=Default Availability=Available
// [8] QString relativeFilePath(const QString &) const

/*
Returns the path to fileName relative to the directory.


  QDir dir("/home/bob");
  QString s;

  s = dir.relativeFilePath("images/file.jpg");     // s is "images/file.jpg"
  s = dir.relativeFilePath("/home/mary/file.txt"); // s is "../mary/file.txt"



See also absoluteFilePath(), filePath(), and canonicalPath().
*/
impl /*struct*/ QDir {
  pub fn relativeFilePath_0<RetType, T: QDir_relativeFilePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.relativeFilePath_0(self);
    // return 1;
  }
}
pub trait QDir_relativeFilePath_0<RetType> {
  fn relativeFilePath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_relativeFilePath_0<usize> for (usize) {
  fn relativeFilePath_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir16relativeFilePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:132
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString toNativeSeparators(const QString &)

/*
Returns pathName with the '/' separators converted to separators that are appropriate for the underlying operating system.

On Windows, toNativeSeparators("c:/winnt/system32") returns "c:\winnt\system32".

The returned string may be the same as the argument on some operating systems, for example on Unix.

This function was introduced in  Qt 4.2.

See also fromNativeSeparators() and separator().
*/
impl /*struct*/ QDir {
  pub fn toNativeSeparators_0<RetType, T: QDir_toNativeSeparators_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.toNativeSeparators_0();
    // return 1;
  }
}
pub trait QDir_toNativeSeparators_0<RetType> {
  fn toNativeSeparators_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_toNativeSeparators_0<usize> for (usize) {
  fn toNativeSeparators_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir18toNativeSeparatorsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:133
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString fromNativeSeparators(const QString &)

/*
Returns pathName using '/' as file separator. On Windows, for instance, fromNativeSeparators("c:\\winnt\\system32") returns "c:/winnt/system32".

The returned string may be the same as the argument on some operating systems, for example on Unix.

This function was introduced in  Qt 4.2.

See also toNativeSeparators() and separator().
*/
impl /*struct*/ QDir {
  pub fn fromNativeSeparators_0<RetType, T: QDir_fromNativeSeparators_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromNativeSeparators_0();
    // return 1;
  }
}
pub trait QDir_fromNativeSeparators_0<RetType> {
  fn fromNativeSeparators_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_fromNativeSeparators_0<usize> for (usize) {
  fn fromNativeSeparators_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir20fromNativeSeparatorsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:135
// index:0
// Public Visibility=Default Availability=Available
// [1] bool cd(const QString &)

/*
Changes the QDir's directory to dirName.

Returns true if the new directory exists; otherwise returns false. Note that the logical cd() operation is not performed if the new directory does not exist.

Calling cd("..") is equivalent to calling cdUp().

See also cdUp(), isReadable(), exists(), and path().
*/
impl /*struct*/ QDir {
  pub fn cd_0<RetType, T: QDir_cd_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cd_0(self);
    // return 1;
  }
}
pub trait QDir_cd_0<RetType> {
  fn cd_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_cd_0<bool> for (usize) {
  fn cd_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir2cdERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:136
// index:0
// Public Visibility=Default Availability=Available
// [1] bool cdUp()

/*
Changes directory by moving one directory up from the QDir's current directory.

Returns true if the new directory exists; otherwise returns false. Note that the logical cdUp() operation is not performed if the new directory does not exist.

See also cd(), isReadable(), exists(), and path().
*/
impl /*struct*/ QDir {
  pub fn cdUp_0<RetType, T: QDir_cdUp_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.cdUp_0(self);
    // return 1;
  }
}
pub trait QDir_cdUp_0<RetType> {
  fn cdUp_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_cdUp_0<bool> for () {
  fn cdUp_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir4cdUpEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:138
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList nameFilters() const

/*
Returns the string list set by setNameFilters()

See also setNameFilters().
*/
impl /*struct*/ QDir {
  pub fn nameFilters_0<RetType, T: QDir_nameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.nameFilters_0(self);
    // return 1;
  }
}
pub trait QDir_nameFilters_0<RetType> {
  fn nameFilters_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_nameFilters_0<usize> for () {
  fn nameFilters_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir11nameFiltersEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:139
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setNameFilters(const QStringList &)

/*
Sets the name filters used by entryList() and entryInfoList() to the list of filters specified by nameFilters.

Each name filter is a wildcard (globbing) filter that understands * and ? wildcards. (See QRegExp wildcard matching.)

For example, the following code sets three name filters on a QDir to ensure that only files with extensions typically used for C++ source files are listed:


      QStringList filters;
      filters << "*.cpp" << "*.cxx" << "*.cc";
      dir.setNameFilters(filters);



See also nameFilters() and setFilter().
*/
impl /*struct*/ QDir {
  pub fn setNameFilters_0<RetType, T: QDir_setNameFilters_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setNameFilters_0(self);
    // return 1;
  }
}
pub trait QDir_setNameFilters_0<RetType> {
  fn setNameFilters_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_setNameFilters_0<(/*void*/)> for (usize) {
  fn setNameFilters_0(self , rsthis: & QDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir14setNameFiltersERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:141
// index:0
// Public Visibility=Default Availability=Available
// [4] QDir::Filters filter() const

/*
Returns the value set by setFilter()

See also setFilter().
*/
impl /*struct*/ QDir {
  pub fn filter_0<RetType, T: QDir_filter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.filter_0(self);
    // return 1;
  }
}
pub trait QDir_filter_0<RetType> {
  fn filter_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_filter_0<i32> for () {
  fn filter_0(self , rsthis: & QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir6filterEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:142
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFilter(QDir::Filters)

/*
Sets the filter used by entryList() and entryInfoList() to filters. The filter is used to specify the kind of files that should be returned by entryList() and entryInfoList(). See QDir::Filter.

See also filter() and setNameFilters().
*/
impl /*struct*/ QDir {
  pub fn setFilter_0<RetType, T: QDir_setFilter_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFilter_0(self);
    // return 1;
  }
}
pub trait QDir_setFilter_0<RetType> {
  fn setFilter_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_setFilter_0<(/*void*/)> for (i32) {
  fn setFilter_0(self , rsthis: & QDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir9setFilterE6QFlagsINS_6FilterEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:143
// index:0
// Public Visibility=Default Availability=Available
// [4] QDir::SortFlags sorting() const

/*
Returns the value set by setSorting()

See also setSorting() and SortFlag.
*/
impl /*struct*/ QDir {
  pub fn sorting_0<RetType, T: QDir_sorting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sorting_0(self);
    // return 1;
  }
}
pub trait QDir_sorting_0<RetType> {
  fn sorting_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_sorting_0<i32> for () {
  fn sorting_0(self , rsthis: & QDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir7sortingEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:144
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setSorting(QDir::SortFlags)

/*
Sets the sort order used by entryList() and entryInfoList().

The sort is specified by OR-ing values from the enum QDir::SortFlag.

See also sorting() and SortFlag.
*/
impl /*struct*/ QDir {
  pub fn setSorting_0<RetType, T: QDir_setSorting_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setSorting_0(self);
    // return 1;
  }
}
pub trait QDir_setSorting_0<RetType> {
  fn setSorting_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_setSorting_0<(/*void*/)> for (i32) {
  fn setSorting_0(self , rsthis: & QDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN4QDir10setSortingE6QFlagsINS_8SortFlagEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qdir.h:146
// index:0
// Public Visibility=Default Availability=Available
// [4] uint count() const

/*
Returns the total number of directories and files in the directory.

Equivalent to entryList().count().

See also operator[]() and entryList().
*/
impl /*struct*/ QDir {
  pub fn count_0<RetType, T: QDir_count_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.count_0(self);
    // return 1;
  }
}
pub trait QDir_count_0<RetType> {
  fn count_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_count_0<u32> for () {
  fn count_0(self , rsthis: & QDir) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir5countEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: u32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:147
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isEmpty(QDir::Filters) const

/*
Returns whether the directory is empty.

Equivalent to count() == 0 with filters QDir::AllEntries | QDir::NoDotAndDotDot, but faster as it just checks whether the directory contains at least one entry.

Note: Unless you set the filters flags to include QDir::NoDotAndDotDot (as the default value does), no directory is empty.

This function was introduced in  Qt 5.9.

See also count(), entryList(), and setFilter().
*/
impl /*struct*/ QDir {
  pub fn isEmpty_0<RetType, T: QDir_isEmpty_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isEmpty_0(self);
    // return 1;
  }
}
pub trait QDir_isEmpty_0<RetType> {
  fn isEmpty_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_isEmpty_0<bool> for (i32) {
  fn isEmpty_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir7isEmptyE6QFlagsINS_6FilterEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:149
// index:0
// Public Visibility=Default Availability=Available
// [8] QString operator[](int) const

/*

*/
impl /*struct*/ QDir {
  pub fn operator_get_index_0<RetType, T: QDir_operator_get_index_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_get_index_0(self);
    // return 1;
  }
}
pub trait QDir_operator_get_index_0<RetType> {
  fn operator_get_index_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_operator_get_index_0<usize> for (i32) {
  fn operator_get_index_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDirixEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:151
// index:0
// Public static Visibility=Default Availability=Available
// [8] QStringList nameFiltersFromString(const QString &)

/*

*/
impl /*struct*/ QDir {
  pub fn nameFiltersFromString_0<RetType, T: QDir_nameFiltersFromString_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.nameFiltersFromString_0();
    // return 1;
  }
}
pub trait QDir_nameFiltersFromString_0<RetType> {
  fn nameFiltersFromString_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_nameFiltersFromString_0<usize> for (usize) {
  fn nameFiltersFromString_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir21nameFiltersFromStringERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:153
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList entryList(QDir::Filters, QDir::SortFlags) const

/*
Returns a list of the names of all the files and directories in the directory, ordered according to the name and attribute filters previously set with setNameFilters() and setFilter(), and sorted according to the flags set with setSorting().

The name filter, file attribute filter, and sorting specification can be overridden using the nameFilters, filters, and sort arguments.

Returns an empty list if the directory is unreadable, does not exist, or if nothing matches the specification.

See also entryInfoList(), setNameFilters(), setSorting(), and setFilter().
*/
impl /*struct*/ QDir {
  pub fn entryList_0<RetType, T: QDir_entryList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.entryList_0(self);
    // return 1;
  }
}
pub trait QDir_entryList_0<RetType> {
  fn entryList_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_entryList_0<usize> for (i32,i32) {
  fn entryList_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir9entryListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:154
// index:1
// Public Visibility=Default Availability=Available
// [8] QStringList entryList(const QStringList &, QDir::Filters, QDir::SortFlags) const

/*
Returns a list of the names of all the files and directories in the directory, ordered according to the name and attribute filters previously set with setNameFilters() and setFilter(), and sorted according to the flags set with setSorting().

The name filter, file attribute filter, and sorting specification can be overridden using the nameFilters, filters, and sort arguments.

Returns an empty list if the directory is unreadable, does not exist, or if nothing matches the specification.

See also entryInfoList(), setNameFilters(), setSorting(), and setFilter().
*/
impl /*struct*/ QDir {
  pub fn entryList_1<RetType, T: QDir_entryList_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.entryList_1(self);
    // return 1;
  }
}
pub trait QDir_entryList_1<RetType> {
  fn entryList_1(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_entryList_1<usize> for (usize,i32,i32) {
  fn entryList_1(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir9entryListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] QFileInfoList entryInfoList(QDir::Filters, QDir::SortFlags) const

/*
Returns a list of QFileInfo objects for all the files and directories in the directory, ordered according to the name and attribute filters previously set with setNameFilters() and setFilter(), and sorted according to the flags set with setSorting().

The name filter, file attribute filter, and sorting specification can be overridden using the nameFilters, filters, and sort arguments.

Returns an empty list if the directory is unreadable, does not exist, or if nothing matches the specification.

See also entryList(), setNameFilters(), setSorting(), setFilter(), isReadable(), and exists().
*/
impl /*struct*/ QDir {
  pub fn entryInfoList_0<RetType, T: QDir_entryInfoList_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.entryInfoList_0(self);
    // return 1;
  }
}
pub trait QDir_entryInfoList_0<RetType> {
  fn entryInfoList_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_entryInfoList_0<usize> for (i32,i32) {
  fn entryInfoList_0(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir13entryInfoListE6QFlagsINS_6FilterEES0_INS_8SortFlagEE", 2,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:158
// index:1
// Public Visibility=Default Availability=Available
// [-2] QFileInfoList entryInfoList(const QStringList &, QDir::Filters, QDir::SortFlags) const

/*
Returns a list of QFileInfo objects for all the files and directories in the directory, ordered according to the name and attribute filters previously set with setNameFilters() and setFilter(), and sorted according to the flags set with setSorting().

The name filter, file attribute filter, and sorting specification can be overridden using the nameFilters, filters, and sort arguments.

Returns an empty list if the directory is unreadable, does not exist, or if nothing matches the specification.

See also entryList(), setNameFilters(), setSorting(), setFilter(), isReadable(), and exists().
*/
impl /*struct*/ QDir {
  pub fn entryInfoList_1<RetType, T: QDir_entryInfoList_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.entryInfoList_1(self);
    // return 1;
  }
}
pub trait QDir_entryInfoList_1<RetType> {
  fn entryInfoList_1(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_entryInfoList_1<usize> for (usize,i32,i32) {
  fn entryInfoList_1(self , rsthis: & QDir) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir13entryInfoListERK11QStringList6QFlagsINS_6FilterEES3_INS_8SortFlagEE", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:161
// index:0
// Public Visibility=Default Availability=Available
// [1] bool mkdir(const QString &) const

/*
Creates a sub-directory called dirName.

Returns true on success; otherwise returns false.

If the directory already exists when this function is called, it will return false.

See also rmdir().
*/
impl /*struct*/ QDir {
  pub fn mkdir_0<RetType, T: QDir_mkdir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mkdir_0(self);
    // return 1;
  }
}
pub trait QDir_mkdir_0<RetType> {
  fn mkdir_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_mkdir_0<bool> for (usize) {
  fn mkdir_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir5mkdirERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:162
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rmdir(const QString &) const

/*
Removes the directory specified by dirName.

The directory must be empty for rmdir() to succeed.

Returns true if successful; otherwise returns false.

See also mkdir().
*/
impl /*struct*/ QDir {
  pub fn rmdir_0<RetType, T: QDir_rmdir_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rmdir_0(self);
    // return 1;
  }
}
pub trait QDir_rmdir_0<RetType> {
  fn rmdir_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_rmdir_0<bool> for (usize) {
  fn rmdir_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir5rmdirERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:163
// index:0
// Public Visibility=Default Availability=Available
// [1] bool mkpath(const QString &) const

/*
Creates the directory path dirPath.

The function will create all parent directories necessary to create the directory.

Returns true if successful; otherwise returns false.

If the path already exists when this function is called, it will return true.

See also rmpath().
*/
impl /*struct*/ QDir {
  pub fn mkpath_0<RetType, T: QDir_mkpath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mkpath_0(self);
    // return 1;
  }
}
pub trait QDir_mkpath_0<RetType> {
  fn mkpath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_mkpath_0<bool> for (usize) {
  fn mkpath_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir6mkpathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rmpath(const QString &) const

/*
Removes the directory path dirPath.

The function will remove all parent directories in dirPath, provided that they are empty. This is the opposite of mkpath(dirPath).

Returns true if successful; otherwise returns false.

See also mkpath().
*/
impl /*struct*/ QDir {
  pub fn rmpath_0<RetType, T: QDir_rmpath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rmpath_0(self);
    // return 1;
  }
}
pub trait QDir_rmpath_0<RetType> {
  fn rmpath_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_rmpath_0<bool> for (usize) {
  fn rmpath_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir6rmpathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:166
// index:0
// Public Visibility=Default Availability=Available
// [1] bool removeRecursively()

/*
Removes the directory, including all its contents.

Returns true if successful, otherwise false.

If a file or directory cannot be removed, removeRecursively() keeps going and attempts to delete as many files and sub-directories as possible, then returns false.

If the directory was already removed, the method returns true (expected result already reached).

Note: this function is meant for removing a small application-internal directory (such as a temporary directory), but not user-visible directories. For user-visible operations, it is rather recommended to report errors more precisely to the user, to offer solutions in case of errors, to show progress during the deletion since it could take several minutes, etc.

This function was introduced in  Qt 5.0.
*/
impl /*struct*/ QDir {
  pub fn removeRecursively_0<RetType, T: QDir_removeRecursively_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeRecursively_0(self);
    // return 1;
  }
}
pub trait QDir_removeRecursively_0<RetType> {
  fn removeRecursively_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_removeRecursively_0<bool> for () {
  fn removeRecursively_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir17removeRecursivelyEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:168
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isReadable() const

/*
Returns true if the directory is readable and we can open files by name; otherwise returns false.

Warning: A false value from this function is not a guarantee that files in the directory are not accessible.

See also QFileInfo::isReadable().
*/
impl /*struct*/ QDir {
  pub fn isReadable_0<RetType, T: QDir_isReadable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isReadable_0(self);
    // return 1;
  }
}
pub trait QDir_isReadable_0<RetType> {
  fn isReadable_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_isReadable_0<bool> for () {
  fn isReadable_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir10isReadableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:169
// index:0
// Public Visibility=Default Availability=Available
// [1] bool exists() const

/*
Returns true if the file called name exists; otherwise returns false.

Unless name contains an absolute file path, the file name is assumed to be relative to the directory itself, so this function is typically used to check for the presence of files within a directory.

See also QFileInfo::exists() and QFile::exists().
*/
impl /*struct*/ QDir {
  pub fn exists_0<RetType, T: QDir_exists_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exists_0(self);
    // return 1;
  }
}
pub trait QDir_exists_0<RetType> {
  fn exists_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_exists_0<bool> for () {
  fn exists_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir6existsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:183
// index:1
// Public Visibility=Default Availability=Available
// [1] bool exists(const QString &) const

/*
Returns true if the file called name exists; otherwise returns false.

Unless name contains an absolute file path, the file name is assumed to be relative to the directory itself, so this function is typically used to check for the presence of files within a directory.

See also QFileInfo::exists() and QFile::exists().
*/
impl /*struct*/ QDir {
  pub fn exists_1<RetType, T: QDir_exists_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.exists_1(self);
    // return 1;
  }
}
pub trait QDir_exists_1<RetType> {
  fn exists_1(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_exists_1<bool> for (usize) {
  fn exists_1(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir6existsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRoot() const

/*
Returns true if the directory is the root directory; otherwise returns false.

Note: If the directory is a symbolic link to the root directory this function returns false. If you want to test for this use canonicalPath(), e.g.


  QDir dir("/tmp/root_link");
  dir = dir.canonicalPath();
  if (dir.isRoot())
      qWarning("It is a root link");



See also root() and rootPath().
*/
impl /*struct*/ QDir {
  pub fn isRoot_0<RetType, T: QDir_isRoot_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRoot_0(self);
    // return 1;
  }
}
pub trait QDir_isRoot_0<RetType> {
  fn isRoot_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_isRoot_0<bool> for () {
  fn isRoot_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir6isRootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:172
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isRelativePath(const QString &)

/*
Returns true if path is relative; returns false if it is absolute.

See also isRelative(), isAbsolutePath(), and makeAbsolute().
*/
impl /*struct*/ QDir {
  pub fn isRelativePath_0<RetType, T: QDir_isRelativePath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isRelativePath_0();
    // return 1;
  }
}
pub trait QDir_isRelativePath_0<RetType> {
  fn isRelativePath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_isRelativePath_0<bool> for (usize) {
  fn isRelativePath_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir14isRelativePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:173
// index:0
// Public static inline Visibility=Default Availability=Available
// [1] bool isAbsolutePath(const QString &)

/*
Returns true if path is absolute; returns false if it is relative.

See also isAbsolute(), isRelativePath(), makeAbsolute(), and cleanPath().
*/
impl /*struct*/ QDir {
  pub fn isAbsolutePath_0<RetType, T: QDir_isAbsolutePath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isAbsolutePath_0();
    // return 1;
  }
}
pub trait QDir_isAbsolutePath_0<RetType> {
  fn isAbsolutePath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_isAbsolutePath_0<bool> for (usize) {
  fn isAbsolutePath_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir14isAbsolutePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:174
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isRelative() const

/*
Returns true if the directory path is relative; otherwise returns false. (Under Unix a path is relative if it does not start with a "/").

See also makeAbsolute(), isAbsolute(), isAbsolutePath(), and cleanPath().
*/
impl /*struct*/ QDir {
  pub fn isRelative_0<RetType, T: QDir_isRelative_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isRelative_0(self);
    // return 1;
  }
}
pub trait QDir_isRelative_0<RetType> {
  fn isRelative_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_isRelative_0<bool> for () {
  fn isRelative_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir10isRelativeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:175
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAbsolute() const

/*
Returns true if the directory's path is absolute; otherwise returns false. See isAbsolutePath().

See also isRelative(), makeAbsolute(), and cleanPath().
*/
impl /*struct*/ QDir {
  pub fn isAbsolute_0<RetType, T: QDir_isAbsolute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAbsolute_0(self);
    // return 1;
  }
}
pub trait QDir_isAbsolute_0<RetType> {
  fn isAbsolute_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_isAbsolute_0<bool> for () {
  fn isAbsolute_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDir10isAbsoluteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:176
// index:0
// Public Visibility=Default Availability=Available
// [1] bool makeAbsolute()

/*
Converts the directory path to an absolute path. If it is already absolute nothing happens. Returns true if the conversion succeeded; otherwise returns false.

See also isAbsolute(), isAbsolutePath(), isRelative(), and cleanPath().
*/
impl /*struct*/ QDir {
  pub fn makeAbsolute_0<RetType, T: QDir_makeAbsolute_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.makeAbsolute_0(self);
    // return 1;
  }
}
pub trait QDir_makeAbsolute_0<RetType> {
  fn makeAbsolute_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_makeAbsolute_0<bool> for () {
  fn makeAbsolute_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir12makeAbsoluteEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:178
// index:0
// Public Visibility=Default Availability=Available
// [1] bool operator==(const QDir &) const

/*

*/
impl /*struct*/ QDir {
  pub fn operator_equal_equal_0<RetType, T: QDir_operator_equal_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_equal_0(self);
    // return 1;
  }
}
pub trait QDir_operator_equal_equal_0<RetType> {
  fn operator_equal_equal_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_operator_equal_equal_0<bool> for (usize) {
  fn operator_equal_equal_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDireqERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:179
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool operator!=(const QDir &) const

/*

*/
impl /*struct*/ QDir {
  pub fn operator_not_equal_0<RetType, T: QDir_operator_not_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_not_equal_0(self);
    // return 1;
  }
}
pub trait QDir_operator_not_equal_0<RetType> {
  fn operator_not_equal_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_operator_not_equal_0<bool> for (usize) {
  fn operator_not_equal_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK4QDirneERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:181
// index:0
// Public Visibility=Default Availability=Available
// [1] bool remove(const QString &)

/*
Removes the file, fileName.

Returns true if the file is removed successfully; otherwise returns false.
*/
impl /*struct*/ QDir {
  pub fn remove_0<RetType, T: QDir_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QDir_remove_0<RetType> {
  fn remove_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_remove_0<bool> for (usize) {
  fn remove_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir6removeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:182
// index:0
// Public Visibility=Default Availability=Available
// [1] bool rename(const QString &, const QString &)

/*
Renames a file or directory from oldName to newName, and returns true if successful; otherwise returns false.

On most file systems, rename() fails only if oldName does not exist, or if a file with the new name already exists. However, there are also other reasons why rename() can fail. For example, on at least one file system rename() fails if newName points to an open file.

If oldName is a file (not a directory) that can't be renamed right away, Qt will try to copy oldName to newName and remove oldName.

See also QFile::rename().
*/
impl /*struct*/ QDir {
  pub fn rename_0<RetType, T: QDir_rename_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.rename_0(self);
    // return 1;
  }
}
pub trait QDir_rename_0<RetType> {
  fn rename_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_rename_0<bool> for (usize,usize) {
  fn rename_0(self , rsthis: & QDir) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir6renameERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:185
// index:0
// Public static Visibility=Default Availability=Available
// [-2] QFileInfoList drives()

/*
Returns a list of the root directories on this system.

On Windows this returns a list of QFileInfo objects containing "C:/", "D:/", etc. On other operating systems, it returns a list containing just one root directory (i.e. "/").

See also root() and rootPath().
*/
impl /*struct*/ QDir {
  pub fn drives_0<RetType, T: QDir_drives_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.drives_0();
    // return 1;
  }
}
pub trait QDir_drives_0<RetType> {
  fn drives_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_drives_0<usize> for () {
  fn drives_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir6drivesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:187
// index:0
// Public static inline Visibility=Default Availability=Available
// [2] QChar listSeparator()

/*
Returns the native path list separator: ':' under Unix and ';' under Windows.

This function was introduced in  Qt 5.6.

See also separator().
*/
impl /*struct*/ QDir {
  pub fn listSeparator_0<RetType, T: QDir_listSeparator_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.listSeparator_0();
    // return 1;
  }
}
pub trait QDir_listSeparator_0<RetType> {
  fn listSeparator_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_listSeparator_0<usize> for () {
  fn listSeparator_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir13listSeparatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:196
// index:0
// Public static Visibility=Default Availability=Available
// [2] QChar separator()

/*
Returns the native directory separator: "/" under Unix and "\" under Windows.

You do not need to use this function to build file paths. If you always use "/", Qt will translate your paths to conform to the underlying operating system. If you want to display paths to the user using their operating system's separator use toNativeSeparators().

See also listSeparator().
*/
impl /*struct*/ QDir {
  pub fn separator_0<RetType, T: QDir_separator_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.separator_0();
    // return 1;
  }
}
pub trait QDir_separator_0<RetType> {
  fn separator_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_separator_0<usize> for () {
  fn separator_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir9separatorEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:198
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool setCurrent(const QString &)

/*
Sets the application's current working directory to path. Returns true if the directory was successfully changed; otherwise returns false.

See also current(), currentPath(), home(), root(), and temp().
*/
impl /*struct*/ QDir {
  pub fn setCurrent_0<RetType, T: QDir_setCurrent_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCurrent_0();
    // return 1;
  }
}
pub trait QDir_setCurrent_0<RetType> {
  fn setCurrent_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_setCurrent_0<bool> for (usize) {
  fn setCurrent_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir10setCurrentERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:199
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QDir current()

/*
Returns the application's current directory.

The directory is constructed using the absolute path of the current directory, ensuring that its path() will be the same as its absolutePath().

See also currentPath(), setCurrent(), home(), root(), and temp().
*/
impl /*struct*/ QDir {
  pub fn current_0<RetType, T: QDir_current_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.current_0();
    // return 1;
  }
}
pub trait QDir_current_0<RetType> {
  fn current_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_current_0<usize> for () {
  fn current_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir7currentEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:200
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString currentPath()

/*
Returns the absolute path of the application's current directory. The current directory is the last directory set with QDir::setCurrent() or, if that was never called, the directory at which this application was started at by the parent process.

See also current(), setCurrent(), homePath(), rootPath(), tempPath(), and QCoreApplication::applicationDirPath().
*/
impl /*struct*/ QDir {
  pub fn currentPath_0<RetType, T: QDir_currentPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.currentPath_0();
    // return 1;
  }
}
pub trait QDir_currentPath_0<RetType> {
  fn currentPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_currentPath_0<usize> for () {
  fn currentPath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir11currentPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:202
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QDir home()

/*
Returns the user's home directory.

The directory is constructed using the absolute path of the home directory, ensuring that its path() will be the same as its absolutePath().

See homePath() for details.

See also drives(), current(), root(), and temp().
*/
impl /*struct*/ QDir {
  pub fn home_0<RetType, T: QDir_home_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.home_0();
    // return 1;
  }
}
pub trait QDir_home_0<RetType> {
  fn home_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_home_0<usize> for () {
  fn home_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir4homeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:203
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString homePath()

/*
Returns the absolute path of the user's home directory.

Under Windows this function will return the directory of the current user's profile. Typically, this is:


  C:/Documents and Settings/Username



Use the toNativeSeparators() function to convert the separators to the ones that are appropriate for the underlying operating system.

If the directory of the current user's profile does not exist or cannot be retrieved, the following alternatives will be checked (in the given order) until an existing and available path is found:
*/
impl /*struct*/ QDir {
  pub fn homePath_0<RetType, T: QDir_homePath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.homePath_0();
    // return 1;
  }
}
pub trait QDir_homePath_0<RetType> {
  fn homePath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_homePath_0<usize> for () {
  fn homePath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir8homePathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:204
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QDir root()

/*
Returns the root directory.

The directory is constructed using the absolute path of the root directory, ensuring that its path() will be the same as its absolutePath().

See rootPath() for details.

See also drives(), current(), home(), and temp().
*/
impl /*struct*/ QDir {
  pub fn root_0<RetType, T: QDir_root_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.root_0();
    // return 1;
  }
}
pub trait QDir_root_0<RetType> {
  fn root_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_root_0<usize> for () {
  fn root_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir4rootEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:205
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString rootPath()

/*
Returns the absolute path of the root directory.

For Unix operating systems this returns "/". For Windows file systems this normally returns "c:/".

See also root(), drives(), currentPath(), homePath(), and tempPath().
*/
impl /*struct*/ QDir {
  pub fn rootPath_0<RetType, T: QDir_rootPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.rootPath_0();
    // return 1;
  }
}
pub trait QDir_rootPath_0<RetType> {
  fn rootPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_rootPath_0<usize> for () {
  fn rootPath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir8rootPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:206
// index:0
// Public static inline Visibility=Default Availability=Available
// [8] QDir temp()

/*
Returns the system's temporary directory.

The directory is constructed using the absolute path of the temporary directory, ensuring that its path() will be the same as its absolutePath().

See tempPath() for details.

See also drives(), current(), home(), and root().
*/
impl /*struct*/ QDir {
  pub fn temp_0<RetType, T: QDir_temp_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.temp_0();
    // return 1;
  }
}
pub trait QDir_temp_0<RetType> {
  fn temp_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_temp_0<usize> for () {
  fn temp_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir4tempEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:207
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString tempPath()

/*
Returns the absolute path of the system's temporary directory.

On Unix/Linux systems this is the path in the TMPDIR environment variable or /tmp if TMPDIR is not defined. On Windows this is usually the path in the TEMP or TMP environment variable. The path returned by this method doesn't end with a directory separator unless it is the root directory (of a drive).

See also temp(), currentPath(), homePath(), and rootPath().
*/
impl /*struct*/ QDir {
  pub fn tempPath_0<RetType, T: QDir_tempPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.tempPath_0();
    // return 1;
  }
}
pub trait QDir_tempPath_0<RetType> {
  fn tempPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_tempPath_0<usize> for () {
  fn tempPath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir8tempPathEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:210
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool match(const QStringList &, const QString &)

/*
Returns true if the fileName matches the wildcard (glob) pattern filter; otherwise returns false. The filter may contain multiple patterns separated by spaces or semicolons. The matching is case insensitive.

See also QRegExp wildcard matching, QRegExp::exactMatch(), entryList(), and entryInfoList().
*/
impl /*struct*/ QDir {
  pub fn match__0<RetType, T: QDir_match__0<RetType>>( overload_args: T) -> RetType {
    return overload_args.match__0();
    // return 1;
  }
}
pub trait QDir_match__0<RetType> {
  fn match__0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_match__0<bool> for (usize,usize) {
  fn match__0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir5matchERK11QStringListRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:211
// index:1
// Public static Visibility=Default Availability=Available
// [1] bool match(const QString &, const QString &)

/*
Returns true if the fileName matches the wildcard (glob) pattern filter; otherwise returns false. The filter may contain multiple patterns separated by spaces or semicolons. The matching is case insensitive.

See also QRegExp wildcard matching, QRegExp::exactMatch(), entryList(), and entryInfoList().
*/
impl /*struct*/ QDir {
  pub fn match__1<RetType, T: QDir_match__1<RetType>>( overload_args: T) -> RetType {
    return overload_args.match__1();
    // return 1;
  }
}
pub trait QDir_match__1<RetType> {
  fn match__1(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_match__1<bool> for (usize,usize) {
  fn match__1(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir5matchERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:214
// index:0
// Public static Visibility=Default Availability=Available
// [8] QString cleanPath(const QString &)

/*
Returns path with directory separators normalized (converted to "/") and redundant ones removed, and "."s and ".."s resolved (as far as possible).

Symbolic links are kept. This function does not return the canonical path, but rather the simplest version of the input. For example, "./local" becomes "local", "local/../bin" becomes "bin" and "/local/usr/../bin" becomes "/local/bin".

See also absolutePath() and canonicalPath().
*/
impl /*struct*/ QDir {
  pub fn cleanPath_0<RetType, T: QDir_cleanPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanPath_0();
    // return 1;
  }
}
pub trait QDir_cleanPath_0<RetType> {
  fn cleanPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QDir_cleanPath_0<usize> for (usize) {
  fn cleanPath_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN4QDir9cleanPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qdir.h:215
// index:0
// Public Visibility=Default Availability=Available
// [-2] void refresh() const

/*
Refreshes the directory information.
*/
impl /*struct*/ QDir {
  pub fn refresh_0<RetType, T: QDir_refresh_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.refresh_0(self);
    // return 1;
  }
}
pub trait QDir_refresh_0<RetType> {
  fn refresh_0(self , rsthis: & QDir) -> RetType;
}
impl<'a> /*trait*/ QDir_refresh_0<(/*void*/)> for () {
  fn refresh_0(self , rsthis: & QDir) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZNK4QDir7refreshEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}


/*


*/
pub type QDir__Filter = i32;
// 
pub const QDir__Dirs :QDir__Filter = 1;
// 
pub const QDir__Files :QDir__Filter = 2;
// 
pub const QDir__Drives :QDir__Filter = 4;
// 
pub const QDir__NoSymLinks :QDir__Filter = 8;
// 
pub const QDir__AllEntries :QDir__Filter = 7;
// 
pub const QDir__TypeMask :QDir__Filter = 15;
// 
pub const QDir__Readable :QDir__Filter = 16;
// 
pub const QDir__Writable :QDir__Filter = 32;
// 
pub const QDir__Executable :QDir__Filter = 64;
// 
pub const QDir__PermissionMask :QDir__Filter = 112;
// 
pub const QDir__Modified :QDir__Filter = 128;
// 
pub const QDir__Hidden :QDir__Filter = 256;
// 
pub const QDir__System :QDir__Filter = 512;
// 
pub const QDir__AccessMask :QDir__Filter = 1008;
// 
pub const QDir__AllDirs :QDir__Filter = 1024;
// 
pub const QDir__CaseSensitive :QDir__Filter = 2048;
// 
pub const QDir__NoDot :QDir__Filter = 8192;
// 
pub const QDir__NoDotDot :QDir__Filter = 16384;
// 
pub const QDir__NoDotAndDotDot :QDir__Filter = 24576;
// 
pub const QDir__NoFilter :QDir__Filter = -1;
pub fn QDir_FilterItemName(val: i32) ->String {
  match val {
     QDir__Dirs => // 1
     {return String::from("Dirs");}
     QDir__Files => // 2
     {return String::from("Files");}
     QDir__Drives => // 4
     {return String::from("Drives");}
     QDir__NoSymLinks => // 8
     {return String::from("NoSymLinks");}
     QDir__AllEntries => // 7
     {return String::from("AllEntries");}
     QDir__TypeMask => // 15
     {return String::from("TypeMask");}
     QDir__Readable => // 16
     {return String::from("Readable");}
     QDir__Writable => // 32
     {return String::from("Writable");}
     QDir__Executable => // 64
     {return String::from("Executable");}
     QDir__PermissionMask => // 112
     {return String::from("PermissionMask");}
     QDir__Modified => // 128
     {return String::from("Modified");}
     QDir__Hidden => // 256
     {return String::from("Hidden");}
     QDir__System => // 512
     {return String::from("System");}
     QDir__AccessMask => // 1008
     {return String::from("AccessMask");}
     QDir__AllDirs => // 1024
     {return String::from("AllDirs");}
     QDir__CaseSensitive => // 2048
     {return String::from("CaseSensitive");}
     QDir__NoDot => // 8192
     {return String::from("NoDot");}
     QDir__NoDotDot => // 16384
     {return String::from("NoDotDot");}
     QDir__NoDotAndDotDot => // 24576
     {return String::from("NoDotAndDotDot");}
     QDir__NoFilter => // -1
     {return String::from("NoFilter");}
  _ => {return format!("{}", val);}
}
}
pub fn QDir_FilterItemName_s(val: i32) ->String {
  //var nilthis *QDir
  //return nilthis.FilterItemName(val);
  return QDir_FilterItemName(val);
}


/*


*/
pub type QDir__SortFlag = i32;
// 
pub const QDir__Name :QDir__SortFlag = 0;
// 
pub const QDir__Time :QDir__SortFlag = 1;
// 
pub const QDir__Size :QDir__SortFlag = 2;
// 
pub const QDir__Unsorted :QDir__SortFlag = 3;
// 
pub const QDir__SortByMask :QDir__SortFlag = 3;
// 
pub const QDir__DirsFirst :QDir__SortFlag = 4;
// 
pub const QDir__Reversed :QDir__SortFlag = 8;
// 
pub const QDir__IgnoreCase :QDir__SortFlag = 16;
// 
pub const QDir__DirsLast :QDir__SortFlag = 32;
// 
pub const QDir__LocaleAware :QDir__SortFlag = 64;
// 
pub const QDir__Type :QDir__SortFlag = 128;
// 
pub const QDir__NoSort :QDir__SortFlag = -1;
pub fn QDir_SortFlagItemName(val: i32) ->String {
  match val {
     QDir__Name => // 0
     {return String::from("Name");}
     QDir__Time => // 1
     {return String::from("Time");}
     QDir__Size => // 2
     {return String::from("Size");}
     QDir__Unsorted => // 3
     {return String::from("Unsorted,SortByMask");}
    // QDir__SortByMask => // 3
    // {return String::from("");}
     QDir__DirsFirst => // 4
     {return String::from("DirsFirst");}
     QDir__Reversed => // 8
     {return String::from("Reversed");}
     QDir__IgnoreCase => // 16
     {return String::from("IgnoreCase");}
     QDir__DirsLast => // 32
     {return String::from("DirsLast");}
     QDir__LocaleAware => // 64
     {return String::from("LocaleAware");}
     QDir__Type => // 128
     {return String::from("Type");}
     QDir__NoSort => // -1
     {return String::from("NoSort");}
  _ => {return format!("{}", val);}
}
}
pub fn QDir_SortFlagItemName_s(val: i32) ->String {
  //var nilthis *QDir
  //return nilthis.SortFlagItemName(val);
  return QDir_SortFlagItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
