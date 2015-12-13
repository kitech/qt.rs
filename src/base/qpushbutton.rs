// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmenu::QMenu;
use super::qicon::QIcon;
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QPushButton::setMenu(QMenu * menu);
  fn _ZN11QPushButton7setMenuEP5QMenu(arg0: *mut c_void) -> i32;
  // proto: void QPushButton::setFlat(bool );
  fn _ZN11QPushButton7setFlatEb(arg0: int8_t) -> i32;
  // proto: void QPushButton::setAutoDefault(bool );
  fn _ZN11QPushButton14setAutoDefaultEb(arg0: int8_t) -> i32;
  // proto: QSize QPushButton::minimumSizeHint();
  fn _ZNK11QPushButton15minimumSizeHintEv() -> i32;
  // proto: void QPushButton::setDefault(bool );
  fn _ZN11QPushButton10setDefaultEb(arg0: int8_t) -> i32;
  // proto: void QPushButton::FreeQPushButton();
  fn _ZN11QPushButtonD0Ev() -> i32;
  // proto: void QPushButton::NewQPushButton(const QPushButton & );
  fn _ZN11QPushButtonC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QPushButton::isDefault();
  fn _ZNK11QPushButton9isDefaultEv() -> i32;
  // proto: void QPushButton::NewQPushButton(const QIcon & icon, const QString & text, QWidget * parent);
  fn _ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: bool QPushButton::autoDefault();
  fn _ZNK11QPushButton11autoDefaultEv() -> i32;
  // proto: QSize QPushButton::sizeHint();
  fn _ZNK11QPushButton8sizeHintEv() -> i32;
  // proto: const QMetaObject * QPushButton::metaObject();
  fn _ZNK11QPushButton10metaObjectEv() -> i32;
  // proto: QMenu * QPushButton::menu();
  fn _ZNK11QPushButton4menuEv() -> i32;
  // proto: void QPushButton::NewQPushButton(QWidget * parent);
  fn _ZN11QPushButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QPushButton::showMenu();
  fn _ZN11QPushButton8showMenuEv() -> i32;
  // proto: void QPushButton::NewQPushButton(const QString & text, QWidget * parent);
  fn _ZN11QPushButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: bool QPushButton::isFlat();
  fn _ZNK11QPushButton6isFlatEv() -> i32;
}

// body block begin
// class sizeof(QPushButton)=1
pub struct QPushButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPushButton {
  pub fn setMenu<T: QPushButton_setMenu>(&mut self, value: T) -> i32 {
    value.setMenu(self);
    return 1;
  }
}

pub trait QPushButton_setMenu {
  fn setMenu(self, this: &mut QPushButton) -> i32;
}

// proto: void QPushButton::setMenu(QMenu * menu);
impl<'a> /*trait*/ QPushButton_setMenu for (&'a mut QMenu) {
  fn setMenu(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButton7setMenuEP5QMenu(arg0)};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn setFlat<T: QPushButton_setFlat>(&mut self, value: T) -> i32 {
    value.setFlat(self);
    return 1;
  }
}

pub trait QPushButton_setFlat {
  fn setFlat(self, this: &mut QPushButton) -> i32;
}

// proto: void QPushButton::setFlat(bool );
impl<'a> /*trait*/ QPushButton_setFlat for (i8) {
  fn setFlat(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton7setFlatEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QPushButton7setFlatEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn setAutoDefault<T: QPushButton_setAutoDefault>(&mut self, value: T) -> i32 {
    value.setAutoDefault(self);
    return 1;
  }
}

pub trait QPushButton_setAutoDefault {
  fn setAutoDefault(self, this: &mut QPushButton) -> i32;
}

// proto: void QPushButton::setAutoDefault(bool );
impl<'a> /*trait*/ QPushButton_setAutoDefault for (i8) {
  fn setAutoDefault(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton14setAutoDefaultEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QPushButton14setAutoDefaultEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn minimumSizeHint<T: QPushButton_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QPushButton_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QPushButton) -> i32;
}

// proto: QSize QPushButton::minimumSizeHint();
impl<'a> /*trait*/ QPushButton_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton15minimumSizeHintEv()};
    unsafe {_ZNK11QPushButton15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn setDefault<T: QPushButton_setDefault>(&mut self, value: T) -> i32 {
    value.setDefault(self);
    return 1;
  }
}

pub trait QPushButton_setDefault {
  fn setDefault(self, this: &mut QPushButton) -> i32;
}

// proto: void QPushButton::setDefault(bool );
impl<'a> /*trait*/ QPushButton_setDefault for (i8) {
  fn setDefault(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton10setDefaultEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QPushButton10setDefaultEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn FreeQPushButton<T: QPushButton_FreeQPushButton>(&mut self, value: T) -> i32 {
    value.FreeQPushButton(self);
    return 1;
  }
}

pub trait QPushButton_FreeQPushButton {
  fn FreeQPushButton(self, this: &mut QPushButton) -> i32;
}

// proto: void QPushButton::FreeQPushButton();
impl<'a> /*trait*/ QPushButton_FreeQPushButton for () {
  fn FreeQPushButton(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonD0Ev()};
    unsafe {_ZN11QPushButtonD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn NewQPushButton<T: QPushButton_NewQPushButton>(value: T) -> QPushButton {
    let rsthis = value.NewQPushButton();
    return rsthis;
    // return 1;
  }
}

pub trait QPushButton_NewQPushButton {
  fn NewQPushButton(self) -> QPushButton;
}

// proto: void QPushButton::NewQPushButton(const QPushButton & );
impl<'a> /*trait*/ QPushButton_NewQPushButton for (&'a  QPushButton) {
  fn NewQPushButton(self) -> QPushButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPushButtonC1ERKS_(qthis, arg0)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn isDefault<T: QPushButton_isDefault>(&mut self, value: T) -> i32 {
    value.isDefault(self);
    return 1;
  }
}

pub trait QPushButton_isDefault {
  fn isDefault(self, this: &mut QPushButton) -> i32;
}

// proto: bool QPushButton::isDefault();
impl<'a> /*trait*/ QPushButton_isDefault for () {
  fn isDefault(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton9isDefaultEv()};
    unsafe {_ZNK11QPushButton9isDefaultEv()};
    return 1;
  }
}

// proto: void QPushButton::NewQPushButton(const QIcon & icon, const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPushButton_NewQPushButton for (&'a  QIcon, &'a  QString, &'a mut QWidget) {
  fn NewQPushButton(self) -> QPushButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(qthis, arg0, arg1, arg2)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn autoDefault<T: QPushButton_autoDefault>(&mut self, value: T) -> i32 {
    value.autoDefault(self);
    return 1;
  }
}

pub trait QPushButton_autoDefault {
  fn autoDefault(self, this: &mut QPushButton) -> i32;
}

// proto: bool QPushButton::autoDefault();
impl<'a> /*trait*/ QPushButton_autoDefault for () {
  fn autoDefault(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton11autoDefaultEv()};
    unsafe {_ZNK11QPushButton11autoDefaultEv()};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn sizeHint<T: QPushButton_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QPushButton_sizeHint {
  fn sizeHint(self, this: &mut QPushButton) -> i32;
}

// proto: QSize QPushButton::sizeHint();
impl<'a> /*trait*/ QPushButton_sizeHint for () {
  fn sizeHint(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton8sizeHintEv()};
    unsafe {_ZNK11QPushButton8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn metaObject<T: QPushButton_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QPushButton_metaObject {
  fn metaObject(self, this: &mut QPushButton) -> i32;
}

// proto: const QMetaObject * QPushButton::metaObject();
impl<'a> /*trait*/ QPushButton_metaObject for () {
  fn metaObject(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton10metaObjectEv()};
    unsafe {_ZNK11QPushButton10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn menu<T: QPushButton_menu>(&mut self, value: T) -> i32 {
    value.menu(self);
    return 1;
  }
}

pub trait QPushButton_menu {
  fn menu(self, this: &mut QPushButton) -> i32;
}

// proto: QMenu * QPushButton::menu();
impl<'a> /*trait*/ QPushButton_menu for () {
  fn menu(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton4menuEv()};
    unsafe {_ZNK11QPushButton4menuEv()};
    return 1;
  }
}

// proto: void QPushButton::NewQPushButton(QWidget * parent);
impl<'a> /*trait*/ QPushButton_NewQPushButton for (&'a mut QWidget) {
  fn NewQPushButton(self) -> QPushButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn showMenu<T: QPushButton_showMenu>(&mut self, value: T) -> i32 {
    value.showMenu(self);
    return 1;
  }
}

pub trait QPushButton_showMenu {
  fn showMenu(self, this: &mut QPushButton) -> i32;
}

// proto: void QPushButton::showMenu();
impl<'a> /*trait*/ QPushButton_showMenu for () {
  fn showMenu(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton8showMenuEv()};
    unsafe {_ZN11QPushButton8showMenuEv()};
    return 1;
  }
}

// proto: void QPushButton::NewQPushButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPushButton_NewQPushButton for (&'a  QString, &'a mut QWidget) {
  fn NewQPushButton(self) -> QPushButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn isFlat<T: QPushButton_isFlat>(&mut self, value: T) -> i32 {
    value.isFlat(self);
    return 1;
  }
}

pub trait QPushButton_isFlat {
  fn isFlat(self, this: &mut QPushButton) -> i32;
}

// proto: bool QPushButton::isFlat();
impl<'a> /*trait*/ QPushButton_isFlat for () {
  fn isFlat(self, this: &mut QPushButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton6isFlatEv()};
    unsafe {_ZNK11QPushButton6isFlatEv()};
    return 1;
  }
}

