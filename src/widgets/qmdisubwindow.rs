// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qmdisubwindow.h
// dst-file: /src/widgets/qmdisubwindow.rs
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
use super::super::core::qsize::QSize; // 771
use super::qmenu::QMenu; // 773
use super::qmdiarea::QMdiArea; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMdiSubWindow_Class_Size() -> c_int;
  // proto:  int QMdiSubWindow::keyboardSingleStep();
  fn _ZNK13QMdiSubWindow18keyboardSingleStepEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMdiSubWindow::keyboardPageStep();
  fn _ZNK13QMdiSubWindow16keyboardPageStepEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
  fn _ZNK13QMdiSubWindow10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QMdiSubWindow::sizeHint();
  fn _ZNK13QMdiSubWindow8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
  fn _ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
  fn _ZN13QMdiSubWindow13setSystemMenuEP5QMenu(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMdiSubWindow::~QMdiSubWindow();
  fn _ZN13QMdiSubWindowD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
  fn _ZN13QMdiSubWindow21setKeyboardSingleStepEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QWidget * QMdiSubWindow::widget();
  fn _ZNK13QMdiSubWindow6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiSubWindow::showShaded();
  fn _ZN13QMdiSubWindow10showShadedEv(qthis: u64 /* *mut c_void*/);
  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
  fn _ZNK13QMdiSubWindow22maximizedButtonsWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QMdiSubWindow::minimumSizeHint();
  fn _ZNK13QMdiSubWindow15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiSubWindow::showSystemMenu();
  fn _ZN13QMdiSubWindow14showSystemMenuEv(qthis: u64 /* *mut c_void*/);
  // proto:  QMenu * QMdiSubWindow::systemMenu();
  fn _ZNK13QMdiSubWindow10systemMenuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiSubWindow::QMdiSubWindow(const QMdiSubWindow & );
  fn _ZN13QMdiSubWindowC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
  fn _ZN13QMdiSubWindow9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QMdiSubWindow::isShaded();
  fn _ZNK13QMdiSubWindow8isShadedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
  fn _ZNK13QMdiSubWindow7mdiAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
  fn _ZN13QMdiSubWindow19setKeyboardPageStepEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QMdiSubWindow_SlotProxy_connect__ZN13QMdiSubWindow18windowStateChangedE6QFlagsIN2Qt11WindowStateEES3_(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QMdiSubWindow_SlotProxy_connect__ZN13QMdiSubWindow15aboutToActivateEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMdiSubWindow)=1
#[derive(Default)]
pub struct QMdiSubWindow {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _aboutToActivate: QMdiSubWindow_aboutToActivate_signal,
  pub _windowStateChanged: QMdiSubWindow_windowStateChanged_signal,
}

impl /*struct*/ QMdiSubWindow {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMdiSubWindow {
    return QMdiSubWindow{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMdiSubWindow {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QMdiSubWindow {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  int QMdiSubWindow::keyboardSingleStep();
impl /*struct*/ QMdiSubWindow {
  pub fn keyboardSingleStep<RetType, T: QMdiSubWindow_keyboardSingleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardSingleStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_keyboardSingleStep<RetType> {
  fn keyboardSingleStep(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  int QMdiSubWindow::keyboardSingleStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardSingleStep<i32> for () {
  fn keyboardSingleStep(self , rsthis: & QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow18keyboardSingleStepEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow18keyboardSingleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QMdiSubWindow::keyboardPageStep();
impl /*struct*/ QMdiSubWindow {
  pub fn keyboardPageStep<RetType, T: QMdiSubWindow_keyboardPageStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardPageStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_keyboardPageStep<RetType> {
  fn keyboardPageStep(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  int QMdiSubWindow::keyboardPageStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardPageStep<i32> for () {
  fn keyboardPageStep(self , rsthis: & QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow16keyboardPageStepEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow16keyboardPageStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
impl /*struct*/ QMdiSubWindow {
  pub fn metaObject<RetType, T: QMdiSubWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
impl<'a> /*trait*/ QMdiSubWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10metaObjectEv()};
     unsafe {_ZNK13QMdiSubWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QMdiSubWindow::sizeHint();
impl /*struct*/ QMdiSubWindow {
  pub fn sizeHint<RetType, T: QMdiSubWindow_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QSize QMdiSubWindow::sizeHint();
impl<'a> /*trait*/ QMdiSubWindow_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QMdiSubWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
impl /*struct*/ QMdiSubWindow {
  pub fn maximizedSystemMenuIconWidget<RetType, T: QMdiSubWindow_maximizedSystemMenuIconWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximizedSystemMenuIconWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_maximizedSystemMenuIconWidget<RetType> {
  fn maximizedSystemMenuIconWidget(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedSystemMenuIconWidget<QWidget> for () {
  fn maximizedSystemMenuIconWidget(self , rsthis: & QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
impl /*struct*/ QMdiSubWindow {
  pub fn setSystemMenu<RetType, T: QMdiSubWindow_setSystemMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSystemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setSystemMenu<RetType> {
  fn setSystemMenu(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
impl<'a> /*trait*/ QMdiSubWindow_setSystemMenu<()> for (&'a QMenu) {
  fn setSystemMenu(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow13setSystemMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QMdiSubWindow13setSystemMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::~QMdiSubWindow();
impl /*struct*/ QMdiSubWindow {
  pub fn free<RetType, T: QMdiSubWindow_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_free<RetType> {
  fn free(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::~QMdiSubWindow();
impl<'a> /*trait*/ QMdiSubWindow_free<()> for () {
  fn free(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowD2Ev()};
     unsafe {_ZN13QMdiSubWindowD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardSingleStep<RetType, T: QMdiSubWindow_setKeyboardSingleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardSingleStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardSingleStep<RetType> {
  fn setKeyboardSingleStep(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardSingleStep<()> for (i32) {
  fn setKeyboardSingleStep(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow21setKeyboardSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QMdiSubWindow21setKeyboardSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QMdiSubWindow::widget();
impl /*struct*/ QMdiSubWindow {
  pub fn widget<RetType, T: QMdiSubWindow_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_widget<RetType> {
  fn widget(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QWidget * QMdiSubWindow::widget();
impl<'a> /*trait*/ QMdiSubWindow_widget<QWidget> for () {
  fn widget(self , rsthis: & QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow6widgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::showShaded();
impl /*struct*/ QMdiSubWindow {
  pub fn showShaded<RetType, T: QMdiSubWindow_showShaded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showShaded(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_showShaded<RetType> {
  fn showShaded(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::showShaded();
impl<'a> /*trait*/ QMdiSubWindow_showShaded<()> for () {
  fn showShaded(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow10showShadedEv()};
     unsafe {_ZN13QMdiSubWindow10showShadedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
impl /*struct*/ QMdiSubWindow {
  pub fn maximizedButtonsWidget<RetType, T: QMdiSubWindow_maximizedButtonsWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximizedButtonsWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_maximizedButtonsWidget<RetType> {
  fn maximizedButtonsWidget(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedButtonsWidget<QWidget> for () {
  fn maximizedButtonsWidget(self , rsthis: & QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QMdiSubWindow::minimumSizeHint();
impl /*struct*/ QMdiSubWindow {
  pub fn minimumSizeHint<RetType, T: QMdiSubWindow_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QSize QMdiSubWindow::minimumSizeHint();
impl<'a> /*trait*/ QMdiSubWindow_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QMdiSubWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::showSystemMenu();
impl /*struct*/ QMdiSubWindow {
  pub fn showSystemMenu<RetType, T: QMdiSubWindow_showSystemMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showSystemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_showSystemMenu<RetType> {
  fn showSystemMenu(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::showSystemMenu();
impl<'a> /*trait*/ QMdiSubWindow_showSystemMenu<()> for () {
  fn showSystemMenu(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow14showSystemMenuEv()};
     unsafe {_ZN13QMdiSubWindow14showSystemMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMenu * QMdiSubWindow::systemMenu();
impl /*struct*/ QMdiSubWindow {
  pub fn systemMenu<RetType, T: QMdiSubWindow_systemMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.systemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_systemMenu<RetType> {
  fn systemMenu(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QMenu * QMdiSubWindow::systemMenu();
impl<'a> /*trait*/ QMdiSubWindow_systemMenu<QMenu> for () {
  fn systemMenu(self , rsthis: & QMdiSubWindow) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10systemMenuEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow10systemMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::QMdiSubWindow(const QMdiSubWindow & );
impl /*struct*/ QMdiSubWindow {
  pub fn new<T: QMdiSubWindow_new>(value: T) -> QMdiSubWindow {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiSubWindow_new {
  fn new(self) -> QMdiSubWindow;
}

  // proto:  void QMdiSubWindow::QMdiSubWindow(const QMdiSubWindow & );
impl<'a> /*trait*/ QMdiSubWindow_new for (&'a QMdiSubWindow) {
  fn new(self) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowC2ERKS_()};
    let ctysz: c_int = unsafe{QMdiSubWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QMdiSubWindowC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QMdiSubWindow{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
impl /*struct*/ QMdiSubWindow {
  pub fn setWidget<RetType, T: QMdiSubWindow_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setWidget<RetType> {
  fn setWidget(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
impl<'a> /*trait*/ QMdiSubWindow_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QMdiSubWindow9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMdiSubWindow::isShaded();
impl /*struct*/ QMdiSubWindow {
  pub fn isShaded<RetType, T: QMdiSubWindow_isShaded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isShaded(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_isShaded<RetType> {
  fn isShaded(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  bool QMdiSubWindow::isShaded();
impl<'a> /*trait*/ QMdiSubWindow_isShaded<i8> for () {
  fn isShaded(self , rsthis: & QMdiSubWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8isShadedEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow8isShadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
impl /*struct*/ QMdiSubWindow {
  pub fn mdiArea<RetType, T: QMdiSubWindow_mdiArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mdiArea(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_mdiArea<RetType> {
  fn mdiArea(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
impl<'a> /*trait*/ QMdiSubWindow_mdiArea<QMdiArea> for () {
  fn mdiArea(self , rsthis: & QMdiSubWindow) -> QMdiArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow7mdiAreaEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow7mdiAreaEv(rsthis.qclsinst)};
    let mut ret1 = QMdiArea::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardPageStep<RetType, T: QMdiSubWindow_setKeyboardPageStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardPageStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardPageStep<RetType> {
  fn setKeyboardPageStep(self , rsthis: & QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardPageStep<()> for (i32) {
  fn setKeyboardPageStep(self , rsthis: & QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow19setKeyboardPageStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QMdiSubWindow19setKeyboardPageStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QMdiSubWindow_aboutToActivate
pub struct QMdiSubWindow_aboutToActivate_signal{poi:u64}
impl /* struct */ QMdiSubWindow {
  pub fn aboutToActivate(&self) -> QMdiSubWindow_aboutToActivate_signal {
     return QMdiSubWindow_aboutToActivate_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMdiSubWindow_aboutToActivate_signal {
  pub fn connect<T: QMdiSubWindow_aboutToActivate_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMdiSubWindow_aboutToActivate_signal_connect {
  fn connect(self, sigthis: QMdiSubWindow_aboutToActivate_signal);
}

#[derive(Default)] // for QMdiSubWindow_windowStateChanged
pub struct QMdiSubWindow_windowStateChanged_signal{poi:u64}
impl /* struct */ QMdiSubWindow {
  pub fn windowStateChanged(&self) -> QMdiSubWindow_windowStateChanged_signal {
     return QMdiSubWindow_windowStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMdiSubWindow_windowStateChanged_signal {
  pub fn connect<T: QMdiSubWindow_windowStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMdiSubWindow_windowStateChanged_signal_connect {
  fn connect(self, sigthis: QMdiSubWindow_windowStateChanged_signal);
}

// windowStateChanged(Qt::WindowStates, Qt::WindowStates)
extern fn QMdiSubWindow_windowStateChanged_signal_connect_cb_0(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QMdiSubWindow_windowStateChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QMdiSubWindow_windowStateChanged_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QMdiSubWindow_windowStateChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMdiSubWindow_windowStateChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMdiSubWindow_SlotProxy_connect__ZN13QMdiSubWindow18windowStateChangedE6QFlagsIN2Qt11WindowStateEES3_(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMdiSubWindow_windowStateChanged_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QMdiSubWindow_windowStateChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMdiSubWindow_windowStateChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMdiSubWindow_SlotProxy_connect__ZN13QMdiSubWindow18windowStateChangedE6QFlagsIN2Qt11WindowStateEES3_(arg0, arg1, arg2)};
  }
}
// aboutToActivate()
extern fn QMdiSubWindow_aboutToActivate_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QMdiSubWindow_aboutToActivate_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QMdiSubWindow_aboutToActivate_signal_connect for fn() {
  fn connect(self, sigthis: QMdiSubWindow_aboutToActivate_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMdiSubWindow_aboutToActivate_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMdiSubWindow_SlotProxy_connect__ZN13QMdiSubWindow15aboutToActivateEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMdiSubWindow_aboutToActivate_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QMdiSubWindow_aboutToActivate_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMdiSubWindow_aboutToActivate_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMdiSubWindow_SlotProxy_connect__ZN13QMdiSubWindow15aboutToActivateEv(arg0, arg1, arg2)};
  }
}
// <= body block end

