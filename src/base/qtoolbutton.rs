// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaction::QAction;
use super::qsize::QSize;
use super::qmenu::QMenu;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QToolButton::setAutoRaise(bool enable);
  fn _ZN11QToolButton12setAutoRaiseEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QAction * QToolButton::defaultAction();
  fn _ZNK11QToolButton13defaultActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolButton::triggered(QAction * );
  fn _ZN11QToolButton9triggeredEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QToolButton::metaObject();
  fn _ZNK11QToolButton10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QToolButton::minimumSizeHint();
  fn _ZNK11QToolButton15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolButton::FreeQToolButton();
  fn _ZN11QToolButtonD0Ev(qthis: *mut c_void) ;
  // proto:  void QToolButton::showMenu();
  fn _ZN11QToolButton8showMenuEv(qthis: *mut c_void) ;
  // proto:  QSize QToolButton::sizeHint();
  fn _ZNK11QToolButton8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolButton::NewQToolButton(const QToolButton & );
  fn _ZN11QToolButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QToolButton::autoRaise();
  fn _ZNK11QToolButton9autoRaiseEv(qthis: *mut c_void) -> int8_t;
  // proto:  QMenu * QToolButton::menu();
  fn _ZNK11QToolButton4menuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QToolButton::setMenu(QMenu * menu);
  fn _ZN11QToolButton7setMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QToolButton::NewQToolButton(QWidget * parent);
  fn _ZN11QToolButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QToolButton::setDefaultAction(QAction * );
  fn _ZN11QToolButton16setDefaultActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QToolButton)=1
pub struct QToolButton {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolButton {
  pub fn setAutoRaise<RetType, T: QToolButton_setAutoRaise<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoRaise(self);
    // return 1;
  }
}

pub trait QToolButton_setAutoRaise<RetType> {
  fn setAutoRaise(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  void QToolButton::setAutoRaise(bool enable);
impl<'a> /*trait*/ QToolButton_setAutoRaise<()> for (i8) {
  fn setAutoRaise(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton12setAutoRaiseEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QToolButton12setAutoRaiseEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn defaultAction<RetType, T: QToolButton_defaultAction<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultAction(self);
    // return 1;
  }
}

pub trait QToolButton_defaultAction<RetType> {
  fn defaultAction(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  QAction * QToolButton::defaultAction();
impl<'a> /*trait*/ QToolButton_defaultAction<QAction> for () {
  fn defaultAction(self, rsthis: &mut QToolButton) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton13defaultActionEv()};
    let mut ret = unsafe {_ZNK11QToolButton13defaultActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn triggered<RetType, T: QToolButton_triggered<RetType>>(&mut self, value: T) -> RetType {
    return value.triggered(self);
    // return 1;
  }
}

pub trait QToolButton_triggered<RetType> {
  fn triggered(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  void QToolButton::triggered(QAction * );
impl<'a> /*trait*/ QToolButton_triggered<()> for (&'a mut QAction) {
  fn triggered(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QToolButton9triggeredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn metaObject<RetType, T: QToolButton_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QToolButton_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  const QMetaObject * QToolButton::metaObject();
impl<'a> /*trait*/ QToolButton_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton10metaObjectEv()};
     unsafe {_ZNK11QToolButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn minimumSizeHint<RetType, T: QToolButton_minimumSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QToolButton_minimumSizeHint<RetType> {
  fn minimumSizeHint(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  QSize QToolButton::minimumSizeHint();
impl<'a> /*trait*/ QToolButton_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self, rsthis: &mut QToolButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK11QToolButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn FreeQToolButton<RetType, T: QToolButton_FreeQToolButton<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQToolButton(self);
    // return 1;
  }
}

pub trait QToolButton_FreeQToolButton<RetType> {
  fn FreeQToolButton(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  void QToolButton::FreeQToolButton();
impl<'a> /*trait*/ QToolButton_FreeQToolButton<()> for () {
  fn FreeQToolButton(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonD0Ev()};
     unsafe {_ZN11QToolButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn showMenu<RetType, T: QToolButton_showMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.showMenu(self);
    // return 1;
  }
}

pub trait QToolButton_showMenu<RetType> {
  fn showMenu(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  void QToolButton::showMenu();
impl<'a> /*trait*/ QToolButton_showMenu<()> for () {
  fn showMenu(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton8showMenuEv()};
     unsafe {_ZN11QToolButton8showMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn sizeHint<RetType, T: QToolButton_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QToolButton_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  QSize QToolButton::sizeHint();
impl<'a> /*trait*/ QToolButton_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QToolButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QToolButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QToolButtonC1ERKS_(qthis, arg0)};
    let rsthis = QToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn autoRaise<RetType, T: QToolButton_autoRaise<RetType>>(&mut self, value: T) -> RetType {
    return value.autoRaise(self);
    // return 1;
  }
}

pub trait QToolButton_autoRaise<RetType> {
  fn autoRaise(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  bool QToolButton::autoRaise();
impl<'a> /*trait*/ QToolButton_autoRaise<i8> for () {
  fn autoRaise(self, rsthis: &mut QToolButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton9autoRaiseEv()};
    let mut ret = unsafe {_ZNK11QToolButton9autoRaiseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn menu<RetType, T: QToolButton_menu<RetType>>(&mut self, value: T) -> RetType {
    return value.menu(self);
    // return 1;
  }
}

pub trait QToolButton_menu<RetType> {
  fn menu(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  QMenu * QToolButton::menu();
impl<'a> /*trait*/ QToolButton_menu<QMenu> for () {
  fn menu(self, rsthis: &mut QToolButton) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton4menuEv()};
    let mut ret = unsafe {_ZNK11QToolButton4menuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolButton {
  pub fn setMenu<RetType, T: QToolButton_setMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.setMenu(self);
    // return 1;
  }
}

pub trait QToolButton_setMenu<RetType> {
  fn setMenu(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  void QToolButton::setMenu(QMenu * menu);
impl<'a> /*trait*/ QToolButton_setMenu<()> for (&'a mut QMenu) {
  fn setMenu(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QToolButton7setMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn setDefaultAction<RetType, T: QToolButton_setDefaultAction<RetType>>(&mut self, value: T) -> RetType {
    return value.setDefaultAction(self);
    // return 1;
  }
}

pub trait QToolButton_setDefaultAction<RetType> {
  fn setDefaultAction(self, rsthis: &mut QToolButton) -> RetType;
}

// proto:  void QToolButton::setDefaultAction(QAction * );
impl<'a> /*trait*/ QToolButton_setDefaultAction<()> for (&'a mut QAction) {
  fn setDefaultAction(self, rsthis: &mut QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton16setDefaultActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QToolButton16setDefaultActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

