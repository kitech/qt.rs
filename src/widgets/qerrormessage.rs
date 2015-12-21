// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QErrorMessage::metaObject();
  fn _ZNK13QErrorMessage10metaObjectEv(qthis: *mut c_void);
  // proto:  void QErrorMessage::QErrorMessage(QWidget * parent);
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
  fn _ZN13QErrorMessageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QErrorMessage)=1
pub struct QErrorMessage {
  pub qclsinst: *mut c_void,
}

  // proto:  const QMetaObject * QErrorMessage::metaObject();
impl /*struct*/ QErrorMessage {
  pub fn metaObject<RetType, T: QErrorMessage_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QErrorMessage_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QErrorMessage) -> RetType;
}

  // proto:  const QMetaObject * QErrorMessage::metaObject();
impl<'a> /*trait*/ QErrorMessage_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QErrorMessage10metaObjectEv()};
     unsafe {_ZNK13QErrorMessage10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::QErrorMessage(QWidget * parent);
impl /*struct*/ QErrorMessage {
  pub fn NewQErrorMessage<T: QErrorMessage_NewQErrorMessage>(value: T) -> QErrorMessage {
    let rsthis = value.NewQErrorMessage();
    return rsthis;
    // return 1;
  }
}

pub trait QErrorMessage_NewQErrorMessage {
  fn NewQErrorMessage(self) -> QErrorMessage;
}

  // proto:  void QErrorMessage::QErrorMessage(QWidget * parent);
impl<'a> /*trait*/ QErrorMessage_NewQErrorMessage for (QWidget) {
  fn NewQErrorMessage(self) -> QErrorMessage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QErrorMessageC1EP7QWidget(qthis, arg0)};
    let rsthis = QErrorMessage{qclsinst: qthis};
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
    let mut ret1 = QErrorMessage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
impl /*struct*/ QErrorMessage {
  pub fn showMessage<RetType, T: QErrorMessage_showMessage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showMessage(self);
    // return 1;
  }
}

pub trait QErrorMessage_showMessage<RetType> {
  fn showMessage(self , rsthis: &mut QErrorMessage) -> RetType;
}

  // proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
impl<'a> /*trait*/ QErrorMessage_showMessage<()> for (QString, QString) {
  fn showMessage(self , rsthis: &mut QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage11showMessageERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QErrorMessage11showMessageERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::showMessage(const QString & message);
impl<'a> /*trait*/ QErrorMessage_showMessage<()> for (QString) {
  fn showMessage(self , rsthis: &mut QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage11showMessageERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QErrorMessage11showMessageERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::~QErrorMessage();
impl /*struct*/ QErrorMessage {
  pub fn FreeQErrorMessage<RetType, T: QErrorMessage_FreeQErrorMessage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQErrorMessage(self);
    // return 1;
  }
}

pub trait QErrorMessage_FreeQErrorMessage<RetType> {
  fn FreeQErrorMessage(self , rsthis: &mut QErrorMessage) -> RetType;
}

  // proto:  void QErrorMessage::~QErrorMessage();
impl<'a> /*trait*/ QErrorMessage_FreeQErrorMessage<()> for () {
  fn FreeQErrorMessage(self , rsthis: &mut QErrorMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageD0Ev()};
     unsafe {_ZN13QErrorMessageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QErrorMessage::QErrorMessage(const QErrorMessage & );
impl<'a> /*trait*/ QErrorMessage_NewQErrorMessage for (QErrorMessage) {
  fn NewQErrorMessage(self) -> QErrorMessage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QErrorMessageC1ERKS_(qthis, arg0)};
    let rsthis = QErrorMessage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

