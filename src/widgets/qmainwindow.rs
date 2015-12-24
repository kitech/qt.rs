// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtWidgets/qmainwindow.h
// dst-file: /src/widgets/qmainwindow.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::qstatusbar::QStatusBar; // 773
use super::qtoolbar::QToolBar; // 773
use super::super::core::qsize::QSize; // 771
use super::qdockwidget::QDockWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qmenu::QMenu; // 773
use super::qmenubar::QMenuBar; // 773
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  QStatusBar * QMainWindow::statusBar();
  fn _ZNK11QMainWindow9statusBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setAnimated(bool enabled);
  fn _ZN11QMainWindow11setAnimatedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QMainWindow::setDockNestingEnabled(bool enabled);
  fn _ZN11QMainWindow21setDockNestingEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QMainWindow::iconSizeChanged(const QSize & iconSize);
  fn _ZN11QMainWindow15iconSizeChangedERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QMainWindow::unifiedTitleAndToolBarOnMac();
  fn _ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv(qthis: *mut c_void) -> c_char;
  // proto:  QWidget * QMainWindow::menuWidget();
  fn _ZNK11QMainWindow10menuWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
  fn _ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QMainWindow::setDocumentMode(bool enabled);
  fn _ZN11QMainWindow15setDocumentModeEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QWidget * QMainWindow::centralWidget();
  fn _ZNK11QMainWindow13centralWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
  fn _ZN11QMainWindow16removeDockWidgetEP11QDockWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMainWindow::QMainWindow(const QMainWindow & );
  fn _ZN11QMainWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QMainWindow::isAnimated();
  fn _ZNK11QMainWindow10isAnimatedEv(qthis: *mut c_void) -> c_char;
  // proto:  QToolBar * QMainWindow::addToolBar(const QString & title);
  fn _ZN11QMainWindow10addToolBarERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setIconSize(const QSize & iconSize);
  fn _ZN11QMainWindow11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QMainWindow::saveState(int version);
  fn _ZNK11QMainWindow9saveStateEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QMainWindow::restoreState(const QByteArray & state, int version);
  fn _ZN11QMainWindow12restoreStateERK10QByteArrayi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto:  void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
  fn _ZN11QMainWindow13insertToolBarEP8QToolBarS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QMenu * QMainWindow::createPopupMenu();
  fn _ZN11QMainWindow15createPopupMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
  fn _ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QMainWindow::addToolBar(QToolBar * toolbar);
  fn _ZN11QMainWindow10addToolBarEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMainWindow::removeToolBarBreak(QToolBar * before);
  fn _ZN11QMainWindow18removeToolBarBreakEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QMainWindow::toolBarBreak(QToolBar * toolbar);
  fn _ZNK11QMainWindow12toolBarBreakEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
  fn _ZN11QMainWindow17restoreDockWidgetEP11QDockWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QMenuBar * QMainWindow::menuBar();
  fn _ZNK11QMainWindow7menuBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMainWindow::setStatusBar(QStatusBar * statusbar);
  fn _ZN11QMainWindow12setStatusBarEP10QStatusBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMainWindow::~QMainWindow();
  fn _ZN11QMainWindowD0Ev(qthis: *mut c_void);
  // proto:  bool QMainWindow::isSeparator(const QPoint & pos);
  fn _ZNK11QMainWindow11isSeparatorERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QSize QMainWindow::iconSize();
  fn _ZNK11QMainWindow8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QMainWindow::metaObject();
  fn _ZNK11QMainWindow10metaObjectEv(qthis: *mut c_void);
  // proto:  void QMainWindow::insertToolBarBreak(QToolBar * before);
  fn _ZN11QMainWindow18insertToolBarBreakEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QMainWindow::takeCentralWidget();
  fn _ZN11QMainWindow17takeCentralWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMainWindow::isDockNestingEnabled();
  fn _ZNK11QMainWindow20isDockNestingEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QMainWindow::documentMode();
  fn _ZNK11QMainWindow12documentModeEv(qthis: *mut c_void) -> c_char;
  // proto:  void QMainWindow::setMenuWidget(QWidget * menubar);
  fn _ZN11QMainWindow13setMenuWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMainWindow::removeToolBar(QToolBar * toolbar);
  fn _ZN11QMainWindow13removeToolBarEP8QToolBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMainWindow::setCentralWidget(QWidget * widget);
  fn _ZN11QMainWindow16setCentralWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMainWindow::setMenuBar(QMenuBar * menubar);
  fn _ZN11QMainWindow10setMenuBarEP8QMenuBar(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
  fn _ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMainWindow)=1
pub struct QMainWindow {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMainWindow {
  pub fn inheritFrom(qthis: *mut c_void) -> QMainWindow {
    return QMainWindow{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMainWindow {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QMainWindow {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QStatusBar * QMainWindow::statusBar();
impl /*struct*/ QMainWindow {
  pub fn statusBar<RetType, T: QMainWindow_statusBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.statusBar(self);
    // return 1;
  }
}

pub trait QMainWindow_statusBar<RetType> {
  fn statusBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QStatusBar * QMainWindow::statusBar();
impl<'a> /*trait*/ QMainWindow_statusBar<QStatusBar> for () {
  fn statusBar(self , rsthis: & QMainWindow) -> QStatusBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow9statusBarEv()};
    let mut ret = unsafe {_ZNK11QMainWindow9statusBarEv(rsthis.qclsinst)};
    let mut ret1 = QStatusBar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMainWindow::setAnimated(bool enabled);
impl /*struct*/ QMainWindow {
  pub fn setAnimated<RetType, T: QMainWindow_setAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAnimated(self);
    // return 1;
  }
}

pub trait QMainWindow_setAnimated<RetType> {
  fn setAnimated(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setAnimated(bool enabled);
impl<'a> /*trait*/ QMainWindow_setAnimated<()> for (i8) {
  fn setAnimated(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setAnimatedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QMainWindow11setAnimatedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::setDockNestingEnabled(bool enabled);
impl /*struct*/ QMainWindow {
  pub fn setDockNestingEnabled<RetType, T: QMainWindow_setDockNestingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDockNestingEnabled(self);
    // return 1;
  }
}

pub trait QMainWindow_setDockNestingEnabled<RetType> {
  fn setDockNestingEnabled(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setDockNestingEnabled(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDockNestingEnabled<()> for (i8) {
  fn setDockNestingEnabled(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow21setDockNestingEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QMainWindow21setDockNestingEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::iconSizeChanged(const QSize & iconSize);
impl /*struct*/ QMainWindow {
  pub fn iconSizeChanged<RetType, T: QMainWindow_iconSizeChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSizeChanged(self);
    // return 1;
  }
}

pub trait QMainWindow_iconSizeChanged<RetType> {
  fn iconSizeChanged(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::iconSizeChanged(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_iconSizeChanged<()> for (&'a QSize) {
  fn iconSizeChanged(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15iconSizeChangedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow15iconSizeChangedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMainWindow::unifiedTitleAndToolBarOnMac();
impl /*struct*/ QMainWindow {
  pub fn unifiedTitleAndToolBarOnMac<RetType, T: QMainWindow_unifiedTitleAndToolBarOnMac<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unifiedTitleAndToolBarOnMac(self);
    // return 1;
  }
}

pub trait QMainWindow_unifiedTitleAndToolBarOnMac<RetType> {
  fn unifiedTitleAndToolBarOnMac(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::unifiedTitleAndToolBarOnMac();
impl<'a> /*trait*/ QMainWindow_unifiedTitleAndToolBarOnMac<i8> for () {
  fn unifiedTitleAndToolBarOnMac(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv()};
    let mut ret = unsafe {_ZNK11QMainWindow27unifiedTitleAndToolBarOnMacEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QWidget * QMainWindow::menuWidget();
impl /*struct*/ QMainWindow {
  pub fn menuWidget<RetType, T: QMainWindow_menuWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.menuWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_menuWidget<RetType> {
  fn menuWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QWidget * QMainWindow::menuWidget();
impl<'a> /*trait*/ QMainWindow_menuWidget<QWidget> for () {
  fn menuWidget(self , rsthis: & QMainWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10menuWidgetEv()};
    let mut ret = unsafe {_ZNK11QMainWindow10menuWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
impl /*struct*/ QMainWindow {
  pub fn tabifyDockWidget<RetType, T: QMainWindow_tabifyDockWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabifyDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_tabifyDockWidget<RetType> {
  fn tabifyDockWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::tabifyDockWidget(QDockWidget * first, QDockWidget * second);
impl<'a> /*trait*/ QMainWindow_tabifyDockWidget<()> for (&'a QDockWidget, &'a QDockWidget) {
  fn tabifyDockWidget(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16tabifyDockWidgetEP11QDockWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMainWindow::setDocumentMode(bool enabled);
impl /*struct*/ QMainWindow {
  pub fn setDocumentMode<RetType, T: QMainWindow_setDocumentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode(self);
    // return 1;
  }
}

pub trait QMainWindow_setDocumentMode<RetType> {
  fn setDocumentMode(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMainWindow_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15setDocumentModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QMainWindow15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QMainWindow::centralWidget();
impl /*struct*/ QMainWindow {
  pub fn centralWidget<RetType, T: QMainWindow_centralWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_centralWidget<RetType> {
  fn centralWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QWidget * QMainWindow::centralWidget();
impl<'a> /*trait*/ QMainWindow_centralWidget<QWidget> for () {
  fn centralWidget(self , rsthis: & QMainWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow13centralWidgetEv()};
    let mut ret = unsafe {_ZNK11QMainWindow13centralWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
impl /*struct*/ QMainWindow {
  pub fn removeDockWidget<RetType, T: QMainWindow_removeDockWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_removeDockWidget<RetType> {
  fn removeDockWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::removeDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_removeDockWidget<()> for (&'a QDockWidget) {
  fn removeDockWidget(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16removeDockWidgetEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16removeDockWidgetEP11QDockWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::QMainWindow(const QMainWindow & );
impl /*struct*/ QMainWindow {
  pub fn New<T: QMainWindow_New>(value: T) -> QMainWindow {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMainWindow_New {
  fn New(self) -> QMainWindow;
}

  // proto:  void QMainWindow::QMainWindow(const QMainWindow & );
impl<'a> /*trait*/ QMainWindow_New for (&'a QMainWindow) {
  fn New(self) -> QMainWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QMainWindowC1ERKS_(qthis, arg0)};
    let rsthis = QMainWindow{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QMainWindow::isAnimated();
impl /*struct*/ QMainWindow {
  pub fn isAnimated<RetType, T: QMainWindow_isAnimated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAnimated(self);
    // return 1;
  }
}

pub trait QMainWindow_isAnimated<RetType> {
  fn isAnimated(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::isAnimated();
impl<'a> /*trait*/ QMainWindow_isAnimated<i8> for () {
  fn isAnimated(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10isAnimatedEv()};
    let mut ret = unsafe {_ZNK11QMainWindow10isAnimatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QToolBar * QMainWindow::addToolBar(const QString & title);
impl /*struct*/ QMainWindow {
  pub fn addToolBar<RetType, T: QMainWindow_addToolBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_addToolBar<RetType> {
  fn addToolBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QToolBar * QMainWindow::addToolBar(const QString & title);
impl<'a> /*trait*/ QMainWindow_addToolBar<QToolBar> for (&'a QString) {
  fn addToolBar(self , rsthis: & QMainWindow) -> QToolBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10addToolBarERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QMainWindow10addToolBarERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QToolBar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMainWindow::setIconSize(const QSize & iconSize);
impl /*struct*/ QMainWindow {
  pub fn setIconSize<RetType, T: QMainWindow_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QMainWindow_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setIconSize(const QSize & iconSize);
impl<'a> /*trait*/ QMainWindow_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QMainWindow::saveState(int version);
impl /*struct*/ QMainWindow {
  pub fn saveState<RetType, T: QMainWindow_saveState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.saveState(self);
    // return 1;
  }
}

pub trait QMainWindow_saveState<RetType> {
  fn saveState(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QByteArray QMainWindow::saveState(int version);
impl<'a> /*trait*/ QMainWindow_saveState<QByteArray> for (i32) {
  fn saveState(self , rsthis: & QMainWindow) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow9saveStateEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QMainWindow9saveStateEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMainWindow::restoreState(const QByteArray & state, int version);
impl /*struct*/ QMainWindow {
  pub fn restoreState<RetType, T: QMainWindow_restoreState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restoreState(self);
    // return 1;
  }
}

pub trait QMainWindow_restoreState<RetType> {
  fn restoreState(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::restoreState(const QByteArray & state, int version);
impl<'a> /*trait*/ QMainWindow_restoreState<i8> for (&'a QByteArray, i32) {
  fn restoreState(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12restoreStateERK10QByteArrayi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN11QMainWindow12restoreStateERK10QByteArrayi(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
impl /*struct*/ QMainWindow {
  pub fn insertToolBar<RetType, T: QMainWindow_insertToolBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_insertToolBar<RetType> {
  fn insertToolBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::insertToolBar(QToolBar * before, QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_insertToolBar<()> for (&'a QToolBar, &'a QToolBar) {
  fn insertToolBar(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13insertToolBarEP8QToolBarS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13insertToolBarEP8QToolBarS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QMenu * QMainWindow::createPopupMenu();
impl /*struct*/ QMainWindow {
  pub fn createPopupMenu<RetType, T: QMainWindow_createPopupMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createPopupMenu(self);
    // return 1;
  }
}

pub trait QMainWindow_createPopupMenu<RetType> {
  fn createPopupMenu(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QMenu * QMainWindow::createPopupMenu();
impl<'a> /*trait*/ QMainWindow_createPopupMenu<QMenu> for () {
  fn createPopupMenu(self , rsthis: & QMainWindow) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow15createPopupMenuEv()};
    let mut ret = unsafe {_ZN11QMainWindow15createPopupMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
impl /*struct*/ QMainWindow {
  pub fn setUnifiedTitleAndToolBarOnMac<RetType, T: QMainWindow_setUnifiedTitleAndToolBarOnMac<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUnifiedTitleAndToolBarOnMac(self);
    // return 1;
  }
}

pub trait QMainWindow_setUnifiedTitleAndToolBarOnMac<RetType> {
  fn setUnifiedTitleAndToolBarOnMac(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setUnifiedTitleAndToolBarOnMac(bool set);
impl<'a> /*trait*/ QMainWindow_setUnifiedTitleAndToolBarOnMac<()> for (i8) {
  fn setUnifiedTitleAndToolBarOnMac(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QMainWindow30setUnifiedTitleAndToolBarOnMacEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::addToolBar(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_addToolBar<()> for (&'a QToolBar) {
  fn addToolBar(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10addToolBarEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow10addToolBarEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::removeToolBarBreak(QToolBar * before);
impl /*struct*/ QMainWindow {
  pub fn removeToolBarBreak<RetType, T: QMainWindow_removeToolBarBreak<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeToolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_removeToolBarBreak<RetType> {
  fn removeToolBarBreak(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::removeToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_removeToolBarBreak<()> for (&'a QToolBar) {
  fn removeToolBarBreak(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18removeToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow18removeToolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMainWindow::toolBarBreak(QToolBar * toolbar);
impl /*struct*/ QMainWindow {
  pub fn toolBarBreak<RetType, T: QMainWindow_toolBarBreak<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_toolBarBreak<RetType> {
  fn toolBarBreak(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::toolBarBreak(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_toolBarBreak<i8> for (&'a QToolBar) {
  fn toolBarBreak(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12toolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QMainWindow12toolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
impl /*struct*/ QMainWindow {
  pub fn restoreDockWidget<RetType, T: QMainWindow_restoreDockWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restoreDockWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_restoreDockWidget<RetType> {
  fn restoreDockWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::restoreDockWidget(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_restoreDockWidget<i8> for (&'a QDockWidget) {
  fn restoreDockWidget(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QMainWindow17restoreDockWidgetEP11QDockWidget(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMenuBar * QMainWindow::menuBar();
impl /*struct*/ QMainWindow {
  pub fn menuBar<RetType, T: QMainWindow_menuBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.menuBar(self);
    // return 1;
  }
}

pub trait QMainWindow_menuBar<RetType> {
  fn menuBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QMenuBar * QMainWindow::menuBar();
impl<'a> /*trait*/ QMainWindow_menuBar<QMenuBar> for () {
  fn menuBar(self , rsthis: & QMainWindow) -> QMenuBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow7menuBarEv()};
    let mut ret = unsafe {_ZNK11QMainWindow7menuBarEv(rsthis.qclsinst)};
    let mut ret1 = QMenuBar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMainWindow::setStatusBar(QStatusBar * statusbar);
impl /*struct*/ QMainWindow {
  pub fn setStatusBar<RetType, T: QMainWindow_setStatusBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusBar(self);
    // return 1;
  }
}

pub trait QMainWindow_setStatusBar<RetType> {
  fn setStatusBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setStatusBar(QStatusBar * statusbar);
impl<'a> /*trait*/ QMainWindow_setStatusBar<()> for (&'a QStatusBar) {
  fn setStatusBar(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow12setStatusBarEP10QStatusBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow12setStatusBarEP10QStatusBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::~QMainWindow();
impl /*struct*/ QMainWindow {
  pub fn Free<RetType, T: QMainWindow_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMainWindow_Free<RetType> {
  fn Free(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::~QMainWindow();
impl<'a> /*trait*/ QMainWindow_Free<()> for () {
  fn Free(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindowD0Ev()};
     unsafe {_ZN11QMainWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMainWindow::isSeparator(const QPoint & pos);
impl /*struct*/ QMainWindow {
  pub fn isSeparator<RetType, T: QMainWindow_isSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSeparator(self);
    // return 1;
  }
}

pub trait QMainWindow_isSeparator<RetType> {
  fn isSeparator(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::isSeparator(const QPoint & pos);
impl<'a> /*trait*/ QMainWindow_isSeparator<i8> for (&'a QPoint) {
  fn isSeparator(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow11isSeparatorERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QMainWindow11isSeparatorERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QMainWindow::iconSize();
impl /*struct*/ QMainWindow {
  pub fn iconSize<RetType, T: QMainWindow_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QMainWindow_iconSize<RetType> {
  fn iconSize(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QSize QMainWindow::iconSize();
impl<'a> /*trait*/ QMainWindow_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QMainWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow8iconSizeEv()};
    let mut ret = unsafe {_ZNK11QMainWindow8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMainWindow::metaObject();
impl /*struct*/ QMainWindow {
  pub fn metaObject<RetType, T: QMainWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMainWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  const QMetaObject * QMainWindow::metaObject();
impl<'a> /*trait*/ QMainWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow10metaObjectEv()};
     unsafe {_ZNK11QMainWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMainWindow::insertToolBarBreak(QToolBar * before);
impl /*struct*/ QMainWindow {
  pub fn insertToolBarBreak<RetType, T: QMainWindow_insertToolBarBreak<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertToolBarBreak(self);
    // return 1;
  }
}

pub trait QMainWindow_insertToolBarBreak<RetType> {
  fn insertToolBarBreak(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::insertToolBarBreak(QToolBar * before);
impl<'a> /*trait*/ QMainWindow_insertToolBarBreak<()> for (&'a QToolBar) {
  fn insertToolBarBreak(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow18insertToolBarBreakEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow18insertToolBarBreakEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QMainWindow::takeCentralWidget();
impl /*struct*/ QMainWindow {
  pub fn takeCentralWidget<RetType, T: QMainWindow_takeCentralWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.takeCentralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_takeCentralWidget<RetType> {
  fn takeCentralWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QWidget * QMainWindow::takeCentralWidget();
impl<'a> /*trait*/ QMainWindow_takeCentralWidget<QWidget> for () {
  fn takeCentralWidget(self , rsthis: & QMainWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow17takeCentralWidgetEv()};
    let mut ret = unsafe {_ZN11QMainWindow17takeCentralWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMainWindow::isDockNestingEnabled();
impl /*struct*/ QMainWindow {
  pub fn isDockNestingEnabled<RetType, T: QMainWindow_isDockNestingEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDockNestingEnabled(self);
    // return 1;
  }
}

pub trait QMainWindow_isDockNestingEnabled<RetType> {
  fn isDockNestingEnabled(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::isDockNestingEnabled();
impl<'a> /*trait*/ QMainWindow_isDockNestingEnabled<i8> for () {
  fn isDockNestingEnabled(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow20isDockNestingEnabledEv()};
    let mut ret = unsafe {_ZNK11QMainWindow20isDockNestingEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QMainWindow::documentMode();
impl /*struct*/ QMainWindow {
  pub fn documentMode<RetType, T: QMainWindow_documentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentMode(self);
    // return 1;
  }
}

pub trait QMainWindow_documentMode<RetType> {
  fn documentMode(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  bool QMainWindow::documentMode();
impl<'a> /*trait*/ QMainWindow_documentMode<i8> for () {
  fn documentMode(self , rsthis: & QMainWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow12documentModeEv()};
    let mut ret = unsafe {_ZNK11QMainWindow12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMainWindow::setMenuWidget(QWidget * menubar);
impl /*struct*/ QMainWindow {
  pub fn setMenuWidget<RetType, T: QMainWindow_setMenuWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMenuWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_setMenuWidget<RetType> {
  fn setMenuWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setMenuWidget(QWidget * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuWidget<()> for (&'a QWidget) {
  fn setMenuWidget(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13setMenuWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13setMenuWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::removeToolBar(QToolBar * toolbar);
impl /*struct*/ QMainWindow {
  pub fn removeToolBar<RetType, T: QMainWindow_removeToolBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeToolBar(self);
    // return 1;
  }
}

pub trait QMainWindow_removeToolBar<RetType> {
  fn removeToolBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::removeToolBar(QToolBar * toolbar);
impl<'a> /*trait*/ QMainWindow_removeToolBar<()> for (&'a QToolBar) {
  fn removeToolBar(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow13removeToolBarEP8QToolBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow13removeToolBarEP8QToolBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::setCentralWidget(QWidget * widget);
impl /*struct*/ QMainWindow {
  pub fn setCentralWidget<RetType, T: QMainWindow_setCentralWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCentralWidget(self);
    // return 1;
  }
}

pub trait QMainWindow_setCentralWidget<RetType> {
  fn setCentralWidget(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setCentralWidget(QWidget * widget);
impl<'a> /*trait*/ QMainWindow_setCentralWidget<()> for (&'a QWidget) {
  fn setCentralWidget(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow16setCentralWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow16setCentralWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMainWindow::setMenuBar(QMenuBar * menubar);
impl /*struct*/ QMainWindow {
  pub fn setMenuBar<RetType, T: QMainWindow_setMenuBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMenuBar(self);
    // return 1;
  }
}

pub trait QMainWindow_setMenuBar<RetType> {
  fn setMenuBar(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  void QMainWindow::setMenuBar(QMenuBar * menubar);
impl<'a> /*trait*/ QMainWindow_setMenuBar<()> for (&'a QMenuBar) {
  fn setMenuBar(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMainWindow10setMenuBarEP8QMenuBar()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QMainWindow10setMenuBarEP8QMenuBar(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
impl /*struct*/ QMainWindow {
  pub fn tabifiedDockWidgets<RetType, T: QMainWindow_tabifiedDockWidgets<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabifiedDockWidgets(self);
    // return 1;
  }
}

pub trait QMainWindow_tabifiedDockWidgets<RetType> {
  fn tabifiedDockWidgets(self , rsthis: & QMainWindow) -> RetType;
}

  // proto:  QList<QDockWidget *> QMainWindow::tabifiedDockWidgets(QDockWidget * dockwidget);
impl<'a> /*trait*/ QMainWindow_tabifiedDockWidgets<()> for (&'a QDockWidget) {
  fn tabifiedDockWidgets(self , rsthis: & QMainWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK11QMainWindow19tabifiedDockWidgetsEP11QDockWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

