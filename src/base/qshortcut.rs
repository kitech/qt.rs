// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qkeysequence::QKeySequence;
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QShortcut::setKey(const QKeySequence & key);
  fn _ZN9QShortcut6setKeyERK12QKeySequence(arg0: *const c_void) -> i32;
  // proto: void QShortcut::activated();
  fn _ZN9QShortcut9activatedEv() -> i32;
  // proto: const QMetaObject * QShortcut::metaObject();
  fn _ZNK9QShortcut10metaObjectEv() -> i32;
  // proto: QWidget * QShortcut::parentWidget();
  fn _ZNK9QShortcut12parentWidgetEv() -> i32;
  // proto: void QShortcut::setAutoRepeat(bool on);
  fn _ZN9QShortcut13setAutoRepeatEb(arg0: int8_t) -> i32;
  // proto: bool QShortcut::isEnabled();
  fn _ZNK9QShortcut9isEnabledEv() -> i32;
  // proto: QKeySequence QShortcut::key();
  fn _ZNK9QShortcut3keyEv() -> i32;
  // proto: void QShortcut::FreeQShortcut();
  fn _ZN9QShortcutD0Ev() -> i32;
  // proto: void QShortcut::setWhatsThis(const QString & text);
  fn _ZN9QShortcut12setWhatsThisERK7QString(arg0: *const c_void) -> i32;
  // proto: void QShortcut::setEnabled(bool enable);
  fn _ZN9QShortcut10setEnabledEb(arg0: int8_t) -> i32;
  // proto: int QShortcut::id();
  fn _ZNK9QShortcut2idEv() -> i32;
  // proto: QString QShortcut::whatsThis();
  fn _ZNK9QShortcut9whatsThisEv() -> i32;
  // proto: void QShortcut::NewQShortcut(QWidget * parent);
  fn _ZN9QShortcutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QShortcut::activatedAmbiguously();
  fn _ZN9QShortcut20activatedAmbiguouslyEv() -> i32;
  // proto: bool QShortcut::autoRepeat();
  fn _ZNK9QShortcut10autoRepeatEv() -> i32;
}

// body block begin
// class sizeof(QShortcut)=1
pub struct QShortcut {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShortcut {
  pub fn setKey<T: QShortcut_setKey>(&mut self, value: T) -> i32 {
    value.setKey(self);
    return 1;
  }
}

pub trait QShortcut_setKey {
  fn setKey(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::setKey(const QKeySequence & key);
impl<'a> /*trait*/ QShortcut_setKey for (&'a  QKeySequence) {
  fn setKey(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut6setKeyERK12QKeySequence()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QShortcut6setKeyERK12QKeySequence(arg0)};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn activated<T: QShortcut_activated>(&mut self, value: T) -> i32 {
    value.activated(self);
    return 1;
  }
}

pub trait QShortcut_activated {
  fn activated(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::activated();
impl<'a> /*trait*/ QShortcut_activated for () {
  fn activated(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut9activatedEv()};
    unsafe {_ZN9QShortcut9activatedEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn metaObject<T: QShortcut_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QShortcut_metaObject {
  fn metaObject(self, this: &mut QShortcut) -> i32;
}

// proto: const QMetaObject * QShortcut::metaObject();
impl<'a> /*trait*/ QShortcut_metaObject for () {
  fn metaObject(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10metaObjectEv()};
    unsafe {_ZNK9QShortcut10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn parentWidget<T: QShortcut_parentWidget>(&mut self, value: T) -> i32 {
    value.parentWidget(self);
    return 1;
  }
}

pub trait QShortcut_parentWidget {
  fn parentWidget(self, this: &mut QShortcut) -> i32;
}

// proto: QWidget * QShortcut::parentWidget();
impl<'a> /*trait*/ QShortcut_parentWidget for () {
  fn parentWidget(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut12parentWidgetEv()};
    unsafe {_ZNK9QShortcut12parentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setAutoRepeat<T: QShortcut_setAutoRepeat>(&mut self, value: T) -> i32 {
    value.setAutoRepeat(self);
    return 1;
  }
}

pub trait QShortcut_setAutoRepeat {
  fn setAutoRepeat(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::setAutoRepeat(bool on);
impl<'a> /*trait*/ QShortcut_setAutoRepeat for (i8) {
  fn setAutoRepeat(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut13setAutoRepeatEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QShortcut13setAutoRepeatEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn isEnabled<T: QShortcut_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QShortcut_isEnabled {
  fn isEnabled(self, this: &mut QShortcut) -> i32;
}

// proto: bool QShortcut::isEnabled();
impl<'a> /*trait*/ QShortcut_isEnabled for () {
  fn isEnabled(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9isEnabledEv()};
    unsafe {_ZNK9QShortcut9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn key<T: QShortcut_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QShortcut_key {
  fn key(self, this: &mut QShortcut) -> i32;
}

// proto: QKeySequence QShortcut::key();
impl<'a> /*trait*/ QShortcut_key for () {
  fn key(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut3keyEv()};
    unsafe {_ZNK9QShortcut3keyEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn FreeQShortcut<T: QShortcut_FreeQShortcut>(&mut self, value: T) -> i32 {
    value.FreeQShortcut(self);
    return 1;
  }
}

pub trait QShortcut_FreeQShortcut {
  fn FreeQShortcut(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::FreeQShortcut();
impl<'a> /*trait*/ QShortcut_FreeQShortcut for () {
  fn FreeQShortcut(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutD0Ev()};
    unsafe {_ZN9QShortcutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setWhatsThis<T: QShortcut_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QShortcut_setWhatsThis {
  fn setWhatsThis(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::setWhatsThis(const QString & text);
impl<'a> /*trait*/ QShortcut_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QShortcut12setWhatsThisERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setEnabled<T: QShortcut_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QShortcut_setEnabled {
  fn setEnabled(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::setEnabled(bool enable);
impl<'a> /*trait*/ QShortcut_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN9QShortcut10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn id<T: QShortcut_id>(&mut self, value: T) -> i32 {
    value.id(self);
    return 1;
  }
}

pub trait QShortcut_id {
  fn id(self, this: &mut QShortcut) -> i32;
}

// proto: int QShortcut::id();
impl<'a> /*trait*/ QShortcut_id for () {
  fn id(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut2idEv()};
    unsafe {_ZNK9QShortcut2idEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn whatsThis<T: QShortcut_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QShortcut_whatsThis {
  fn whatsThis(self, this: &mut QShortcut) -> i32;
}

// proto: QString QShortcut::whatsThis();
impl<'a> /*trait*/ QShortcut_whatsThis for () {
  fn whatsThis(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9whatsThisEv()};
    unsafe {_ZNK9QShortcut9whatsThisEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn NewQShortcut<T: QShortcut_NewQShortcut>(value: T) -> QShortcut {
    let rsthis = value.NewQShortcut();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcut_NewQShortcut {
  fn NewQShortcut(self) -> QShortcut;
}

// proto: void QShortcut::NewQShortcut(QWidget * parent);
impl<'a> /*trait*/ QShortcut_NewQShortcut for (&'a mut QWidget) {
  fn NewQShortcut(self) -> QShortcut {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QShortcutC1EP7QWidget(qthis, arg0)};
    let rsthis = QShortcut{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn activatedAmbiguously<T: QShortcut_activatedAmbiguously>(&mut self, value: T) -> i32 {
    value.activatedAmbiguously(self);
    return 1;
  }
}

pub trait QShortcut_activatedAmbiguously {
  fn activatedAmbiguously(self, this: &mut QShortcut) -> i32;
}

// proto: void QShortcut::activatedAmbiguously();
impl<'a> /*trait*/ QShortcut_activatedAmbiguously for () {
  fn activatedAmbiguously(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut20activatedAmbiguouslyEv()};
    unsafe {_ZN9QShortcut20activatedAmbiguouslyEv()};
    return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn autoRepeat<T: QShortcut_autoRepeat>(&mut self, value: T) -> i32 {
    value.autoRepeat(self);
    return 1;
  }
}

pub trait QShortcut_autoRepeat {
  fn autoRepeat(self, this: &mut QShortcut) -> i32;
}

// proto: bool QShortcut::autoRepeat();
impl<'a> /*trait*/ QShortcut_autoRepeat for () {
  fn autoRepeat(self, this: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10autoRepeatEv()};
    unsafe {_ZNK9QShortcut10autoRepeatEv()};
    return 1;
  }
}

