// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qbackingstore.h
// dst-file: /src/gui/qbackingstore.rs
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
use std::ops::Deref;
use super::qpaintdevice::QPaintDevice; // 773
use super::qwindow::QWindow; // 773
use super::super::core::qsize::QSize; // 771
use super::qregion::QRegion; // 773
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QPaintDevice * QBackingStore::paintDevice();
  fn _ZN13QBackingStore11paintDeviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWindow * QBackingStore::window();
  fn _ZNK13QBackingStore6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QBackingStore::size();
  fn _ZNK13QBackingStore4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRegion QBackingStore::staticContents();
  fn _ZNK13QBackingStore14staticContentsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QBackingStore::~QBackingStore();
  fn _ZN13QBackingStoreD0Ev(qthis: *mut c_void);
  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
  fn _ZN13QBackingStore17setStaticContentsERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBackingStore::resize(const QSize & size);
  fn _ZN13QBackingStore6resizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
  fn _ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QBackingStore::beginPaint(const QRegion & );
  fn _ZN13QBackingStore10beginPaintERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QBackingStore::hasStaticContents();
  fn _ZNK13QBackingStore17hasStaticContentsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QBackingStore::endPaint();
  fn _ZN13QBackingStore8endPaintEv(qthis: *mut c_void);
  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
  fn _ZN13QBackingStore6scrollERK7QRegionii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> c_char;
  // proto:  QPlatformBackingStore * QBackingStore::handle();
  fn _ZNK13QBackingStore6handleEv(qthis: *mut c_void);
  // proto:  void QBackingStore::QBackingStore(QWindow * window);
  fn _ZN13QBackingStoreC1EP7QWindow(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QBackingStore)=1
pub struct QBackingStore {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBackingStore {
  pub fn inheritFrom(qthis: *mut c_void) -> QBackingStore {
    return QBackingStore{qclsinst: qthis};
  }
}
  // proto:  QPaintDevice * QBackingStore::paintDevice();
impl /*struct*/ QBackingStore {
  pub fn paintDevice<RetType, T: QBackingStore_paintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintDevice(self);
    // return 1;
  }
}

pub trait QBackingStore_paintDevice<RetType> {
  fn paintDevice(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  QPaintDevice * QBackingStore::paintDevice();
impl<'a> /*trait*/ QBackingStore_paintDevice<QPaintDevice> for () {
  fn paintDevice(self , rsthis: &mut QBackingStore) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore11paintDeviceEv()};
    let mut ret = unsafe {_ZN13QBackingStore11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWindow * QBackingStore::window();
impl /*struct*/ QBackingStore {
  pub fn window<RetType, T: QBackingStore_window<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QBackingStore_window<RetType> {
  fn window(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  QWindow * QBackingStore::window();
impl<'a> /*trait*/ QBackingStore_window<QWindow> for () {
  fn window(self , rsthis: &mut QBackingStore) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore6windowEv()};
    let mut ret = unsafe {_ZNK13QBackingStore6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QBackingStore::size();
impl /*struct*/ QBackingStore {
  pub fn size<RetType, T: QBackingStore_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QBackingStore_size<RetType> {
  fn size(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  QSize QBackingStore::size();
impl<'a> /*trait*/ QBackingStore_size<QSize> for () {
  fn size(self , rsthis: &mut QBackingStore) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore4sizeEv()};
    let mut ret = unsafe {_ZNK13QBackingStore4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QBackingStore::staticContents();
impl /*struct*/ QBackingStore {
  pub fn staticContents<RetType, T: QBackingStore_staticContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.staticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_staticContents<RetType> {
  fn staticContents(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  QRegion QBackingStore::staticContents();
impl<'a> /*trait*/ QBackingStore_staticContents<QRegion> for () {
  fn staticContents(self , rsthis: &mut QBackingStore) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore14staticContentsEv()};
    let mut ret = unsafe {_ZNK13QBackingStore14staticContentsEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBackingStore::~QBackingStore();
impl /*struct*/ QBackingStore {
  pub fn FreeQBackingStore<RetType, T: QBackingStore_FreeQBackingStore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQBackingStore(self);
    // return 1;
  }
}

pub trait QBackingStore_FreeQBackingStore<RetType> {
  fn FreeQBackingStore(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::~QBackingStore();
impl<'a> /*trait*/ QBackingStore_FreeQBackingStore<()> for () {
  fn FreeQBackingStore(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStoreD0Ev()};
     unsafe {_ZN13QBackingStoreD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
impl /*struct*/ QBackingStore {
  pub fn setStaticContents<RetType, T: QBackingStore_setStaticContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStaticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_setStaticContents<RetType> {
  fn setStaticContents(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
impl<'a> /*trait*/ QBackingStore_setStaticContents<()> for (QRegion) {
  fn setStaticContents(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore17setStaticContentsERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore17setStaticContentsERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBackingStore::resize(const QSize & size);
impl /*struct*/ QBackingStore {
  pub fn resize<RetType, T: QBackingStore_resize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QBackingStore_resize<RetType> {
  fn resize(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::resize(const QSize & size);
impl<'a> /*trait*/ QBackingStore_resize<()> for (QSize) {
  fn resize(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
impl /*struct*/ QBackingStore {
  pub fn flush<RetType, T: QBackingStore_flush<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QBackingStore_flush<RetType> {
  fn flush(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
impl<'a> /*trait*/ QBackingStore_flush<()> for (QRegion, QWindow, QPoint) {
  fn flush(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QBackingStore::beginPaint(const QRegion & );
impl /*struct*/ QBackingStore {
  pub fn beginPaint<RetType, T: QBackingStore_beginPaint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.beginPaint(self);
    // return 1;
  }
}

pub trait QBackingStore_beginPaint<RetType> {
  fn beginPaint(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::beginPaint(const QRegion & );
impl<'a> /*trait*/ QBackingStore_beginPaint<()> for (QRegion) {
  fn beginPaint(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore10beginPaintERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore10beginPaintERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBackingStore::hasStaticContents();
impl /*struct*/ QBackingStore {
  pub fn hasStaticContents<RetType, T: QBackingStore_hasStaticContents<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasStaticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_hasStaticContents<RetType> {
  fn hasStaticContents(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  bool QBackingStore::hasStaticContents();
impl<'a> /*trait*/ QBackingStore_hasStaticContents<i8> for () {
  fn hasStaticContents(self , rsthis: &mut QBackingStore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore17hasStaticContentsEv()};
    let mut ret = unsafe {_ZNK13QBackingStore17hasStaticContentsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QBackingStore::endPaint();
impl /*struct*/ QBackingStore {
  pub fn endPaint<RetType, T: QBackingStore_endPaint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.endPaint(self);
    // return 1;
  }
}

pub trait QBackingStore_endPaint<RetType> {
  fn endPaint(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::endPaint();
impl<'a> /*trait*/ QBackingStore_endPaint<()> for () {
  fn endPaint(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore8endPaintEv()};
     unsafe {_ZN13QBackingStore8endPaintEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
impl /*struct*/ QBackingStore {
  pub fn scroll<RetType, T: QBackingStore_scroll<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.scroll(self);
    // return 1;
  }
}

pub trait QBackingStore_scroll<RetType> {
  fn scroll(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
impl<'a> /*trait*/ QBackingStore_scroll<i8> for (QRegion, i32, i32) {
  fn scroll(self , rsthis: &mut QBackingStore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore6scrollERK7QRegionii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN13QBackingStore6scrollERK7QRegionii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPlatformBackingStore * QBackingStore::handle();
impl /*struct*/ QBackingStore {
  pub fn handle<RetType, T: QBackingStore_handle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QBackingStore_handle<RetType> {
  fn handle(self , rsthis: &mut QBackingStore) -> RetType;
}

  // proto:  QPlatformBackingStore * QBackingStore::handle();
impl<'a> /*trait*/ QBackingStore_handle<()> for () {
  fn handle(self , rsthis: &mut QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore6handleEv()};
     unsafe {_ZNK13QBackingStore6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBackingStore::QBackingStore(QWindow * window);
impl /*struct*/ QBackingStore {
  pub fn NewQBackingStore<T: QBackingStore_NewQBackingStore>(value: T) -> QBackingStore {
    let rsthis = value.NewQBackingStore();
    return rsthis;
    // return 1;
  }
}

pub trait QBackingStore_NewQBackingStore {
  fn NewQBackingStore(self) -> QBackingStore;
}

  // proto:  void QBackingStore::QBackingStore(QWindow * window);
impl<'a> /*trait*/ QBackingStore_NewQBackingStore for (QWindow) {
  fn NewQBackingStore(self) -> QBackingStore {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStoreC1EP7QWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QBackingStoreC1EP7QWindow(qthis, arg0)};
    let rsthis = QBackingStore{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

