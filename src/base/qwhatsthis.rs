// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QWhatsThis::hideText();
  fn _ZN10QWhatsThis8hideTextEv() -> i32;
  // proto: void QWhatsThis::enterWhatsThisMode();
  fn _ZN10QWhatsThis18enterWhatsThisModeEv() -> i32;
  // proto: bool QWhatsThis::inWhatsThisMode();
  fn _ZN10QWhatsThis15inWhatsThisModeEv() -> i32;
  // proto: void QWhatsThis::leaveWhatsThisMode();
  fn _ZN10QWhatsThis18leaveWhatsThisModeEv() -> i32;
  // proto: void QWhatsThis::NewQWhatsThis();
  fn _ZN10QWhatsThisC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
  fn _ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget(arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: QAction * QWhatsThis::createAction(QObject * parent);
  fn _ZN10QWhatsThis12createActionEP7QObject(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QWhatsThis)=1
pub struct QWhatsThis {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWhatsThis {
  pub fn hideText<T: QWhatsThis_hideText>(&mut self, value: T) -> i32 {
    value.hideText(self);
    return 1;
  }
}

pub trait QWhatsThis_hideText {
  fn hideText(self, this: &mut QWhatsThis) -> i32;
}

// proto: void QWhatsThis::hideText();
impl<'a> /*trait*/ QWhatsThis_hideText for () {
  fn hideText(self, this: &mut QWhatsThis) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis8hideTextEv()};
    unsafe {_ZN10QWhatsThis8hideTextEv()};
    return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn enterWhatsThisMode<T: QWhatsThis_enterWhatsThisMode>(&mut self, value: T) -> i32 {
    value.enterWhatsThisMode(self);
    return 1;
  }
}

pub trait QWhatsThis_enterWhatsThisMode {
  fn enterWhatsThisMode(self, this: &mut QWhatsThis) -> i32;
}

// proto: void QWhatsThis::enterWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_enterWhatsThisMode for () {
  fn enterWhatsThisMode(self, this: &mut QWhatsThis) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis18enterWhatsThisModeEv()};
    unsafe {_ZN10QWhatsThis18enterWhatsThisModeEv()};
    return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn inWhatsThisMode<T: QWhatsThis_inWhatsThisMode>(&mut self, value: T) -> i32 {
    value.inWhatsThisMode(self);
    return 1;
  }
}

pub trait QWhatsThis_inWhatsThisMode {
  fn inWhatsThisMode(self, this: &mut QWhatsThis) -> i32;
}

// proto: bool QWhatsThis::inWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_inWhatsThisMode for () {
  fn inWhatsThisMode(self, this: &mut QWhatsThis) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis15inWhatsThisModeEv()};
    unsafe {_ZN10QWhatsThis15inWhatsThisModeEv()};
    return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn leaveWhatsThisMode<T: QWhatsThis_leaveWhatsThisMode>(&mut self, value: T) -> i32 {
    value.leaveWhatsThisMode(self);
    return 1;
  }
}

pub trait QWhatsThis_leaveWhatsThisMode {
  fn leaveWhatsThisMode(self, this: &mut QWhatsThis) -> i32;
}

// proto: void QWhatsThis::leaveWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_leaveWhatsThisMode for () {
  fn leaveWhatsThisMode(self, this: &mut QWhatsThis) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis18leaveWhatsThisModeEv()};
    unsafe {_ZN10QWhatsThis18leaveWhatsThisModeEv()};
    return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn NewQWhatsThis<T: QWhatsThis_NewQWhatsThis>(value: T) -> QWhatsThis {
    let rsthis = value.NewQWhatsThis();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThis_NewQWhatsThis {
  fn NewQWhatsThis(self) -> QWhatsThis;
}

// proto: void QWhatsThis::NewQWhatsThis();
impl<'a> /*trait*/ QWhatsThis_NewQWhatsThis for () {
  fn NewQWhatsThis(self) -> QWhatsThis {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThisC1Ev()};
    unsafe {_ZN10QWhatsThisC1Ev(qthis)};
    let rsthis = QWhatsThis{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn showText<T: QWhatsThis_showText>(&mut self, value: T) -> i32 {
    value.showText(self);
    return 1;
  }
}

pub trait QWhatsThis_showText {
  fn showText(self, this: &mut QWhatsThis) -> i32;
}

// proto: void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QWhatsThis_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget) {
  fn showText(self, this: &mut QWhatsThis) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QWhatsThis {
  pub fn createAction<T: QWhatsThis_createAction>(&mut self, value: T) -> i32 {
    value.createAction(self);
    return 1;
  }
}

pub trait QWhatsThis_createAction {
  fn createAction(self, this: &mut QWhatsThis) -> i32;
}

// proto: QAction * QWhatsThis::createAction(QObject * parent);
impl<'a> /*trait*/ QWhatsThis_createAction for (&'a mut QObject) {
  fn createAction(self, this: &mut QWhatsThis) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis12createActionEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QWhatsThis12createActionEP7QObject(arg0)};
    return 1;
  }
}

