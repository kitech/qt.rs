// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qpoint::QPoint;
use super::qmimedata::QMimeData;
use super::qpixmap::QPixmap;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QObject * QDrag::target();
  fn _ZNK5QDrag6targetEv() -> i32;
  // proto: QMimeData * QDrag::mimeData();
  fn _ZNK5QDrag8mimeDataEv() -> i32;
  // proto: void QDrag::NewQDrag(QObject * dragSource);
  fn _ZN5QDragC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QDrag::FreeQDrag();
  fn _ZN5QDragD0Ev() -> i32;
  // proto: void QDrag::NewQDrag(const QDrag & );
  fn _ZN5QDragC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QDrag::setHotSpot(const QPoint & hotspot);
  fn _ZN5QDrag10setHotSpotERK6QPoint(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QDrag::metaObject();
  fn _ZNK5QDrag10metaObjectEv() -> i32;
  // proto: void QDrag::setMimeData(QMimeData * data);
  fn _ZN5QDrag11setMimeDataEP9QMimeData(arg0: *mut c_void) -> i32;
  // proto: QPixmap QDrag::pixmap();
  fn _ZNK5QDrag6pixmapEv() -> i32;
  // proto: QPoint QDrag::hotSpot();
  fn _ZNK5QDrag7hotSpotEv() -> i32;
  // proto: void QDrag::setPixmap(const QPixmap & );
  fn _ZN5QDrag9setPixmapERK7QPixmap(arg0: *const c_void) -> i32;
  // proto: QObject * QDrag::source();
  fn _ZNK5QDrag6sourceEv() -> i32;
  // proto: void QDrag::targetChanged(QObject * newTarget);
  fn _ZN5QDrag13targetChangedEP7QObject(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QDrag)=1
pub struct QDrag {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDrag {
  pub fn target<T: QDrag_target>(&mut self, value: T) -> i32 {
    value.target(self);
    return 1;
  }
}

pub trait QDrag_target {
  fn target(self, this: &mut QDrag) -> i32;
}

// proto: QObject * QDrag::target();
impl<'a> /*trait*/ QDrag_target for () {
  fn target(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6targetEv()};
    unsafe {_ZNK5QDrag6targetEv()};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn mimeData<T: QDrag_mimeData>(&mut self, value: T) -> i32 {
    value.mimeData(self);
    return 1;
  }
}

pub trait QDrag_mimeData {
  fn mimeData(self, this: &mut QDrag) -> i32;
}

// proto: QMimeData * QDrag::mimeData();
impl<'a> /*trait*/ QDrag_mimeData for () {
  fn mimeData(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag8mimeDataEv()};
    unsafe {_ZNK5QDrag8mimeDataEv()};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn NewQDrag<T: QDrag_NewQDrag>(value: T) -> QDrag {
    let rsthis = value.NewQDrag();
    return rsthis;
    // return 1;
  }
}

pub trait QDrag_NewQDrag {
  fn NewQDrag(self) -> QDrag;
}

// proto: void QDrag::NewQDrag(QObject * dragSource);
impl<'a> /*trait*/ QDrag_NewQDrag for (&'a mut QObject) {
  fn NewQDrag(self) -> QDrag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDragC1EP7QObject(qthis, arg0)};
    let rsthis = QDrag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn FreeQDrag<T: QDrag_FreeQDrag>(&mut self, value: T) -> i32 {
    value.FreeQDrag(self);
    return 1;
  }
}

pub trait QDrag_FreeQDrag {
  fn FreeQDrag(self, this: &mut QDrag) -> i32;
}

// proto: void QDrag::FreeQDrag();
impl<'a> /*trait*/ QDrag_FreeQDrag for () {
  fn FreeQDrag(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragD0Ev()};
    unsafe {_ZN5QDragD0Ev()};
    return 1;
  }
}

// proto: void QDrag::NewQDrag(const QDrag & );
impl<'a> /*trait*/ QDrag_NewQDrag for (&'a  QDrag) {
  fn NewQDrag(self) -> QDrag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QDragC1ERKS_(qthis, arg0)};
    let rsthis = QDrag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn setHotSpot<T: QDrag_setHotSpot>(&mut self, value: T) -> i32 {
    value.setHotSpot(self);
    return 1;
  }
}

pub trait QDrag_setHotSpot {
  fn setHotSpot(self, this: &mut QDrag) -> i32;
}

// proto: void QDrag::setHotSpot(const QPoint & hotspot);
impl<'a> /*trait*/ QDrag_setHotSpot for (&'a  QPoint) {
  fn setHotSpot(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag10setHotSpotERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QDrag10setHotSpotERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn metaObject<T: QDrag_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QDrag_metaObject {
  fn metaObject(self, this: &mut QDrag) -> i32;
}

// proto: const QMetaObject * QDrag::metaObject();
impl<'a> /*trait*/ QDrag_metaObject for () {
  fn metaObject(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag10metaObjectEv()};
    unsafe {_ZNK5QDrag10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn setMimeData<T: QDrag_setMimeData>(&mut self, value: T) -> i32 {
    value.setMimeData(self);
    return 1;
  }
}

pub trait QDrag_setMimeData {
  fn setMimeData(self, this: &mut QDrag) -> i32;
}

// proto: void QDrag::setMimeData(QMimeData * data);
impl<'a> /*trait*/ QDrag_setMimeData for (&'a mut QMimeData) {
  fn setMimeData(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag11setMimeDataEP9QMimeData()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDrag11setMimeDataEP9QMimeData(arg0)};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn pixmap<T: QDrag_pixmap>(&mut self, value: T) -> i32 {
    value.pixmap(self);
    return 1;
  }
}

pub trait QDrag_pixmap {
  fn pixmap(self, this: &mut QDrag) -> i32;
}

// proto: QPixmap QDrag::pixmap();
impl<'a> /*trait*/ QDrag_pixmap for () {
  fn pixmap(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6pixmapEv()};
    unsafe {_ZNK5QDrag6pixmapEv()};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn hotSpot<T: QDrag_hotSpot>(&mut self, value: T) -> i32 {
    value.hotSpot(self);
    return 1;
  }
}

pub trait QDrag_hotSpot {
  fn hotSpot(self, this: &mut QDrag) -> i32;
}

// proto: QPoint QDrag::hotSpot();
impl<'a> /*trait*/ QDrag_hotSpot for () {
  fn hotSpot(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag7hotSpotEv()};
    unsafe {_ZNK5QDrag7hotSpotEv()};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn setPixmap<T: QDrag_setPixmap>(&mut self, value: T) -> i32 {
    value.setPixmap(self);
    return 1;
  }
}

pub trait QDrag_setPixmap {
  fn setPixmap(self, this: &mut QDrag) -> i32;
}

// proto: void QDrag::setPixmap(const QPixmap & );
impl<'a> /*trait*/ QDrag_setPixmap for (&'a  QPixmap) {
  fn setPixmap(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QDrag9setPixmapERK7QPixmap(arg0)};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn source<T: QDrag_source>(&mut self, value: T) -> i32 {
    value.source(self);
    return 1;
  }
}

pub trait QDrag_source {
  fn source(self, this: &mut QDrag) -> i32;
}

// proto: QObject * QDrag::source();
impl<'a> /*trait*/ QDrag_source for () {
  fn source(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6sourceEv()};
    unsafe {_ZNK5QDrag6sourceEv()};
    return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn targetChanged<T: QDrag_targetChanged>(&mut self, value: T) -> i32 {
    value.targetChanged(self);
    return 1;
  }
}

pub trait QDrag_targetChanged {
  fn targetChanged(self, this: &mut QDrag) -> i32;
}

// proto: void QDrag::targetChanged(QObject * newTarget);
impl<'a> /*trait*/ QDrag_targetChanged for (&'a mut QObject) {
  fn targetChanged(self, this: &mut QDrag) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag13targetChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDrag13targetChangedEP7QObject(arg0)};
    return 1;
  }
}

