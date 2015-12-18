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
  pub fn statusBar<RetType, T: QMainWindow_statusBar<RetType>>(&mut self, value: T) -> RetType {
    return value.statusBar(self);
    // return 1;
  }
}

pub trait QMainWindow_statusBar<RetType> {
  fn statusBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QStatusBar * QMainWindow::statusBar();
impl<'a> /*trait*/ QMainWindow_statusBar<QStatusBar> for () {
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
  pub fn setAnimated<RetType, T: QMainWindow_setAnimated<RetType>>(&mut self, value: T) -> RetType {
    return value.setAnimated(self);
    // return 1;
  }
}

pub trait QMainWindow_setAnimated<RetType> {
  fn setAnimated(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setAnimated(bool enabled);
impl<'a> /*trait*/ QMainWindow_setAnimated<()> for (i8) {
  fn setAnimated(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setAnimatedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setDockNestingEnabled<RetType, T: QMainWindow_setDockNestingEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setDockNestingEnabled(self);
    // return 1;
  }
}

pub trait QMainWindow_setDockNestingEnabled<RetType> {
  fn setDockNestingEnabled(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setDockNestingEnabled(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDockNestingEnabled<()> for (i8) {
  fn setDockNestingEnabled(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow21setDockNestingEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow21setDockNestingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn iconSizeChanged<RetType, T: QMainWindow_iconSizeChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.iconSizeChanged(self);
    // return 1;
  }
}

pub trait QMainWindow_iconSizeChanged<RetType> {
  fn iconSizeChanged(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_iconSizeChanged<()> for (&'a  QSize) {
  fn iconSizeChanged(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow15iconSizeChangedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn unifiedTitleAndToolBarOnMac<RetType, T: QMainWindow_unifiedTitleAndToolBarOnMac<RetType>>(&mut self, value: T) -> RetType {
    return value.unifiedTitleAndToolBarOnMac(self);
    // return 1;
  }
}

pub trait QMainWindow_unifiedTitleAndToolBarOnMac<RetType> {
  fn unifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::unifiedTitleAndToolBarOnMac();
impl<'a> /*trait*/ QMainWindow_unifiedTitleAndToolBarOnMac<i8> for () {
  fn unifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv()};
    let mut ret = unsafe {_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn menuWidget<RetType, T: QMainWindow_menuWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.menuWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_menuWidget<RetType> {
  fn menuWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QWidget * QMainWindow::menuWidget();
impl<'a> /*trait*/ QMainWindow_menuWidget<QWidget> for () {
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
  pub fn tabifyDockWidget<RetType, T: QMainWindow_tabifyDockWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.tabifyDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_tabifyDockWidget<RetType> {
  fn tabifyDockWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
impl<'a> /*trait*/ QMainWindow_tabifyDockWidget<()> for (&'a mut QDockWidget, &'a mut QDockWidget) {
  fn tabifyDockWidget(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setDocumentMode<RetType, T: QMainWindow_setDocumentMode<RetType>>(&mut self, value: T) -> RetType {
    return value.setDocumentMode(self);
    // return 1;
  }
}

pub trait QMainWindow_setDocumentMode<RetType> {
  fn setDocumentMode(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15setDocumentModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn centralWidget<RetType, T: QMainWindow_centralWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.centralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_centralWidget<RetType> {
  fn centralWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QWidget * QMainWindow::centralWidget();
impl<'a> /*trait*/ QMainWindow_centralWidget<QWidget> for () {
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
  pub fn removeDockWidget<RetType, T: QMainWindow_removeDockWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.removeDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_removeDockWidget<RetType> {
  fn removeDockWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_removeDockWidget<()> for (&'a mut QDockWidget) {
  fn removeDockWidget(self, rsthis: &mut QMainWindow) -> () {
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
  pub fn isAnimated<RetType, T: QMainWindow_isAnimated<RetType>>(&mut self, value: T) -> RetType {
    return value.isAnimated(self);
    // return 1;
  }
}

pub trait QMainWindow_isAnimated<RetType> {
  fn isAnimated(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::isAnimated();
impl<'a> /*trait*/ QMainWindow_isAnimated<i8> for () {
  fn isAnimated(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10isAnimatedEv()};
    let mut ret = unsafe {_ZNK11QMainWindow10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn addToolBar<RetType, T: QMainWindow_addToolBar<RetType>>(&mut self, value: T) -> RetType {
    return value.addToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_addToolBar<RetType> {
  fn addToolBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QToolBar * QMainWindow::addToolBar(const QString & title);
impl<'a> /*trait*/ QMainWindow_addToolBar<QToolBar> for (&'a  QString) {
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
  pub fn setIconSize<RetType, T: QMainWindow_setIconSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setIconSize(self);
    // return 1;
  }
}

pub trait QMainWindow_setIconSize<RetType> {
  fn setIconSize(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_setIconSize<()> for (&'a  QSize) {
  fn setIconSize(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn saveState<RetType, T: QMainWindow_saveState<RetType>>(&mut self, value: T) -> RetType {
    return value.saveState(self);
    // return 1;
  }
}

pub trait QMainWindow_saveState<RetType> {
  fn saveState(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QByteArray QMainWindow::saveState(int version);
impl<'a> /*trait*/ QMainWindow_saveState<QByteArray> for (i32) {
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
  pub fn restoreState<RetType, T: QMainWindow_restoreState<RetType>>(&mut self, value: T) -> RetType {
    return value.restoreState(self);
    // return 1;
  }
}

pub trait QMainWindow_restoreState<RetType> {
  fn restoreState(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::restoreState(const QByteArray & state, int version);
impl<'a> /*trait*/ QMainWindow_restoreState<i8> for (&'a  QByteArray, i32) {
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
  pub fn insertToolBar<RetType, T: QMainWindow_insertToolBar<RetType>>(&mut self, value: T) -> RetType {
    return value.insertToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_insertToolBar<RetType> {
  fn insertToolBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_insertToolBar<()> for (&'a mut QToolBar, &'a mut QToolBar) {
  fn insertToolBar(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13insertToolBarEP8QToolBarS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13insertToolBarEP8QToolBarS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn createPopupMenu<RetType, T: QMainWindow_createPopupMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.createPopupMenu(self);
    // return 1;
  }
}

pub trait QMainWindow_createPopupMenu<RetType> {
  fn createPopupMenu(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QMenu * QMainWindow::createPopupMenu();
impl<'a> /*trait*/ QMainWindow_createPopupMenu<QMenu> for () {
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
  pub fn setUnifiedTitleAndToolBarOnMac<RetType, T: QMainWindow_setUnifiedTitleAndToolBarOnMac<RetType>>(&mut self, value: T) -> RetType {
    return value.setUnifiedTitleAndToolBarOnMac(self);
    // return 1;
  }
}

pub trait QMainWindow_setUnifiedTitleAndToolBarOnMac<RetType> {
  fn setUnifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
impl<'a> /*trait*/ QMainWindow_setUnifiedTitleAndToolBarOnMac<()> for (i8) {
  fn setUnifiedTitleAndToolBarOnMac(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeToolBarBreak<RetType, T: QMainWindow_removeToolBarBreak<RetType>>(&mut self, value: T) -> RetType {
    return value.removeToolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_removeToolBarBreak<RetType> {
  fn removeToolBarBreak(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::removeToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_removeToolBarBreak<()> for (&'a mut QToolBar) {
  fn removeToolBarBreak(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18removeToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow18removeToolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn toolBarBreak<RetType, T: QMainWindow_toolBarBreak<RetType>>(&mut self, value: T) -> RetType {
    return value.toolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_toolBarBreak<RetType> {
  fn toolBarBreak(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::toolBarBreak(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_toolBarBreak<i8> for (&'a mut QToolBar) {
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
  pub fn restoreDockWidget<RetType, T: QMainWindow_restoreDockWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.restoreDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_restoreDockWidget<RetType> {
  fn restoreDockWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_restoreDockWidget<i8> for (&'a mut QDockWidget) {
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
  pub fn menuBar<RetType, T: QMainWindow_menuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.menuBar(self);
    // return 1;
  }
}

pub trait QMainWindow_menuBar<RetType> {
  fn menuBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QMenuBar * QMainWindow::menuBar();
impl<'a> /*trait*/ QMainWindow_menuBar<QMenuBar> for () {
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
  pub fn setStatusBar<RetType, T: QMainWindow_setStatusBar<RetType>>(&mut self, value: T) -> RetType {
    return value.setStatusBar(self);
    // return 1;
  }
}

pub trait QMainWindow_setStatusBar<RetType> {
  fn setStatusBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setStatusBar(QStatusBar * statusbar);
impl<'a> /*trait*/ QMainWindow_setStatusBar<()> for (&'a mut QStatusBar) {
  fn setStatusBar(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12setStatusBarEP10QStatusBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow12setStatusBarEP10QStatusBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn FreeQMainWindow<RetType, T: QMainWindow_FreeQMainWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQMainWindow(self);
    // return 1;
  }
}

pub trait QMainWindow_FreeQMainWindow<RetType> {
  fn FreeQMainWindow(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::FreeQMainWindow();
impl<'a> /*trait*/ QMainWindow_FreeQMainWindow<()> for () {
  fn FreeQMainWindow(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindowD0Ev()};
     unsafe {_ZN11QMainWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn isSeparator<RetType, T: QMainWindow_isSeparator<RetType>>(&mut self, value: T) -> RetType {
    return value.isSeparator(self);
    // return 1;
  }
}

pub trait QMainWindow_isSeparator<RetType> {
  fn isSeparator(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::isSeparator(const QPoint & pos);
impl<'a> /*trait*/ QMainWindow_isSeparator<i8> for (&'a  QPoint) {
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
  pub fn iconSize<RetType, T: QMainWindow_iconSize<RetType>>(&mut self, value: T) -> RetType {
    return value.iconSize(self);
    // return 1;
  }
}

pub trait QMainWindow_iconSize<RetType> {
  fn iconSize(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QSize QMainWindow::iconSize();
impl<'a> /*trait*/ QMainWindow_iconSize<QSize> for () {
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
  pub fn metaObject<RetType, T: QMainWindow_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QMainWindow_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  const QMetaObject * QMainWindow::metaObject();
impl<'a> /*trait*/ QMainWindow_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10metaObjectEv()};
     unsafe {_ZNK11QMainWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn insertToolBarBreak<RetType, T: QMainWindow_insertToolBarBreak<RetType>>(&mut self, value: T) -> RetType {
    return value.insertToolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_insertToolBarBreak<RetType> {
  fn insertToolBarBreak(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::insertToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_insertToolBarBreak<()> for (&'a mut QToolBar) {
  fn insertToolBarBreak(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18insertToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow18insertToolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn takeCentralWidget<RetType, T: QMainWindow_takeCentralWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.takeCentralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_takeCentralWidget<RetType> {
  fn takeCentralWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QWidget * QMainWindow::takeCentralWidget();
impl<'a> /*trait*/ QMainWindow_takeCentralWidget<QWidget> for () {
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
  pub fn isDockNestingEnabled<RetType, T: QMainWindow_isDockNestingEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isDockNestingEnabled(self);
    // return 1;
  }
}

pub trait QMainWindow_isDockNestingEnabled<RetType> {
  fn isDockNestingEnabled(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::isDockNestingEnabled();
impl<'a> /*trait*/ QMainWindow_isDockNestingEnabled<i8> for () {
  fn isDockNestingEnabled(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow20isDockNestingEnabledEv()};
    let mut ret = unsafe {_ZNK11QMainWindow20isDockNestingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn documentMode<RetType, T: QMainWindow_documentMode<RetType>>(&mut self, value: T) -> RetType {
    return value.documentMode(self);
    // return 1;
  }
}

pub trait QMainWindow_documentMode<RetType> {
  fn documentMode(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  bool QMainWindow::documentMode();
impl<'a> /*trait*/ QMainWindow_documentMode<i8> for () {
  fn documentMode(self, rsthis: &mut QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12documentModeEv()};
    let mut ret = unsafe {_ZNK11QMainWindow12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setMenuWidget<RetType, T: QMainWindow_setMenuWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.setMenuWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_setMenuWidget<RetType> {
  fn setMenuWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setMenuWidget(QWidget * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuWidget<()> for (&'a mut QWidget) {
  fn setMenuWidget(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13setMenuWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13setMenuWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn removeToolBar<RetType, T: QMainWindow_removeToolBar<RetType>>(&mut self, value: T) -> RetType {
    return value.removeToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_removeToolBar<RetType> {
  fn removeToolBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::removeToolBar(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_removeToolBar<()> for (&'a mut QToolBar) {
  fn removeToolBar(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13removeToolBarEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13removeToolBarEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setCentralWidget<RetType, T: QMainWindow_setCentralWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.setCentralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_setCentralWidget<RetType> {
  fn setCentralWidget(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setCentralWidget(QWidget * widget);
impl<'a> /*trait*/ QMainWindow_setCentralWidget<()> for (&'a mut QWidget) {
  fn setCentralWidget(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16setCentralWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16setCentralWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn setMenuBar<RetType, T: QMainWindow_setMenuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.setMenuBar(self);
    // return 1;
  }
}

pub trait QMainWindow_setMenuBar<RetType> {
  fn setMenuBar(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  void QMainWindow::setMenuBar(QMenuBar * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuBar<()> for (&'a mut QMenuBar) {
  fn setMenuBar(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10setMenuBarEP8QMenuBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow10setMenuBarEP8QMenuBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMainWindow {
  pub fn tabifiedDockWidgets<RetType, T: QMainWindow_tabifiedDockWidgets<RetType>>(&mut self, value: T) -> RetType {
    return value.tabifiedDockWidgets(self);
    // return 1;
  }
}

pub trait QMainWindow_tabifiedDockWidgets<RetType> {
  fn tabifiedDockWidgets(self, rsthis: &mut QMainWindow) -> RetType;
}

// proto:  QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_tabifiedDockWidgets<()> for (&'a mut QDockWidget) {
  fn tabifiedDockWidgets(self, rsthis: &mut QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

