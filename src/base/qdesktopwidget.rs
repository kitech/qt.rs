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

// proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry<RetType, T: QDesktopWidget_screenGeometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenGeometry(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenGeometry<RetType> {
  fn screenGeometry(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry<QRect> for (&'a  QPoint) {
  fn screenGeometry(self , rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QWidget * QDesktopWidget::screen(int screen);
impl /*struct*/ QDesktopWidget {
  pub fn screen<RetType, T: QDesktopWidget_screen<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screen<RetType> {
  fn screen(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  QWidget * QDesktopWidget::screen(int screen);
impl<'a> /*trait*/ QDesktopWidget_screen<QWidget> for (i32) {
  fn screen(self , rsthis: &mut QDesktopWidget) -> QWidget {
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
impl<'a> /*trait*/ QDesktopWidget_screenGeometry<QRect> for (&'a  QWidget) {
  fn screenGeometry(self , rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QDesktopWidget::screenCountChanged(int );
impl /*struct*/ QDesktopWidget {
  pub fn screenCountChanged<RetType, T: QDesktopWidget_screenCountChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenCountChanged(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenCountChanged<RetType> {
  fn screenCountChanged(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  void QDesktopWidget::screenCountChanged(int );
impl<'a> /*trait*/ QDesktopWidget_screenCountChanged<()> for (i32) {
  fn screenCountChanged(self , rsthis: &mut QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget18screenCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget18screenCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QDesktopWidget::numScreens();
impl /*struct*/ QDesktopWidget {
  pub fn numScreens<RetType, T: QDesktopWidget_numScreens<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.numScreens(self);
    // return 1;
  }
}

pub trait QDesktopWidget_numScreens<RetType> {
  fn numScreens(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  int QDesktopWidget::numScreens();
impl<'a> /*trait*/ QDesktopWidget_numScreens<i32> for () {
  fn numScreens(self , rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10numScreensEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget10numScreensEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QDesktopWidget::FreeQDesktopWidget();
impl /*struct*/ QDesktopWidget {
  pub fn FreeQDesktopWidget<RetType, T: QDesktopWidget_FreeQDesktopWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQDesktopWidget(self);
    // return 1;
  }
}

pub trait QDesktopWidget_FreeQDesktopWidget<RetType> {
  fn FreeQDesktopWidget(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  void QDesktopWidget::FreeQDesktopWidget();
impl<'a> /*trait*/ QDesktopWidget_FreeQDesktopWidget<()> for () {
  fn FreeQDesktopWidget(self , rsthis: &mut QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetD0Ev()};
     unsafe {_ZN14QDesktopWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  const QRect QDesktopWidget::screenGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry<QRect> for (i32) {
  fn screenGeometry(self , rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry<RetType, T: QDesktopWidget_availableGeometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.availableGeometry(self);
    // return 1;
  }
}

pub trait QDesktopWidget_availableGeometry<RetType> {
  fn availableGeometry(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry<QRect> for (&'a  QWidget) {
  fn availableGeometry(self , rsthis: &mut QDesktopWidget) -> QRect {
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

// proto:  void QDesktopWidget::resized(int );
impl /*struct*/ QDesktopWidget {
  pub fn resized<RetType, T: QDesktopWidget_resized<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.resized(self);
    // return 1;
  }
}

pub trait QDesktopWidget_resized<RetType> {
  fn resized(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  void QDesktopWidget::resized(int );
impl<'a> /*trait*/ QDesktopWidget_resized<()> for (i32) {
  fn resized(self , rsthis: &mut QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget7resizedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget7resizedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QDesktopWidget::screenNumber(const QPoint & );
impl /*struct*/ QDesktopWidget {
  pub fn screenNumber<RetType, T: QDesktopWidget_screenNumber<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenNumber(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenNumber<RetType> {
  fn screenNumber(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  int QDesktopWidget::screenNumber(const QPoint & );
impl<'a> /*trait*/ QDesktopWidget_screenNumber<i32> for (&'a  QPoint) {
  fn screenNumber(self , rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget12screenNumberERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget12screenNumberERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QDesktopWidget::screenCount();
impl /*struct*/ QDesktopWidget {
  pub fn screenCount<RetType, T: QDesktopWidget_screenCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.screenCount(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenCount<RetType> {
  fn screenCount(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  int QDesktopWidget::screenCount();
impl<'a> /*trait*/ QDesktopWidget_screenCount<i32> for () {
  fn screenCount(self , rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget11screenCountEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget11screenCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QDesktopWidget::isVirtualDesktop();
impl /*struct*/ QDesktopWidget {
  pub fn isVirtualDesktop<RetType, T: QDesktopWidget_isVirtualDesktop<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isVirtualDesktop(self);
    // return 1;
  }
}

pub trait QDesktopWidget_isVirtualDesktop<RetType> {
  fn isVirtualDesktop(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  bool QDesktopWidget::isVirtualDesktop();
impl<'a> /*trait*/ QDesktopWidget_isVirtualDesktop<i8> for () {
  fn isVirtualDesktop(self , rsthis: &mut QDesktopWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget16isVirtualDesktopEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget16isVirtualDesktopEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  int QDesktopWidget::screenNumber(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenNumber<i32> for (&'a  QWidget) {
  fn screenNumber(self , rsthis: &mut QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget12screenNumberEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget12screenNumberEPK7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QDesktopWidget::primaryScreen();
impl /*struct*/ QDesktopWidget {
  pub fn primaryScreen<RetType, T: QDesktopWidget_primaryScreen<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.primaryScreen(self);
    // return 1;
  }
}

pub trait QDesktopWidget_primaryScreen<RetType> {
  fn primaryScreen(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  int QDesktopWidget::primaryScreen();
impl<'a> /*trait*/ QDesktopWidget_primaryScreen<i32> for () {
  fn primaryScreen(self , rsthis: &mut QDesktopWidget) -> i32 {
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
impl<'a> /*trait*/ QDesktopWidget_availableGeometry<QRect> for (&'a  QPoint) {
  fn availableGeometry(self , rsthis: &mut QDesktopWidget) -> QRect {
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
impl<'a> /*trait*/ QDesktopWidget_availableGeometry<QRect> for (i32) {
  fn availableGeometry(self , rsthis: &mut QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QMetaObject * QDesktopWidget::metaObject();
impl /*struct*/ QDesktopWidget {
  pub fn metaObject<RetType, T: QDesktopWidget_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDesktopWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  const QMetaObject * QDesktopWidget::metaObject();
impl<'a> /*trait*/ QDesktopWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10metaObjectEv()};
     unsafe {_ZNK14QDesktopWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QDesktopWidget::workAreaResized(int );
impl /*struct*/ QDesktopWidget {
  pub fn workAreaResized<RetType, T: QDesktopWidget_workAreaResized<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.workAreaResized(self);
    // return 1;
  }
}

pub trait QDesktopWidget_workAreaResized<RetType> {
  fn workAreaResized(self , rsthis: &mut QDesktopWidget) -> RetType;
}

// proto:  void QDesktopWidget::workAreaResized(int );
impl<'a> /*trait*/ QDesktopWidget_workAreaResized<()> for (i32) {
  fn workAreaResized(self , rsthis: &mut QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget15workAreaResizedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget15workAreaResizedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

