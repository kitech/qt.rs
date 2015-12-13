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
  fn _ZN13QTemporaryDir6removeEv() -> i32;
  fn _ZNK13QTemporaryDir10autoRemoveEv() -> i32;
  fn _ZNK13QTemporaryDir7isValidEv() -> i32;
  fn _ZN13QTemporaryDir13setAutoRemoveEb(arg0: int8_t) -> i32;
  fn _ZN13QTemporaryDirD0Ev() -> i32;
  fn _ZN13QTemporaryDirC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN13QTemporaryDirC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK13QTemporaryDir4pathEv() -> i32;
  fn _ZN13QTemporaryDirC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTemporaryDir)=1
pub struct QTemporaryDir {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTemporaryDir {
  pub fn remove<T: QTemporaryDir_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QTemporaryDir_remove {
  fn remove(self, this: &mut QTemporaryDir) -> i32;
}

// proto: bool QTemporaryDir::remove();
impl<'a> /*trait*/ QTemporaryDir_remove for () {
  fn remove(self, this: &mut QTemporaryDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir6removeEv()};
    unsafe {_ZN13QTemporaryDir6removeEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn autoRemove<T: QTemporaryDir_autoRemove>(&mut self, value: T) -> i32 {
    value.autoRemove(self);
    return 1;
  }
}

pub trait QTemporaryDir_autoRemove {
  fn autoRemove(self, this: &mut QTemporaryDir) -> i32;
}

// proto: bool QTemporaryDir::autoRemove();
impl<'a> /*trait*/ QTemporaryDir_autoRemove for () {
  fn autoRemove(self, this: &mut QTemporaryDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir10autoRemoveEv()};
    unsafe {_ZNK13QTemporaryDir10autoRemoveEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn isValid<T: QTemporaryDir_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTemporaryDir_isValid {
  fn isValid(self, this: &mut QTemporaryDir) -> i32;
}

// proto: bool QTemporaryDir::isValid();
impl<'a> /*trait*/ QTemporaryDir_isValid for () {
  fn isValid(self, this: &mut QTemporaryDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir7isValidEv()};
    unsafe {_ZNK13QTemporaryDir7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn setAutoRemove<T: QTemporaryDir_setAutoRemove>(&mut self, value: T) -> i32 {
    value.setAutoRemove(self);
    return 1;
  }
}

pub trait QTemporaryDir_setAutoRemove {
  fn setAutoRemove(self, this: &mut QTemporaryDir) -> i32;
}

// proto: void QTemporaryDir::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryDir_setAutoRemove for (i8) {
  fn setAutoRemove(self, this: &mut QTemporaryDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir13setAutoRemoveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN13QTemporaryDir13setAutoRemoveEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn FreeQTemporaryDir<T: QTemporaryDir_FreeQTemporaryDir>(&mut self, value: T) -> i32 {
    value.FreeQTemporaryDir(self);
    return 1;
  }
}

pub trait QTemporaryDir_FreeQTemporaryDir {
  fn FreeQTemporaryDir(self, this: &mut QTemporaryDir) -> i32;
}

// proto: void QTemporaryDir::FreeQTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_FreeQTemporaryDir for () {
  fn FreeQTemporaryDir(self, this: &mut QTemporaryDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirD0Ev()};
    unsafe {_ZN13QTemporaryDirD0Ev()};
    return 1;
  }
}

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

// proto: void QTemporaryDir::NewQTemporaryDir();
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

// proto: void QTemporaryDir::NewQTemporaryDir(const QString & templateName);
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for (&'a  QString) {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTemporaryDirC1ERK7QString(qthis, arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTemporaryDir {
  pub fn path<T: QTemporaryDir_path>(&mut self, value: T) -> i32 {
    value.path(self);
    return 1;
  }
}

pub trait QTemporaryDir_path {
  fn path(self, this: &mut QTemporaryDir) -> i32;
}

// proto: QString QTemporaryDir::path();
impl<'a> /*trait*/ QTemporaryDir_path for () {
  fn path(self, this: &mut QTemporaryDir) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir4pathEv()};
    unsafe {_ZNK13QTemporaryDir4pathEv()};
    return 1;
  }
}

// proto: void QTemporaryDir::NewQTemporaryDir(const QTemporaryDir & );
impl<'a> /*trait*/ QTemporaryDir_NewQTemporaryDir for (&'a  QTemporaryDir) {
  fn NewQTemporaryDir(self) -> QTemporaryDir {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QTemporaryDirC1ERKS_(qthis, arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

