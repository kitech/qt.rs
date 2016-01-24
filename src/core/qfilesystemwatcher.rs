// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qfilesystemwatcher.h
// dst-file: /src/core/qfilesystemwatcher.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
use super::qstringlist::QStringList; // 773
use super::qobjectdefs::QMetaObject; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFileSystemWatcher_Class_Size() -> c_int;
  // proto:  void QFileSystemWatcher::~QFileSystemWatcher();
  fn C_ZN18QFileSystemWatcherD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
  fn C_ZN18QFileSystemWatcher10removePathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QStringList QFileSystemWatcher::directories();
  fn C_ZNK18QFileSystemWatcher11directoriesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QFileSystemWatcher::files();
  fn C_ZNK18QFileSystemWatcher5filesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
  fn C_ZN18QFileSystemWatcher8addPathsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
  fn C_ZN18QFileSystemWatcher11removePathsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFileSystemWatcher::QFileSystemWatcher(QObject * parent);
  fn C_ZN18QFileSystemWatcherC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
  fn C_ZN18QFileSystemWatcher7addPathERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
  fn C_ZNK18QFileSystemWatcher10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QFileSystemWatcher::QFileSystemWatcher(const QStringList & paths, QObject * parent);
  fn C_ZN18QFileSystemWatcherC2ERK11QStringListP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QFileSystemWatcher)=1
#[derive(Default)]
pub struct QFileSystemWatcher {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _directoryChanged: QFileSystemWatcher_directoryChanged_signal,
  pub _fileChanged: QFileSystemWatcher_fileChanged_signal,
}

impl /*struct*/ QFileSystemWatcher {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFileSystemWatcher {
    return QFileSystemWatcher{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFileSystemWatcher {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QFileSystemWatcher {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QFileSystemWatcher::~QFileSystemWatcher();
impl /*struct*/ QFileSystemWatcher {
  pub fn free<RetType, T: QFileSystemWatcher_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_free<RetType> {
  fn free(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  void QFileSystemWatcher::~QFileSystemWatcher();
impl<'a> /*trait*/ QFileSystemWatcher_free<()> for () {
  fn free(self , rsthis: & QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherD2Ev()};
     unsafe {C_ZN18QFileSystemWatcherD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
impl /*struct*/ QFileSystemWatcher {
  pub fn removePath<RetType, T: QFileSystemWatcher_removePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removePath(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_removePath<RetType> {
  fn removePath(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_removePath<i8> for (&'a QString) {
  fn removePath(self , rsthis: & QFileSystemWatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher10removePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QFileSystemWatcher10removePathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::directories();
impl /*struct*/ QFileSystemWatcher {
  pub fn directories<RetType, T: QFileSystemWatcher_directories<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.directories(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_directories<RetType> {
  fn directories(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::directories();
impl<'a> /*trait*/ QFileSystemWatcher_directories<()> for () {
  fn directories(self , rsthis: & QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher11directoriesEv()};
     unsafe {C_ZNK18QFileSystemWatcher11directoriesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::files();
impl /*struct*/ QFileSystemWatcher {
  pub fn files<RetType, T: QFileSystemWatcher_files<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.files(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_files<RetType> {
  fn files(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::files();
impl<'a> /*trait*/ QFileSystemWatcher_files<()> for () {
  fn files(self , rsthis: & QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher5filesEv()};
     unsafe {C_ZNK18QFileSystemWatcher5filesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
impl /*struct*/ QFileSystemWatcher {
  pub fn addPaths<RetType, T: QFileSystemWatcher_addPaths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPaths(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_addPaths<RetType> {
  fn addPaths(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_addPaths<()> for (&'a QStringList) {
  fn addPaths(self , rsthis: & QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher8addPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QFileSystemWatcher8addPathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
impl /*struct*/ QFileSystemWatcher {
  pub fn removePaths<RetType, T: QFileSystemWatcher_removePaths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removePaths(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_removePaths<RetType> {
  fn removePaths(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_removePaths<()> for (&'a QStringList) {
  fn removePaths(self , rsthis: & QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher11removePathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QFileSystemWatcher11removePathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFileSystemWatcher::QFileSystemWatcher(QObject * parent);
impl /*struct*/ QFileSystemWatcher {
  pub fn new<T: QFileSystemWatcher_new>(value: T) -> QFileSystemWatcher {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemWatcher_new {
  fn new(self) -> QFileSystemWatcher;
}

  // proto:  void QFileSystemWatcher::QFileSystemWatcher(QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_new for (&'a QObject) {
  fn new(self) -> QFileSystemWatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherC2EP7QObject()};
    let ctysz: c_int = unsafe{QFileSystemWatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QFileSystemWatcherC2EP7QObject(arg0)};
    let rsthis = QFileSystemWatcher{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
impl /*struct*/ QFileSystemWatcher {
  pub fn addPath<RetType, T: QFileSystemWatcher_addPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPath(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_addPath<RetType> {
  fn addPath(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_addPath<i8> for (&'a QString) {
  fn addPath(self , rsthis: & QFileSystemWatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher7addPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QFileSystemWatcher7addPathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
impl /*struct*/ QFileSystemWatcher {
  pub fn metaObject<RetType, T: QFileSystemWatcher_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFileSystemWatcher) -> RetType;
}

  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
impl<'a> /*trait*/ QFileSystemWatcher_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QFileSystemWatcher) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QFileSystemWatcher10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSystemWatcher::QFileSystemWatcher(const QStringList & paths, QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_new for (&'a QStringList, &'a QObject) {
  fn new(self) -> QFileSystemWatcher {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherC2ERK11QStringListP7QObject()};
    let ctysz: c_int = unsafe{QFileSystemWatcher_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QFileSystemWatcherC2ERK11QStringListP7QObject(arg0, arg1)};
    let rsthis = QFileSystemWatcher{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QFileSystemWatcher_directoryChanged
pub struct QFileSystemWatcher_directoryChanged_signal{poi:u64}
impl /* struct */ QFileSystemWatcher {
  pub fn directoryChanged(&self) -> QFileSystemWatcher_directoryChanged_signal {
     return QFileSystemWatcher_directoryChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFileSystemWatcher_directoryChanged_signal {
  pub fn connect<T: QFileSystemWatcher_directoryChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFileSystemWatcher_directoryChanged_signal_connect {
  fn connect(self, sigthis: QFileSystemWatcher_directoryChanged_signal);
}

#[derive(Default)] // for QFileSystemWatcher_fileChanged
pub struct QFileSystemWatcher_fileChanged_signal{poi:u64}
impl /* struct */ QFileSystemWatcher {
  pub fn fileChanged(&self) -> QFileSystemWatcher_fileChanged_signal {
     return QFileSystemWatcher_fileChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QFileSystemWatcher_fileChanged_signal {
  pub fn connect<T: QFileSystemWatcher_fileChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QFileSystemWatcher_fileChanged_signal_connect {
  fn connect(self, sigthis: QFileSystemWatcher_fileChanged_signal);
}

// <= body block end

