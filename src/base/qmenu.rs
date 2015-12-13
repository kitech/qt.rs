// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qaction::QAction;
use super::qicon::QIcon;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qpoint::QPoint;
use super::qobject::QObject;
use super::qkeysequence::QKeySequence;
use super::qrect::QRect;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QMenu::isTearOffEnabled();
  fn _ZNK5QMenu16isTearOffEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QMenu::toolTipsVisible();
  fn _ZNK5QMenu15toolTipsVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  QAction * QMenu::menuAction();
  fn _ZNK5QMenu10menuActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text);
  fn _ZN5QMenu9addActionERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::triggered(QAction * action);
  fn _ZN5QMenu9triggeredEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenu::setTearOffEnabled(bool );
  fn _ZN5QMenu17setTearOffEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QAction * QMenu::addSection(const QString & text);
  fn _ZN5QMenu10addSectionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QMenu::metaObject();
  fn _ZNK5QMenu10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QMenu::clear();
  fn _ZN5QMenu5clearEv(qthis: *mut c_void) ;
  // proto:  QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
  fn _ZN5QMenu10insertMenuEP7QActionPS_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QIcon QMenu::icon();
  fn _ZNK5QMenu4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::insertSection(QAction * before, const QString & text);
  fn _ZN5QMenu13insertSectionEP7QActionRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QPlatformMenu * QMenu::platformMenu();
  fn _ZN5QMenu12platformMenuEv(qthis: *mut c_void) ;
  // proto:  void QMenu::setNoReplayFor(QWidget * widget);
  fn _ZN5QMenu14setNoReplayForEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenu::setIcon(const QIcon & icon);
  fn _ZN5QMenu7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QMenu::exec(const QPoint & pos, QAction * at);
  fn _ZN5QMenu4execERK6QPointP7QAction(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QMenu::separatorsCollapsible();
  fn _ZNK5QMenu21separatorsCollapsibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  QMenu * QMenu::addMenu(const QString & title);
  fn _ZN5QMenu7addMenuERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::hovered(QAction * action);
  fn _ZN5QMenu7hoveredEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QMenu::addSeparator();
  fn _ZN5QMenu12addSeparatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::hideTearOffMenu();
  fn _ZN5QMenu15hideTearOffMenuEv(qthis: *mut c_void) ;
  // proto:  QAction * QMenu::addAction(const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
  fn _ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *const c_char, arg3: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::NewQMenu(QWidget * parent);
  fn _ZN5QMenuC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenu::setActiveAction(QAction * act);
  fn _ZN5QMenu15setActiveActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenu::setSeparatorsCollapsible(bool collapse);
  fn _ZN5QMenu24setSeparatorsCollapsibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMenu::NewQMenu(const QMenu & );
  fn _ZN5QMenuC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QMenu::addAction(const QString & text);
  fn _ZN5QMenu9addActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::activeAction();
  fn _ZNK5QMenu12activeActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMenu::isEmpty();
  fn _ZNK5QMenu7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
  fn _ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *const c_char, arg4: *mut c_void) -> *mut c_void;
  // proto:  QRect QMenu::actionGeometry(QAction * );
  fn _ZNK5QMenu14actionGeometryEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::NewQMenu(const QString & title, QWidget * parent);
  fn _ZN5QMenuC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QAction * QMenu::insertSeparator(QAction * before);
  fn _ZN5QMenu15insertSeparatorEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::aboutToHide();
  fn _ZN5QMenu11aboutToHideEv(qthis: *mut c_void) ;
  // proto:  QAction * QMenu::addSection(const QIcon & icon, const QString & text);
  fn _ZN5QMenu10addSectionERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QMenu::isTearOffMenuVisible();
  fn _ZNK5QMenu20isTearOffMenuVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMenu::FreeQMenu();
  fn _ZN5QMenuD0Ev(qthis: *mut c_void) ;
  // proto:  QString QMenu::title();
  fn _ZNK5QMenu5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::defaultAction();
  fn _ZNK5QMenu13defaultActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::aboutToShow();
  fn _ZN5QMenu11aboutToShowEv(qthis: *mut c_void) ;
  // proto:  QSize QMenu::sizeHint();
  fn _ZNK5QMenu8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::setDefaultAction(QAction * );
  fn _ZN5QMenu16setDefaultActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAction * QMenu::actionAt(const QPoint & );
  fn _ZNK5QMenu8actionAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::insertSection(QAction * before, const QIcon & icon, const QString & text);
  fn _ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QMenu::popup(const QPoint & pos, QAction * at);
  fn _ZN5QMenu5popupERK6QPointP7QAction(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QMenu::setToolTipsVisible(bool visible);
  fn _ZN5QMenu18setToolTipsVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMenu::setTitle(const QString & title);
  fn _ZN5QMenu8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMenu * QMenu::addMenu(const QIcon & icon, const QString & title);
  fn _ZN5QMenu7addMenuERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenu::exec();
  fn _ZN5QMenu4execEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QMenu)=1
pub struct QMenu {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMenu {
  pub fn isTearOffEnabled<T: QMenu_isTearOffEnabled>(&mut self, value: T) -> i8 {
    return value.isTearOffEnabled(self);
    // return 1;
  }
}

pub trait QMenu_isTearOffEnabled {
  fn isTearOffEnabled(self, rsthis: &mut QMenu) -> i8;
}

// proto:  bool QMenu::isTearOffEnabled();
impl<'a> /*trait*/ QMenu_isTearOffEnabled for () {
  fn isTearOffEnabled(self, rsthis: &mut QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu16isTearOffEnabledEv()};
    let mut ret = unsafe {_ZNK5QMenu16isTearOffEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn toolTipsVisible<T: QMenu_toolTipsVisible>(&mut self, value: T) -> i8 {
    return value.toolTipsVisible(self);
    // return 1;
  }
}

pub trait QMenu_toolTipsVisible {
  fn toolTipsVisible(self, rsthis: &mut QMenu) -> i8;
}

// proto:  bool QMenu::toolTipsVisible();
impl<'a> /*trait*/ QMenu_toolTipsVisible for () {
  fn toolTipsVisible(self, rsthis: &mut QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu15toolTipsVisibleEv()};
    let mut ret = unsafe {_ZNK5QMenu15toolTipsVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn menuAction<T: QMenu_menuAction>(&mut self, value: T) -> QAction {
    return value.menuAction(self);
    // return 1;
  }
}

pub trait QMenu_menuAction {
  fn menuAction(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::menuAction();
impl<'a> /*trait*/ QMenu_menuAction for () {
  fn menuAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu10menuActionEv()};
    let mut ret = unsafe {_ZNK5QMenu10menuActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addAction<T: QMenu_addAction>(&mut self, value: T) -> QAction {
    return value.addAction(self);
    // return 1;
  }
}

pub trait QMenu_addAction {
  fn addAction(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QIcon, &'a  QString) {
  fn addAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu9addActionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn triggered<T: QMenu_triggered>(&mut self, value: T)  {
     value.triggered(self);
    // return 1;
  }
}

pub trait QMenu_triggered {
  fn triggered(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::triggered(QAction * action);
impl<'a> /*trait*/ QMenu_triggered for (&'a mut QAction) {
  fn triggered(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu9triggeredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setTearOffEnabled<T: QMenu_setTearOffEnabled>(&mut self, value: T)  {
     value.setTearOffEnabled(self);
    // return 1;
  }
}

pub trait QMenu_setTearOffEnabled {
  fn setTearOffEnabled(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setTearOffEnabled(bool );
impl<'a> /*trait*/ QMenu_setTearOffEnabled for (i8) {
  fn setTearOffEnabled(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu17setTearOffEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QMenu17setTearOffEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addSection<T: QMenu_addSection>(&mut self, value: T) -> QAction {
    return value.addSection(self);
    // return 1;
  }
}

pub trait QMenu_addSection {
  fn addSection(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::addSection(const QString & text);
impl<'a> /*trait*/ QMenu_addSection for (&'a  QString) {
  fn addSection(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10addSectionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu10addSectionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn metaObject<T: QMenu_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMenu_metaObject {
  fn metaObject(self, rsthis: &mut QMenu) ;
}

// proto:  const QMetaObject * QMenu::metaObject();
impl<'a> /*trait*/ QMenu_metaObject for () {
  fn metaObject(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu10metaObjectEv()};
     unsafe {_ZNK5QMenu10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn clear<T: QMenu_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QMenu_clear {
  fn clear(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::clear();
impl<'a> /*trait*/ QMenu_clear for () {
  fn clear(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu5clearEv()};
     unsafe {_ZN5QMenu5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn insertMenu<T: QMenu_insertMenu>(&mut self, value: T) -> QAction {
    return value.insertMenu(self);
    // return 1;
  }
}

pub trait QMenu_insertMenu {
  fn insertMenu(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::insertMenu(QAction * before, QMenu * menu);
impl<'a> /*trait*/ QMenu_insertMenu for (&'a mut QAction, &'a mut QMenu) {
  fn insertMenu(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10insertMenuEP7QActionPS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu10insertMenuEP7QActionPS_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn icon<T: QMenu_icon>(&mut self, value: T) -> QIcon {
    return value.icon(self);
    // return 1;
  }
}

pub trait QMenu_icon {
  fn icon(self, rsthis: &mut QMenu) -> QIcon;
}

// proto:  QIcon QMenu::icon();
impl<'a> /*trait*/ QMenu_icon for () {
  fn icon(self, rsthis: &mut QMenu) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu4iconEv()};
    let mut ret = unsafe {_ZNK5QMenu4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn insertSection<T: QMenu_insertSection>(&mut self, value: T) -> QAction {
    return value.insertSection(self);
    // return 1;
  }
}

pub trait QMenu_insertSection {
  fn insertSection(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::insertSection(QAction * before, const QString & text);
impl<'a> /*trait*/ QMenu_insertSection for (&'a mut QAction, &'a  QString) {
  fn insertSection(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu13insertSectionEP7QActionRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu13insertSectionEP7QActionRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn platformMenu<T: QMenu_platformMenu>(&mut self, value: T)  {
     value.platformMenu(self);
    // return 1;
  }
}

pub trait QMenu_platformMenu {
  fn platformMenu(self, rsthis: &mut QMenu) ;
}

// proto:  QPlatformMenu * QMenu::platformMenu();
impl<'a> /*trait*/ QMenu_platformMenu for () {
  fn platformMenu(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu12platformMenuEv()};
     unsafe {_ZN5QMenu12platformMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setNoReplayFor<T: QMenu_setNoReplayFor>(&mut self, value: T)  {
     value.setNoReplayFor(self);
    // return 1;
  }
}

pub trait QMenu_setNoReplayFor {
  fn setNoReplayFor(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setNoReplayFor(QWidget * widget);
impl<'a> /*trait*/ QMenu_setNoReplayFor for (&'a mut QWidget) {
  fn setNoReplayFor(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu14setNoReplayForEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu14setNoReplayForEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setIcon<T: QMenu_setIcon>(&mut self, value: T)  {
     value.setIcon(self);
    // return 1;
  }
}

pub trait QMenu_setIcon {
  fn setIcon(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QMenu_setIcon for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn exec<T: QMenu_exec>(&mut self, value: T) -> QAction {
    return value.exec(self);
    // return 1;
  }
}

pub trait QMenu_exec {
  fn exec(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::exec(const QPoint & pos, QAction * at);
impl<'a> /*trait*/ QMenu_exec for (&'a  QPoint, &'a mut QAction) {
  fn exec(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu4execERK6QPointP7QAction()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu4execERK6QPointP7QAction(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn separatorsCollapsible<T: QMenu_separatorsCollapsible>(&mut self, value: T) -> i8 {
    return value.separatorsCollapsible(self);
    // return 1;
  }
}

pub trait QMenu_separatorsCollapsible {
  fn separatorsCollapsible(self, rsthis: &mut QMenu) -> i8;
}

// proto:  bool QMenu::separatorsCollapsible();
impl<'a> /*trait*/ QMenu_separatorsCollapsible for () {
  fn separatorsCollapsible(self, rsthis: &mut QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu21separatorsCollapsibleEv()};
    let mut ret = unsafe {_ZNK5QMenu21separatorsCollapsibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addMenu<T: QMenu_addMenu>(&mut self, value: T) -> QMenu {
    return value.addMenu(self);
    // return 1;
  }
}

pub trait QMenu_addMenu {
  fn addMenu(self, rsthis: &mut QMenu) -> QMenu;
}

// proto:  QMenu * QMenu::addMenu(const QString & title);
impl<'a> /*trait*/ QMenu_addMenu for (&'a  QString) {
  fn addMenu(self, rsthis: &mut QMenu) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu7addMenuERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn hovered<T: QMenu_hovered>(&mut self, value: T)  {
     value.hovered(self);
    // return 1;
  }
}

pub trait QMenu_hovered {
  fn hovered(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::hovered(QAction * action);
impl<'a> /*trait*/ QMenu_hovered for (&'a mut QAction) {
  fn hovered(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7hoveredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu7hoveredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn addSeparator<T: QMenu_addSeparator>(&mut self, value: T) -> QAction {
    return value.addSeparator(self);
    // return 1;
  }
}

pub trait QMenu_addSeparator {
  fn addSeparator(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::addSeparator();
impl<'a> /*trait*/ QMenu_addSeparator for () {
  fn addSeparator(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu12addSeparatorEv()};
    let mut ret = unsafe {_ZN5QMenu12addSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn hideTearOffMenu<T: QMenu_hideTearOffMenu>(&mut self, value: T)  {
     value.hideTearOffMenu(self);
    // return 1;
  }
}

pub trait QMenu_hideTearOffMenu {
  fn hideTearOffMenu(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::hideTearOffMenu();
impl<'a> /*trait*/ QMenu_hideTearOffMenu for () {
  fn hideTearOffMenu(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15hideTearOffMenuEv()};
     unsafe {_ZN5QMenu15hideTearOffMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QAction * QMenu::addAction(const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QString, &'a  QObject, &'a  String, &'a  QKeySequence) {
  fn addAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu9addActionERK7QStringPK7QObjectPKcRK12QKeySequence(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn setActiveAction<T: QMenu_setActiveAction>(&mut self, value: T)  {
     value.setActiveAction(self);
    // return 1;
  }
}

pub trait QMenu_setActiveAction {
  fn setActiveAction(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setActiveAction(QAction * act);
impl<'a> /*trait*/ QMenu_setActiveAction for (&'a mut QAction) {
  fn setActiveAction(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15setActiveActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu15setActiveActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setSeparatorsCollapsible<T: QMenu_setSeparatorsCollapsible>(&mut self, value: T)  {
     value.setSeparatorsCollapsible(self);
    // return 1;
  }
}

pub trait QMenu_setSeparatorsCollapsible {
  fn setSeparatorsCollapsible(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setSeparatorsCollapsible(bool collapse);
impl<'a> /*trait*/ QMenu_setSeparatorsCollapsible for (i8) {
  fn setSeparatorsCollapsible(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu24setSeparatorsCollapsibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QMenu24setSeparatorsCollapsibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QMenu::NewQMenu(const QMenu & );
impl<'a> /*trait*/ QMenu_NewQMenu for (&'a  QMenu) {
  fn NewQMenu(self) -> QMenu {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenuC1ERKS_(qthis, arg0)};
    let rsthis = QMenu{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QAction * QMenu::addAction(const QString & text);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QString) {
  fn addAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn activeAction<T: QMenu_activeAction>(&mut self, value: T) -> QAction {
    return value.activeAction(self);
    // return 1;
  }
}

pub trait QMenu_activeAction {
  fn activeAction(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::activeAction();
impl<'a> /*trait*/ QMenu_activeAction for () {
  fn activeAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu12activeActionEv()};
    let mut ret = unsafe {_ZNK5QMenu12activeActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn isEmpty<T: QMenu_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QMenu_isEmpty {
  fn isEmpty(self, rsthis: &mut QMenu) -> i8;
}

// proto:  bool QMenu::isEmpty();
impl<'a> /*trait*/ QMenu_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu7isEmptyEv()};
    let mut ret = unsafe {_ZNK5QMenu7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QAction * QMenu::addAction(const QIcon & icon, const QString & text, const QObject * receiver, const char * member, const QKeySequence & shortcut);
impl<'a> /*trait*/ QMenu_addAction for (&'a  QIcon, &'a  QString, &'a  QObject, &'a  String, &'a  QKeySequence) {
  fn addAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.as_ptr()  as *const c_char;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu9addActionERK5QIconRK7QStringPK7QObjectPKcRK12QKeySequence(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn actionGeometry<T: QMenu_actionGeometry>(&mut self, value: T) -> QRect {
    return value.actionGeometry(self);
    // return 1;
  }
}

pub trait QMenu_actionGeometry {
  fn actionGeometry(self, rsthis: &mut QMenu) -> QRect;
}

// proto:  QRect QMenu::actionGeometry(QAction * );
impl<'a> /*trait*/ QMenu_actionGeometry for (&'a mut QAction) {
  fn actionGeometry(self, rsthis: &mut QMenu) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QMenu14actionGeometryEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QMenu::NewQMenu(const QString & title, QWidget * parent);
impl<'a> /*trait*/ QMenu_NewQMenu for (&'a  QString, &'a mut QWidget) {
  fn NewQMenu(self) -> QMenu {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QMenuC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QMenu{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn insertSeparator<T: QMenu_insertSeparator>(&mut self, value: T) -> QAction {
    return value.insertSeparator(self);
    // return 1;
  }
}

pub trait QMenu_insertSeparator {
  fn insertSeparator(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::insertSeparator(QAction * before);
impl<'a> /*trait*/ QMenu_insertSeparator for (&'a mut QAction) {
  fn insertSeparator(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu15insertSeparatorEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn aboutToHide<T: QMenu_aboutToHide>(&mut self, value: T)  {
     value.aboutToHide(self);
    // return 1;
  }
}

pub trait QMenu_aboutToHide {
  fn aboutToHide(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::aboutToHide();
impl<'a> /*trait*/ QMenu_aboutToHide for () {
  fn aboutToHide(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu11aboutToHideEv()};
     unsafe {_ZN5QMenu11aboutToHideEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QAction * QMenu::addSection(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_addSection for (&'a  QIcon, &'a  QString) {
  fn addSection(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu10addSectionERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu10addSectionERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn isTearOffMenuVisible<T: QMenu_isTearOffMenuVisible>(&mut self, value: T) -> i8 {
    return value.isTearOffMenuVisible(self);
    // return 1;
  }
}

pub trait QMenu_isTearOffMenuVisible {
  fn isTearOffMenuVisible(self, rsthis: &mut QMenu) -> i8;
}

// proto:  bool QMenu::isTearOffMenuVisible();
impl<'a> /*trait*/ QMenu_isTearOffMenuVisible for () {
  fn isTearOffMenuVisible(self, rsthis: &mut QMenu) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu20isTearOffMenuVisibleEv()};
    let mut ret = unsafe {_ZNK5QMenu20isTearOffMenuVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn FreeQMenu<T: QMenu_FreeQMenu>(&mut self, value: T)  {
     value.FreeQMenu(self);
    // return 1;
  }
}

pub trait QMenu_FreeQMenu {
  fn FreeQMenu(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::FreeQMenu();
impl<'a> /*trait*/ QMenu_FreeQMenu for () {
  fn FreeQMenu(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenuD0Ev()};
     unsafe {_ZN5QMenuD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn title<T: QMenu_title>(&mut self, value: T) -> QString {
    return value.title(self);
    // return 1;
  }
}

pub trait QMenu_title {
  fn title(self, rsthis: &mut QMenu) -> QString;
}

// proto:  QString QMenu::title();
impl<'a> /*trait*/ QMenu_title for () {
  fn title(self, rsthis: &mut QMenu) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu5titleEv()};
    let mut ret = unsafe {_ZNK5QMenu5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn defaultAction<T: QMenu_defaultAction>(&mut self, value: T) -> QAction {
    return value.defaultAction(self);
    // return 1;
  }
}

pub trait QMenu_defaultAction {
  fn defaultAction(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::defaultAction();
impl<'a> /*trait*/ QMenu_defaultAction for () {
  fn defaultAction(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu13defaultActionEv()};
    let mut ret = unsafe {_ZNK5QMenu13defaultActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn aboutToShow<T: QMenu_aboutToShow>(&mut self, value: T)  {
     value.aboutToShow(self);
    // return 1;
  }
}

pub trait QMenu_aboutToShow {
  fn aboutToShow(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::aboutToShow();
impl<'a> /*trait*/ QMenu_aboutToShow for () {
  fn aboutToShow(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu11aboutToShowEv()};
     unsafe {_ZN5QMenu11aboutToShowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn sizeHint<T: QMenu_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QMenu_sizeHint {
  fn sizeHint(self, rsthis: &mut QMenu) -> QSize;
}

// proto:  QSize QMenu::sizeHint();
impl<'a> /*trait*/ QMenu_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QMenu) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu8sizeHintEv()};
    let mut ret = unsafe {_ZNK5QMenu8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setDefaultAction<T: QMenu_setDefaultAction>(&mut self, value: T)  {
     value.setDefaultAction(self);
    // return 1;
  }
}

pub trait QMenu_setDefaultAction {
  fn setDefaultAction(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setDefaultAction(QAction * );
impl<'a> /*trait*/ QMenu_setDefaultAction for (&'a mut QAction) {
  fn setDefaultAction(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu16setDefaultActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu16setDefaultActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn actionAt<T: QMenu_actionAt>(&mut self, value: T) -> QAction {
    return value.actionAt(self);
    // return 1;
  }
}

pub trait QMenu_actionAt {
  fn actionAt(self, rsthis: &mut QMenu) -> QAction;
}

// proto:  QAction * QMenu::actionAt(const QPoint & );
impl<'a> /*trait*/ QMenu_actionAt for (&'a  QPoint) {
  fn actionAt(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QMenu8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QMenu8actionAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAction * QMenu::insertSection(QAction * before, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QMenu_insertSection for (&'a mut QAction, &'a  QIcon, &'a  QString) {
  fn insertSection(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu13insertSectionEP7QActionRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn popup<T: QMenu_popup>(&mut self, value: T)  {
     value.popup(self);
    // return 1;
  }
}

pub trait QMenu_popup {
  fn popup(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::popup(const QPoint & pos, QAction * at);
impl<'a> /*trait*/ QMenu_popup for (&'a  QPoint, &'a mut QAction) {
  fn popup(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu5popupERK6QPointP7QAction()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu5popupERK6QPointP7QAction(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setToolTipsVisible<T: QMenu_setToolTipsVisible>(&mut self, value: T)  {
     value.setToolTipsVisible(self);
    // return 1;
  }
}

pub trait QMenu_setToolTipsVisible {
  fn setToolTipsVisible(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setToolTipsVisible(bool visible);
impl<'a> /*trait*/ QMenu_setToolTipsVisible for (i8) {
  fn setToolTipsVisible(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu18setToolTipsVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN5QMenu18setToolTipsVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenu {
  pub fn setTitle<T: QMenu_setTitle>(&mut self, value: T)  {
     value.setTitle(self);
    // return 1;
  }
}

pub trait QMenu_setTitle {
  fn setTitle(self, rsthis: &mut QMenu) ;
}

// proto:  void QMenu::setTitle(const QString & title);
impl<'a> /*trait*/ QMenu_setTitle for (&'a  QString) {
  fn setTitle(self, rsthis: &mut QMenu)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QMenu8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QMenu * QMenu::addMenu(const QIcon & icon, const QString & title);
impl<'a> /*trait*/ QMenu_addMenu for (&'a  QIcon, &'a  QString) {
  fn addMenu(self, rsthis: &mut QMenu) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu7addMenuERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QMenu7addMenuERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QAction * QMenu::exec();
impl<'a> /*trait*/ QMenu_exec for () {
  fn exec(self, rsthis: &mut QMenu) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QMenu4execEv()};
    let mut ret = unsafe {_ZN5QMenu4execEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

