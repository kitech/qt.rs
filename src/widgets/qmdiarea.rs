// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtWidgets/qmdiarea.h
// dst-file: /src/widgets/qmdiarea.rs
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
use super::qabstractscrollarea::QAbstractScrollArea; // 773
use std::ops::Deref;
use super::super::gui::qbrush::QBrush; // 771
use super::qwidget::QWidget; // 773
use super::qmdisubwindow::QMdiSubWindow; // 773
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  void QMdiArea::activateNextSubWindow();
  fn _ZN8QMdiArea21activateNextSubWindowEv(qthis: *mut c_void);
  // proto:  void QMdiArea::setBackground(const QBrush & background);
  fn _ZN8QMdiArea13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMdiArea::~QMdiArea();
  fn _ZN8QMdiAreaD0Ev(qthis: *mut c_void);
  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
  fn _ZN8QMdiArea15removeSubWindowEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMdiArea::setTabsClosable(bool closable);
  fn _ZN8QMdiArea15setTabsClosableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
  fn _ZNK8QMdiArea16currentSubWindowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMdiArea::tabsMovable();
  fn _ZNK8QMdiArea11tabsMovableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QMdiArea::activatePreviousSubWindow();
  fn _ZN8QMdiArea25activatePreviousSubWindowEv(qthis: *mut c_void);
  // proto:  void QMdiArea::setDocumentMode(bool enabled);
  fn _ZN8QMdiArea15setDocumentModeEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QMdiArea::documentMode();
  fn _ZNK8QMdiArea12documentModeEv(qthis: *mut c_void) -> c_char;
  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
  fn _ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
  fn _ZNK8QMdiArea15activeSubWindowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiArea::setTabsMovable(bool movable);
  fn _ZN8QMdiArea14setTabsMovableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QMdiArea::metaObject();
  fn _ZNK8QMdiArea10metaObjectEv(qthis: *mut c_void);
  // proto:  void QMdiArea::QMdiArea(QWidget * parent);
  fn _ZN8QMdiAreaC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QMdiArea::sizeHint();
  fn _ZNK8QMdiArea8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiArea::closeAllSubWindows();
  fn _ZN8QMdiArea18closeAllSubWindowsEv(qthis: *mut c_void);
  // proto:  void QMdiArea::QMdiArea(const QMdiArea & );
  fn _ZN8QMdiAreaC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMdiArea::subWindowActivated(QMdiSubWindow * );
  fn _ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMdiArea::cascadeSubWindows();
  fn _ZN8QMdiArea17cascadeSubWindowsEv(qthis: *mut c_void);
  // proto:  void QMdiArea::closeActiveSubWindow();
  fn _ZN8QMdiArea20closeActiveSubWindowEv(qthis: *mut c_void);
  // proto:  QBrush QMdiArea::background();
  fn _ZNK8QMdiArea10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiArea::tileSubWindows();
  fn _ZN8QMdiArea14tileSubWindowsEv(qthis: *mut c_void);
  // proto:  bool QMdiArea::tabsClosable();
  fn _ZNK8QMdiArea12tabsClosableEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QMdiArea::minimumSizeHint();
  fn _ZNK8QMdiArea15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QMdiArea)=1
pub struct QMdiArea {
  qbase: QAbstractScrollArea,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMdiArea {
  pub fn inheritFrom(qthis: *mut c_void) -> QMdiArea {
    return QMdiArea{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QMdiArea {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QMdiArea {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  void QMdiArea::activateNextSubWindow();
impl /*struct*/ QMdiArea {
  pub fn activateNextSubWindow<RetType, T: QMdiArea_activateNextSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activateNextSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activateNextSubWindow<RetType> {
  fn activateNextSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::activateNextSubWindow();
impl<'a> /*trait*/ QMdiArea_activateNextSubWindow<()> for () {
  fn activateNextSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea21activateNextSubWindowEv()};
     unsafe {_ZN8QMdiArea21activateNextSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::setBackground(const QBrush & background);
impl /*struct*/ QMdiArea {
  pub fn setBackground<RetType, T: QMdiArea_setBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QMdiArea_setBackground<RetType> {
  fn setBackground(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setBackground(const QBrush & background);
impl<'a> /*trait*/ QMdiArea_setBackground<()> for (&'a QBrush) {
  fn setBackground(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiArea::~QMdiArea();
impl /*struct*/ QMdiArea {
  pub fn Free<RetType, T: QMdiArea_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QMdiArea_Free<RetType> {
  fn Free(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::~QMdiArea();
impl<'a> /*trait*/ QMdiArea_Free<()> for () {
  fn Free(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaD0Ev()};
     unsafe {_ZN8QMdiAreaD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
impl /*struct*/ QMdiArea {
  pub fn removeSubWindow<RetType, T: QMdiArea_removeSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_removeSubWindow<RetType> {
  fn removeSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
impl<'a> /*trait*/ QMdiArea_removeSubWindow<()> for (&'a QWidget) {
  fn removeSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15removeSubWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea15removeSubWindowEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiArea::setTabsClosable(bool closable);
impl /*struct*/ QMdiArea {
  pub fn setTabsClosable<RetType, T: QMdiArea_setTabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable(self);
    // return 1;
  }
}

pub trait QMdiArea_setTabsClosable<RetType> {
  fn setTabsClosable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setTabsClosable(bool closable);
impl<'a> /*trait*/ QMdiArea_setTabsClosable<()> for (i8) {
  fn setTabsClosable(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15setTabsClosableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QMdiArea15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
impl /*struct*/ QMdiArea {
  pub fn currentSubWindow<RetType, T: QMdiArea_currentSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_currentSubWindow<RetType> {
  fn currentSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
impl<'a> /*trait*/ QMdiArea_currentSubWindow<QMdiSubWindow> for () {
  fn currentSubWindow(self , rsthis: & QMdiArea) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea16currentSubWindowEv()};
    let mut ret = unsafe {_ZNK8QMdiArea16currentSubWindowEv(rsthis.qclsinst)};
    let mut ret1 = QMdiSubWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMdiArea::tabsMovable();
impl /*struct*/ QMdiArea {
  pub fn tabsMovable<RetType, T: QMdiArea_tabsMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabsMovable(self);
    // return 1;
  }
}

pub trait QMdiArea_tabsMovable<RetType> {
  fn tabsMovable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  bool QMdiArea::tabsMovable();
impl<'a> /*trait*/ QMdiArea_tabsMovable<i8> for () {
  fn tabsMovable(self , rsthis: & QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea11tabsMovableEv()};
    let mut ret = unsafe {_ZNK8QMdiArea11tabsMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMdiArea::activatePreviousSubWindow();
impl /*struct*/ QMdiArea {
  pub fn activatePreviousSubWindow<RetType, T: QMdiArea_activatePreviousSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activatePreviousSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activatePreviousSubWindow<RetType> {
  fn activatePreviousSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::activatePreviousSubWindow();
impl<'a> /*trait*/ QMdiArea_activatePreviousSubWindow<()> for () {
  fn activatePreviousSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea25activatePreviousSubWindowEv()};
     unsafe {_ZN8QMdiArea25activatePreviousSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::setDocumentMode(bool enabled);
impl /*struct*/ QMdiArea {
  pub fn setDocumentMode<RetType, T: QMdiArea_setDocumentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode(self);
    // return 1;
  }
}

pub trait QMdiArea_setDocumentMode<RetType> {
  fn setDocumentMode(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMdiArea_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15setDocumentModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QMdiArea15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMdiArea::documentMode();
impl /*struct*/ QMdiArea {
  pub fn documentMode<RetType, T: QMdiArea_documentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentMode(self);
    // return 1;
  }
}

pub trait QMdiArea_documentMode<RetType> {
  fn documentMode(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  bool QMdiArea::documentMode();
impl<'a> /*trait*/ QMdiArea_documentMode<i8> for () {
  fn documentMode(self , rsthis: & QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea12documentModeEv()};
    let mut ret = unsafe {_ZNK8QMdiArea12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
impl /*struct*/ QMdiArea {
  pub fn setActiveSubWindow<RetType, T: QMdiArea_setActiveSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActiveSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_setActiveSubWindow<RetType> {
  fn setActiveSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
impl<'a> /*trait*/ QMdiArea_setActiveSubWindow<()> for (&'a QMdiSubWindow) {
  fn setActiveSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
impl /*struct*/ QMdiArea {
  pub fn activeSubWindow<RetType, T: QMdiArea_activeSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activeSubWindow<RetType> {
  fn activeSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
impl<'a> /*trait*/ QMdiArea_activeSubWindow<QMdiSubWindow> for () {
  fn activeSubWindow(self , rsthis: & QMdiArea) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea15activeSubWindowEv()};
    let mut ret = unsafe {_ZNK8QMdiArea15activeSubWindowEv(rsthis.qclsinst)};
    let mut ret1 = QMdiSubWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::setTabsMovable(bool movable);
impl /*struct*/ QMdiArea {
  pub fn setTabsMovable<RetType, T: QMdiArea_setTabsMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabsMovable(self);
    // return 1;
  }
}

pub trait QMdiArea_setTabsMovable<RetType> {
  fn setTabsMovable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setTabsMovable(bool movable);
impl<'a> /*trait*/ QMdiArea_setTabsMovable<()> for (i8) {
  fn setTabsMovable(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea14setTabsMovableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QMdiArea14setTabsMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QMdiArea::metaObject();
impl /*struct*/ QMdiArea {
  pub fn metaObject<RetType, T: QMdiArea_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMdiArea_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  const QMetaObject * QMdiArea::metaObject();
impl<'a> /*trait*/ QMdiArea_metaObject<()> for () {
  fn metaObject(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea10metaObjectEv()};
     unsafe {_ZNK8QMdiArea10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::QMdiArea(QWidget * parent);
impl /*struct*/ QMdiArea {
  pub fn New<T: QMdiArea_New>(value: T) -> QMdiArea {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiArea_New {
  fn New(self) -> QMdiArea;
}

  // proto:  void QMdiArea::QMdiArea(QWidget * parent);
impl<'a> /*trait*/ QMdiArea_New for (&'a QWidget) {
  fn New(self) -> QMdiArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMdiAreaC1EP7QWidget(qthis, arg0)};
    let rsthis = QMdiArea{/**/qbase: QAbstractScrollArea::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QMdiArea::sizeHint();
impl /*struct*/ QMdiArea {
  pub fn sizeHint<RetType, T: QMdiArea_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QMdiArea_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QSize QMdiArea::sizeHint();
impl<'a> /*trait*/ QMdiArea_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QMdiArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea8sizeHintEv()};
    let mut ret = unsafe {_ZNK8QMdiArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::closeAllSubWindows();
impl /*struct*/ QMdiArea {
  pub fn closeAllSubWindows<RetType, T: QMdiArea_closeAllSubWindows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closeAllSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_closeAllSubWindows<RetType> {
  fn closeAllSubWindows(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::closeAllSubWindows();
impl<'a> /*trait*/ QMdiArea_closeAllSubWindows<()> for () {
  fn closeAllSubWindows(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18closeAllSubWindowsEv()};
     unsafe {_ZN8QMdiArea18closeAllSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::QMdiArea(const QMdiArea & );
impl<'a> /*trait*/ QMdiArea_New for (&'a QMdiArea) {
  fn New(self) -> QMdiArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMdiAreaC1ERKS_(qthis, arg0)};
    let rsthis = QMdiArea{/**/qbase: QAbstractScrollArea::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMdiArea::subWindowActivated(QMdiSubWindow * );
impl /*struct*/ QMdiArea {
  pub fn subWindowActivated<RetType, T: QMdiArea_subWindowActivated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subWindowActivated(self);
    // return 1;
  }
}

pub trait QMdiArea_subWindowActivated<RetType> {
  fn subWindowActivated(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::subWindowActivated(QMdiSubWindow * );
impl<'a> /*trait*/ QMdiArea_subWindowActivated<()> for (&'a QMdiSubWindow) {
  fn subWindowActivated(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiArea::cascadeSubWindows();
impl /*struct*/ QMdiArea {
  pub fn cascadeSubWindows<RetType, T: QMdiArea_cascadeSubWindows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cascadeSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_cascadeSubWindows<RetType> {
  fn cascadeSubWindows(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::cascadeSubWindows();
impl<'a> /*trait*/ QMdiArea_cascadeSubWindows<()> for () {
  fn cascadeSubWindows(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea17cascadeSubWindowsEv()};
     unsafe {_ZN8QMdiArea17cascadeSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::closeActiveSubWindow();
impl /*struct*/ QMdiArea {
  pub fn closeActiveSubWindow<RetType, T: QMdiArea_closeActiveSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closeActiveSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_closeActiveSubWindow<RetType> {
  fn closeActiveSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::closeActiveSubWindow();
impl<'a> /*trait*/ QMdiArea_closeActiveSubWindow<()> for () {
  fn closeActiveSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea20closeActiveSubWindowEv()};
     unsafe {_ZN8QMdiArea20closeActiveSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QBrush QMdiArea::background();
impl /*struct*/ QMdiArea {
  pub fn background<RetType, T: QMdiArea_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QMdiArea_background<RetType> {
  fn background(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QBrush QMdiArea::background();
impl<'a> /*trait*/ QMdiArea_background<QBrush> for () {
  fn background(self , rsthis: & QMdiArea) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea10backgroundEv()};
    let mut ret = unsafe {_ZNK8QMdiArea10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::tileSubWindows();
impl /*struct*/ QMdiArea {
  pub fn tileSubWindows<RetType, T: QMdiArea_tileSubWindows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tileSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_tileSubWindows<RetType> {
  fn tileSubWindows(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::tileSubWindows();
impl<'a> /*trait*/ QMdiArea_tileSubWindows<()> for () {
  fn tileSubWindows(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea14tileSubWindowsEv()};
     unsafe {_ZN8QMdiArea14tileSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMdiArea::tabsClosable();
impl /*struct*/ QMdiArea {
  pub fn tabsClosable<RetType, T: QMdiArea_tabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable(self);
    // return 1;
  }
}

pub trait QMdiArea_tabsClosable<RetType> {
  fn tabsClosable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  bool QMdiArea::tabsClosable();
impl<'a> /*trait*/ QMdiArea_tabsClosable<i8> for () {
  fn tabsClosable(self , rsthis: & QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea12tabsClosableEv()};
    let mut ret = unsafe {_ZNK8QMdiArea12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QMdiArea::minimumSizeHint();
impl /*struct*/ QMdiArea {
  pub fn minimumSizeHint<RetType, T: QMdiArea_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMdiArea_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QSize QMdiArea::minimumSizeHint();
impl<'a> /*trait*/ QMdiArea_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QMdiArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK8QMdiArea15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

