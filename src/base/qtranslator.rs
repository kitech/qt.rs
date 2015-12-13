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
use super::qlocale::QLocale;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK11QTranslator10metaObjectEv() -> i32;
  fn _ZN11QTranslatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK11QTranslator7isEmptyEv() -> i32;
  fn _ZN11QTranslatorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN11QTranslatorD0Ev() -> i32;
  fn _ZN11QTranslator4loadERK7QStringS2_S2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  fn _ZNK11QTranslator9translateEPKcS1_S1_i(arg0: *const c_char, arg1: *const c_char, arg2: *const c_char, arg3: c_int) -> i32;
  fn _ZN11QTranslator4loadEPKhiRK7QString(arg0: *const c_uchar, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTranslator)=1
pub struct QTranslator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTranslator {
  pub fn metaObject<T: QTranslator_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTranslator_metaObject {
  fn metaObject(self, this: &mut QTranslator) -> i32;
}

// proto: const QMetaObject * QTranslator::metaObject();
impl<'a> /*trait*/ QTranslator_metaObject for () {
  fn metaObject(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator10metaObjectEv()};
    unsafe {_ZNK11QTranslator10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn NewQTranslator<T: QTranslator_NewQTranslator>(value: T) -> QTranslator {
    let rsthis = value.NewQTranslator();
    return rsthis;
    // return 1;
  }
}

pub trait QTranslator_NewQTranslator {
  fn NewQTranslator(self) -> QTranslator;
}

// proto: void QTranslator::NewQTranslator(QObject * parent);
impl<'a> /*trait*/ QTranslator_NewQTranslator for (&'a mut QObject) {
  fn NewQTranslator(self) -> QTranslator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTranslatorC1EP7QObject(qthis, arg0)};
    let rsthis = QTranslator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn isEmpty<T: QTranslator_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QTranslator_isEmpty {
  fn isEmpty(self, this: &mut QTranslator) -> i32;
}

// proto: bool QTranslator::isEmpty();
impl<'a> /*trait*/ QTranslator_isEmpty for () {
  fn isEmpty(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator7isEmptyEv()};
    unsafe {_ZNK11QTranslator7isEmptyEv()};
    return 1;
  }
}

// proto: void QTranslator::NewQTranslator(const QTranslator & );
impl<'a> /*trait*/ QTranslator_NewQTranslator for (&'a  QTranslator) {
  fn NewQTranslator(self) -> QTranslator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QTranslatorC1ERKS_(qthis, arg0)};
    let rsthis = QTranslator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn FreeQTranslator<T: QTranslator_FreeQTranslator>(&mut self, value: T) -> i32 {
    value.FreeQTranslator(self);
    return 1;
  }
}

pub trait QTranslator_FreeQTranslator {
  fn FreeQTranslator(self, this: &mut QTranslator) -> i32;
}

// proto: void QTranslator::FreeQTranslator();
impl<'a> /*trait*/ QTranslator_FreeQTranslator for () {
  fn FreeQTranslator(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorD0Ev()};
    unsafe {_ZN11QTranslatorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn load<T: QTranslator_load>(&mut self, value: T) -> i32 {
    value.load(self);
    return 1;
  }
}

pub trait QTranslator_load {
  fn load(self, this: &mut QTranslator) -> i32;
}

// proto: bool QTranslator::load(const QString & filename, const QString & directory, const QString & search_delimiters, const QString & suffix);
impl<'a> /*trait*/ QTranslator_load for (&'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn load(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadERK7QStringS2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN11QTranslator4loadERK7QStringS2_S2_S2_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn translate<T: QTranslator_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QTranslator_translate {
  fn translate(self, this: &mut QTranslator) -> i32;
}

// proto: QString QTranslator::translate(const char * context, const char * sourceText, const char * disambiguation, int n);
impl<'a> /*trait*/ QTranslator_translate for (&'a  String, &'a  String, &'a  String, i32) {
  fn translate(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator9translateEPKcS1_S1_i()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK11QTranslator9translateEPKcS1_S1_i(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: bool QTranslator::load(const uchar * data, int len, const QString & directory);
impl<'a> /*trait*/ QTranslator_load for (&'a  String, i32, &'a  QString) {
  fn load(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadEPKhiRK7QString()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN11QTranslator4loadEPKhiRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: bool QTranslator::load(const QLocale & locale, const QString & filename, const QString & prefix, const QString & directory, const QString & suffix);
impl<'a> /*trait*/ QTranslator_load for (&'a  QLocale, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn load(self, this: &mut QTranslator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

