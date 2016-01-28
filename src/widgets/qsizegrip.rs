// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qsizegrip.h
// dst-file: /src/widgets/qsizegrip.rs
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSizeGrip_Class_Size() -> c_int;
  // proto:  void QSizeGrip::QSizeGrip(QWidget * parent);
  fn C_ZN9QSizeGripC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QSizeGrip::~QSizeGrip();
  fn C_ZN9QSizeGripD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSizeGrip::setVisible(bool );
  fn C_ZN9QSizeGrip10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QSizeGrip::metaObject();
  fn C_ZNK9QSizeGrip10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QSizeGrip::sizeHint();
  fn C_ZNK9QSizeGrip8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSizeGrip)=1
#[derive(Default)]
pub struct QSizeGrip {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSizeGrip {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSizeGrip {
    return QSizeGrip{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSizeGrip {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QSizeGrip {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QSizeGrip::QSizeGrip(QWidget * parent);
impl /*struct*/ QSizeGrip {
  pub fn new<T: QSizeGrip_new>(value: T) -> QSizeGrip {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeGrip_new {
  fn new(self) -> QSizeGrip;
}

  // proto:  void QSizeGrip::QSizeGrip(QWidget * parent);
impl<'a> /*trait*/ QSizeGrip_new for (&'a QWidget) {
  fn new(self) -> QSizeGrip {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGripC2EP7QWidget()};
    let ctysz: c_int = unsafe{QSizeGrip_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QSizeGripC2EP7QWidget(arg0)};
    let rsthis = QSizeGrip{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSizeGrip::~QSizeGrip();
impl /*struct*/ QSizeGrip {
  pub fn free<RetType, T: QSizeGrip_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSizeGrip_free<RetType> {
  fn free(self , rsthis: & QSizeGrip) -> RetType;
}

  // proto:  void QSizeGrip::~QSizeGrip();
impl<'a> /*trait*/ QSizeGrip_free<()> for () {
  fn free(self , rsthis: & QSizeGrip) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGripD2Ev()};
     unsafe {C_ZN9QSizeGripD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSizeGrip::setVisible(bool );
impl /*struct*/ QSizeGrip {
  pub fn setVisible<RetType, T: QSizeGrip_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QSizeGrip_setVisible<RetType> {
  fn setVisible(self , rsthis: & QSizeGrip) -> RetType;
}

  // proto:  void QSizeGrip::setVisible(bool );
impl<'a> /*trait*/ QSizeGrip_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QSizeGrip) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QSizeGrip10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QSizeGrip10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSizeGrip::metaObject();
impl /*struct*/ QSizeGrip {
  pub fn metaObject<RetType, T: QSizeGrip_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSizeGrip_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSizeGrip) -> RetType;
}

  // proto:  const QMetaObject * QSizeGrip::metaObject();
impl<'a> /*trait*/ QSizeGrip_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSizeGrip) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSizeGrip10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QSizeGrip10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSizeGrip::sizeHint();
impl /*struct*/ QSizeGrip {
  pub fn sizeHint<RetType, T: QSizeGrip_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSizeGrip_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QSizeGrip) -> RetType;
}

  // proto:  QSize QSizeGrip::sizeHint();
impl<'a> /*trait*/ QSizeGrip_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QSizeGrip) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QSizeGrip8sizeHintEv()};
    let mut ret = unsafe {C_ZNK9QSizeGrip8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

