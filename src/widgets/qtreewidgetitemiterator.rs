// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtWidgets/qtreewidgetitemiterator.h
// dst-file: /src/widgets/qtreewidgetitemiterator.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::qtreewidget::QTreeWidget; // 773
use super::qtreewidget::QTreeWidgetItem; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QTreeWidgetItemIterator::~QTreeWidgetItemIterator();
  fn _ZN23QTreeWidgetItemIteratorD0Ev(qthis: *mut c_void);
  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
  fn _ZN23QTreeWidgetItemIteratorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTreeWidgetItemIterator)=1
pub struct QTreeWidgetItemIterator {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTreeWidgetItemIterator {
  pub fn inheritFrom(qthis: *mut c_void) -> QTreeWidgetItemIterator {
    return QTreeWidgetItemIterator{qclsinst: qthis};
  }
}
  // proto:  void QTreeWidgetItemIterator::~QTreeWidgetItemIterator();
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn Free<RetType, T: QTreeWidgetItemIterator_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_Free<RetType> {
  fn Free(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}

  // proto:  void QTreeWidgetItemIterator::~QTreeWidgetItemIterator();
impl<'a> /*trait*/ QTreeWidgetItemIterator_Free<()> for () {
  fn Free(self , rsthis: & QTreeWidgetItemIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorD0Ev()};
     unsafe {_ZN23QTreeWidgetItemIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn New<T: QTreeWidgetItemIterator_New>(value: T) -> QTreeWidgetItemIterator {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_New {
  fn New(self) -> QTreeWidgetItemIterator;
}

  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
impl<'a> /*trait*/ QTreeWidgetItemIterator_New for (&'a QTreeWidgetItemIterator) {
  fn New(self) -> QTreeWidgetItemIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QTreeWidgetItemIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

