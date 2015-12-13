// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmenu::QMenu;
use super::qsize::QSize;
use super::qicon::QIcon;
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPushButton::setMenu(QMenu * menu);
  fn _ZN11QPushButton7setMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPushButton::setFlat(bool );
  fn _ZN11QPushButton7setFlatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPushButton::setAutoDefault(bool );
  fn _ZN11QPushButton14setAutoDefaultEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QPushButton::minimumSizeHint();
  fn _ZNK11QPushButton15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPushButton::setDefault(bool );
  fn _ZN11QPushButton10setDefaultEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPushButton::FreeQPushButton();
  fn _ZN11QPushButtonD0Ev(qthis: *mut c_void) ;
  // proto:  void QPushButton::NewQPushButton(const QPushButton & );
  fn _ZN11QPushButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPushButton::isDefault();
  fn _ZNK11QPushButton9isDefaultEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPushButton::NewQPushButton(const QIcon & icon, const QString & text, QWidget * parent);
  fn _ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QPushButton::autoDefault();
  fn _ZNK11QPushButton11autoDefaultEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QPushButton::sizeHint();
  fn _ZNK11QPushButton8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QPushButton::metaObject();
  fn _ZNK11QPushButton10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QMenu * QPushButton::menu();
  fn _ZNK11QPushButton4menuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPushButton::NewQPushButton(QWidget * parent);
  fn _ZN11QPushButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPushButton::showMenu();
  fn _ZN11QPushButton8showMenuEv(qthis: *mut c_void) ;
  // proto:  void QPushButton::NewQPushButton(const QString & text, QWidget * parent);
  fn _ZN11QPushButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QPushButton::isFlat();
  fn _ZNK11QPushButton6isFlatEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QPushButton)=1
pub struct QPushButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPushButton {
  pub fn setMenu<T: QPushButton_setMenu>(&mut self, value: T)  {
     value.setMenu(self);
    // return 1;
  }
}

pub trait QPushButton_setMenu {
  fn setMenu(self, rsthis: &mut QPushButton) ;
}

// proto:  void QPushButton::setMenu(QMenu * menu);
impl<'a> /*trait*/ QPushButton_setMenu for (&'a mut QMenu) {
  fn setMenu(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPushButton7setMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn setFlat<T: QPushButton_setFlat>(&mut self, value: T)  {
     value.setFlat(self);
    // return 1;
  }
}

pub trait QPushButton_setFlat {
  fn setFlat(self, rsthis: &mut QPushButton) ;
}

// proto:  void QPushButton::setFlat(bool );
impl<'a> /*trait*/ QPushButton_setFlat for (i8) {
  fn setFlat(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton7setFlatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QPushButton7setFlatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn setAutoDefault<T: QPushButton_setAutoDefault>(&mut self, value: T)  {
     value.setAutoDefault(self);
    // return 1;
  }
}

pub trait QPushButton_setAutoDefault {
  fn setAutoDefault(self, rsthis: &mut QPushButton) ;
}

// proto:  void QPushButton::setAutoDefault(bool );
impl<'a> /*trait*/ QPushButton_setAutoDefault for (i8) {
  fn setAutoDefault(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton14setAutoDefaultEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QPushButton14setAutoDefaultEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn minimumSizeHint<T: QPushButton_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QPushButton_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QPushButton) -> QSize;
}

// proto:  QSize QPushButton::minimumSizeHint();
impl<'a> /*trait*/ QPushButton_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QPushButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK11QPushButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn setDefault<T: QPushButton_setDefault>(&mut self, value: T)  {
     value.setDefault(self);
    // return 1;
  }
}

pub trait QPushButton_setDefault {
  fn setDefault(self, rsthis: &mut QPushButton) ;
}

// proto:  void QPushButton::setDefault(bool );
impl<'a> /*trait*/ QPushButton_setDefault for (i8) {
  fn setDefault(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton10setDefaultEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QPushButton10setDefaultEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn FreeQPushButton<T: QPushButton_FreeQPushButton>(&mut self, value: T)  {
     value.FreeQPushButton(self);
    // return 1;
  }
}

pub trait QPushButton_FreeQPushButton {
  fn FreeQPushButton(self, rsthis: &mut QPushButton) ;
}

// proto:  void QPushButton::FreeQPushButton();
impl<'a> /*trait*/ QPushButton_FreeQPushButton for () {
  fn FreeQPushButton(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonD0Ev()};
     unsafe {_ZN11QPushButtonD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButtonC1ERKS_(qthis, arg0)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn isDefault<T: QPushButton_isDefault>(&mut self, value: T) -> i8 {
    return value.isDefault(self);
    // return 1;
  }
}

pub trait QPushButton_isDefault {
  fn isDefault(self, rsthis: &mut QPushButton) -> i8;
}

// proto:  bool QPushButton::isDefault();
impl<'a> /*trait*/ QPushButton_isDefault for () {
  fn isDefault(self, rsthis: &mut QPushButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton9isDefaultEv()};
    let mut ret = unsafe {_ZNK11QPushButton9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QPushButton::NewQPushButton(const QIcon & icon, const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPushButton_NewQPushButton for (&'a  QIcon, &'a  QString, &'a mut QWidget) {
  fn NewQPushButton(self) -> QPushButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(qthis, arg0, arg1, arg2)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn autoDefault<T: QPushButton_autoDefault>(&mut self, value: T) -> i8 {
    return value.autoDefault(self);
    // return 1;
  }
}

pub trait QPushButton_autoDefault {
  fn autoDefault(self, rsthis: &mut QPushButton) -> i8;
}

// proto:  bool QPushButton::autoDefault();
impl<'a> /*trait*/ QPushButton_autoDefault for () {
  fn autoDefault(self, rsthis: &mut QPushButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton11autoDefaultEv()};
    let mut ret = unsafe {_ZNK11QPushButton11autoDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn sizeHint<T: QPushButton_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QPushButton_sizeHint {
  fn sizeHint(self, rsthis: &mut QPushButton) -> QSize;
}

// proto:  QSize QPushButton::sizeHint();
impl<'a> /*trait*/ QPushButton_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QPushButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QPushButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn metaObject<T: QPushButton_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QPushButton_metaObject {
  fn metaObject(self, rsthis: &mut QPushButton) ;
}

// proto:  const QMetaObject * QPushButton::metaObject();
impl<'a> /*trait*/ QPushButton_metaObject for () {
  fn metaObject(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton10metaObjectEv()};
     unsafe {_ZNK11QPushButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn menu<T: QPushButton_menu>(&mut self, value: T) -> QMenu {
    return value.menu(self);
    // return 1;
  }
}

pub trait QPushButton_menu {
  fn menu(self, rsthis: &mut QPushButton) -> QMenu;
}

// proto:  QMenu * QPushButton::menu();
impl<'a> /*trait*/ QPushButton_menu for () {
  fn menu(self, rsthis: &mut QPushButton) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton4menuEv()};
    let mut ret = unsafe {_ZNK11QPushButton4menuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn showMenu<T: QPushButton_showMenu>(&mut self, value: T)  {
     value.showMenu(self);
    // return 1;
  }
}

pub trait QPushButton_showMenu {
  fn showMenu(self, rsthis: &mut QPushButton) ;
}

// proto:  void QPushButton::showMenu();
impl<'a> /*trait*/ QPushButton_showMenu for () {
  fn showMenu(self, rsthis: &mut QPushButton)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton8showMenuEv()};
     unsafe {_ZN11QPushButton8showMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QPushButton::NewQPushButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPushButton_NewQPushButton for (&'a  QString, &'a mut QWidget) {
  fn NewQPushButton(self) -> QPushButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QPushButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QPushButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPushButton {
  pub fn isFlat<T: QPushButton_isFlat>(&mut self, value: T) -> i8 {
    return value.isFlat(self);
    // return 1;
  }
}

pub trait QPushButton_isFlat {
  fn isFlat(self, rsthis: &mut QPushButton) -> i8;
}

// proto:  bool QPushButton::isFlat();
impl<'a> /*trait*/ QPushButton_isFlat for () {
  fn isFlat(self, rsthis: &mut QPushButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton6isFlatEv()};
    let mut ret = unsafe {_ZNK11QPushButton6isFlatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

