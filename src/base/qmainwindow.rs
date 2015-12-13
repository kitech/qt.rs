// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qdockwidget::QDockWidget;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qtoolbar::QToolBar;
use super::qstatusbar::QStatusBar;
use super::qpoint::QPoint;
use super::qwidget::QWidget;
use super::qmenubar::QMenuBar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QStatusBar * QMainWindow::statusBar();
  fn _ZNK11QMainWindow9statusBarEv() -> i32;
  // proto: void QMainWindow::setAnimated(bool enabled);
  fn _ZN11QMainWindow11setAnimatedEb(arg0: int8_t) -> i32;
  // proto: void QMainWindow::setDockNestingEnabled(bool enabled);
  fn _ZN11QMainWindow21setDockNestingEnabledEb(arg0: int8_t) -> i32;
  // proto: void QMainWindow::iconSizeChanged(const QSize & iconSize);
  fn _ZN11QMainWindow15iconSizeChangedERK5QSize(arg0: *const c_void) -> i32;
  // proto: bool QMainWindow::unifiedTitleAndToolBarOnMac();
  fn _ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv() -> i32;
  // proto: QWidget * QMainWindow::menuWidget();
  fn _ZNK11QMainWindow10menuWidgetEv() -> i32;
  // proto: void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
  fn _ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QMainWindow::setDocumentMode(bool enabled);
  fn _ZN11QMainWindow15setDocumentModeEb(arg0: int8_t) -> i32;
  // proto: QWidget * QMainWindow::centralWidget();
  fn _ZNK11QMainWindow13centralWidgetEv() -> i32;
  // proto: void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
  fn _ZN11QMainWindow16removeDockWidgetEP11QDockWidget(arg0: *mut c_void) -> i32;
  // proto: void QMainWindow::NewQMainWindow(const QMainWindow & );
  fn _ZN11QMainWindowC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QMainWindow::isAnimated();
  fn _ZNK11QMainWindow10isAnimatedEv() -> i32;
  // proto: QToolBar * QMainWindow::addToolBar(const QString & title);
  fn _ZN11QMainWindow10addToolBarERK7QString(arg0: *const c_void) -> i32;
  // proto: void QMainWindow::setIconSize(const QSize & iconSize);
  fn _ZN11QMainWindow11setIconSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QByteArray QMainWindow::saveState(int version);
  fn _ZNK11QMainWindow9saveStateEi(arg0: c_int) -> i32;
  // proto: bool QMainWindow::restoreState(const QByteArray & state, int version);
  fn _ZN11QMainWindow12restoreStateERK10QByteArrayi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
  fn _ZN11QMainWindow13insertToolBarEP8QToolBarS1_(arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: QMenu * QMainWindow::createPopupMenu();
  fn _ZN11QMainWindow15createPopupMenuEv() -> i32;
  // proto: void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
  fn _ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(arg0: int8_t) -> i32;
  // proto: void QMainWindow::addToolBar(QToolBar * toolbar);
  fn _ZN11QMainWindow10addToolBarEP8QToolBar(arg0: *mut c_void) -> i32;
  // proto: void QMainWindow::removeToolBarBreak(QToolBar * before);
  fn _ZN11QMainWindow18removeToolBarBreakEP8QToolBar(arg0: *mut c_void) -> i32;
  // proto: bool QMainWindow::toolBarBreak(QToolBar * toolbar);
  fn _ZNK11QMainWindow12toolBarBreakEP8QToolBar(arg0: *mut c_void) -> i32;
  // proto: bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
  fn _ZN11QMainWindow17restoreDockWidgetEP11QDockWidget(arg0: *mut c_void) -> i32;
  // proto: QMenuBar * QMainWindow::menuBar();
  fn _ZNK11QMainWindow7menuBarEv() -> i32;
  // proto: void QMainWindow::setStatusBar(QStatusBar * statusbar);
  fn _ZN11QMainWindow12setStatusBarEP10QStatusBar(arg0: *mut c_void) -> i32;
  // proto: void QMainWindow::FreeQMainWindow();
  fn _ZN11QMainWindowD0Ev() -> i32;
  // proto: bool QMainWindow::isSeparator(const QPoint & pos);
  fn _ZNK11QMainWindow11isSeparatorERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QSize QMainWindow::iconSize();
  fn _ZNK11QMainWindow8iconSizeEv() -> i32;
  // proto: const QMetaObject * QMainWindow::metaObject();
  fn _ZNK11QMainWindow10metaObjectEv() -> i32;
  // proto: void QMainWindow::insertToolBarBreak(QToolBar * before);
  fn _ZN11QMainWindow18insertToolBarBreakEP8QToolBar(arg0: *mut c_void) -> i32;
  // proto: QWidget * QMainWindow::takeCentralWidget();
  fn _ZN11QMainWindow17takeCentralWidgetEv() -> i32;
  // proto: bool QMainWindow::isDockNestingEnabled();
  fn _ZNK11QMainWindow20isDockNestingEnabledEv() -> i32;
  // proto: bool QMainWindow::documentMode();
  fn _ZNK11QMainWindow12documentModeEv() -> i32;
  // proto: void QMainWindow::setMenuWidget(QWidget * menubar);
  fn _ZN11QMainWindow13setMenuWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QMainWindow::removeToolBar(QToolBar * toolbar);
  fn _ZN11QMainWindow13removeToolBarEP8QToolBar(arg0: *mut c_void) -> i32;
  // proto: void QMainWindow::setCentralWidget(QWidget * widget);
  fn _ZN11QMainWindow16setCentralWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QMainWindow::setMenuBar(QMenuBar * menubar);
  fn _ZN11QMainWindow10setMenuBarEP8QMenuBar(arg0: *mut c_void) -> i32;
  // proto: QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
  fn _ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QMainWindow)=1
pub struct QMainWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMainWindow {
  pub fn statusBar<T: QMainWindow_statusBar>(&mut self, value: T) -> i32 {
    value.statusBar(self);
    return 1;
  }
}

pub trait QMainWindow_statusBar {
  fn statusBar(self, this: &mut QMainWindow) -> i32;
}

// proto: QStatusBar * QMainWindow::statusBar();
impl<'a> /*trait*/ QMainWindow_statusBar for () {
  fn statusBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow9statusBarEv()};
    unsafe {_ZNK11QMainWindow9statusBarEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setAnimated<T: QMainWindow_setAnimated>(&mut self, value: T) -> i32 {
    value.setAnimated(self);
    return 1;
  }
}

pub trait QMainWindow_setAnimated {
  fn setAnimated(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setAnimated(bool enabled);
impl<'a> /*trait*/ QMainWindow_setAnimated for (i8) {
  fn setAnimated(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setAnimatedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QMainWindow11setAnimatedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setDockNestingEnabled<T: QMainWindow_setDockNestingEnabled>(&mut self, value: T) -> i32 {
    value.setDockNestingEnabled(self);
    return 1;
  }
}

pub trait QMainWindow_setDockNestingEnabled {
  fn setDockNestingEnabled(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setDockNestingEnabled(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDockNestingEnabled for (i8) {
  fn setDockNestingEnabled(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow21setDockNestingEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QMainWindow21setDockNestingEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn iconSizeChanged<T: QMainWindow_iconSizeChanged>(&mut self, value: T) -> i32 {
    value.iconSizeChanged(self);
    return 1;
  }
}

pub trait QMainWindow_iconSizeChanged {
  fn iconSizeChanged(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_iconSizeChanged for (&'a  QSize) {
  fn iconSizeChanged(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QMainWindow15iconSizeChangedERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn unifiedTitleAndToolBarOnMac<T: QMainWindow_unifiedTitleAndToolBarOnMac>(&mut self, value: T) -> i32 {
    value.unifiedTitleAndToolBarOnMac(self);
    return 1;
  }
}

pub trait QMainWindow_unifiedTitleAndToolBarOnMac {
  fn unifiedTitleAndToolBarOnMac(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::unifiedTitleAndToolBarOnMac();
impl<'a> /*trait*/ QMainWindow_unifiedTitleAndToolBarOnMac for () {
  fn unifiedTitleAndToolBarOnMac(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv()};
    unsafe {_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn menuWidget<T: QMainWindow_menuWidget>(&mut self, value: T) -> i32 {
    value.menuWidget(self);
    return 1;
  }
}

pub trait QMainWindow_menuWidget {
  fn menuWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: QWidget * QMainWindow::menuWidget();
impl<'a> /*trait*/ QMainWindow_menuWidget for () {
  fn menuWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10menuWidgetEv()};
    unsafe {_ZNK11QMainWindow10menuWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn tabifyDockWidget<T: QMainWindow_tabifyDockWidget>(&mut self, value: T) -> i32 {
    value.tabifyDockWidget(self);
    return 1;
  }
}

pub trait QMainWindow_tabifyDockWidget {
  fn tabifyDockWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
impl<'a> /*trait*/ QMainWindow_tabifyDockWidget for (&'a mut QDockWidget, &'a mut QDockWidget) {
  fn tabifyDockWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setDocumentMode<T: QMainWindow_setDocumentMode>(&mut self, value: T) -> i32 {
    value.setDocumentMode(self);
    return 1;
  }
}

pub trait QMainWindow_setDocumentMode {
  fn setDocumentMode(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDocumentMode for (i8) {
  fn setDocumentMode(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15setDocumentModeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QMainWindow15setDocumentModeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn centralWidget<T: QMainWindow_centralWidget>(&mut self, value: T) -> i32 {
    value.centralWidget(self);
    return 1;
  }
}

pub trait QMainWindow_centralWidget {
  fn centralWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: QWidget * QMainWindow::centralWidget();
impl<'a> /*trait*/ QMainWindow_centralWidget for () {
  fn centralWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow13centralWidgetEv()};
    unsafe {_ZNK11QMainWindow13centralWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeDockWidget<T: QMainWindow_removeDockWidget>(&mut self, value: T) -> i32 {
    value.removeDockWidget(self);
    return 1;
  }
}

pub trait QMainWindow_removeDockWidget {
  fn removeDockWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_removeDockWidget for (&'a mut QDockWidget) {
  fn removeDockWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16removeDockWidgetEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow16removeDockWidgetEP11QDockWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn NewQMainWindow<T: QMainWindow_NewQMainWindow>(value: T) -> QMainWindow {
    let rsthis = value.NewQMainWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QMainWindow_NewQMainWindow {
  fn NewQMainWindow(self) -> QMainWindow;
}

// proto: void QMainWindow::NewQMainWindow(const QMainWindow & );
impl<'a> /*trait*/ QMainWindow_NewQMainWindow for (&'a  QMainWindow) {
  fn NewQMainWindow(self) -> QMainWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QMainWindowC1ERKS_(qthis, arg0)};
    let rsthis = QMainWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isAnimated<T: QMainWindow_isAnimated>(&mut self, value: T) -> i32 {
    value.isAnimated(self);
    return 1;
  }
}

pub trait QMainWindow_isAnimated {
  fn isAnimated(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::isAnimated();
impl<'a> /*trait*/ QMainWindow_isAnimated for () {
  fn isAnimated(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10isAnimatedEv()};
    unsafe {_ZNK11QMainWindow10isAnimatedEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn addToolBar<T: QMainWindow_addToolBar>(&mut self, value: T) -> i32 {
    value.addToolBar(self);
    return 1;
  }
}

pub trait QMainWindow_addToolBar {
  fn addToolBar(self, this: &mut QMainWindow) -> i32;
}

// proto: QToolBar * QMainWindow::addToolBar(const QString & title);
impl<'a> /*trait*/ QMainWindow_addToolBar for (&'a  QString) {
  fn addToolBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10addToolBarERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QMainWindow10addToolBarERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setIconSize<T: QMainWindow_setIconSize>(&mut self, value: T) -> i32 {
    value.setIconSize(self);
    return 1;
  }
}

pub trait QMainWindow_setIconSize {
  fn setIconSize(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_setIconSize for (&'a  QSize) {
  fn setIconSize(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QMainWindow11setIconSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn saveState<T: QMainWindow_saveState>(&mut self, value: T) -> i32 {
    value.saveState(self);
    return 1;
  }
}

pub trait QMainWindow_saveState {
  fn saveState(self, this: &mut QMainWindow) -> i32;
}

// proto: QByteArray QMainWindow::saveState(int version);
impl<'a> /*trait*/ QMainWindow_saveState for (i32) {
  fn saveState(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow9saveStateEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QMainWindow9saveStateEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn restoreState<T: QMainWindow_restoreState>(&mut self, value: T) -> i32 {
    value.restoreState(self);
    return 1;
  }
}

pub trait QMainWindow_restoreState {
  fn restoreState(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::restoreState(const QByteArray & state, int version);
impl<'a> /*trait*/ QMainWindow_restoreState for (&'a  QByteArray, i32) {
  fn restoreState(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12restoreStateERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QMainWindow12restoreStateERK10QByteArrayi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn insertToolBar<T: QMainWindow_insertToolBar>(&mut self, value: T) -> i32 {
    value.insertToolBar(self);
    return 1;
  }
}

pub trait QMainWindow_insertToolBar {
  fn insertToolBar(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_insertToolBar for (&'a mut QToolBar, &'a mut QToolBar) {
  fn insertToolBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13insertToolBarEP8QToolBarS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow13insertToolBarEP8QToolBarS1_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn createPopupMenu<T: QMainWindow_createPopupMenu>(&mut self, value: T) -> i32 {
    value.createPopupMenu(self);
    return 1;
  }
}

pub trait QMainWindow_createPopupMenu {
  fn createPopupMenu(self, this: &mut QMainWindow) -> i32;
}

// proto: QMenu * QMainWindow::createPopupMenu();
impl<'a> /*trait*/ QMainWindow_createPopupMenu for () {
  fn createPopupMenu(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15createPopupMenuEv()};
    unsafe {_ZN11QMainWindow15createPopupMenuEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setUnifiedTitleAndToolBarOnMac<T: QMainWindow_setUnifiedTitleAndToolBarOnMac>(&mut self, value: T) -> i32 {
    value.setUnifiedTitleAndToolBarOnMac(self);
    return 1;
  }
}

pub trait QMainWindow_setUnifiedTitleAndToolBarOnMac {
  fn setUnifiedTitleAndToolBarOnMac(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
impl<'a> /*trait*/ QMainWindow_setUnifiedTitleAndToolBarOnMac for (i8) {
  fn setUnifiedTitleAndToolBarOnMac(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(arg0)};
    return 1;
  }
}

// proto: void QMainWindow::addToolBar(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_addToolBar for (&'a mut QToolBar) {
  fn addToolBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10addToolBarEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow10addToolBarEP8QToolBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeToolBarBreak<T: QMainWindow_removeToolBarBreak>(&mut self, value: T) -> i32 {
    value.removeToolBarBreak(self);
    return 1;
  }
}

pub trait QMainWindow_removeToolBarBreak {
  fn removeToolBarBreak(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::removeToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_removeToolBarBreak for (&'a mut QToolBar) {
  fn removeToolBarBreak(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18removeToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow18removeToolBarBreakEP8QToolBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn toolBarBreak<T: QMainWindow_toolBarBreak>(&mut self, value: T) -> i32 {
    value.toolBarBreak(self);
    return 1;
  }
}

pub trait QMainWindow_toolBarBreak {
  fn toolBarBreak(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::toolBarBreak(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_toolBarBreak for (&'a mut QToolBar) {
  fn toolBarBreak(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12toolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK11QMainWindow12toolBarBreakEP8QToolBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn restoreDockWidget<T: QMainWindow_restoreDockWidget>(&mut self, value: T) -> i32 {
    value.restoreDockWidget(self);
    return 1;
  }
}

pub trait QMainWindow_restoreDockWidget {
  fn restoreDockWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_restoreDockWidget for (&'a mut QDockWidget) {
  fn restoreDockWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn menuBar<T: QMainWindow_menuBar>(&mut self, value: T) -> i32 {
    value.menuBar(self);
    return 1;
  }
}

pub trait QMainWindow_menuBar {
  fn menuBar(self, this: &mut QMainWindow) -> i32;
}

// proto: QMenuBar * QMainWindow::menuBar();
impl<'a> /*trait*/ QMainWindow_menuBar for () {
  fn menuBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow7menuBarEv()};
    unsafe {_ZNK11QMainWindow7menuBarEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setStatusBar<T: QMainWindow_setStatusBar>(&mut self, value: T) -> i32 {
    value.setStatusBar(self);
    return 1;
  }
}

pub trait QMainWindow_setStatusBar {
  fn setStatusBar(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setStatusBar(QStatusBar * statusbar);
impl<'a> /*trait*/ QMainWindow_setStatusBar for (&'a mut QStatusBar) {
  fn setStatusBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12setStatusBarEP10QStatusBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow12setStatusBarEP10QStatusBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn FreeQMainWindow<T: QMainWindow_FreeQMainWindow>(&mut self, value: T) -> i32 {
    value.FreeQMainWindow(self);
    return 1;
  }
}

pub trait QMainWindow_FreeQMainWindow {
  fn FreeQMainWindow(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::FreeQMainWindow();
impl<'a> /*trait*/ QMainWindow_FreeQMainWindow for () {
  fn FreeQMainWindow(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindowD0Ev()};
    unsafe {_ZN11QMainWindowD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isSeparator<T: QMainWindow_isSeparator>(&mut self, value: T) -> i32 {
    value.isSeparator(self);
    return 1;
  }
}

pub trait QMainWindow_isSeparator {
  fn isSeparator(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::isSeparator(const QPoint & pos);
impl<'a> /*trait*/ QMainWindow_isSeparator for (&'a  QPoint) {
  fn isSeparator(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow11isSeparatorERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QMainWindow11isSeparatorERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn iconSize<T: QMainWindow_iconSize>(&mut self, value: T) -> i32 {
    value.iconSize(self);
    return 1;
  }
}

pub trait QMainWindow_iconSize {
  fn iconSize(self, this: &mut QMainWindow) -> i32;
}

// proto: QSize QMainWindow::iconSize();
impl<'a> /*trait*/ QMainWindow_iconSize for () {
  fn iconSize(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow8iconSizeEv()};
    unsafe {_ZNK11QMainWindow8iconSizeEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn metaObject<T: QMainWindow_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMainWindow_metaObject {
  fn metaObject(self, this: &mut QMainWindow) -> i32;
}

// proto: const QMetaObject * QMainWindow::metaObject();
impl<'a> /*trait*/ QMainWindow_metaObject for () {
  fn metaObject(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10metaObjectEv()};
    unsafe {_ZNK11QMainWindow10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn insertToolBarBreak<T: QMainWindow_insertToolBarBreak>(&mut self, value: T) -> i32 {
    value.insertToolBarBreak(self);
    return 1;
  }
}

pub trait QMainWindow_insertToolBarBreak {
  fn insertToolBarBreak(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::insertToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_insertToolBarBreak for (&'a mut QToolBar) {
  fn insertToolBarBreak(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18insertToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow18insertToolBarBreakEP8QToolBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn takeCentralWidget<T: QMainWindow_takeCentralWidget>(&mut self, value: T) -> i32 {
    value.takeCentralWidget(self);
    return 1;
  }
}

pub trait QMainWindow_takeCentralWidget {
  fn takeCentralWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: QWidget * QMainWindow::takeCentralWidget();
impl<'a> /*trait*/ QMainWindow_takeCentralWidget for () {
  fn takeCentralWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow17takeCentralWidgetEv()};
    unsafe {_ZN11QMainWindow17takeCentralWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isDockNestingEnabled<T: QMainWindow_isDockNestingEnabled>(&mut self, value: T) -> i32 {
    value.isDockNestingEnabled(self);
    return 1;
  }
}

pub trait QMainWindow_isDockNestingEnabled {
  fn isDockNestingEnabled(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::isDockNestingEnabled();
impl<'a> /*trait*/ QMainWindow_isDockNestingEnabled for () {
  fn isDockNestingEnabled(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow20isDockNestingEnabledEv()};
    unsafe {_ZNK11QMainWindow20isDockNestingEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn documentMode<T: QMainWindow_documentMode>(&mut self, value: T) -> i32 {
    value.documentMode(self);
    return 1;
  }
}

pub trait QMainWindow_documentMode {
  fn documentMode(self, this: &mut QMainWindow) -> i32;
}

// proto: bool QMainWindow::documentMode();
impl<'a> /*trait*/ QMainWindow_documentMode for () {
  fn documentMode(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12documentModeEv()};
    unsafe {_ZNK11QMainWindow12documentModeEv()};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setMenuWidget<T: QMainWindow_setMenuWidget>(&mut self, value: T) -> i32 {
    value.setMenuWidget(self);
    return 1;
  }
}

pub trait QMainWindow_setMenuWidget {
  fn setMenuWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setMenuWidget(QWidget * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuWidget for (&'a mut QWidget) {
  fn setMenuWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13setMenuWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow13setMenuWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeToolBar<T: QMainWindow_removeToolBar>(&mut self, value: T) -> i32 {
    value.removeToolBar(self);
    return 1;
  }
}

pub trait QMainWindow_removeToolBar {
  fn removeToolBar(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::removeToolBar(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_removeToolBar for (&'a mut QToolBar) {
  fn removeToolBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13removeToolBarEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow13removeToolBarEP8QToolBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setCentralWidget<T: QMainWindow_setCentralWidget>(&mut self, value: T) -> i32 {
    value.setCentralWidget(self);
    return 1;
  }
}

pub trait QMainWindow_setCentralWidget {
  fn setCentralWidget(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setCentralWidget(QWidget * widget);
impl<'a> /*trait*/ QMainWindow_setCentralWidget for (&'a mut QWidget) {
  fn setCentralWidget(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16setCentralWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow16setCentralWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setMenuBar<T: QMainWindow_setMenuBar>(&mut self, value: T) -> i32 {
    value.setMenuBar(self);
    return 1;
  }
}

pub trait QMainWindow_setMenuBar {
  fn setMenuBar(self, this: &mut QMainWindow) -> i32;
}

// proto: void QMainWindow::setMenuBar(QMenuBar * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuBar for (&'a mut QMenuBar) {
  fn setMenuBar(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10setMenuBarEP8QMenuBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindow10setMenuBarEP8QMenuBar(arg0)};
    return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn tabifiedDockWidgets<T: QMainWindow_tabifiedDockWidgets>(&mut self, value: T) -> i32 {
    value.tabifiedDockWidgets(self);
    return 1;
  }
}

pub trait QMainWindow_tabifiedDockWidgets {
  fn tabifiedDockWidgets(self, this: &mut QMainWindow) -> i32;
}

// proto: QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_tabifiedDockWidgets for (&'a mut QDockWidget) {
  fn tabifiedDockWidgets(self, this: &mut QMainWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(arg0)};
    return 1;
  }
}

