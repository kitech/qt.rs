// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpaintdevice::QPaintDevice;
use super::qwindow::QWindow;
use super::qsize::QSize;
use super::qregion::QRegion;
use super::qpoint::QPoint;

// ext block begin
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
  // proto:  void QBackingStore::FreeQBackingStore();
  fn _ZN13QBackingStoreD0Ev(qthis: *mut c_void) ;
  // proto:  void QBackingStore::setStaticContents(const QRegion & region);
  fn _ZN13QBackingStore17setStaticContentsERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBackingStore::resize(const QSize & size);
  fn _ZN13QBackingStore6resizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
  fn _ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QBackingStore::beginPaint(const QRegion & );
  fn _ZN13QBackingStore10beginPaintERK7QRegion(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QBackingStore::hasStaticContents();
  fn _ZNK13QBackingStore17hasStaticContentsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBackingStore::endPaint();
  fn _ZN13QBackingStore8endPaintEv(qthis: *mut c_void) ;
  // proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
  fn _ZN13QBackingStore6scrollERK7QRegionii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) -> int8_t;
  // proto:  QPlatformBackingStore * QBackingStore::handle();
  fn _ZNK13QBackingStore6handleEv(qthis: *mut c_void) ;
  // proto:  void QBackingStore::NewQBackingStore(QWindow * window);
  fn _ZN13QBackingStoreC1EP7QWindow(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QBackingStore)=1
pub struct QBackingStore {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBackingStore {
  pub fn paintDevice<T: QBackingStore_paintDevice>(&mut self, value: T) -> QPaintDevice {
    return value.paintDevice(self);
    // return 1;
  }
}

pub trait QBackingStore_paintDevice {
  fn paintDevice(self, rsthis: &mut QBackingStore) -> QPaintDevice;
}

// proto:  QPaintDevice * QBackingStore::paintDevice();
impl<'a> /*trait*/ QBackingStore_paintDevice for () {
  fn paintDevice(self, rsthis: &mut QBackingStore) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore11paintDeviceEv()};
    let mut ret = unsafe {_ZN13QBackingStore11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn window<T: QBackingStore_window>(&mut self, value: T) -> QWindow {
    return value.window(self);
    // return 1;
  }
}

pub trait QBackingStore_window {
  fn window(self, rsthis: &mut QBackingStore) -> QWindow;
}

// proto:  QWindow * QBackingStore::window();
impl<'a> /*trait*/ QBackingStore_window for () {
  fn window(self, rsthis: &mut QBackingStore) -> QWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore6windowEv()};
    let mut ret = unsafe {_ZNK13QBackingStore6windowEv(rsthis.qclsinst)};
    let mut ret1 = QWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn size<T: QBackingStore_size>(&mut self, value: T) -> QSize {
    return value.size(self);
    // return 1;
  }
}

pub trait QBackingStore_size {
  fn size(self, rsthis: &mut QBackingStore) -> QSize;
}

// proto:  QSize QBackingStore::size();
impl<'a> /*trait*/ QBackingStore_size for () {
  fn size(self, rsthis: &mut QBackingStore) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore4sizeEv()};
    let mut ret = unsafe {_ZNK13QBackingStore4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn staticContents<T: QBackingStore_staticContents>(&mut self, value: T) -> QRegion {
    return value.staticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_staticContents {
  fn staticContents(self, rsthis: &mut QBackingStore) -> QRegion;
}

// proto:  QRegion QBackingStore::staticContents();
impl<'a> /*trait*/ QBackingStore_staticContents for () {
  fn staticContents(self, rsthis: &mut QBackingStore) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore14staticContentsEv()};
    let mut ret = unsafe {_ZNK13QBackingStore14staticContentsEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn FreeQBackingStore<T: QBackingStore_FreeQBackingStore>(&mut self, value: T)  {
     value.FreeQBackingStore(self);
    // return 1;
  }
}

pub trait QBackingStore_FreeQBackingStore {
  fn FreeQBackingStore(self, rsthis: &mut QBackingStore) ;
}

// proto:  void QBackingStore::FreeQBackingStore();
impl<'a> /*trait*/ QBackingStore_FreeQBackingStore for () {
  fn FreeQBackingStore(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStoreD0Ev()};
     unsafe {_ZN13QBackingStoreD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn setStaticContents<T: QBackingStore_setStaticContents>(&mut self, value: T)  {
     value.setStaticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_setStaticContents {
  fn setStaticContents(self, rsthis: &mut QBackingStore) ;
}

// proto:  void QBackingStore::setStaticContents(const QRegion & region);
impl<'a> /*trait*/ QBackingStore_setStaticContents for (&'a  QRegion) {
  fn setStaticContents(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore17setStaticContentsERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore17setStaticContentsERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn resize<T: QBackingStore_resize>(&mut self, value: T)  {
     value.resize(self);
    // return 1;
  }
}

pub trait QBackingStore_resize {
  fn resize(self, rsthis: &mut QBackingStore) ;
}

// proto:  void QBackingStore::resize(const QSize & size);
impl<'a> /*trait*/ QBackingStore_resize for (&'a  QSize) {
  fn resize(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn flush<T: QBackingStore_flush>(&mut self, value: T)  {
     value.flush(self);
    // return 1;
  }
}

pub trait QBackingStore_flush {
  fn flush(self, rsthis: &mut QBackingStore) ;
}

// proto:  void QBackingStore::flush(const QRegion & region, QWindow * window, const QPoint & offset);
impl<'a> /*trait*/ QBackingStore_flush for (&'a  QRegion, &'a mut QWindow, &'a  QPoint) {
  fn flush(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore5flushERK7QRegionP7QWindowRK6QPoint(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn beginPaint<T: QBackingStore_beginPaint>(&mut self, value: T)  {
     value.beginPaint(self);
    // return 1;
  }
}

pub trait QBackingStore_beginPaint {
  fn beginPaint(self, rsthis: &mut QBackingStore) ;
}

// proto:  void QBackingStore::beginPaint(const QRegion & );
impl<'a> /*trait*/ QBackingStore_beginPaint for (&'a  QRegion) {
  fn beginPaint(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore10beginPaintERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QBackingStore10beginPaintERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn hasStaticContents<T: QBackingStore_hasStaticContents>(&mut self, value: T) -> i8 {
    return value.hasStaticContents(self);
    // return 1;
  }
}

pub trait QBackingStore_hasStaticContents {
  fn hasStaticContents(self, rsthis: &mut QBackingStore) -> i8;
}

// proto:  bool QBackingStore::hasStaticContents();
impl<'a> /*trait*/ QBackingStore_hasStaticContents for () {
  fn hasStaticContents(self, rsthis: &mut QBackingStore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore17hasStaticContentsEv()};
    let mut ret = unsafe {_ZNK13QBackingStore17hasStaticContentsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn endPaint<T: QBackingStore_endPaint>(&mut self, value: T)  {
     value.endPaint(self);
    // return 1;
  }
}

pub trait QBackingStore_endPaint {
  fn endPaint(self, rsthis: &mut QBackingStore) ;
}

// proto:  void QBackingStore::endPaint();
impl<'a> /*trait*/ QBackingStore_endPaint for () {
  fn endPaint(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QBackingStore8endPaintEv()};
     unsafe {_ZN13QBackingStore8endPaintEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBackingStore {
  pub fn scroll<T: QBackingStore_scroll>(&mut self, value: T) -> i8 {
    return value.scroll(self);
    // return 1;
  }
}

pub trait QBackingStore_scroll {
  fn scroll(self, rsthis: &mut QBackingStore) -> i8;
}

// proto:  bool QBackingStore::scroll(const QRegion & area, int dx, int dy);
impl<'a> /*trait*/ QBackingStore_scroll for (&'a  QRegion, i32, i32) {
  fn scroll(self, rsthis: &mut QBackingStore) -> i8 {
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

impl /*struct*/ QBackingStore {
  pub fn handle<T: QBackingStore_handle>(&mut self, value: T)  {
     value.handle(self);
    // return 1;
  }
}

pub trait QBackingStore_handle {
  fn handle(self, rsthis: &mut QBackingStore) ;
}

// proto:  QPlatformBackingStore * QBackingStore::handle();
impl<'a> /*trait*/ QBackingStore_handle for () {
  fn handle(self, rsthis: &mut QBackingStore)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QBackingStore6handleEv()};
     unsafe {_ZNK13QBackingStore6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

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

// proto: void QBackingStore::NewQBackingStore(QWindow * window);
impl<'a> /*trait*/ QBackingStore_NewQBackingStore for (&'a mut QWindow) {
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

