// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN18QFileSystemWatcherD0Ev() -> i32;
  fn _ZN18QFileSystemWatcher10removePathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK18QFileSystemWatcher11directoriesEv() -> i32;
  fn _ZNK18QFileSystemWatcher5filesEv() -> i32;
  fn _ZN18QFileSystemWatcher8addPathsERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN18QFileSystemWatcher11removePathsERK11QStringList(arg0: *const c_void) -> i32;
  fn _ZN18QFileSystemWatcherC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN18QFileSystemWatcher7addPathERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK18QFileSystemWatcher10metaObjectEv() -> i32;
  fn _ZN18QFileSystemWatcherC1ERK11QStringListP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QFileSystemWatcher)=1
pub struct QFileSystemWatcher {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileSystemWatcher {
  pub fn FreeQFileSystemWatcher<T: QFileSystemWatcher_FreeQFileSystemWatcher>(&mut self, value: T) -> i32 {
    value.FreeQFileSystemWatcher(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_FreeQFileSystemWatcher {
  fn FreeQFileSystemWatcher(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: void QFileSystemWatcher::FreeQFileSystemWatcher();
impl<'a> /*trait*/ QFileSystemWatcher_FreeQFileSystemWatcher for () {
  fn FreeQFileSystemWatcher(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherD0Ev()};
    unsafe {_ZN18QFileSystemWatcherD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn removePath<T: QFileSystemWatcher_removePath>(&mut self, value: T) -> i32 {
    value.removePath(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_removePath {
  fn removePath(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: bool QFileSystemWatcher::removePath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_removePath for (&'a  QString) {
  fn removePath(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher10removePathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QFileSystemWatcher10removePathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn directories<T: QFileSystemWatcher_directories>(&mut self, value: T) -> i32 {
    value.directories(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_directories {
  fn directories(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: QStringList QFileSystemWatcher::directories();
impl<'a> /*trait*/ QFileSystemWatcher_directories for () {
  fn directories(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher11directoriesEv()};
    unsafe {_ZNK18QFileSystemWatcher11directoriesEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn files<T: QFileSystemWatcher_files>(&mut self, value: T) -> i32 {
    value.files(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_files {
  fn files(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: QStringList QFileSystemWatcher::files();
impl<'a> /*trait*/ QFileSystemWatcher_files for () {
  fn files(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher5filesEv()};
    unsafe {_ZNK18QFileSystemWatcher5filesEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn addPaths<T: QFileSystemWatcher_addPaths>(&mut self, value: T) -> i32 {
    value.addPaths(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_addPaths {
  fn addPaths(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: QStringList QFileSystemWatcher::addPaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_addPaths for (&'a  QStringList) {
  fn addPaths(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher8addPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QFileSystemWatcher8addPathsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn removePaths<T: QFileSystemWatcher_removePaths>(&mut self, value: T) -> i32 {
    value.removePaths(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_removePaths {
  fn removePaths(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: QStringList QFileSystemWatcher::removePaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_removePaths for (&'a  QStringList) {
  fn removePaths(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher11removePathsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QFileSystemWatcher11removePathsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn NewQFileSystemWatcher<T: QFileSystemWatcher_NewQFileSystemWatcher>(value: T) -> QFileSystemWatcher {
    let rsthis = value.NewQFileSystemWatcher();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemWatcher_NewQFileSystemWatcher {
  fn NewQFileSystemWatcher(self) -> QFileSystemWatcher;
}

// proto: void QFileSystemWatcher::NewQFileSystemWatcher(QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_NewQFileSystemWatcher for (&'a mut QObject) {
  fn NewQFileSystemWatcher(self) -> QFileSystemWatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QFileSystemWatcherC1EP7QObject(qthis, arg0)};
    let rsthis = QFileSystemWatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn addPath<T: QFileSystemWatcher_addPath>(&mut self, value: T) -> i32 {
    value.addPath(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_addPath {
  fn addPath(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: bool QFileSystemWatcher::addPath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_addPath for (&'a  QString) {
  fn addPath(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher7addPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QFileSystemWatcher7addPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn metaObject<T: QFileSystemWatcher_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFileSystemWatcher_metaObject {
  fn metaObject(self, this: &mut QFileSystemWatcher) -> i32;
}

// proto: const QMetaObject * QFileSystemWatcher::metaObject();
impl<'a> /*trait*/ QFileSystemWatcher_metaObject for () {
  fn metaObject(self, this: &mut QFileSystemWatcher) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher10metaObjectEv()};
    unsafe {_ZNK18QFileSystemWatcher10metaObjectEv()};
    return 1;
  }
}

// proto: void QFileSystemWatcher::NewQFileSystemWatcher(const QStringList & paths, QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_NewQFileSystemWatcher for (&'a  QStringList, &'a mut QObject) {
  fn NewQFileSystemWatcher(self) -> QFileSystemWatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherC1ERK11QStringListP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN18QFileSystemWatcherC1ERK11QStringListP7QObject(qthis, arg0, arg1)};
    let rsthis = QFileSystemWatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

