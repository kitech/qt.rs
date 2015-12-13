// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmenu::QMenu;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QMdiSubWindow::keyboardSingleStep();
  fn _ZNK13QMdiSubWindow18keyboardSingleStepEv() -> i32;
  // proto: int QMdiSubWindow::keyboardPageStep();
  fn _ZNK13QMdiSubWindow16keyboardPageStepEv() -> i32;
  // proto: const QMetaObject * QMdiSubWindow::metaObject();
  fn _ZNK13QMdiSubWindow10metaObjectEv() -> i32;
  // proto: QSize QMdiSubWindow::sizeHint();
  fn _ZNK13QMdiSubWindow8sizeHintEv() -> i32;
  // proto: QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
  fn _ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv() -> i32;
  // proto: void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
  fn _ZN13QMdiSubWindow13setSystemMenuEP5QMenu(arg0: *mut c_void) -> i32;
  // proto: void QMdiSubWindow::FreeQMdiSubWindow();
  fn _ZN13QMdiSubWindowD0Ev() -> i32;
  // proto: void QMdiSubWindow::setKeyboardSingleStep(int step);
  fn _ZN13QMdiSubWindow21setKeyboardSingleStepEi(arg0: c_int) -> i32;
  // proto: QWidget * QMdiSubWindow::widget();
  fn _ZNK13QMdiSubWindow6widgetEv() -> i32;
  // proto: void QMdiSubWindow::showShaded();
  fn _ZN13QMdiSubWindow10showShadedEv() -> i32;
  // proto: QWidget * QMdiSubWindow::maximizedButtonsWidget();
  fn _ZNK13QMdiSubWindow22maximizedButtonsWidgetEv() -> i32;
  // proto: QSize QMdiSubWindow::minimumSizeHint();
  fn _ZNK13QMdiSubWindow15minimumSizeHintEv() -> i32;
  // proto: void QMdiSubWindow::aboutToActivate();
  fn _ZN13QMdiSubWindow15aboutToActivateEv() -> i32;
  // proto: void QMdiSubWindow::showSystemMenu();
  fn _ZN13QMdiSubWindow14showSystemMenuEv() -> i32;
  // proto: QMenu * QMdiSubWindow::systemMenu();
  fn _ZNK13QMdiSubWindow10systemMenuEv() -> i32;
  // proto: void QMdiSubWindow::NewQMdiSubWindow(const QMdiSubWindow & );
  fn _ZN13QMdiSubWindowC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QMdiSubWindow::setWidget(QWidget * widget);
  fn _ZN13QMdiSubWindow9setWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: bool QMdiSubWindow::isShaded();
  fn _ZNK13QMdiSubWindow8isShadedEv() -> i32;
  // proto: QMdiArea * QMdiSubWindow::mdiArea();
  fn _ZNK13QMdiSubWindow7mdiAreaEv() -> i32;
  // proto: void QMdiSubWindow::setKeyboardPageStep(int step);
  fn _ZN13QMdiSubWindow19setKeyboardPageStepEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QMdiSubWindow)=1
pub struct QMdiSubWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMdiSubWindow {
  pub fn keyboardSingleStep<T: QMdiSubWindow_keyboardSingleStep>(&mut self, value: T) -> i32 {
    value.keyboardSingleStep(self);
    return 1;
  }
}

pub trait QMdiSubWindow_keyboardSingleStep {
  fn keyboardSingleStep(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: int QMdiSubWindow::keyboardSingleStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardSingleStep for () {
  fn keyboardSingleStep(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow18keyboardSingleStepEv()};
    unsafe {_ZNK13QMdiSubWindow18keyboardSingleStepEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn keyboardPageStep<T: QMdiSubWindow_keyboardPageStep>(&mut self, value: T) -> i32 {
    value.keyboardPageStep(self);
    return 1;
  }
}

pub trait QMdiSubWindow_keyboardPageStep {
  fn keyboardPageStep(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: int QMdiSubWindow::keyboardPageStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardPageStep for () {
  fn keyboardPageStep(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow16keyboardPageStepEv()};
    unsafe {_ZNK13QMdiSubWindow16keyboardPageStepEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn metaObject<T: QMdiSubWindow_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QMdiSubWindow_metaObject {
  fn metaObject(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: const QMetaObject * QMdiSubWindow::metaObject();
impl<'a> /*trait*/ QMdiSubWindow_metaObject for () {
  fn metaObject(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10metaObjectEv()};
    unsafe {_ZNK13QMdiSubWindow10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn sizeHint<T: QMdiSubWindow_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QMdiSubWindow_sizeHint {
  fn sizeHint(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QSize QMdiSubWindow::sizeHint();
impl<'a> /*trait*/ QMdiSubWindow_sizeHint for () {
  fn sizeHint(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8sizeHintEv()};
    unsafe {_ZNK13QMdiSubWindow8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn maximizedSystemMenuIconWidget<T: QMdiSubWindow_maximizedSystemMenuIconWidget>(&mut self, value: T) -> i32 {
    value.maximizedSystemMenuIconWidget(self);
    return 1;
  }
}

pub trait QMdiSubWindow_maximizedSystemMenuIconWidget {
  fn maximizedSystemMenuIconWidget(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedSystemMenuIconWidget for () {
  fn maximizedSystemMenuIconWidget(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv()};
    unsafe {_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setSystemMenu<T: QMdiSubWindow_setSystemMenu>(&mut self, value: T) -> i32 {
    value.setSystemMenu(self);
    return 1;
  }
}

pub trait QMdiSubWindow_setSystemMenu {
  fn setSystemMenu(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
impl<'a> /*trait*/ QMdiSubWindow_setSystemMenu for (&'a mut QMenu) {
  fn setSystemMenu(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow13setSystemMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QMdiSubWindow13setSystemMenuEP5QMenu(arg0)};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn FreeQMdiSubWindow<T: QMdiSubWindow_FreeQMdiSubWindow>(&mut self, value: T) -> i32 {
    value.FreeQMdiSubWindow(self);
    return 1;
  }
}

pub trait QMdiSubWindow_FreeQMdiSubWindow {
  fn FreeQMdiSubWindow(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::FreeQMdiSubWindow();
impl<'a> /*trait*/ QMdiSubWindow_FreeQMdiSubWindow for () {
  fn FreeQMdiSubWindow(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowD0Ev()};
    unsafe {_ZN13QMdiSubWindowD0Ev()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardSingleStep<T: QMdiSubWindow_setKeyboardSingleStep>(&mut self, value: T) -> i32 {
    value.setKeyboardSingleStep(self);
    return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardSingleStep {
  fn setKeyboardSingleStep(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::setKeyboardSingleStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardSingleStep for (i32) {
  fn setKeyboardSingleStep(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow21setKeyboardSingleStepEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QMdiSubWindow21setKeyboardSingleStepEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn widget<T: QMdiSubWindow_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QMdiSubWindow_widget {
  fn widget(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QWidget * QMdiSubWindow::widget();
impl<'a> /*trait*/ QMdiSubWindow_widget for () {
  fn widget(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow6widgetEv()};
    unsafe {_ZNK13QMdiSubWindow6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn showShaded<T: QMdiSubWindow_showShaded>(&mut self, value: T) -> i32 {
    value.showShaded(self);
    return 1;
  }
}

pub trait QMdiSubWindow_showShaded {
  fn showShaded(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::showShaded();
impl<'a> /*trait*/ QMdiSubWindow_showShaded for () {
  fn showShaded(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow10showShadedEv()};
    unsafe {_ZN13QMdiSubWindow10showShadedEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn maximizedButtonsWidget<T: QMdiSubWindow_maximizedButtonsWidget>(&mut self, value: T) -> i32 {
    value.maximizedButtonsWidget(self);
    return 1;
  }
}

pub trait QMdiSubWindow_maximizedButtonsWidget {
  fn maximizedButtonsWidget(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QWidget * QMdiSubWindow::maximizedButtonsWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedButtonsWidget for () {
  fn maximizedButtonsWidget(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv()};
    unsafe {_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn minimumSizeHint<T: QMdiSubWindow_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QMdiSubWindow_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QSize QMdiSubWindow::minimumSizeHint();
impl<'a> /*trait*/ QMdiSubWindow_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow15minimumSizeHintEv()};
    unsafe {_ZNK13QMdiSubWindow15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn aboutToActivate<T: QMdiSubWindow_aboutToActivate>(&mut self, value: T) -> i32 {
    value.aboutToActivate(self);
    return 1;
  }
}

pub trait QMdiSubWindow_aboutToActivate {
  fn aboutToActivate(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::aboutToActivate();
impl<'a> /*trait*/ QMdiSubWindow_aboutToActivate for () {
  fn aboutToActivate(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow15aboutToActivateEv()};
    unsafe {_ZN13QMdiSubWindow15aboutToActivateEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn showSystemMenu<T: QMdiSubWindow_showSystemMenu>(&mut self, value: T) -> i32 {
    value.showSystemMenu(self);
    return 1;
  }
}

pub trait QMdiSubWindow_showSystemMenu {
  fn showSystemMenu(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::showSystemMenu();
impl<'a> /*trait*/ QMdiSubWindow_showSystemMenu for () {
  fn showSystemMenu(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow14showSystemMenuEv()};
    unsafe {_ZN13QMdiSubWindow14showSystemMenuEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn systemMenu<T: QMdiSubWindow_systemMenu>(&mut self, value: T) -> i32 {
    value.systemMenu(self);
    return 1;
  }
}

pub trait QMdiSubWindow_systemMenu {
  fn systemMenu(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QMenu * QMdiSubWindow::systemMenu();
impl<'a> /*trait*/ QMdiSubWindow_systemMenu for () {
  fn systemMenu(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10systemMenuEv()};
    unsafe {_ZNK13QMdiSubWindow10systemMenuEv()};
    return 1;
  }
}

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

// proto: void QMdiSubWindow::NewQMdiSubWindow(const QMdiSubWindow & );
impl<'a> /*trait*/ QMdiSubWindow_NewQMdiSubWindow for (&'a  QMdiSubWindow) {
  fn NewQMdiSubWindow(self) -> QMdiSubWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN13QMdiSubWindowC1ERKS_(qthis, arg0)};
    let rsthis = QMdiSubWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setWidget<T: QMdiSubWindow_setWidget>(&mut self, value: T) -> i32 {
    value.setWidget(self);
    return 1;
  }
}

pub trait QMdiSubWindow_setWidget {
  fn setWidget(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::setWidget(QWidget * widget);
impl<'a> /*trait*/ QMdiSubWindow_setWidget for (&'a mut QWidget) {
  fn setWidget(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QMdiSubWindow9setWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn isShaded<T: QMdiSubWindow_isShaded>(&mut self, value: T) -> i32 {
    value.isShaded(self);
    return 1;
  }
}

pub trait QMdiSubWindow_isShaded {
  fn isShaded(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: bool QMdiSubWindow::isShaded();
impl<'a> /*trait*/ QMdiSubWindow_isShaded for () {
  fn isShaded(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8isShadedEv()};
    unsafe {_ZNK13QMdiSubWindow8isShadedEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn mdiArea<T: QMdiSubWindow_mdiArea>(&mut self, value: T) -> i32 {
    value.mdiArea(self);
    return 1;
  }
}

pub trait QMdiSubWindow_mdiArea {
  fn mdiArea(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: QMdiArea * QMdiSubWindow::mdiArea();
impl<'a> /*trait*/ QMdiSubWindow_mdiArea for () {
  fn mdiArea(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow7mdiAreaEv()};
    unsafe {_ZNK13QMdiSubWindow7mdiAreaEv()};
    return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardPageStep<T: QMdiSubWindow_setKeyboardPageStep>(&mut self, value: T) -> i32 {
    value.setKeyboardPageStep(self);
    return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardPageStep {
  fn setKeyboardPageStep(self, this: &mut QMdiSubWindow) -> i32;
}

// proto: void QMdiSubWindow::setKeyboardPageStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardPageStep for (i32) {
  fn setKeyboardPageStep(self, this: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow19setKeyboardPageStepEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN13QMdiSubWindow19setKeyboardPageStepEi(arg0)};
    return 1;
  }
}

