// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLDebugMessage::NewQOpenGLDebugMessage();
  fn _ZN19QOpenGLDebugMessageC1Ev(qthis: *mut c_void) ;
  // proto:  void QOpenGLDebugMessage::FreeQOpenGLDebugMessage();
  fn _ZN19QOpenGLDebugMessageD0Ev(qthis: *mut c_void) ;
  // proto:  QOpenGLDebugMessage::GLuint QOpenGLDebugMessage::id();
  fn _ZNK19QOpenGLDebugMessage2idEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLDebugMessage::NewQOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
  fn _ZN19QOpenGLDebugMessageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QOpenGLDebugMessage::message();
  fn _ZNK19QOpenGLDebugMessage7messageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
  fn _ZN19QOpenGLDebugMessage4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLDebugMessage)=1
pub struct QOpenGLDebugMessage {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn NewQOpenGLDebugMessage<T: QOpenGLDebugMessage_NewQOpenGLDebugMessage>(value: T) -> QOpenGLDebugMessage {
    let rsthis = value.NewQOpenGLDebugMessage();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_NewQOpenGLDebugMessage {
  fn NewQOpenGLDebugMessage(self) -> QOpenGLDebugMessage;
}

// proto: void QOpenGLDebugMessage::NewQOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_NewQOpenGLDebugMessage for () {
  fn NewQOpenGLDebugMessage(self) -> QOpenGLDebugMessage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageC1Ev()};
    unsafe {_ZN19QOpenGLDebugMessageC1Ev(qthis)};
    let rsthis = QOpenGLDebugMessage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn FreeQOpenGLDebugMessage<T: QOpenGLDebugMessage_FreeQOpenGLDebugMessage>(&mut self, value: T)  {
     value.FreeQOpenGLDebugMessage(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_FreeQOpenGLDebugMessage {
  fn FreeQOpenGLDebugMessage(self, rsthis: &mut QOpenGLDebugMessage) ;
}

// proto:  void QOpenGLDebugMessage::FreeQOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_FreeQOpenGLDebugMessage for () {
  fn FreeQOpenGLDebugMessage(self, rsthis: &mut QOpenGLDebugMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageD0Ev()};
     unsafe {_ZN19QOpenGLDebugMessageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn id<T: QOpenGLDebugMessage_id>(&mut self, value: T)  {
     value.id(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_id {
  fn id(self, rsthis: &mut QOpenGLDebugMessage) ;
}

// proto:  QOpenGLDebugMessage::GLuint QOpenGLDebugMessage::id();
impl<'a> /*trait*/ QOpenGLDebugMessage_id for () {
  fn id(self, rsthis: &mut QOpenGLDebugMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage2idEv()};
     unsafe {_ZNK19QOpenGLDebugMessage2idEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLDebugMessage::NewQOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_NewQOpenGLDebugMessage for (&'a  QOpenGLDebugMessage) {
  fn NewQOpenGLDebugMessage(self) -> QOpenGLDebugMessage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QOpenGLDebugMessageC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLDebugMessage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn message<T: QOpenGLDebugMessage_message>(&mut self, value: T) -> QString {
    return value.message(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_message {
  fn message(self, rsthis: &mut QOpenGLDebugMessage) -> QString;
}

// proto:  QString QOpenGLDebugMessage::message();
impl<'a> /*trait*/ QOpenGLDebugMessage_message for () {
  fn message(self, rsthis: &mut QOpenGLDebugMessage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage7messageEv()};
    let mut ret = unsafe {_ZNK19QOpenGLDebugMessage7messageEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOpenGLDebugMessage {
  pub fn swap<T: QOpenGLDebugMessage_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_swap {
  fn swap(self, rsthis: &mut QOpenGLDebugMessage) ;
}

// proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_swap for (&'a mut QOpenGLDebugMessage) {
  fn swap(self, rsthis: &mut QOpenGLDebugMessage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QOpenGLDebugMessage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

