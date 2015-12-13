// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qsizepolicy::QSizePolicy;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QSize QSpacerItem::minimumSize();
  fn _ZNK11QSpacerItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizePolicy QSpacerItem::sizePolicy();
  fn _ZNK11QSpacerItem10sizePolicyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSpacerItem::FreeQSpacerItem();
  fn _ZN11QSpacerItemD0Ev(qthis: *mut c_void) ;
  // proto:  QSize QSpacerItem::sizeHint();
  fn _ZNK11QSpacerItem8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QSpacerItem::maximumSize();
  fn _ZNK11QSpacerItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSpacerItem::isEmpty();
  fn _ZNK11QSpacerItem7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QSpacerItem::geometry();
  fn _ZNK11QSpacerItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSpacerItem::setGeometry(const QRect & );
  fn _ZN11QSpacerItem11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSpacerItem * QSpacerItem::spacerItem();
  fn _ZN11QSpacerItem10spacerItemEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QSpacerItem)=1
pub struct QSpacerItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSpacerItem {
  pub fn minimumSize<T: QSpacerItem_minimumSize>(&mut self, value: T) -> QSize {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_minimumSize {
  fn minimumSize(self, rsthis: &mut QSpacerItem) -> QSize;
}

// proto:  QSize QSpacerItem::minimumSize();
impl<'a> /*trait*/ QSpacerItem_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn sizePolicy<T: QSpacerItem_sizePolicy>(&mut self, value: T) -> QSizePolicy {
    return value.sizePolicy(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizePolicy {
  fn sizePolicy(self, rsthis: &mut QSpacerItem) -> QSizePolicy;
}

// proto:  QSizePolicy QSpacerItem::sizePolicy();
impl<'a> /*trait*/ QSpacerItem_sizePolicy for () {
  fn sizePolicy(self, rsthis: &mut QSpacerItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn FreeQSpacerItem<T: QSpacerItem_FreeQSpacerItem>(&mut self, value: T)  {
     value.FreeQSpacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_FreeQSpacerItem {
  fn FreeQSpacerItem(self, rsthis: &mut QSpacerItem) ;
}

// proto:  void QSpacerItem::FreeQSpacerItem();
impl<'a> /*trait*/ QSpacerItem_FreeQSpacerItem for () {
  fn FreeQSpacerItem(self, rsthis: &mut QSpacerItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItemD0Ev()};
     unsafe {_ZN11QSpacerItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn sizeHint<T: QSpacerItem_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QSpacerItem_sizeHint {
  fn sizeHint(self, rsthis: &mut QSpacerItem) -> QSize;
}

// proto:  QSize QSpacerItem::sizeHint();
impl<'a> /*trait*/ QSpacerItem_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn maximumSize<T: QSpacerItem_maximumSize>(&mut self, value: T) -> QSize {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QSpacerItem_maximumSize {
  fn maximumSize(self, rsthis: &mut QSpacerItem) -> QSize;
}

// proto:  QSize QSpacerItem::maximumSize();
impl<'a> /*trait*/ QSpacerItem_maximumSize for () {
  fn maximumSize(self, rsthis: &mut QSpacerItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn isEmpty<T: QSpacerItem_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QSpacerItem_isEmpty {
  fn isEmpty(self, rsthis: &mut QSpacerItem) -> i8;
}

// proto:  bool QSpacerItem::isEmpty();
impl<'a> /*trait*/ QSpacerItem_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QSpacerItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn geometry<T: QSpacerItem_geometry>(&mut self, value: T) -> QRect {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_geometry {
  fn geometry(self, rsthis: &mut QSpacerItem) -> QRect;
}

// proto:  QRect QSpacerItem::geometry();
impl<'a> /*trait*/ QSpacerItem_geometry for () {
  fn geometry(self, rsthis: &mut QSpacerItem) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8geometryEv()};
    let mut ret = unsafe {_ZNK11QSpacerItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn setGeometry<T: QSpacerItem_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QSpacerItem_setGeometry {
  fn setGeometry(self, rsthis: &mut QSpacerItem) ;
}

// proto:  void QSpacerItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QSpacerItem_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QSpacerItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QSpacerItem11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn spacerItem<T: QSpacerItem_spacerItem>(&mut self, value: T) -> QSpacerItem {
    return value.spacerItem(self);
    // return 1;
  }
}

pub trait QSpacerItem_spacerItem {
  fn spacerItem(self, rsthis: &mut QSpacerItem) -> QSpacerItem;
}

// proto:  QSpacerItem * QSpacerItem::spacerItem();
impl<'a> /*trait*/ QSpacerItem_spacerItem for () {
  fn spacerItem(self, rsthis: &mut QSpacerItem) -> QSpacerItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem10spacerItemEv()};
    let mut ret = unsafe {_ZN11QSpacerItem10spacerItemEv(rsthis.qclsinst)};
    let mut ret1 = QSpacerItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

