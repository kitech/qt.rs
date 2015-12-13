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
  fn _ZN8QLibrary7resolveERK7QStringS2_PKc(arg0: *const c_void, arg1: *const c_void, arg2: *const c_char) -> i32;
  fn _ZN8QLibraryC1ERK7QStringS2_P7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  fn _ZNK8QLibrary10metaObjectEv() -> i32;
  fn _ZN8QLibrary7resolveERK7QStringPKc(arg0: *const c_void, arg1: *const c_char) -> i32;
  fn _ZN8QLibraryC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK8QLibrary8isLoadedEv() -> i32;
  fn _ZN8QLibrary4loadEv() -> i32;
  fn _ZN8QLibraryC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  fn _ZNK8QLibrary8fileNameEv() -> i32;
  fn _ZN8QLibrary11setFileNameERK7QString(arg0: *const c_void) -> i32;
  fn _ZN8QLibraryC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QLibrary9isLibraryERK7QString(arg0: *const c_void) -> i32;
  fn _ZN8QLibrary6unloadEv() -> i32;
  fn _ZN8QLibrary7resolveEPKc(arg0: *const c_char) -> i32;
  fn _ZN8QLibrary12setLoadHintsE6QFlagsINS_8LoadHintEE(arg0: c_int) -> i32;
  fn _ZN8QLibrary21setFileNameAndVersionERK7QStringS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN8QLibrary21setFileNameAndVersionERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN8QLibraryC1ERK7QStringiP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: c_int, arg2: *mut c_void) -> i32;
  fn _ZN8QLibraryD0Ev() -> i32;
  fn _ZNK8QLibrary11errorStringEv() -> i32;
  fn _ZN8QLibrary7resolveERK7QStringiPKc(arg0: *const c_void, arg1: c_int, arg2: *const c_char) -> i32;
}

// body block begin
// class sizeof(QLibrary)=1
pub struct QLibrary {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLibrary {
  pub fn resolve<T: QLibrary_resolve>(&mut self, value: T) -> i32 {
    value.resolve(self);
    return 1;
  }
}

pub trait QLibrary_resolve {
  fn resolve(self, this: &mut QLibrary) -> i32;
}

// proto: QFunctionPointer QLibrary::resolve(const QString & fileName, const QString & version, const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve for (&'a  QString, &'a  QString, &'a  String) {
  fn resolve(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveERK7QStringS2_PKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN8QLibrary7resolveERK7QStringS2_PKc(arg0, arg1, arg2)};
    return 1;
  }
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERK7QStringS2_P7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn metaObject<T: QLibrary_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QLibrary_metaObject {
  fn metaObject(self, this: &mut QLibrary) -> i32;
}

// proto: const QMetaObject * QLibrary::metaObject();
impl<'a> /*trait*/ QLibrary_metaObject for () {
  fn metaObject(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary10metaObjectEv()};
    unsafe {_ZNK8QLibrary10metaObjectEv()};
    return 1;
  }
}

// proto: QFunctionPointer QLibrary::resolve(const QString & fileName, const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve for (&'a  QString, &'a  String) {
  fn resolve(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveERK7QStringPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN8QLibrary7resolveERK7QStringPKc(arg0, arg1)};
    return 1;
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
  pub fn isLoaded<T: QLibrary_isLoaded>(&mut self, value: T) -> i32 {
    value.isLoaded(self);
    return 1;
  }
}

pub trait QLibrary_isLoaded {
  fn isLoaded(self, this: &mut QLibrary) -> i32;
}

// proto: bool QLibrary::isLoaded();
impl<'a> /*trait*/ QLibrary_isLoaded for () {
  fn isLoaded(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary8isLoadedEv()};
    unsafe {_ZNK8QLibrary8isLoadedEv()};
    return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn load<T: QLibrary_load>(&mut self, value: T) -> i32 {
    value.load(self);
    return 1;
  }
}

pub trait QLibrary_load {
  fn load(self, this: &mut QLibrary) -> i32;
}

// proto: bool QLibrary::load();
impl<'a> /*trait*/ QLibrary_load for () {
  fn load(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary4loadEv()};
    unsafe {_ZN8QLibrary4loadEv()};
    return 1;
  }
}

// proto: void QLibrary::NewQLibrary(const QString & fileName, QObject * parent);
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QString, &'a mut QObject) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn fileName<T: QLibrary_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QLibrary_fileName {
  fn fileName(self, this: &mut QLibrary) -> i32;
}

// proto: QString QLibrary::fileName();
impl<'a> /*trait*/ QLibrary_fileName for () {
  fn fileName(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary8fileNameEv()};
    unsafe {_ZNK8QLibrary8fileNameEv()};
    return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn setFileName<T: QLibrary_setFileName>(&mut self, value: T) -> i32 {
    value.setFileName(self);
    return 1;
  }
}

pub trait QLibrary_setFileName {
  fn setFileName(self, this: &mut QLibrary) -> i32;
}

// proto: void QLibrary::setFileName(const QString & fileName);
impl<'a> /*trait*/ QLibrary_setFileName for (&'a  QString) {
  fn setFileName(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QLibrary11setFileNameERK7QString(arg0)};
    return 1;
  }
}

// proto: void QLibrary::NewQLibrary(const QLibrary & );
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QLibrary) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QLibraryC1ERKS_(qthis, arg0)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn isLibrary<T: QLibrary_isLibrary>(&mut self, value: T) -> i32 {
    value.isLibrary(self);
    return 1;
  }
}

pub trait QLibrary_isLibrary {
  fn isLibrary(self, this: &mut QLibrary) -> i32;
}

// proto: bool QLibrary::isLibrary(const QString & fileName);
impl<'a> /*trait*/ QLibrary_isLibrary for (&'a  QString) {
  fn isLibrary(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary9isLibraryERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QLibrary9isLibraryERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn unload<T: QLibrary_unload>(&mut self, value: T) -> i32 {
    value.unload(self);
    return 1;
  }
}

pub trait QLibrary_unload {
  fn unload(self, this: &mut QLibrary) -> i32;
}

// proto: bool QLibrary::unload();
impl<'a> /*trait*/ QLibrary_unload for () {
  fn unload(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary6unloadEv()};
    unsafe {_ZN8QLibrary6unloadEv()};
    return 1;
  }
}

// proto: QFunctionPointer QLibrary::resolve(const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve for (&'a  String) {
  fn resolve(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveEPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN8QLibrary7resolveEPKc(arg0)};
    return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn setLoadHints<T: QLibrary_setLoadHints>(&mut self, value: T) -> i32 {
    value.setLoadHints(self);
    return 1;
  }
}

pub trait QLibrary_setLoadHints {
  fn setLoadHints(self, this: &mut QLibrary) -> i32;
}

// proto: void QLibrary::setLoadHints(LoadHints hints);
impl<'a> /*trait*/ QLibrary_setLoadHints for (i32) {
  fn setLoadHints(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary12setLoadHintsE6QFlagsINS_8LoadHintEE()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QLibrary12setLoadHintsE6QFlagsINS_8LoadHintEE(arg0)};
    return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn setFileNameAndVersion<T: QLibrary_setFileNameAndVersion>(&mut self, value: T) -> i32 {
    value.setFileNameAndVersion(self);
    return 1;
  }
}

pub trait QLibrary_setFileNameAndVersion {
  fn setFileNameAndVersion(self, this: &mut QLibrary) -> i32;
}

// proto: void QLibrary::setFileNameAndVersion(const QString & fileName, const QString & version);
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion for (&'a  QString, &'a  QString) {
  fn setFileNameAndVersion(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QLibrary::setFileNameAndVersion(const QString & fileName, int verNum);
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion for (&'a  QString, i32) {
  fn setFileNameAndVersion(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary21setFileNameAndVersionERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QLibrary21setFileNameAndVersionERK7QStringi(arg0, arg1)};
    return 1;
  }
}

// proto: void QLibrary::NewQLibrary(const QString & fileName, int verNum, QObject * parent);
impl<'a> /*trait*/ QLibrary_NewQLibrary for (&'a  QString, i32, &'a mut QObject) {
  fn NewQLibrary(self) -> QLibrary {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryC1ERK7QStringiP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN8QLibraryC1ERK7QStringiP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QLibrary{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn FreeQLibrary<T: QLibrary_FreeQLibrary>(&mut self, value: T) -> i32 {
    value.FreeQLibrary(self);
    return 1;
  }
}

pub trait QLibrary_FreeQLibrary {
  fn FreeQLibrary(self, this: &mut QLibrary) -> i32;
}

// proto: void QLibrary::FreeQLibrary();
impl<'a> /*trait*/ QLibrary_FreeQLibrary for () {
  fn FreeQLibrary(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibraryD0Ev()};
    unsafe {_ZN8QLibraryD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLibrary {
  pub fn errorString<T: QLibrary_errorString>(&mut self, value: T) -> i32 {
    value.errorString(self);
    return 1;
  }
}

pub trait QLibrary_errorString {
  fn errorString(self, this: &mut QLibrary) -> i32;
}

// proto: QString QLibrary::errorString();
impl<'a> /*trait*/ QLibrary_errorString for () {
  fn errorString(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QLibrary11errorStringEv()};
    unsafe {_ZNK8QLibrary11errorStringEv()};
    return 1;
  }
}

// proto: QFunctionPointer QLibrary::resolve(const QString & fileName, int verNum, const char * symbol);
impl<'a> /*trait*/ QLibrary_resolve for (&'a  QString, i32, &'a  String) {
  fn resolve(self, this: &mut QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QLibrary7resolveERK7QStringiPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN8QLibrary7resolveERK7QStringiPKc(arg0, arg1, arg2)};
    return 1;
  }
}

