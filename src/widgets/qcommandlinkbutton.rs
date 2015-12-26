// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtWidgets/qcommandlinkbutton.h
// dst-file: /src/widgets/qcommandlinkbutton.rs
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
use super::qpushbutton::QPushButton; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCommandLinkButton_Class_Size() -> c_int;
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
  fn dector_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  const QMetaObject * QCommandLinkButton::metaObject();
  fn _ZNK18QCommandLinkButton10metaObjectEv(qthis: *mut c_void);
  // proto:  void QCommandLinkButton::~QCommandLinkButton();
  fn _ZN18QCommandLinkButtonD0Ev(qthis: *mut c_void);
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QCommandLinkButton & );
  fn dector_ZN18QCommandLinkButtonC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLinkButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLinkButton::QCommandLinkButton(QWidget * parent);
  fn dector_ZN18QCommandLinkButtonC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLinkButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QCommandLinkButton::description();
  fn _ZNK18QCommandLinkButton11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, QWidget * parent);
  fn dector_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QCommandLinkButton::setDescription(const QString & description);
  fn _ZN18QCommandLinkButton14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCommandLinkButton)=1
pub struct QCommandLinkButton {
  qbase: QPushButton,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCommandLinkButton {
  pub fn inheritFrom(qthis: *mut c_void) -> QCommandLinkButton {
    return QCommandLinkButton{qbase: QPushButton::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QCommandLinkButton {
  type Target = QPushButton;

  fn deref(&self) -> &QPushButton {
    return & self.qbase;
  }
}
impl AsRef<QPushButton> for QCommandLinkButton {
  fn as_ref(& self) -> & QPushButton {
    return & self.qbase;
  }
}
  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
impl /*struct*/ QCommandLinkButton {
  pub fn New<T: QCommandLinkButton_New>(value: T) -> QCommandLinkButton {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLinkButton_New {
  fn New(self) -> QCommandLinkButton;
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_New for (&'a QString, &'a QString, &'a QWidget) {
  fn New(self) -> QCommandLinkButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget()};
    let ctysz: c_int = unsafe{QCommandLinkButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(arg0, arg1, arg2)};
    let rsthis = QCommandLinkButton{/**/qbase: QPushButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QCommandLinkButton::metaObject();
impl /*struct*/ QCommandLinkButton {
  pub fn metaObject<RetType, T: QCommandLinkButton_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_metaObject<RetType> {
  fn metaObject(self , rsthis: & QCommandLinkButton) -> RetType;
}

  // proto:  const QMetaObject * QCommandLinkButton::metaObject();
impl<'a> /*trait*/ QCommandLinkButton_metaObject<()> for () {
  fn metaObject(self , rsthis: & QCommandLinkButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLinkButton10metaObjectEv()};
     unsafe {_ZNK18QCommandLinkButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::~QCommandLinkButton();
impl /*struct*/ QCommandLinkButton {
  pub fn Free<RetType, T: QCommandLinkButton_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_Free<RetType> {
  fn Free(self , rsthis: & QCommandLinkButton) -> RetType;
}

  // proto:  void QCommandLinkButton::~QCommandLinkButton();
impl<'a> /*trait*/ QCommandLinkButton_Free<()> for () {
  fn Free(self , rsthis: & QCommandLinkButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonD0Ev()};
     unsafe {_ZN18QCommandLinkButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QCommandLinkButton & );
impl<'a> /*trait*/ QCommandLinkButton_New for (&'a QCommandLinkButton) {
  fn New(self) -> QCommandLinkButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERKS_()};
    let ctysz: c_int = unsafe{QCommandLinkButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLinkButtonC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN18QCommandLinkButtonC1ERKS_(arg0)};
    let rsthis = QCommandLinkButton{/**/qbase: QPushButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_New for (&'a QWidget) {
  fn New(self) -> QCommandLinkButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1EP7QWidget()};
    let ctysz: c_int = unsafe{QCommandLinkButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLinkButtonC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN18QCommandLinkButtonC1EP7QWidget(arg0)};
    let rsthis = QCommandLinkButton{/**/qbase: QPushButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QCommandLinkButton::description();
impl /*struct*/ QCommandLinkButton {
  pub fn description<RetType, T: QCommandLinkButton_description<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_description<RetType> {
  fn description(self , rsthis: & QCommandLinkButton) -> RetType;
}

  // proto:  QString QCommandLinkButton::description();
impl<'a> /*trait*/ QCommandLinkButton_description<QString> for () {
  fn description(self , rsthis: & QCommandLinkButton) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLinkButton11descriptionEv()};
    let mut ret = unsafe {_ZNK18QCommandLinkButton11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::QCommandLinkButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_New for (&'a QString, &'a QWidget) {
  fn New(self) -> QCommandLinkButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QCommandLinkButton_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QCommandLinkButton{/**/qbase: QPushButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLinkButton::setDescription(const QString & description);
impl /*struct*/ QCommandLinkButton {
  pub fn setDescription<RetType, T: QCommandLinkButton_setDescription<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QCommandLinkButton_setDescription<RetType> {
  fn setDescription(self , rsthis: & QCommandLinkButton) -> RetType;
}

  // proto:  void QCommandLinkButton::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLinkButton_setDescription<()> for (&'a QString) {
  fn setDescription(self , rsthis: & QCommandLinkButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButton14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLinkButton14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

