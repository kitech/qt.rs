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
  // proto:  void QFileSystemWatcher::FreeQFileSystemWatcher();
  fn _ZN18QFileSystemWatcherD0Ev(qthis: *mut c_void) ;
  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
  fn _ZN18QFileSystemWatcher10removePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QStringList QFileSystemWatcher::directories();
  fn _ZNK18QFileSystemWatcher11directoriesEv(qthis: *mut c_void) ;
  // proto:  QStringList QFileSystemWatcher::files();
  fn _ZNK18QFileSystemWatcher5filesEv(qthis: *mut c_void) ;
  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
  fn _ZN18QFileSystemWatcher8addPathsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
  fn _ZN18QFileSystemWatcher11removePathsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileSystemWatcher::NewQFileSystemWatcher(QObject * parent);
  fn _ZN18QFileSystemWatcherC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
  fn _ZN18QFileSystemWatcher7addPathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
  fn _ZNK18QFileSystemWatcher10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFileSystemWatcher::NewQFileSystemWatcher(const QStringList & paths, QObject * parent);
  fn _ZN18QFileSystemWatcherC1ERK11QStringListP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QFileSystemWatcher)=1
pub struct QFileSystemWatcher {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileSystemWatcher {
  pub fn FreeQFileSystemWatcher<T: QFileSystemWatcher_FreeQFileSystemWatcher>(&mut self, value: T)  {
     value.FreeQFileSystemWatcher(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_FreeQFileSystemWatcher {
  fn FreeQFileSystemWatcher(self, rsthis: &mut QFileSystemWatcher) ;
}

// proto:  void QFileSystemWatcher::FreeQFileSystemWatcher();
impl<'a> /*trait*/ QFileSystemWatcher_FreeQFileSystemWatcher for () {
  fn FreeQFileSystemWatcher(self, rsthis: &mut QFileSystemWatcher)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherD0Ev()};
     unsafe {_ZN18QFileSystemWatcherD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn removePath<T: QFileSystemWatcher_removePath>(&mut self, value: T) -> i8 {
    return value.removePath(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_removePath {
  fn removePath(self, rsthis: &mut QFileSystemWatcher) -> i8;
}

// proto:  bool QFileSystemWatcher::removePath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_removePath for (&'a  QString) {
  fn removePath(self, rsthis: &mut QFileSystemWatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher10removePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QFileSystemWatcher10removePathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn directories<T: QFileSystemWatcher_directories>(&mut self, value: T)  {
     value.directories(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_directories {
  fn directories(self, rsthis: &mut QFileSystemWatcher) ;
}

// proto:  QStringList QFileSystemWatcher::directories();
impl<'a> /*trait*/ QFileSystemWatcher_directories for () {
  fn directories(self, rsthis: &mut QFileSystemWatcher)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher11directoriesEv()};
     unsafe {_ZNK18QFileSystemWatcher11directoriesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn files<T: QFileSystemWatcher_files>(&mut self, value: T)  {
     value.files(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_files {
  fn files(self, rsthis: &mut QFileSystemWatcher) ;
}

// proto:  QStringList QFileSystemWatcher::files();
impl<'a> /*trait*/ QFileSystemWatcher_files for () {
  fn files(self, rsthis: &mut QFileSystemWatcher)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher5filesEv()};
     unsafe {_ZNK18QFileSystemWatcher5filesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn addPaths<T: QFileSystemWatcher_addPaths>(&mut self, value: T)  {
     value.addPaths(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_addPaths {
  fn addPaths(self, rsthis: &mut QFileSystemWatcher) ;
}

// proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_addPaths for (&'a  QStringList) {
  fn addPaths(self, rsthis: &mut QFileSystemWatcher)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher8addPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QFileSystemWatcher8addPathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn removePaths<T: QFileSystemWatcher_removePaths>(&mut self, value: T)  {
     value.removePaths(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_removePaths {
  fn removePaths(self, rsthis: &mut QFileSystemWatcher) ;
}

// proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_removePaths for (&'a  QStringList) {
  fn removePaths(self, rsthis: &mut QFileSystemWatcher)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher11removePathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QFileSystemWatcher11removePathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn addPath<T: QFileSystemWatcher_addPath>(&mut self, value: T) -> i8 {
    return value.addPath(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_addPath {
  fn addPath(self, rsthis: &mut QFileSystemWatcher) -> i8;
}

// proto:  bool QFileSystemWatcher::addPath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_addPath for (&'a  QString) {
  fn addPath(self, rsthis: &mut QFileSystemWatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher7addPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QFileSystemWatcher7addPathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemWatcher {
  pub fn metaObject<T: QFileSystemWatcher_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_metaObject {
  fn metaObject(self, rsthis: &mut QFileSystemWatcher) ;
}

// proto:  const QMetaObject * QFileSystemWatcher::metaObject();
impl<'a> /*trait*/ QFileSystemWatcher_metaObject for () {
  fn metaObject(self, rsthis: &mut QFileSystemWatcher)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher10metaObjectEv()};
     unsafe {_ZNK18QFileSystemWatcher10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QFileSystemWatcher::NewQFileSystemWatcher(const QStringList & paths, QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_NewQFileSystemWatcher for (&'a  QStringList, &'a mut QObject) {
  fn NewQFileSystemWatcher(self) -> QFileSystemWatcher {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherC1ERK11QStringListP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN18QFileSystemWatcherC1ERK11QStringListP7QObject(qthis, arg0, arg1)};
    let rsthis = QFileSystemWatcher{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

