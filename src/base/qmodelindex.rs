// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QModelIndex::NewQModelIndex();
  fn _ZN11QModelIndexC1Ev(qthis: *mut c_void) ;
  // proto:  int QModelIndex::column();
  fn _ZNK11QModelIndex6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  int QModelIndex::internalId();
  fn _ZNK11QModelIndex10internalIdEv(qthis: *mut c_void) -> c_int;
  // proto:  QModelIndex QModelIndex::child(int row, int column);
  fn _ZNK11QModelIndex5childEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void * QModelIndex::internalPointer();
  fn _ZNK11QModelIndex15internalPointerEv(qthis: *mut c_void) ;
  // proto:  bool QModelIndex::isValid();
  fn _ZNK11QModelIndex7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QModelIndex QModelIndex::parent();
  fn _ZNK11QModelIndex6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
  fn _ZNK11QModelIndex7siblingEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QAbstractItemModel * QModelIndex::model();
  fn _ZNK11QModelIndex5modelEv(qthis: *mut c_void) ;
  // proto:  QVariant QModelIndex::data(int role);
  fn _ZNK11QModelIndex4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QModelIndex::row();
  fn _ZNK11QModelIndex3rowEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QModelIndex)=24
pub struct QModelIndex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QModelIndex {
  pub fn NewQModelIndex<T: QModelIndex_NewQModelIndex>(value: T) -> QModelIndex {
    let rsthis = value.NewQModelIndex();
    return rsthis;
    // return 1;
  }
}

pub trait QModelIndex_NewQModelIndex {
  fn NewQModelIndex(self) -> QModelIndex;
}

// proto: void QModelIndex::NewQModelIndex();
impl<'a> /*trait*/ QModelIndex_NewQModelIndex for () {
  fn NewQModelIndex(self) -> QModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QModelIndexC1Ev()};
    unsafe {_ZN11QModelIndexC1Ev(qthis)};
    let rsthis = QModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn column<T: QModelIndex_column>(&mut self, value: T) -> i32 {
    return value.column(self);
    // return 1;
  }
}

pub trait QModelIndex_column {
  fn column(self, rsthis: &mut QModelIndex) -> i32;
}

// proto:  int QModelIndex::column();
impl<'a> /*trait*/ QModelIndex_column for () {
  fn column(self, rsthis: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6columnEv()};
    let mut ret = unsafe {_ZNK11QModelIndex6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn internalId<T: QModelIndex_internalId>(&mut self, value: T) -> i32 {
    return value.internalId(self);
    // return 1;
  }
}

pub trait QModelIndex_internalId {
  fn internalId(self, rsthis: &mut QModelIndex) -> i32;
}

// proto:  int QModelIndex::internalId();
impl<'a> /*trait*/ QModelIndex_internalId for () {
  fn internalId(self, rsthis: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex10internalIdEv()};
    let mut ret = unsafe {_ZNK11QModelIndex10internalIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn child<T: QModelIndex_child>(&mut self, value: T) -> QModelIndex {
    return value.child(self);
    // return 1;
  }
}

pub trait QModelIndex_child {
  fn child(self, rsthis: &mut QModelIndex) -> QModelIndex;
}

// proto:  QModelIndex QModelIndex::child(int row, int column);
impl<'a> /*trait*/ QModelIndex_child for (i32, i32) {
  fn child(self, rsthis: &mut QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QModelIndex5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn internalPointer<T: QModelIndex_internalPointer>(&mut self, value: T)  {
     value.internalPointer(self);
    // return 1;
  }
}

pub trait QModelIndex_internalPointer {
  fn internalPointer(self, rsthis: &mut QModelIndex) ;
}

// proto:  void * QModelIndex::internalPointer();
impl<'a> /*trait*/ QModelIndex_internalPointer for () {
  fn internalPointer(self, rsthis: &mut QModelIndex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex15internalPointerEv()};
     unsafe {_ZNK11QModelIndex15internalPointerEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn isValid<T: QModelIndex_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QModelIndex_isValid {
  fn isValid(self, rsthis: &mut QModelIndex) -> i8;
}

// proto:  bool QModelIndex::isValid();
impl<'a> /*trait*/ QModelIndex_isValid for () {
  fn isValid(self, rsthis: &mut QModelIndex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7isValidEv()};
    let mut ret = unsafe {_ZNK11QModelIndex7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn parent<T: QModelIndex_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QModelIndex_parent {
  fn parent(self, rsthis: &mut QModelIndex) -> QModelIndex;
}

// proto:  QModelIndex QModelIndex::parent();
impl<'a> /*trait*/ QModelIndex_parent for () {
  fn parent(self, rsthis: &mut QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6parentEv()};
    let mut ret = unsafe {_ZNK11QModelIndex6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn sibling<T: QModelIndex_sibling>(&mut self, value: T) -> QModelIndex {
    return value.sibling(self);
    // return 1;
  }
}

pub trait QModelIndex_sibling {
  fn sibling(self, rsthis: &mut QModelIndex) -> QModelIndex;
}

// proto:  QModelIndex QModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QModelIndex_sibling for (i32, i32) {
  fn sibling(self, rsthis: &mut QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QModelIndex7siblingEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn model<T: QModelIndex_model>(&mut self, value: T)  {
     value.model(self);
    // return 1;
  }
}

pub trait QModelIndex_model {
  fn model(self, rsthis: &mut QModelIndex) ;
}

// proto:  const QAbstractItemModel * QModelIndex::model();
impl<'a> /*trait*/ QModelIndex_model for () {
  fn model(self, rsthis: &mut QModelIndex)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5modelEv()};
     unsafe {_ZNK11QModelIndex5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn data<T: QModelIndex_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QModelIndex_data {
  fn data(self, rsthis: &mut QModelIndex) -> QVariant;
}

// proto:  QVariant QModelIndex::data(int role);
impl<'a> /*trait*/ QModelIndex_data for (i32) {
  fn data(self, rsthis: &mut QModelIndex) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QModelIndex4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn row<T: QModelIndex_row>(&mut self, value: T) -> i32 {
    return value.row(self);
    // return 1;
  }
}

pub trait QModelIndex_row {
  fn row(self, rsthis: &mut QModelIndex) -> i32;
}

// proto:  int QModelIndex::row();
impl<'a> /*trait*/ QModelIndex_row for () {
  fn row(self, rsthis: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex3rowEv()};
    let mut ret = unsafe {_ZNK11QModelIndex3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

