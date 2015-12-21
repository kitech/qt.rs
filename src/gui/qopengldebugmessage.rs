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
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
  fn _ZN19QOpenGLDebugMessageC1Ev(qthis: *mut c_void);
  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
  fn _ZN19QOpenGLDebugMessageD0Ev(qthis: *mut c_void);
  // proto:  GLuint QOpenGLDebugMessage::id();
  fn _ZNK19QOpenGLDebugMessage2idEv(qthis: *mut c_void);
  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
  fn _ZN19QOpenGLDebugMessageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QOpenGLDebugMessage::message();
  fn _ZNK19QOpenGLDebugMessage7messageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
  fn _ZN19QOpenGLDebugMessage4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QOpenGLDebugMessage)=1
pub struct QOpenGLDebugMessage {
  pub qclsinst: *mut c_void,
}

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
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

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage();
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

  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn FreeQOpenGLDebugMessage<RetType, T: QOpenGLDebugMessage_FreeQOpenGLDebugMessage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQOpenGLDebugMessage(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_FreeQOpenGLDebugMessage<RetType> {
  fn FreeQOpenGLDebugMessage(self , rsthis: &mut QOpenGLDebugMessage) -> RetType;
}

  // proto:  void QOpenGLDebugMessage::~QOpenGLDebugMessage();
impl<'a> /*trait*/ QOpenGLDebugMessage_FreeQOpenGLDebugMessage<()> for () {
  fn FreeQOpenGLDebugMessage(self , rsthis: &mut QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessageD0Ev()};
     unsafe {_ZN19QOpenGLDebugMessageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLDebugMessage::id();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn id<RetType, T: QOpenGLDebugMessage_id<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.id(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_id<RetType> {
  fn id(self , rsthis: &mut QOpenGLDebugMessage) -> RetType;
}

  // proto:  GLuint QOpenGLDebugMessage::id();
impl<'a> /*trait*/ QOpenGLDebugMessage_id<()> for () {
  fn id(self , rsthis: &mut QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage2idEv()};
     unsafe {_ZNK19QOpenGLDebugMessage2idEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_NewQOpenGLDebugMessage for (QOpenGLDebugMessage) {
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

  // proto:  QString QOpenGLDebugMessage::message();
impl /*struct*/ QOpenGLDebugMessage {
  pub fn message<RetType, T: QOpenGLDebugMessage_message<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.message(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_message<RetType> {
  fn message(self , rsthis: &mut QOpenGLDebugMessage) -> RetType;
}

  // proto:  QString QOpenGLDebugMessage::message();
impl<'a> /*trait*/ QOpenGLDebugMessage_message<QString> for () {
  fn message(self , rsthis: &mut QOpenGLDebugMessage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QOpenGLDebugMessage7messageEv()};
    let mut ret = unsafe {_ZNK19QOpenGLDebugMessage7messageEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
impl /*struct*/ QOpenGLDebugMessage {
  pub fn swap<RetType, T: QOpenGLDebugMessage_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QOpenGLDebugMessage_swap<RetType> {
  fn swap(self , rsthis: &mut QOpenGLDebugMessage) -> RetType;
}

  // proto:  void QOpenGLDebugMessage::swap(QOpenGLDebugMessage & debugMessage);
impl<'a> /*trait*/ QOpenGLDebugMessage_swap<()> for (QOpenGLDebugMessage) {
  fn swap(self , rsthis: &mut QOpenGLDebugMessage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QOpenGLDebugMessage4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QOpenGLDebugMessage4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

