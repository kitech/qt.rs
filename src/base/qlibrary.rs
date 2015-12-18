// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QLibrary::NewQLibrary(const QString & fileName, const QString & version, QObject * parent);
  fn _ZN8QLibraryC1ERK7QStringS2_P7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  const QMetaObject * QLibrary::metaObject();
  fn _ZNK8QLibrary10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QLibrary::NewQLibrary(QObject * parent);
  fn _ZN8QLibraryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QLibrary::isLoaded();
  fn _ZNK8QLibrary8isLoadedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QLibrary::load();
  fn _ZN8QLibrary4loadEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLibrary::NewQLibrary(const QString & fileName, QObject * parent);
  fn _ZN8QLibraryC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QLibrary::fileName();
  fn _ZNK8QLibrary8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLibrary::setFileName(const QString & fileName);
  fn _ZN8QLibrary11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLibrary::NewQLibrary(const QLibrary & );
  fn _ZN8QLibraryC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static bool QLibrary::isLibrary(const QString & fileName);
  fn _ZN8QLibrary9isLibraryERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  bool QLibrary::unload();
  fn _ZN8QLibrary6unloadEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, const QString & version);
  fn _ZN8QLibrary21setFileNameAndVersionERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, int verNum);
  fn _ZN8QLibrary21setFileNameAndVersionERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QLibrary::NewQLibrary(const QString & fileName, int verNum, QObject * parent);
  fn _ZN8QLibraryC1ERK7QStringiP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QLibrary::FreeQLibrary();
  fn _ZN8QLibraryD0Ev(qthis: *mut c_void) ;
  // proto:  QString QLibrary::errorString();
  fn _ZNK8QLibrary11errorStringEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QLibrary)=1
pub struct QLibrary {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLibrary {
  pub fn NewQLibrary<T: QLibrary_NewQLibrary>(value: T) -> QLibrary {
    let rsthis = value.NewQLibrary();
    return rsthis;
    // return 1;
  }
}

pub trait QLibrary_NewQLibrary {
  fn NewQLibrary(self) -> QLibrary;
}

// proto: void QLibrary::NewQLibrary(const QString & fileName, const QString & version, QObject * parent);
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QString, &'a  QString, &'a mut QObject) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringS2_P7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERK7QStringS2_P7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn metaObject<RetType, T: QLibrary_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QLibrary_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  const QMetaObject * QLibrary::metaObject();
impl<'a> /*trait*/ QLibrary_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary10metaObjectEv()};
     unsafe {_ZNK8QLibrary10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QLibrary::NewQLibrary(QObject * parent);
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a mut QObject) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1EP7QObject(qthis, arg0)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn isLoaded<RetType, T: QLibrary_isLoaded<RetType>>(&mut self, value: T) -> RetType {
    return value.isLoaded(self);
    // return 1;
  }
}

pub trait QLibrary_isLoaded<RetType> {
  fn isLoaded(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  bool QLibrary::isLoaded();
impl<'a> /*trait*/ QLibrary_isLoaded<i8> for () {
  fn isLoaded(self, rsthis: &mut QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary8isLoadedEv()};
    let mut ret = unsafe {_ZNK8QLibrary8isLoadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn load<RetType, T: QLibrary_load<RetType>>(&mut self, value: T) -> RetType {
    return value.load(self);
    // return 1;
  }
}

pub trait QLibrary_load<RetType> {
  fn load(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  bool QLibrary::load();
impl<'a> /*trait*/ QLibrary_load<i8> for () {
  fn load(self, rsthis: &mut QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary4loadEv()};
    let mut ret = unsafe {_ZN8QLibrary4loadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QLibrary::NewQLibrary(const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QString, &'a mut QObject) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn fileName<RetType, T: QLibrary_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QLibrary_fileName<RetType> {
  fn fileName(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  QString QLibrary::fileName();
impl<'a> /*trait*/ QLibrary_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QLibrary) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary8fileNameEv()};
    let mut ret = unsafe {_ZNK8QLibrary8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn setFileName<RetType, T: QLibrary_setFileName<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileName(self);
    // return 1;
  }
}

pub trait QLibrary_setFileName<RetType> {
  fn setFileName(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  void QLibrary::setFileName(const QString & fileName);
impl<'a> /*trait*/ QLibrary_setFileName<()> for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QLibrary11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QLibrary::NewQLibrary(const QLibrary & );
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QLibrary) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERKS_(qthis, arg0)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn isLibrary<RetType, T: QLibrary_isLibrary<RetType>>(&mut self, value: T) -> RetType {
    return value.isLibrary(self);
    // return 1;
  }
}

pub trait QLibrary_isLibrary<RetType> {
  fn isLibrary(self, rsthis: &mut QLibrary) -> RetType;
}

// proto: static bool QLibrary::isLibrary(const QString & fileName);
impl<'a> /*trait*/ QLibrary_isLibrary<i8> for (&'a  QString) {
  fn isLibrary(self, rsthis: &mut QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary9isLibraryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QLibrary9isLibraryERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn unload<RetType, T: QLibrary_unload<RetType>>(&mut self, value: T) -> RetType {
    return value.unload(self);
    // return 1;
  }
}

pub trait QLibrary_unload<RetType> {
  fn unload(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  bool QLibrary::unload();
impl<'a> /*trait*/ QLibrary_unload<i8> for () {
  fn unload(self, rsthis: &mut QLibrary) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary6unloadEv()};
    let mut ret = unsafe {_ZN8QLibrary6unloadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn setFileNameAndVersion<RetType, T: QLibrary_setFileNameAndVersion<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileNameAndVersion(self);
    // return 1;
  }
}

pub trait QLibrary_setFileNameAndVersion<RetType> {
  fn setFileNameAndVersion(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, const QString & version);
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion<()> for (&'a  QString, &'a  QString) {
  fn setFileNameAndVersion(self, rsthis: &mut QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QLibrary::setFileNameAndVersion(const QString & fileName, int verNum);
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion<()> for (&'a  QString, i32) {
  fn setFileNameAndVersion(self, rsthis: &mut QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary21setFileNameAndVersionERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QLibrary21setFileNameAndVersionERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QLibrary::NewQLibrary(const QString & fileName, int verNum, QObject * parent);
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QString, i32, &'a mut QObject) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringiP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERK7QStringiP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn FreeQLibrary<RetType, T: QLibrary_FreeQLibrary<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQLibrary(self);
    // return 1;
  }
}

pub trait QLibrary_FreeQLibrary<RetType> {
  fn FreeQLibrary(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  void QLibrary::FreeQLibrary();
impl<'a> /*trait*/ QLibrary_FreeQLibrary<()> for () {
  fn FreeQLibrary(self, rsthis: &mut QLibrary) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryD0Ev()};
     unsafe {_ZN8QLibraryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn errorString<RetType, T: QLibrary_errorString<RetType>>(&mut self, value: T) -> RetType {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QLibrary_errorString<RetType> {
  fn errorString(self, rsthis: &mut QLibrary) -> RetType;
}

// proto:  QString QLibrary::errorString();
impl<'a> /*trait*/ QLibrary_errorString<QString> for () {
  fn errorString(self, rsthis: &mut QLibrary) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary11errorStringEv()};
    let mut ret = unsafe {_ZNK8QLibrary11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

