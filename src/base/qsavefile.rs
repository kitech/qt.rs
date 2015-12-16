// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSaveFile::cancelWriting();
  fn _ZN9QSaveFile13cancelWritingEv(qthis: *mut c_void) ;
  // proto:  void QSaveFile::NewQSaveFile(QObject * parent);
  fn _ZN9QSaveFileC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSaveFile::NewQSaveFile(const QSaveFile & );
  fn _ZN9QSaveFileC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSaveFile::NewQSaveFile(const QString & name, QObject * parent);
  fn _ZN9QSaveFileC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QSaveFile::fileName();
  fn _ZNK9QSaveFile8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSaveFile::NewQSaveFile(const QString & name);
  fn _ZN9QSaveFileC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QSaveFile::metaObject();
  fn _ZNK9QSaveFile10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QSaveFile::commit();
  fn _ZN9QSaveFile6commitEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSaveFile::FreeQSaveFile();
  fn _ZN9QSaveFileD0Ev(qthis: *mut c_void) ;
  // proto:  void QSaveFile::setFileName(const QString & name);
  fn _ZN9QSaveFile11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSaveFile::directWriteFallback();
  fn _ZNK9QSaveFile19directWriteFallbackEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
  fn _ZN9QSaveFile22setDirectWriteFallbackEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QSaveFile)=1
pub struct QSaveFile {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSaveFile {
  pub fn cancelWriting<T: QSaveFile_cancelWriting>(&mut self, value: T)  {
     value.cancelWriting(self);
    // return 1;
  }
}

pub trait QSaveFile_cancelWriting {
  fn cancelWriting(self, rsthis: &mut QSaveFile) ;
}

// proto:  void QSaveFile::cancelWriting();
impl<'a> /*trait*/ QSaveFile_cancelWriting for () {
  fn cancelWriting(self, rsthis: &mut QSaveFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile13cancelWritingEv()};
     unsafe {_ZN9QSaveFile13cancelWritingEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn NewQSaveFile<T: QSaveFile_NewQSaveFile>(value: T) -> QSaveFile {
    let rsthis = value.NewQSaveFile();
    return rsthis;
    // return 1;
  }
}

pub trait QSaveFile_NewQSaveFile {
  fn NewQSaveFile(self) -> QSaveFile;
}

// proto: void QSaveFile::NewQSaveFile(QObject * parent);
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (&'a mut QObject) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1EP7QObject(qthis, arg0)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSaveFile::NewQSaveFile(const QSaveFile & );
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (&'a  QSaveFile) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1ERKS_(qthis, arg0)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSaveFile::NewQSaveFile(const QString & name, QObject * parent);
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (&'a  QString, &'a mut QObject) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn fileName<T: QSaveFile_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QSaveFile_fileName {
  fn fileName(self, rsthis: &mut QSaveFile) -> QString;
}

// proto:  QString QSaveFile::fileName();
impl<'a> /*trait*/ QSaveFile_fileName for () {
  fn fileName(self, rsthis: &mut QSaveFile) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile8fileNameEv()};
    let mut ret = unsafe {_ZNK9QSaveFile8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QSaveFile::NewQSaveFile(const QString & name);
impl<'a> /*trait*/ QSaveFile_NewQSaveFile for (&'a  QString) {
  fn NewQSaveFile(self) -> QSaveFile {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QSaveFileC1ERK7QString(qthis, arg0)};
    let rsthis = QSaveFile{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn metaObject<T: QSaveFile_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSaveFile_metaObject {
  fn metaObject(self, rsthis: &mut QSaveFile) ;
}

// proto:  const QMetaObject * QSaveFile::metaObject();
impl<'a> /*trait*/ QSaveFile_metaObject for () {
  fn metaObject(self, rsthis: &mut QSaveFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile10metaObjectEv()};
     unsafe {_ZNK9QSaveFile10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn commit<T: QSaveFile_commit>(&mut self, value: T) -> i8 {
    return value.commit(self);
    // return 1;
  }
}

pub trait QSaveFile_commit {
  fn commit(self, rsthis: &mut QSaveFile) -> i8;
}

// proto:  bool QSaveFile::commit();
impl<'a> /*trait*/ QSaveFile_commit for () {
  fn commit(self, rsthis: &mut QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile6commitEv()};
    let mut ret = unsafe {_ZN9QSaveFile6commitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn FreeQSaveFile<T: QSaveFile_FreeQSaveFile>(&mut self, value: T)  {
     value.FreeQSaveFile(self);
    // return 1;
  }
}

pub trait QSaveFile_FreeQSaveFile {
  fn FreeQSaveFile(self, rsthis: &mut QSaveFile) ;
}

// proto:  void QSaveFile::FreeQSaveFile();
impl<'a> /*trait*/ QSaveFile_FreeQSaveFile for () {
  fn FreeQSaveFile(self, rsthis: &mut QSaveFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFileD0Ev()};
     unsafe {_ZN9QSaveFileD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn setFileName<T: QSaveFile_setFileName>(&mut self, value: T)  {
     value.setFileName(self);
    // return 1;
  }
}

pub trait QSaveFile_setFileName {
  fn setFileName(self, rsthis: &mut QSaveFile) ;
}

// proto:  void QSaveFile::setFileName(const QString & name);
impl<'a> /*trait*/ QSaveFile_setFileName for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QSaveFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QSaveFile11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn directWriteFallback<T: QSaveFile_directWriteFallback>(&mut self, value: T) -> i8 {
    return value.directWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_directWriteFallback {
  fn directWriteFallback(self, rsthis: &mut QSaveFile) -> i8;
}

// proto:  bool QSaveFile::directWriteFallback();
impl<'a> /*trait*/ QSaveFile_directWriteFallback for () {
  fn directWriteFallback(self, rsthis: &mut QSaveFile) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSaveFile19directWriteFallbackEv()};
    let mut ret = unsafe {_ZNK9QSaveFile19directWriteFallbackEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSaveFile {
  pub fn setDirectWriteFallback<T: QSaveFile_setDirectWriteFallback>(&mut self, value: T)  {
     value.setDirectWriteFallback(self);
    // return 1;
  }
}

pub trait QSaveFile_setDirectWriteFallback {
  fn setDirectWriteFallback(self, rsthis: &mut QSaveFile) ;
}

// proto:  void QSaveFile::setDirectWriteFallback(bool enabled);
impl<'a> /*trait*/ QSaveFile_setDirectWriteFallback for (i8) {
  fn setDirectWriteFallback(self, rsthis: &mut QSaveFile)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSaveFile22setDirectWriteFallbackEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QSaveFile22setDirectWriteFallbackEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

