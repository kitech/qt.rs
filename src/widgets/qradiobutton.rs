// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtWidgets/qradiobutton.h
// dst-file: /src/widgets/qradiobutton.rs
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
use super::qabstractbutton::QAbstractButton; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRadioButton_Class_Size() -> c_int;
  // proto:  const QMetaObject * QRadioButton::metaObject();
  fn _ZNK12QRadioButton10metaObjectEv(qthis: *mut c_void);
  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
  fn dector_ZN12QRadioButtonC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QRadioButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QRadioButton::sizeHint();
  fn _ZNK12QRadioButton8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QRadioButton::minimumSizeHint();
  fn _ZNK12QRadioButton15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRadioButton::~QRadioButton();
  fn _ZN12QRadioButtonD0Ev(qthis: *mut c_void);
  // proto:  void QRadioButton::QRadioButton(const QRadioButton & );
  fn dector_ZN12QRadioButtonC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QRadioButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadioButton::QRadioButton(const QString & text, QWidget * parent);
  fn dector_ZN12QRadioButtonC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN12QRadioButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRadioButton)=1
pub struct QRadioButton {
  qbase: QAbstractButton,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRadioButton {
  pub fn inheritFrom(qthis: *mut c_void) -> QRadioButton {
    return QRadioButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QRadioButton {
  type Target = QAbstractButton;

  fn deref(&self) -> &QAbstractButton {
    return & self.qbase;
  }
}
impl AsRef<QAbstractButton> for QRadioButton {
  fn as_ref(& self) -> & QAbstractButton {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QRadioButton::metaObject();
impl /*struct*/ QRadioButton {
  pub fn metaObject<RetType, T: QRadioButton_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRadioButton_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  const QMetaObject * QRadioButton::metaObject();
impl<'a> /*trait*/ QRadioButton_metaObject<()> for () {
  fn metaObject(self , rsthis: & QRadioButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton10metaObjectEv()};
     unsafe {_ZNK12QRadioButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
impl /*struct*/ QRadioButton {
  pub fn New<T: QRadioButton_New>(value: T) -> QRadioButton {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRadioButton_New {
  fn New(self) -> QRadioButton;
}

  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
impl<'a> /*trait*/ QRadioButton_New for (&'a QWidget) {
  fn New(self) -> QRadioButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1EP7QWidget()};
    let ctysz: c_int = unsafe{QRadioButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QRadioButtonC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QRadioButtonC1EP7QWidget(arg0)};
    let rsthis = QRadioButton{/**/qbase: QAbstractButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QRadioButton::sizeHint();
impl /*struct*/ QRadioButton {
  pub fn sizeHint<RetType, T: QRadioButton_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  QSize QRadioButton::sizeHint();
impl<'a> /*trait*/ QRadioButton_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QRadioButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QRadioButton::minimumSizeHint();
impl /*struct*/ QRadioButton {
  pub fn minimumSizeHint<RetType, T: QRadioButton_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  QSize QRadioButton::minimumSizeHint();
impl<'a> /*trait*/ QRadioButton_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QRadioButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadioButton::~QRadioButton();
impl /*struct*/ QRadioButton {
  pub fn Free<RetType, T: QRadioButton_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRadioButton_Free<RetType> {
  fn Free(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  void QRadioButton::~QRadioButton();
impl<'a> /*trait*/ QRadioButton_Free<()> for () {
  fn Free(self , rsthis: & QRadioButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonD0Ev()};
     unsafe {_ZN12QRadioButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(const QRadioButton & );
impl<'a> /*trait*/ QRadioButton_New for (&'a QRadioButton) {
  fn New(self) -> QRadioButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1ERKS_()};
    let ctysz: c_int = unsafe{QRadioButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QRadioButtonC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QRadioButtonC1ERKS_(arg0)};
    let rsthis = QRadioButton{/**/qbase: QAbstractButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QRadioButton_New for (&'a QString, &'a QWidget) {
  fn New(self) -> QRadioButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QRadioButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN12QRadioButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN12QRadioButtonC1ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QRadioButton{/**/qbase: QAbstractButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

