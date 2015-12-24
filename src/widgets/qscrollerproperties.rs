// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
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
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  void QScrollerProperties::QScrollerProperties(const QScrollerProperties & sp);
  fn _ZN19QScrollerPropertiesC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QScrollerProperties::setDefaultScrollerProperties(const QScrollerProperties & sp);
  fn _ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0: *mut c_void);
  // proto:  void QScrollerProperties::~QScrollerProperties();
  fn _ZN19QScrollerPropertiesD0Ev(qthis: *mut c_void);
  // proto: static void QScrollerProperties::unsetDefaultScrollerProperties();
  fn _ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv();
  // proto:  void QScrollerProperties::QScrollerProperties();
  fn _ZN19QScrollerPropertiesC1Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QScrollerProperties)=1
pub struct QScrollerProperties {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollerProperties {
  pub fn inheritFrom(qthis: *mut c_void) -> QScrollerProperties {
    return QScrollerProperties{qclsinst: qthis};
  }
}
  // proto:  void QScrollerProperties::QScrollerProperties(const QScrollerProperties & sp);
impl /*struct*/ QScrollerProperties {
  pub fn New<T: QScrollerProperties_New>(value: T) -> QScrollerProperties {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollerProperties_New {
  fn New(self) -> QScrollerProperties;
}

  // proto:  void QScrollerProperties::QScrollerProperties(const QScrollerProperties & sp);
impl<'a> /*trait*/ QScrollerProperties_New for (&'a QScrollerProperties) {
  fn New(self) -> QScrollerProperties {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QScrollerPropertiesC1ERKS_(qthis, arg0)};
    let rsthis = QScrollerProperties{qclsinst: qthis};
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
     unsafe {_ZN19QScrollerProperties28setDefaultScrollerPropertiesERKS_(arg0)};
    // return 1;
  }
}

  // proto:  void QScrollerProperties::~QScrollerProperties();
impl /*struct*/ QScrollerProperties {
  pub fn Free<RetType, T: QScrollerProperties_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QScrollerProperties_Free<RetType> {
  fn Free(self , rsthis: & QScrollerProperties) -> RetType;
}

  // proto:  void QScrollerProperties::~QScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_Free<()> for () {
  fn Free(self , rsthis: & QScrollerProperties) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesD0Ev()};
     unsafe {_ZN19QScrollerPropertiesD0Ev(rsthis.qclsinst)};
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
     unsafe {_ZN19QScrollerProperties30unsetDefaultScrollerPropertiesEv()};
    // return 1;
  }
}

  // proto:  void QScrollerProperties::QScrollerProperties();
impl<'a> /*trait*/ QScrollerProperties_New for () {
  fn New(self) -> QScrollerProperties {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QScrollerPropertiesC1Ev()};
    unsafe {_ZN19QScrollerPropertiesC1Ev(qthis)};
    let rsthis = QScrollerProperties{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

