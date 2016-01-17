// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
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
  fn _ZN23QTreeWidgetItemIteratorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
  fn _ZN23QTreeWidgetItemIteratorC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
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
  pub fn free<RetType, T: QTreeWidgetItemIterator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_free<RetType> {
  fn free(self , rsthis: & QTreeWidgetItemIterator) -> RetType;
}

  // proto:  void QTreeWidgetItemIterator::~QTreeWidgetItemIterator();
impl<'a> /*trait*/ QTreeWidgetItemIterator_free<()> for () {
  fn free(self , rsthis: & QTreeWidgetItemIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorD2Ev()};
     unsafe {_ZN23QTreeWidgetItemIteratorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
impl /*struct*/ QTreeWidgetItemIterator {
  pub fn new<T: QTreeWidgetItemIterator_new>(value: T) -> QTreeWidgetItemIterator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTreeWidgetItemIterator_new {
  fn new(self) -> QTreeWidgetItemIterator;
}

  // proto:  void QTreeWidgetItemIterator::QTreeWidgetItemIterator(const QTreeWidgetItemIterator & it);
impl<'a> /*trait*/ QTreeWidgetItemIterator_new for (&'a QTreeWidgetItemIterator) {
  fn new(self) -> QTreeWidgetItemIterator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QTreeWidgetItemIteratorC2ERKS_()};
    let ctysz: c_int = unsafe{QTreeWidgetItemIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QTreeWidgetItemIteratorC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QTreeWidgetItemIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

