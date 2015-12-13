// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QRect QDesktopWidget::screenGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget14screenGeometryERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QWidget * QDesktopWidget::screen(int screen);
  fn _ZN14QDesktopWidget6screenEi(arg0: c_int) -> i32;
  // proto: const QRect QDesktopWidget::screenGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget14screenGeometryEPK7QWidget(arg0: *const c_void) -> i32;
  // proto: void QDesktopWidget::screenCountChanged(int );
  fn _ZN14QDesktopWidget18screenCountChangedEi(arg0: c_int) -> i32;
  // proto: int QDesktopWidget::numScreens();
  fn _ZNK14QDesktopWidget10numScreensEv() -> i32;
  // proto: void QDesktopWidget::FreeQDesktopWidget();
  fn _ZN14QDesktopWidgetD0Ev() -> i32;
  // proto: const QRect QDesktopWidget::screenGeometry(int screen);
  fn _ZNK14QDesktopWidget14screenGeometryEi(arg0: c_int) -> i32;
  // proto: const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget17availableGeometryEPK7QWidget(arg0: *const c_void) -> i32;
  // proto: void QDesktopWidget::NewQDesktopWidget(const QDesktopWidget & );
  fn _ZN14QDesktopWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QDesktopWidget::resized(int );
  fn _ZN14QDesktopWidget7resizedEi(arg0: c_int) -> i32;
  // proto: int QDesktopWidget::screenNumber(const QPoint & );
  fn _ZNK14QDesktopWidget12screenNumberERK6QPoint(arg0: *const c_void) -> i32;
  // proto: int QDesktopWidget::screenCount();
  fn _ZNK14QDesktopWidget11screenCountEv() -> i32;
  // proto: bool QDesktopWidget::isVirtualDesktop();
  fn _ZNK14QDesktopWidget16isVirtualDesktopEv() -> i32;
  // proto: int QDesktopWidget::screenNumber(const QWidget * widget);
  fn _ZNK14QDesktopWidget12screenNumberEPK7QWidget(arg0: *const c_void) -> i32;
  // proto: int QDesktopWidget::primaryScreen();
  fn _ZNK14QDesktopWidget13primaryScreenEv() -> i32;
  // proto: void QDesktopWidget::NewQDesktopWidget();
  fn _ZN14QDesktopWidgetC1Ev(qthis: *mut c_void) -> i32;
  // proto: const QRect QDesktopWidget::availableGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget17availableGeometryERK6QPoint(arg0: *const c_void) -> i32;
  // proto: const QRect QDesktopWidget::availableGeometry(int screen);
  fn _ZNK14QDesktopWidget17availableGeometryEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QDesktopWidget::metaObject();
  fn _ZNK14QDesktopWidget10metaObjectEv() -> i32;
  // proto: void QDesktopWidget::workAreaResized(int );
  fn _ZN14QDesktopWidget15workAreaResizedEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QDesktopWidget)=1
pub struct QDesktopWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry<T: QDesktopWidget_screenGeometry>(&mut self, value: T) -> i32 {
    value.screenGeometry(self);
    return 1;
  }
}

pub trait QDesktopWidget_screenGeometry {
  fn screenGeometry(self, this: &mut QDesktopWidget) -> i32;
}

// proto: const QRect QDesktopWidget::screenGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry for (&'a  QPoint) {
  fn screenGeometry(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDesktopWidget14screenGeometryERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screen<T: QDesktopWidget_screen>(&mut self, value: T) -> i32 {
    value.screen(self);
    return 1;
  }
}

pub trait QDesktopWidget_screen {
  fn screen(self, this: &mut QDesktopWidget) -> i32;
}

// proto: QWidget * QDesktopWidget::screen(int screen);
impl<'a> /*trait*/ QDesktopWidget_screen for (i32) {
  fn screen(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget6screenEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QDesktopWidget6screenEi(arg0)};
    return 1;
  }
}

// proto: const QRect QDesktopWidget::screenGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry for (&'a  QWidget) {
  fn screenGeometry(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDesktopWidget14screenGeometryEPK7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screenCountChanged<T: QDesktopWidget_screenCountChanged>(&mut self, value: T) -> i32 {
    value.screenCountChanged(self);
    return 1;
  }
}

pub trait QDesktopWidget_screenCountChanged {
  fn screenCountChanged(self, this: &mut QDesktopWidget) -> i32;
}

// proto: void QDesktopWidget::screenCountChanged(int );
impl<'a> /*trait*/ QDesktopWidget_screenCountChanged for (i32) {
  fn screenCountChanged(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget18screenCountChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QDesktopWidget18screenCountChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn numScreens<T: QDesktopWidget_numScreens>(&mut self, value: T) -> i32 {
    value.numScreens(self);
    return 1;
  }
}

pub trait QDesktopWidget_numScreens {
  fn numScreens(self, this: &mut QDesktopWidget) -> i32;
}

// proto: int QDesktopWidget::numScreens();
impl<'a> /*trait*/ QDesktopWidget_numScreens for () {
  fn numScreens(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10numScreensEv()};
    unsafe {_ZNK14QDesktopWidget10numScreensEv()};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn FreeQDesktopWidget<T: QDesktopWidget_FreeQDesktopWidget>(&mut self, value: T) -> i32 {
    value.FreeQDesktopWidget(self);
    return 1;
  }
}

pub trait QDesktopWidget_FreeQDesktopWidget {
  fn FreeQDesktopWidget(self, this: &mut QDesktopWidget) -> i32;
}

// proto: void QDesktopWidget::FreeQDesktopWidget();
impl<'a> /*trait*/ QDesktopWidget_FreeQDesktopWidget for () {
  fn FreeQDesktopWidget(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetD0Ev()};
    unsafe {_ZN14QDesktopWidgetD0Ev()};
    return 1;
  }
}

// proto: const QRect QDesktopWidget::screenGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry for (i32) {
  fn screenGeometry(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK14QDesktopWidget14screenGeometryEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry<T: QDesktopWidget_availableGeometry>(&mut self, value: T) -> i32 {
    value.availableGeometry(self);
    return 1;
  }
}

pub trait QDesktopWidget_availableGeometry {
  fn availableGeometry(self, this: &mut QDesktopWidget) -> i32;
}

// proto: const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry for (&'a  QWidget) {
  fn availableGeometry(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDesktopWidget17availableGeometryEPK7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn NewQDesktopWidget<T: QDesktopWidget_NewQDesktopWidget>(value: T) -> QDesktopWidget {
    let rsthis = value.NewQDesktopWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QDesktopWidget_NewQDesktopWidget {
  fn NewQDesktopWidget(self) -> QDesktopWidget;
}

// proto: void QDesktopWidget::NewQDesktopWidget(const QDesktopWidget & );
impl<'a> /*trait*/ QDesktopWidget_NewQDesktopWidget for (&'a  QDesktopWidget) {
  fn NewQDesktopWidget(self) -> QDesktopWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QDesktopWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QDesktopWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn resized<T: QDesktopWidget_resized>(&mut self, value: T) -> i32 {
    value.resized(self);
    return 1;
  }
}

pub trait QDesktopWidget_resized {
  fn resized(self, this: &mut QDesktopWidget) -> i32;
}

// proto: void QDesktopWidget::resized(int );
impl<'a> /*trait*/ QDesktopWidget_resized for (i32) {
  fn resized(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget7resizedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QDesktopWidget7resizedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screenNumber<T: QDesktopWidget_screenNumber>(&mut self, value: T) -> i32 {
    value.screenNumber(self);
    return 1;
  }
}

pub trait QDesktopWidget_screenNumber {
  fn screenNumber(self, this: &mut QDesktopWidget) -> i32;
}

// proto: int QDesktopWidget::screenNumber(const QPoint & );
impl<'a> /*trait*/ QDesktopWidget_screenNumber for (&'a  QPoint) {
  fn screenNumber(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget12screenNumberERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDesktopWidget12screenNumberERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screenCount<T: QDesktopWidget_screenCount>(&mut self, value: T) -> i32 {
    value.screenCount(self);
    return 1;
  }
}

pub trait QDesktopWidget_screenCount {
  fn screenCount(self, this: &mut QDesktopWidget) -> i32;
}

// proto: int QDesktopWidget::screenCount();
impl<'a> /*trait*/ QDesktopWidget_screenCount for () {
  fn screenCount(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget11screenCountEv()};
    unsafe {_ZNK14QDesktopWidget11screenCountEv()};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn isVirtualDesktop<T: QDesktopWidget_isVirtualDesktop>(&mut self, value: T) -> i32 {
    value.isVirtualDesktop(self);
    return 1;
  }
}

pub trait QDesktopWidget_isVirtualDesktop {
  fn isVirtualDesktop(self, this: &mut QDesktopWidget) -> i32;
}

// proto: bool QDesktopWidget::isVirtualDesktop();
impl<'a> /*trait*/ QDesktopWidget_isVirtualDesktop for () {
  fn isVirtualDesktop(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget16isVirtualDesktopEv()};
    unsafe {_ZNK14QDesktopWidget16isVirtualDesktopEv()};
    return 1;
  }
}

// proto: int QDesktopWidget::screenNumber(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenNumber for (&'a  QWidget) {
  fn screenNumber(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget12screenNumberEPK7QWidget()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDesktopWidget12screenNumberEPK7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn primaryScreen<T: QDesktopWidget_primaryScreen>(&mut self, value: T) -> i32 {
    value.primaryScreen(self);
    return 1;
  }
}

pub trait QDesktopWidget_primaryScreen {
  fn primaryScreen(self, this: &mut QDesktopWidget) -> i32;
}

// proto: int QDesktopWidget::primaryScreen();
impl<'a> /*trait*/ QDesktopWidget_primaryScreen for () {
  fn primaryScreen(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget13primaryScreenEv()};
    unsafe {_ZNK14QDesktopWidget13primaryScreenEv()};
    return 1;
  }
}

// proto: void QDesktopWidget::NewQDesktopWidget();
impl<'a> /*trait*/ QDesktopWidget_NewQDesktopWidget for () {
  fn NewQDesktopWidget(self) -> QDesktopWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetC1Ev()};
    unsafe {_ZN14QDesktopWidgetC1Ev(qthis)};
    let rsthis = QDesktopWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: const QRect QDesktopWidget::availableGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry for (&'a  QPoint) {
  fn availableGeometry(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK14QDesktopWidget17availableGeometryERK6QPoint(arg0)};
    return 1;
  }
}

// proto: const QRect QDesktopWidget::availableGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry for (i32) {
  fn availableGeometry(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK14QDesktopWidget17availableGeometryEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn metaObject<T: QDesktopWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDesktopWidget_metaObject {
  fn metaObject(self, this: &mut QDesktopWidget) -> i32;
}

// proto: const QMetaObject * QDesktopWidget::metaObject();
impl<'a> /*trait*/ QDesktopWidget_metaObject for () {
  fn metaObject(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10metaObjectEv()};
    unsafe {_ZNK14QDesktopWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn workAreaResized<T: QDesktopWidget_workAreaResized>(&mut self, value: T) -> i32 {
    value.workAreaResized(self);
    return 1;
  }
}

pub trait QDesktopWidget_workAreaResized {
  fn workAreaResized(self, this: &mut QDesktopWidget) -> i32;
}

// proto: void QDesktopWidget::workAreaResized(int );
impl<'a> /*trait*/ QDesktopWidget_workAreaResized for (i32) {
  fn workAreaResized(self, this: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget15workAreaResizedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QDesktopWidget15workAreaResizedEi(arg0)};
    return 1;
  }
}

