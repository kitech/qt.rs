// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaction::QAction;
use super::qmenu::QMenu;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QToolButton::setAutoRaise(bool enable);
  fn _ZN11QToolButton12setAutoRaiseEb(arg0: int8_t) -> i32;
  // proto: QAction * QToolButton::defaultAction();
  fn _ZNK11QToolButton13defaultActionEv() -> i32;
  // proto: void QToolButton::triggered(QAction * );
  fn _ZN11QToolButton9triggeredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QToolButton::metaObject();
  fn _ZNK11QToolButton10metaObjectEv() -> i32;
  // proto: QSize QToolButton::minimumSizeHint();
  fn _ZNK11QToolButton15minimumSizeHintEv() -> i32;
  // proto: void QToolButton::FreeQToolButton();
  fn _ZN11QToolButtonD0Ev() -> i32;
  // proto: void QToolButton::showMenu();
  fn _ZN11QToolButton8showMenuEv() -> i32;
  // proto: QSize QToolButton::sizeHint();
  fn _ZNK11QToolButton8sizeHintEv() -> i32;
  // proto: void QToolButton::NewQToolButton(const QToolButton & );
  fn _ZN11QToolButtonC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QToolButton::autoRaise();
  fn _ZNK11QToolButton9autoRaiseEv() -> i32;
  // proto: QMenu * QToolButton::menu();
  fn _ZNK11QToolButton4menuEv() -> i32;
  // proto: void QToolButton::setMenu(QMenu * menu);
  fn _ZN11QToolButton7setMenuEP5QMenu(arg0: *mut c_void) -> i32;
  // proto: void QToolButton::NewQToolButton(QWidget * parent);
  fn _ZN11QToolButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QToolButton::setDefaultAction(QAction * );
  fn _ZN11QToolButton16setDefaultActionEP7QAction(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QToolButton)=1
pub struct QToolButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolButton {
  pub fn setAutoRaise<T: QToolButton_setAutoRaise>(&mut self, value: T) -> i32 {
    value.setAutoRaise(self);
    return 1;
  }
}

pub trait QToolButton_setAutoRaise {
  fn setAutoRaise(self, this: &mut QToolButton) -> i32;
}

// proto: void QToolButton::setAutoRaise(bool enable);
impl<'a> /*trait*/ QToolButton_setAutoRaise for (i8) {
  fn setAutoRaise(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton12setAutoRaiseEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QToolButton12setAutoRaiseEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn defaultAction<T: QToolButton_defaultAction>(&mut self, value: T) -> i32 {
    value.defaultAction(self);
    return 1;
  }
}

pub trait QToolButton_defaultAction {
  fn defaultAction(self, this: &mut QToolButton) -> i32;
}

// proto: QAction * QToolButton::defaultAction();
impl<'a> /*trait*/ QToolButton_defaultAction for () {
  fn defaultAction(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton13defaultActionEv()};
    unsafe {_ZNK11QToolButton13defaultActionEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn triggered<T: QToolButton_triggered>(&mut self, value: T) -> i32 {
    value.triggered(self);
    return 1;
  }
}

pub trait QToolButton_triggered {
  fn triggered(self, this: &mut QToolButton) -> i32;
}

// proto: void QToolButton::triggered(QAction * );
impl<'a> /*trait*/ QToolButton_triggered for (&'a mut QAction) {
  fn triggered(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QToolButton9triggeredEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn metaObject<T: QToolButton_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QToolButton_metaObject {
  fn metaObject(self, this: &mut QToolButton) -> i32;
}

// proto: const QMetaObject * QToolButton::metaObject();
impl<'a> /*trait*/ QToolButton_metaObject for () {
  fn metaObject(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton10metaObjectEv()};
    unsafe {_ZNK11QToolButton10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn minimumSizeHint<T: QToolButton_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QToolButton_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QToolButton) -> i32;
}

// proto: QSize QToolButton::minimumSizeHint();
impl<'a> /*trait*/ QToolButton_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton15minimumSizeHintEv()};
    unsafe {_ZNK11QToolButton15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn FreeQToolButton<T: QToolButton_FreeQToolButton>(&mut self, value: T) -> i32 {
    value.FreeQToolButton(self);
    return 1;
  }
}

pub trait QToolButton_FreeQToolButton {
  fn FreeQToolButton(self, this: &mut QToolButton) -> i32;
}

// proto: void QToolButton::FreeQToolButton();
impl<'a> /*trait*/ QToolButton_FreeQToolButton for () {
  fn FreeQToolButton(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonD0Ev()};
    unsafe {_ZN11QToolButtonD0Ev()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn showMenu<T: QToolButton_showMenu>(&mut self, value: T) -> i32 {
    value.showMenu(self);
    return 1;
  }
}

pub trait QToolButton_showMenu {
  fn showMenu(self, this: &mut QToolButton) -> i32;
}

// proto: void QToolButton::showMenu();
impl<'a> /*trait*/ QToolButton_showMenu for () {
  fn showMenu(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton8showMenuEv()};
    unsafe {_ZN11QToolButton8showMenuEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn sizeHint<T: QToolButton_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QToolButton_sizeHint {
  fn sizeHint(self, this: &mut QToolButton) -> i32;
}

// proto: QSize QToolButton::sizeHint();
impl<'a> /*trait*/ QToolButton_sizeHint for () {
  fn sizeHint(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton8sizeHintEv()};
    unsafe {_ZNK11QToolButton8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn NewQToolButton<T: QToolButton_NewQToolButton>(value: T) -> QToolButton {
    let rsthis = value.NewQToolButton();
    return rsthis;
    // return 1;
  }
}

pub trait QToolButton_NewQToolButton {
  fn NewQToolButton(self) -> QToolButton;
}

// proto: void QToolButton::NewQToolButton(const QToolButton & );
impl<'a> /*trait*/ QToolButton_NewQToolButton for (&'a  QToolButton) {
  fn NewQToolButton(self) -> QToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QToolButtonC1ERKS_(qthis, arg0)};
    let rsthis = QToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn autoRaise<T: QToolButton_autoRaise>(&mut self, value: T) -> i32 {
    value.autoRaise(self);
    return 1;
  }
}

pub trait QToolButton_autoRaise {
  fn autoRaise(self, this: &mut QToolButton) -> i32;
}

// proto: bool QToolButton::autoRaise();
impl<'a> /*trait*/ QToolButton_autoRaise for () {
  fn autoRaise(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton9autoRaiseEv()};
    unsafe {_ZNK11QToolButton9autoRaiseEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn menu<T: QToolButton_menu>(&mut self, value: T) -> i32 {
    value.menu(self);
    return 1;
  }
}

pub trait QToolButton_menu {
  fn menu(self, this: &mut QToolButton) -> i32;
}

// proto: QMenu * QToolButton::menu();
impl<'a> /*trait*/ QToolButton_menu for () {
  fn menu(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton4menuEv()};
    unsafe {_ZNK11QToolButton4menuEv()};
    return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn setMenu<T: QToolButton_setMenu>(&mut self, value: T) -> i32 {
    value.setMenu(self);
    return 1;
  }
}

pub trait QToolButton_setMenu {
  fn setMenu(self, this: &mut QToolButton) -> i32;
}

// proto: void QToolButton::setMenu(QMenu * menu);
impl<'a> /*trait*/ QToolButton_setMenu for (&'a mut QMenu) {
  fn setMenu(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QToolButton7setMenuEP5QMenu(arg0)};
    return 1;
  }
}

// proto: void QToolButton::NewQToolButton(QWidget * parent);
impl<'a> /*trait*/ QToolButton_NewQToolButton for (&'a mut QWidget) {
  fn NewQToolButton(self) -> QToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QToolButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn setDefaultAction<T: QToolButton_setDefaultAction>(&mut self, value: T) -> i32 {
    value.setDefaultAction(self);
    return 1;
  }
}

pub trait QToolButton_setDefaultAction {
  fn setDefaultAction(self, this: &mut QToolButton) -> i32;
}

// proto: void QToolButton::setDefaultAction(QAction * );
impl<'a> /*trait*/ QToolButton_setDefaultAction for (&'a mut QAction) {
  fn setDefaultAction(self, this: &mut QToolButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton16setDefaultActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QToolButton16setDefaultActionEP7QAction(arg0)};
    return 1;
  }
}

