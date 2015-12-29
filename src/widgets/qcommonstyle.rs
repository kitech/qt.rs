// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
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
use super::qstyle::QStyle; // 773
use std::ops::Deref;
use super::qstyleoption::QStyleOption; // 773
use super::qwidget::QWidget; // 773
use super::qstyleoption::QStyleOptionComplex; // 773
use super::super::gui::qpainter::QPainter; // 771
use super::super::gui::qpixmap::QPixmap; // 771
use super::super::gui::qpalette::QPalette; // 771
use super::super::core::qsize::QSize; // 771
use super::qapplication::QApplication; // 773
use super::qstyleoption::QStyleHintReturn; // 773
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCommonStyle_Class_Size() -> c_int;
  // proto:  void QCommonStyle::polish(QWidget * widget);
  fn _ZN12QCommonStyle6polishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::polish(QPalette & );
  fn _ZN12QCommonStyle6polishER8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::QCommonStyle(const QCommonStyle & );
  fn dector_ZN12QCommonStyleC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QCommonStyleC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::unpolish(QWidget * widget);
  fn _ZN12QCommonStyle8unpolishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::unpolish(QApplication * application);
  fn _ZN12QCommonStyle8unpolishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::polish(QApplication * app);
  fn _ZN12QCommonStyle6polishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QCommonStyle::QCommonStyle();
  fn dector_ZN12QCommonStyleC1Ev() -> *mut c_void;
  fn _ZN12QCommonStyleC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QCommonStyle::metaObject();
  fn _ZNK12QCommonStyle10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QCommonStyle::~QCommonStyle();
  fn _ZN12QCommonStyleD0Ev(qthis: u64 /* *mut c_void*/);
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
     unsafe {_ZN12QCommonStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::polish(QPalette & );
impl<'a> /*trait*/ QCommonStyle_polish<()> for (&'a QPalette) {
  fn polish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::QCommonStyle(const QCommonStyle & );
impl /*struct*/ QCommonStyle {
  pub fn New<T: QCommonStyle_New>(value: T) -> QCommonStyle {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCommonStyle_New {
  fn New(self) -> QCommonStyle;
}

  // proto:  void QCommonStyle::QCommonStyle(const QCommonStyle & );
impl<'a> /*trait*/ QCommonStyle_New for (&'a QCommonStyle) {
  fn New(self) -> QCommonStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleC1ERKS_()};
    let ctysz: c_int = unsafe{QCommonStyle_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QCommonStyleC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QCommonStyleC1ERKS_(arg0)} as u64;
    let rsthis = QCommonStyle{qbase: QStyle::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
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
     unsafe {_ZN12QCommonStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::unpolish(QApplication * application);
impl<'a> /*trait*/ QCommonStyle_unpolish<()> for (&'a QApplication) {
  fn unpolish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::polish(QApplication * app);
impl<'a> /*trait*/ QCommonStyle_polish<()> for (&'a QApplication) {
  fn polish(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::QCommonStyle();
impl<'a> /*trait*/ QCommonStyle_New for () {
  fn New(self) -> QCommonStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleC1Ev()};
    let ctysz: c_int = unsafe{QCommonStyle_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN12QCommonStyleC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN12QCommonStyleC1Ev()} as u64;
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
impl<'a> /*trait*/ QCommonStyle_metaObject<()> for () {
  fn metaObject(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QCommonStyle10metaObjectEv()};
     unsafe {_ZNK12QCommonStyle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::~QCommonStyle();
impl /*struct*/ QCommonStyle {
  pub fn Free<RetType, T: QCommonStyle_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCommonStyle_Free<RetType> {
  fn Free(self , rsthis: & QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::~QCommonStyle();
impl<'a> /*trait*/ QCommonStyle_Free<()> for () {
  fn Free(self , rsthis: & QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleD0Ev()};
     unsafe {_ZN12QCommonStyleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

