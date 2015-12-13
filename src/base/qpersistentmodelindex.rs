// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK21QPersistentModelIndex7siblingEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK21QPersistentModelIndex4dataEi(arg0: c_int) -> i32;
  fn _ZNK21QPersistentModelIndex6parentEv() -> i32;
  fn _ZN21QPersistentModelIndexC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK21QPersistentModelIndex15internalPointerEv() -> i32;
  fn _ZNK21QPersistentModelIndex3rowEv() -> i32;
  fn _ZNK21QPersistentModelIndex10internalIdEv() -> i32;
  fn _ZNK21QPersistentModelIndex5modelEv() -> i32;
  fn _ZN21QPersistentModelIndexD0Ev() -> i32;
  fn _ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN21QPersistentModelIndexC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK21QPersistentModelIndex6columnEv() -> i32;
  fn _ZN21QPersistentModelIndex4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZNK21QPersistentModelIndex5childEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK21QPersistentModelIndex7isValidEv() -> i32;
}

// body block begin
// class sizeof(QPersistentModelIndex)=8
pub struct QPersistentModelIndex {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPersistentModelIndex {
  pub fn sibling<T: QPersistentModelIndex_sibling>(&mut self, value: T) -> i32 {
    value.sibling(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_sibling {
  fn sibling(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: QModelIndex QPersistentModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_sibling for (i32, i32) {
  fn sibling(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK21QPersistentModelIndex7siblingEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn data<T: QPersistentModelIndex_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_data {
  fn data(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: QVariant QPersistentModelIndex::data(int role);
impl<'a> /*trait*/ QPersistentModelIndex_data for (i32) {
  fn data(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex4dataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK21QPersistentModelIndex4dataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn parent<T: QPersistentModelIndex_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_parent {
  fn parent(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: QModelIndex QPersistentModelIndex::parent();
impl<'a> /*trait*/ QPersistentModelIndex_parent for () {
  fn parent(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6parentEv()};
    unsafe {_ZNK21QPersistentModelIndex6parentEv()};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn NewQPersistentModelIndex<T: QPersistentModelIndex_NewQPersistentModelIndex>(value: T) -> QPersistentModelIndex {
    let rsthis = value.NewQPersistentModelIndex();
    return rsthis;
    // return 1;
  }
}

pub trait QPersistentModelIndex_NewQPersistentModelIndex {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex;
}

// proto: void QPersistentModelIndex::NewQPersistentModelIndex(const QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for (&'a  QPersistentModelIndex) {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QPersistentModelIndexC1ERKS_(qthis, arg0)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn internalPointer<T: QPersistentModelIndex_internalPointer>(&mut self, value: T) -> i32 {
    value.internalPointer(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_internalPointer {
  fn internalPointer(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: void * QPersistentModelIndex::internalPointer();
impl<'a> /*trait*/ QPersistentModelIndex_internalPointer for () {
  fn internalPointer(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex15internalPointerEv()};
    unsafe {_ZNK21QPersistentModelIndex15internalPointerEv()};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn row<T: QPersistentModelIndex_row>(&mut self, value: T) -> i32 {
    value.row(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_row {
  fn row(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: int QPersistentModelIndex::row();
impl<'a> /*trait*/ QPersistentModelIndex_row for () {
  fn row(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex3rowEv()};
    unsafe {_ZNK21QPersistentModelIndex3rowEv()};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn internalId<T: QPersistentModelIndex_internalId>(&mut self, value: T) -> i32 {
    value.internalId(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_internalId {
  fn internalId(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: int QPersistentModelIndex::internalId();
impl<'a> /*trait*/ QPersistentModelIndex_internalId for () {
  fn internalId(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex10internalIdEv()};
    unsafe {_ZNK21QPersistentModelIndex10internalIdEv()};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn model<T: QPersistentModelIndex_model>(&mut self, value: T) -> i32 {
    value.model(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_model {
  fn model(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: const QAbstractItemModel * QPersistentModelIndex::model();
impl<'a> /*trait*/ QPersistentModelIndex_model for () {
  fn model(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5modelEv()};
    unsafe {_ZNK21QPersistentModelIndex5modelEv()};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn FreeQPersistentModelIndex<T: QPersistentModelIndex_FreeQPersistentModelIndex>(&mut self, value: T) -> i32 {
    value.FreeQPersistentModelIndex(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_FreeQPersistentModelIndex {
  fn FreeQPersistentModelIndex(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: void QPersistentModelIndex::FreeQPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_FreeQPersistentModelIndex for () {
  fn FreeQPersistentModelIndex(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexD0Ev()};
    unsafe {_ZN21QPersistentModelIndexD0Ev()};
    return 1;
  }
}

// proto: void QPersistentModelIndex::NewQPersistentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for (&'a  QModelIndex) {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPersistentModelIndex::NewQPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for () {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1Ev()};
    unsafe {_ZN21QPersistentModelIndexC1Ev(qthis)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn column<T: QPersistentModelIndex_column>(&mut self, value: T) -> i32 {
    value.column(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_column {
  fn column(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: int QPersistentModelIndex::column();
impl<'a> /*trait*/ QPersistentModelIndex_column for () {
  fn column(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6columnEv()};
    unsafe {_ZNK21QPersistentModelIndex6columnEv()};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn swap<T: QPersistentModelIndex_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_swap {
  fn swap(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: void QPersistentModelIndex::swap(QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_swap for (&'a mut QPersistentModelIndex) {
  fn swap(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndex4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QPersistentModelIndex4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn child<T: QPersistentModelIndex_child>(&mut self, value: T) -> i32 {
    value.child(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_child {
  fn child(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: QModelIndex QPersistentModelIndex::child(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_child for (i32, i32) {
  fn child(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK21QPersistentModelIndex5childEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn isValid<T: QPersistentModelIndex_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QPersistentModelIndex_isValid {
  fn isValid(self, this: &mut QPersistentModelIndex) -> i32;
}

// proto: bool QPersistentModelIndex::isValid();
impl<'a> /*trait*/ QPersistentModelIndex_isValid for () {
  fn isValid(self, this: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7isValidEv()};
    unsafe {_ZNK21QPersistentModelIndex7isValidEv()};
    return 1;
  }
}

