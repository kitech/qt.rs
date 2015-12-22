// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QSize QAbstractButton::iconSize();
  fn _ZNK15QAbstractButton8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractButton::click();
  fn _ZN15QAbstractButton5clickEv(qthis: *mut c_void);
  // proto:  void QAbstractButton::~QAbstractButton();
  fn _ZN15QAbstractButtonD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractButton::setChecked(bool );
  fn _ZN15QAbstractButton10setCheckedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QKeySequence QAbstractButton::shortcut();
  fn _ZNK15QAbstractButton8shortcutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QButtonGroup * QAbstractButton::group();
  fn _ZNK15QAbstractButton5groupEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractButton::isCheckable();
  fn _ZNK15QAbstractButton11isCheckableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractButton::QAbstractButton(QWidget * parent);
  fn _ZN15QAbstractButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QAbstractButton::isDown();
  fn _ZNK15QAbstractButton6isDownEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractButton::toggled(bool checked);
  fn _ZN15QAbstractButton7toggledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAbstractButton::setAutoExclusive(bool );
  fn _ZN15QAbstractButton16setAutoExclusiveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QAbstractButton::metaObject();
  fn _ZNK15QAbstractButton10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QAbstractButton::isChecked();
  fn _ZNK15QAbstractButton9isCheckedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractButton::setAutoRepeatDelay(int );
  fn _ZN15QAbstractButton18setAutoRepeatDelayEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QAbstractButton::autoRepeatDelay();
  fn _ZNK15QAbstractButton15autoRepeatDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QAbstractButton::autoExclusive();
  fn _ZNK15QAbstractButton13autoExclusiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractButton::clicked(bool checked);
  fn _ZN15QAbstractButton7clickedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAbstractButton::released();
  fn _ZN15QAbstractButton8releasedEv(qthis: *mut c_void);
  // proto:  void QAbstractButton::toggle();
  fn _ZN15QAbstractButton6toggleEv(qthis: *mut c_void);
  // proto:  void QAbstractButton::setIcon(const QIcon & icon);
  fn _ZN15QAbstractButton7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractButton::setAutoRepeatInterval(int );
  fn _ZN15QAbstractButton21setAutoRepeatIntervalEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractButton::setAutoRepeat(bool );
  fn _ZN15QAbstractButton13setAutoRepeatEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAbstractButton::animateClick(int msec);
  fn _ZN15QAbstractButton12animateClickEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractButton::setDown(bool );
  fn _ZN15QAbstractButton7setDownEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QAbstractButton::text();
  fn _ZNK15QAbstractButton4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractButton::setShortcut(const QKeySequence & key);
  fn _ZN15QAbstractButton11setShortcutERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractButton::setCheckable(bool );
  fn _ZN15QAbstractButton12setCheckableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QIcon QAbstractButton::icon();
  fn _ZNK15QAbstractButton4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractButton::setText(const QString & text);
  fn _ZN15QAbstractButton7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QAbstractButton::autoRepeatInterval();
  fn _ZNK15QAbstractButton18autoRepeatIntervalEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QAbstractButton::autoRepeat();
  fn _ZNK15QAbstractButton10autoRepeatEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractButton::setIconSize(const QSize & size);
  fn _ZN15QAbstractButton11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractButton::pressed();
  fn _ZN15QAbstractButton7pressedEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractButton)=1
pub struct QAbstractButton {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractButton {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractButton {
    return QAbstractButton{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAbstractButton {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return &self.qbase;
  }
}
impl AsRef<QWidget> for QAbstractButton {
  fn as_ref(&self) -> &QWidget {
    return &self.qbase;
  }
}
  // proto:  QSize QAbstractButton::iconSize();
impl /*struct*/ QAbstractButton {
  pub fn iconSize<RetType, T: QAbstractButton_iconSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QAbstractButton_iconSize<RetType> {
  fn iconSize(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  QSize QAbstractButton::iconSize();
impl<'a> /*trait*/ QAbstractButton_iconSize<QSize> for () {
  fn iconSize(self , rsthis: &mut QAbstractButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton8iconSizeEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractButton::click();
impl /*struct*/ QAbstractButton {
  pub fn click<RetType, T: QAbstractButton_click<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.click(self);
    // return 1;
  }
}

pub trait QAbstractButton_click<RetType> {
  fn click(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::click();
impl<'a> /*trait*/ QAbstractButton_click<()> for () {
  fn click(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton5clickEv()};
     unsafe {_ZN15QAbstractButton5clickEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::~QAbstractButton();
impl /*struct*/ QAbstractButton {
  pub fn FreeQAbstractButton<RetType, T: QAbstractButton_FreeQAbstractButton<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractButton(self);
    // return 1;
  }
}

pub trait QAbstractButton_FreeQAbstractButton<RetType> {
  fn FreeQAbstractButton(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::~QAbstractButton();
impl<'a> /*trait*/ QAbstractButton_FreeQAbstractButton<()> for () {
  fn FreeQAbstractButton(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButtonD0Ev()};
     unsafe {_ZN15QAbstractButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setChecked(bool );
impl /*struct*/ QAbstractButton {
  pub fn setChecked<RetType, T: QAbstractButton_setChecked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setChecked(self);
    // return 1;
  }
}

pub trait QAbstractButton_setChecked<RetType> {
  fn setChecked(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setChecked(bool );
impl<'a> /*trait*/ QAbstractButton_setChecked<()> for (i8) {
  fn setChecked(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton10setCheckedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QKeySequence QAbstractButton::shortcut();
impl /*struct*/ QAbstractButton {
  pub fn shortcut<RetType, T: QAbstractButton_shortcut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.shortcut(self);
    // return 1;
  }
}

pub trait QAbstractButton_shortcut<RetType> {
  fn shortcut(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  QKeySequence QAbstractButton::shortcut();
impl<'a> /*trait*/ QAbstractButton_shortcut<QKeySequence> for () {
  fn shortcut(self , rsthis: &mut QAbstractButton) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton8shortcutEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton8shortcutEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QButtonGroup * QAbstractButton::group();
impl /*struct*/ QAbstractButton {
  pub fn group<RetType, T: QAbstractButton_group<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QAbstractButton_group<RetType> {
  fn group(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  QButtonGroup * QAbstractButton::group();
impl<'a> /*trait*/ QAbstractButton_group<QButtonGroup> for () {
  fn group(self , rsthis: &mut QAbstractButton) -> QButtonGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton5groupEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton5groupEv(rsthis.qclsinst)};
    let mut ret1 = QButtonGroup::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::isCheckable();
impl /*struct*/ QAbstractButton {
  pub fn isCheckable<RetType, T: QAbstractButton_isCheckable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCheckable(self);
    // return 1;
  }
}

pub trait QAbstractButton_isCheckable<RetType> {
  fn isCheckable(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::isCheckable();
impl<'a> /*trait*/ QAbstractButton_isCheckable<i8> for () {
  fn isCheckable(self , rsthis: &mut QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton11isCheckableEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::QAbstractButton(QWidget * parent);
impl /*struct*/ QAbstractButton {
  pub fn NewQAbstractButton<T: QAbstractButton_NewQAbstractButton>(value: T) -> QAbstractButton {
    let rsthis = value.NewQAbstractButton();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractButton_NewQAbstractButton {
  fn NewQAbstractButton(self) -> QAbstractButton;
}

  // proto:  void QAbstractButton::QAbstractButton(QWidget * parent);
impl<'a> /*trait*/ QAbstractButton_NewQAbstractButton for (QWidget) {
  fn NewQAbstractButton(self) -> QAbstractButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAbstractButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QAbstractButton{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::isDown();
impl /*struct*/ QAbstractButton {
  pub fn isDown<RetType, T: QAbstractButton_isDown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDown(self);
    // return 1;
  }
}

pub trait QAbstractButton_isDown<RetType> {
  fn isDown(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::isDown();
impl<'a> /*trait*/ QAbstractButton_isDown<i8> for () {
  fn isDown(self , rsthis: &mut QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton6isDownEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton6isDownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::toggled(bool checked);
impl /*struct*/ QAbstractButton {
  pub fn toggled<RetType, T: QAbstractButton_toggled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toggled(self);
    // return 1;
  }
}

pub trait QAbstractButton_toggled<RetType> {
  fn toggled(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::toggled(bool checked);
impl<'a> /*trait*/ QAbstractButton_toggled<()> for (i8) {
  fn toggled(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7toggledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton7toggledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoExclusive(bool );
impl /*struct*/ QAbstractButton {
  pub fn setAutoExclusive<RetType, T: QAbstractButton_setAutoExclusive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoExclusive(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoExclusive<RetType> {
  fn setAutoExclusive(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoExclusive(bool );
impl<'a> /*trait*/ QAbstractButton_setAutoExclusive<()> for (i8) {
  fn setAutoExclusive(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton16setAutoExclusiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton16setAutoExclusiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractButton::metaObject();
impl /*struct*/ QAbstractButton {
  pub fn metaObject<RetType, T: QAbstractButton_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractButton_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  const QMetaObject * QAbstractButton::metaObject();
impl<'a> /*trait*/ QAbstractButton_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton10metaObjectEv()};
     unsafe {_ZNK15QAbstractButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractButton::isChecked();
impl /*struct*/ QAbstractButton {
  pub fn isChecked<RetType, T: QAbstractButton_isChecked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isChecked(self);
    // return 1;
  }
}

pub trait QAbstractButton_isChecked<RetType> {
  fn isChecked(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::isChecked();
impl<'a> /*trait*/ QAbstractButton_isChecked<i8> for () {
  fn isChecked(self , rsthis: &mut QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton9isCheckedEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoRepeatDelay(int );
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeatDelay<RetType, T: QAbstractButton_setAutoRepeatDelay<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeatDelay(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoRepeatDelay<RetType> {
  fn setAutoRepeatDelay(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoRepeatDelay(int );
impl<'a> /*trait*/ QAbstractButton_setAutoRepeatDelay<()> for (i32) {
  fn setAutoRepeatDelay(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton18setAutoRepeatDelayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractButton18setAutoRepeatDelayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractButton::autoRepeatDelay();
impl /*struct*/ QAbstractButton {
  pub fn autoRepeatDelay<RetType, T: QAbstractButton_autoRepeatDelay<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoRepeatDelay(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoRepeatDelay<RetType> {
  fn autoRepeatDelay(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  int QAbstractButton::autoRepeatDelay();
impl<'a> /*trait*/ QAbstractButton_autoRepeatDelay<i32> for () {
  fn autoRepeatDelay(self , rsthis: &mut QAbstractButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton15autoRepeatDelayEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton15autoRepeatDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::autoExclusive();
impl /*struct*/ QAbstractButton {
  pub fn autoExclusive<RetType, T: QAbstractButton_autoExclusive<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoExclusive(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoExclusive<RetType> {
  fn autoExclusive(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::autoExclusive();
impl<'a> /*trait*/ QAbstractButton_autoExclusive<i8> for () {
  fn autoExclusive(self , rsthis: &mut QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton13autoExclusiveEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton13autoExclusiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::clicked(bool checked);
impl /*struct*/ QAbstractButton {
  pub fn clicked<RetType, T: QAbstractButton_clicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clicked(self);
    // return 1;
  }
}

pub trait QAbstractButton_clicked<RetType> {
  fn clicked(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::clicked(bool checked);
impl<'a> /*trait*/ QAbstractButton_clicked<()> for (i8) {
  fn clicked(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7clickedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton7clickedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::released();
impl /*struct*/ QAbstractButton {
  pub fn released<RetType, T: QAbstractButton_released<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.released(self);
    // return 1;
  }
}

pub trait QAbstractButton_released<RetType> {
  fn released(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::released();
impl<'a> /*trait*/ QAbstractButton_released<()> for () {
  fn released(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton8releasedEv()};
     unsafe {_ZN15QAbstractButton8releasedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::toggle();
impl /*struct*/ QAbstractButton {
  pub fn toggle<RetType, T: QAbstractButton_toggle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toggle(self);
    // return 1;
  }
}

pub trait QAbstractButton_toggle<RetType> {
  fn toggle(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::toggle();
impl<'a> /*trait*/ QAbstractButton_toggle<()> for () {
  fn toggle(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton6toggleEv()};
     unsafe {_ZN15QAbstractButton6toggleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setIcon(const QIcon & icon);
impl /*struct*/ QAbstractButton {
  pub fn setIcon<RetType, T: QAbstractButton_setIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QAbstractButton_setIcon<RetType> {
  fn setIcon(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QAbstractButton_setIcon<()> for (QIcon) {
  fn setIcon(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoRepeatInterval(int );
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeatInterval<RetType, T: QAbstractButton_setAutoRepeatInterval<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeatInterval(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoRepeatInterval<RetType> {
  fn setAutoRepeatInterval(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoRepeatInterval(int );
impl<'a> /*trait*/ QAbstractButton_setAutoRepeatInterval<()> for (i32) {
  fn setAutoRepeatInterval(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton21setAutoRepeatIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractButton21setAutoRepeatIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setAutoRepeat(bool );
impl /*struct*/ QAbstractButton {
  pub fn setAutoRepeat<RetType, T: QAbstractButton_setAutoRepeat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QAbstractButton_setAutoRepeat<RetType> {
  fn setAutoRepeat(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setAutoRepeat(bool );
impl<'a> /*trait*/ QAbstractButton_setAutoRepeat<()> for (i8) {
  fn setAutoRepeat(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton13setAutoRepeatEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::animateClick(int msec);
impl /*struct*/ QAbstractButton {
  pub fn animateClick<RetType, T: QAbstractButton_animateClick<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.animateClick(self);
    // return 1;
  }
}

pub trait QAbstractButton_animateClick<RetType> {
  fn animateClick(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::animateClick(int msec);
impl<'a> /*trait*/ QAbstractButton_animateClick<()> for (i32) {
  fn animateClick(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton12animateClickEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractButton12animateClickEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setDown(bool );
impl /*struct*/ QAbstractButton {
  pub fn setDown<RetType, T: QAbstractButton_setDown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDown(self);
    // return 1;
  }
}

pub trait QAbstractButton_setDown<RetType> {
  fn setDown(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setDown(bool );
impl<'a> /*trait*/ QAbstractButton_setDown<()> for (i8) {
  fn setDown(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7setDownEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton7setDownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QAbstractButton::text();
impl /*struct*/ QAbstractButton {
  pub fn text<RetType, T: QAbstractButton_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QAbstractButton_text<RetType> {
  fn text(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  QString QAbstractButton::text();
impl<'a> /*trait*/ QAbstractButton_text<QString> for () {
  fn text(self , rsthis: &mut QAbstractButton) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton4textEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setShortcut(const QKeySequence & key);
impl /*struct*/ QAbstractButton {
  pub fn setShortcut<RetType, T: QAbstractButton_setShortcut<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setShortcut(self);
    // return 1;
  }
}

pub trait QAbstractButton_setShortcut<RetType> {
  fn setShortcut(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setShortcut(const QKeySequence & key);
impl<'a> /*trait*/ QAbstractButton_setShortcut<()> for (QKeySequence) {
  fn setShortcut(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton11setShortcutERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton11setShortcutERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::setCheckable(bool );
impl /*struct*/ QAbstractButton {
  pub fn setCheckable<RetType, T: QAbstractButton_setCheckable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCheckable(self);
    // return 1;
  }
}

pub trait QAbstractButton_setCheckable<RetType> {
  fn setCheckable(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setCheckable(bool );
impl<'a> /*trait*/ QAbstractButton_setCheckable<()> for (i8) {
  fn setCheckable(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton12setCheckableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractButton12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QAbstractButton::icon();
impl /*struct*/ QAbstractButton {
  pub fn icon<RetType, T: QAbstractButton_icon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QAbstractButton_icon<RetType> {
  fn icon(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  QIcon QAbstractButton::icon();
impl<'a> /*trait*/ QAbstractButton_icon<QIcon> for () {
  fn icon(self , rsthis: &mut QAbstractButton) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton4iconEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setText(const QString & text);
impl /*struct*/ QAbstractButton {
  pub fn setText<RetType, T: QAbstractButton_setText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QAbstractButton_setText<RetType> {
  fn setText(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setText(const QString & text);
impl<'a> /*trait*/ QAbstractButton_setText<()> for (QString) {
  fn setText(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractButton::autoRepeatInterval();
impl /*struct*/ QAbstractButton {
  pub fn autoRepeatInterval<RetType, T: QAbstractButton_autoRepeatInterval<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoRepeatInterval(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoRepeatInterval<RetType> {
  fn autoRepeatInterval(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  int QAbstractButton::autoRepeatInterval();
impl<'a> /*trait*/ QAbstractButton_autoRepeatInterval<i32> for () {
  fn autoRepeatInterval(self , rsthis: &mut QAbstractButton) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton18autoRepeatIntervalEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton18autoRepeatIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QAbstractButton::autoRepeat();
impl /*struct*/ QAbstractButton {
  pub fn autoRepeat<RetType, T: QAbstractButton_autoRepeat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat(self);
    // return 1;
  }
}

pub trait QAbstractButton_autoRepeat<RetType> {
  fn autoRepeat(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  bool QAbstractButton::autoRepeat();
impl<'a> /*trait*/ QAbstractButton_autoRepeat<i8> for () {
  fn autoRepeat(self , rsthis: &mut QAbstractButton) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractButton10autoRepeatEv()};
    let mut ret = unsafe {_ZNK15QAbstractButton10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractButton::setIconSize(const QSize & size);
impl /*struct*/ QAbstractButton {
  pub fn setIconSize<RetType, T: QAbstractButton_setIconSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QAbstractButton_setIconSize<RetType> {
  fn setIconSize(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::setIconSize(const QSize & size);
impl<'a> /*trait*/ QAbstractButton_setIconSize<()> for (QSize) {
  fn setIconSize(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QAbstractButton11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractButton::pressed();
impl /*struct*/ QAbstractButton {
  pub fn pressed<RetType, T: QAbstractButton_pressed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pressed(self);
    // return 1;
  }
}

pub trait QAbstractButton_pressed<RetType> {
  fn pressed(self , rsthis: &mut QAbstractButton) -> RetType;
}

  // proto:  void QAbstractButton::pressed();
impl<'a> /*trait*/ QAbstractButton_pressed<()> for () {
  fn pressed(self , rsthis: &mut QAbstractButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractButton7pressedEv()};
     unsafe {_ZN15QAbstractButton7pressedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

