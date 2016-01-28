// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qscrollerproperties.h
// dst-file: /src/widgets/qscrollerproperties.rs
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
use std::ops::Deref;
use super::super::core::qvariant::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QScrollerProperties_Class_Size() -> c_int;
  // proto:  void QScrollerProperties::QScrollerProperties(const QScrollerProperties & sp);
  fn C_ZN19QScrollerPropertiesC2ERKS_(arg0: *mut c_void) -> u64;
  // proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
  fn C_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0: *mut c_void);
  // proto:  void QScrollerProperties::~QScrollerProperties();
  fn C_ZN19QScrollerPropertiesD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
  fn C_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv();
  // proto:  void QScrollerProperties::QScrollerProperties();
  fn C_ZN19QScrollerPropertiesC2Ev() -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QScrollerProperties)=1
#[derive(Default)]
pub struct QScrollerProperties {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QScrollerProperties {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScrollerProperties {
    return QScrollerProperties{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QScrollerProperties::QScrollerProperties(const QScrollerProperties & sp);
impl /*struct*/ QScrollerProperties {
  pub fn new<T: QScrollerProperties_new>(value: T) -> QScrollerProperties {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollerProperties_new {
  fn new(self) -> QScrollerProperties;
}

  // proto:  void QScrollerProperties::QScrollerProperties(const QScrollerProperties & sp);
impl<'a> /*trait*/ QScrollerProperties_new for (&'a QScrollerProperties) {
  fn new(self) -> QScrollerProperties {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesC2ERKS_()};
    let ctysz: c_int = unsafe{QScrollerProperties_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QScrollerPropertiesC2ERKS_(arg0)};
    let rsthis = QScrollerProperties{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
impl /*struct*/ QScrollerProperties {
  pub fn setDefaultScrollerProperties_s<RetType, T: QScrollerProperties_setDefaultScrollerProperties_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultScrollerProperties_s();
    // return 1;
  }
}

pub trait QScrollerProperties_setDefaultScrollerProperties_s<RetType> {
  fn setDefaultScrollerProperties_s(self ) -> RetType;
}

  // proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
impl<'a> /*trait*/ QScrollerProperties_setDefaultScrollerProperties_s<()> for (&'a QScrollerProperties) {
  fn setDefaultScrollerProperties_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0)};
    // return 1;
  }
}

  // proto:  void QScrollerProperties::~QScrollerProperties();
impl /*struct*/ QScrollerProperties {
  pub fn free<RetType, T: QScrollerProperties_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScrollerProperties_free<RetType> {
  fn free(self , rsthis: & QScrollerProperties) -> RetType;
}

  // proto:  void QScrollerProperties::~QScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_free<()> for () {
  fn free(self , rsthis: & QScrollerProperties) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesD2Ev()};
     unsafe {C_ZN19QScrollerPropertiesD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
impl /*struct*/ QScrollerProperties {
  pub fn unsetDefaultScrollerProperties_s<RetType, T: QScrollerProperties_unsetDefaultScrollerProperties_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.unsetDefaultScrollerProperties_s();
    // return 1;
  }
}

pub trait QScrollerProperties_unsetDefaultScrollerProperties_s<RetType> {
  fn unsetDefaultScrollerProperties_s(self ) -> RetType;
}

  // proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_unsetDefaultScrollerProperties_s<()> for () {
  fn unsetDefaultScrollerProperties_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
     unsafe {C_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
    // return 1;
  }
}

  // proto:  void QScrollerProperties::QScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_new for () {
  fn new(self) -> QScrollerProperties {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesC2Ev()};
    let ctysz: c_int = unsafe{QScrollerProperties_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN19QScrollerPropertiesC2Ev()};
    let rsthis = QScrollerProperties{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

