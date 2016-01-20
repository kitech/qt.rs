// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlpropertymap.h
// dst-file: /src/qml/qqmlpropertymap.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlPropertyMap_Class_Size() -> c_int;
  // proto:  QStringList QQmlPropertyMap::keys();
  fn _ZNK15QQmlPropertyMap4keysEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlPropertyMap::QQmlPropertyMap(QObject * parent);
  fn _ZN15QQmlPropertyMapC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlPropertyMap::QQmlPropertyMap(const QQmlPropertyMap & );
  fn _ZN15QQmlPropertyMapC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVariant QQmlPropertyMap::value(const QString & key);
  fn _ZNK15QQmlPropertyMap5valueERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QQmlPropertyMap::size();
  fn _ZNK15QQmlPropertyMap4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QQmlPropertyMap::contains(const QString & key);
  fn _ZNK15QQmlPropertyMap8containsERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QQmlPropertyMap::count();
  fn _ZNK15QQmlPropertyMap5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QQmlPropertyMap::metaObject();
  fn _ZNK15QQmlPropertyMap10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlPropertyMap::clear(const QString & key);
  fn _ZN15QQmlPropertyMap5clearERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlPropertyMap::insert(const QString & key, const QVariant & value);
  fn _ZN15QQmlPropertyMap6insertERK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QQmlPropertyMap::isEmpty();
  fn _ZNK15QQmlPropertyMap7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQmlPropertyMap::~QQmlPropertyMap();
  fn _ZN15QQmlPropertyMapD2Ev(qthis: u64 /* *mut c_void*/);
  fn QQmlPropertyMap_SlotProxy_connect__ZN15QQmlPropertyMap12valueChangedERK7QStringRK8QVariant(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlPropertyMap)=1
#[derive(Default)]
pub struct QQmlPropertyMap {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _valueChanged: QQmlPropertyMap_valueChanged_signal,
}

impl /*struct*/ QQmlPropertyMap {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlPropertyMap {
    return QQmlPropertyMap{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlPropertyMap {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQmlPropertyMap {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QStringList QQmlPropertyMap::keys();
impl /*struct*/ QQmlPropertyMap {
  pub fn keys<RetType, T: QQmlPropertyMap_keys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_keys<RetType> {
  fn keys(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  QStringList QQmlPropertyMap::keys();
impl<'a> /*trait*/ QQmlPropertyMap_keys<()> for () {
  fn keys(self , rsthis: & QQmlPropertyMap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap4keysEv()};
     unsafe {_ZNK15QQmlPropertyMap4keysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlPropertyMap::QQmlPropertyMap(QObject * parent);
impl /*struct*/ QQmlPropertyMap {
  pub fn new<T: QQmlPropertyMap_new>(value: T) -> QQmlPropertyMap {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlPropertyMap_new {
  fn new(self) -> QQmlPropertyMap;
}

  // proto:  void QQmlPropertyMap::QQmlPropertyMap(QObject * parent);
impl<'a> /*trait*/ QQmlPropertyMap_new for (&'a QObject) {
  fn new(self) -> QQmlPropertyMap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQmlPropertyMapC2EP7QObject()};
    let ctysz: c_int = unsafe{QQmlPropertyMap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QQmlPropertyMapC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlPropertyMap{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlPropertyMap::QQmlPropertyMap(const QQmlPropertyMap & );
impl<'a> /*trait*/ QQmlPropertyMap_new for (&'a QQmlPropertyMap) {
  fn new(self) -> QQmlPropertyMap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQmlPropertyMapC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlPropertyMap_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QQmlPropertyMapC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlPropertyMap{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariant QQmlPropertyMap::value(const QString & key);
impl /*struct*/ QQmlPropertyMap {
  pub fn value<RetType, T: QQmlPropertyMap_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_value<RetType> {
  fn value(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  QVariant QQmlPropertyMap::value(const QString & key);
impl<'a> /*trait*/ QQmlPropertyMap_value<QVariant> for (&'a QString) {
  fn value(self , rsthis: & QQmlPropertyMap) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QQmlPropertyMap5valueERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QQmlPropertyMap::size();
impl /*struct*/ QQmlPropertyMap {
  pub fn size<RetType, T: QQmlPropertyMap_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_size<RetType> {
  fn size(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  int QQmlPropertyMap::size();
impl<'a> /*trait*/ QQmlPropertyMap_size<i32> for () {
  fn size(self , rsthis: & QQmlPropertyMap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap4sizeEv()};
    let mut ret = unsafe {_ZNK15QQmlPropertyMap4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QQmlPropertyMap::contains(const QString & key);
impl /*struct*/ QQmlPropertyMap {
  pub fn contains<RetType, T: QQmlPropertyMap_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_contains<RetType> {
  fn contains(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  bool QQmlPropertyMap::contains(const QString & key);
impl<'a> /*trait*/ QQmlPropertyMap_contains<i8> for (&'a QString) {
  fn contains(self , rsthis: & QQmlPropertyMap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QQmlPropertyMap8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QQmlPropertyMap::count();
impl /*struct*/ QQmlPropertyMap {
  pub fn count<RetType, T: QQmlPropertyMap_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_count<RetType> {
  fn count(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  int QQmlPropertyMap::count();
impl<'a> /*trait*/ QQmlPropertyMap_count<i32> for () {
  fn count(self , rsthis: & QQmlPropertyMap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap5countEv()};
    let mut ret = unsafe {_ZNK15QQmlPropertyMap5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlPropertyMap::metaObject();
impl /*struct*/ QQmlPropertyMap {
  pub fn metaObject<RetType, T: QQmlPropertyMap_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  const QMetaObject * QQmlPropertyMap::metaObject();
impl<'a> /*trait*/ QQmlPropertyMap_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlPropertyMap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap10metaObjectEv()};
     unsafe {_ZNK15QQmlPropertyMap10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlPropertyMap::clear(const QString & key);
impl /*struct*/ QQmlPropertyMap {
  pub fn clear<RetType, T: QQmlPropertyMap_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_clear<RetType> {
  fn clear(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  void QQmlPropertyMap::clear(const QString & key);
impl<'a> /*trait*/ QQmlPropertyMap_clear<()> for (&'a QString) {
  fn clear(self , rsthis: & QQmlPropertyMap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQmlPropertyMap5clearERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QQmlPropertyMap5clearERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlPropertyMap::insert(const QString & key, const QVariant & value);
impl /*struct*/ QQmlPropertyMap {
  pub fn insert<RetType, T: QQmlPropertyMap_insert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insert(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_insert<RetType> {
  fn insert(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  void QQmlPropertyMap::insert(const QString & key, const QVariant & value);
impl<'a> /*trait*/ QQmlPropertyMap_insert<()> for (&'a QString, &'a QVariant) {
  fn insert(self , rsthis: & QQmlPropertyMap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQmlPropertyMap6insertERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15QQmlPropertyMap6insertERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QQmlPropertyMap::isEmpty();
impl /*struct*/ QQmlPropertyMap {
  pub fn isEmpty<RetType, T: QQmlPropertyMap_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  bool QQmlPropertyMap::isEmpty();
impl<'a> /*trait*/ QQmlPropertyMap_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QQmlPropertyMap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QQmlPropertyMap7isEmptyEv()};
    let mut ret = unsafe {_ZNK15QQmlPropertyMap7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQmlPropertyMap::~QQmlPropertyMap();
impl /*struct*/ QQmlPropertyMap {
  pub fn free<RetType, T: QQmlPropertyMap_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlPropertyMap_free<RetType> {
  fn free(self , rsthis: & QQmlPropertyMap) -> RetType;
}

  // proto:  void QQmlPropertyMap::~QQmlPropertyMap();
impl<'a> /*trait*/ QQmlPropertyMap_free<()> for () {
  fn free(self , rsthis: & QQmlPropertyMap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QQmlPropertyMapD2Ev()};
     unsafe {_ZN15QQmlPropertyMapD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QQmlPropertyMap_valueChanged
pub struct QQmlPropertyMap_valueChanged_signal{poi:u64}
impl /* struct */ QQmlPropertyMap {
  pub fn valueChanged(&self) -> QQmlPropertyMap_valueChanged_signal {
     return QQmlPropertyMap_valueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQmlPropertyMap_valueChanged_signal {
  pub fn connect<T: QQmlPropertyMap_valueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQmlPropertyMap_valueChanged_signal_connect {
  fn connect(self, sigthis: QQmlPropertyMap_valueChanged_signal);
}

// valueChanged(const class QString &, const class QVariant &)
extern fn QQmlPropertyMap_valueChanged_signal_connect_cb_0(rsfptr:fn(QString, QVariant), arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  let rsarg1 = QVariant::inheritFrom(arg1 as u64);
  rsfptr(rsarg0,rsarg1);
}
extern fn QQmlPropertyMap_valueChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QString, QVariant)>, arg0: *mut c_void, arg1: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  let rsarg1 = QVariant::inheritFrom(arg1 as u64);
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QQmlPropertyMap_valueChanged_signal_connect for fn(QString, QVariant) {
  fn connect(self, sigthis: QQmlPropertyMap_valueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlPropertyMap_valueChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQmlPropertyMap_SlotProxy_connect__ZN15QQmlPropertyMap12valueChangedERK7QStringRK8QVariant(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQmlPropertyMap_valueChanged_signal_connect for Box<Fn(QString, QVariant)> {
  fn connect(self, sigthis: QQmlPropertyMap_valueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQmlPropertyMap_valueChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQmlPropertyMap_SlotProxy_connect__ZN15QQmlPropertyMap12valueChangedERK7QStringRK8QVariant(arg0, arg1, arg2)};
  }
}
// <= body block end

