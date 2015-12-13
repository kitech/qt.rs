// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qkeysequence::QKeySequence;
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QShortcut::setKey(const QKeySequence & key);
  fn _ZN9QShortcut6setKeyERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QShortcut::activated();
  fn _ZN9QShortcut9activatedEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QShortcut::metaObject();
  fn _ZNK9QShortcut10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QWidget * QShortcut::parentWidget();
  fn _ZNK9QShortcut12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::setAutoRepeat(bool on);
  fn _ZN9QShortcut13setAutoRepeatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QShortcut::isEnabled();
  fn _ZNK9QShortcut9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  QKeySequence QShortcut::key();
  fn _ZNK9QShortcut3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::FreeQShortcut();
  fn _ZN9QShortcutD0Ev(qthis: *mut c_void) ;
  // proto:  void QShortcut::setWhatsThis(const QString & text);
  fn _ZN9QShortcut12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QShortcut::setEnabled(bool enable);
  fn _ZN9QShortcut10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QShortcut::id();
  fn _ZNK9QShortcut2idEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QShortcut::whatsThis();
  fn _ZNK9QShortcut9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::NewQShortcut(QWidget * parent);
  fn _ZN9QShortcutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QShortcut::activatedAmbiguously();
  fn _ZN9QShortcut20activatedAmbiguouslyEv(qthis: *mut c_void) ;
  // proto:  bool QShortcut::autoRepeat();
  fn _ZNK9QShortcut10autoRepeatEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QShortcut)=1
pub struct QShortcut {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShortcut {
  pub fn setKey<T: QShortcut_setKey>(&mut self, value: T)  {
     value.setKey(self);
    // return 1;
  }
}

pub trait QShortcut_setKey {
  fn setKey(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::setKey(const QKeySequence & key);
impl<'a> /*trait*/ QShortcut_setKey for (&'a  QKeySequence) {
  fn setKey(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut6setKeyERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QShortcut6setKeyERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn activated<T: QShortcut_activated>(&mut self, value: T)  {
     value.activated(self);
    // return 1;
  }
}

pub trait QShortcut_activated {
  fn activated(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::activated();
impl<'a> /*trait*/ QShortcut_activated for () {
  fn activated(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut9activatedEv()};
     unsafe {_ZN9QShortcut9activatedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn metaObject<T: QShortcut_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QShortcut_metaObject {
  fn metaObject(self, rsthis: &mut QShortcut) ;
}

// proto:  const QMetaObject * QShortcut::metaObject();
impl<'a> /*trait*/ QShortcut_metaObject for () {
  fn metaObject(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10metaObjectEv()};
     unsafe {_ZNK9QShortcut10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn parentWidget<T: QShortcut_parentWidget>(&mut self, value: T) -> QWidget {
    return value.parentWidget(self);
    // return 1;
  }
}

pub trait QShortcut_parentWidget {
  fn parentWidget(self, rsthis: &mut QShortcut) -> QWidget;
}

// proto:  QWidget * QShortcut::parentWidget();
impl<'a> /*trait*/ QShortcut_parentWidget for () {
  fn parentWidget(self, rsthis: &mut QShortcut) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut12parentWidgetEv()};
    let mut ret = unsafe {_ZNK9QShortcut12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setAutoRepeat<T: QShortcut_setAutoRepeat>(&mut self, value: T)  {
     value.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QShortcut_setAutoRepeat {
  fn setAutoRepeat(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::setAutoRepeat(bool on);
impl<'a> /*trait*/ QShortcut_setAutoRepeat for (i8) {
  fn setAutoRepeat(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut13setAutoRepeatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QShortcut13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn isEnabled<T: QShortcut_isEnabled>(&mut self, value: T) -> i8 {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QShortcut_isEnabled {
  fn isEnabled(self, rsthis: &mut QShortcut) -> i8;
}

// proto:  bool QShortcut::isEnabled();
impl<'a> /*trait*/ QShortcut_isEnabled for () {
  fn isEnabled(self, rsthis: &mut QShortcut) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9isEnabledEv()};
    let mut ret = unsafe {_ZNK9QShortcut9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn key<T: QShortcut_key>(&mut self, value: T) -> QKeySequence {
    return value.key(self);
    // return 1;
  }
}

pub trait QShortcut_key {
  fn key(self, rsthis: &mut QShortcut) -> QKeySequence;
}

// proto:  QKeySequence QShortcut::key();
impl<'a> /*trait*/ QShortcut_key for () {
  fn key(self, rsthis: &mut QShortcut) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut3keyEv()};
    let mut ret = unsafe {_ZNK9QShortcut3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn FreeQShortcut<T: QShortcut_FreeQShortcut>(&mut self, value: T)  {
     value.FreeQShortcut(self);
    // return 1;
  }
}

pub trait QShortcut_FreeQShortcut {
  fn FreeQShortcut(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::FreeQShortcut();
impl<'a> /*trait*/ QShortcut_FreeQShortcut for () {
  fn FreeQShortcut(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutD0Ev()};
     unsafe {_ZN9QShortcutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setWhatsThis<T: QShortcut_setWhatsThis>(&mut self, value: T)  {
     value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QShortcut_setWhatsThis {
  fn setWhatsThis(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::setWhatsThis(const QString & text);
impl<'a> /*trait*/ QShortcut_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QShortcut12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setEnabled<T: QShortcut_setEnabled>(&mut self, value: T)  {
     value.setEnabled(self);
    // return 1;
  }
}

pub trait QShortcut_setEnabled {
  fn setEnabled(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::setEnabled(bool enable);
impl<'a> /*trait*/ QShortcut_setEnabled for (i8) {
  fn setEnabled(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QShortcut10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn id<T: QShortcut_id>(&mut self, value: T) -> i32 {
    return value.id(self);
    // return 1;
  }
}

pub trait QShortcut_id {
  fn id(self, rsthis: &mut QShortcut) -> i32;
}

// proto:  int QShortcut::id();
impl<'a> /*trait*/ QShortcut_id for () {
  fn id(self, rsthis: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut2idEv()};
    let mut ret = unsafe {_ZNK9QShortcut2idEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn whatsThis<T: QShortcut_whatsThis>(&mut self, value: T) -> QString {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QShortcut_whatsThis {
  fn whatsThis(self, rsthis: &mut QShortcut) -> QString;
}

// proto:  QString QShortcut::whatsThis();
impl<'a> /*trait*/ QShortcut_whatsThis for () {
  fn whatsThis(self, rsthis: &mut QShortcut) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9whatsThisEv()};
    let mut ret = unsafe {_ZNK9QShortcut9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn activatedAmbiguously<T: QShortcut_activatedAmbiguously>(&mut self, value: T)  {
     value.activatedAmbiguously(self);
    // return 1;
  }
}

pub trait QShortcut_activatedAmbiguously {
  fn activatedAmbiguously(self, rsthis: &mut QShortcut) ;
}

// proto:  void QShortcut::activatedAmbiguously();
impl<'a> /*trait*/ QShortcut_activatedAmbiguously for () {
  fn activatedAmbiguously(self, rsthis: &mut QShortcut)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut20activatedAmbiguouslyEv()};
     unsafe {_ZN9QShortcut20activatedAmbiguouslyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn autoRepeat<T: QShortcut_autoRepeat>(&mut self, value: T) -> i8 {
    return value.autoRepeat(self);
    // return 1;
  }
}

pub trait QShortcut_autoRepeat {
  fn autoRepeat(self, rsthis: &mut QShortcut) -> i8;
}

// proto:  bool QShortcut::autoRepeat();
impl<'a> /*trait*/ QShortcut_autoRepeat for () {
  fn autoRepeat(self, rsthis: &mut QShortcut) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10autoRepeatEv()};
    let mut ret = unsafe {_ZNK9QShortcut10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

