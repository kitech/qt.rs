// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qfileinfo::QFileInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QDirIterator::fileName();
  fn _ZNK12QDirIterator8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QDirIterator::path();
  fn _ZNK12QDirIterator4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDirIterator::NewQDirIterator(const QDirIterator & );
  fn _ZN12QDirIteratorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QDirIterator::next();
  fn _ZN12QDirIterator4nextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QDirIterator::filePath();
  fn _ZNK12QDirIterator8filePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDirIterator::FreeQDirIterator();
  fn _ZN12QDirIteratorD0Ev(qthis: *mut c_void) ;
  // proto:  QFileInfo QDirIterator::fileInfo();
  fn _ZNK12QDirIterator8fileInfoEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDirIterator::hasNext();
  fn _ZNK12QDirIterator7hasNextEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QDirIterator)=1
pub struct QDirIterator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDirIterator {
  pub fn fileName<T: QDirIterator_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QDirIterator_fileName {
  fn fileName(self, rsthis: &mut QDirIterator) -> QString;
}

// proto:  QString QDirIterator::fileName();
impl<'a> /*trait*/ QDirIterator_fileName for () {
  fn fileName(self, rsthis: &mut QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8fileNameEv()};
    let mut ret = unsafe {_ZNK12QDirIterator8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn path<T: QDirIterator_path>(&mut self, value: T) -> QString {
    return value.path(self);
    // return 1;
  }
}

pub trait QDirIterator_path {
  fn path(self, rsthis: &mut QDirIterator) -> QString;
}

// proto:  QString QDirIterator::path();
impl<'a> /*trait*/ QDirIterator_path for () {
  fn path(self, rsthis: &mut QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator4pathEv()};
    let mut ret = unsafe {_ZNK12QDirIterator4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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

// proto: void QDirIterator::NewQDirIterator(const QDirIterator & );
impl<'a> /*trait*/ QDirIterator_NewQDirIterator for (&'a  QDirIterator) {
  fn NewQDirIterator(self) -> QDirIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QDirIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QDirIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn next<T: QDirIterator_next>(&mut self, value: T) -> QString {
    return value.next(self);
    // return 1;
  }
}

pub trait QDirIterator_next {
  fn next(self, rsthis: &mut QDirIterator) -> QString;
}

// proto:  QString QDirIterator::next();
impl<'a> /*trait*/ QDirIterator_next for () {
  fn next(self, rsthis: &mut QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIterator4nextEv()};
    let mut ret = unsafe {_ZN12QDirIterator4nextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn filePath<T: QDirIterator_filePath>(&mut self, value: T) -> QString {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QDirIterator_filePath {
  fn filePath(self, rsthis: &mut QDirIterator) -> QString;
}

// proto:  QString QDirIterator::filePath();
impl<'a> /*trait*/ QDirIterator_filePath for () {
  fn filePath(self, rsthis: &mut QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8filePathEv()};
    let mut ret = unsafe {_ZNK12QDirIterator8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn FreeQDirIterator<T: QDirIterator_FreeQDirIterator>(&mut self, value: T)  {
     value.FreeQDirIterator(self);
    // return 1;
  }
}

pub trait QDirIterator_FreeQDirIterator {
  fn FreeQDirIterator(self, rsthis: &mut QDirIterator) ;
}

// proto:  void QDirIterator::FreeQDirIterator();
impl<'a> /*trait*/ QDirIterator_FreeQDirIterator for () {
  fn FreeQDirIterator(self, rsthis: &mut QDirIterator)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorD0Ev()};
     unsafe {_ZN12QDirIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn fileInfo<T: QDirIterator_fileInfo>(&mut self, value: T) -> QFileInfo {
    return value.fileInfo(self);
    // return 1;
  }
}

pub trait QDirIterator_fileInfo {
  fn fileInfo(self, rsthis: &mut QDirIterator) -> QFileInfo;
}

// proto:  QFileInfo QDirIterator::fileInfo();
impl<'a> /*trait*/ QDirIterator_fileInfo for () {
  fn fileInfo(self, rsthis: &mut QDirIterator) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8fileInfoEv()};
    let mut ret = unsafe {_ZNK12QDirIterator8fileInfoEv(rsthis.qclsinst)};
    let mut ret1 = QFileInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirIterator {
  pub fn hasNext<T: QDirIterator_hasNext>(&mut self, value: T) -> i8 {
    return value.hasNext(self);
    // return 1;
  }
}

pub trait QDirIterator_hasNext {
  fn hasNext(self, rsthis: &mut QDirIterator) -> i8;
}

// proto:  bool QDirIterator::hasNext();
impl<'a> /*trait*/ QDirIterator_hasNext for () {
  fn hasNext(self, rsthis: &mut QDirIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator7hasNextEv()};
    let mut ret = unsafe {_ZNK12QDirIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

