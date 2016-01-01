// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qtoolbutton.h
// dst-file: /src/widgets/qtoolbutton.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qabstractbutton::QAbstractButton; // 773
use std::ops::Deref;
use super::qaction::QAction; // 773
use super::super::core::qsize::QSize; // 771
use super::qmenu::QMenu; // 773
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QToolButton_Class_Size() -> c_int;
  // proto:  void QToolButton::setAutoRaise(bool enable);
  fn _ZN11QToolButton12setAutoRaiseEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QAction * QToolButton::defaultAction();
  fn _ZNK11QToolButton13defaultActionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QToolButton::metaObject();
  fn _ZNK11QToolButton10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QToolButton::minimumSizeHint();
  fn _ZNK11QToolButton15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QToolButton::~QToolButton();
  fn _ZN11QToolButtonD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QToolButton::showMenu();
  fn _ZN11QToolButton8showMenuEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QToolButton::sizeHint();
  fn _ZNK11QToolButton8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QToolButton::QToolButton(const QToolButton & );
  fn dector_ZN11QToolButtonC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QToolButtonC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QToolButton::autoRaise();
  fn _ZNK11QToolButton9autoRaiseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QMenu * QToolButton::menu();
  fn _ZNK11QToolButton4menuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QToolButton::setMenu(QMenu * menu);
  fn _ZN11QToolButton7setMenuEP5QMenu(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QToolButton::QToolButton(QWidget * parent);
  fn dector_ZN11QToolButtonC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QToolButtonC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QToolButton::setDefaultAction(QAction * );
  fn _ZN11QToolButton16setDefaultActionEP7QAction(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QToolButton_SlotProxy_connect__ZN11QToolButton9triggeredEP7QAction(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QToolButton)=1
#[derive(Default)]
pub struct QToolButton {
  qbase: QAbstractButton,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _triggered: QToolButton_triggered_signal,
}

impl /*struct*/ QToolButton {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QToolButton {
    return QToolButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QToolButton {
  type Target = QAbstractButton;

  fn deref(&self) -> &QAbstractButton {
    return & self.qbase;
  }
}
impl AsRef<QAbstractButton> for QToolButton {
  fn as_ref(& self) -> & QAbstractButton {
    return & self.qbase;
  }
}
  // proto:  void QToolButton::setAutoRaise(bool enable);
impl /*struct*/ QToolButton {
  pub fn setAutoRaise<RetType, T: QToolButton_setAutoRaise<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRaise(self);
    // return 1;
  }
}

pub trait QToolButton_setAutoRaise<RetType> {
  fn setAutoRaise(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  void QToolButton::setAutoRaise(bool enable);
impl<'a> /*trait*/ QToolButton_setAutoRaise<()> for (i8) {
  fn setAutoRaise(self , rsthis: & QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton12setAutoRaiseEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QToolButton12setAutoRaiseEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAction * QToolButton::defaultAction();
impl /*struct*/ QToolButton {
  pub fn defaultAction<RetType, T: QToolButton_defaultAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultAction(self);
    // return 1;
  }
}

pub trait QToolButton_defaultAction<RetType> {
  fn defaultAction(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  QAction * QToolButton::defaultAction();
impl<'a> /*trait*/ QToolButton_defaultAction<QAction> for () {
  fn defaultAction(self , rsthis: & QToolButton) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton13defaultActionEv()};
    let mut ret = unsafe {_ZNK11QToolButton13defaultActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QToolButton::metaObject();
impl /*struct*/ QToolButton {
  pub fn metaObject<RetType, T: QToolButton_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QToolButton_metaObject<RetType> {
  fn metaObject(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  const QMetaObject * QToolButton::metaObject();
impl<'a> /*trait*/ QToolButton_metaObject<()> for () {
  fn metaObject(self , rsthis: & QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton10metaObjectEv()};
     unsafe {_ZNK11QToolButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QToolButton::minimumSizeHint();
impl /*struct*/ QToolButton {
  pub fn minimumSizeHint<RetType, T: QToolButton_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QToolButton_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  QSize QToolButton::minimumSizeHint();
impl<'a> /*trait*/ QToolButton_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QToolButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK11QToolButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolButton::~QToolButton();
impl /*struct*/ QToolButton {
  pub fn free<RetType, T: QToolButton_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QToolButton_free<RetType> {
  fn free(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  void QToolButton::~QToolButton();
impl<'a> /*trait*/ QToolButton_free<()> for () {
  fn free(self , rsthis: & QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonD0Ev()};
     unsafe {_ZN11QToolButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QToolButton::showMenu();
impl /*struct*/ QToolButton {
  pub fn showMenu<RetType, T: QToolButton_showMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMenu(self);
    // return 1;
  }
}

pub trait QToolButton_showMenu<RetType> {
  fn showMenu(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  void QToolButton::showMenu();
impl<'a> /*trait*/ QToolButton_showMenu<()> for () {
  fn showMenu(self , rsthis: & QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton8showMenuEv()};
     unsafe {_ZN11QToolButton8showMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QToolButton::sizeHint();
impl /*struct*/ QToolButton {
  pub fn sizeHint<RetType, T: QToolButton_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QToolButton_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  QSize QToolButton::sizeHint();
impl<'a> /*trait*/ QToolButton_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QToolButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QToolButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolButton::QToolButton(const QToolButton & );
impl /*struct*/ QToolButton {
  pub fn new<T: QToolButton_new>(value: T) -> QToolButton {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QToolButton_new {
  fn new(self) -> QToolButton;
}

  // proto:  void QToolButton::QToolButton(const QToolButton & );
impl<'a> /*trait*/ QToolButton_new for (&'a QToolButton) {
  fn new(self) -> QToolButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonC1ERKS_()};
    let ctysz: c_int = unsafe{QToolButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QToolButtonC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QToolButtonC1ERKS_(arg0)} as u64;
    let rsthis = QToolButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QToolButton::autoRaise();
impl /*struct*/ QToolButton {
  pub fn autoRaise<RetType, T: QToolButton_autoRaise<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRaise(self);
    // return 1;
  }
}

pub trait QToolButton_autoRaise<RetType> {
  fn autoRaise(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  bool QToolButton::autoRaise();
impl<'a> /*trait*/ QToolButton_autoRaise<i8> for () {
  fn autoRaise(self , rsthis: & QToolButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton9autoRaiseEv()};
    let mut ret = unsafe {_ZNK11QToolButton9autoRaiseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMenu * QToolButton::menu();
impl /*struct*/ QToolButton {
  pub fn menu<RetType, T: QToolButton_menu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.menu(self);
    // return 1;
  }
}

pub trait QToolButton_menu<RetType> {
  fn menu(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  QMenu * QToolButton::menu();
impl<'a> /*trait*/ QToolButton_menu<QMenu> for () {
  fn menu(self , rsthis: & QToolButton) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QToolButton4menuEv()};
    let mut ret = unsafe {_ZNK11QToolButton4menuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolButton::setMenu(QMenu * menu);
impl /*struct*/ QToolButton {
  pub fn setMenu<RetType, T: QToolButton_setMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMenu(self);
    // return 1;
  }
}

pub trait QToolButton_setMenu<RetType> {
  fn setMenu(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  void QToolButton::setMenu(QMenu * menu);
impl<'a> /*trait*/ QToolButton_setMenu<()> for (&'a QMenu) {
  fn setMenu(self , rsthis: & QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QToolButton7setMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QToolButton::QToolButton(QWidget * parent);
impl<'a> /*trait*/ QToolButton_new for (&'a QWidget) {
  fn new(self) -> QToolButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButtonC1EP7QWidget()};
    let ctysz: c_int = unsafe{QToolButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QToolButtonC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QToolButtonC1EP7QWidget(arg0)} as u64;
    let rsthis = QToolButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolButton::setDefaultAction(QAction * );
impl /*struct*/ QToolButton {
  pub fn setDefaultAction<RetType, T: QToolButton_setDefaultAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultAction(self);
    // return 1;
  }
}

pub trait QToolButton_setDefaultAction<RetType> {
  fn setDefaultAction(self , rsthis: & QToolButton) -> RetType;
}

  // proto:  void QToolButton::setDefaultAction(QAction * );
impl<'a> /*trait*/ QToolButton_setDefaultAction<()> for (&'a QAction) {
  fn setDefaultAction(self , rsthis: & QToolButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QToolButton16setDefaultActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QToolButton16setDefaultActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QToolButton_triggered
pub struct QToolButton_triggered_signal{poi:u64}
impl /* struct */ QToolButton {
  pub fn triggered(&self) -> QToolButton_triggered_signal {
     return QToolButton_triggered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QToolButton_triggered_signal {
  pub fn connect<T: QToolButton_triggered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QToolButton_triggered_signal_connect {
  fn connect(self, sigthis: QToolButton_triggered_signal);
}

// triggered(class QAction *)
extern fn QToolButton_triggered_signal_connect_cb_0(rsfptr:fn(QAction), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QToolButton_triggered_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QAction)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QAction::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QToolButton_triggered_signal_connect for fn(QAction) {
  fn connect(self, sigthis: QToolButton_triggered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolButton_triggered_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QToolButton_SlotProxy_connect__ZN11QToolButton9triggeredEP7QAction(arg0, arg1, arg2)};
  }
}
impl /* trait */ QToolButton_triggered_signal_connect for Box<Fn(QAction)> {
  fn connect(self, sigthis: QToolButton_triggered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QToolButton_triggered_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QToolButton_SlotProxy_connect__ZN11QToolButton9triggeredEP7QAction(arg0, arg1, arg2)};
  }
}
// <= body block end

