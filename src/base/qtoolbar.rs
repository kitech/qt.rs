// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qicon::QIcon;
use super::qstring::QString;
use super::qobject::QObject;
use super::qaction::QAction;
use super::qwidget::QWidget;
use super::qsize::QSize;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_char) -> i32;
  // proto: bool QToolBar::isFloatable();
  fn _ZNK8QToolBar11isFloatableEv() -> i32;
  // proto: QSize QToolBar::iconSize();
  fn _ZNK8QToolBar8iconSizeEv() -> i32;
  // proto: QRect QToolBar::actionGeometry(QAction * action);
  fn _ZNK8QToolBar14actionGeometryEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QWidget * QToolBar::widgetForAction(QAction * action);
  fn _ZNK8QToolBar15widgetForActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QToolBar::visibilityChanged(bool visible);
  fn _ZN8QToolBar17visibilityChangedEb(arg0: int8_t) -> i32;
  // proto: void QToolBar::clear();
  fn _ZN8QToolBar5clearEv() -> i32;
  // proto: void QToolBar::NewQToolBar(const QString & title, QWidget * parent);
  fn _ZN8QToolBarC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QToolBar::setMovable(bool movable);
  fn _ZN8QToolBar10setMovableEb(arg0: int8_t) -> i32;
  // proto: bool QToolBar::isMovable();
  fn _ZNK8QToolBar9isMovableEv() -> i32;
  // proto: void QToolBar::setIconSize(const QSize & iconSize);
  fn _ZN8QToolBar11setIconSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QAction * QToolBar::addSeparator();
  fn _ZN8QToolBar12addSeparatorEv() -> i32;
  // proto: void QToolBar::setFloatable(bool floatable);
  fn _ZN8QToolBar12setFloatableEb(arg0: int8_t) -> i32;
  // proto: QAction * QToolBar::addAction(const QString & text);
  fn _ZN8QToolBar9addActionERK7QString(arg0: *const c_void) -> i32;
  // proto: void QToolBar::topLevelChanged(bool topLevel);
  fn _ZN8QToolBar15topLevelChangedEb(arg0: int8_t) -> i32;
  // proto: void QToolBar::actionTriggered(QAction * action);
  fn _ZN8QToolBar15actionTriggeredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
  fn _ZN8QToolBar9addActionERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QToolBar::NewQToolBar(QWidget * parent);
  fn _ZN8QToolBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QAction * QToolBar::actionAt(const QPoint & p);
  fn _ZNK8QToolBar8actionAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QToolBar::movableChanged(bool movable);
  fn _ZN8QToolBar14movableChangedEb(arg0: int8_t) -> i32;
  // proto: QAction * QToolBar::actionAt(int x, int y);
  fn _ZNK8QToolBar8actionAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QToolBar::iconSizeChanged(const QSize & iconSize);
  fn _ZN8QToolBar15iconSizeChangedERK5QSize(arg0: *const c_void) -> i32;
  // proto: bool QToolBar::isFloating();
  fn _ZNK8QToolBar10isFloatingEv() -> i32;
  // proto: QAction * QToolBar::toggleViewAction();
  fn _ZNK8QToolBar16toggleViewActionEv() -> i32;
  // proto: void QToolBar::NewQToolBar(const QToolBar & );
  fn _ZN8QToolBarC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QToolBar::FreeQToolBar();
  fn _ZN8QToolBarD0Ev() -> i32;
  // proto: QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(arg0: *const c_void, arg1: *const c_void, arg2: *const c_char) -> i32;
  // proto: QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
  fn _ZN8QToolBar12insertWidgetEP7QActionP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: QAction * QToolBar::addWidget(QWidget * widget);
  fn _ZN8QToolBar9addWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QToolBar::metaObject();
  fn _ZNK8QToolBar10metaObjectEv() -> i32;
  // proto: QAction * QToolBar::insertSeparator(QAction * before);
  fn _ZN8QToolBar15insertSeparatorEP7QAction(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QToolBar)=1
pub struct QToolBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolBar {
  pub fn addAction<T: QToolBar_addAction>(&mut self, value: T) -> i32 {
    value.addAction(self);
    return 1;
  }
}

pub trait QToolBar_addAction {
  fn addAction(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QToolBar_addAction for (&'a  QIcon, &'a  QString, &'a  QObject, &'a  String) {
  fn addAction(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.as_ptr()  as *const c_char;
    unsafe {_ZN8QToolBar9addActionERK5QIconRK7QStringPK7QObjectPKc(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn isFloatable<T: QToolBar_isFloatable>(&mut self, value: T) -> i32 {
    value.isFloatable(self);
    return 1;
  }
}

pub trait QToolBar_isFloatable {
  fn isFloatable(self, this: &mut QToolBar) -> i32;
}

// proto: bool QToolBar::isFloatable();
impl<'a> /*trait*/ QToolBar_isFloatable for () {
  fn isFloatable(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar11isFloatableEv()};
    unsafe {_ZNK8QToolBar11isFloatableEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn iconSize<T: QToolBar_iconSize>(&mut self, value: T) -> i32 {
    value.iconSize(self);
    return 1;
  }
}

pub trait QToolBar_iconSize {
  fn iconSize(self, this: &mut QToolBar) -> i32;
}

// proto: QSize QToolBar::iconSize();
impl<'a> /*trait*/ QToolBar_iconSize for () {
  fn iconSize(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8iconSizeEv()};
    unsafe {_ZNK8QToolBar8iconSizeEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn actionGeometry<T: QToolBar_actionGeometry>(&mut self, value: T) -> i32 {
    value.actionGeometry(self);
    return 1;
  }
}

pub trait QToolBar_actionGeometry {
  fn actionGeometry(self, this: &mut QToolBar) -> i32;
}

// proto: QRect QToolBar::actionGeometry(QAction * action);
impl<'a> /*trait*/ QToolBar_actionGeometry for (&'a mut QAction) {
  fn actionGeometry(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK8QToolBar14actionGeometryEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn widgetForAction<T: QToolBar_widgetForAction>(&mut self, value: T) -> i32 {
    value.widgetForAction(self);
    return 1;
  }
}

pub trait QToolBar_widgetForAction {
  fn widgetForAction(self, this: &mut QToolBar) -> i32;
}

// proto: QWidget * QToolBar::widgetForAction(QAction * action);
impl<'a> /*trait*/ QToolBar_widgetForAction for (&'a mut QAction) {
  fn widgetForAction(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar15widgetForActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK8QToolBar15widgetForActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn visibilityChanged<T: QToolBar_visibilityChanged>(&mut self, value: T) -> i32 {
    value.visibilityChanged(self);
    return 1;
  }
}

pub trait QToolBar_visibilityChanged {
  fn visibilityChanged(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::visibilityChanged(bool visible);
impl<'a> /*trait*/ QToolBar_visibilityChanged for (i8) {
  fn visibilityChanged(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar17visibilityChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QToolBar17visibilityChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn clear<T: QToolBar_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QToolBar_clear {
  fn clear(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::clear();
impl<'a> /*trait*/ QToolBar_clear for () {
  fn clear(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar5clearEv()};
    unsafe {_ZN8QToolBar5clearEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn NewQToolBar<T: QToolBar_NewQToolBar>(value: T) -> QToolBar {
    let rsthis = value.NewQToolBar();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBar_NewQToolBar {
  fn NewQToolBar(self) -> QToolBar;
}

// proto: void QToolBar::NewQToolBar(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QToolBar_NewQToolBar for (&'a  QString, &'a mut QWidget) {
  fn NewQToolBar(self) -> QToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBarC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn setMovable<T: QToolBar_setMovable>(&mut self, value: T) -> i32 {
    value.setMovable(self);
    return 1;
  }
}

pub trait QToolBar_setMovable {
  fn setMovable(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::setMovable(bool movable);
impl<'a> /*trait*/ QToolBar_setMovable for (i8) {
  fn setMovable(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar10setMovableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QToolBar10setMovableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn isMovable<T: QToolBar_isMovable>(&mut self, value: T) -> i32 {
    value.isMovable(self);
    return 1;
  }
}

pub trait QToolBar_isMovable {
  fn isMovable(self, this: &mut QToolBar) -> i32;
}

// proto: bool QToolBar::isMovable();
impl<'a> /*trait*/ QToolBar_isMovable for () {
  fn isMovable(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar9isMovableEv()};
    unsafe {_ZNK8QToolBar9isMovableEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn setIconSize<T: QToolBar_setIconSize>(&mut self, value: T) -> i32 {
    value.setIconSize(self);
    return 1;
  }
}

pub trait QToolBar_setIconSize {
  fn setIconSize(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QToolBar_setIconSize for (&'a  QSize) {
  fn setIconSize(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBar11setIconSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn addSeparator<T: QToolBar_addSeparator>(&mut self, value: T) -> i32 {
    value.addSeparator(self);
    return 1;
  }
}

pub trait QToolBar_addSeparator {
  fn addSeparator(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::addSeparator();
impl<'a> /*trait*/ QToolBar_addSeparator for () {
  fn addSeparator(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12addSeparatorEv()};
    unsafe {_ZN8QToolBar12addSeparatorEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn setFloatable<T: QToolBar_setFloatable>(&mut self, value: T) -> i32 {
    value.setFloatable(self);
    return 1;
  }
}

pub trait QToolBar_setFloatable {
  fn setFloatable(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::setFloatable(bool floatable);
impl<'a> /*trait*/ QToolBar_setFloatable for (i8) {
  fn setFloatable(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12setFloatableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QToolBar12setFloatableEb(arg0)};
    return 1;
  }
}

// proto: QAction * QToolBar::addAction(const QString & text);
impl<'a> /*trait*/ QToolBar_addAction for (&'a  QString) {
  fn addAction(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBar9addActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn topLevelChanged<T: QToolBar_topLevelChanged>(&mut self, value: T) -> i32 {
    value.topLevelChanged(self);
    return 1;
  }
}

pub trait QToolBar_topLevelChanged {
  fn topLevelChanged(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::topLevelChanged(bool topLevel);
impl<'a> /*trait*/ QToolBar_topLevelChanged for (i8) {
  fn topLevelChanged(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15topLevelChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QToolBar15topLevelChangedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn actionTriggered<T: QToolBar_actionTriggered>(&mut self, value: T) -> i32 {
    value.actionTriggered(self);
    return 1;
  }
}

pub trait QToolBar_actionTriggered {
  fn actionTriggered(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::actionTriggered(QAction * action);
impl<'a> /*trait*/ QToolBar_actionTriggered for (&'a mut QAction) {
  fn actionTriggered(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15actionTriggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBar15actionTriggeredEP7QAction(arg0)};
    return 1;
  }
}

// proto: QAction * QToolBar::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBar_addAction for (&'a  QIcon, &'a  QString) {
  fn addAction(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBar9addActionERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: void QToolBar::NewQToolBar(QWidget * parent);
impl<'a> /*trait*/ QToolBar_NewQToolBar for (&'a mut QWidget) {
  fn NewQToolBar(self) -> QToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn actionAt<T: QToolBar_actionAt>(&mut self, value: T) -> i32 {
    value.actionAt(self);
    return 1;
  }
}

pub trait QToolBar_actionAt {
  fn actionAt(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::actionAt(const QPoint & p);
impl<'a> /*trait*/ QToolBar_actionAt for (&'a  QPoint) {
  fn actionAt(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK8QToolBar8actionAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn movableChanged<T: QToolBar_movableChanged>(&mut self, value: T) -> i32 {
    value.movableChanged(self);
    return 1;
  }
}

pub trait QToolBar_movableChanged {
  fn movableChanged(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::movableChanged(bool movable);
impl<'a> /*trait*/ QToolBar_movableChanged for (i8) {
  fn movableChanged(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar14movableChangedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QToolBar14movableChangedEb(arg0)};
    return 1;
  }
}

// proto: QAction * QToolBar::actionAt(int x, int y);
impl<'a> /*trait*/ QToolBar_actionAt for (i32, i32) {
  fn actionAt(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar8actionAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK8QToolBar8actionAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn iconSizeChanged<T: QToolBar_iconSizeChanged>(&mut self, value: T) -> i32 {
    value.iconSizeChanged(self);
    return 1;
  }
}

pub trait QToolBar_iconSizeChanged {
  fn iconSizeChanged(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QToolBar_iconSizeChanged for (&'a  QSize) {
  fn iconSizeChanged(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBar15iconSizeChangedERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn isFloating<T: QToolBar_isFloating>(&mut self, value: T) -> i32 {
    value.isFloating(self);
    return 1;
  }
}

pub trait QToolBar_isFloating {
  fn isFloating(self, this: &mut QToolBar) -> i32;
}

// proto: bool QToolBar::isFloating();
impl<'a> /*trait*/ QToolBar_isFloating for () {
  fn isFloating(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10isFloatingEv()};
    unsafe {_ZNK8QToolBar10isFloatingEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn toggleViewAction<T: QToolBar_toggleViewAction>(&mut self, value: T) -> i32 {
    value.toggleViewAction(self);
    return 1;
  }
}

pub trait QToolBar_toggleViewAction {
  fn toggleViewAction(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::toggleViewAction();
impl<'a> /*trait*/ QToolBar_toggleViewAction for () {
  fn toggleViewAction(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar16toggleViewActionEv()};
    unsafe {_ZNK8QToolBar16toggleViewActionEv()};
    return 1;
  }
}

// proto: void QToolBar::NewQToolBar(const QToolBar & );
impl<'a> /*trait*/ QToolBar_NewQToolBar for (&'a  QToolBar) {
  fn NewQToolBar(self) -> QToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolBarC1ERKS_(qthis, arg0)};
    let rsthis = QToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn FreeQToolBar<T: QToolBar_FreeQToolBar>(&mut self, value: T) -> i32 {
    value.FreeQToolBar(self);
    return 1;
  }
}

pub trait QToolBar_FreeQToolBar {
  fn FreeQToolBar(self, this: &mut QToolBar) -> i32;
}

// proto: void QToolBar::FreeQToolBar();
impl<'a> /*trait*/ QToolBar_FreeQToolBar for () {
  fn FreeQToolBar(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBarD0Ev()};
    unsafe {_ZN8QToolBarD0Ev()};
    return 1;
  }
}

// proto: QAction * QToolBar::addAction(const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QToolBar_addAction for (&'a  QString, &'a  QObject, &'a  String) {
  fn addAction(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN8QToolBar9addActionERK7QStringPK7QObjectPKc(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn insertWidget<T: QToolBar_insertWidget>(&mut self, value: T) -> i32 {
    value.insertWidget(self);
    return 1;
  }
}

pub trait QToolBar_insertWidget {
  fn insertWidget(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::insertWidget(QAction * before, QWidget * widget);
impl<'a> /*trait*/ QToolBar_insertWidget for (&'a mut QAction, &'a mut QWidget) {
  fn insertWidget(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar12insertWidgetEP7QActionP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBar12insertWidgetEP7QActionP7QWidget(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn addWidget<T: QToolBar_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QToolBar_addWidget {
  fn addWidget(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::addWidget(QWidget * widget);
impl<'a> /*trait*/ QToolBar_addWidget for (&'a mut QWidget) {
  fn addWidget(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBar9addWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn metaObject<T: QToolBar_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QToolBar_metaObject {
  fn metaObject(self, this: &mut QToolBar) -> i32;
}

// proto: const QMetaObject * QToolBar::metaObject();
impl<'a> /*trait*/ QToolBar_metaObject for () {
  fn metaObject(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBar10metaObjectEv()};
    unsafe {_ZNK8QToolBar10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QToolBar {
  pub fn insertSeparator<T: QToolBar_insertSeparator>(&mut self, value: T) -> i32 {
    value.insertSeparator(self);
    return 1;
  }
}

pub trait QToolBar_insertSeparator {
  fn insertSeparator(self, this: &mut QToolBar) -> i32;
}

// proto: QAction * QToolBar::insertSeparator(QAction * before);
impl<'a> /*trait*/ QToolBar_insertSeparator for (&'a mut QAction) {
  fn insertSeparator(self, this: &mut QToolBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBar15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBar15insertSeparatorEP7QAction(arg0)};
    return 1;
  }
}

