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
  // proto:  void QTreeWidgetItemIterator::FreeQTreeWidgetItemIterator();
  fn _ZN23QTreeWidgetItemIteratorD0Ev(qthis: *mut c_void) ;
  // proto:  void QTreeWidgetItemIterator::NewQTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
  fn _ZN23QTreeWidgetItemIteratorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QTreeWidgetItemIterator)=1
pub struct QTreeWidgetItemIterator {
  pub qclsinst: *mut c_void,
}

// proto:  void QTreeWidgetItemIterator::FreeQTreeWidgetItemIterator();
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn FreeQTreeWidgetItemIterator<RetType, T: QTreeWidgetItemIterator_FreeQTreeWidgetItemIterator<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQTreeWidgetItemIterator(self);
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_FreeQTreeWidgetItemIterator<RetType> {
  fn FreeQTreeWidgetItemIterator(self , rsthis: &mut QTreeWidgetItemIterator) -> RetType;
}

// proto:  void QTreeWidgetItemIterator::FreeQTreeWidgetItemIterator();
impl<'a> /*trait*/ QTreeWidgetItemIterator_FreeQTreeWidgetItemIterator<()> for () {
  fn FreeQTreeWidgetItemIterator(self , rsthis: &mut QTreeWidgetItemIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorD0Ev()};
     unsafe {_ZN23QTreeWidgetItemIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QTreeWidgetItemIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

