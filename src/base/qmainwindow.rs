// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstatusbar::QStatusBar;
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qdockwidget::QDockWidget;
use super::qstring::QString;
use super::qtoolbar::QToolBar;
use super::qbytearray::QByteArray;
use super::qmenu::QMenu;
use super::qmenubar::QMenuBar;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QStatusBar * QMainWindow::statusBar();
  fn _ZNK11QMainWindow9statusBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setAnimated(bool enabled);
  fn _ZN11QMainWindow11setAnimatedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMainWindow::setDockNestingEnabled(bool enabled);
  fn _ZN11QMainWindow21setDockNestingEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMainWindow::iconSizeChanged(const QSize & iconSize);
  fn _ZN11QMainWindow15iconSizeChangedERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMainWindow::unifiedTitleAndToolBarOnMac();
  fn _ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv(qthis: *mut c_void) -> int8_t;
  // proto:  QWidget * QMainWindow::menuWidget();
  fn _ZNK11QMainWindow10menuWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
  fn _ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QMainWindow::setDocumentMode(bool enabled);
  fn _ZN11QMainWindow15setDocumentModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QWidget * QMainWindow::centralWidget();
  fn _ZNK11QMainWindow13centralWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
  fn _ZN11QMainWindow16removeDockWidgetEP11QDockWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMainWindow::NewQMainWindow(const QMainWindow & );
  fn _ZN11QMainWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMainWindow::isAnimated();
  fn _ZNK11QMainWindow10isAnimatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QToolBar * QMainWindow::addToolBar(const QString & title);
  fn _ZN11QMainWindow10addToolBarERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setIconSize(const QSize & iconSize);
  fn _ZN11QMainWindow11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QMainWindow::saveState(int version);
  fn _ZNK11QMainWindow9saveStateEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QMainWindow::restoreState(const QByteArray & state, int version);
  fn _ZN11QMainWindow12restoreStateERK10QByteArrayi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> int8_t;
  // proto:  void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
  fn _ZN11QMainWindow13insertToolBarEP8QToolBarS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QMenu * QMainWindow::createPopupMenu();
  fn _ZN11QMainWindow15createPopupMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
  fn _ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMainWindow::removeToolBarBreak(QToolBar * before);
  fn _ZN11QMainWindow18removeToolBarBreakEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMainWindow::toolBarBreak(QToolBar * toolbar);
  fn _ZNK11QMainWindow12toolBarBreakEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
  fn _ZN11QMainWindow17restoreDockWidgetEP11QDockWidget(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QMenuBar * QMainWindow::menuBar();
  fn _ZNK11QMainWindow7menuBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setStatusBar(QStatusBar * statusbar);
  fn _ZN11QMainWindow12setStatusBarEP10QStatusBar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMainWindow::FreeQMainWindow();
  fn _ZN11QMainWindowD0Ev(qthis: *mut c_void) ;
  // proto:  bool QMainWindow::isSeparator(const QPoint & pos);
  fn _ZNK11QMainWindow11isSeparatorERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QSize QMainWindow::iconSize();
  fn _ZNK11QMainWindow8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QMainWindow::metaObject();
  fn _ZNK11QMainWindow10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QMainWindow::insertToolBarBreak(QToolBar * before);
  fn _ZN11QMainWindow18insertToolBarBreakEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QMainWindow::takeCentralWidget();
  fn _ZN11QMainWindow17takeCentralWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMainWindow::isDockNestingEnabled();
  fn _ZNK11QMainWindow20isDockNestingEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QMainWindow::documentMode();
  fn _ZNK11QMainWindow12documentModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMainWindow::setMenuWidget(QWidget * menubar);
  fn _ZN11QMainWindow13setMenuWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMainWindow::removeToolBar(QToolBar * toolbar);
  fn _ZN11QMainWindow13removeToolBarEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMainWindow::setCentralWidget(QWidget * widget);
  fn _ZN11QMainWindow16setCentralWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMainWindow::setMenuBar(QMenuBar * menubar);
  fn _ZN11QMainWindow10setMenuBarEP8QMenuBar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
  fn _ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QMainWindow)=1
pub struct QMainWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMainWindow {
  pub fn statusBar<T: QMainWindow_statusBar>(&mut self, value: T) -> QStatusBar {
    return value.statusBar(self);
    // return 1;
  }
}

pub trait QMainWindow_statusBar {
  fn statusBar(self, rsthis: &mut QMainWindow) -> QStatusBar;
}

// proto:  QStatusBar * QMainWindow::statusBar();
impl<'a> /*trait*/ QMainWindow_statusBar for () {
  fn statusBar(self, rsthis: &mut QMainWindow) -> QStatusBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow9statusBarEv()};
    let mut ret = unsafe {_ZNK11QMainWindow9statusBarEv(rsthis.qclsinst)};
    let mut ret1 = QStatusBar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setAnimated<T: QMainWindow_setAnimated>(&mut self, value: T)  {
     value.setAnimated(self);
    // return 1;
  }
}

pub trait QMainWindow_setAnimated {
  fn setAnimated(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setAnimated(bool enabled);
impl<'a> /*trait*/ QMainWindow_setAnimated for (i8) {
  fn setAnimated(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setAnimatedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setDockNestingEnabled<T: QMainWindow_setDockNestingEnabled>(&mut self, value: T)  {
     value.setDockNestingEnabled(self);
    // return 1;
  }
}

pub trait QMainWindow_setDockNestingEnabled {
  fn setDockNestingEnabled(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setDockNestingEnabled(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDockNestingEnabled for (i8) {
  fn setDockNestingEnabled(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow21setDockNestingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow21setDockNestingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn iconSizeChanged<T: QMainWindow_iconSizeChanged>(&mut self, value: T)  {
     value.iconSizeChanged(self);
    // return 1;
  }
}

pub trait QMainWindow_iconSizeChanged {
  fn iconSizeChanged(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_iconSizeChanged for (&'a  QSize) {
  fn iconSizeChanged(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow15iconSizeChangedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn unifiedTitleAndToolBarOnMac<T: QMainWindow_unifiedTitleAndToolBarOnMac>(&mut self, value: T) -> i8 {
    return value.unifiedTitleAndToolBarOnMac(self);
    // return 1;
  }
}

pub trait QMainWindow_unifiedTitleAndToolBarOnMac {
  fn unifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::unifiedTitleAndToolBarOnMac();
impl<'a> /*trait*/ QMainWindow_unifiedTitleAndToolBarOnMac for () {
  fn unifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv()};
    let mut ret = unsafe {_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn menuWidget<T: QMainWindow_menuWidget>(&mut self, value: T) -> QWidget {
    return value.menuWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_menuWidget {
  fn menuWidget(self, rsthis: &mut QMainWindow) -> QWidget;
}

// proto:  QWidget * QMainWindow::menuWidget();
impl<'a> /*trait*/ QMainWindow_menuWidget for () {
  fn menuWidget(self, rsthis: &mut QMainWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10menuWidgetEv()};
    let mut ret = unsafe {_ZNK11QMainWindow10menuWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn tabifyDockWidget<T: QMainWindow_tabifyDockWidget>(&mut self, value: T)  {
     value.tabifyDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_tabifyDockWidget {
  fn tabifyDockWidget(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
impl<'a> /*trait*/ QMainWindow_tabifyDockWidget for (&'a mut QDockWidget, &'a mut QDockWidget) {
  fn tabifyDockWidget(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setDocumentMode<T: QMainWindow_setDocumentMode>(&mut self, value: T)  {
     value.setDocumentMode(self);
    // return 1;
  }
}

pub trait QMainWindow_setDocumentMode {
  fn setDocumentMode(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDocumentMode for (i8) {
  fn setDocumentMode(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15setDocumentModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn centralWidget<T: QMainWindow_centralWidget>(&mut self, value: T) -> QWidget {
    return value.centralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_centralWidget {
  fn centralWidget(self, rsthis: &mut QMainWindow) -> QWidget;
}

// proto:  QWidget * QMainWindow::centralWidget();
impl<'a> /*trait*/ QMainWindow_centralWidget for () {
  fn centralWidget(self, rsthis: &mut QMainWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow13centralWidgetEv()};
    let mut ret = unsafe {_ZNK11QMainWindow13centralWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeDockWidget<T: QMainWindow_removeDockWidget>(&mut self, value: T)  {
     value.removeDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_removeDockWidget {
  fn removeDockWidget(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_removeDockWidget for (&'a mut QDockWidget) {
  fn removeDockWidget(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16removeDockWidgetEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16removeDockWidgetEP11QDockWidget(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindowC1ERKS_(qthis, arg0)};
    let rsthis = QMainWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isAnimated<T: QMainWindow_isAnimated>(&mut self, value: T) -> i8 {
    return value.isAnimated(self);
    // return 1;
  }
}

pub trait QMainWindow_isAnimated {
  fn isAnimated(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::isAnimated();
impl<'a> /*trait*/ QMainWindow_isAnimated for () {
  fn isAnimated(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10isAnimatedEv()};
    let mut ret = unsafe {_ZNK11QMainWindow10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn addToolBar<T: QMainWindow_addToolBar>(&mut self, value: T) -> QToolBar {
    return value.addToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_addToolBar {
  fn addToolBar(self, rsthis: &mut QMainWindow) -> QToolBar;
}

// proto:  QToolBar * QMainWindow::addToolBar(const QString & title);
impl<'a> /*trait*/ QMainWindow_addToolBar for (&'a  QString) {
  fn addToolBar(self, rsthis: &mut QMainWindow) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10addToolBarERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QMainWindow10addToolBarERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QToolBar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setIconSize<T: QMainWindow_setIconSize>(&mut self, value: T)  {
     value.setIconSize(self);
    // return 1;
  }
}

pub trait QMainWindow_setIconSize {
  fn setIconSize(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_setIconSize for (&'a  QSize) {
  fn setIconSize(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn saveState<T: QMainWindow_saveState>(&mut self, value: T) -> QByteArray {
    return value.saveState(self);
    // return 1;
  }
}

pub trait QMainWindow_saveState {
  fn saveState(self, rsthis: &mut QMainWindow) -> QByteArray;
}

// proto:  QByteArray QMainWindow::saveState(int version);
impl<'a> /*trait*/ QMainWindow_saveState for (i32) {
  fn saveState(self, rsthis: &mut QMainWindow) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow9saveStateEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMainWindow9saveStateEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn restoreState<T: QMainWindow_restoreState>(&mut self, value: T) -> i8 {
    return value.restoreState(self);
    // return 1;
  }
}

pub trait QMainWindow_restoreState {
  fn restoreState(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::restoreState(const QByteArray & state, int version);
impl<'a> /*trait*/ QMainWindow_restoreState for (&'a  QByteArray, i32) {
  fn restoreState(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12restoreStateERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QMainWindow12restoreStateERK10QByteArrayi(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn insertToolBar<T: QMainWindow_insertToolBar>(&mut self, value: T)  {
     value.insertToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_insertToolBar {
  fn insertToolBar(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_insertToolBar for (&'a mut QToolBar, &'a mut QToolBar) {
  fn insertToolBar(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13insertToolBarEP8QToolBarS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13insertToolBarEP8QToolBarS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn createPopupMenu<T: QMainWindow_createPopupMenu>(&mut self, value: T) -> QMenu {
    return value.createPopupMenu(self);
    // return 1;
  }
}

pub trait QMainWindow_createPopupMenu {
  fn createPopupMenu(self, rsthis: &mut QMainWindow) -> QMenu;
}

// proto:  QMenu * QMainWindow::createPopupMenu();
impl<'a> /*trait*/ QMainWindow_createPopupMenu for () {
  fn createPopupMenu(self, rsthis: &mut QMainWindow) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15createPopupMenuEv()};
    let mut ret = unsafe {_ZN11QMainWindow15createPopupMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setUnifiedTitleAndToolBarOnMac<T: QMainWindow_setUnifiedTitleAndToolBarOnMac>(&mut self, value: T)  {
     value.setUnifiedTitleAndToolBarOnMac(self);
    // return 1;
  }
}

pub trait QMainWindow_setUnifiedTitleAndToolBarOnMac {
  fn setUnifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
impl<'a> /*trait*/ QMainWindow_setUnifiedTitleAndToolBarOnMac for (i8) {
  fn setUnifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeToolBarBreak<T: QMainWindow_removeToolBarBreak>(&mut self, value: T)  {
     value.removeToolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_removeToolBarBreak {
  fn removeToolBarBreak(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::removeToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_removeToolBarBreak for (&'a mut QToolBar) {
  fn removeToolBarBreak(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18removeToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow18removeToolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn toolBarBreak<T: QMainWindow_toolBarBreak>(&mut self, value: T) -> i8 {
    return value.toolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_toolBarBreak {
  fn toolBarBreak(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::toolBarBreak(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_toolBarBreak for (&'a mut QToolBar) {
  fn toolBarBreak(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12toolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QMainWindow12toolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn restoreDockWidget<T: QMainWindow_restoreDockWidget>(&mut self, value: T) -> i8 {
    return value.restoreDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_restoreDockWidget {
  fn restoreDockWidget(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_restoreDockWidget for (&'a mut QDockWidget) {
  fn restoreDockWidget(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn menuBar<T: QMainWindow_menuBar>(&mut self, value: T) -> QMenuBar {
    return value.menuBar(self);
    // return 1;
  }
}

pub trait QMainWindow_menuBar {
  fn menuBar(self, rsthis: &mut QMainWindow) -> QMenuBar;
}

// proto:  QMenuBar * QMainWindow::menuBar();
impl<'a> /*trait*/ QMainWindow_menuBar for () {
  fn menuBar(self, rsthis: &mut QMainWindow) -> QMenuBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow7menuBarEv()};
    let mut ret = unsafe {_ZNK11QMainWindow7menuBarEv(rsthis.qclsinst)};
    let mut ret1 = QMenuBar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setStatusBar<T: QMainWindow_setStatusBar>(&mut self, value: T)  {
     value.setStatusBar(self);
    // return 1;
  }
}

pub trait QMainWindow_setStatusBar {
  fn setStatusBar(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setStatusBar(QStatusBar * statusbar);
impl<'a> /*trait*/ QMainWindow_setStatusBar for (&'a mut QStatusBar) {
  fn setStatusBar(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12setStatusBarEP10QStatusBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow12setStatusBarEP10QStatusBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn FreeQMainWindow<T: QMainWindow_FreeQMainWindow>(&mut self, value: T)  {
     value.FreeQMainWindow(self);
    // return 1;
  }
}

pub trait QMainWindow_FreeQMainWindow {
  fn FreeQMainWindow(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::FreeQMainWindow();
impl<'a> /*trait*/ QMainWindow_FreeQMainWindow for () {
  fn FreeQMainWindow(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindowD0Ev()};
     unsafe {_ZN11QMainWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isSeparator<T: QMainWindow_isSeparator>(&mut self, value: T) -> i8 {
    return value.isSeparator(self);
    // return 1;
  }
}

pub trait QMainWindow_isSeparator {
  fn isSeparator(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::isSeparator(const QPoint & pos);
impl<'a> /*trait*/ QMainWindow_isSeparator for (&'a  QPoint) {
  fn isSeparator(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow11isSeparatorERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QMainWindow11isSeparatorERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn iconSize<T: QMainWindow_iconSize>(&mut self, value: T) -> QSize {
    return value.iconSize(self);
    // return 1;
  }
}

pub trait QMainWindow_iconSize {
  fn iconSize(self, rsthis: &mut QMainWindow) -> QSize;
}

// proto:  QSize QMainWindow::iconSize();
impl<'a> /*trait*/ QMainWindow_iconSize for () {
  fn iconSize(self, rsthis: &mut QMainWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow8iconSizeEv()};
    let mut ret = unsafe {_ZNK11QMainWindow8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn metaObject<T: QMainWindow_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMainWindow_metaObject {
  fn metaObject(self, rsthis: &mut QMainWindow) ;
}

// proto:  const QMetaObject * QMainWindow::metaObject();
impl<'a> /*trait*/ QMainWindow_metaObject for () {
  fn metaObject(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10metaObjectEv()};
     unsafe {_ZNK11QMainWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn insertToolBarBreak<T: QMainWindow_insertToolBarBreak>(&mut self, value: T)  {
     value.insertToolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_insertToolBarBreak {
  fn insertToolBarBreak(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::insertToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_insertToolBarBreak for (&'a mut QToolBar) {
  fn insertToolBarBreak(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18insertToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow18insertToolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn takeCentralWidget<T: QMainWindow_takeCentralWidget>(&mut self, value: T) -> QWidget {
    return value.takeCentralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_takeCentralWidget {
  fn takeCentralWidget(self, rsthis: &mut QMainWindow) -> QWidget;
}

// proto:  QWidget * QMainWindow::takeCentralWidget();
impl<'a> /*trait*/ QMainWindow_takeCentralWidget for () {
  fn takeCentralWidget(self, rsthis: &mut QMainWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow17takeCentralWidgetEv()};
    let mut ret = unsafe {_ZN11QMainWindow17takeCentralWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isDockNestingEnabled<T: QMainWindow_isDockNestingEnabled>(&mut self, value: T) -> i8 {
    return value.isDockNestingEnabled(self);
    // return 1;
  }
}

pub trait QMainWindow_isDockNestingEnabled {
  fn isDockNestingEnabled(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::isDockNestingEnabled();
impl<'a> /*trait*/ QMainWindow_isDockNestingEnabled for () {
  fn isDockNestingEnabled(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow20isDockNestingEnabledEv()};
    let mut ret = unsafe {_ZNK11QMainWindow20isDockNestingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn documentMode<T: QMainWindow_documentMode>(&mut self, value: T) -> i8 {
    return value.documentMode(self);
    // return 1;
  }
}

pub trait QMainWindow_documentMode {
  fn documentMode(self, rsthis: &mut QMainWindow) -> i8;
}

// proto:  bool QMainWindow::documentMode();
impl<'a> /*trait*/ QMainWindow_documentMode for () {
  fn documentMode(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12documentModeEv()};
    let mut ret = unsafe {_ZNK11QMainWindow12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setMenuWidget<T: QMainWindow_setMenuWidget>(&mut self, value: T)  {
     value.setMenuWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_setMenuWidget {
  fn setMenuWidget(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setMenuWidget(QWidget * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuWidget for (&'a mut QWidget) {
  fn setMenuWidget(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13setMenuWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13setMenuWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeToolBar<T: QMainWindow_removeToolBar>(&mut self, value: T)  {
     value.removeToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_removeToolBar {
  fn removeToolBar(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::removeToolBar(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_removeToolBar for (&'a mut QToolBar) {
  fn removeToolBar(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13removeToolBarEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13removeToolBarEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setCentralWidget<T: QMainWindow_setCentralWidget>(&mut self, value: T)  {
     value.setCentralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_setCentralWidget {
  fn setCentralWidget(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setCentralWidget(QWidget * widget);
impl<'a> /*trait*/ QMainWindow_setCentralWidget for (&'a mut QWidget) {
  fn setCentralWidget(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16setCentralWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16setCentralWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setMenuBar<T: QMainWindow_setMenuBar>(&mut self, value: T)  {
     value.setMenuBar(self);
    // return 1;
  }
}

pub trait QMainWindow_setMenuBar {
  fn setMenuBar(self, rsthis: &mut QMainWindow) ;
}

// proto:  void QMainWindow::setMenuBar(QMenuBar * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuBar for (&'a mut QMenuBar) {
  fn setMenuBar(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10setMenuBarEP8QMenuBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow10setMenuBarEP8QMenuBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn tabifiedDockWidgets<T: QMainWindow_tabifiedDockWidgets>(&mut self, value: T)  {
     value.tabifiedDockWidgets(self);
    // return 1;
  }
}

pub trait QMainWindow_tabifiedDockWidgets {
  fn tabifiedDockWidgets(self, rsthis: &mut QMainWindow) ;
}

// proto:  QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_tabifiedDockWidgets for (&'a mut QDockWidget) {
  fn tabifiedDockWidgets(self, rsthis: &mut QMainWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

