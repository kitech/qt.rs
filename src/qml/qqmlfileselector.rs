// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQml/qqmlfileselector.h
// dst-file: /src/qml/qqmlfileselector.rs
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
use super::super::core::qstringlist::QStringList; // 771
use super::qqmlengine::QQmlEngine; // 773
use super::super::core::qfileselector::QFileSelector; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQmlFileSelector_Class_Size() -> c_int;
  // proto:  void QQmlFileSelector::setExtraSelectors(QStringList & strings);
  fn _ZN16QQmlFileSelector17setExtraSelectorsER11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlFileSelector::QQmlFileSelector(QQmlEngine * engine, QObject * parent);
  fn _ZN16QQmlFileSelectorC2EP10QQmlEngineP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static QQmlFileSelector * QQmlFileSelector::get(QQmlEngine * );
  fn _ZN16QQmlFileSelector3getEP10QQmlEngine(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QQmlFileSelector::setSelector(QFileSelector * selector);
  fn _ZN16QQmlFileSelector11setSelectorEP13QFileSelector(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QQmlFileSelector::metaObject();
  fn _ZNK16QQmlFileSelector10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQmlFileSelector::QQmlFileSelector(const QQmlFileSelector & );
  fn _ZN16QQmlFileSelectorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlFileSelector::setExtraSelectors(const QStringList & strings);
  fn _ZN16QQmlFileSelector17setExtraSelectorsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQmlFileSelector::~QQmlFileSelector();
  fn _ZN16QQmlFileSelectorD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QQmlFileSelector)=1
#[derive(Default)]
pub struct QQmlFileSelector {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QQmlFileSelector {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQmlFileSelector {
    return QQmlFileSelector{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQmlFileSelector {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QQmlFileSelector {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QQmlFileSelector::setExtraSelectors(QStringList & strings);
impl /*struct*/ QQmlFileSelector {
  pub fn setExtraSelectors<RetType, T: QQmlFileSelector_setExtraSelectors<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExtraSelectors(self);
    // return 1;
  }
}

pub trait QQmlFileSelector_setExtraSelectors<RetType> {
  fn setExtraSelectors(self , rsthis: & QQmlFileSelector) -> RetType;
}

  // proto:  void QQmlFileSelector::setExtraSelectors(QStringList & strings);
impl<'a> /*trait*/ QQmlFileSelector_setExtraSelectors<()> for (&'a QStringList) {
  fn setExtraSelectors(self , rsthis: & QQmlFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlFileSelector17setExtraSelectorsER11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QQmlFileSelector17setExtraSelectorsER11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQmlFileSelector::QQmlFileSelector(QQmlEngine * engine, QObject * parent);
impl /*struct*/ QQmlFileSelector {
  pub fn new<T: QQmlFileSelector_new>(value: T) -> QQmlFileSelector {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQmlFileSelector_new {
  fn new(self) -> QQmlFileSelector;
}

  // proto:  void QQmlFileSelector::QQmlFileSelector(QQmlEngine * engine, QObject * parent);
impl<'a> /*trait*/ QQmlFileSelector_new for (&'a QQmlEngine, &'a QObject) {
  fn new(self) -> QQmlFileSelector {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlFileSelectorC2EP10QQmlEngineP7QObject()};
    let ctysz: c_int = unsafe{QQmlFileSelector_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QQmlFileSelectorC2EP10QQmlEngineP7QObject(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlFileSelector{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QQmlFileSelector * QQmlFileSelector::get(QQmlEngine * );
impl /*struct*/ QQmlFileSelector {
  pub fn get_s<RetType, T: QQmlFileSelector_get_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.get_s();
    // return 1;
  }
}

pub trait QQmlFileSelector_get_s<RetType> {
  fn get_s(self ) -> RetType;
}

  // proto: static QQmlFileSelector * QQmlFileSelector::get(QQmlEngine * );
impl<'a> /*trait*/ QQmlFileSelector_get_s<QQmlFileSelector> for (&'a QQmlEngine) {
  fn get_s(self ) -> QQmlFileSelector {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlFileSelector3getEP10QQmlEngine()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QQmlFileSelector3getEP10QQmlEngine(arg0)};
    let mut ret1 = QQmlFileSelector::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQmlFileSelector::setSelector(QFileSelector * selector);
impl /*struct*/ QQmlFileSelector {
  pub fn setSelector<RetType, T: QQmlFileSelector_setSelector<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelector(self);
    // return 1;
  }
}

pub trait QQmlFileSelector_setSelector<RetType> {
  fn setSelector(self , rsthis: & QQmlFileSelector) -> RetType;
}

  // proto:  void QQmlFileSelector::setSelector(QFileSelector * selector);
impl<'a> /*trait*/ QQmlFileSelector_setSelector<()> for (&'a QFileSelector) {
  fn setSelector(self , rsthis: & QQmlFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlFileSelector11setSelectorEP13QFileSelector()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QQmlFileSelector11setSelectorEP13QFileSelector(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QQmlFileSelector::metaObject();
impl /*struct*/ QQmlFileSelector {
  pub fn metaObject<RetType, T: QQmlFileSelector_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQmlFileSelector_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQmlFileSelector) -> RetType;
}

  // proto:  const QMetaObject * QQmlFileSelector::metaObject();
impl<'a> /*trait*/ QQmlFileSelector_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQmlFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QQmlFileSelector10metaObjectEv()};
     unsafe {_ZNK16QQmlFileSelector10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQmlFileSelector::QQmlFileSelector(const QQmlFileSelector & );
impl<'a> /*trait*/ QQmlFileSelector_new for (&'a QQmlFileSelector) {
  fn new(self) -> QQmlFileSelector {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlFileSelectorC2ERKS_()};
    let ctysz: c_int = unsafe{QQmlFileSelector_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QQmlFileSelectorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQmlFileSelector{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQmlFileSelector::~QQmlFileSelector();
impl /*struct*/ QQmlFileSelector {
  pub fn free<RetType, T: QQmlFileSelector_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQmlFileSelector_free<RetType> {
  fn free(self , rsthis: & QQmlFileSelector) -> RetType;
}

  // proto:  void QQmlFileSelector::~QQmlFileSelector();
impl<'a> /*trait*/ QQmlFileSelector_free<()> for () {
  fn free(self , rsthis: & QQmlFileSelector) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QQmlFileSelectorD2Ev()};
     unsafe {_ZN16QQmlFileSelectorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

