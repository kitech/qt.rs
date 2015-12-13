// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qmenu::QMenu;
use super::qaction::QAction;
use super::qpoint::QPoint;
use super::qwidget::QWidget;
use super::qicon::QIcon;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QAction * QMenuBar::addAction(const QString & text);
  fn _ZN8QMenuBar9addActionERK7QString(arg0: *const c_void) -> i32;
  // proto: QPlatformMenuBar * QMenuBar::platformMenuBar();
  fn _ZN8QMenuBar15platformMenuBarEv() -> i32;
  // proto: void QMenuBar::setNativeMenuBar(bool nativeMenuBar);
  fn _ZN8QMenuBar16setNativeMenuBarEb(arg0: int8_t) -> i32;
  // proto: void QMenuBar::FreeQMenuBar();
  fn _ZN8QMenuBarD0Ev() -> i32;
  // proto: QAction * QMenuBar::addMenu(QMenu * menu);
  fn _ZN8QMenuBar7addMenuEP5QMenu(arg0: *mut c_void) -> i32;
  // proto: void QMenuBar::triggered(QAction * action);
  fn _ZN8QMenuBar9triggeredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QSize QMenuBar::sizeHint();
  fn _ZNK8QMenuBar8sizeHintEv() -> i32;
  // proto: QAction * QMenuBar::actionAt(const QPoint & );
  fn _ZNK8QMenuBar8actionAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QMenuBar::hovered(QAction * action);
  fn _ZN8QMenuBar7hoveredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QMenuBar::metaObject();
  fn _ZNK8QMenuBar10metaObjectEv() -> i32;
  // proto: bool QMenuBar::isNativeMenuBar();
  fn _ZNK8QMenuBar15isNativeMenuBarEv() -> i32;
  // proto: QAction * QMenuBar::insertSeparator(QAction * before);
  fn _ZN8QMenuBar15insertSeparatorEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QAction * QMenuBar::addSeparator();
  fn _ZN8QMenuBar12addSeparatorEv() -> i32;
  // proto: QSize QMenuBar::minimumSizeHint();
  fn _ZNK8QMenuBar15minimumSizeHintEv() -> i32;
  // proto: bool QMenuBar::isDefaultUp();
  fn _ZNK8QMenuBar11isDefaultUpEv() -> i32;
  // proto: void QMenuBar::NewQMenuBar(const QMenuBar & );
  fn _ZN8QMenuBarC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QMenuBar::NewQMenuBar(QWidget * parent);
  fn _ZN8QMenuBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QMenuBar::setActiveAction(QAction * action);
  fn _ZN8QMenuBar15setActiveActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QMenuBar::clear();
  fn _ZN8QMenuBar5clearEv() -> i32;
  // proto: QAction * QMenuBar::activeAction();
  fn _ZNK8QMenuBar12activeActionEv() -> i32;
  // proto: QMenu * QMenuBar::addMenu(const QIcon & icon, const QString & title);
  fn _ZN8QMenuBar7addMenuERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QMenu * QMenuBar::addMenu(const QString & title);
  fn _ZN8QMenuBar7addMenuERK7QString(arg0: *const c_void) -> i32;
  // proto: QRect QMenuBar::actionGeometry(QAction * );
  fn _ZNK8QMenuBar14actionGeometryEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QAction * QMenuBar::insertMenu(QAction * before, QMenu * menu);
  fn _ZN8QMenuBar10insertMenuEP7QActionP5QMenu(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QMenuBar::setDefaultUp(bool );
  fn _ZN8QMenuBar12setDefaultUpEb(arg0: int8_t) -> i32;
  // proto: void QMenuBar::setVisible(bool visible);
  fn _ZN8QMenuBar10setVisibleEb(arg0: int8_t) -> i32;
  // proto: QAction * QMenuBar::addAction(const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc(arg0: *const c_void, arg1: *const c_void, arg2: *const c_char) -> i32;
  // proto: int QMenuBar::heightForWidth(int );
  fn _ZNK8QMenuBar14heightForWidthEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QMenuBar)=1
pub struct QMenuBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMenuBar {
  pub fn addAction<T: QMenuBar_addAction>(&mut self, value: T) -> i32 {
    value.addAction(self);
    return 1;
  }
}

pub trait QMenuBar_addAction {
  fn addAction(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::addAction(const QString & text);
impl<'a> /*trait*/ QMenuBar_addAction for (&'a  QString) {
  fn addAction(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar9addActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QMenuBar9addActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn platformMenuBar<T: QMenuBar_platformMenuBar>(&mut self, value: T) -> i32 {
    value.platformMenuBar(self);
    return 1;
  }
}

pub trait QMenuBar_platformMenuBar {
  fn platformMenuBar(self, this: &mut QMenuBar) -> i32;
}

// proto: QPlatformMenuBar * QMenuBar::platformMenuBar();
impl<'a> /*trait*/ QMenuBar_platformMenuBar for () {
  fn platformMenuBar(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar15platformMenuBarEv()};
    unsafe {_ZN8QMenuBar15platformMenuBarEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setNativeMenuBar<T: QMenuBar_setNativeMenuBar>(&mut self, value: T) -> i32 {
    value.setNativeMenuBar(self);
    return 1;
  }
}

pub trait QMenuBar_setNativeMenuBar {
  fn setNativeMenuBar(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::setNativeMenuBar(bool nativeMenuBar);
impl<'a> /*trait*/ QMenuBar_setNativeMenuBar for (i8) {
  fn setNativeMenuBar(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar16setNativeMenuBarEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QMenuBar16setNativeMenuBarEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn FreeQMenuBar<T: QMenuBar_FreeQMenuBar>(&mut self, value: T) -> i32 {
    value.FreeQMenuBar(self);
    return 1;
  }
}

pub trait QMenuBar_FreeQMenuBar {
  fn FreeQMenuBar(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::FreeQMenuBar();
impl<'a> /*trait*/ QMenuBar_FreeQMenuBar for () {
  fn FreeQMenuBar(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBarD0Ev()};
    unsafe {_ZN8QMenuBarD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn addMenu<T: QMenuBar_addMenu>(&mut self, value: T) -> i32 {
    value.addMenu(self);
    return 1;
  }
}

pub trait QMenuBar_addMenu {
  fn addMenu(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::addMenu(QMenu * menu);
impl<'a> /*trait*/ QMenuBar_addMenu for (&'a mut QMenu) {
  fn addMenu(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7addMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBar7addMenuEP5QMenu(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn triggered<T: QMenuBar_triggered>(&mut self, value: T) -> i32 {
    value.triggered(self);
    return 1;
  }
}

pub trait QMenuBar_triggered {
  fn triggered(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::triggered(QAction * action);
impl<'a> /*trait*/ QMenuBar_triggered for (&'a mut QAction) {
  fn triggered(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBar9triggeredEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn sizeHint<T: QMenuBar_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QMenuBar_sizeHint {
  fn sizeHint(self, this: &mut QMenuBar) -> i32;
}

// proto: QSize QMenuBar::sizeHint();
impl<'a> /*trait*/ QMenuBar_sizeHint for () {
  fn sizeHint(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar8sizeHintEv()};
    unsafe {_ZNK8QMenuBar8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn actionAt<T: QMenuBar_actionAt>(&mut self, value: T) -> i32 {
    value.actionAt(self);
    return 1;
  }
}

pub trait QMenuBar_actionAt {
  fn actionAt(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::actionAt(const QPoint & );
impl<'a> /*trait*/ QMenuBar_actionAt for (&'a  QPoint) {
  fn actionAt(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QMenuBar8actionAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn hovered<T: QMenuBar_hovered>(&mut self, value: T) -> i32 {
    value.hovered(self);
    return 1;
  }
}

pub trait QMenuBar_hovered {
  fn hovered(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::hovered(QAction * action);
impl<'a> /*trait*/ QMenuBar_hovered for (&'a mut QAction) {
  fn hovered(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7hoveredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBar7hoveredEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn metaObject<T: QMenuBar_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMenuBar_metaObject {
  fn metaObject(self, this: &mut QMenuBar) -> i32;
}

// proto: const QMetaObject * QMenuBar::metaObject();
impl<'a> /*trait*/ QMenuBar_metaObject for () {
  fn metaObject(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar10metaObjectEv()};
    unsafe {_ZNK8QMenuBar10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn isNativeMenuBar<T: QMenuBar_isNativeMenuBar>(&mut self, value: T) -> i32 {
    value.isNativeMenuBar(self);
    return 1;
  }
}

pub trait QMenuBar_isNativeMenuBar {
  fn isNativeMenuBar(self, this: &mut QMenuBar) -> i32;
}

// proto: bool QMenuBar::isNativeMenuBar();
impl<'a> /*trait*/ QMenuBar_isNativeMenuBar for () {
  fn isNativeMenuBar(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar15isNativeMenuBarEv()};
    unsafe {_ZNK8QMenuBar15isNativeMenuBarEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn insertSeparator<T: QMenuBar_insertSeparator>(&mut self, value: T) -> i32 {
    value.insertSeparator(self);
    return 1;
  }
}

pub trait QMenuBar_insertSeparator {
  fn insertSeparator(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::insertSeparator(QAction * before);
impl<'a> /*trait*/ QMenuBar_insertSeparator for (&'a mut QAction) {
  fn insertSeparator(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBar15insertSeparatorEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn addSeparator<T: QMenuBar_addSeparator>(&mut self, value: T) -> i32 {
    value.addSeparator(self);
    return 1;
  }
}

pub trait QMenuBar_addSeparator {
  fn addSeparator(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::addSeparator();
impl<'a> /*trait*/ QMenuBar_addSeparator for () {
  fn addSeparator(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar12addSeparatorEv()};
    unsafe {_ZN8QMenuBar12addSeparatorEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn minimumSizeHint<T: QMenuBar_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QMenuBar_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QMenuBar) -> i32;
}

// proto: QSize QMenuBar::minimumSizeHint();
impl<'a> /*trait*/ QMenuBar_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar15minimumSizeHintEv()};
    unsafe {_ZNK8QMenuBar15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn isDefaultUp<T: QMenuBar_isDefaultUp>(&mut self, value: T) -> i32 {
    value.isDefaultUp(self);
    return 1;
  }
}

pub trait QMenuBar_isDefaultUp {
  fn isDefaultUp(self, this: &mut QMenuBar) -> i32;
}

// proto: bool QMenuBar::isDefaultUp();
impl<'a> /*trait*/ QMenuBar_isDefaultUp for () {
  fn isDefaultUp(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar11isDefaultUpEv()};
    unsafe {_ZNK8QMenuBar11isDefaultUpEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn NewQMenuBar<T: QMenuBar_NewQMenuBar>(value: T) -> QMenuBar {
    let rsthis = value.NewQMenuBar();
    return rsthis;
    // return 1;
  }
}

pub trait QMenuBar_NewQMenuBar {
  fn NewQMenuBar(self) -> QMenuBar;
}

// proto: void QMenuBar::NewQMenuBar(const QMenuBar & );
impl<'a> /*trait*/ QMenuBar_NewQMenuBar for (&'a  QMenuBar) {
  fn NewQMenuBar(self) -> QMenuBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBarC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QMenuBarC1ERKS_(qthis, arg0)};
    let rsthis = QMenuBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMenuBar::NewQMenuBar(QWidget * parent);
impl<'a> /*trait*/ QMenuBar_NewQMenuBar for (&'a mut QWidget) {
  fn NewQMenuBar(self) -> QMenuBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QMenuBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setActiveAction<T: QMenuBar_setActiveAction>(&mut self, value: T) -> i32 {
    value.setActiveAction(self);
    return 1;
  }
}

pub trait QMenuBar_setActiveAction {
  fn setActiveAction(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::setActiveAction(QAction * action);
impl<'a> /*trait*/ QMenuBar_setActiveAction for (&'a mut QAction) {
  fn setActiveAction(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar15setActiveActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBar15setActiveActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn clear<T: QMenuBar_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QMenuBar_clear {
  fn clear(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::clear();
impl<'a> /*trait*/ QMenuBar_clear for () {
  fn clear(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar5clearEv()};
    unsafe {_ZN8QMenuBar5clearEv()};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn activeAction<T: QMenuBar_activeAction>(&mut self, value: T) -> i32 {
    value.activeAction(self);
    return 1;
  }
}

pub trait QMenuBar_activeAction {
  fn activeAction(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::activeAction();
impl<'a> /*trait*/ QMenuBar_activeAction for () {
  fn activeAction(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar12activeActionEv()};
    unsafe {_ZNK8QMenuBar12activeActionEv()};
    return 1;
  }
}

// proto: QMenu * QMenuBar::addMenu(const QIcon & icon, const QString & title);
impl<'a> /*trait*/ QMenuBar_addMenu for (&'a  QIcon, &'a  QString) {
  fn addMenu(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7addMenuERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QMenuBar7addMenuERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: QMenu * QMenuBar::addMenu(const QString & title);
impl<'a> /*trait*/ QMenuBar_addMenu for (&'a  QString) {
  fn addMenu(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7addMenuERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QMenuBar7addMenuERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn actionGeometry<T: QMenuBar_actionGeometry>(&mut self, value: T) -> i32 {
    value.actionGeometry(self);
    return 1;
  }
}

pub trait QMenuBar_actionGeometry {
  fn actionGeometry(self, this: &mut QMenuBar) -> i32;
}

// proto: QRect QMenuBar::actionGeometry(QAction * );
impl<'a> /*trait*/ QMenuBar_actionGeometry for (&'a mut QAction) {
  fn actionGeometry(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK8QMenuBar14actionGeometryEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn insertMenu<T: QMenuBar_insertMenu>(&mut self, value: T) -> i32 {
    value.insertMenu(self);
    return 1;
  }
}

pub trait QMenuBar_insertMenu {
  fn insertMenu(self, this: &mut QMenuBar) -> i32;
}

// proto: QAction * QMenuBar::insertMenu(QAction * before, QMenu * menu);
impl<'a> /*trait*/ QMenuBar_insertMenu for (&'a mut QAction, &'a mut QMenu) {
  fn insertMenu(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar10insertMenuEP7QActionP5QMenu()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBar10insertMenuEP7QActionP5QMenu(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setDefaultUp<T: QMenuBar_setDefaultUp>(&mut self, value: T) -> i32 {
    value.setDefaultUp(self);
    return 1;
  }
}

pub trait QMenuBar_setDefaultUp {
  fn setDefaultUp(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::setDefaultUp(bool );
impl<'a> /*trait*/ QMenuBar_setDefaultUp for (i8) {
  fn setDefaultUp(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar12setDefaultUpEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QMenuBar12setDefaultUpEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setVisible<T: QMenuBar_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QMenuBar_setVisible {
  fn setVisible(self, this: &mut QMenuBar) -> i32;
}

// proto: void QMenuBar::setVisible(bool visible);
impl<'a> /*trait*/ QMenuBar_setVisible for (i8) {
  fn setVisible(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QMenuBar10setVisibleEb(arg0)};
    return 1;
  }
}

// proto: QAction * QMenuBar::addAction(const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QMenuBar_addAction for (&'a  QString, &'a  QObject, &'a  String) {
  fn addAction(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn heightForWidth<T: QMenuBar_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QMenuBar_heightForWidth {
  fn heightForWidth(self, this: &mut QMenuBar) -> i32;
}

// proto: int QMenuBar::heightForWidth(int );
impl<'a> /*trait*/ QMenuBar_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QMenuBar14heightForWidthEi(arg0)};
    return 1;
  }
}

