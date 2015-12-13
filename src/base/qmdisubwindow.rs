// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qmenu::QMenu;
use super::qmdiarea::QMdiArea;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QMdiSubWindow::keyboardSingleStep();
  fn _ZNK13QMdiSubWindow18keyboardSingleStepEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMdiSubWindow::keyboardPageStep();
  fn _ZNK13QMdiSubWindow16keyboardPageStepEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QMdiSubWindow::metaObject();
  fn _ZNK13QMdiSubWindow10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QSize QMdiSubWindow::sizeHint();
  fn _ZNK13QMdiSubWindow8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
  fn _ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
  fn _ZN13QMdiSubWindow13setSystemMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMdiSubWindow::FreeQMdiSubWindow();
  fn _ZN13QMdiSubWindowD0Ev(qthis: *mut c_void) ;
  // proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
  fn _ZN13QMdiSubWindow21setKeyboardSingleStepEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QWidget * QMdiSubWindow::widget();
  fn _ZNK13QMdiSubWindow6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::showShaded();
  fn _ZN13QMdiSubWindow10showShadedEv(qthis: *mut c_void) ;
  // proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
  fn _ZNK13QMdiSubWindow22maximizedButtonsWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QMdiSubWindow::minimumSizeHint();
  fn _ZNK13QMdiSubWindow15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::aboutToActivate();
  fn _ZN13QMdiSubWindow15aboutToActivateEv(qthis: *mut c_void) ;
  // proto:  void QMdiSubWindow::showSystemMenu();
  fn _ZN13QMdiSubWindow14showSystemMenuEv(qthis: *mut c_void) ;
  // proto:  QMenu * QMdiSubWindow::systemMenu();
  fn _ZNK13QMdiSubWindow10systemMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::NewQMdiSubWindow(const QMdiSubWindow & );
  fn _ZN13QMdiSubWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMdiSubWindow::setWidget(QWidget * widget);
  fn _ZN13QMdiSubWindow9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMdiSubWindow::isShaded();
  fn _ZNK13QMdiSubWindow8isShadedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QMdiArea * QMdiSubWindow::mdiArea();
  fn _ZNK13QMdiSubWindow7mdiAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
  fn _ZN13QMdiSubWindow19setKeyboardPageStepEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QMdiSubWindow)=1
pub struct QMdiSubWindow {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMdiSubWindow {
  pub fn keyboardSingleStep<T: QMdiSubWindow_keyboardSingleStep>(&mut self, value: T) -> i32 {
    return value.keyboardSingleStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_keyboardSingleStep {
  fn keyboardSingleStep(self, rsthis: &mut QMdiSubWindow) -> i32;
}

// proto:  int QMdiSubWindow::keyboardSingleStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardSingleStep for () {
  fn keyboardSingleStep(self, rsthis: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow18keyboardSingleStepEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow18keyboardSingleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn keyboardPageStep<T: QMdiSubWindow_keyboardPageStep>(&mut self, value: T) -> i32 {
    return value.keyboardPageStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_keyboardPageStep {
  fn keyboardPageStep(self, rsthis: &mut QMdiSubWindow) -> i32;
}

// proto:  int QMdiSubWindow::keyboardPageStep();
impl<'a> /*trait*/ QMdiSubWindow_keyboardPageStep for () {
  fn keyboardPageStep(self, rsthis: &mut QMdiSubWindow) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow16keyboardPageStepEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow16keyboardPageStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn metaObject<T: QMdiSubWindow_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_metaObject {
  fn metaObject(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  const QMetaObject * QMdiSubWindow::metaObject();
impl<'a> /*trait*/ QMdiSubWindow_metaObject for () {
  fn metaObject(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10metaObjectEv()};
     unsafe {_ZNK13QMdiSubWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn sizeHint<T: QMdiSubWindow_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_sizeHint {
  fn sizeHint(self, rsthis: &mut QMdiSubWindow) -> QSize;
}

// proto:  QSize QMdiSubWindow::sizeHint();
impl<'a> /*trait*/ QMdiSubWindow_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QMdiSubWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn maximizedSystemMenuIconWidget<T: QMdiSubWindow_maximizedSystemMenuIconWidget>(&mut self, value: T) -> QWidget {
    return value.maximizedSystemMenuIconWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_maximizedSystemMenuIconWidget {
  fn maximizedSystemMenuIconWidget(self, rsthis: &mut QMdiSubWindow) -> QWidget;
}

// proto:  QWidget * QMdiSubWindow::maximizedSystemMenuIconWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedSystemMenuIconWidget for () {
  fn maximizedSystemMenuIconWidget(self, rsthis: &mut QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow29maximizedSystemMenuIconWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setSystemMenu<T: QMdiSubWindow_setSystemMenu>(&mut self, value: T)  {
     value.setSystemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setSystemMenu {
  fn setSystemMenu(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::setSystemMenu(QMenu * systemMenu);
impl<'a> /*trait*/ QMdiSubWindow_setSystemMenu for (&'a mut QMenu) {
  fn setSystemMenu(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow13setSystemMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QMdiSubWindow13setSystemMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn FreeQMdiSubWindow<T: QMdiSubWindow_FreeQMdiSubWindow>(&mut self, value: T)  {
     value.FreeQMdiSubWindow(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_FreeQMdiSubWindow {
  fn FreeQMdiSubWindow(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::FreeQMdiSubWindow();
impl<'a> /*trait*/ QMdiSubWindow_FreeQMdiSubWindow for () {
  fn FreeQMdiSubWindow(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindowD0Ev()};
     unsafe {_ZN13QMdiSubWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardSingleStep<T: QMdiSubWindow_setKeyboardSingleStep>(&mut self, value: T)  {
     value.setKeyboardSingleStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardSingleStep {
  fn setKeyboardSingleStep(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::setKeyboardSingleStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardSingleStep for (i32) {
  fn setKeyboardSingleStep(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow21setKeyboardSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QMdiSubWindow21setKeyboardSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn widget<T: QMdiSubWindow_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_widget {
  fn widget(self, rsthis: &mut QMdiSubWindow) -> QWidget;
}

// proto:  QWidget * QMdiSubWindow::widget();
impl<'a> /*trait*/ QMdiSubWindow_widget for () {
  fn widget(self, rsthis: &mut QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow6widgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn showShaded<T: QMdiSubWindow_showShaded>(&mut self, value: T)  {
     value.showShaded(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_showShaded {
  fn showShaded(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::showShaded();
impl<'a> /*trait*/ QMdiSubWindow_showShaded for () {
  fn showShaded(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow10showShadedEv()};
     unsafe {_ZN13QMdiSubWindow10showShadedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn maximizedButtonsWidget<T: QMdiSubWindow_maximizedButtonsWidget>(&mut self, value: T) -> QWidget {
    return value.maximizedButtonsWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_maximizedButtonsWidget {
  fn maximizedButtonsWidget(self, rsthis: &mut QMdiSubWindow) -> QWidget;
}

// proto:  QWidget * QMdiSubWindow::maximizedButtonsWidget();
impl<'a> /*trait*/ QMdiSubWindow_maximizedButtonsWidget for () {
  fn maximizedButtonsWidget(self, rsthis: &mut QMdiSubWindow) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow22maximizedButtonsWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn minimumSizeHint<T: QMdiSubWindow_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QMdiSubWindow) -> QSize;
}

// proto:  QSize QMdiSubWindow::minimumSizeHint();
impl<'a> /*trait*/ QMdiSubWindow_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QMdiSubWindow) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn aboutToActivate<T: QMdiSubWindow_aboutToActivate>(&mut self, value: T)  {
     value.aboutToActivate(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_aboutToActivate {
  fn aboutToActivate(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::aboutToActivate();
impl<'a> /*trait*/ QMdiSubWindow_aboutToActivate for () {
  fn aboutToActivate(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow15aboutToActivateEv()};
     unsafe {_ZN13QMdiSubWindow15aboutToActivateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn showSystemMenu<T: QMdiSubWindow_showSystemMenu>(&mut self, value: T)  {
     value.showSystemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_showSystemMenu {
  fn showSystemMenu(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::showSystemMenu();
impl<'a> /*trait*/ QMdiSubWindow_showSystemMenu for () {
  fn showSystemMenu(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow14showSystemMenuEv()};
     unsafe {_ZN13QMdiSubWindow14showSystemMenuEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn systemMenu<T: QMdiSubWindow_systemMenu>(&mut self, value: T) -> QMenu {
    return value.systemMenu(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_systemMenu {
  fn systemMenu(self, rsthis: &mut QMdiSubWindow) -> QMenu;
}

// proto:  QMenu * QMdiSubWindow::systemMenu();
impl<'a> /*trait*/ QMdiSubWindow_systemMenu for () {
  fn systemMenu(self, rsthis: &mut QMdiSubWindow) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow10systemMenuEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow10systemMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QMdiSubWindowC1ERKS_(qthis, arg0)};
    let rsthis = QMdiSubWindow{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setWidget<T: QMdiSubWindow_setWidget>(&mut self, value: T)  {
     value.setWidget(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setWidget {
  fn setWidget(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::setWidget(QWidget * widget);
impl<'a> /*trait*/ QMdiSubWindow_setWidget for (&'a mut QWidget) {
  fn setWidget(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QMdiSubWindow9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn isShaded<T: QMdiSubWindow_isShaded>(&mut self, value: T) -> i8 {
    return value.isShaded(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_isShaded {
  fn isShaded(self, rsthis: &mut QMdiSubWindow) -> i8;
}

// proto:  bool QMdiSubWindow::isShaded();
impl<'a> /*trait*/ QMdiSubWindow_isShaded for () {
  fn isShaded(self, rsthis: &mut QMdiSubWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow8isShadedEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow8isShadedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn mdiArea<T: QMdiSubWindow_mdiArea>(&mut self, value: T) -> QMdiArea {
    return value.mdiArea(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_mdiArea {
  fn mdiArea(self, rsthis: &mut QMdiSubWindow) -> QMdiArea;
}

// proto:  QMdiArea * QMdiSubWindow::mdiArea();
impl<'a> /*trait*/ QMdiSubWindow_mdiArea for () {
  fn mdiArea(self, rsthis: &mut QMdiSubWindow) -> QMdiArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QMdiSubWindow7mdiAreaEv()};
    let mut ret = unsafe {_ZNK13QMdiSubWindow7mdiAreaEv(rsthis.qclsinst)};
    let mut ret1 = QMdiArea{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiSubWindow {
  pub fn setKeyboardPageStep<T: QMdiSubWindow_setKeyboardPageStep>(&mut self, value: T)  {
     value.setKeyboardPageStep(self);
    // return 1;
  }
}

pub trait QMdiSubWindow_setKeyboardPageStep {
  fn setKeyboardPageStep(self, rsthis: &mut QMdiSubWindow) ;
}

// proto:  void QMdiSubWindow::setKeyboardPageStep(int step);
impl<'a> /*trait*/ QMdiSubWindow_setKeyboardPageStep for (i32) {
  fn setKeyboardPageStep(self, rsthis: &mut QMdiSubWindow)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QMdiSubWindow19setKeyboardPageStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QMdiSubWindow19setKeyboardPageStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

