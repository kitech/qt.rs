// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qfileselector.h
// dst-file: /src/core/qfileselector.rs
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
use super::qurl::QUrl; // 773
use super::qstringlist::QStringList; // 773
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFileSelector_Class_Size() -> c_int;
  // proto:  QStringList QFileSelector::allSelectors();
  fn _ZNK13QFileSelector12allSelectorsEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QFileSelector::metaObject();
  fn _ZNK13QFileSelector10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QUrl QFileSelector::select(const QUrl & filePath);
  fn _ZNK13QFileSelector6selectERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSelector::QFileSelector(QObject * parent);
  fn _ZN13QFileSelectorC2EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
  fn _ZN13QFileSelector17setExtraSelectorsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QFileSelector::select(const QString & filePath);
  fn _ZNK13QFileSelector6selectERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSelector::~QFileSelector();
  fn _ZN13QFileSelectorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStringList QFileSelector::extraSelectors();
  fn _ZNK13QFileSelector14extraSelectorsEv(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QFileSelector)=1
#[derive(Default)]
pub struct QFileSelector {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFileSelector {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFileSelector {
    return QFileSelector{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QFileSelector {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QFileSelector {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QStringList QFileSelector::allSelectors();
impl /*struct*/ QFileSelector {
  pub fn allSelectors<RetType, T: QFileSelector_allSelectors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_allSelectors<RetType> {
  fn allSelectors(self , rsthis: & QFileSelector) -> RetType;
}

  // proto:  QStringList QFileSelector::allSelectors();
impl<'a> /*trait*/ QFileSelector_allSelectors<()> for () {
  fn allSelectors(self , rsthis: & QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector12allSelectorsEv()};
     unsafe {_ZNK13QFileSelector12allSelectorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QFileSelector::metaObject();
impl /*struct*/ QFileSelector {
  pub fn metaObject<RetType, T: QFileSelector_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFileSelector_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFileSelector) -> RetType;
}

  // proto:  const QMetaObject * QFileSelector::metaObject();
impl<'a> /*trait*/ QFileSelector_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector10metaObjectEv()};
     unsafe {_ZNK13QFileSelector10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QUrl QFileSelector::select(const QUrl & filePath);
impl /*struct*/ QFileSelector {
  pub fn select<RetType, T: QFileSelector_select<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.select(self);
    // return 1;
  }
}

pub trait QFileSelector_select<RetType> {
  fn select(self , rsthis: & QFileSelector) -> RetType;
}

  // proto:  QUrl QFileSelector::select(const QUrl & filePath);
impl<'a> /*trait*/ QFileSelector_select<QUrl> for (&'a QUrl) {
  fn select(self , rsthis: & QFileSelector) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector6selectERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFileSelector6selectERK4QUrl(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSelector::QFileSelector(QObject * parent);
impl /*struct*/ QFileSelector {
  pub fn new<T: QFileSelector_new>(value: T) -> QFileSelector {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSelector_new {
  fn new(self) -> QFileSelector;
}

  // proto:  void QFileSelector::QFileSelector(QObject * parent);
impl<'a> /*trait*/ QFileSelector_new for (&'a QObject) {
  fn new(self) -> QFileSelector {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelectorC2EP7QObject()};
    let ctysz: c_int = unsafe{QFileSelector_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QFileSelectorC2EP7QObject(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QFileSelector{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
impl /*struct*/ QFileSelector {
  pub fn setExtraSelectors<RetType, T: QFileSelector_setExtraSelectors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExtraSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_setExtraSelectors<RetType> {
  fn setExtraSelectors(self , rsthis: & QFileSelector) -> RetType;
}

  // proto:  void QFileSelector::setExtraSelectors(const QStringList & list);
impl<'a> /*trait*/ QFileSelector_setExtraSelectors<()> for (&'a QStringList) {
  fn setExtraSelectors(self , rsthis: & QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelector17setExtraSelectorsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QFileSelector17setExtraSelectorsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QFileSelector::select(const QString & filePath);
impl<'a> /*trait*/ QFileSelector_select<QString> for (&'a QString) {
  fn select(self , rsthis: & QFileSelector) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector6selectERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QFileSelector6selectERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileSelector::~QFileSelector();
impl /*struct*/ QFileSelector {
  pub fn free<RetType, T: QFileSelector_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFileSelector_free<RetType> {
  fn free(self , rsthis: & QFileSelector) -> RetType;
}

  // proto:  void QFileSelector::~QFileSelector();
impl<'a> /*trait*/ QFileSelector_free<()> for () {
  fn free(self , rsthis: & QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QFileSelectorD2Ev()};
     unsafe {_ZN13QFileSelectorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QFileSelector::extraSelectors();
impl /*struct*/ QFileSelector {
  pub fn extraSelectors<RetType, T: QFileSelector_extraSelectors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.extraSelectors(self);
    // return 1;
  }
}

pub trait QFileSelector_extraSelectors<RetType> {
  fn extraSelectors(self , rsthis: & QFileSelector) -> RetType;
}

  // proto:  QStringList QFileSelector::extraSelectors();
impl<'a> /*trait*/ QFileSelector_extraSelectors<()> for () {
  fn extraSelectors(self , rsthis: & QFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QFileSelector14extraSelectorsEv()};
     unsafe {_ZNK13QFileSelector14extraSelectorsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

