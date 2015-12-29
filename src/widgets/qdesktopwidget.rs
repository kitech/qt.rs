// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDesktopWidget_Class_Size() -> c_int;
  // proto:  const QRect QDesktopWidget::screenGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget14screenGeometryERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QDesktopWidget::screen(int screen);
  fn _ZN14QDesktopWidget6screenEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::screenGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget14screenGeometryEPK7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDesktopWidget::screenCountChanged(int );
  fn _ZN14QDesktopWidget18screenCountChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QDesktopWidget::numScreens();
  fn _ZNK14QDesktopWidget10numScreensEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDesktopWidget::~QDesktopWidget();
  fn _ZN14QDesktopWidgetD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QRect QDesktopWidget::screenGeometry(int screen);
  fn _ZNK14QDesktopWidget14screenGeometryEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::availableGeometry(const QWidget * widget);
  fn _ZNK14QDesktopWidget17availableGeometryEPK7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QDesktopWidget::QDesktopWidget(const QDesktopWidget & );
  fn dector_ZN14QDesktopWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN14QDesktopWidgetC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDesktopWidget::resized(int );
  fn _ZN14QDesktopWidget7resizedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QDesktopWidget::screenNumber(const QPoint & );
  fn _ZNK14QDesktopWidget12screenNumberERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QDesktopWidget::screenCount();
  fn _ZNK14QDesktopWidget11screenCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QDesktopWidget::isVirtualDesktop();
  fn _ZNK14QDesktopWidget16isVirtualDesktopEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QDesktopWidget::screenNumber(const QWidget * widget);
  fn _ZNK14QDesktopWidget12screenNumberEPK7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QDesktopWidget::primaryScreen();
  fn _ZNK14QDesktopWidget13primaryScreenEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QDesktopWidget::QDesktopWidget();
  fn dector_ZN14QDesktopWidgetC1Ev() -> *mut c_void;
  fn _ZN14QDesktopWidgetC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QRect QDesktopWidget::availableGeometry(const QPoint & point);
  fn _ZNK14QDesktopWidget17availableGeometryERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QRect QDesktopWidget::availableGeometry(int screen);
  fn _ZNK14QDesktopWidget17availableGeometryEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QDesktopWidget::metaObject();
  fn _ZNK14QDesktopWidget10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDesktopWidget::workAreaResized(int );
  fn _ZN14QDesktopWidget15workAreaResizedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget18screenCountChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDesktopWidget_SlotProxy_connect_box__ZN14QDesktopWidget18screenCountChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget7resizedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDesktopWidget_SlotProxy_connect_box__ZN14QDesktopWidget7resizedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget15workAreaResizedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDesktopWidget_SlotProxy_connect_box__ZN14QDesktopWidget15workAreaResizedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDesktopWidget)=1
#[derive(Default)]
pub struct QDesktopWidget {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _screenCountChanged_1: QDesktopWidget_screenCountChanged_signal,
  pub _resized_1: QDesktopWidget_resized_signal,
  pub _workAreaResized_1: QDesktopWidget_workAreaResized_signal,
}

impl /*struct*/ QDesktopWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDesktopWidget {
    return QDesktopWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret1 = QWidget::inheritFrom(ret as u64);
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
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QDesktopWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN14QDesktopWidgetC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN14QDesktopWidgetC1ERKS_(arg0)} as u64;
    let rsthis = QDesktopWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QDesktopWidgetC1Ev()};
    let ctysz: c_int = unsafe{QDesktopWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN14QDesktopWidgetC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN14QDesktopWidgetC1Ev()} as u64;
    let rsthis = QDesktopWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret1 = QRect::inheritFrom(ret as u64);
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
    let mut ret1 = QRect::inheritFrom(ret as u64);
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

#[derive(Default)] // for QDesktopWidget_screenCountChanged
pub struct QDesktopWidget_screenCountChanged_signal{poi:u64}
impl /* struct */ QDesktopWidget {
  pub fn screenCountChanged_1(&self) -> QDesktopWidget_screenCountChanged_signal {
     return QDesktopWidget_screenCountChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDesktopWidget_screenCountChanged_signal {
  pub fn connect<T: QDesktopWidget_screenCountChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDesktopWidget_screenCountChanged_signal_connect {
  fn connect(self, sigthis: QDesktopWidget_screenCountChanged_signal);
}

#[derive(Default)] // for QDesktopWidget_resized
pub struct QDesktopWidget_resized_signal{poi:u64}
impl /* struct */ QDesktopWidget {
  pub fn resized_1(&self) -> QDesktopWidget_resized_signal {
     return QDesktopWidget_resized_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDesktopWidget_resized_signal {
  pub fn connect<T: QDesktopWidget_resized_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDesktopWidget_resized_signal_connect {
  fn connect(self, sigthis: QDesktopWidget_resized_signal);
}

#[derive(Default)] // for QDesktopWidget_workAreaResized
pub struct QDesktopWidget_workAreaResized_signal{poi:u64}
impl /* struct */ QDesktopWidget {
  pub fn workAreaResized_1(&self) -> QDesktopWidget_workAreaResized_signal {
     return QDesktopWidget_workAreaResized_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDesktopWidget_workAreaResized_signal {
  pub fn connect<T: QDesktopWidget_workAreaResized_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDesktopWidget_workAreaResized_signal_connect {
  fn connect(self, sigthis: QDesktopWidget_workAreaResized_signal);
}

// screenCountChanged(int)
extern fn QDesktopWidget_screenCountChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QDesktopWidget_screenCountChanged_signal_connect_cb_box_0(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QDesktopWidget_screenCountChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDesktopWidget_screenCountChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDesktopWidget_screenCountChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget18screenCountChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDesktopWidget_screenCountChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QDesktopWidget_screenCountChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDesktopWidget_screenCountChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget18screenCountChangedEi(arg0, arg1, arg2)};
  }
}
// resized(int)
extern fn QDesktopWidget_resized_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QDesktopWidget_resized_signal_connect_cb_box_1(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QDesktopWidget_resized_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDesktopWidget_resized_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDesktopWidget_resized_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget7resizedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDesktopWidget_resized_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QDesktopWidget_resized_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDesktopWidget_resized_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget7resizedEi(arg0, arg1, arg2)};
  }
}
// workAreaResized(int)
extern fn QDesktopWidget_workAreaResized_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
extern fn QDesktopWidget_workAreaResized_signal_connect_cb_box_2(rsfptr_raw:*mut c_void, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
}
impl /* trait */ QDesktopWidget_workAreaResized_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDesktopWidget_workAreaResized_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDesktopWidget_workAreaResized_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget15workAreaResizedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDesktopWidget_workAreaResized_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QDesktopWidget_workAreaResized_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDesktopWidget_workAreaResized_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDesktopWidget_SlotProxy_connect__ZN14QDesktopWidget15workAreaResizedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

