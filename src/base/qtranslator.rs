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
  // proto:  const QMetaObject * QTranslator::metaObject();
  fn _ZNK11QTranslator10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTranslator::NewQTranslator(QObject * parent);
  fn _ZN11QTranslatorC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTranslator::isEmpty();
  fn _ZNK11QTranslator7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTranslator::NewQTranslator(const QTranslator & );
  fn _ZN11QTranslatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTranslator::FreeQTranslator();
  fn _ZN11QTranslatorD0Ev(qthis: *mut c_void) ;
  // proto:  bool QTranslator::load(const QString & filename, const QString & directory, const QString & search_delimiters, const QString & suffix);
  fn _ZN11QTranslator4loadERK7QStringS2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> int8_t;
  // proto:  QString QTranslator::translate(const char * context, const char * sourceText, const char * disambiguation, int n);
  fn _ZNK11QTranslator9translateEPKcS1_S1_i(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_char, arg2: *const c_char, arg3: c_int) -> *mut c_void;
  // proto:  bool QTranslator::load(const uchar * data, int len, const QString & directory);
  fn _ZN11QTranslator4loadEPKhiRK7QString(qthis: *mut c_void, arg0: *const c_uchar, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  bool QTranslator::load(const QLocale & locale, const QString & filename, const QString & prefix, const QString & directory, const QString & suffix);
  fn _ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QTranslator)=1
pub struct QTranslator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTranslator {
  pub fn metaObject<T: QTranslator_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTranslator_metaObject {
  fn metaObject(self, rsthis: &mut QTranslator) ;
}

// proto:  const QMetaObject * QTranslator::metaObject();
impl<'a> /*trait*/ QTranslator_metaObject for () {
  fn metaObject(self, rsthis: &mut QTranslator)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator10metaObjectEv()};
     unsafe {_ZNK11QTranslator10metaObjectEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn isEmpty<T: QTranslator_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QTranslator_isEmpty {
  fn isEmpty(self, rsthis: &mut QTranslator) -> i8;
}

// proto:  bool QTranslator::isEmpty();
impl<'a> /*trait*/ QTranslator_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QTranslator7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QTranslator::NewQTranslator(const QTranslator & );
impl<'a> /*trait*/ QTranslator_NewQTranslator for (&'a  QTranslator) {
  fn NewQTranslator(self) -> QTranslator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QTranslatorC1ERKS_(qthis, arg0)};
    let rsthis = QTranslator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn FreeQTranslator<T: QTranslator_FreeQTranslator>(&mut self, value: T)  {
     value.FreeQTranslator(self);
    // return 1;
  }
}

pub trait QTranslator_FreeQTranslator {
  fn FreeQTranslator(self, rsthis: &mut QTranslator) ;
}

// proto:  void QTranslator::FreeQTranslator();
impl<'a> /*trait*/ QTranslator_FreeQTranslator for () {
  fn FreeQTranslator(self, rsthis: &mut QTranslator)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorD0Ev()};
     unsafe {_ZN11QTranslatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn load<T: QTranslator_load>(&mut self, value: T) -> i8 {
    return value.load(self);
    // return 1;
  }
}

pub trait QTranslator_load {
  fn load(self, rsthis: &mut QTranslator) -> i8;
}

// proto:  bool QTranslator::load(const QString & filename, const QString & directory, const QString & search_delimiters, const QString & suffix);
impl<'a> /*trait*/ QTranslator_load for (&'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn load(self, rsthis: &mut QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadERK7QStringS2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTranslator4loadERK7QStringS2_S2_S2_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTranslator {
  pub fn translate<T: QTranslator_translate>(&mut self, value: T) -> QString {
    return value.translate(self);
    // return 1;
  }
}

pub trait QTranslator_translate {
  fn translate(self, rsthis: &mut QTranslator) -> QString;
}

// proto:  QString QTranslator::translate(const char * context, const char * sourceText, const char * disambiguation, int n);
impl<'a> /*trait*/ QTranslator_translate for (&'a  String, &'a  String, &'a  String, i32) {
  fn translate(self, rsthis: &mut QTranslator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator9translateEPKcS1_S1_i()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK11QTranslator9translateEPKcS1_S1_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QTranslator::load(const uchar * data, int len, const QString & directory);
impl<'a> /*trait*/ QTranslator_load for (&'a  String, i32, &'a  QString) {
  fn load(self, rsthis: &mut QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadEPKhiRK7QString()};
    let arg0 = self.0.as_ptr()  as *const c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTranslator4loadEPKhiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QTranslator::load(const QLocale & locale, const QString & filename, const QString & prefix, const QString & directory, const QString & suffix);
impl<'a> /*trait*/ QTranslator_load for (&'a  QLocale, &'a  QString, &'a  QString, &'a  QString, &'a  QString) {
  fn load(self, rsthis: &mut QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    return ret as i8;
    // return 1;
  }
}

