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
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
  fn _ZN24QStyleOptionGraphicsItemC1Ev(qthis: *mut c_void);
  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
  fn _ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0: *mut c_void) -> c_double;
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
  fn _ZN24QStyleOptionGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(int version);
  fn _ZN24QStyleOptionGraphicsItemC1Ei(qthis: *mut c_void, arg0: c_int);
}

// body block begin
// class sizeof(QStyleOptionGraphicsItem)=1
pub struct QStyleOptionGraphicsItem {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
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

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
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

  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn levelOfDetailFromTransform_s<RetType, T: QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.levelOfDetailFromTransform_s();
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<RetType> {
  fn levelOfDetailFromTransform_s(self ) -> RetType;
}

  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<f64> for (QTransform) {
  fn levelOfDetailFromTransform_s(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for (QStyleOptionGraphicsItem) {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QStyleOptionGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(int version);
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

