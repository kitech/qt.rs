// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qcommonstyle.h
// dst-file: /src/widgets/qcommonstyle.rs
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
use super::qstyle::*; // 773
use std::ops::Deref;
use super::qstyleoption::*; // 773
use super::qwidget::*; // 773
use super::super::gui::qpainter::*; // 771
use super::super::gui::qpixmap::*; // 771
use super::super::gui::qpalette::*; // 771
use super::super::core::qsize::*; // 771
use super::qapplication::*; // 773
use super::super::core::qpoint::*; // 771
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCommonStyle_Class_Size() -> c_int;
  // proto:  void QCommonStyle::polish(QWidget * widget);
  fn C_ZN12QCommonStyle6polishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::polish(QPalette & );
  fn C_ZN12QCommonStyle6polishER8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::unpolish(QWidget * widget);
  fn C_ZN12QCommonStyle8unpolishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::unpolish(QApplication * application);
  fn C_ZN12QCommonStyle8unpolishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::polish(QApplication * app);
  fn C_ZN12QCommonStyle6polishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::QCommonStyle();
  fn C_ZN12QCommonStyleC2Ev() -> u64;
  // proto:  const QMetaObject * QCommonStyle::metaObject();
  fn C_ZNK12QCommonStyle10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCommonStyle::~QCommonStyle();
  fn C_ZN12QCommonStyleD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QCommonStyle)=1
#[derive(Default)]
pub struct QCommonStyle {
  qbase: QStyle,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QCommonStyle {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCommonStyle {
    return QCommonStyle{qbase: QStyle::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QCommonStyle {
  type Target = QStyle;

  fn deref(&self) -> &QStyle {
    return & self.qbase;
  }
}
impl AsRef<QStyle> for QCommonStyle {
  fn as_ref(& self) -> & QStyle {
    return & self.qbase;
  }
}
  // proto:  void QCommonStyle::polish(QWidget * widget);
impl /*struct*/ QCommonStyle {
  pub fn polish<RetType, T: QCommonStyle_polish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QCommonStyle_polish<RetType> {
  fn polish(self , rsthis: & QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::polish(QWidget * widget);
impl<'a> /*trait*/ QCommonStyle_polish<()> for (&'a QWidget) {
  fn polish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QCommonStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::polish(QPalette & );
impl<'a> /*trait*/ QCommonStyle_polish<()> for (&'a QPalette) {
  fn polish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QCommonStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::unpolish(QWidget * widget);
impl /*struct*/ QCommonStyle {
  pub fn unpolish<RetType, T: QCommonStyle_unpolish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unpolish(self);
    // return 1;
  }
}

pub trait QCommonStyle_unpolish<RetType> {
  fn unpolish(self , rsthis: & QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::unpolish(QWidget * widget);
impl<'a> /*trait*/ QCommonStyle_unpolish<()> for (&'a QWidget) {
  fn unpolish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QCommonStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::unpolish(QApplication * application);
impl<'a> /*trait*/ QCommonStyle_unpolish<()> for (&'a QApplication) {
  fn unpolish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QCommonStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::polish(QApplication * app);
impl<'a> /*trait*/ QCommonStyle_polish<()> for (&'a QApplication) {
  fn polish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QCommonStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::QCommonStyle();
impl /*struct*/ QCommonStyle {
  pub fn new<T: QCommonStyle_new>(value: T) -> QCommonStyle {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QCommonStyle_new {
  fn new(self) -> QCommonStyle;
}

  // proto:  void QCommonStyle::QCommonStyle();
impl<'a> /*trait*/ QCommonStyle_new for () {
  fn new(self) -> QCommonStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleC2Ev()};
    let ctysz: c_int = unsafe{QCommonStyle_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN12QCommonStyleC2Ev()};
    let rsthis = QCommonStyle{qbase: QStyle::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QCommonStyle::metaObject();
impl /*struct*/ QCommonStyle {
  pub fn metaObject<RetType, T: QCommonStyle_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCommonStyle_metaObject<RetType> {
  fn metaObject(self , rsthis: & QCommonStyle) -> RetType;
}

  // proto:  const QMetaObject * QCommonStyle::metaObject();
impl<'a> /*trait*/ QCommonStyle_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QCommonStyle) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QCommonStyle10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QCommonStyle10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommonStyle::~QCommonStyle();
impl /*struct*/ QCommonStyle {
  pub fn free<RetType, T: QCommonStyle_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QCommonStyle_free<RetType> {
  fn free(self , rsthis: & QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::~QCommonStyle();
impl<'a> /*trait*/ QCommonStyle_free<()> for () {
  fn free(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleD2Ev()};
     unsafe {C_ZN12QCommonStyleD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

