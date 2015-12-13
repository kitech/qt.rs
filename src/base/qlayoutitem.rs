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
  // proto: QSpacerItem * QLayoutItem::spacerItem();
  fn _ZN11QLayoutItem10spacerItemEv() -> i32;
  // proto: QSize QLayoutItem::minimumSize();
  fn _ZNK11QLayoutItem11minimumSizeEv() -> i32;
  // proto: QWidget * QLayoutItem::widget();
  fn _ZN11QLayoutItem6widgetEv() -> i32;
  // proto: void QLayoutItem::invalidate();
  fn _ZN11QLayoutItem10invalidateEv() -> i32;
  // proto: void QLayoutItem::setGeometry(const QRect & );
  fn _ZN11QLayoutItem11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: QLayout * QLayoutItem::layout();
  fn _ZN11QLayoutItem6layoutEv() -> i32;
  // proto: bool QLayoutItem::isEmpty();
  fn _ZNK11QLayoutItem7isEmptyEv() -> i32;
  // proto: QSize QLayoutItem::sizeHint();
  fn _ZNK11QLayoutItem8sizeHintEv() -> i32;
  // proto: void QLayoutItem::FreeQLayoutItem();
  fn _ZN11QLayoutItemD0Ev() -> i32;
  // proto: bool QLayoutItem::hasHeightForWidth();
  fn _ZNK11QLayoutItem17hasHeightForWidthEv() -> i32;
  // proto: int QLayoutItem::heightForWidth(int );
  fn _ZNK11QLayoutItem14heightForWidthEi(arg0: c_int) -> i32;
  // proto: QRect QLayoutItem::geometry();
  fn _ZNK11QLayoutItem8geometryEv() -> i32;
  // proto: QSize QLayoutItem::maximumSize();
  fn _ZNK11QLayoutItem11maximumSizeEv() -> i32;
  // proto: int QLayoutItem::minimumHeightForWidth(int );
  fn _ZNK11QLayoutItem21minimumHeightForWidthEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QLayoutItem)=1
pub struct QLayoutItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLayoutItem {
  pub fn spacerItem<T: QLayoutItem_spacerItem>(&mut self, value: T) -> i32 {
    value.spacerItem(self);
    return 1;
  }
}

pub trait QLayoutItem_spacerItem {
  fn spacerItem(self, this: &mut QLayoutItem) -> i32;
}

// proto: QSpacerItem * QLayoutItem::spacerItem();
impl<'a> /*trait*/ QLayoutItem_spacerItem for () {
  fn spacerItem(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10spacerItemEv()};
    unsafe {_ZN11QLayoutItem10spacerItemEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn minimumSize<T: QLayoutItem_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QLayoutItem_minimumSize {
  fn minimumSize(self, this: &mut QLayoutItem) -> i32;
}

// proto: QSize QLayoutItem::minimumSize();
impl<'a> /*trait*/ QLayoutItem_minimumSize for () {
  fn minimumSize(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11minimumSizeEv()};
    unsafe {_ZNK11QLayoutItem11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn widget<T: QLayoutItem_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QLayoutItem_widget {
  fn widget(self, this: &mut QLayoutItem) -> i32;
}

// proto: QWidget * QLayoutItem::widget();
impl<'a> /*trait*/ QLayoutItem_widget for () {
  fn widget(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6widgetEv()};
    unsafe {_ZN11QLayoutItem6widgetEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn invalidate<T: QLayoutItem_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QLayoutItem_invalidate {
  fn invalidate(self, this: &mut QLayoutItem) -> i32;
}

// proto: void QLayoutItem::invalidate();
impl<'a> /*trait*/ QLayoutItem_invalidate for () {
  fn invalidate(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem10invalidateEv()};
    unsafe {_ZN11QLayoutItem10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn setGeometry<T: QLayoutItem_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QLayoutItem_setGeometry {
  fn setGeometry(self, this: &mut QLayoutItem) -> i32;
}

// proto: void QLayoutItem::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayoutItem_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QLayoutItem11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn layout<T: QLayoutItem_layout>(&mut self, value: T) -> i32 {
    value.layout(self);
    return 1;
  }
}

pub trait QLayoutItem_layout {
  fn layout(self, this: &mut QLayoutItem) -> i32;
}

// proto: QLayout * QLayoutItem::layout();
impl<'a> /*trait*/ QLayoutItem_layout for () {
  fn layout(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItem6layoutEv()};
    unsafe {_ZN11QLayoutItem6layoutEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn isEmpty<T: QLayoutItem_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QLayoutItem_isEmpty {
  fn isEmpty(self, this: &mut QLayoutItem) -> i32;
}

// proto: bool QLayoutItem::isEmpty();
impl<'a> /*trait*/ QLayoutItem_isEmpty for () {
  fn isEmpty(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem7isEmptyEv()};
    unsafe {_ZNK11QLayoutItem7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn sizeHint<T: QLayoutItem_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QLayoutItem_sizeHint {
  fn sizeHint(self, this: &mut QLayoutItem) -> i32;
}

// proto: QSize QLayoutItem::sizeHint();
impl<'a> /*trait*/ QLayoutItem_sizeHint for () {
  fn sizeHint(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8sizeHintEv()};
    unsafe {_ZNK11QLayoutItem8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn FreeQLayoutItem<T: QLayoutItem_FreeQLayoutItem>(&mut self, value: T) -> i32 {
    value.FreeQLayoutItem(self);
    return 1;
  }
}

pub trait QLayoutItem_FreeQLayoutItem {
  fn FreeQLayoutItem(self, this: &mut QLayoutItem) -> i32;
}

// proto: void QLayoutItem::FreeQLayoutItem();
impl<'a> /*trait*/ QLayoutItem_FreeQLayoutItem for () {
  fn FreeQLayoutItem(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QLayoutItemD0Ev()};
    unsafe {_ZN11QLayoutItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn hasHeightForWidth<T: QLayoutItem_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QLayoutItem_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QLayoutItem) -> i32;
}

// proto: bool QLayoutItem::hasHeightForWidth();
impl<'a> /*trait*/ QLayoutItem_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem17hasHeightForWidthEv()};
    unsafe {_ZNK11QLayoutItem17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn heightForWidth<T: QLayoutItem_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QLayoutItem_heightForWidth {
  fn heightForWidth(self, this: &mut QLayoutItem) -> i32;
}

// proto: int QLayoutItem::heightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QLayoutItem14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn geometry<T: QLayoutItem_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QLayoutItem_geometry {
  fn geometry(self, this: &mut QLayoutItem) -> i32;
}

// proto: QRect QLayoutItem::geometry();
impl<'a> /*trait*/ QLayoutItem_geometry for () {
  fn geometry(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem8geometryEv()};
    unsafe {_ZNK11QLayoutItem8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn maximumSize<T: QLayoutItem_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QLayoutItem_maximumSize {
  fn maximumSize(self, this: &mut QLayoutItem) -> i32;
}

// proto: QSize QLayoutItem::maximumSize();
impl<'a> /*trait*/ QLayoutItem_maximumSize for () {
  fn maximumSize(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem11maximumSizeEv()};
    unsafe {_ZNK11QLayoutItem11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QLayoutItem {
  pub fn minimumHeightForWidth<T: QLayoutItem_minimumHeightForWidth>(&mut self, value: T) -> i32 {
    value.minimumHeightForWidth(self);
    return 1;
  }
}

pub trait QLayoutItem_minimumHeightForWidth {
  fn minimumHeightForWidth(self, this: &mut QLayoutItem) -> i32;
}

// proto: int QLayoutItem::minimumHeightForWidth(int );
impl<'a> /*trait*/ QLayoutItem_minimumHeightForWidth for (i32) {
  fn minimumHeightForWidth(self, this: &mut QLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QLayoutItem21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QLayoutItem21minimumHeightForWidthEi(arg0)};
    return 1;
  }
}

