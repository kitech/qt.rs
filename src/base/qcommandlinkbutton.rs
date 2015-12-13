// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QCommandLinkButton::NewQCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
  fn _ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: const QMetaObject * QCommandLinkButton::metaObject();
  fn _ZNK18QCommandLinkButton10metaObjectEv() -> i32;
  // proto: void QCommandLinkButton::FreeQCommandLinkButton();
  fn _ZN18QCommandLinkButtonD0Ev() -> i32;
  // proto: void QCommandLinkButton::NewQCommandLinkButton(const QCommandLinkButton & );
  fn _ZN18QCommandLinkButtonC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QCommandLinkButton::NewQCommandLinkButton(QWidget * parent);
  fn _ZN18QCommandLinkButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QString QCommandLinkButton::description();
  fn _ZNK18QCommandLinkButton11descriptionEv() -> i32;
  // proto: void QCommandLinkButton::NewQCommandLinkButton(const QString & text, QWidget * parent);
  fn _ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QCommandLinkButton::setDescription(const QString & description);
  fn _ZN18QCommandLinkButton14setDescriptionERK7QString(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QCommandLinkButton)=1
pub struct QCommandLinkButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCommandLinkButton {
  pub fn NewQCommandLinkButton<T: QCommandLinkButton_NewQCommandLinkButton>(value: T) -> QCommandLinkButton {
    let rsthis = value.NewQCommandLinkButton();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLinkButton_NewQCommandLinkButton {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton;
}

// proto: void QCommandLinkButton::NewQCommandLinkButton(const QString & text, const QString & description, QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (&'a  QString, &'a  QString, &'a mut QWidget) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1ERK7QStringS2_P7QWidget(qthis, arg0, arg1, arg2)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLinkButton {
  pub fn metaObject<T: QCommandLinkButton_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QCommandLinkButton_metaObject {
  fn metaObject(self, this: &mut QCommandLinkButton) -> i32;
}

// proto: const QMetaObject * QCommandLinkButton::metaObject();
impl<'a> /*trait*/ QCommandLinkButton_metaObject for () {
  fn metaObject(self, this: &mut QCommandLinkButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLinkButton10metaObjectEv()};
    unsafe {_ZNK18QCommandLinkButton10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QCommandLinkButton {
  pub fn FreeQCommandLinkButton<T: QCommandLinkButton_FreeQCommandLinkButton>(&mut self, value: T) -> i32 {
    value.FreeQCommandLinkButton(self);
    return 1;
  }
}

pub trait QCommandLinkButton_FreeQCommandLinkButton {
  fn FreeQCommandLinkButton(self, this: &mut QCommandLinkButton) -> i32;
}

// proto: void QCommandLinkButton::FreeQCommandLinkButton();
impl<'a> /*trait*/ QCommandLinkButton_FreeQCommandLinkButton for () {
  fn FreeQCommandLinkButton(self, this: &mut QCommandLinkButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonD0Ev()};
    unsafe {_ZN18QCommandLinkButtonD0Ev()};
    return 1;
  }
}

// proto: void QCommandLinkButton::NewQCommandLinkButton(const QCommandLinkButton & );
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (&'a  QCommandLinkButton) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLinkButtonC1ERKS_(qthis, arg0)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QCommandLinkButton::NewQCommandLinkButton(QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (&'a mut QWidget) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLinkButton {
  pub fn description<T: QCommandLinkButton_description>(&mut self, value: T) -> i32 {
    value.description(self);
    return 1;
  }
}

pub trait QCommandLinkButton_description {
  fn description(self, this: &mut QCommandLinkButton) -> i32;
}

// proto: QString QCommandLinkButton::description();
impl<'a> /*trait*/ QCommandLinkButton_description for () {
  fn description(self, this: &mut QCommandLinkButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLinkButton11descriptionEv()};
    unsafe {_ZNK18QCommandLinkButton11descriptionEv()};
    return 1;
  }
}

// proto: void QCommandLinkButton::NewQCommandLinkButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QCommandLinkButton_NewQCommandLinkButton for (&'a  QString, &'a mut QWidget) {
  fn NewQCommandLinkButton(self) -> QCommandLinkButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLinkButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QCommandLinkButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCommandLinkButton {
  pub fn setDescription<T: QCommandLinkButton_setDescription>(&mut self, value: T) -> i32 {
    value.setDescription(self);
    return 1;
  }
}

pub trait QCommandLinkButton_setDescription {
  fn setDescription(self, this: &mut QCommandLinkButton) -> i32;
}

// proto: void QCommandLinkButton::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLinkButton_setDescription for (&'a  QString) {
  fn setDescription(self, this: &mut QCommandLinkButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLinkButton14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN18QCommandLinkButton14setDescriptionERK7QString(arg0)};
    return 1;
  }
}

