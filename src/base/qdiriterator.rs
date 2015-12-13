// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qdir::QDir;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK12QDirIterator8fileNameEv() -> i32;
  fn _ZN12QDirIteratorC1ERK7QString6QFlagsINS_12IteratorFlagEE(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZNK12QDirIterator4pathEv() -> i32;
  fn _ZN12QDirIteratorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN12QDirIteratorC1ERK4QDir6QFlagsINS_12IteratorFlagEE(qthis: *mut c_void, arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN12QDirIterator4nextEv() -> i32;
  fn _ZNK12QDirIterator8filePathEv() -> i32;
  fn _ZN12QDirIteratorD0Ev() -> i32;
  fn _ZNK12QDirIterator8fileInfoEv() -> i32;
  fn _ZNK12QDirIterator7hasNextEv() -> i32;
}

// body block begin
// class sizeof(QDirIterator)=1
pub struct QDirIterator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDirIterator {
  pub fn fileName<T: QDirIterator_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QDirIterator_fileName {
  fn fileName(self, this: &mut QDirIterator) -> i32;
}

// proto: QString QDirIterator::fileName();
impl<'a> /*trait*/ QDirIterator_fileName for () {
  fn fileName(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8fileNameEv()};
    unsafe {_ZNK12QDirIterator8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn NewQDirIterator<T: QDirIterator_NewQDirIterator>(value: T) -> QDirIterator {
    let rsthis = value.NewQDirIterator();
    return rsthis;
    // return 1;
  }
}

pub trait QDirIterator_NewQDirIterator {
  fn NewQDirIterator(self) -> QDirIterator;
}

// proto: void QDirIterator::NewQDirIterator(const QString & path, IteratorFlags flags);
impl<'a> /*trait*/ QDirIterator_NewQDirIterator for (&'a  QString, i32) {
  fn NewQDirIterator(self) -> QDirIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorC1ERK7QString6QFlagsINS_12IteratorFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QDirIteratorC1ERK7QString6QFlagsINS_12IteratorFlagEE(qthis, arg0, arg1)};
    let rsthis = QDirIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn path<T: QDirIterator_path>(&mut self, value: T) -> i32 {
    value.path(self);
    return 1;
  }
}

pub trait QDirIterator_path {
  fn path(self, this: &mut QDirIterator) -> i32;
}

// proto: QString QDirIterator::path();
impl<'a> /*trait*/ QDirIterator_path for () {
  fn path(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator4pathEv()};
    unsafe {_ZNK12QDirIterator4pathEv()};
    return 1;
  }
}

// proto: void QDirIterator::NewQDirIterator(const QDirIterator & );
impl<'a> /*trait*/ QDirIterator_NewQDirIterator for (&'a  QDirIterator) {
  fn NewQDirIterator(self) -> QDirIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QDirIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QDirIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QDirIterator::NewQDirIterator(const QDir & dir, IteratorFlags flags);
impl<'a> /*trait*/ QDirIterator_NewQDirIterator for (&'a  QDir, i32) {
  fn NewQDirIterator(self) -> QDirIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorC1ERK4QDir6QFlagsINS_12IteratorFlagEE()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QDirIteratorC1ERK4QDir6QFlagsINS_12IteratorFlagEE(qthis, arg0, arg1)};
    let rsthis = QDirIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn next<T: QDirIterator_next>(&mut self, value: T) -> i32 {
    value.next(self);
    return 1;
  }
}

pub trait QDirIterator_next {
  fn next(self, this: &mut QDirIterator) -> i32;
}

// proto: QString QDirIterator::next();
impl<'a> /*trait*/ QDirIterator_next for () {
  fn next(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIterator4nextEv()};
    unsafe {_ZN12QDirIterator4nextEv()};
    return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn filePath<T: QDirIterator_filePath>(&mut self, value: T) -> i32 {
    value.filePath(self);
    return 1;
  }
}

pub trait QDirIterator_filePath {
  fn filePath(self, this: &mut QDirIterator) -> i32;
}

// proto: QString QDirIterator::filePath();
impl<'a> /*trait*/ QDirIterator_filePath for () {
  fn filePath(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8filePathEv()};
    unsafe {_ZNK12QDirIterator8filePathEv()};
    return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn FreeQDirIterator<T: QDirIterator_FreeQDirIterator>(&mut self, value: T) -> i32 {
    value.FreeQDirIterator(self);
    return 1;
  }
}

pub trait QDirIterator_FreeQDirIterator {
  fn FreeQDirIterator(self, this: &mut QDirIterator) -> i32;
}

// proto: void QDirIterator::FreeQDirIterator();
impl<'a> /*trait*/ QDirIterator_FreeQDirIterator for () {
  fn FreeQDirIterator(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorD0Ev()};
    unsafe {_ZN12QDirIteratorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn fileInfo<T: QDirIterator_fileInfo>(&mut self, value: T) -> i32 {
    value.fileInfo(self);
    return 1;
  }
}

pub trait QDirIterator_fileInfo {
  fn fileInfo(self, this: &mut QDirIterator) -> i32;
}

// proto: QFileInfo QDirIterator::fileInfo();
impl<'a> /*trait*/ QDirIterator_fileInfo for () {
  fn fileInfo(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8fileInfoEv()};
    unsafe {_ZNK12QDirIterator8fileInfoEv()};
    return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn hasNext<T: QDirIterator_hasNext>(&mut self, value: T) -> i32 {
    value.hasNext(self);
    return 1;
  }
}

pub trait QDirIterator_hasNext {
  fn hasNext(self, this: &mut QDirIterator) -> i32;
}

// proto: bool QDirIterator::hasNext();
impl<'a> /*trait*/ QDirIterator_hasNext for () {
  fn hasNext(self, this: &mut QDirIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator7hasNextEv()};
    unsafe {_ZNK12QDirIterator7hasNextEv()};
    return 1;
  }
}

