// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qabstractbutton.h
// dst-file: /src/widgets/qabstractbutton.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qsize::QSize; // 771
use super::super::gui::qkeysequence::QKeySequence; // 771
use super::qbuttongroup::QButtonGroup; // 773
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractButton_Class_Size() -> c_int;
  // proto:  QSize QAbstractButton::iconSize();
  fn _ZNK15QAbstractButton8iconSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractButton::click();
  fn _ZN15QAbstractButton5clickEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractButton::~QAbstractButton();
  fn _ZN15QAbstractButtonD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractButton::setChecked(bool );
  fn _ZN15QAbstractButton10setCheckedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QKeySequence QAbstractButton::shortcut();
  fn _ZNK15QAbstractButton8shortcutEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QButtonGroup * QAbstractButton::group();
  fn _ZNK15QAbstractButton5groupEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QAbstractButton::isCheckable();
  fn _ZNK15QAbstractButton11isCheckableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractButton::QAbstractButton(QWidget * parent);
  fn dector_ZN15QAbstractButtonC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QAbstractButtonC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QAbstractButton::isDown();
  fn _ZNK15QAbstractButton6isDownEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractButton::setAutoExclusive(bool );
  fn _ZN15QAbstractButton16setAutoExclusiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QAbstractButton::metaObject();
  fn _ZNK15QAbstractButton10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAbstractButton::isChecked();
  fn _ZNK15QAbstractButton9isCheckedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractButton::setAutoRepeatDelay(int );
  fn _ZN15QAbstractButton18setAutoRepeatDelayEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QAbstractButton::autoRepeatDelay();
  fn _ZNK15QAbstractButton15autoRepeatDelayEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QAbstractButton::autoExclusive();
  fn _ZNK15QAbstractButton13autoExclusiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractButton::toggle();
  fn _ZN15QAbstractButton6toggleEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractButton::setIcon(const QIcon & icon);
  fn _ZN15QAbstractButton7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractButton::setAutoRepeatInterval(int );
  fn _ZN15QAbstractButton21setAutoRepeatIntervalEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractButton::setAutoRepeat(bool );
  fn _ZN15QAbstractButton13setAutoRepeatEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractButton::QAbstractButton(const QAbstractButton & );
  fn dector_ZN15QAbstractButtonC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN15QAbstractButtonC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractButton::animateClick(int msec);
  fn _ZN15QAbstractButton12animateClickEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractButton::setDown(bool );
  fn _ZN15QAbstractButton7setDownEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QAbstractButton::text();
  fn _ZNK15QAbstractButton4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractButton::setShortcut(const QKeySequence & key);
  fn _ZN15QAbstractButton11setShortcutERK12QKeySequence(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractButton::setCheckable(bool );
  fn _ZN15QAbstractButton12setCheckableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QIcon QAbstractButton::icon();
  fn _ZNK15QAbstractButton4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractButton::setText(const QString & text);
  fn _ZN15QAbstractButton7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QAbstractButton::autoRepeatInterval();
  fn _ZNK15QAbstractButton18autoRepeatIntervalEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QAbstractButton::autoRepeat();
  fn _ZNK15QAbstractButton10autoRepeatEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractButton::setIconSize(const QSize & size);
  fn _ZN15QAbstractButton11setIconSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7clickedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7toggledEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7pressedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractButton_SlotProxy_connect__ZN15QAbstractButton8releasedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractButton)=1
#[derive(Default)]
pub struct QAbstractButton {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _released: QAbstractButton_released_signal,
  pub _clicked: QAbstractButton_clicked_signal,
  pub _pressed: QAbstractButton_pressed_signal,
  pub _toggled: QAbstractButton_toggled_signal,
}

impl /*struct*/ QAbstractButton {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractButton {
    return QAbstractButton{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractButton {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QAbstractButton {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QSize QAbstractButton::iconSize();
impl /*struct*/ QAbstractButton {
  pub fn iconSize<RetType, T: QAbstractButton_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QAbstractButton_iconSize<RetType> {
  fn iconSize(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  QSize QAbstractButton::iconSize();
impl<'a> /*trait*/ QAbstractButton_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QAbstractButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton8iconSizeEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractButton::click();
impl /*struct*/ QAbstractButton {
  pub fn click<RetType, T: QAbstractButton_click<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.click(self);
    // return 1;
  }
}

pub trait QAbstractButton_click<RetType> {
  fn click(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::click();
impl<'a> /*trait*/ QAbstractButton_click<()> for () {
  fn click(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton5clickEv()};
     unsafe {_ZN15QAbstractButton5clickEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::~QAbstractButton();
impl /*struct*/ QAbstractButton {
  pub fn free<RetType, T: QAbstractButton_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractButton_free<RetType> {
  fn free(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::~QAbstractButton();
impl<'a> /*trait*/ QAbstractButton_free<()> for () {
  fn free(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButtonD0Ev()};
     unsafe {_ZN15QAbstractButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setChecked(bool );
impl /*struct*/ QAbstractButton {
  pub fn setChecked<RetType, T: QAbstractButton_setChecked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setChecked(self);
    // return 1;
  }
}

pub trait QAbstractButton_setChecked<RetType> {
  fn setChecked(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setChecked(bool );
impl<'a> /*trait*/ QAbstractButton_setChecked<()> for (i8) {
  fn setChecked(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton10setCheckedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QKeySequence QAbstractButton::shortcut();
impl /*struct*/ QAbstractButton {
  pub fn shortcut<RetType, T: QAbstractButton_shortcut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shortcut(self);
    // return 1;
  }
}

pub trait QAbstractButton_shortcut<RetType> {
  fn shortcut(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  QKeySequence QAbstractButton::shortcut();
impl<'a> /*trait*/ QAbstractButton_shortcut<QKeySequence> for () {
  fn shortcut(self , rsthis: & QAbstractButton) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton8shortcutEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton8shortcutEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QButtonGroup * QAbstractButton::group();
impl /*struct*/ QAbstractButton {
  pub fn group<RetType, T: QAbstractButton_group<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QAbstractButton_group<RetType> {
  fn group(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  QButtonGroup * QAbstractButton::group();
impl<'a> /*trait*/ QAbstractButton_group<QButtonGroup> for () {
  fn group(self , rsthis: & QAbstractButton) -> QButtonGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton5groupEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton5groupEv(rsthis.qclsinst)};
    let mut ret1 = QButtonGroup::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::isCheckable();
impl /*struct*/ QAbstractButton {
  pub fn isCheckable<RetType, T: QAbstractButton_isCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCheckable(self);
    // return 1;
  }
}

pub trait QAbstractButton_isCheckable<RetType> {
  fn isCheckable(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::isCheckable();
impl<'a> /*trait*/ QAbstractButton_isCheckable<i8> for () {
  fn isCheckable(self , rsthis: & QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton11isCheckableEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::QAbstractButton(QWidget * parent);
impl /*struct*/ QAbstractButton {
  pub fn new<T: QAbstractButton_new>(value: T) -> QAbstractButton {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractButton_new {
  fn new(self) -> QAbstractButton;
}

  // proto:  void QAbstractButton::QAbstractButton(QWidget * parent);
impl<'a> /*trait*/ QAbstractButton_new for (&'a QWidget) {
  fn new(self) -> QAbstractButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButtonC1EP7QWidget()};
    let ctysz: c_int = unsafe{QAbstractButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QAbstractButtonC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QAbstractButtonC1EP7QWidget(arg0)} as u64;
    let rsthis = QAbstractButton{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::isDown();
impl /*struct*/ QAbstractButton {
  pub fn isDown<RetType, T: QAbstractButton_isDown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDown(self);
    // return 1;
  }
}

pub trait QAbstractButton_isDown<RetType> {
  fn isDown(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::isDown();
impl<'a> /*trait*/ QAbstractButton_isDown<i8> for () {
  fn isDown(self , rsthis: & QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton6isDownEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton6isDownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoExclusive(bool );
impl /*struct*/ QAbstractButton {
  pub fn setAutoExclusive<RetType, T: QAbstractButton_setAutoExclusive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoExclusive(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoExclusive<RetType> {
  fn setAutoExclusive(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoExclusive(bool );
impl<'a> /*trait*/ QAbstractButton_setAutoExclusive<()> for (i8) {
  fn setAutoExclusive(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton16setAutoExclusiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton16setAutoExclusiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractButton::metaObject();
impl /*struct*/ QAbstractButton {
  pub fn metaObject<RetType, T: QAbstractButton_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractButton_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  const QMetaObject * QAbstractButton::metaObject();
impl<'a> /*trait*/ QAbstractButton_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton10metaObjectEv()};
     unsafe {_ZNK15QAbstractButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractButton::isChecked();
impl /*struct*/ QAbstractButton {
  pub fn isChecked<RetType, T: QAbstractButton_isChecked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isChecked(self);
    // return 1;
  }
}

pub trait QAbstractButton_isChecked<RetType> {
  fn isChecked(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::isChecked();
impl<'a> /*trait*/ QAbstractButton_isChecked<i8> for () {
  fn isChecked(self , rsthis: & QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton9isCheckedEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoRepeatDelay(int );
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeatDelay<RetType, T: QAbstractButton_setAutoRepeatDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeatDelay(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoRepeatDelay<RetType> {
  fn setAutoRepeatDelay(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoRepeatDelay(int );
impl<'a> /*trait*/ QAbstractButton_setAutoRepeatDelay<()> for (i32) {
  fn setAutoRepeatDelay(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton18setAutoRepeatDelayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractButton18setAutoRepeatDelayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractButton::autoRepeatDelay();
impl /*struct*/ QAbstractButton {
  pub fn autoRepeatDelay<RetType, T: QAbstractButton_autoRepeatDelay<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRepeatDelay(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoRepeatDelay<RetType> {
  fn autoRepeatDelay(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  int QAbstractButton::autoRepeatDelay();
impl<'a> /*trait*/ QAbstractButton_autoRepeatDelay<i32> for () {
  fn autoRepeatDelay(self , rsthis: & QAbstractButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton15autoRepeatDelayEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton15autoRepeatDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::autoExclusive();
impl /*struct*/ QAbstractButton {
  pub fn autoExclusive<RetType, T: QAbstractButton_autoExclusive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoExclusive(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoExclusive<RetType> {
  fn autoExclusive(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::autoExclusive();
impl<'a> /*trait*/ QAbstractButton_autoExclusive<i8> for () {
  fn autoExclusive(self , rsthis: & QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton13autoExclusiveEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton13autoExclusiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::toggle();
impl /*struct*/ QAbstractButton {
  pub fn toggle<RetType, T: QAbstractButton_toggle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggle(self);
    // return 1;
  }
}

pub trait QAbstractButton_toggle<RetType> {
  fn toggle(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::toggle();
impl<'a> /*trait*/ QAbstractButton_toggle<()> for () {
  fn toggle(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton6toggleEv()};
     unsafe {_ZN15QAbstractButton6toggleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setIcon(const QIcon & icon);
impl /*struct*/ QAbstractButton {
  pub fn setIcon<RetType, T: QAbstractButton_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QAbstractButton_setIcon<RetType> {
  fn setIcon(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QAbstractButton_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoRepeatInterval(int );
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeatInterval<RetType, T: QAbstractButton_setAutoRepeatInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeatInterval(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoRepeatInterval<RetType> {
  fn setAutoRepeatInterval(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoRepeatInterval(int );
impl<'a> /*trait*/ QAbstractButton_setAutoRepeatInterval<()> for (i32) {
  fn setAutoRepeatInterval(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton21setAutoRepeatIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractButton21setAutoRepeatIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoRepeat(bool );
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeat<RetType, T: QAbstractButton_setAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoRepeat<RetType> {
  fn setAutoRepeat(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoRepeat(bool );
impl<'a> /*trait*/ QAbstractButton_setAutoRepeat<()> for (i8) {
  fn setAutoRepeat(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton13setAutoRepeatEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::QAbstractButton(const QAbstractButton & );
impl<'a> /*trait*/ QAbstractButton_new for (&'a QAbstractButton) {
  fn new(self) -> QAbstractButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButtonC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN15QAbstractButtonC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN15QAbstractButtonC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractButton{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractButton::animateClick(int msec);
impl /*struct*/ QAbstractButton {
  pub fn animateClick<RetType, T: QAbstractButton_animateClick<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.animateClick(self);
    // return 1;
  }
}

pub trait QAbstractButton_animateClick<RetType> {
  fn animateClick(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::animateClick(int msec);
impl<'a> /*trait*/ QAbstractButton_animateClick<()> for (i32) {
  fn animateClick(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton12animateClickEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractButton12animateClickEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setDown(bool );
impl /*struct*/ QAbstractButton {
  pub fn setDown<RetType, T: QAbstractButton_setDown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDown(self);
    // return 1;
  }
}

pub trait QAbstractButton_setDown<RetType> {
  fn setDown(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setDown(bool );
impl<'a> /*trait*/ QAbstractButton_setDown<()> for (i8) {
  fn setDown(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7setDownEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton7setDownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QAbstractButton::text();
impl /*struct*/ QAbstractButton {
  pub fn text<RetType, T: QAbstractButton_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QAbstractButton_text<RetType> {
  fn text(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  QString QAbstractButton::text();
impl<'a> /*trait*/ QAbstractButton_text<QString> for () {
  fn text(self , rsthis: & QAbstractButton) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton4textEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setShortcut(const QKeySequence & key);
impl /*struct*/ QAbstractButton {
  pub fn setShortcut<RetType, T: QAbstractButton_setShortcut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShortcut(self);
    // return 1;
  }
}

pub trait QAbstractButton_setShortcut<RetType> {
  fn setShortcut(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setShortcut(const QKeySequence & key);
impl<'a> /*trait*/ QAbstractButton_setShortcut<()> for (&'a QKeySequence) {
  fn setShortcut(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton11setShortcutERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton11setShortcutERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setCheckable(bool );
impl /*struct*/ QAbstractButton {
  pub fn setCheckable<RetType, T: QAbstractButton_setCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCheckable(self);
    // return 1;
  }
}

pub trait QAbstractButton_setCheckable<RetType> {
  fn setCheckable(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setCheckable(bool );
impl<'a> /*trait*/ QAbstractButton_setCheckable<()> for (i8) {
  fn setCheckable(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton12setCheckableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QAbstractButton::icon();
impl /*struct*/ QAbstractButton {
  pub fn icon<RetType, T: QAbstractButton_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QAbstractButton_icon<RetType> {
  fn icon(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  QIcon QAbstractButton::icon();
impl<'a> /*trait*/ QAbstractButton_icon<QIcon> for () {
  fn icon(self , rsthis: & QAbstractButton) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton4iconEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setText(const QString & text);
impl /*struct*/ QAbstractButton {
  pub fn setText<RetType, T: QAbstractButton_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QAbstractButton_setText<RetType> {
  fn setText(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setText(const QString & text);
impl<'a> /*trait*/ QAbstractButton_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractButton::autoRepeatInterval();
impl /*struct*/ QAbstractButton {
  pub fn autoRepeatInterval<RetType, T: QAbstractButton_autoRepeatInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRepeatInterval(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoRepeatInterval<RetType> {
  fn autoRepeatInterval(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  int QAbstractButton::autoRepeatInterval();
impl<'a> /*trait*/ QAbstractButton_autoRepeatInterval<i32> for () {
  fn autoRepeatInterval(self , rsthis: & QAbstractButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton18autoRepeatIntervalEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton18autoRepeatIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::autoRepeat();
impl /*struct*/ QAbstractButton {
  pub fn autoRepeat<RetType, T: QAbstractButton_autoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoRepeat<RetType> {
  fn autoRepeat(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::autoRepeat();
impl<'a> /*trait*/ QAbstractButton_autoRepeat<i8> for () {
  fn autoRepeat(self , rsthis: & QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton10autoRepeatEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setIconSize(const QSize & size);
impl /*struct*/ QAbstractButton {
  pub fn setIconSize<RetType, T: QAbstractButton_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QAbstractButton_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setIconSize(const QSize & size);
impl<'a> /*trait*/ QAbstractButton_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QAbstractButton_released
pub struct QAbstractButton_released_signal{poi:u64}
impl /* struct */ QAbstractButton {
  pub fn released(&self) -> QAbstractButton_released_signal {
     return QAbstractButton_released_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractButton_released_signal {
  pub fn connect<T: QAbstractButton_released_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractButton_released_signal_connect {
  fn connect(self, sigthis: QAbstractButton_released_signal);
}

#[derive(Default)] // for QAbstractButton_clicked
pub struct QAbstractButton_clicked_signal{poi:u64}
impl /* struct */ QAbstractButton {
  pub fn clicked(&self) -> QAbstractButton_clicked_signal {
     return QAbstractButton_clicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractButton_clicked_signal {
  pub fn connect<T: QAbstractButton_clicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractButton_clicked_signal_connect {
  fn connect(self, sigthis: QAbstractButton_clicked_signal);
}

#[derive(Default)] // for QAbstractButton_pressed
pub struct QAbstractButton_pressed_signal{poi:u64}
impl /* struct */ QAbstractButton {
  pub fn pressed(&self) -> QAbstractButton_pressed_signal {
     return QAbstractButton_pressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractButton_pressed_signal {
  pub fn connect<T: QAbstractButton_pressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractButton_pressed_signal_connect {
  fn connect(self, sigthis: QAbstractButton_pressed_signal);
}

#[derive(Default)] // for QAbstractButton_toggled
pub struct QAbstractButton_toggled_signal{poi:u64}
impl /* struct */ QAbstractButton {
  pub fn toggled(&self) -> QAbstractButton_toggled_signal {
     return QAbstractButton_toggled_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractButton_toggled_signal {
  pub fn connect<T: QAbstractButton_toggled_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractButton_toggled_signal_connect {
  fn connect(self, sigthis: QAbstractButton_toggled_signal);
}

// clicked(_Bool)
extern fn QAbstractButton_clicked_signal_connect_cb_0(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QAbstractButton_clicked_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractButton_clicked_signal_connect for fn(i8) {
  fn connect(self, sigthis: QAbstractButton_clicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_clicked_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7clickedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractButton_clicked_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QAbstractButton_clicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_clicked_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7clickedEb(arg0, arg1, arg2)};
  }
}
// toggled(_Bool)
extern fn QAbstractButton_toggled_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QAbstractButton_toggled_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractButton_toggled_signal_connect for fn(i8) {
  fn connect(self, sigthis: QAbstractButton_toggled_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_toggled_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7toggledEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractButton_toggled_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QAbstractButton_toggled_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_toggled_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7toggledEb(arg0, arg1, arg2)};
  }
}
// pressed()
extern fn QAbstractButton_pressed_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractButton_pressed_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractButton_pressed_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractButton_pressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_pressed_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7pressedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractButton_pressed_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractButton_pressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_pressed_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton7pressedEv(arg0, arg1, arg2)};
  }
}
// released()
extern fn QAbstractButton_released_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractButton_released_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractButton_released_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractButton_released_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_released_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton8releasedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractButton_released_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractButton_released_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractButton_released_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractButton_SlotProxy_connect__ZN15QAbstractButton8releasedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

