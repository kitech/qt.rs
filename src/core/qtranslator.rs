// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qtranslator.h
// dst-file: /src/core/qtranslator.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qobject::*; // 773
use std::ops::Deref;
use super::qobjectdefs::*; // 773
use super::qstring::*; // 773
use super::qlocale::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTranslator_Class_Size() -> c_int;
  // proto:  const QMetaObject * QTranslator::metaObject();
  fn C_ZNK11QTranslator10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTranslator::QTranslator(QObject * parent);
  fn C_ZN11QTranslatorC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QTranslator::isEmpty();
  fn C_ZNK11QTranslator7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTranslator::~QTranslator();
  fn C_ZN11QTranslatorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTranslator::load(const QString & filename, const QString & directory, const QString & search_delimiters, const QString & suffix);
  fn C_ZN11QTranslator4loadERK7QStringS2_S2_S2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_char;
  // proto:  QString QTranslator::translate(const char * context, const char * sourceText, const char * disambiguation, int n);
  fn C_ZNK11QTranslator9translateEPKcS1_S1_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_char, arg2: *mut c_char, arg3: c_int) -> *mut c_void;
  // proto:  bool QTranslator::load(const uchar * data, int len, const QString & directory);
  fn C_ZN11QTranslator4loadEPKhiRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_uchar, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QTranslator::load(const QLocale & locale, const QString & filename, const QString & prefix, const QString & directory, const QString & suffix);
  fn C_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QTranslator)=1
#[derive(Default)]
pub struct QTranslator {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTranslator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTranslator {
    return QTranslator{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTranslator {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QTranslator {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QTranslator::metaObject();
impl /*struct*/ QTranslator {
  pub fn metaObject<RetType, T: QTranslator_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTranslator_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTranslator) -> RetType;
}

  // proto:  const QMetaObject * QTranslator::metaObject();
impl<'a> /*trait*/ QTranslator_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTranslator) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QTranslator10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTranslator::QTranslator(QObject * parent);
impl /*struct*/ QTranslator {
  pub fn new<T: QTranslator_new>(value: T) -> QTranslator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTranslator_new {
  fn new(self) -> QTranslator;
}

  // proto:  void QTranslator::QTranslator(QObject * parent);
impl<'a> /*trait*/ QTranslator_new for (Option<&'a QObject>) {
  fn new(self) -> QTranslator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorC2EP7QObject()};
    let ctysz: c_int = unsafe{QTranslator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QTranslatorC2EP7QObject(arg0)};
    let rsthis = QTranslator{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTranslator::isEmpty();
impl /*struct*/ QTranslator {
  pub fn isEmpty<RetType, T: QTranslator_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTranslator_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QTranslator) -> RetType;
}

  // proto:  bool QTranslator::isEmpty();
impl<'a> /*trait*/ QTranslator_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator7isEmptyEv()};
    let mut ret = unsafe {C_ZNK11QTranslator7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTranslator::~QTranslator();
impl /*struct*/ QTranslator {
  pub fn free<RetType, T: QTranslator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTranslator_free<RetType> {
  fn free(self , rsthis: & QTranslator) -> RetType;
}

  // proto:  void QTranslator::~QTranslator();
impl<'a> /*trait*/ QTranslator_free<()> for () {
  fn free(self , rsthis: & QTranslator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslatorD2Ev()};
     unsafe {C_ZN11QTranslatorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTranslator::load(const QString & filename, const QString & directory, const QString & search_delimiters, const QString & suffix);
impl /*struct*/ QTranslator {
  pub fn load<RetType, T: QTranslator_load<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QTranslator_load<RetType> {
  fn load(self , rsthis: & QTranslator) -> RetType;
}

  // proto:  bool QTranslator::load(const QString & filename, const QString & directory, const QString & search_delimiters, const QString & suffix);
impl<'a> /*trait*/ QTranslator_load<i8> for (&'a QString, Option<&'a QString>, Option<&'a QString>, Option<&'a QString>) {
  fn load(self , rsthis: & QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadERK7QStringS2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {QString::new(()).qclsinst} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let arg2 = (if self.2.is_none() {QString::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let arg3 = (if self.3.is_none() {QString::new(()).qclsinst} else {self.3.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN11QTranslator4loadERK7QStringS2_S2_S2_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QTranslator::translate(const char * context, const char * sourceText, const char * disambiguation, int n);
impl /*struct*/ QTranslator {
  pub fn translate<RetType, T: QTranslator_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QTranslator_translate<RetType> {
  fn translate(self , rsthis: & QTranslator) -> RetType;
}

  // proto:  QString QTranslator::translate(const char * context, const char * sourceText, const char * disambiguation, int n);
impl<'a> /*trait*/ QTranslator_translate<QString> for (&'a  String, &'a  String, Option<&'a  String>, Option<i32>) {
  fn translate(self , rsthis: & QTranslator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTranslator9translateEPKcS1_S1_i()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = (if self.2.is_none() {0 as *const u8} else {self.2.unwrap().as_ptr()})  as *mut c_char;
    let arg3 = (if self.3.is_none() {-1} else {self.3.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZNK11QTranslator9translateEPKcS1_S1_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTranslator::load(const uchar * data, int len, const QString & directory);
impl<'a> /*trait*/ QTranslator_load<i8> for (&'a  String, i32, Option<&'a QString>) {
  fn load(self , rsthis: & QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadEPKhiRK7QString()};
    let arg0 = self.0.as_ptr()  as *mut c_uchar;
    let arg1 = self.1  as c_int;
    let arg2 = (if self.2.is_none() {QString::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN11QTranslator4loadEPKhiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QTranslator::load(const QLocale & locale, const QString & filename, const QString & prefix, const QString & directory, const QString & suffix);
impl<'a> /*trait*/ QTranslator_load<i8> for (&'a QLocale, &'a QString, Option<&'a QString>, Option<&'a QString>, Option<&'a QString>) {
  fn load(self , rsthis: & QTranslator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = (if self.2.is_none() {QString::new(()).qclsinst} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let arg3 = (if self.3.is_none() {QString::new(()).qclsinst} else {self.3.unwrap().qclsinst})  as *mut c_void;
    let arg4 = (if self.4.is_none() {QString::new(()).qclsinst} else {self.4.unwrap().qclsinst})  as *mut c_void;
    let mut ret = unsafe {C_ZN11QTranslator4loadERK7QLocaleRK7QStringS5_S5_S5_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

