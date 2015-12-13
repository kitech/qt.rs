// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QSize QSpacerItem::minimumSize();
  fn _ZNK11QSpacerItem11minimumSizeEv() -> i32;
  // proto: QSizePolicy QSpacerItem::sizePolicy();
  fn _ZNK11QSpacerItem10sizePolicyEv() -> i32;
  // proto: void QSpacerItem::FreeQSpacerItem();
  fn _ZN11QSpacerItemD0Ev() -> i32;
  // proto: QSize QSpacerItem::sizeHint();
  fn _ZNK11QSpacerItem8sizeHintEv() -> i32;
  // proto: QSize QSpacerItem::maximumSize();
  fn _ZNK11QSpacerItem11maximumSizeEv() -> i32;
  // proto: bool QSpacerItem::isEmpty();
  fn _ZNK11QSpacerItem7isEmptyEv() -> i32;
  // proto: QRect QSpacerItem::geometry();
  fn _ZNK11QSpacerItem8geometryEv() -> i32;
  // proto: void QSpacerItem::setGeometry(const QRect & );
  fn _ZN11QSpacerItem11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: QSpacerItem * QSpacerItem::spacerItem();
  fn _ZN11QSpacerItem10spacerItemEv() -> i32;
}

// body block begin
// class sizeof(QSpacerItem)=1
pub struct QSpacerItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSpacerItem {
  pub fn minimumSize<T: QSpacerItem_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QSpacerItem_minimumSize {
  fn minimumSize(self, this: &mut QSpacerItem) -> i32;
}

// proto: QSize QSpacerItem::minimumSize();
impl<'a> /*trait*/ QSpacerItem_minimumSize for () {
  fn minimumSize(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11minimumSizeEv()};
    unsafe {_ZNK11QSpacerItem11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn sizePolicy<T: QSpacerItem_sizePolicy>(&mut self, value: T) -> i32 {
    value.sizePolicy(self);
    return 1;
  }
}

pub trait QSpacerItem_sizePolicy {
  fn sizePolicy(self, this: &mut QSpacerItem) -> i32;
}

// proto: QSizePolicy QSpacerItem::sizePolicy();
impl<'a> /*trait*/ QSpacerItem_sizePolicy for () {
  fn sizePolicy(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem10sizePolicyEv()};
    unsafe {_ZNK11QSpacerItem10sizePolicyEv()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn FreeQSpacerItem<T: QSpacerItem_FreeQSpacerItem>(&mut self, value: T) -> i32 {
    value.FreeQSpacerItem(self);
    return 1;
  }
}

pub trait QSpacerItem_FreeQSpacerItem {
  fn FreeQSpacerItem(self, this: &mut QSpacerItem) -> i32;
}

// proto: void QSpacerItem::FreeQSpacerItem();
impl<'a> /*trait*/ QSpacerItem_FreeQSpacerItem for () {
  fn FreeQSpacerItem(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItemD0Ev()};
    unsafe {_ZN11QSpacerItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn sizeHint<T: QSpacerItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QSpacerItem_sizeHint {
  fn sizeHint(self, this: &mut QSpacerItem) -> i32;
}

// proto: QSize QSpacerItem::sizeHint();
impl<'a> /*trait*/ QSpacerItem_sizeHint for () {
  fn sizeHint(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8sizeHintEv()};
    unsafe {_ZNK11QSpacerItem8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn maximumSize<T: QSpacerItem_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QSpacerItem_maximumSize {
  fn maximumSize(self, this: &mut QSpacerItem) -> i32;
}

// proto: QSize QSpacerItem::maximumSize();
impl<'a> /*trait*/ QSpacerItem_maximumSize for () {
  fn maximumSize(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem11maximumSizeEv()};
    unsafe {_ZNK11QSpacerItem11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn isEmpty<T: QSpacerItem_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QSpacerItem_isEmpty {
  fn isEmpty(self, this: &mut QSpacerItem) -> i32;
}

// proto: bool QSpacerItem::isEmpty();
impl<'a> /*trait*/ QSpacerItem_isEmpty for () {
  fn isEmpty(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem7isEmptyEv()};
    unsafe {_ZNK11QSpacerItem7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn geometry<T: QSpacerItem_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QSpacerItem_geometry {
  fn geometry(self, this: &mut QSpacerItem) -> i32;
}

// proto: QRect QSpacerItem::geometry();
impl<'a> /*trait*/ QSpacerItem_geometry for () {
  fn geometry(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSpacerItem8geometryEv()};
    unsafe {_ZNK11QSpacerItem8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn setGeometry<T: QSpacerItem_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QSpacerItem_setGeometry {
  fn setGeometry(self, this: &mut QSpacerItem) -> i32;
}

// proto: void QSpacerItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QSpacerItem_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QSpacerItem11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QSpacerItem {
  pub fn spacerItem<T: QSpacerItem_spacerItem>(&mut self, value: T) -> i32 {
    value.spacerItem(self);
    return 1;
  }
}

pub trait QSpacerItem_spacerItem {
  fn spacerItem(self, this: &mut QSpacerItem) -> i32;
}

// proto: QSpacerItem * QSpacerItem::spacerItem();
impl<'a> /*trait*/ QSpacerItem_spacerItem for () {
  fn spacerItem(self, this: &mut QSpacerItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSpacerItem10spacerItemEv()};
    unsafe {_ZN11QSpacerItem10spacerItemEv()};
    return 1;
  }
}

