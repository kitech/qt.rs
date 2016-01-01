// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qpushbutton.h
// dst-file: /src/widgets/qpushbutton.rs
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
use super::qmenu::QMenu; // 773
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPushButton_Class_Size() -> c_int;
  // proto:  void QPushButton::setMenu(QMenu * menu);
  fn _ZN11QPushButton7setMenuEP5QMenu(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPushButton::setFlat(bool );
  fn _ZN11QPushButton7setFlatEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QPushButton::setAutoDefault(bool );
  fn _ZN11QPushButton14setAutoDefaultEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QPushButton::minimumSizeHint();
  fn _ZNK11QPushButton15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPushButton::setDefault(bool );
  fn _ZN11QPushButton10setDefaultEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QPushButton::~QPushButton();
  fn _ZN11QPushButtonD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPushButton::QPushButton(const QPushButton & );
  fn dector_ZN11QPushButtonC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QPushButtonC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QPushButton::isDefault();
  fn _ZNK11QPushButton9isDefaultEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPushButton::QPushButton(const QIcon & icon, const QString & text, QWidget * parent);
  fn dector_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QPushButton::autoDefault();
  fn _ZNK11QPushButton11autoDefaultEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QPushButton::sizeHint();
  fn _ZNK11QPushButton8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QPushButton::metaObject();
  fn _ZNK11QPushButton10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QMenu * QPushButton::menu();
  fn _ZNK11QPushButton4menuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPushButton::QPushButton(QWidget * parent);
  fn dector_ZN11QPushButtonC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QPushButtonC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPushButton::showMenu();
  fn _ZN11QPushButton8showMenuEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPushButton::QPushButton(const QString & text, QWidget * parent);
  fn dector_ZN11QPushButtonC1ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN11QPushButtonC1ERK7QStringP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QPushButton::isFlat();
  fn _ZNK11QPushButton6isFlatEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QPushButton)=1
#[derive(Default)]
pub struct QPushButton {
  qbase: QAbstractButton,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPushButton {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPushButton {
    return QPushButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPushButton {
  type Target = QAbstractButton;

  fn deref(&self) -> &QAbstractButton {
    return & self.qbase;
  }
}
impl AsRef<QAbstractButton> for QPushButton {
  fn as_ref(& self) -> & QAbstractButton {
    return & self.qbase;
  }
}
  // proto:  void QPushButton::setMenu(QMenu * menu);
impl /*struct*/ QPushButton {
  pub fn setMenu<RetType, T: QPushButton_setMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMenu(self);
    // return 1;
  }
}

pub trait QPushButton_setMenu<RetType> {
  fn setMenu(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  void QPushButton::setMenu(QMenu * menu);
impl<'a> /*trait*/ QPushButton_setMenu<()> for (&'a QMenu) {
  fn setMenu(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPushButton7setMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPushButton::setFlat(bool );
impl /*struct*/ QPushButton {
  pub fn setFlat<RetType, T: QPushButton_setFlat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFlat(self);
    // return 1;
  }
}

pub trait QPushButton_setFlat<RetType> {
  fn setFlat(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  void QPushButton::setFlat(bool );
impl<'a> /*trait*/ QPushButton_setFlat<()> for (i8) {
  fn setFlat(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton7setFlatEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QPushButton7setFlatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPushButton::setAutoDefault(bool );
impl /*struct*/ QPushButton {
  pub fn setAutoDefault<RetType, T: QPushButton_setAutoDefault<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoDefault(self);
    // return 1;
  }
}

pub trait QPushButton_setAutoDefault<RetType> {
  fn setAutoDefault(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  void QPushButton::setAutoDefault(bool );
impl<'a> /*trait*/ QPushButton_setAutoDefault<()> for (i8) {
  fn setAutoDefault(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton14setAutoDefaultEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QPushButton14setAutoDefaultEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QPushButton::minimumSizeHint();
impl /*struct*/ QPushButton {
  pub fn minimumSizeHint<RetType, T: QPushButton_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QPushButton_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  QSize QPushButton::minimumSizeHint();
impl<'a> /*trait*/ QPushButton_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QPushButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK11QPushButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPushButton::setDefault(bool );
impl /*struct*/ QPushButton {
  pub fn setDefault<RetType, T: QPushButton_setDefault<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefault(self);
    // return 1;
  }
}

pub trait QPushButton_setDefault<RetType> {
  fn setDefault(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  void QPushButton::setDefault(bool );
impl<'a> /*trait*/ QPushButton_setDefault<()> for (i8) {
  fn setDefault(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton10setDefaultEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QPushButton10setDefaultEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPushButton::~QPushButton();
impl /*struct*/ QPushButton {
  pub fn free<RetType, T: QPushButton_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPushButton_free<RetType> {
  fn free(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  void QPushButton::~QPushButton();
impl<'a> /*trait*/ QPushButton_free<()> for () {
  fn free(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonD0Ev()};
     unsafe {_ZN11QPushButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPushButton::QPushButton(const QPushButton & );
impl /*struct*/ QPushButton {
  pub fn new<T: QPushButton_new>(value: T) -> QPushButton {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPushButton_new {
  fn new(self) -> QPushButton;
}

  // proto:  void QPushButton::QPushButton(const QPushButton & );
impl<'a> /*trait*/ QPushButton_new for (&'a QPushButton) {
  fn new(self) -> QPushButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERKS_()};
    let ctysz: c_int = unsafe{QPushButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QPushButtonC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QPushButtonC1ERKS_(arg0)} as u64;
    let rsthis = QPushButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPushButton::isDefault();
impl /*struct*/ QPushButton {
  pub fn isDefault<RetType, T: QPushButton_isDefault<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDefault(self);
    // return 1;
  }
}

pub trait QPushButton_isDefault<RetType> {
  fn isDefault(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  bool QPushButton::isDefault();
impl<'a> /*trait*/ QPushButton_isDefault<i8> for () {
  fn isDefault(self , rsthis: & QPushButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton9isDefaultEv()};
    let mut ret = unsafe {_ZNK11QPushButton9isDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPushButton::QPushButton(const QIcon & icon, const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPushButton_new for (&'a QIcon, &'a QString, &'a QWidget) {
  fn new(self) -> QPushButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QPushButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(qthis, arg0, arg1, arg2)};
    let qthis: u64 = unsafe {dector_ZN11QPushButtonC1ERK5QIconRK7QStringP7QWidget(arg0, arg1, arg2)} as u64;
    let rsthis = QPushButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPushButton::autoDefault();
impl /*struct*/ QPushButton {
  pub fn autoDefault<RetType, T: QPushButton_autoDefault<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoDefault(self);
    // return 1;
  }
}

pub trait QPushButton_autoDefault<RetType> {
  fn autoDefault(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  bool QPushButton::autoDefault();
impl<'a> /*trait*/ QPushButton_autoDefault<i8> for () {
  fn autoDefault(self , rsthis: & QPushButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton11autoDefaultEv()};
    let mut ret = unsafe {_ZNK11QPushButton11autoDefaultEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QPushButton::sizeHint();
impl /*struct*/ QPushButton {
  pub fn sizeHint<RetType, T: QPushButton_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QPushButton_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  QSize QPushButton::sizeHint();
impl<'a> /*trait*/ QPushButton_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QPushButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QPushButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPushButton::metaObject();
impl /*struct*/ QPushButton {
  pub fn metaObject<RetType, T: QPushButton_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPushButton_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  const QMetaObject * QPushButton::metaObject();
impl<'a> /*trait*/ QPushButton_metaObject<()> for () {
  fn metaObject(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton10metaObjectEv()};
     unsafe {_ZNK11QPushButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMenu * QPushButton::menu();
impl /*struct*/ QPushButton {
  pub fn menu<RetType, T: QPushButton_menu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.menu(self);
    // return 1;
  }
}

pub trait QPushButton_menu<RetType> {
  fn menu(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  QMenu * QPushButton::menu();
impl<'a> /*trait*/ QPushButton_menu<QMenu> for () {
  fn menu(self , rsthis: & QPushButton) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton4menuEv()};
    let mut ret = unsafe {_ZNK11QPushButton4menuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPushButton::QPushButton(QWidget * parent);
impl<'a> /*trait*/ QPushButton_new for (&'a QWidget) {
  fn new(self) -> QPushButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1EP7QWidget()};
    let ctysz: c_int = unsafe{QPushButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QPushButtonC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN11QPushButtonC1EP7QWidget(arg0)} as u64;
    let rsthis = QPushButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPushButton::showMenu();
impl /*struct*/ QPushButton {
  pub fn showMenu<RetType, T: QPushButton_showMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMenu(self);
    // return 1;
  }
}

pub trait QPushButton_showMenu<RetType> {
  fn showMenu(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  void QPushButton::showMenu();
impl<'a> /*trait*/ QPushButton_showMenu<()> for () {
  fn showMenu(self , rsthis: & QPushButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButton8showMenuEv()};
     unsafe {_ZN11QPushButton8showMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPushButton::QPushButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QPushButton_new for (&'a QString, &'a QWidget) {
  fn new(self) -> QPushButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPushButtonC1ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QPushButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN11QPushButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN11QPushButtonC1ERK7QStringP7QWidget(arg0, arg1)} as u64;
    let rsthis = QPushButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QPushButton::isFlat();
impl /*struct*/ QPushButton {
  pub fn isFlat<RetType, T: QPushButton_isFlat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFlat(self);
    // return 1;
  }
}

pub trait QPushButton_isFlat<RetType> {
  fn isFlat(self , rsthis: & QPushButton) -> RetType;
}

  // proto:  bool QPushButton::isFlat();
impl<'a> /*trait*/ QPushButton_isFlat<i8> for () {
  fn isFlat(self , rsthis: & QPushButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPushButton6isFlatEv()};
    let mut ret = unsafe {_ZNK11QPushButton6isFlatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

