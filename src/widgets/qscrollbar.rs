// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qscrollbar.h
// dst-file: /src/widgets/qscrollbar.rs
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
use super::qabstractslider::QAbstractSlider; // 773
use std::ops::Deref;
use super::super::core::qcoreevent::QEvent; // 771
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QScrollBar::event(QEvent * event);
  fn _ZN10QScrollBar5eventEP6QEvent(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QScrollBar::QScrollBar(const QScrollBar & );
  fn _ZN10QScrollBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QScrollBar::metaObject();
  fn _ZNK10QScrollBar10metaObjectEv(qthis: *mut c_void);
  // proto:  QSize QScrollBar::sizeHint();
  fn _ZNK10QScrollBar8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollBar::QScrollBar(QWidget * parent);
  fn _ZN10QScrollBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QScrollBar::~QScrollBar();
  fn _ZN10QScrollBarD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QScrollBar)=1
pub struct QScrollBar {
  qbase: QAbstractSlider,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollBar {
  pub fn inheritFrom(qthis: *mut c_void) -> QScrollBar {
    return QScrollBar{qbase: QAbstractSlider::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QScrollBar {
  type Target = QAbstractSlider;

  fn deref(&self) -> &QAbstractSlider {
    return &self.qbase;
  }
}
impl AsRef<QAbstractSlider> for QScrollBar {
  fn as_ref(&self) -> &QAbstractSlider {
    return &self.qbase;
  }
}
  // proto:  bool QScrollBar::event(QEvent * event);
impl /*struct*/ QScrollBar {
  pub fn event<RetType, T: QScrollBar_event<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QScrollBar_event<RetType> {
  fn event(self , rsthis: &mut QScrollBar) -> RetType;
}

  // proto:  bool QScrollBar::event(QEvent * event);
impl<'a> /*trait*/ QScrollBar_event<i8> for (QEvent) {
  fn event(self , rsthis: &mut QScrollBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBar5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QScrollBar5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QScrollBar::QScrollBar(const QScrollBar & );
impl /*struct*/ QScrollBar {
  pub fn NewQScrollBar<T: QScrollBar_NewQScrollBar>(value: T) -> QScrollBar {
    let rsthis = value.NewQScrollBar();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollBar_NewQScrollBar {
  fn NewQScrollBar(self) -> QScrollBar;
}

  // proto:  void QScrollBar::QScrollBar(const QScrollBar & );
impl<'a> /*trait*/ QScrollBar_NewQScrollBar for (QScrollBar) {
  fn NewQScrollBar(self) -> QScrollBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QScrollBarC1ERKS_(qthis, arg0)};
    let rsthis = QScrollBar{/**/qbase: QAbstractSlider::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QScrollBar::metaObject();
impl /*struct*/ QScrollBar {
  pub fn metaObject<RetType, T: QScrollBar_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScrollBar_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QScrollBar) -> RetType;
}

  // proto:  const QMetaObject * QScrollBar::metaObject();
impl<'a> /*trait*/ QScrollBar_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QScrollBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QScrollBar10metaObjectEv()};
     unsafe {_ZNK10QScrollBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QScrollBar::sizeHint();
impl /*struct*/ QScrollBar {
  pub fn sizeHint<RetType, T: QScrollBar_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QScrollBar_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QScrollBar) -> RetType;
}

  // proto:  QSize QScrollBar::sizeHint();
impl<'a> /*trait*/ QScrollBar_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QScrollBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QScrollBar8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QScrollBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollBar::QScrollBar(QWidget * parent);
impl<'a> /*trait*/ QScrollBar_NewQScrollBar for (QWidget) {
  fn NewQScrollBar(self) -> QScrollBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QScrollBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QScrollBar{/**/qbase: QAbstractSlider::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QScrollBar::~QScrollBar();
impl /*struct*/ QScrollBar {
  pub fn FreeQScrollBar<RetType, T: QScrollBar_FreeQScrollBar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQScrollBar(self);
    // return 1;
  }
}

pub trait QScrollBar_FreeQScrollBar<RetType> {
  fn FreeQScrollBar(self , rsthis: &mut QScrollBar) -> RetType;
}

  // proto:  void QScrollBar::~QScrollBar();
impl<'a> /*trait*/ QScrollBar_FreeQScrollBar<()> for () {
  fn FreeQScrollBar(self , rsthis: &mut QScrollBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarD0Ev()};
     unsafe {_ZN10QScrollBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

