// auto generated, do not modify.
// created: Tue Dec 29 22:57:40 2015
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTreeWidgetItemIterator_Class_Size() -> c_int;
  // proto:  void QTreeWidgetItemIterator::~QTreeWidgetItemIterator();
  fn _ZN23QTreeWidgetItemIteratorD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
  fn dector_ZN23QTreeWidgetItemIteratorC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN23QTreeWidgetItemIteratorC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTreeWidgetItemIterator)=1
#[derive(Default)]
pub struct QTreeWidgetItemIterator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTreeWidgetItemIterator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTreeWidgetItemIterator {
    return QTreeWidgetItemIterator{qclsinst: qthis, ..Default::default()};
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
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorC1ERKS_()};
    let ctysz: c_int = unsafe{QTreeWidgetItemIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN23QTreeWidgetItemIteratorC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN23QTreeWidgetItemIteratorC1ERKS_(arg0)} as u64;
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

