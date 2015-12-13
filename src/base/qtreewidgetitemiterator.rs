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
  // proto: void QTreeWidgetItemIterator::FreeQTreeWidgetItemIterator();
  fn _ZN23QTreeWidgetItemIteratorD0Ev() -> i32;
  // proto: void QTreeWidgetItemIterator::NewQTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
  fn _ZN23QTreeWidgetItemIteratorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QTreeWidgetItemIterator)=1
pub struct QTreeWidgetItemIterator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeWidgetItemIterator {
  pub fn FreeQTreeWidgetItemIterator<T: QTreeWidgetItemIterator_FreeQTreeWidgetItemIterator>(&mut self, value: T) -> i32 {
    value.FreeQTreeWidgetItemIterator(self);
    return 1;
  }
}

pub trait QTreeWidgetItemIterator_FreeQTreeWidgetItemIterator {
  fn FreeQTreeWidgetItemIterator(self, this: &mut QTreeWidgetItemIterator) -> i32;
}

// proto: void QTreeWidgetItemIterator::FreeQTreeWidgetItemIterator();
impl<'a> /*trait*/ QTreeWidgetItemIterator_FreeQTreeWidgetItemIterator for () {
  fn FreeQTreeWidgetItemIterator(self, this: &mut QTreeWidgetItemIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorD0Ev()};
    unsafe {_ZN23QTreeWidgetItemIteratorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTreeWidgetItemIterator {
  pub fn NewQTreeWidgetItemIterator<T: QTreeWidgetItemIterator_NewQTreeWidgetItemIterator>(value: T) -> QTreeWidgetItemIterator {
    let rsthis = value.NewQTreeWidgetItemIterator();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_NewQTreeWidgetItemIterator {
  fn NewQTreeWidgetItemIterator(self) -> QTreeWidgetItemIterator;
}

// proto: void QTreeWidgetItemIterator::NewQTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
impl<'a> /*trait*/ QTreeWidgetItemIterator_NewQTreeWidgetItemIterator for (&'a  QTreeWidgetItemIterator) {
  fn NewQTreeWidgetItemIterator(self) -> QTreeWidgetItemIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN23QTreeWidgetItemIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

