// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtransform::QTransform;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStyleOptionGraphicsItem::NewQStyleOptionGraphicsItem();
  fn _ZN24QStyleOptionGraphicsItemC1Ev(qthis: *mut c_void) -> i32;
  // proto: double QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
  fn _ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0: *const c_void) -> i32;
  // proto: void QStyleOptionGraphicsItem::NewQStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
  fn _ZN24QStyleOptionGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStyleOptionGraphicsItem::NewQStyleOptionGraphicsItem(int version);
  fn _ZN24QStyleOptionGraphicsItemC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QStyleOptionGraphicsItem)=1
pub struct QStyleOptionGraphicsItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn NewQStyleOptionGraphicsItem<T: QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem>(value: T) -> QStyleOptionGraphicsItem {
    let rsthis = value.NewQStyleOptionGraphicsItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem;
}

// proto: void QStyleOptionGraphicsItem::NewQStyleOptionGraphicsItem();
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for () {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1Ev()};
    unsafe {_ZN24QStyleOptionGraphicsItemC1Ev(qthis)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn levelOfDetailFromTransform<T: QStyleOptionGraphicsItem_levelOfDetailFromTransform>(&mut self, value: T) -> i32 {
    value.levelOfDetailFromTransform(self);
    return 1;
  }
}

pub trait QStyleOptionGraphicsItem_levelOfDetailFromTransform {
  fn levelOfDetailFromTransform(self, this: &mut QStyleOptionGraphicsItem) -> i32;
}

// proto: double QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_levelOfDetailFromTransform for (&'a  QTransform) {
  fn levelOfDetailFromTransform(self, this: &mut QStyleOptionGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0)};
    return 1;
  }
}

// proto: void QStyleOptionGraphicsItem::NewQStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for (&'a  QStyleOptionGraphicsItem) {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN24QStyleOptionGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionGraphicsItem::NewQStyleOptionGraphicsItem(int version);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for (i32) {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN24QStyleOptionGraphicsItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

