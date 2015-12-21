// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::qmenu::QMenu; // 773
use super::qmdiarea::QMdiArea; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int QMdiSubWindow::keyboardSingleStep();
  fn _ZNK13QMdiSubWindow18keyboardSingleStepEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMdiSubWindow::keyboardPageStep();
  fn _ZNK13QMdiSubWindow16keyboardPageStepEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
  fn _ZNK13QMdiSubWindow10metaObjectEv(qthis: *mut c_void);
  // proto:  QSize QMdiSubWindow::sizeHint();
  fn _ZNK13QMdiSubWindow8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
  fn _ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
  fn _ZN13QMdiSubWindow13setSystemMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMdiSubWindow::~QMdiSubWindow();
  fn _ZN13QMdiSubWindowD0Ev(qthis: *mut c_void);
  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
  fn _ZN13QMdiSubWindow21setKeyboardSingleStepEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QWidget * QMdiSubWindow::widget();
  fn _ZNK13QMdiSubWindow6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::showShaded();
  fn _ZN13QMdiSubWindow10showShadedEv(qthis: *mut c_void);
  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
  fn _ZNK13QMdiSubWindow22maximizedButtonsWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QMdiSubWindow::minimumSizeHint();
  fn _ZNK13QMdiSubWindow15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::aboutToActivate();
  fn _ZN13QMdiSubWindow15aboutToActivateEv(qthis: *mut c_void);
  // proto:  void QMdiSubWindow::showSystemMenu();
  fn _ZN13QMdiSubWindow14showSystemMenuEv(qthis: *mut c_void);
  // proto:  QMenu * QMdiSubWindow::systemMenu();
  fn _ZNK13QMdiSubWindow10systemMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::QMdiSubWindow(const QMdiSubWindow & );
  fn _ZN13QMdiSubWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
  fn _ZN13QMdiSubWindow9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QMdiSubWindow::isShaded();
  fn _ZNK13QMdiSubWindow8isShadedEv(qthis: *mut c_void) -> c_char;
  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
  fn _ZNK13QMdiSubWindow7mdiAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
  fn _ZN13QMdiSubWindow19setKeyboardPageStepEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QMdiSubWindow)=1
pub struct QMdiSubWindow {
  pub qclsinst: *mut c_void,
}

  // proto:  int QMdiSubWindow::keyboardSingleStep();
impl /*struct*/ QMdiSubWindow {
  pub fn keyboardSingleStep<RetType, T: QMdiSubWindow_keyboardSingleStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keyboardSingleStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_keyboardSingleStep<RetType> {
  fn keyboardSingleStep(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  int QMdiSubWindow::keyboardSingleStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardSingleStep<i32> for () {
  fn keyboardSingleStep(self , rsthis: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow18keyboardSingleStepEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow18keyboardSingleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QMdiSubWindow::keyboardPageStep();
impl /*struct*/ QMdiSubWindow {
  pub fn keyboardPageStep<RetType, T: QMdiSubWindow_keyboardPageStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keyboardPageStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_keyboardPageStep<RetType> {
  fn keyboardPageStep(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  int QMdiSubWindow::keyboardPageStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardPageStep<i32> for () {
  fn keyboardPageStep(self , rsthis: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow16keyboardPageStepEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow16keyboardPageStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
impl /*struct*/ QMdiSubWindow {
  pub fn metaObject<RetType, T: QMdiSubWindow_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
impl<'a> /*trait*/ QMdiSubWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10metaObjectEv()};
     unsafe {_ZNK13QMdiSubWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QMdiSubWindow::sizeHint();
impl /*struct*/ QMdiSubWindow {
  pub fn sizeHint<RetType, T: QMdiSubWindow_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QSize QMdiSubWindow::sizeHint();
impl<'a> /*trait*/ QMdiSubWindow_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QMdiSubWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
impl /*struct*/ QMdiSubWindow {
  pub fn maximizedSystemMenuIconWidget<RetType, T: QMdiSubWindow_maximizedSystemMenuIconWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximizedSystemMenuIconWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_maximizedSystemMenuIconWidget<RetType> {
  fn maximizedSystemMenuIconWidget(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedSystemMenuIconWidget<QWidget> for () {
  fn maximizedSystemMenuIconWidget(self , rsthis: &mut QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
impl /*struct*/ QMdiSubWindow {
  pub fn setSystemMenu<RetType, T: QMdiSubWindow_setSystemMenu<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSystemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setSystemMenu<RetType> {
  fn setSystemMenu(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
impl<'a> /*trait*/ QMdiSubWindow_setSystemMenu<()> for (QMenu) {
  fn setSystemMenu(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow13setSystemMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QMdiSubWindow13setSystemMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::~QMdiSubWindow();
impl /*struct*/ QMdiSubWindow {
  pub fn FreeQMdiSubWindow<RetType, T: QMdiSubWindow_FreeQMdiSubWindow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMdiSubWindow(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_FreeQMdiSubWindow<RetType> {
  fn FreeQMdiSubWindow(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::~QMdiSubWindow();
impl<'a> /*trait*/ QMdiSubWindow_FreeQMdiSubWindow<()> for () {
  fn FreeQMdiSubWindow(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowD0Ev()};
     unsafe {_ZN13QMdiSubWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardSingleStep<RetType, T: QMdiSubWindow_setKeyboardSingleStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardSingleStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardSingleStep<RetType> {
  fn setKeyboardSingleStep(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardSingleStep<()> for (i32) {
  fn setKeyboardSingleStep(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow21setKeyboardSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QMdiSubWindow21setKeyboardSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QMdiSubWindow::widget();
impl /*struct*/ QMdiSubWindow {
  pub fn widget<RetType, T: QMdiSubWindow_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_widget<RetType> {
  fn widget(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QWidget * QMdiSubWindow::widget();
impl<'a> /*trait*/ QMdiSubWindow_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow6widgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::showShaded();
impl /*struct*/ QMdiSubWindow {
  pub fn showShaded<RetType, T: QMdiSubWindow_showShaded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showShaded(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_showShaded<RetType> {
  fn showShaded(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::showShaded();
impl<'a> /*trait*/ QMdiSubWindow_showShaded<()> for () {
  fn showShaded(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow10showShadedEv()};
     unsafe {_ZN13QMdiSubWindow10showShadedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
impl /*struct*/ QMdiSubWindow {
  pub fn maximizedButtonsWidget<RetType, T: QMdiSubWindow_maximizedButtonsWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximizedButtonsWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_maximizedButtonsWidget<RetType> {
  fn maximizedButtonsWidget(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedButtonsWidget<QWidget> for () {
  fn maximizedButtonsWidget(self , rsthis: &mut QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QMdiSubWindow::minimumSizeHint();
impl /*struct*/ QMdiSubWindow {
  pub fn minimumSizeHint<RetType, T: QMdiSubWindow_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QSize QMdiSubWindow::minimumSizeHint();
impl<'a> /*trait*/ QMdiSubWindow_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QMdiSubWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::aboutToActivate();
impl /*struct*/ QMdiSubWindow {
  pub fn aboutToActivate<RetType, T: QMdiSubWindow_aboutToActivate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.aboutToActivate(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_aboutToActivate<RetType> {
  fn aboutToActivate(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::aboutToActivate();
impl<'a> /*trait*/ QMdiSubWindow_aboutToActivate<()> for () {
  fn aboutToActivate(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow15aboutToActivateEv()};
     unsafe {_ZN13QMdiSubWindow15aboutToActivateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::showSystemMenu();
impl /*struct*/ QMdiSubWindow {
  pub fn showSystemMenu<RetType, T: QMdiSubWindow_showSystemMenu<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.showSystemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_showSystemMenu<RetType> {
  fn showSystemMenu(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::showSystemMenu();
impl<'a> /*trait*/ QMdiSubWindow_showSystemMenu<()> for () {
  fn showSystemMenu(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow14showSystemMenuEv()};
     unsafe {_ZN13QMdiSubWindow14showSystemMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMenu * QMdiSubWindow::systemMenu();
impl /*struct*/ QMdiSubWindow {
  pub fn systemMenu<RetType, T: QMdiSubWindow_systemMenu<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.systemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_systemMenu<RetType> {
  fn systemMenu(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QMenu * QMdiSubWindow::systemMenu();
impl<'a> /*trait*/ QMdiSubWindow_systemMenu<QMenu> for () {
  fn systemMenu(self , rsthis: &mut QMdiSubWindow) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10systemMenuEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow10systemMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::QMdiSubWindow(const QMdiSubWindow & );
impl /*struct*/ QMdiSubWindow {
  pub fn NewQMdiSubWindow<T: QMdiSubWindow_NewQMdiSubWindow>(value: T) -> QMdiSubWindow {
    let rsthis = value.NewQMdiSubWindow();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiSubWindow_NewQMdiSubWindow {
  fn NewQMdiSubWindow(self) -> QMdiSubWindow;
}

  // proto:  void QMdiSubWindow::QMdiSubWindow(const QMdiSubWindow & );
impl<'a> /*trait*/ QMdiSubWindow_NewQMdiSubWindow for (QMdiSubWindow) {
  fn NewQMdiSubWindow(self) -> QMdiSubWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QMdiSubWindowC1ERKS_(qthis, arg0)};
    let rsthis = QMdiSubWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
impl /*struct*/ QMdiSubWindow {
  pub fn setWidget<RetType, T: QMdiSubWindow_setWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setWidget<RetType> {
  fn setWidget(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
impl<'a> /*trait*/ QMdiSubWindow_setWidget<()> for (QWidget) {
  fn setWidget(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QMdiSubWindow9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMdiSubWindow::isShaded();
impl /*struct*/ QMdiSubWindow {
  pub fn isShaded<RetType, T: QMdiSubWindow_isShaded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isShaded(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_isShaded<RetType> {
  fn isShaded(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  bool QMdiSubWindow::isShaded();
impl<'a> /*trait*/ QMdiSubWindow_isShaded<i8> for () {
  fn isShaded(self , rsthis: &mut QMdiSubWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8isShadedEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow8isShadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
impl /*struct*/ QMdiSubWindow {
  pub fn mdiArea<RetType, T: QMdiSubWindow_mdiArea<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mdiArea(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_mdiArea<RetType> {
  fn mdiArea(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
impl<'a> /*trait*/ QMdiSubWindow_mdiArea<QMdiArea> for () {
  fn mdiArea(self , rsthis: &mut QMdiSubWindow) -> QMdiArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow7mdiAreaEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow7mdiAreaEv(rsthis.qclsinst)};
    let mut ret1 = QMdiArea{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardPageStep<RetType, T: QMdiSubWindow_setKeyboardPageStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardPageStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardPageStep<RetType> {
  fn setKeyboardPageStep(self , rsthis: &mut QMdiSubWindow) -> RetType;
}

  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardPageStep<()> for (i32) {
  fn setKeyboardPageStep(self , rsthis: &mut QMdiSubWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow19setKeyboardPageStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QMdiSubWindow19setKeyboardPageStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

