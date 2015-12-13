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
  fn _ZNK13QErrorMessage10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QErrorMessage::NewQErrorMessage(QWidget * parent);
  fn _ZN13QErrorMessageC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QErrorMessage * QErrorMessage::qtHandler();
  fn _ZN13QErrorMessage9qtHandlerEv() -> *mut c_void;
  // proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
  fn _ZN13QErrorMessage11showMessageERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QErrorMessage::showMessage(const QString & message);
  fn _ZN13QErrorMessage11showMessageERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QErrorMessage::FreeQErrorMessage();
  fn _ZN13QErrorMessageD0Ev(qthis: *mut c_void) ;
  // proto:  void QErrorMessage::NewQErrorMessage(const QErrorMessage & );
  fn _ZN13QErrorMessageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QErrorMessage)=1
pub struct QErrorMessage {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QErrorMessage {
  pub fn metaObject<T: QErrorMessage_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QErrorMessage_metaObject {
  fn metaObject(self, rsthis: &mut QErrorMessage) ;
}

// proto:  const QMetaObject * QErrorMessage::metaObject();
impl<'a> /*trait*/ QErrorMessage_metaObject for () {
  fn metaObject(self, rsthis: &mut QErrorMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QErrorMessage10metaObjectEv()};
     unsafe {_ZNK13QErrorMessage10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

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

// proto: void QErrorMessage::NewQErrorMessage(QWidget * parent);
impl<'a> /*trait*/ QErrorMessage_NewQErrorMessage for (&'a mut QWidget) {
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

impl /*struct*/ QErrorMessage {
  pub fn qtHandler<T: QErrorMessage_qtHandler>(&mut self, value: T) -> QErrorMessage {
    return value.qtHandler(self);
    // return 1;
  }
}

pub trait QErrorMessage_qtHandler {
  fn qtHandler(self, rsthis: &mut QErrorMessage) -> QErrorMessage;
}

// proto: static QErrorMessage * QErrorMessage::qtHandler();
impl<'a> /*trait*/ QErrorMessage_qtHandler for () {
  fn qtHandler(self, rsthis: &mut QErrorMessage) -> QErrorMessage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage9qtHandlerEv()};
    let mut ret = unsafe {_ZN13QErrorMessage9qtHandlerEv()};
    let mut ret1 = QErrorMessage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QErrorMessage {
  pub fn showMessage<T: QErrorMessage_showMessage>(&mut self, value: T)  {
     value.showMessage(self);
    // return 1;
  }
}

pub trait QErrorMessage_showMessage {
  fn showMessage(self, rsthis: &mut QErrorMessage) ;
}

// proto:  void QErrorMessage::showMessage(const QString & message, const QString & type);
impl<'a> /*trait*/ QErrorMessage_showMessage for (&'a  QString, &'a  QString) {
  fn showMessage(self, rsthis: &mut QErrorMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage11showMessageERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QErrorMessage11showMessageERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QErrorMessage::showMessage(const QString & message);
impl<'a> /*trait*/ QErrorMessage_showMessage for (&'a  QString) {
  fn showMessage(self, rsthis: &mut QErrorMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessage11showMessageERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QErrorMessage11showMessageERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QErrorMessage {
  pub fn FreeQErrorMessage<T: QErrorMessage_FreeQErrorMessage>(&mut self, value: T)  {
     value.FreeQErrorMessage(self);
    // return 1;
  }
}

pub trait QErrorMessage_FreeQErrorMessage {
  fn FreeQErrorMessage(self, rsthis: &mut QErrorMessage) ;
}

// proto:  void QErrorMessage::FreeQErrorMessage();
impl<'a> /*trait*/ QErrorMessage_FreeQErrorMessage for () {
  fn FreeQErrorMessage(self, rsthis: &mut QErrorMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QErrorMessageD0Ev()};
     unsafe {_ZN13QErrorMessageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QErrorMessage::NewQErrorMessage(const QErrorMessage & );
impl<'a> /*trait*/ QErrorMessage_NewQErrorMessage for (&'a  QErrorMessage) {
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

