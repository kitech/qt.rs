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
  fn _ZN11QModelIndexC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK11QModelIndex6columnEv() -> i32;
  fn _ZNK11QModelIndex10internalIdEv() -> i32;
  fn _ZNK11QModelIndex5childEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK11QModelIndex15internalPointerEv() -> i32;
  fn _ZNK11QModelIndex7isValidEv() -> i32;
  fn _ZNK11QModelIndex6parentEv() -> i32;
  fn _ZNK11QModelIndex7siblingEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK11QModelIndex5modelEv() -> i32;
  fn _ZNK11QModelIndex4dataEi(arg0: c_int) -> i32;
  fn _ZNK11QModelIndex3rowEv() -> i32;
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
    value.column(self);
    return 1;
  }
}

pub trait QModelIndex_column {
  fn column(self, this: &mut QModelIndex) -> i32;
}

// proto: int QModelIndex::column();
impl<'a> /*trait*/ QModelIndex_column for () {
  fn column(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6columnEv()};
    unsafe {_ZNK11QModelIndex6columnEv()};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn internalId<T: QModelIndex_internalId>(&mut self, value: T) -> i32 {
    value.internalId(self);
    return 1;
  }
}

pub trait QModelIndex_internalId {
  fn internalId(self, this: &mut QModelIndex) -> i32;
}

// proto: int QModelIndex::internalId();
impl<'a> /*trait*/ QModelIndex_internalId for () {
  fn internalId(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex10internalIdEv()};
    unsafe {_ZNK11QModelIndex10internalIdEv()};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn child<T: QModelIndex_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QModelIndex_child {
  fn child(self, this: &mut QModelIndex) -> i32;
}

// proto: QModelIndex QModelIndex::child(int row, int column);
impl<'a> /*trait*/ QModelIndex_child for (i32, i32) {
  fn child(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QModelIndex5childEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn internalPointer<T: QModelIndex_internalPointer>(&mut self, value: T) -> i32 {
    value.internalPointer(self);
    return 1;
  }
}

pub trait QModelIndex_internalPointer {
  fn internalPointer(self, this: &mut QModelIndex) -> i32;
}

// proto: void * QModelIndex::internalPointer();
impl<'a> /*trait*/ QModelIndex_internalPointer for () {
  fn internalPointer(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex15internalPointerEv()};
    unsafe {_ZNK11QModelIndex15internalPointerEv()};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn isValid<T: QModelIndex_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QModelIndex_isValid {
  fn isValid(self, this: &mut QModelIndex) -> i32;
}

// proto: bool QModelIndex::isValid();
impl<'a> /*trait*/ QModelIndex_isValid for () {
  fn isValid(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7isValidEv()};
    unsafe {_ZNK11QModelIndex7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn parent<T: QModelIndex_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QModelIndex_parent {
  fn parent(self, this: &mut QModelIndex) -> i32;
}

// proto: QModelIndex QModelIndex::parent();
impl<'a> /*trait*/ QModelIndex_parent for () {
  fn parent(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6parentEv()};
    unsafe {_ZNK11QModelIndex6parentEv()};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn sibling<T: QModelIndex_sibling>(&mut self, value: T) -> i32 {
    value.sibling(self);
    return 1;
  }
}

pub trait QModelIndex_sibling {
  fn sibling(self, this: &mut QModelIndex) -> i32;
}

// proto: QModelIndex QModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QModelIndex_sibling for (i32, i32) {
  fn sibling(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QModelIndex7siblingEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn model<T: QModelIndex_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QModelIndex_model {
  fn model(self, this: &mut QModelIndex) -> i32;
}

// proto: const QAbstractItemModel * QModelIndex::model();
impl<'a> /*trait*/ QModelIndex_model for () {
  fn model(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5modelEv()};
    unsafe {_ZNK11QModelIndex5modelEv()};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn data<T: QModelIndex_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QModelIndex_data {
  fn data(self, this: &mut QModelIndex) -> i32;
}

// proto: QVariant QModelIndex::data(int role);
impl<'a> /*trait*/ QModelIndex_data for (i32) {
  fn data(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex4dataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QModelIndex4dataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QModelIndex {
  pub fn row<T: QModelIndex_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QModelIndex_row {
  fn row(self, this: &mut QModelIndex) -> i32;
}

// proto: int QModelIndex::row();
impl<'a> /*trait*/ QModelIndex_row for () {
  fn row(self, this: &mut QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex3rowEv()};
    unsafe {_ZNK11QModelIndex3rowEv()};
    return 1;
  }
}

