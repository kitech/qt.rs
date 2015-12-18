// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qmimedata::QMimeData;
use super::qpoint::QPoint;
use super::qpixmap::QPixmap;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QObject * QDrag::target();
  fn _ZNK5QDrag6targetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMimeData * QDrag::mimeData();
  fn _ZNK5QDrag8mimeDataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDrag::NewQDrag(QObject * dragSource);
  fn _ZN5QDragC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDrag::FreeQDrag();
  fn _ZN5QDragD0Ev(qthis: *mut c_void) ;
  // proto:  void QDrag::NewQDrag(const QDrag & );
  fn _ZN5QDragC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDrag::setHotSpot(const QPoint & hotspot);
  fn _ZN5QDrag10setHotSpotERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QDrag::metaObject();
  fn _ZNK5QDrag10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QDrag::setMimeData(QMimeData * data);
  fn _ZN5QDrag11setMimeDataEP9QMimeData(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPixmap QDrag::pixmap();
  fn _ZNK5QDrag6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QDrag::hotSpot();
  fn _ZNK5QDrag7hotSpotEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDrag::setPixmap(const QPixmap & );
  fn _ZN5QDrag9setPixmapERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QObject * QDrag::source();
  fn _ZNK5QDrag6sourceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDrag::targetChanged(QObject * newTarget);
  fn _ZN5QDrag13targetChangedEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QDrag)=1
pub struct QDrag {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDrag {
  pub fn target<RetType, T: QDrag_target<RetType>>(&mut self, value: T) -> RetType {
    return value.target(self);
    // return 1;
  }
}

pub trait QDrag_target<RetType> {
  fn target(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  QObject * QDrag::target();
impl<'a> /*trait*/ QDrag_target<QObject> for () {
  fn target(self, rsthis: &mut QDrag) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6targetEv()};
    let mut ret = unsafe {_ZNK5QDrag6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn mimeData<RetType, T: QDrag_mimeData<RetType>>(&mut self, value: T) -> RetType {
    return value.mimeData(self);
    // return 1;
  }
}

pub trait QDrag_mimeData<RetType> {
  fn mimeData(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  QMimeData * QDrag::mimeData();
impl<'a> /*trait*/ QDrag_mimeData<QMimeData> for () {
  fn mimeData(self, rsthis: &mut QDrag) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag8mimeDataEv()};
    let mut ret = unsafe {_ZNK5QDrag8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn FreeQDrag<RetType, T: QDrag_FreeQDrag<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDrag(self);
    // return 1;
  }
}

pub trait QDrag_FreeQDrag<RetType> {
  fn FreeQDrag(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  void QDrag::FreeQDrag();
impl<'a> /*trait*/ QDrag_FreeQDrag<()> for () {
  fn FreeQDrag(self, rsthis: &mut QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragD0Ev()};
     unsafe {_ZN5QDragD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QDrag::NewQDrag(const QDrag & );
impl<'a> /*trait*/ QDrag_NewQDrag for (&'a  QDrag) {
  fn NewQDrag(self) -> QDrag {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QDragC1ERKS_(qthis, arg0)};
    let rsthis = QDrag{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn setHotSpot<RetType, T: QDrag_setHotSpot<RetType>>(&mut self, value: T) -> RetType {
    return value.setHotSpot(self);
    // return 1;
  }
}

pub trait QDrag_setHotSpot<RetType> {
  fn setHotSpot(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  void QDrag::setHotSpot(const QPoint & hotspot);
impl<'a> /*trait*/ QDrag_setHotSpot<()> for (&'a  QPoint) {
  fn setHotSpot(self, rsthis: &mut QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag10setHotSpotERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag10setHotSpotERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn metaObject<RetType, T: QDrag_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QDrag_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  const QMetaObject * QDrag::metaObject();
impl<'a> /*trait*/ QDrag_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag10metaObjectEv()};
     unsafe {_ZNK5QDrag10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn setMimeData<RetType, T: QDrag_setMimeData<RetType>>(&mut self, value: T) -> RetType {
    return value.setMimeData(self);
    // return 1;
  }
}

pub trait QDrag_setMimeData<RetType> {
  fn setMimeData(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  void QDrag::setMimeData(QMimeData * data);
impl<'a> /*trait*/ QDrag_setMimeData<()> for (&'a mut QMimeData) {
  fn setMimeData(self, rsthis: &mut QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag11setMimeDataEP9QMimeData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag11setMimeDataEP9QMimeData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn pixmap<RetType, T: QDrag_pixmap<RetType>>(&mut self, value: T) -> RetType {
    return value.pixmap(self);
    // return 1;
  }
}

pub trait QDrag_pixmap<RetType> {
  fn pixmap(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  QPixmap QDrag::pixmap();
impl<'a> /*trait*/ QDrag_pixmap<QPixmap> for () {
  fn pixmap(self, rsthis: &mut QDrag) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6pixmapEv()};
    let mut ret = unsafe {_ZNK5QDrag6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn hotSpot<RetType, T: QDrag_hotSpot<RetType>>(&mut self, value: T) -> RetType {
    return value.hotSpot(self);
    // return 1;
  }
}

pub trait QDrag_hotSpot<RetType> {
  fn hotSpot(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  QPoint QDrag::hotSpot();
impl<'a> /*trait*/ QDrag_hotSpot<QPoint> for () {
  fn hotSpot(self, rsthis: &mut QDrag) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag7hotSpotEv()};
    let mut ret = unsafe {_ZNK5QDrag7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn setPixmap<RetType, T: QDrag_setPixmap<RetType>>(&mut self, value: T) -> RetType {
    return value.setPixmap(self);
    // return 1;
  }
}

pub trait QDrag_setPixmap<RetType> {
  fn setPixmap(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  void QDrag::setPixmap(const QPixmap & );
impl<'a> /*trait*/ QDrag_setPixmap<()> for (&'a  QPixmap) {
  fn setPixmap(self, rsthis: &mut QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn source<RetType, T: QDrag_source<RetType>>(&mut self, value: T) -> RetType {
    return value.source(self);
    // return 1;
  }
}

pub trait QDrag_source<RetType> {
  fn source(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  QObject * QDrag::source();
impl<'a> /*trait*/ QDrag_source<QObject> for () {
  fn source(self, rsthis: &mut QDrag) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6sourceEv()};
    let mut ret = unsafe {_ZNK5QDrag6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDrag {
  pub fn targetChanged<RetType, T: QDrag_targetChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.targetChanged(self);
    // return 1;
  }
}

pub trait QDrag_targetChanged<RetType> {
  fn targetChanged(self, rsthis: &mut QDrag) -> RetType;
}

// proto:  void QDrag::targetChanged(QObject * newTarget);
impl<'a> /*trait*/ QDrag_targetChanged<()> for (&'a mut QObject) {
  fn targetChanged(self, rsthis: &mut QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag13targetChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag13targetChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

