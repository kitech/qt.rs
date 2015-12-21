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
  // proto:  void QFileSystemWatcher::~QFileSystemWatcher();
  fn _ZN18QFileSystemWatcherD0Ev(qthis: *mut c_void);
  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
  fn _ZN18QFileSystemWatcher10removePathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QStringList QFileSystemWatcher::directories();
  fn _ZNK18QFileSystemWatcher11directoriesEv(qthis: *mut c_void);
  // proto:  QStringList QFileSystemWatcher::files();
  fn _ZNK18QFileSystemWatcher5filesEv(qthis: *mut c_void);
  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
  fn _ZN18QFileSystemWatcher8addPathsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
  fn _ZN18QFileSystemWatcher11removePathsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFileSystemWatcher::QFileSystemWatcher(QObject * parent);
  fn _ZN18QFileSystemWatcherC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
  fn _ZN18QFileSystemWatcher7addPathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
  fn _ZNK18QFileSystemWatcher10metaObjectEv(qthis: *mut c_void);
  // proto:  void QFileSystemWatcher::QFileSystemWatcher(const QStringList & paths, QObject * parent);
  fn _ZN18QFileSystemWatcherC1ERK11QStringListP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
}

// body block begin
// class sizeof(QFileSystemWatcher)=1
pub struct QFileSystemWatcher {
  pub qclsinst: *mut c_void,
}

  // proto:  void QFileSystemWatcher::~QFileSystemWatcher();
impl /*struct*/ QFileSystemWatcher {
  pub fn FreeQFileSystemWatcher<RetType, T: QFileSystemWatcher_FreeQFileSystemWatcher<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQFileSystemWatcher(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_FreeQFileSystemWatcher<RetType> {
  fn FreeQFileSystemWatcher(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  void QFileSystemWatcher::~QFileSystemWatcher();
impl<'a> /*trait*/ QFileSystemWatcher_FreeQFileSystemWatcher<()> for () {
  fn FreeQFileSystemWatcher(self , rsthis: &mut QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcherD0Ev()};
     unsafe {_ZN18QFileSystemWatcherD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
impl /*struct*/ QFileSystemWatcher {
  pub fn removePath<RetType, T: QFileSystemWatcher_removePath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removePath(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_removePath<RetType> {
  fn removePath(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  bool QFileSystemWatcher::removePath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_removePath<i8> for (QString) {
  fn removePath(self , rsthis: &mut QFileSystemWatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher10removePathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QFileSystemWatcher10removePathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::directories();
impl /*struct*/ QFileSystemWatcher {
  pub fn directories<RetType, T: QFileSystemWatcher_directories<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.directories(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_directories<RetType> {
  fn directories(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::directories();
impl<'a> /*trait*/ QFileSystemWatcher_directories<()> for () {
  fn directories(self , rsthis: &mut QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher11directoriesEv()};
     unsafe {_ZNK18QFileSystemWatcher11directoriesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::files();
impl /*struct*/ QFileSystemWatcher {
  pub fn files<RetType, T: QFileSystemWatcher_files<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.files(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_files<RetType> {
  fn files(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::files();
impl<'a> /*trait*/ QFileSystemWatcher_files<()> for () {
  fn files(self , rsthis: &mut QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher5filesEv()};
     unsafe {_ZNK18QFileSystemWatcher5filesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
impl /*struct*/ QFileSystemWatcher {
  pub fn addPaths<RetType, T: QFileSystemWatcher_addPaths<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addPaths(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_addPaths<RetType> {
  fn addPaths(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::addPaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_addPaths<()> for (QStringList) {
  fn addPaths(self , rsthis: &mut QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher8addPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QFileSystemWatcher8addPathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
impl /*struct*/ QFileSystemWatcher {
  pub fn removePaths<RetType, T: QFileSystemWatcher_removePaths<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removePaths(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_removePaths<RetType> {
  fn removePaths(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  QStringList QFileSystemWatcher::removePaths(const QStringList & files);
impl<'a> /*trait*/ QFileSystemWatcher_removePaths<()> for (QStringList) {
  fn removePaths(self , rsthis: &mut QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher11removePathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QFileSystemWatcher11removePathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFileSystemWatcher::QFileSystemWatcher(QObject * parent);
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

  // proto:  void QFileSystemWatcher::QFileSystemWatcher(QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_NewQFileSystemWatcher for (QObject) {
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

  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
impl /*struct*/ QFileSystemWatcher {
  pub fn addPath<RetType, T: QFileSystemWatcher_addPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addPath(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_addPath<RetType> {
  fn addPath(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  bool QFileSystemWatcher::addPath(const QString & file);
impl<'a> /*trait*/ QFileSystemWatcher_addPath<i8> for (QString) {
  fn addPath(self , rsthis: &mut QFileSystemWatcher) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QFileSystemWatcher7addPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QFileSystemWatcher7addPathERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
impl /*struct*/ QFileSystemWatcher {
  pub fn metaObject<RetType, T: QFileSystemWatcher_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFileSystemWatcher_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QFileSystemWatcher) -> RetType;
}

  // proto:  const QMetaObject * QFileSystemWatcher::metaObject();
impl<'a> /*trait*/ QFileSystemWatcher_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QFileSystemWatcher) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QFileSystemWatcher10metaObjectEv()};
     unsafe {_ZNK18QFileSystemWatcher10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFileSystemWatcher::QFileSystemWatcher(const QStringList & paths, QObject * parent);
impl<'a> /*trait*/ QFileSystemWatcher_NewQFileSystemWatcher for (QStringList, QObject) {
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

