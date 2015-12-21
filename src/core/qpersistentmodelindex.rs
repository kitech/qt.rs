// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
  fn _ZNK21QPersistentModelIndex7siblingEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QVariant QPersistentModelIndex::data(int role);
  fn _ZNK21QPersistentModelIndex4dataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QModelIndex QPersistentModelIndex::parent();
  fn _ZNK21QPersistentModelIndex6parentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex & other);
  fn _ZN21QPersistentModelIndexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void * QPersistentModelIndex::internalPointer();
  fn _ZNK21QPersistentModelIndex15internalPointerEv(qthis: *mut c_void);
  // proto:  int QPersistentModelIndex::row();
  fn _ZNK21QPersistentModelIndex3rowEv(qthis: *mut c_void) -> c_int;
  // proto:  quintptr QPersistentModelIndex::internalId();
  fn _ZNK21QPersistentModelIndex10internalIdEv(qthis: *mut c_void) -> c_int;
  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
  fn _ZNK21QPersistentModelIndex5modelEv(qthis: *mut c_void);
  // proto:  void QPersistentModelIndex::~QPersistentModelIndex();
  fn _ZN21QPersistentModelIndexD0Ev(qthis: *mut c_void);
  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QModelIndex & index);
  fn _ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPersistentModelIndex::QPersistentModelIndex();
  fn _ZN21QPersistentModelIndexC1Ev(qthis: *mut c_void);
  // proto:  int QPersistentModelIndex::column();
  fn _ZNK21QPersistentModelIndex6columnEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
  fn _ZN21QPersistentModelIndex4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
  fn _ZNK21QPersistentModelIndex5childEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QPersistentModelIndex::isValid();
  fn _ZNK21QPersistentModelIndex7isValidEv(qthis: *mut c_void) -> c_char;
}

// body block begin
// class sizeof(QPersistentModelIndex)=8
pub struct QPersistentModelIndex {
  pub qclsinst: *mut c_void,
}

  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
impl /*struct*/ QPersistentModelIndex {
  pub fn sibling<RetType, T: QPersistentModelIndex_sibling<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_sibling<RetType> {
  fn sibling(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_sibling<QModelIndex> for (i32, i32) {
  fn sibling(self , rsthis: &mut QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex7siblingEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QPersistentModelIndex::data(int role);
impl /*struct*/ QPersistentModelIndex {
  pub fn data<RetType, T: QPersistentModelIndex_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_data<RetType> {
  fn data(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  QVariant QPersistentModelIndex::data(int role);
impl<'a> /*trait*/ QPersistentModelIndex_data<QVariant> for (i32) {
  fn data(self , rsthis: &mut QPersistentModelIndex) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QPersistentModelIndex::parent();
impl /*struct*/ QPersistentModelIndex {
  pub fn parent<RetType, T: QPersistentModelIndex_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_parent<RetType> {
  fn parent(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  QModelIndex QPersistentModelIndex::parent();
impl<'a> /*trait*/ QPersistentModelIndex_parent<QModelIndex> for () {
  fn parent(self , rsthis: &mut QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6parentEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex & other);
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

  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for (QPersistentModelIndex) {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QPersistentModelIndexC1ERKS_(qthis, arg0)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void * QPersistentModelIndex::internalPointer();
impl /*struct*/ QPersistentModelIndex {
  pub fn internalPointer<RetType, T: QPersistentModelIndex_internalPointer<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.internalPointer(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_internalPointer<RetType> {
  fn internalPointer(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  void * QPersistentModelIndex::internalPointer();
impl<'a> /*trait*/ QPersistentModelIndex_internalPointer<()> for () {
  fn internalPointer(self , rsthis: &mut QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex15internalPointerEv()};
     unsafe {_ZNK21QPersistentModelIndex15internalPointerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QPersistentModelIndex::row();
impl /*struct*/ QPersistentModelIndex {
  pub fn row<RetType, T: QPersistentModelIndex_row<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_row<RetType> {
  fn row(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  int QPersistentModelIndex::row();
impl<'a> /*trait*/ QPersistentModelIndex_row<i32> for () {
  fn row(self , rsthis: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex3rowEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  quintptr QPersistentModelIndex::internalId();
impl /*struct*/ QPersistentModelIndex {
  pub fn internalId<RetType, T: QPersistentModelIndex_internalId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.internalId(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_internalId<RetType> {
  fn internalId(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  quintptr QPersistentModelIndex::internalId();
impl<'a> /*trait*/ QPersistentModelIndex_internalId<i32> for () {
  fn internalId(self , rsthis: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex10internalIdEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex10internalIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
impl /*struct*/ QPersistentModelIndex {
  pub fn model<RetType, T: QPersistentModelIndex_model<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_model<RetType> {
  fn model(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
impl<'a> /*trait*/ QPersistentModelIndex_model<()> for () {
  fn model(self , rsthis: &mut QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5modelEv()};
     unsafe {_ZNK21QPersistentModelIndex5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::~QPersistentModelIndex();
impl /*struct*/ QPersistentModelIndex {
  pub fn FreeQPersistentModelIndex<RetType, T: QPersistentModelIndex_FreeQPersistentModelIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPersistentModelIndex(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_FreeQPersistentModelIndex<RetType> {
  fn FreeQPersistentModelIndex(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  void QPersistentModelIndex::~QPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_FreeQPersistentModelIndex<()> for () {
  fn FreeQPersistentModelIndex(self , rsthis: &mut QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexD0Ev()};
     unsafe {_ZN21QPersistentModelIndexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QPersistentModelIndex_NewQPersistentModelIndex for (QModelIndex) {
  fn NewQPersistentModelIndex(self) -> QPersistentModelIndex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QPersistentModelIndex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex();
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

  // proto:  int QPersistentModelIndex::column();
impl /*struct*/ QPersistentModelIndex {
  pub fn column<RetType, T: QPersistentModelIndex_column<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_column<RetType> {
  fn column(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  int QPersistentModelIndex::column();
impl<'a> /*trait*/ QPersistentModelIndex_column<i32> for () {
  fn column(self , rsthis: &mut QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6columnEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
impl /*struct*/ QPersistentModelIndex {
  pub fn swap<RetType, T: QPersistentModelIndex_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_swap<RetType> {
  fn swap(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_swap<()> for (QPersistentModelIndex) {
  fn swap(self , rsthis: &mut QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndex4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QPersistentModelIndex4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
impl /*struct*/ QPersistentModelIndex {
  pub fn child<RetType, T: QPersistentModelIndex_child<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_child<RetType> {
  fn child(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_child<QModelIndex> for (i32, i32) {
  fn child(self , rsthis: &mut QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPersistentModelIndex::isValid();
impl /*struct*/ QPersistentModelIndex {
  pub fn isValid<RetType, T: QPersistentModelIndex_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_isValid<RetType> {
  fn isValid(self , rsthis: &mut QPersistentModelIndex) -> RetType;
}

  // proto:  bool QPersistentModelIndex::isValid();
impl<'a> /*trait*/ QPersistentModelIndex_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QPersistentModelIndex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7isValidEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

