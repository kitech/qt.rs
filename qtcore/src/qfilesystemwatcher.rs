

// mod ::core::QFileSystemWatcher
// package qtcore
// /usr/include/qt/QtCore/qfilesystemwatcher.h
// #include <qfilesystemwatcher.h>
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
#[derive(Default)] // class sizeof(QFileSystemWatcher)=16
pub struct QFileSystemWatcher {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QFileSystemWatcher_ITF interface {
//    QObject_ITF
//    QFileSystemWatcher_PTR() *QFileSystemWatcher
//}
//func (ptr *QFileSystemWatcher) QFileSystemWatcher_PTR() *QFileSystemWatcher { return ptr }

impl /*struct*/ QFileSystemWatcher {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QFileSystemWatcher {
    return QFileSystemWatcher{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QFileSystemWatcher {
//  type Target = QFileSystemWatcherBASE;
//
//  fn deref(&self) -> &QFileSystemWatcherBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QFileSystemWatcherBASE> for QFileSystemWatcher {
//  fn as_ref(& self) -> & QFileSystemWatcherBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qfilesystemwatcher.h:54
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QFileSystemWatcher {
  pub fn metaObject_0<RetType, T: QFileSystemWatcher_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QFileSystemWatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFileSystemWatcher10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:58
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QFileSystemWatcher(QObject *)

/*
Constructs a new file system watcher object with the given parent.
*/
// QFileSystemWatcher(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFileSystemWatcher {
  pub fn QFileSystemWatcher_0<T: QFileSystemWatcher_QFileSystemWatcher_0>(value: T) -> QFileSystemWatcher {
    let rsthis = value.QFileSystemWatcher_0();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemWatcher_QFileSystemWatcher_0 {
  fn QFileSystemWatcher_0(self) -> QFileSystemWatcher;
}
// QFileSystemWatcher(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileSystemWatcher_QFileSystemWatcher_0 for (usize) {
  fn QFileSystemWatcher_0(self) -> QFileSystemWatcher {
    // unsafe{_ZN18QFileSystemWatcherC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcherC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileSystemWatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:59
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QFileSystemWatcher(const QStringList &, QObject *)

/*
Constructs a new file system watcher object with the given parent.
*/
// QFileSystemWatcher(const QStringList &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QFileSystemWatcher {
  pub fn QFileSystemWatcher_1<T: QFileSystemWatcher_QFileSystemWatcher_1>(value: T) -> QFileSystemWatcher {
    let rsthis = value.QFileSystemWatcher_1();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemWatcher_QFileSystemWatcher_1 {
  fn QFileSystemWatcher_1(self) -> QFileSystemWatcher;
}
// QFileSystemWatcher(const QStringList &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QFileSystemWatcher_QFileSystemWatcher_1 for (usize,usize) {
  fn QFileSystemWatcher_1(self) -> QFileSystemWatcher {
    // unsafe{_ZN18QFileSystemWatcherC2ERK11QStringListP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcherC2ERK11QStringListP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QFileSystemWatcher{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:60
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QFileSystemWatcher()

/*

*/
pub fn DeleteQFileSystemWatcher(this :*mut QFileSystemWatcher) {
    // let rv = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcherD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qfilesystemwatcher.h:62
// index:0
// Public Visibility=Default Availability=Available
// [1] bool addPath(const QString &)

/*
Adds path to the file system watcher if path exists. The path is not added if it does not exist, or if it is already being monitored by the file system watcher.

If path specifies a directory, the directoryChanged() signal will be emitted when path is modified or removed from disk; otherwise the fileChanged() signal is emitted when path is modified, renamed or removed.

If the watch was successful, true is returned.

Reasons for a watch failure are generally system-dependent, but may include the resource not existing, access failures, or the total watch count limit, if the platform has one.

Note: There may be a system dependent limit to the number of files and directories that can be monitored simultaneously. If this limit is been reached, path will not be monitored, and false is returned.

See also addPaths() and removePath().
*/
impl /*struct*/ QFileSystemWatcher {
  pub fn addPath_0<RetType, T: QFileSystemWatcher_addPath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPath_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_addPath_0<RetType> {
  fn addPath_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_addPath_0<bool> for (usize) {
  fn addPath_0(self , rsthis: & QFileSystemWatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcher7addPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:63
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList addPaths(const QStringList &)

/*
Adds each path in paths to the file system watcher. Paths are not added if they not exist, or if they are already being monitored by the file system watcher.

If a path specifies a directory, the directoryChanged() signal will be emitted when the path is modified or removed from disk; otherwise the fileChanged() signal is emitted when the path is modified, renamed, or removed.

The return value is a list of paths that could not be watched.

Reasons for a watch failure are generally system-dependent, but may include the resource not existing, access failures, or the total watch count limit, if the platform has one.

Note: There may be a system dependent limit to the number of files and directories that can be monitored simultaneously. If this limit has been reached, the excess paths will not be monitored, and they will be added to the returned QStringList.

See also addPath() and removePaths().
*/
impl /*struct*/ QFileSystemWatcher {
  pub fn addPaths_0<RetType, T: QFileSystemWatcher_addPaths_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.addPaths_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_addPaths_0<RetType> {
  fn addPaths_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_addPaths_0<usize> for (usize) {
  fn addPaths_0(self , rsthis: & QFileSystemWatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcher8addPathsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:64
// index:0
// Public Visibility=Default Availability=Available
// [1] bool removePath(const QString &)

/*
Removes the specified path from the file system watcher.

If the watch is successfully removed, true is returned.

Reasons for watch removal failing are generally system-dependent, but may be due to the path having already been deleted, for example.

See also removePaths() and addPath().
*/
impl /*struct*/ QFileSystemWatcher {
  pub fn removePath_0<RetType, T: QFileSystemWatcher_removePath_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removePath_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_removePath_0<RetType> {
  fn removePath_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_removePath_0<bool> for (usize) {
  fn removePath_0(self , rsthis: & QFileSystemWatcher) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcher10removePathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList removePaths(const QStringList &)

/*
Removes the specified paths from the file system watcher.

The return value is a list of paths which were not able to be unwatched successfully.

Reasons for watch removal failing are generally system-dependent, but may be due to the path having already been deleted, for example.

See also removePath() and addPaths().
*/
impl /*struct*/ QFileSystemWatcher {
  pub fn removePaths_0<RetType, T: QFileSystemWatcher_removePaths_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removePaths_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_removePaths_0<RetType> {
  fn removePaths_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_removePaths_0<usize> for (usize) {
  fn removePaths_0(self , rsthis: & QFileSystemWatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN18QFileSystemWatcher11removePathsERK11QStringList", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:67
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList files() const

/*
Returns a list of paths to files that are being watched.

See also directories().
*/
impl /*struct*/ QFileSystemWatcher {
  pub fn files_0<RetType, T: QFileSystemWatcher_files_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.files_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_files_0<RetType> {
  fn files_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_files_0<usize> for () {
  fn files_0(self , rsthis: & QFileSystemWatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFileSystemWatcher5filesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qfilesystemwatcher.h:68
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList directories() const

/*
Returns a list of paths to directories that are being watched.

See also files().
*/
impl /*struct*/ QFileSystemWatcher {
  pub fn directories_0<RetType, T: QFileSystemWatcher_directories_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.directories_0(self);
    // return 1;
  }
}
pub trait QFileSystemWatcher_directories_0<RetType> {
  fn directories_0(self , rsthis: & QFileSystemWatcher) -> RetType;
}
impl<'a> /*trait*/ QFileSystemWatcher_directories_0<usize> for () {
  fn directories_0(self , rsthis: & QFileSystemWatcher) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK18QFileSystemWatcher11directoriesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
