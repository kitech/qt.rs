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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStandardPaths::NewQStandardPaths();
  fn _ZN14QStandardPathsC1Ev(qthis: *mut c_void) ;
  // proto:  void QStandardPaths::FreeQStandardPaths();
  fn _ZN14QStandardPathsD0Ev(qthis: *mut c_void) ;
  // proto: static void QStandardPaths::setTestModeEnabled(bool testMode);
  fn _ZN14QStandardPaths18setTestModeEnabledEb(arg0: int8_t) ;
  // proto: static QString QStandardPaths::findExecutable(const QString & executableName, const QStringList & paths);
  fn _ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static void QStandardPaths::enableTestMode(bool testMode);
  fn _ZN14QStandardPaths14enableTestModeEb(arg0: int8_t) ;
  // proto: static bool QStandardPaths::isTestModeEnabled();
  fn _ZN14QStandardPaths17isTestModeEnabledEv() -> int8_t;
}

// body block begin
// class sizeof(QStandardPaths)=1
pub struct QStandardPaths {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStandardPaths {
  pub fn NewQStandardPaths<T: QStandardPaths_NewQStandardPaths>(value: T) -> QStandardPaths {
    let rsthis = value.NewQStandardPaths();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardPaths_NewQStandardPaths {
  fn NewQStandardPaths(self) -> QStandardPaths;
}

// proto: void QStandardPaths::NewQStandardPaths();
impl<'a> /*trait*/ QStandardPaths_NewQStandardPaths for () {
  fn NewQStandardPaths(self) -> QStandardPaths {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPathsC1Ev()};
    unsafe {_ZN14QStandardPathsC1Ev(qthis)};
    let rsthis = QStandardPaths{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStandardPaths {
  pub fn FreeQStandardPaths<T: QStandardPaths_FreeQStandardPaths>(&mut self, value: T)  {
     value.FreeQStandardPaths(self);
    // return 1;
  }
}

pub trait QStandardPaths_FreeQStandardPaths {
  fn FreeQStandardPaths(self, rsthis: &mut QStandardPaths) ;
}

// proto:  void QStandardPaths::FreeQStandardPaths();
impl<'a> /*trait*/ QStandardPaths_FreeQStandardPaths for () {
  fn FreeQStandardPaths(self, rsthis: &mut QStandardPaths)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPathsD0Ev()};
     unsafe {_ZN14QStandardPathsD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStandardPaths {
  pub fn setTestModeEnabled<T: QStandardPaths_setTestModeEnabled>(&mut self, value: T)  {
     value.setTestModeEnabled(self);
    // return 1;
  }
}

pub trait QStandardPaths_setTestModeEnabled {
  fn setTestModeEnabled(self, rsthis: &mut QStandardPaths) ;
}

// proto: static void QStandardPaths::setTestModeEnabled(bool testMode);
impl<'a> /*trait*/ QStandardPaths_setTestModeEnabled for (i8) {
  fn setTestModeEnabled(self, rsthis: &mut QStandardPaths)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths18setTestModeEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QStandardPaths18setTestModeEnabledEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardPaths {
  pub fn findExecutable<T: QStandardPaths_findExecutable>(&mut self, value: T) -> QString {
    return value.findExecutable(self);
    // return 1;
  }
}

pub trait QStandardPaths_findExecutable {
  fn findExecutable(self, rsthis: &mut QStandardPaths) -> QString;
}

// proto: static QString QStandardPaths::findExecutable(const QString & executableName, const QStringList & paths);
impl<'a> /*trait*/ QStandardPaths_findExecutable for (&'a  QString, &'a  QStringList) {
  fn findExecutable(self, rsthis: &mut QStandardPaths) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList(arg0, arg1)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStandardPaths {
  pub fn enableTestMode<T: QStandardPaths_enableTestMode>(&mut self, value: T)  {
     value.enableTestMode(self);
    // return 1;
  }
}

pub trait QStandardPaths_enableTestMode {
  fn enableTestMode(self, rsthis: &mut QStandardPaths) ;
}

// proto: static void QStandardPaths::enableTestMode(bool testMode);
impl<'a> /*trait*/ QStandardPaths_enableTestMode for (i8) {
  fn enableTestMode(self, rsthis: &mut QStandardPaths)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths14enableTestModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN14QStandardPaths14enableTestModeEb(arg0)};
    // return 1;
  }
}

impl /*struct*/ QStandardPaths {
  pub fn isTestModeEnabled<T: QStandardPaths_isTestModeEnabled>(&mut self, value: T) -> i8 {
    return value.isTestModeEnabled(self);
    // return 1;
  }
}

pub trait QStandardPaths_isTestModeEnabled {
  fn isTestModeEnabled(self, rsthis: &mut QStandardPaths) -> i8;
}

// proto: static bool QStandardPaths::isTestModeEnabled();
impl<'a> /*trait*/ QStandardPaths_isTestModeEnabled for () {
  fn isTestModeEnabled(self, rsthis: &mut QStandardPaths) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths17isTestModeEnabledEv()};
    let mut ret = unsafe {_ZN14QStandardPaths17isTestModeEnabledEv()};
    return ret as i8;
    // return 1;
  }
}

