// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTemporaryDir::remove();
  fn _ZN13QTemporaryDir6removeEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTemporaryDir::autoRemove();
  fn _ZNK13QTemporaryDir10autoRemoveEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTemporaryDir::isValid();
  fn _ZNK13QTemporaryDir7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTemporaryDir::setAutoRemove(bool b);
  fn _ZN13QTemporaryDir13setAutoRemoveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTemporaryDir::~QTemporaryDir();
  fn _ZN13QTemporaryDirD0Ev(qthis: *mut c_void);
  // proto:  void QTemporaryDir::QTemporaryDir();
  fn _ZN13QTemporaryDirC1Ev(qthis: *mut c_void);
  // proto:  void QTemporaryDir::QTemporaryDir(const QString & templateName);
  fn _ZN13QTemporaryDirC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTemporaryDir::path();
  fn _ZNK13QTemporaryDir4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryDir::QTemporaryDir(const QTemporaryDir & );
  fn _ZN13QTemporaryDirC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QTemporaryDir)=1
pub struct QTemporaryDir {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QTemporaryDir::remove();
impl /*struct*/ QTemporaryDir {
  pub fn remove<RetType, T: QTemporaryDir_remove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_remove<RetType> {
  fn remove(self , rsthis: &mut QTemporaryDir) -> RetType;
}

  // proto:  bool QTemporaryDir::remove();
impl<'a> /*trait*/ QTemporaryDir_remove<i8> for () {
  fn remove(self , rsthis: &mut QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir6removeEv()};
    let mut ret = unsafe {_ZN13QTemporaryDir6removeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTemporaryDir::autoRemove();
impl /*struct*/ QTemporaryDir {
  pub fn autoRemove<RetType, T: QTemporaryDir_autoRemove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_autoRemove<RetType> {
  fn autoRemove(self , rsthis: &mut QTemporaryDir) -> RetType;
}

  // proto:  bool QTemporaryDir::autoRemove();
impl<'a> /*trait*/ QTemporaryDir_autoRemove<i8> for () {
  fn autoRemove(self , rsthis: &mut QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir10autoRemoveEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir10autoRemoveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTemporaryDir::isValid();
impl /*struct*/ QTemporaryDir {
  pub fn isValid<RetType, T: QTemporaryDir_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTemporaryDir_isValid<RetType> {
  fn isValid(self , rsthis: &mut QTemporaryDir) -> RetType;
}

  // proto:  bool QTemporaryDir::isValid();
impl<'a> /*trait*/ QTemporaryDir_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir7isValidEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTemporaryDir::setAutoRemove(bool b);
impl /*struct*/ QTemporaryDir {
  pub fn setAutoRemove<RetType, T: QTemporaryDir_setAutoRemove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_setAutoRemove<RetType> {
  fn setAutoRemove(self , rsthis: &mut QTemporaryDir) -> RetType;
}

  // proto:  void QTemporaryDir::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryDir_setAutoRemove<()> for (i8) {
  fn setAutoRemove(self , rsthis: &mut QTemporaryDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir13setAutoRemoveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTemporaryDir13setAutoRemoveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTemporaryDir::~QTemporaryDir();
impl /*struct*/ QTemporaryDir {
  pub fn FreeQTemporaryDir<RetType, T: QTemporaryDir_FreeQTemporaryDir<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTemporaryDir(self);
    // return 1;
  }
}

pub trait QTemporaryDir_FreeQTemporaryDir<RetType> {
  fn FreeQTemporaryDir(self , rsthis: &mut QTemporaryDir) -> RetType;
}

  // proto:  void QTemporaryDir::~QTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_FreeQTemporaryDir<()> for () {
  fn FreeQTemporaryDir(self , rsthis: &mut QTemporaryDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirD0Ev()};
     unsafe {_ZN13QTemporaryDirD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTemporaryDir::QTemporaryDir();
impl /*struct*/ QTemporaryDir {
  pub fn NewQTemporaryDir<T: QTemporaryDir_NewQTemporaryDir>(value: T) -> QTemporaryDir {
    let rsthis = value.NewQTemporaryDir();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryDir_NewQTemporaryDir {
  fn NewQTemporaryDir(self) -> QTemporaryDir;
}

  // proto:  void QTemporaryDir::QTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for () {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1Ev()};
    unsafe {_ZN13QTemporaryDirC1Ev(qthis)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTemporaryDir::QTemporaryDir(const QString & templateName);
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for (QString) {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTemporaryDirC1ERK7QString(qthis, arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTemporaryDir::path();
impl /*struct*/ QTemporaryDir {
  pub fn path<RetType, T: QTemporaryDir_path<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QTemporaryDir_path<RetType> {
  fn path(self , rsthis: &mut QTemporaryDir) -> RetType;
}

  // proto:  QString QTemporaryDir::path();
impl<'a> /*trait*/ QTemporaryDir_path<QString> for () {
  fn path(self , rsthis: &mut QTemporaryDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir4pathEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTemporaryDir::QTemporaryDir(const QTemporaryDir & );
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for (QTemporaryDir) {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QTemporaryDirC1ERKS_(qthis, arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

