// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtWidgets/qdesktopwidget.h
// dst-file: /src/widgets/qdesktopwidget.rs
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
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
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
  fn _ZN14QDesktopWidget18screenCountChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QDesktopWidget::numScreens();
  fn _ZNK14QDesktopWidget10numScreensEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDesktopWidget::~QDesktopWidget();
  fn _ZN14QDesktopWidgetD0Ev(qthis: *mut c_void);
  // proto:  const QRect QDesktopWidget::screenGeometry(int screen);
  fn _ZNK14QDesktopWidget14screenGeometryEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget17availableGeometryEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDesktopWidget::QDesktopWidget(const QDesktopWidget & );
  fn _ZN14QDesktopWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QDesktopWidget::resized(int );
  fn _ZN14QDesktopWidget7resizedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QDesktopWidget::screenNumber(const QPoint & );
  fn _ZNK14QDesktopWidget12screenNumberERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QDesktopWidget::screenCount();
  fn _ZNK14QDesktopWidget11screenCountEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QDesktopWidget::isVirtualDesktop();
  fn _ZNK14QDesktopWidget16isVirtualDesktopEv(qthis: *mut c_void) -> c_char;
  // proto:  int QDesktopWidget::screenNumber(const QWidget * widget);
  fn _ZNK14QDesktopWidget12screenNumberEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QDesktopWidget::primaryScreen();
  fn _ZNK14QDesktopWidget13primaryScreenEv(qthis: *mut c_void) -> c_int;
  // proto:  void QDesktopWidget::QDesktopWidget();
  fn _ZN14QDesktopWidgetC1Ev(qthis: *mut c_void);
  // proto:  const QRect QDesktopWidget::availableGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget17availableGeometryERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::availableGeometry(int screen);
  fn _ZNK14QDesktopWidget17availableGeometryEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QDesktopWidget::metaObject();
  fn _ZNK14QDesktopWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  void QDesktopWidget::workAreaResized(int );
  fn _ZN14QDesktopWidget15workAreaResizedEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QDesktopWidget)=1
pub struct QDesktopWidget {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDesktopWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QDesktopWidget {
    return QDesktopWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QDesktopWidget {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QDesktopWidget {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
impl /*struct*/ QDesktopWidget {
  pub fn screenGeometry<RetType, T: QDesktopWidget_screenGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenGeometry(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenGeometry<RetType> {
  fn screenGeometry(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry<QRect> for (&'a QPoint) {
  fn screenGeometry(self , rsthis: & QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QDesktopWidget::screen(int screen);
impl /*struct*/ QDesktopWidget {
  pub fn screen<RetType, T: QDesktopWidget_screen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screen(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screen<RetType> {
  fn screen(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  QWidget * QDesktopWidget::screen(int screen);
impl<'a> /*trait*/ QDesktopWidget_screen<QWidget> for (i32) {
  fn screen(self , rsthis: & QDesktopWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget6screenEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN14QDesktopWidget6screenEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QRect QDesktopWidget::screenGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry<QRect> for (&'a QWidget) {
  fn screenGeometry(self , rsthis: & QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDesktopWidget::screenCountChanged(int );
impl /*struct*/ QDesktopWidget {
  pub fn screenCountChanged<RetType, T: QDesktopWidget_screenCountChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenCountChanged(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenCountChanged<RetType> {
  fn screenCountChanged(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  void QDesktopWidget::screenCountChanged(int );
impl<'a> /*trait*/ QDesktopWidget_screenCountChanged<()> for (i32) {
  fn screenCountChanged(self , rsthis: & QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget18screenCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget18screenCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDesktopWidget::numScreens();
impl /*struct*/ QDesktopWidget {
  pub fn numScreens<RetType, T: QDesktopWidget_numScreens<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.numScreens(self);
    // return 1;
  }
}

pub trait QDesktopWidget_numScreens<RetType> {
  fn numScreens(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  int QDesktopWidget::numScreens();
impl<'a> /*trait*/ QDesktopWidget_numScreens<i32> for () {
  fn numScreens(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10numScreensEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget10numScreensEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDesktopWidget::~QDesktopWidget();
impl /*struct*/ QDesktopWidget {
  pub fn Free<RetType, T: QDesktopWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDesktopWidget_Free<RetType> {
  fn Free(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  void QDesktopWidget::~QDesktopWidget();
impl<'a> /*trait*/ QDesktopWidget_Free<()> for () {
  fn Free(self , rsthis: & QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetD0Ev()};
     unsafe {_ZN14QDesktopWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QRect QDesktopWidget::screenGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_screenGeometry<QRect> for (i32) {
  fn screenGeometry(self , rsthis: & QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget14screenGeometryEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QDesktopWidget14screenGeometryEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
impl /*struct*/ QDesktopWidget {
  pub fn availableGeometry<RetType, T: QDesktopWidget_availableGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableGeometry(self);
    // return 1;
  }
}

pub trait QDesktopWidget_availableGeometry<RetType> {
  fn availableGeometry(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry<QRect> for (&'a QWidget) {
  fn availableGeometry(self , rsthis: & QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryEPK7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDesktopWidget::QDesktopWidget(const QDesktopWidget & );
impl /*struct*/ QDesktopWidget {
  pub fn New<T: QDesktopWidget_New>(value: T) -> QDesktopWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDesktopWidget_New {
  fn New(self) -> QDesktopWidget;
}

  // proto:  void QDesktopWidget::QDesktopWidget(const QDesktopWidget & );
impl<'a> /*trait*/ QDesktopWidget_New for (&'a QDesktopWidget) {
  fn New(self) -> QDesktopWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QDesktopWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QDesktopWidget{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDesktopWidget::resized(int );
impl /*struct*/ QDesktopWidget {
  pub fn resized<RetType, T: QDesktopWidget_resized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resized(self);
    // return 1;
  }
}

pub trait QDesktopWidget_resized<RetType> {
  fn resized(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  void QDesktopWidget::resized(int );
impl<'a> /*trait*/ QDesktopWidget_resized<()> for (i32) {
  fn resized(self , rsthis: & QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget7resizedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget7resizedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QDesktopWidget::screenNumber(const QPoint & );
impl /*struct*/ QDesktopWidget {
  pub fn screenNumber<RetType, T: QDesktopWidget_screenNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenNumber(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenNumber<RetType> {
  fn screenNumber(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  int QDesktopWidget::screenNumber(const QPoint & );
impl<'a> /*trait*/ QDesktopWidget_screenNumber<i32> for (&'a QPoint) {
  fn screenNumber(self , rsthis: & QDesktopWidget) -> i32 {
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
  pub fn screenCount<RetType, T: QDesktopWidget_screenCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.screenCount(self);
    // return 1;
  }
}

pub trait QDesktopWidget_screenCount<RetType> {
  fn screenCount(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  int QDesktopWidget::screenCount();
impl<'a> /*trait*/ QDesktopWidget_screenCount<i32> for () {
  fn screenCount(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget11screenCountEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget11screenCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QDesktopWidget::isVirtualDesktop();
impl /*struct*/ QDesktopWidget {
  pub fn isVirtualDesktop<RetType, T: QDesktopWidget_isVirtualDesktop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVirtualDesktop(self);
    // return 1;
  }
}

pub trait QDesktopWidget_isVirtualDesktop<RetType> {
  fn isVirtualDesktop(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  bool QDesktopWidget::isVirtualDesktop();
impl<'a> /*trait*/ QDesktopWidget_isVirtualDesktop<i8> for () {
  fn isVirtualDesktop(self , rsthis: & QDesktopWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget16isVirtualDesktopEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget16isVirtualDesktopEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QDesktopWidget::screenNumber(const QWidget * widget);
impl<'a> /*trait*/ QDesktopWidget_screenNumber<i32> for (&'a QWidget) {
  fn screenNumber(self , rsthis: & QDesktopWidget) -> i32 {
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
  pub fn primaryScreen<RetType, T: QDesktopWidget_primaryScreen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.primaryScreen(self);
    // return 1;
  }
}

pub trait QDesktopWidget_primaryScreen<RetType> {
  fn primaryScreen(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  int QDesktopWidget::primaryScreen();
impl<'a> /*trait*/ QDesktopWidget_primaryScreen<i32> for () {
  fn primaryScreen(self , rsthis: & QDesktopWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget13primaryScreenEv()};
    let mut ret = unsafe {_ZNK14QDesktopWidget13primaryScreenEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QDesktopWidget::QDesktopWidget();
impl<'a> /*trait*/ QDesktopWidget_New for () {
  fn New(self) -> QDesktopWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetC1Ev()};
    unsafe {_ZN14QDesktopWidgetC1Ev(qthis)};
    let rsthis = QDesktopWidget{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QRect QDesktopWidget::availableGeometry(const QPoint & point);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry<QRect> for (&'a QPoint) {
  fn availableGeometry(self , rsthis: & QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QRect QDesktopWidget::availableGeometry(int screen);
impl<'a> /*trait*/ QDesktopWidget_availableGeometry<QRect> for (i32) {
  fn availableGeometry(self , rsthis: & QDesktopWidget) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget17availableGeometryEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QDesktopWidget17availableGeometryEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QDesktopWidget::metaObject();
impl /*struct*/ QDesktopWidget {
  pub fn metaObject<RetType, T: QDesktopWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDesktopWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  const QMetaObject * QDesktopWidget::metaObject();
impl<'a> /*trait*/ QDesktopWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QDesktopWidget10metaObjectEv()};
     unsafe {_ZNK14QDesktopWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDesktopWidget::workAreaResized(int );
impl /*struct*/ QDesktopWidget {
  pub fn workAreaResized<RetType, T: QDesktopWidget_workAreaResized<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.workAreaResized(self);
    // return 1;
  }
}

pub trait QDesktopWidget_workAreaResized<RetType> {
  fn workAreaResized(self , rsthis: & QDesktopWidget) -> RetType;
}

  // proto:  void QDesktopWidget::workAreaResized(int );
impl<'a> /*trait*/ QDesktopWidget_workAreaResized<()> for (i32) {
  fn workAreaResized(self , rsthis: & QDesktopWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidget15workAreaResizedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QDesktopWidget15workAreaResizedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

