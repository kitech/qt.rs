// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QCommonStyle::polish(QWidget * widget);
  fn _ZN12QCommonStyle6polishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommonStyle::polish(QPalette & );
  fn _ZN12QCommonStyle6polishER8QPalette(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommonStyle::QCommonStyle(const QCommonStyle & );
  fn _ZN12QCommonStyleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommonStyle::unpolish(QWidget * widget);
  fn _ZN12QCommonStyle8unpolishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommonStyle::unpolish(QApplication * application);
  fn _ZN12QCommonStyle8unpolishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommonStyle::polish(QApplication * app);
  fn _ZN12QCommonStyle6polishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommonStyle::QCommonStyle();
  fn _ZN12QCommonStyleC1Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QCommonStyle::metaObject();
  fn _ZNK12QCommonStyle10metaObjectEv(qthis: *mut c_void);
  // proto:  void QCommonStyle::~QCommonStyle();
  fn _ZN12QCommonStyleD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCommonStyle)=1
pub struct QCommonStyle {
  qbase: QStyle,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCommonStyle {
  pub fn inheritFrom(qthis: *mut c_void) -> QCommonStyle {
    return QCommonStyle{qbase: QStyle::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QCommonStyle {
  type Target = QStyle;

  fn deref(&self) -> &QStyle {
    return &self.qbase;
  }
}
impl AsRef<QStyle> for QCommonStyle {
  fn as_ref(&self) -> &QStyle {
    return &self.qbase;
  }
}
  // proto:  void QCommonStyle::polish(QWidget * widget);
impl /*struct*/ QCommonStyle {
  pub fn polish<RetType, T: QCommonStyle_polish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QCommonStyle_polish<RetType> {
  fn polish(self , rsthis: &mut QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::polish(QWidget * widget);
impl<'a> /*trait*/ QCommonStyle_polish<()> for (QWidget) {
  fn polish(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::polish(QPalette & );
impl<'a> /*trait*/ QCommonStyle_polish<()> for (QPalette) {
  fn polish(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::QCommonStyle(const QCommonStyle & );
impl /*struct*/ QCommonStyle {
  pub fn NewQCommonStyle<T: QCommonStyle_NewQCommonStyle>(value: T) -> QCommonStyle {
    let rsthis = value.NewQCommonStyle();
    return rsthis;
    // return 1;
  }
}

pub trait QCommonStyle_NewQCommonStyle {
  fn NewQCommonStyle(self) -> QCommonStyle;
}

  // proto:  void QCommonStyle::QCommonStyle(const QCommonStyle & );
impl<'a> /*trait*/ QCommonStyle_NewQCommonStyle for (QCommonStyle) {
  fn NewQCommonStyle(self) -> QCommonStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QCommonStyleC1ERKS_(qthis, arg0)};
    let rsthis = QCommonStyle{/**/qbase: QStyle::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommonStyle::unpolish(QWidget * widget);
impl /*struct*/ QCommonStyle {
  pub fn unpolish<RetType, T: QCommonStyle_unpolish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unpolish(self);
    // return 1;
  }
}

pub trait QCommonStyle_unpolish<RetType> {
  fn unpolish(self , rsthis: &mut QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::unpolish(QWidget * widget);
impl<'a> /*trait*/ QCommonStyle_unpolish<()> for (QWidget) {
  fn unpolish(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::unpolish(QApplication * application);
impl<'a> /*trait*/ QCommonStyle_unpolish<()> for (QApplication) {
  fn unpolish(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::polish(QApplication * app);
impl<'a> /*trait*/ QCommonStyle_polish<()> for (QApplication) {
  fn polish(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QCommonStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::QCommonStyle();
impl<'a> /*trait*/ QCommonStyle_NewQCommonStyle for () {
  fn NewQCommonStyle(self) -> QCommonStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleC1Ev()};
    unsafe {_ZN12QCommonStyleC1Ev(qthis)};
    let rsthis = QCommonStyle{/**/qbase: QStyle::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QCommonStyle::metaObject();
impl /*struct*/ QCommonStyle {
  pub fn metaObject<RetType, T: QCommonStyle_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCommonStyle_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QCommonStyle) -> RetType;
}

  // proto:  const QMetaObject * QCommonStyle::metaObject();
impl<'a> /*trait*/ QCommonStyle_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QCommonStyle10metaObjectEv()};
     unsafe {_ZNK12QCommonStyle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommonStyle::~QCommonStyle();
impl /*struct*/ QCommonStyle {
  pub fn FreeQCommonStyle<RetType, T: QCommonStyle_FreeQCommonStyle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCommonStyle(self);
    // return 1;
  }
}

pub trait QCommonStyle_FreeQCommonStyle<RetType> {
  fn FreeQCommonStyle(self , rsthis: &mut QCommonStyle) -> RetType;
}

  // proto:  void QCommonStyle::~QCommonStyle();
impl<'a> /*trait*/ QCommonStyle_FreeQCommonStyle<()> for () {
  fn FreeQCommonStyle(self , rsthis: &mut QCommonStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QCommonStyleD0Ev()};
     unsafe {_ZN12QCommonStyleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

