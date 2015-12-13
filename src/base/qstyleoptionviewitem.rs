// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStyleOptionViewItem::NewQStyleOptionViewItem(const QStyleOptionViewItem & other);
  fn _ZN20QStyleOptionViewItemC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStyleOptionViewItem::NewQStyleOptionViewItem(int version);
  fn _ZN20QStyleOptionViewItemC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QStyleOptionViewItem::NewQStyleOptionViewItem();
  fn _ZN20QStyleOptionViewItemC1Ev(qthis: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QStyleOptionViewItem)=1
pub struct QStyleOptionViewItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionViewItem {
  pub fn NewQStyleOptionViewItem<T: QStyleOptionViewItem_NewQStyleOptionViewItem>(value: T) -> QStyleOptionViewItem {
    let rsthis = value.NewQStyleOptionViewItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionViewItem_NewQStyleOptionViewItem {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem;
}

// proto: void QStyleOptionViewItem::NewQStyleOptionViewItem(const QStyleOptionViewItem & other);
impl<'a> /*trait*/ QStyleOptionViewItem_NewQStyleOptionViewItem for (&'a  QStyleOptionViewItem) {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QStyleOptionViewItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionViewItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionViewItem::NewQStyleOptionViewItem(int version);
impl<'a> /*trait*/ QStyleOptionViewItem_NewQStyleOptionViewItem for (i32) {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionViewItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionViewItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionViewItem::NewQStyleOptionViewItem();
impl<'a> /*trait*/ QStyleOptionViewItem_NewQStyleOptionViewItem for () {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1Ev()};
    unsafe {_ZN20QStyleOptionViewItemC1Ev(qthis)};
    let rsthis = QStyleOptionViewItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

