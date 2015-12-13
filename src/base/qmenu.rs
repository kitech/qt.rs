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
use super::qaction::QAction;
use super::qwidget::QWidget;
use super::qpoint::QPoint;
use super::qobject::QObject;
use super::qkeysequence::QKeySequence;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QMenu::isTearOffEnabled();
  fn _ZNK5QMenu16isTearOffEnabledEv() -> i32;
  // proto: bool QMenu::toolTipsVisible();
  fn _ZNK5QMenu15toolTipsVisibleEv() -> i32;
  // proto: QAction * QMenu::menuAction();
  fn _ZNK5QMenu10menuActionEv() -> i32;
  // proto: QAction * QMenu::addAction(const QIcon & icon, const QString & text);
  fn _ZN5QMenu9addActionERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QMenu::triggered(QAction * action);
  fn _ZN5QMenu9triggeredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QMenu::setTearOffEnabled(bool );
  fn _ZN5QMenu17setTearOffEnabledEb(arg0: int8_t) -> i32;
  // proto: QAction * QMenu::addSection(const QString & text);
  fn _ZN5QMenu10addSectionERK7QString(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QMenu::metaObject();
  fn _ZNK5QMenu10metaObjectEv() -> i32;
  // proto: void QMenu::clear();
  fn _ZN5QMenu5clearEv() -> i32;
  // proto: QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
  fn _ZN5QMenu10insertMenuEP7QActionPS_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: QIcon QMenu::icon();
  fn _ZNK5QMenu4iconEv() -> i32;
  // proto: QAction * QMenu::insertSection(QAction * before, const QString & text);
  fn _ZN5QMenu13insertSectionEP7QActionRK7QString(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: QPlatformMenu * QMenu::platformMenu();
  fn _ZN5QMenu12platformMenuEv() -> i32;
  // proto: void QMenu::setNoReplayFor(QWidget * widget);
  fn _ZN5QMenu14setNoReplayForEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QMenu::setIcon(const QIcon & icon);
  fn _ZN5QMenu7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: QAction * QMenu::exec(const QPoint & pos, QAction * at);
  fn _ZN5QMenu4execERK6QPointP7QAction(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: bool QMenu::separatorsCollapsible();
  fn _ZNK5QMenu21separatorsCollapsibleEv() -> i32;
  // proto: QMenu * QMenu::addMenu(const QString & title);
  fn _ZN5QMenu7addMenuERK7QString(arg0: *const c_void) -> i32;
  // proto: void QMenu::hovered(QAction * action);
  fn _ZN5QMenu7hoveredEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QAction * QMenu::addSeparator();
  fn _ZN5QMenu12addSeparatorEv() -> i32;
  // proto: void QMenu::hideTearOffMenu();
  fn _ZN5QMenu15hideTearOffMenuEv() -> i32;
  // proto: QAction * QMenu::addAction(const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
  fn _ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence(arg0: *const c_void, arg1: *const c_void, arg2: *const c_char, arg3: *const c_void) -> i32;
  // proto: void QMenu::NewQMenu(QWidget * parent);
  fn _ZN5QMenuC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QMenu::setActiveAction(QAction * act);
  fn _ZN5QMenu15setActiveActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QMenu::setSeparatorsCollapsible(bool collapse);
  fn _ZN5QMenu24setSeparatorsCollapsibleEb(arg0: int8_t) -> i32;
  // proto: void QMenu::NewQMenu(const QMenu & );
  fn _ZN5QMenuC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QAction * QMenu::addAction(const QString & text);
  fn _ZN5QMenu9addActionERK7QString(arg0: *const c_void) -> i32;
  // proto: QAction * QMenu::activeAction();
  fn _ZNK5QMenu12activeActionEv() -> i32;
  // proto: bool QMenu::isEmpty();
  fn _ZNK5QMenu7isEmptyEv() -> i32;
  // proto: QAction * QMenu::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
  fn _ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_char, arg4: *const c_void) -> i32;
  // proto: QRect QMenu::actionGeometry(QAction * );
  fn _ZNK5QMenu14actionGeometryEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QMenu::NewQMenu(const QString & title, QWidget * parent);
  fn _ZN5QMenuC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: QAction * QMenu::insertSeparator(QAction * before);
  fn _ZN5QMenu15insertSeparatorEP7QAction(arg0: *mut c_void) -> i32;
  // proto: void QMenu::aboutToHide();
  fn _ZN5QMenu11aboutToHideEv() -> i32;
  // proto: QAction * QMenu::addSection(const QIcon & icon, const QString & text);
  fn _ZN5QMenu10addSectionERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: bool QMenu::isTearOffMenuVisible();
  fn _ZNK5QMenu20isTearOffMenuVisibleEv() -> i32;
  // proto: void QMenu::FreeQMenu();
  fn _ZN5QMenuD0Ev() -> i32;
  // proto: QString QMenu::title();
  fn _ZNK5QMenu5titleEv() -> i32;
  // proto: QAction * QMenu::defaultAction();
  fn _ZNK5QMenu13defaultActionEv() -> i32;
  // proto: QAction * QMenu::addMenu(QMenu * menu);
  fn _ZN5QMenu7addMenuEPS_(arg0: *mut c_void) -> i32;
  // proto: void QMenu::aboutToShow();
  fn _ZN5QMenu11aboutToShowEv() -> i32;
  // proto: QSize QMenu::sizeHint();
  fn _ZNK5QMenu8sizeHintEv() -> i32;
  // proto: void QMenu::setDefaultAction(QAction * );
  fn _ZN5QMenu16setDefaultActionEP7QAction(arg0: *mut c_void) -> i32;
  // proto: QAction * QMenu::actionAt(const QPoint & );
  fn _ZNK5QMenu8actionAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QAction * QMenu::insertSection(QAction * before, const QIcon & icon, const QString & text);
  fn _ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QMenu::popup(const QPoint & pos, QAction * at);
  fn _ZN5QMenu5popupERK6QPointP7QAction(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QMenu::setToolTipsVisible(bool visible);
  fn _ZN5QMenu18setToolTipsVisibleEb(arg0: int8_t) -> i32;
  // proto: void QMenu::setTitle(const QString & title);
  fn _ZN5QMenu8setTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: QMenu * QMenu::addMenu(const QIcon & icon, const QString & title);
  fn _ZN5QMenu7addMenuERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QAction * QMenu::exec();
  fn _ZN5QMenu4execEv() -> i32;
}

// body block begin
// class sizeof(QMenu)=1
pub struct QMenu {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMenu {
  pub fn isTearOffEnabled<T: QMenu_isTearOffEnabled>(&mut self, value: T) -> i32 {
    value.isTearOffEnabled(self);
    return 1;
  }
}

pub trait QMenu_isTearOffEnabled {
  fn isTearOffEnabled(self, this: &mut QMenu) -> i32;
}

// proto: bool QMenu::isTearOffEnabled();
impl<'a> /*trait*/ QMenu_isTearOffEnabled for () {
  fn isTearOffEnabled(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu16isTearOffEnabledEv()};
    unsafe {_ZNK5QMenu16isTearOffEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn toolTipsVisible<T: QMenu_toolTipsVisible>(&mut self, value: T) -> i32 {
    value.toolTipsVisible(self);
    return 1;
  }
}

pub trait QMenu_toolTipsVisible {
  fn toolTipsVisible(self, this: &mut QMenu) -> i32;
}

// proto: bool QMenu::toolTipsVisible();
impl<'a> /*trait*/ QMenu_toolTipsVisible for () {
  fn toolTipsVisible(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu15toolTipsVisibleEv()};
    unsafe {_ZNK5QMenu15toolTipsVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn menuAction<T: QMenu_menuAction>(&mut self, value: T) -> i32 {
    value.menuAction(self);
    return 1;
  }
}

pub trait QMenu_menuAction {
  fn menuAction(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::menuAction();
impl<'a> /*trait*/ QMenu_menuAction for () {
  fn menuAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu10menuActionEv()};
    unsafe {_ZNK5QMenu10menuActionEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addAction<T: QMenu_addAction>(&mut self, value: T) -> i32 {
    value.addAction(self);
    return 1;
  }
}

pub trait QMenu_addAction {
  fn addAction(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QIcon, &'a  QString) {
  fn addAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu9addActionERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn triggered<T: QMenu_triggered>(&mut self, value: T) -> i32 {
    value.triggered(self);
    return 1;
  }
}

pub trait QMenu_triggered {
  fn triggered(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::triggered(QAction * action);
impl<'a> /*trait*/ QMenu_triggered for (&'a mut QAction) {
  fn triggered(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu9triggeredEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setTearOffEnabled<T: QMenu_setTearOffEnabled>(&mut self, value: T) -> i32 {
    value.setTearOffEnabled(self);
    return 1;
  }
}

pub trait QMenu_setTearOffEnabled {
  fn setTearOffEnabled(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setTearOffEnabled(bool );
impl<'a> /*trait*/ QMenu_setTearOffEnabled for (i8) {
  fn setTearOffEnabled(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu17setTearOffEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QMenu17setTearOffEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addSection<T: QMenu_addSection>(&mut self, value: T) -> i32 {
    value.addSection(self);
    return 1;
  }
}

pub trait QMenu_addSection {
  fn addSection(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::addSection(const QString & text);
impl<'a> /*trait*/ QMenu_addSection for (&'a  QString) {
  fn addSection(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10addSectionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu10addSectionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn metaObject<T: QMenu_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMenu_metaObject {
  fn metaObject(self, this: &mut QMenu) -> i32;
}

// proto: const QMetaObject * QMenu::metaObject();
impl<'a> /*trait*/ QMenu_metaObject for () {
  fn metaObject(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu10metaObjectEv()};
    unsafe {_ZNK5QMenu10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn clear<T: QMenu_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QMenu_clear {
  fn clear(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::clear();
impl<'a> /*trait*/ QMenu_clear for () {
  fn clear(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu5clearEv()};
    unsafe {_ZN5QMenu5clearEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn insertMenu<T: QMenu_insertMenu>(&mut self, value: T) -> i32 {
    value.insertMenu(self);
    return 1;
  }
}

pub trait QMenu_insertMenu {
  fn insertMenu(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
impl<'a> /*trait*/ QMenu_insertMenu for (&'a mut QAction, &'a mut QMenu) {
  fn insertMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10insertMenuEP7QActionPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu10insertMenuEP7QActionPS_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn icon<T: QMenu_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QMenu_icon {
  fn icon(self, this: &mut QMenu) -> i32;
}

// proto: QIcon QMenu::icon();
impl<'a> /*trait*/ QMenu_icon for () {
  fn icon(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu4iconEv()};
    unsafe {_ZNK5QMenu4iconEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn insertSection<T: QMenu_insertSection>(&mut self, value: T) -> i32 {
    value.insertSection(self);
    return 1;
  }
}

pub trait QMenu_insertSection {
  fn insertSection(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::insertSection(QAction * before, const QString & text);
impl<'a> /*trait*/ QMenu_insertSection for (&'a mut QAction, &'a  QString) {
  fn insertSection(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu13insertSectionEP7QActionRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu13insertSectionEP7QActionRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn platformMenu<T: QMenu_platformMenu>(&mut self, value: T) -> i32 {
    value.platformMenu(self);
    return 1;
  }
}

pub trait QMenu_platformMenu {
  fn platformMenu(self, this: &mut QMenu) -> i32;
}

// proto: QPlatformMenu * QMenu::platformMenu();
impl<'a> /*trait*/ QMenu_platformMenu for () {
  fn platformMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu12platformMenuEv()};
    unsafe {_ZN5QMenu12platformMenuEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setNoReplayFor<T: QMenu_setNoReplayFor>(&mut self, value: T) -> i32 {
    value.setNoReplayFor(self);
    return 1;
  }
}

pub trait QMenu_setNoReplayFor {
  fn setNoReplayFor(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setNoReplayFor(QWidget * widget);
impl<'a> /*trait*/ QMenu_setNoReplayFor for (&'a mut QWidget) {
  fn setNoReplayFor(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu14setNoReplayForEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu14setNoReplayForEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setIcon<T: QMenu_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QMenu_setIcon {
  fn setIcon(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QMenu_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu7setIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn exec<T: QMenu_exec>(&mut self, value: T) -> i32 {
    value.exec(self);
    return 1;
  }
}

pub trait QMenu_exec {
  fn exec(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::exec(const QPoint & pos, QAction * at);
impl<'a> /*trait*/ QMenu_exec for (&'a  QPoint, &'a mut QAction) {
  fn exec(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu4execERK6QPointP7QAction()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu4execERK6QPointP7QAction(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn separatorsCollapsible<T: QMenu_separatorsCollapsible>(&mut self, value: T) -> i32 {
    value.separatorsCollapsible(self);
    return 1;
  }
}

pub trait QMenu_separatorsCollapsible {
  fn separatorsCollapsible(self, this: &mut QMenu) -> i32;
}

// proto: bool QMenu::separatorsCollapsible();
impl<'a> /*trait*/ QMenu_separatorsCollapsible for () {
  fn separatorsCollapsible(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu21separatorsCollapsibleEv()};
    unsafe {_ZNK5QMenu21separatorsCollapsibleEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addMenu<T: QMenu_addMenu>(&mut self, value: T) -> i32 {
    value.addMenu(self);
    return 1;
  }
}

pub trait QMenu_addMenu {
  fn addMenu(self, this: &mut QMenu) -> i32;
}

// proto: QMenu * QMenu::addMenu(const QString & title);
impl<'a> /*trait*/ QMenu_addMenu for (&'a  QString) {
  fn addMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu7addMenuERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn hovered<T: QMenu_hovered>(&mut self, value: T) -> i32 {
    value.hovered(self);
    return 1;
  }
}

pub trait QMenu_hovered {
  fn hovered(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::hovered(QAction * action);
impl<'a> /*trait*/ QMenu_hovered for (&'a mut QAction) {
  fn hovered(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7hoveredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu7hoveredEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addSeparator<T: QMenu_addSeparator>(&mut self, value: T) -> i32 {
    value.addSeparator(self);
    return 1;
  }
}

pub trait QMenu_addSeparator {
  fn addSeparator(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::addSeparator();
impl<'a> /*trait*/ QMenu_addSeparator for () {
  fn addSeparator(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu12addSeparatorEv()};
    unsafe {_ZN5QMenu12addSeparatorEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn hideTearOffMenu<T: QMenu_hideTearOffMenu>(&mut self, value: T) -> i32 {
    value.hideTearOffMenu(self);
    return 1;
  }
}

pub trait QMenu_hideTearOffMenu {
  fn hideTearOffMenu(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::hideTearOffMenu();
impl<'a> /*trait*/ QMenu_hideTearOffMenu for () {
  fn hideTearOffMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15hideTearOffMenuEv()};
    unsafe {_ZN5QMenu15hideTearOffMenuEv()};
    return 1;
  }
}

// proto: QAction * QMenu::addAction(const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QString, &'a  QObject, &'a  String, &'a  QKeySequence) {
  fn addAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn NewQMenu<T: QMenu_NewQMenu>(value: T) -> QMenu {
    let rsthis = value.NewQMenu();
    return rsthis;
    // return 1;
  }
}

pub trait QMenu_NewQMenu {
  fn NewQMenu(self) -> QMenu;
}

// proto: void QMenu::NewQMenu(QWidget * parent);
impl<'a> /*trait*/ QMenu_NewQMenu for (&'a mut QWidget) {
  fn NewQMenu(self) -> QMenu {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenuC1EP7QWidget(qthis, arg0)};
    let rsthis = QMenu{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setActiveAction<T: QMenu_setActiveAction>(&mut self, value: T) -> i32 {
    value.setActiveAction(self);
    return 1;
  }
}

pub trait QMenu_setActiveAction {
  fn setActiveAction(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setActiveAction(QAction * act);
impl<'a> /*trait*/ QMenu_setActiveAction for (&'a mut QAction) {
  fn setActiveAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15setActiveActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu15setActiveActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setSeparatorsCollapsible<T: QMenu_setSeparatorsCollapsible>(&mut self, value: T) -> i32 {
    value.setSeparatorsCollapsible(self);
    return 1;
  }
}

pub trait QMenu_setSeparatorsCollapsible {
  fn setSeparatorsCollapsible(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setSeparatorsCollapsible(bool collapse);
impl<'a> /*trait*/ QMenu_setSeparatorsCollapsible for (i8) {
  fn setSeparatorsCollapsible(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu24setSeparatorsCollapsibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QMenu24setSeparatorsCollapsibleEb(arg0)};
    return 1;
  }
}

// proto: void QMenu::NewQMenu(const QMenu & );
impl<'a> /*trait*/ QMenu_NewQMenu for (&'a  QMenu) {
  fn NewQMenu(self) -> QMenu {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QMenuC1ERKS_(qthis, arg0)};
    let rsthis = QMenu{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: QAction * QMenu::addAction(const QString & text);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QString) {
  fn addAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu9addActionERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn activeAction<T: QMenu_activeAction>(&mut self, value: T) -> i32 {
    value.activeAction(self);
    return 1;
  }
}

pub trait QMenu_activeAction {
  fn activeAction(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::activeAction();
impl<'a> /*trait*/ QMenu_activeAction for () {
  fn activeAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu12activeActionEv()};
    unsafe {_ZNK5QMenu12activeActionEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn isEmpty<T: QMenu_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QMenu_isEmpty {
  fn isEmpty(self, this: &mut QMenu) -> i32;
}

// proto: bool QMenu::isEmpty();
impl<'a> /*trait*/ QMenu_isEmpty for () {
  fn isEmpty(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu7isEmptyEv()};
    unsafe {_ZNK5QMenu7isEmptyEv()};
    return 1;
  }
}

// proto: QAction * QMenu::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QIcon, &'a  QString, &'a  QObject, &'a  String, &'a  QKeySequence) {
  fn addAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.as_ptr()  as *const c_char;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn actionGeometry<T: QMenu_actionGeometry>(&mut self, value: T) -> i32 {
    value.actionGeometry(self);
    return 1;
  }
}

pub trait QMenu_actionGeometry {
  fn actionGeometry(self, this: &mut QMenu) -> i32;
}

// proto: QRect QMenu::actionGeometry(QAction * );
impl<'a> /*trait*/ QMenu_actionGeometry for (&'a mut QAction) {
  fn actionGeometry(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK5QMenu14actionGeometryEP7QAction(arg0)};
    return 1;
  }
}

// proto: void QMenu::NewQMenu(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QMenu_NewQMenu for (&'a  QString, &'a mut QWidget) {
  fn NewQMenu(self) -> QMenu {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenuC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QMenu{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn insertSeparator<T: QMenu_insertSeparator>(&mut self, value: T) -> i32 {
    value.insertSeparator(self);
    return 1;
  }
}

pub trait QMenu_insertSeparator {
  fn insertSeparator(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::insertSeparator(QAction * before);
impl<'a> /*trait*/ QMenu_insertSeparator for (&'a mut QAction) {
  fn insertSeparator(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu15insertSeparatorEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn aboutToHide<T: QMenu_aboutToHide>(&mut self, value: T) -> i32 {
    value.aboutToHide(self);
    return 1;
  }
}

pub trait QMenu_aboutToHide {
  fn aboutToHide(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::aboutToHide();
impl<'a> /*trait*/ QMenu_aboutToHide for () {
  fn aboutToHide(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu11aboutToHideEv()};
    unsafe {_ZN5QMenu11aboutToHideEv()};
    return 1;
  }
}

// proto: QAction * QMenu::addSection(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_addSection for (&'a  QIcon, &'a  QString) {
  fn addSection(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10addSectionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu10addSectionERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn isTearOffMenuVisible<T: QMenu_isTearOffMenuVisible>(&mut self, value: T) -> i32 {
    value.isTearOffMenuVisible(self);
    return 1;
  }
}

pub trait QMenu_isTearOffMenuVisible {
  fn isTearOffMenuVisible(self, this: &mut QMenu) -> i32;
}

// proto: bool QMenu::isTearOffMenuVisible();
impl<'a> /*trait*/ QMenu_isTearOffMenuVisible for () {
  fn isTearOffMenuVisible(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu20isTearOffMenuVisibleEv()};
    unsafe {_ZNK5QMenu20isTearOffMenuVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn FreeQMenu<T: QMenu_FreeQMenu>(&mut self, value: T) -> i32 {
    value.FreeQMenu(self);
    return 1;
  }
}

pub trait QMenu_FreeQMenu {
  fn FreeQMenu(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::FreeQMenu();
impl<'a> /*trait*/ QMenu_FreeQMenu for () {
  fn FreeQMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuD0Ev()};
    unsafe {_ZN5QMenuD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn title<T: QMenu_title>(&mut self, value: T) -> i32 {
    value.title(self);
    return 1;
  }
}

pub trait QMenu_title {
  fn title(self, this: &mut QMenu) -> i32;
}

// proto: QString QMenu::title();
impl<'a> /*trait*/ QMenu_title for () {
  fn title(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu5titleEv()};
    unsafe {_ZNK5QMenu5titleEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn defaultAction<T: QMenu_defaultAction>(&mut self, value: T) -> i32 {
    value.defaultAction(self);
    return 1;
  }
}

pub trait QMenu_defaultAction {
  fn defaultAction(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::defaultAction();
impl<'a> /*trait*/ QMenu_defaultAction for () {
  fn defaultAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu13defaultActionEv()};
    unsafe {_ZNK5QMenu13defaultActionEv()};
    return 1;
  }
}

// proto: QAction * QMenu::addMenu(QMenu * menu);
impl<'a> /*trait*/ QMenu_addMenu for (&'a mut QMenu) {
  fn addMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu7addMenuEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn aboutToShow<T: QMenu_aboutToShow>(&mut self, value: T) -> i32 {
    value.aboutToShow(self);
    return 1;
  }
}

pub trait QMenu_aboutToShow {
  fn aboutToShow(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::aboutToShow();
impl<'a> /*trait*/ QMenu_aboutToShow for () {
  fn aboutToShow(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu11aboutToShowEv()};
    unsafe {_ZN5QMenu11aboutToShowEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn sizeHint<T: QMenu_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QMenu_sizeHint {
  fn sizeHint(self, this: &mut QMenu) -> i32;
}

// proto: QSize QMenu::sizeHint();
impl<'a> /*trait*/ QMenu_sizeHint for () {
  fn sizeHint(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu8sizeHintEv()};
    unsafe {_ZNK5QMenu8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setDefaultAction<T: QMenu_setDefaultAction>(&mut self, value: T) -> i32 {
    value.setDefaultAction(self);
    return 1;
  }
}

pub trait QMenu_setDefaultAction {
  fn setDefaultAction(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setDefaultAction(QAction * );
impl<'a> /*trait*/ QMenu_setDefaultAction for (&'a mut QAction) {
  fn setDefaultAction(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu16setDefaultActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu16setDefaultActionEP7QAction(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn actionAt<T: QMenu_actionAt>(&mut self, value: T) -> i32 {
    value.actionAt(self);
    return 1;
  }
}

pub trait QMenu_actionAt {
  fn actionAt(self, this: &mut QMenu) -> i32;
}

// proto: QAction * QMenu::actionAt(const QPoint & );
impl<'a> /*trait*/ QMenu_actionAt for (&'a  QPoint) {
  fn actionAt(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QMenu8actionAtERK6QPoint(arg0)};
    return 1;
  }
}

// proto: QAction * QMenu::insertSection(QAction * before, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_insertSection for (&'a mut QAction, &'a  QIcon, &'a  QString) {
  fn insertSection(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn popup<T: QMenu_popup>(&mut self, value: T) -> i32 {
    value.popup(self);
    return 1;
  }
}

pub trait QMenu_popup {
  fn popup(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::popup(const QPoint & pos, QAction * at);
impl<'a> /*trait*/ QMenu_popup for (&'a  QPoint, &'a mut QAction) {
  fn popup(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu5popupERK6QPointP7QAction()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenu5popupERK6QPointP7QAction(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setToolTipsVisible<T: QMenu_setToolTipsVisible>(&mut self, value: T) -> i32 {
    value.setToolTipsVisible(self);
    return 1;
  }
}

pub trait QMenu_setToolTipsVisible {
  fn setToolTipsVisible(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setToolTipsVisible(bool visible);
impl<'a> /*trait*/ QMenu_setToolTipsVisible for (i8) {
  fn setToolTipsVisible(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu18setToolTipsVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN5QMenu18setToolTipsVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setTitle<T: QMenu_setTitle>(&mut self, value: T) -> i32 {
    value.setTitle(self);
    return 1;
  }
}

pub trait QMenu_setTitle {
  fn setTitle(self, this: &mut QMenu) -> i32;
}

// proto: void QMenu::setTitle(const QString & title);
impl<'a> /*trait*/ QMenu_setTitle for (&'a  QString) {
  fn setTitle(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu8setTitleERK7QString(arg0)};
    return 1;
  }
}

// proto: QMenu * QMenu::addMenu(const QIcon & icon, const QString & title);
impl<'a> /*trait*/ QMenu_addMenu for (&'a  QIcon, &'a  QString) {
  fn addMenu(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QMenu7addMenuERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: QAction * QMenu::exec();
impl<'a> /*trait*/ QMenu_exec for () {
  fn exec(self, this: &mut QMenu) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu4execEv()};
    unsafe {_ZN5QMenu4execEv()};
    return 1;
  }
}

