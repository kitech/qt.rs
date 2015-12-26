// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qerrormessage.h
// dst-file: /src/widgets/qerrormessage.rs
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
use super::qdialog::QDialog; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QErrorMessage_Class_Size() -> c_int;
  // proto:  const QMetaObject * QErrorMessage::metaObject();
  fn _ZNK13QErrorMessage10metaObjectEv(qthis: *mut c_void);
  // proto:  void QErrorMessage::QErrorMessage(QWidget * parent);
  fn dector_ZN13QErrorMessageC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QErrorMessageC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QErrorMessage * QErrorMessage::qtHandler();
  fn _ZN13QErrorMessage9qtHandlerEv() -> *mut c_void;
  // proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
  fn _ZN13QErrorMessage11showMessageERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QErrorMessage::showMessage(const QString & message);
  fn _ZN13QErrorMessage11showMessageERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QErrorMessage::~QErrorMessage();
  fn _ZN13QErrorMessageD0Ev(qthis: *mut c_void);
  // proto:  void QErrorMessage::QErrorMessage(const QErrorMessage & );
  fn dector_ZN13QErrorMessageC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QErrorMessageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QErrorMessage)=1
pub struct QErrorMessage {
  qbase: QDialog,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QErrorMessage {
  pub fn inheritFrom(qthis: *mut c_void) -> QErrorMessage {
    return QErrorMessage{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QErrorMessage {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QErrorMessage {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QErrorMessage::metaObject();
impl /*struct*/ QErrorMessage {
  pub fn metaObject<RetType, T: QErrorMessage_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QErrorMessage_metaObject<RetType> {
  fn metaObject(self , rsthis: & QErrorMessage) -> RetType;
}

  // proto:  const QMetaObject * QErrorMessage::metaObject();
impl<'a> /*trait*/ QErrorMessage_metaObject<()> for () {
  fn metaObject(self , rsthis: & QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QErrorMessage10metaObjectEv()};
     unsafe {_ZNK13QErrorMessage10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::QErrorMessage(QWidget * parent);
impl /*struct*/ QErrorMessage {
  pub fn New<T: QErrorMessage_New>(value: T) -> QErrorMessage {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QErrorMessage_New {
  fn New(self) -> QErrorMessage;
}

  // proto:  void QErrorMessage::QErrorMessage(QWidget * parent);
impl<'a> /*trait*/ QErrorMessage_New for (&'a QWidget) {
  fn New(self) -> QErrorMessage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageC1EP7QWidget()};
    let ctysz: c_int = unsafe{QErrorMessage_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QErrorMessageC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QErrorMessageC1EP7QWidget(arg0)};
    let rsthis = QErrorMessage{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QErrorMessage * QErrorMessage::qtHandler();
impl /*struct*/ QErrorMessage {
  pub fn qtHandler_s<RetType, T: QErrorMessage_qtHandler_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.qtHandler_s();
    // return 1;
  }
}

pub trait QErrorMessage_qtHandler_s<RetType> {
  fn qtHandler_s(self ) -> RetType;
}

  // proto: static QErrorMessage * QErrorMessage::qtHandler();
impl<'a> /*trait*/ QErrorMessage_qtHandler_s<QErrorMessage> for () {
  fn qtHandler_s(self ) -> QErrorMessage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage9qtHandlerEv()};
    let mut ret = unsafe {_ZN13QErrorMessage9qtHandlerEv()};
    let mut ret1 = QErrorMessage::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
impl /*struct*/ QErrorMessage {
  pub fn showMessage<RetType, T: QErrorMessage_showMessage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMessage(self);
    // return 1;
  }
}

pub trait QErrorMessage_showMessage<RetType> {
  fn showMessage(self , rsthis: & QErrorMessage) -> RetType;
}

  // proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
impl<'a> /*trait*/ QErrorMessage_showMessage<()> for (&'a QString, &'a QString) {
  fn showMessage(self , rsthis: & QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage11showMessageERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QErrorMessage11showMessageERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::showMessage(const QString & message);
impl<'a> /*trait*/ QErrorMessage_showMessage<()> for (&'a QString) {
  fn showMessage(self , rsthis: & QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage11showMessageERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QErrorMessage11showMessageERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::~QErrorMessage();
impl /*struct*/ QErrorMessage {
  pub fn Free<RetType, T: QErrorMessage_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QErrorMessage_Free<RetType> {
  fn Free(self , rsthis: & QErrorMessage) -> RetType;
}

  // proto:  void QErrorMessage::~QErrorMessage();
impl<'a> /*trait*/ QErrorMessage_Free<()> for () {
  fn Free(self , rsthis: & QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageD0Ev()};
     unsafe {_ZN13QErrorMessageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::QErrorMessage(const QErrorMessage & );
impl<'a> /*trait*/ QErrorMessage_New for (&'a QErrorMessage) {
  fn New(self) -> QErrorMessage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageC1ERKS_()};
    let ctysz: c_int = unsafe{QErrorMessage_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QErrorMessageC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QErrorMessageC1ERKS_(arg0)};
    let rsthis = QErrorMessage{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

