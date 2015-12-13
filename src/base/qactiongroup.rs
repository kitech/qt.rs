// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qaction::QAction;
use super::qicon::QIcon;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QActionGroup::NewQActionGroup(QObject * parent);
  fn _ZN12QActionGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QList<QAction *> QActionGroup::actions();
  fn _ZNK12QActionGroup7actionsEv() -> i32;
  // proto: void QActionGroup::setDisabled(bool b);
  fn _ZN12QActionGroup11setDisabledEb(arg0: int8_t) -> i32;
  // proto: void QActionGroup::setEnabled(bool );
  fn _ZN12QActionGroup10setEnabledEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QActionGroup::metaObject();
  fn _ZNK12QActionGroup10metaObjectEv() -> i32;
  // proto: QAction * QActionGroup::addAction(QAction * a);
  fn _ZN12QActionGroup9addActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QActionGroup::FreeQActionGroup();
  fn _ZN12QActionGroupD0Ev() -> i32;
  // proto: QAction * QActionGroup::checkedAction();
  fn _ZNK12QActionGroup13checkedActionEv() -> i32;
  // proto: QAction * QActionGroup::addAction(const QIcon & icon, const QString & text);
  fn _ZN12QActionGroup9addActionERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QActionGroup::setVisible(bool );
  fn _ZN12QActionGroup10setVisibleEb(arg0: int8_t) -> i32;
  // proto: bool QActionGroup::isVisible();
  fn _ZNK12QActionGroup9isVisibleEv() -> i32;
  // proto: void QActionGroup::setExclusive(bool );
  fn _ZN12QActionGroup12setExclusiveEb(arg0: int8_t) -> i32;
  // proto: QAction * QActionGroup::addAction(const QString & text);
  fn _ZN12QActionGroup9addActionERK7QString(arg0: *const c_void) -> i32;
  // proto: void QActionGroup::triggered(QAction * );
  fn _ZN12QActionGroup9triggeredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: bool QActionGroup::isEnabled();
  fn _ZNK12QActionGroup9isEnabledEv() -> i32;
  // proto: bool QActionGroup::isExclusive();
  fn _ZNK12QActionGroup11isExclusiveEv() -> i32;
  // proto: void QActionGroup::removeAction(QAction * a);
  fn _ZN12QActionGroup12removeActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QActionGroup::NewQActionGroup(const QActionGroup & );
  fn _ZN12QActionGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QActionGroup::hovered(QAction * );
  fn _ZN12QActionGroup7hoveredEP7QAction(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QActionGroup)=1
pub struct QActionGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QActionGroup {
  pub fn NewQActionGroup<T: QActionGroup_NewQActionGroup>(value: T) -> QActionGroup {
    let rsthis = value.NewQActionGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QActionGroup_NewQActionGroup {
  fn NewQActionGroup(self) -> QActionGroup;
}

// proto: void QActionGroup::NewQActionGroup(QObject * parent);
impl<'a> /*trait*/ QActionGroup_NewQActionGroup for (&'a mut QObject) {
  fn NewQActionGroup(self) -> QActionGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QActionGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn actions<T: QActionGroup_actions>(&mut self, value: T) -> i32 {
    value.actions(self);
    return 1;
  }
}

pub trait QActionGroup_actions {
  fn actions(self, this: &mut QActionGroup) -> i32;
}

// proto: QList<QAction *> QActionGroup::actions();
impl<'a> /*trait*/ QActionGroup_actions for () {
  fn actions(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup7actionsEv()};
    unsafe {_ZNK12QActionGroup7actionsEv()};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn setDisabled<T: QActionGroup_setDisabled>(&mut self, value: T) -> i32 {
    value.setDisabled(self);
    return 1;
  }
}

pub trait QActionGroup_setDisabled {
  fn setDisabled(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::setDisabled(bool b);
impl<'a> /*trait*/ QActionGroup_setDisabled for (i8) {
  fn setDisabled(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup11setDisabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QActionGroup11setDisabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn setEnabled<T: QActionGroup_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QActionGroup_setEnabled {
  fn setEnabled(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::setEnabled(bool );
impl<'a> /*trait*/ QActionGroup_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QActionGroup10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn metaObject<T: QActionGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QActionGroup_metaObject {
  fn metaObject(self, this: &mut QActionGroup) -> i32;
}

// proto: const QMetaObject * QActionGroup::metaObject();
impl<'a> /*trait*/ QActionGroup_metaObject for () {
  fn metaObject(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup10metaObjectEv()};
    unsafe {_ZNK12QActionGroup10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn addAction<T: QActionGroup_addAction>(&mut self, value: T) -> i32 {
    value.addAction(self);
    return 1;
  }
}

pub trait QActionGroup_addAction {
  fn addAction(self, this: &mut QActionGroup) -> i32;
}

// proto: QAction * QActionGroup::addAction(QAction * a);
impl<'a> /*trait*/ QActionGroup_addAction for (&'a mut QAction) {
  fn addAction(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroup9addActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn FreeQActionGroup<T: QActionGroup_FreeQActionGroup>(&mut self, value: T) -> i32 {
    value.FreeQActionGroup(self);
    return 1;
  }
}

pub trait QActionGroup_FreeQActionGroup {
  fn FreeQActionGroup(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::FreeQActionGroup();
impl<'a> /*trait*/ QActionGroup_FreeQActionGroup for () {
  fn FreeQActionGroup(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupD0Ev()};
    unsafe {_ZN12QActionGroupD0Ev()};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn checkedAction<T: QActionGroup_checkedAction>(&mut self, value: T) -> i32 {
    value.checkedAction(self);
    return 1;
  }
}

pub trait QActionGroup_checkedAction {
  fn checkedAction(self, this: &mut QActionGroup) -> i32;
}

// proto: QAction * QActionGroup::checkedAction();
impl<'a> /*trait*/ QActionGroup_checkedAction for () {
  fn checkedAction(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup13checkedActionEv()};
    unsafe {_ZNK12QActionGroup13checkedActionEv()};
    return 1;
  }
}

// proto: QAction * QActionGroup::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QActionGroup_addAction for (&'a  QIcon, &'a  QString) {
  fn addAction(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QActionGroup9addActionERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn setVisible<T: QActionGroup_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QActionGroup_setVisible {
  fn setVisible(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::setVisible(bool );
impl<'a> /*trait*/ QActionGroup_setVisible for (i8) {
  fn setVisible(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QActionGroup10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn isVisible<T: QActionGroup_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QActionGroup_isVisible {
  fn isVisible(self, this: &mut QActionGroup) -> i32;
}

// proto: bool QActionGroup::isVisible();
impl<'a> /*trait*/ QActionGroup_isVisible for () {
  fn isVisible(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup9isVisibleEv()};
    unsafe {_ZNK12QActionGroup9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn setExclusive<T: QActionGroup_setExclusive>(&mut self, value: T) -> i32 {
    value.setExclusive(self);
    return 1;
  }
}

pub trait QActionGroup_setExclusive {
  fn setExclusive(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::setExclusive(bool );
impl<'a> /*trait*/ QActionGroup_setExclusive for (i8) {
  fn setExclusive(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup12setExclusiveEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QActionGroup12setExclusiveEb(arg0)};
    return 1;
  }
}

// proto: QAction * QActionGroup::addAction(const QString & text);
impl<'a> /*trait*/ QActionGroup_addAction for (&'a  QString) {
  fn addAction(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9addActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QActionGroup9addActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn triggered<T: QActionGroup_triggered>(&mut self, value: T) -> i32 {
    value.triggered(self);
    return 1;
  }
}

pub trait QActionGroup_triggered {
  fn triggered(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::triggered(QAction * );
impl<'a> /*trait*/ QActionGroup_triggered for (&'a mut QAction) {
  fn triggered(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroup9triggeredEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn isEnabled<T: QActionGroup_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QActionGroup_isEnabled {
  fn isEnabled(self, this: &mut QActionGroup) -> i32;
}

// proto: bool QActionGroup::isEnabled();
impl<'a> /*trait*/ QActionGroup_isEnabled for () {
  fn isEnabled(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup9isEnabledEv()};
    unsafe {_ZNK12QActionGroup9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn isExclusive<T: QActionGroup_isExclusive>(&mut self, value: T) -> i32 {
    value.isExclusive(self);
    return 1;
  }
}

pub trait QActionGroup_isExclusive {
  fn isExclusive(self, this: &mut QActionGroup) -> i32;
}

// proto: bool QActionGroup::isExclusive();
impl<'a> /*trait*/ QActionGroup_isExclusive for () {
  fn isExclusive(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QActionGroup11isExclusiveEv()};
    unsafe {_ZNK12QActionGroup11isExclusiveEv()};
    return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn removeAction<T: QActionGroup_removeAction>(&mut self, value: T) -> i32 {
    value.removeAction(self);
    return 1;
  }
}

pub trait QActionGroup_removeAction {
  fn removeAction(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::removeAction(QAction * a);
impl<'a> /*trait*/ QActionGroup_removeAction for (&'a mut QAction) {
  fn removeAction(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup12removeActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroup12removeActionEP7QAction(arg0)};
    return 1;
  }
}

// proto: void QActionGroup::NewQActionGroup(const QActionGroup & );
impl<'a> /*trait*/ QActionGroup_NewQActionGroup for (&'a  QActionGroup) {
  fn NewQActionGroup(self) -> QActionGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QActionGroupC1ERKS_(qthis, arg0)};
    let rsthis = QActionGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QActionGroup {
  pub fn hovered<T: QActionGroup_hovered>(&mut self, value: T) -> i32 {
    value.hovered(self);
    return 1;
  }
}

pub trait QActionGroup_hovered {
  fn hovered(self, this: &mut QActionGroup) -> i32;
}

// proto: void QActionGroup::hovered(QAction * );
impl<'a> /*trait*/ QActionGroup_hovered for (&'a mut QAction) {
  fn hovered(self, this: &mut QActionGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QActionGroup7hoveredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QActionGroup7hoveredEP7QAction(arg0)};
    return 1;
  }
}

