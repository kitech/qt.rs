// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qrect::QRect;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget14screenGeometryERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QDesktopWidget::screen(int screen);
  fn _ZN14QDesktopWidget6screenEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::screenGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget14screenGeometryEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDesktopWidget::screenCountChanged(int );
  fn _ZN14QDesktopWidget18screenCountChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QDesktopWidget::numScreens();
  fn _ZNK14QDesktopWidget10numScreensEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDesktopWidget::FreeQDesktopWidget();
  fn _ZN14QDesktopWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  const QRect QDesktopWidget::screenGeometry(int screen);
  fn _ZNK14QDesktopWidget14screenGeometryEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget17availableGeometryEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDesktopWidget::NewQDesktopWidget(const QDesktopWidget & );
  fn _ZN14QDesktopWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDesktopWidget::resized(int );
  fn _ZN14QDesktopWidget7resizedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QDesktopWidget::screenNumber(const QPoint & );
  fn _ZNK14QDesktopWidget12screenNumberERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QDesktopWidget::screenCount();
  fn _ZNK14QDesktopWidget11screenCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDesktopWidget::isVirtualDesktop();
  fn _ZNK14QDesktopWidget16isVirtualDesktopEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QDesktopWidget::screenNumber(const QWidget * widget);
  fn _ZNK14QDesktopWidget12screenNumberEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QDesktopWidget::primaryScreen();
  fn _ZNK14QDesktopWidget13primaryScreenEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDesktopWidget::NewQDesktopWidget();
  fn _ZN14QDesktopWidgetC1Ev(qthis: *mut c_void) ;
  // proto:  const QRect QDesktopWidget::availableGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget17availableGeometryERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::availableGeometry(int screen);
  fn _ZNK14QDesktopWidget17availableGeometryEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QDesktopWidget::metaObject();
  fn _ZNK14QDesktopWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDesktopWidget::workAreaResized(int );
  fn _ZN14QDesktopWidget15workAreaResizedEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QDesktopWidget)=1
pub struct QDesktopWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry<T: QDesktopWidget_screenGeometry>(&mut self, value: T) -> QRect {
    return value.screenGeometry(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenGeometry {
  fn screenGeometry(self, rsthis: &mut QDesktopWidget) -> QRect;
}

// proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry for (&'a  QPoint) {
  fn screenGeometry(self, rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screen<T: QDesktopWidget_screen>(&mut self, value: T) -> QWidget {
    return value.screen(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screen {
  fn screen(self, rsthis: &mut QDesktopWidget) -> QWidget;
}

// proto:  QWidget * QDesktopWidget::screen(int screen);
impl<'a> /*trait*/ QDesktopWidget_screen for (i32) {
  fn screen(self, rsthis: &mut QDesktopWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget6screenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QDesktopWidget6screenEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QRect QDesktopWidget::screenGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry for (&'a  QWidget) {
  fn screenGeometry(self, rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screenCountChanged<T: QDesktopWidget_screenCountChanged>(&mut self, value: T)  {
     value.screenCountChanged(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenCountChanged {
  fn screenCountChanged(self, rsthis: &mut QDesktopWidget) ;
}

// proto:  void QDesktopWidget::screenCountChanged(int );
impl<'a> /*trait*/ QDesktopWidget_screenCountChanged for (i32) {
  fn screenCountChanged(self, rsthis: &mut QDesktopWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget18screenCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget18screenCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn numScreens<T: QDesktopWidget_numScreens>(&mut self, value: T) -> i32 {
    return value.numScreens(self);
    // return 1;
  }
}

pub trait QDesktopWidget_numScreens {
  fn numScreens(self, rsthis: &mut QDesktopWidget) -> i32;
}

// proto:  int QDesktopWidget::numScreens();
impl<'a> /*trait*/ QDesktopWidget_numScreens for () {
  fn numScreens(self, rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10numScreensEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget10numScreensEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn FreeQDesktopWidget<T: QDesktopWidget_FreeQDesktopWidget>(&mut self, value: T)  {
     value.FreeQDesktopWidget(self);
    // return 1;
  }
}

pub trait QDesktopWidget_FreeQDesktopWidget {
  fn FreeQDesktopWidget(self, rsthis: &mut QDesktopWidget) ;
}

// proto:  void QDesktopWidget::FreeQDesktopWidget();
impl<'a> /*trait*/ QDesktopWidget_FreeQDesktopWidget for () {
  fn FreeQDesktopWidget(self, rsthis: &mut QDesktopWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetD0Ev()};
     unsafe {_ZN14QDesktopWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QRect QDesktopWidget::screenGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry for (i32) {
  fn screenGeometry(self, rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry<T: QDesktopWidget_availableGeometry>(&mut self, value: T) -> QRect {
    return value.availableGeometry(self);
    // return 1;
  }
}

pub trait QDesktopWidget_availableGeometry {
  fn availableGeometry(self, rsthis: &mut QDesktopWidget) -> QRect;
}

// proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry for (&'a  QWidget) {
  fn availableGeometry(self, rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QDesktopWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QDesktopWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn resized<T: QDesktopWidget_resized>(&mut self, value: T)  {
     value.resized(self);
    // return 1;
  }
}

pub trait QDesktopWidget_resized {
  fn resized(self, rsthis: &mut QDesktopWidget) ;
}

// proto:  void QDesktopWidget::resized(int );
impl<'a> /*trait*/ QDesktopWidget_resized for (i32) {
  fn resized(self, rsthis: &mut QDesktopWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget7resizedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget7resizedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screenNumber<T: QDesktopWidget_screenNumber>(&mut self, value: T) -> i32 {
    return value.screenNumber(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenNumber {
  fn screenNumber(self, rsthis: &mut QDesktopWidget) -> i32;
}

// proto:  int QDesktopWidget::screenNumber(const QPoint & );
impl<'a> /*trait*/ QDesktopWidget_screenNumber for (&'a  QPoint) {
  fn screenNumber(self, rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget12screenNumberERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget12screenNumberERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn screenCount<T: QDesktopWidget_screenCount>(&mut self, value: T) -> i32 {
    return value.screenCount(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenCount {
  fn screenCount(self, rsthis: &mut QDesktopWidget) -> i32;
}

// proto:  int QDesktopWidget::screenCount();
impl<'a> /*trait*/ QDesktopWidget_screenCount for () {
  fn screenCount(self, rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget11screenCountEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget11screenCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn isVirtualDesktop<T: QDesktopWidget_isVirtualDesktop>(&mut self, value: T) -> i8 {
    return value.isVirtualDesktop(self);
    // return 1;
  }
}

pub trait QDesktopWidget_isVirtualDesktop {
  fn isVirtualDesktop(self, rsthis: &mut QDesktopWidget) -> i8;
}

// proto:  bool QDesktopWidget::isVirtualDesktop();
impl<'a> /*trait*/ QDesktopWidget_isVirtualDesktop for () {
  fn isVirtualDesktop(self, rsthis: &mut QDesktopWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget16isVirtualDesktopEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget16isVirtualDesktopEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QDesktopWidget::screenNumber(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenNumber for (&'a  QWidget) {
  fn screenNumber(self, rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget12screenNumberEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget12screenNumberEPK7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn primaryScreen<T: QDesktopWidget_primaryScreen>(&mut self, value: T) -> i32 {
    return value.primaryScreen(self);
    // return 1;
  }
}

pub trait QDesktopWidget_primaryScreen {
  fn primaryScreen(self, rsthis: &mut QDesktopWidget) -> i32;
}

// proto:  int QDesktopWidget::primaryScreen();
impl<'a> /*trait*/ QDesktopWidget_primaryScreen for () {
  fn primaryScreen(self, rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget13primaryScreenEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget13primaryScreenEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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

// proto:  const QRect QDesktopWidget::availableGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry for (&'a  QPoint) {
  fn availableGeometry(self, rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QRect QDesktopWidget::availableGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry for (i32) {
  fn availableGeometry(self, rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn metaObject<T: QDesktopWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDesktopWidget_metaObject {
  fn metaObject(self, rsthis: &mut QDesktopWidget) ;
}

// proto:  const QMetaObject * QDesktopWidget::metaObject();
impl<'a> /*trait*/ QDesktopWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QDesktopWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10metaObjectEv()};
     unsafe {_ZNK14QDesktopWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDesktopWidget {
  pub fn workAreaResized<T: QDesktopWidget_workAreaResized>(&mut self, value: T)  {
     value.workAreaResized(self);
    // return 1;
  }
}

pub trait QDesktopWidget_workAreaResized {
  fn workAreaResized(self, rsthis: &mut QDesktopWidget) ;
}

// proto:  void QDesktopWidget::workAreaResized(int );
impl<'a> /*trait*/ QDesktopWidget_workAreaResized for (i32) {
  fn workAreaResized(self, rsthis: &mut QDesktopWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget15workAreaResizedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget15workAreaResizedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

