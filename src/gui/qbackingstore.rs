// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
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
use super::qpaintdevice::*; // 773
use super::qwindow::*; // 773
use super::super::core::qsize::*; // 771
use super::qregion::*; // 773
use super::super::core::qpoint::*; // 771
// use super::qplatformbackingstore::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QBackingStore_Class_Size() -> c_int;
  // proto:  QPaintDevice * QBackingStore::paintDevice();
  fn C_ZN13QBackingStore11paintDeviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWindow * QBackingStore::window();
  fn C_ZNK13QBackingStore6windowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QBackingStore::size();
  fn C_ZNK13QBackingStore4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRegion QBackingStore::staticContents();
  fn C_ZNK13QBackingStore14staticContentsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QBackingStore::~QBackingStore();
  fn C_ZN13QBackingStoreD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
  fn C_ZN13QBackingStore17setStaticContentsERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBackingStore::resize(const QSize & size);
  fn C_ZN13QBackingStore6resizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
  fn C_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QBackingStore::beginPaint(const QRegion & );
  fn C_ZN13QBackingStore10beginPaintERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QBackingStore::hasStaticContents();
  fn C_ZNK13QBackingStore17hasStaticContentsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QBackingStore::endPaint();
  fn C_ZN13QBackingStore8endPaintEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
  fn C_ZN13QBackingStore6scrollERK7QRegionii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> c_char;
  // proto:  QPlatformBackingStore * QBackingStore::handle();
  fn C_ZNK13QBackingStore6handleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QBackingStore::QBackingStore(QWindow * window);
  fn C_ZN13QBackingStoreC2EP7QWindow(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QBackingStore)=1
#[derive(Default)]
pub struct QBackingStore {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QBackingStore {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QBackingStore {
    return QBackingStore{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QPaintDevice * QBackingStore::paintDevice();
impl /*struct*/ QBackingStore {
  pub fn paintDevice<RetType, T: QBackingStore_paintDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintDevice(self);
    // return 1;
  }
}

pub trait QBackingStore_paintDevice<RetType> {
  fn paintDevice(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  QPaintDevice * QBackingStore::paintDevice();
impl<'a> /*trait*/ QBackingStore_paintDevice<QPaintDevice> for () {
  fn paintDevice(self , rsthis: & QBackingStore) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore11paintDeviceEv()};
    let mut ret = unsafe {C_ZN13QBackingStore11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWindow * QBackingStore::window();
impl /*struct*/ QBackingStore {
  pub fn window<RetType, T: QBackingStore_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QBackingStore_window<RetType> {
  fn window(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  QWindow * QBackingStore::window();
impl<'a> /*trait*/ QBackingStore_window<QWindow> for () {
  fn window(self , rsthis: & QBackingStore) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore6windowEv()};
    let mut ret = unsafe {C_ZNK13QBackingStore6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QBackingStore::size();
impl /*struct*/ QBackingStore {
  pub fn size<RetType, T: QBackingStore_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QBackingStore_size<RetType> {
  fn size(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  QSize QBackingStore::size();
impl<'a> /*trait*/ QBackingStore_size<QSize> for () {
  fn size(self , rsthis: & QBackingStore) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore4sizeEv()};
    let mut ret = unsafe {C_ZNK13QBackingStore4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QBackingStore::staticContents();
impl /*struct*/ QBackingStore {
  pub fn staticContents<RetType, T: QBackingStore_staticContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.staticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_staticContents<RetType> {
  fn staticContents(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  QRegion QBackingStore::staticContents();
impl<'a> /*trait*/ QBackingStore_staticContents<QRegion> for () {
  fn staticContents(self , rsthis: & QBackingStore) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore14staticContentsEv()};
    let mut ret = unsafe {C_ZNK13QBackingStore14staticContentsEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QBackingStore::~QBackingStore();
impl /*struct*/ QBackingStore {
  pub fn free<RetType, T: QBackingStore_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QBackingStore_free<RetType> {
  fn free(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::~QBackingStore();
impl<'a> /*trait*/ QBackingStore_free<()> for () {
  fn free(self , rsthis: & QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStoreD2Ev()};
     unsafe {C_ZN13QBackingStoreD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
impl /*struct*/ QBackingStore {
  pub fn setStaticContents<RetType, T: QBackingStore_setStaticContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStaticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_setStaticContents<RetType> {
  fn setStaticContents(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
impl<'a> /*trait*/ QBackingStore_setStaticContents<()> for (&'a QRegion) {
  fn setStaticContents(self , rsthis: & QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore17setStaticContentsERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QBackingStore17setStaticContentsERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBackingStore::resize(const QSize & size);
impl /*struct*/ QBackingStore {
  pub fn resize<RetType, T: QBackingStore_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QBackingStore_resize<RetType> {
  fn resize(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::resize(const QSize & size);
impl<'a> /*trait*/ QBackingStore_resize<()> for (&'a QSize) {
  fn resize(self , rsthis: & QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QBackingStore6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
impl /*struct*/ QBackingStore {
  pub fn flush<RetType, T: QBackingStore_flush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.flush(self);
    // return 1;
  }
}

pub trait QBackingStore_flush<RetType> {
  fn flush(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
impl<'a> /*trait*/ QBackingStore_flush<()> for (&'a QRegion, &'a QWindow, &'a QPoint) {
  fn flush(self , rsthis: & QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QBackingStore::beginPaint(const QRegion & );
impl /*struct*/ QBackingStore {
  pub fn beginPaint<RetType, T: QBackingStore_beginPaint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginPaint(self);
    // return 1;
  }
}

pub trait QBackingStore_beginPaint<RetType> {
  fn beginPaint(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::beginPaint(const QRegion & );
impl<'a> /*trait*/ QBackingStore_beginPaint<()> for (&'a QRegion) {
  fn beginPaint(self , rsthis: & QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore10beginPaintERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QBackingStore10beginPaintERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QBackingStore::hasStaticContents();
impl /*struct*/ QBackingStore {
  pub fn hasStaticContents<RetType, T: QBackingStore_hasStaticContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasStaticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_hasStaticContents<RetType> {
  fn hasStaticContents(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  bool QBackingStore::hasStaticContents();
impl<'a> /*trait*/ QBackingStore_hasStaticContents<i8> for () {
  fn hasStaticContents(self , rsthis: & QBackingStore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore17hasStaticContentsEv()};
    let mut ret = unsafe {C_ZNK13QBackingStore17hasStaticContentsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QBackingStore::endPaint();
impl /*struct*/ QBackingStore {
  pub fn endPaint<RetType, T: QBackingStore_endPaint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endPaint(self);
    // return 1;
  }
}

pub trait QBackingStore_endPaint<RetType> {
  fn endPaint(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  void QBackingStore::endPaint();
impl<'a> /*trait*/ QBackingStore_endPaint<()> for () {
  fn endPaint(self , rsthis: & QBackingStore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore8endPaintEv()};
     unsafe {C_ZN13QBackingStore8endPaintEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
impl /*struct*/ QBackingStore {
  pub fn scroll<RetType, T: QBackingStore_scroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scroll(self);
    // return 1;
  }
}

pub trait QBackingStore_scroll<RetType> {
  fn scroll(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
impl<'a> /*trait*/ QBackingStore_scroll<i8> for (&'a QRegion, i32, i32) {
  fn scroll(self , rsthis: & QBackingStore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore6scrollERK7QRegionii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN13QBackingStore6scrollERK7QRegionii(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QPlatformBackingStore * QBackingStore::handle();
impl /*struct*/ QBackingStore {
  pub fn handle<RetType, T: QBackingStore_handle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handle(self);
    // return 1;
  }
}

pub trait QBackingStore_handle<RetType> {
  fn handle(self , rsthis: & QBackingStore) -> RetType;
}

  // proto:  QPlatformBackingStore * QBackingStore::handle();
impl<'a> /*trait*/ QBackingStore_handle<u64> for () {
  fn handle(self , rsthis: & QBackingStore) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore6handleEv()};
    let mut ret = unsafe {C_ZNK13QBackingStore6handleEv(rsthis.qclsinst)};
    return ret as u64; // 4
    // return 1;
  }
}

  // proto:  void QBackingStore::QBackingStore(QWindow * window);
impl /*struct*/ QBackingStore {
  pub fn new<T: QBackingStore_new>(value: T) -> QBackingStore {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QBackingStore_new {
  fn new(self) -> QBackingStore;
}

  // proto:  void QBackingStore::QBackingStore(QWindow * window);
impl<'a> /*trait*/ QBackingStore_new for (&'a QWindow) {
  fn new(self) -> QBackingStore {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStoreC2EP7QWindow()};
    let ctysz: c_int = unsafe{QBackingStore_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QBackingStoreC2EP7QWindow(arg0)};
    let rsthis = QBackingStore{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

