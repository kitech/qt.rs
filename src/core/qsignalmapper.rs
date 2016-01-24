// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qsignalmapper.h
// dst-file: /src/core/qsignalmapper.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qobjectdefs::QMetaObject; // 773
use super::qstring::QString; // 773
use super::super::widgets::qwidget::QWidget; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSignalMapper_Class_Size() -> c_int;
  // proto:  void QSignalMapper::removeMappings(QObject * sender);
  fn C_ZN13QSignalMapper14removeMappingsEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSignalMapper::map(QObject * sender);
  fn C_ZN13QSignalMapper3mapEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QSignalMapper::metaObject();
  fn C_ZNK13QSignalMapper10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
  fn C_ZN13QSignalMapper10setMappingEP7QObjectS1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QObject * QSignalMapper::mapping(int id);
  fn C_ZNK13QSignalMapper7mappingEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QSignalMapper::QSignalMapper(QObject * parent);
  fn C_ZN13QSignalMapperC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QSignalMapper::~QSignalMapper();
  fn C_ZN13QSignalMapperD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSignalMapper::setMapping(QObject * sender, int id);
  fn C_ZN13QSignalMapper10setMappingEP7QObjecti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  QObject * QSignalMapper::mapping(const QString & text);
  fn C_ZNK13QSignalMapper7mappingERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::map();
  fn C_ZN13QSignalMapper3mapEv(qthis: u64 /* *mut c_void*/);
  // proto:  QObject * QSignalMapper::mapping(QObject * object);
  fn C_ZNK13QSignalMapper7mappingEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::setMapping(QObject * sender, const QString & text);
  fn C_ZN13QSignalMapper10setMappingEP7QObjectRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QObject * QSignalMapper::mapping(QWidget * widget);
  fn C_ZNK13QSignalMapper7mappingEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::setMapping(QObject * sender, QWidget * widget);
  fn C_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  fn QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEP7QWidget(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEP7QObject(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSignalMapper)=1
#[derive(Default)]
pub struct QSignalMapper {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _mapped: QSignalMapper_mapped_signal,
}

impl /*struct*/ QSignalMapper {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSignalMapper {
    return QSignalMapper{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSignalMapper {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSignalMapper {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QSignalMapper::removeMappings(QObject * sender);
impl /*struct*/ QSignalMapper {
  pub fn removeMappings<RetType, T: QSignalMapper_removeMappings<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeMappings(self);
    // return 1;
  }
}

pub trait QSignalMapper_removeMappings<RetType> {
  fn removeMappings(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::removeMappings(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_removeMappings<()> for (&'a QObject) {
  fn removeMappings(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper14removeMappingsEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSignalMapper14removeMappingsEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::map(QObject * sender);
impl /*struct*/ QSignalMapper {
  pub fn map<RetType, T: QSignalMapper_map<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.map(self);
    // return 1;
  }
}

pub trait QSignalMapper_map<RetType> {
  fn map(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::map(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_map<()> for (&'a QObject) {
  fn map(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper3mapEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSignalMapper3mapEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSignalMapper::metaObject();
impl /*struct*/ QSignalMapper {
  pub fn metaObject<RetType, T: QSignalMapper_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSignalMapper_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  const QMetaObject * QSignalMapper::metaObject();
impl<'a> /*trait*/ QSignalMapper_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSignalMapper) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QSignalMapper10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
impl /*struct*/ QSignalMapper {
  pub fn setMapping<RetType, T: QSignalMapper_setMapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMapping(self);
    // return 1;
  }
}

pub trait QSignalMapper_setMapping<RetType> {
  fn setMapping(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, &'a QObject) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSignalMapper10setMappingEP7QObjectS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(int id);
impl /*struct*/ QSignalMapper {
  pub fn mapping<RetType, T: QSignalMapper_mapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapping(self);
    // return 1;
  }
}

pub trait QSignalMapper_mapping<RetType> {
  fn mapping(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  QObject * QSignalMapper::mapping(int id);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (i32) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QSignalMapper7mappingEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::QSignalMapper(QObject * parent);
impl /*struct*/ QSignalMapper {
  pub fn new<T: QSignalMapper_new>(value: T) -> QSignalMapper {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalMapper_new {
  fn new(self) -> QSignalMapper;
}

  // proto:  void QSignalMapper::QSignalMapper(QObject * parent);
impl<'a> /*trait*/ QSignalMapper_new for (&'a QObject) {
  fn new(self) -> QSignalMapper {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC2EP7QObject()};
    let ctysz: c_int = unsafe{QSignalMapper_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QSignalMapperC2EP7QObject(arg0)};
    let rsthis = QSignalMapper{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalMapper::~QSignalMapper();
impl /*struct*/ QSignalMapper {
  pub fn free<RetType, T: QSignalMapper_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSignalMapper_free<RetType> {
  fn free(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::~QSignalMapper();
impl<'a> /*trait*/ QSignalMapper_free<()> for () {
  fn free(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperD2Ev()};
     unsafe {C_ZN13QSignalMapperD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, int id);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, i32) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QSignalMapper10setMappingEP7QObjecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(const QString & text);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (&'a QString) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QSignalMapper7mappingERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::map();
impl<'a> /*trait*/ QSignalMapper_map<()> for () {
  fn map(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper3mapEv()};
     unsafe {C_ZN13QSignalMapper3mapEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(QObject * object);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (&'a QObject) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QSignalMapper7mappingEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, const QString & text);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, &'a QString) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSignalMapper10setMappingEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(QWidget * widget);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (&'a QWidget) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QSignalMapper7mappingEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, QWidget * widget);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, &'a QWidget) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

#[derive(Default)] // for QSignalMapper_mapped
pub struct QSignalMapper_mapped_signal{poi:u64}
impl /* struct */ QSignalMapper {
  pub fn mapped(&self) -> QSignalMapper_mapped_signal {
     return QSignalMapper_mapped_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSignalMapper_mapped_signal {
  pub fn connect<T: QSignalMapper_mapped_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSignalMapper_mapped_signal_connect {
  fn connect(self, sigthis: QSignalMapper_mapped_signal);
}

// mapped(class QWidget *)
extern fn QSignalMapper_mapped_signal_connect_cb_0(rsfptr:fn(QWidget), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QWidget::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QSignalMapper_mapped_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QWidget)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QWidget::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSignalMapper_mapped_signal_connect for fn(QWidget) {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEP7QWidget(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSignalMapper_mapped_signal_connect for Box<Fn(QWidget)> {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEP7QWidget(arg0, arg1, arg2)};
  }
}
// mapped(class QObject *)
extern fn QSignalMapper_mapped_signal_connect_cb_1(rsfptr:fn(QObject), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QSignalMapper_mapped_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QObject)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSignalMapper_mapped_signal_connect for fn(QObject) {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEP7QObject(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSignalMapper_mapped_signal_connect for Box<Fn(QObject)> {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEP7QObject(arg0, arg1, arg2)};
  }
}
// mapped(const class QString &)
extern fn QSignalMapper_mapped_signal_connect_cb_2(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QSignalMapper_mapped_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSignalMapper_mapped_signal_connect for fn(QString) {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSignalMapper_mapped_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedERK7QString(arg0, arg1, arg2)};
  }
}
// mapped(int)
extern fn QSignalMapper_mapped_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QSignalMapper_mapped_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QSignalMapper_mapped_signal_connect for fn(i32) {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSignalMapper_mapped_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QSignalMapper_mapped_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSignalMapper_mapped_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSignalMapper_SlotProxy_connect__ZN13QSignalMapper6mappedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

