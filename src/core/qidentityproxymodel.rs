// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qidentityproxymodel.h
// dst-file: /src/core/qidentityproxymodel.rs
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
use super::qabstractproxymodel::*; // 773
use std::ops::Deref;
use super::qabstractitemmodel::*; // 773
use super::qitemselectionmodel::*; // 773
use super::qvariant::*; // 773
use super::qobjectdefs::*; // 773
use super::qmimedata::*; // 773
use super::qobject::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QIdentityProxyModel_Class_Size() -> c_int;
  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
  fn C_ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn C_ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
  fn C_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
  fn C_ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
  fn C_ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn C_ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn C_ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
  fn C_ZNK19QIdentityProxyModel10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QIdentityProxyModel::~QIdentityProxyModel();
  fn C_ZN19QIdentityProxyModelD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
  fn C_ZNK19QIdentityProxyModel6parentERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QIdentityProxyModel::setSourceModel(QAbstractItemModel * sourceModel);
  fn C_ZN19QIdentityProxyModel14setSourceModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn C_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn C_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QIdentityProxyModel::QIdentityProxyModel(QObject * parent);
  fn C_ZN19QIdentityProxyModelC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
  fn C_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
  fn C_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
  fn C_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QIdentityProxyModel)=1
#[derive(Default)]
pub struct QIdentityProxyModel {
  qbase: QAbstractProxyModel,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIdentityProxyModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIdentityProxyModel {
    return QIdentityProxyModel{qbase: QAbstractProxyModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QIdentityProxyModel {
  type Target = QAbstractProxyModel;

  fn deref(&self) -> &QAbstractProxyModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractProxyModel> for QIdentityProxyModel {
  fn as_ref(& self) -> & QAbstractProxyModel {
    return & self.qbase;
  }
}
  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn removeRows<RetType, T: QIdentityProxyModel_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_removeRows<RetType> {
  fn removeRows(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_removeRows<i8> for (i32, i32, &'a QModelIndex) {
  fn removeRows(self , rsthis: & QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn removeColumns<RetType, T: QIdentityProxyModel_removeColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_removeColumns<i8> for (i32, i32, &'a QModelIndex) {
  fn removeColumns(self , rsthis: & QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionFromSource<RetType, T: QIdentityProxyModel_mapSelectionFromSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapSelectionFromSource<RetType> {
  fn mapSelectionFromSource(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionFromSource<QItemSelection> for (&'a QItemSelection) {
  fn mapSelectionFromSource(self , rsthis: & QIdentityProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn index<RetType, T: QIdentityProxyModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_index<RetType> {
  fn index(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn insertRows<RetType, T: QIdentityProxyModel_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_insertRows<RetType> {
  fn insertRows(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_insertRows<i8> for (i32, i32, &'a QModelIndex) {
  fn insertRows(self , rsthis: & QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn insertColumns<RetType, T: QIdentityProxyModel_insertColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_insertColumns<i8> for (i32, i32, &'a QModelIndex) {
  fn insertColumns(self , rsthis: & QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QIdentityProxyModel {
  pub fn sibling<RetType, T: QIdentityProxyModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_sibling<RetType> {
  fn sibling(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QIdentityProxyModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
impl /*struct*/ QIdentityProxyModel {
  pub fn metaObject<RetType, T: QIdentityProxyModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
impl<'a> /*trait*/ QIdentityProxyModel_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QIdentityProxyModel) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel10metaObjectEv()};
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIdentityProxyModel::~QIdentityProxyModel();
impl /*struct*/ QIdentityProxyModel {
  pub fn free<RetType, T: QIdentityProxyModel_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_free<RetType> {
  fn free(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  void QIdentityProxyModel::~QIdentityProxyModel();
impl<'a> /*trait*/ QIdentityProxyModel_free<()> for () {
  fn free(self , rsthis: & QIdentityProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModelD2Ev()};
     unsafe {C_ZN19QIdentityProxyModelD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
impl /*struct*/ QIdentityProxyModel {
  pub fn parent<RetType, T: QIdentityProxyModel_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_parent<RetType> {
  fn parent(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QIdentityProxyModel_parent<QModelIndex> for (&'a QModelIndex) {
  fn parent(self , rsthis: & QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIdentityProxyModel::setSourceModel(QAbstractItemModel * sourceModel);
impl /*struct*/ QIdentityProxyModel {
  pub fn setSourceModel<RetType, T: QIdentityProxyModel_setSourceModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSourceModel(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_setSourceModel<RetType> {
  fn setSourceModel(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  void QIdentityProxyModel::setSourceModel(QAbstractItemModel * sourceModel);
impl<'a> /*trait*/ QIdentityProxyModel_setSourceModel<()> for (&'a QAbstractItemModel) {
  fn setSourceModel(self , rsthis: & QIdentityProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel14setSourceModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QIdentityProxyModel14setSourceModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapToSource<RetType, T: QIdentityProxyModel_mapToSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapToSource<RetType> {
  fn mapToSource(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QIdentityProxyModel_mapToSource<QModelIndex> for (&'a QModelIndex) {
  fn mapToSource(self , rsthis: & QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapFromSource<RetType, T: QIdentityProxyModel_mapFromSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapFromSource<RetType> {
  fn mapFromSource(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QIdentityProxyModel_mapFromSource<QModelIndex> for (&'a QModelIndex) {
  fn mapFromSource(self , rsthis: & QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIdentityProxyModel::QIdentityProxyModel(QObject * parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn new<T: QIdentityProxyModel_new>(value: T) -> QIdentityProxyModel {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIdentityProxyModel_new {
  fn new(self) -> QIdentityProxyModel;
}

  // proto:  void QIdentityProxyModel::QIdentityProxyModel(QObject * parent);
impl<'a> /*trait*/ QIdentityProxyModel_new for (&'a QObject) {
  fn new(self) -> QIdentityProxyModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModelC2EP7QObject()};
    let ctysz: c_int = unsafe{QIdentityProxyModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QIdentityProxyModelC2EP7QObject(arg0)};
    let rsthis = QIdentityProxyModel{qbase: QAbstractProxyModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn columnCount<RetType, T: QIdentityProxyModel_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_columnCount<RetType> {
  fn columnCount(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_columnCount<i32> for (&'a QModelIndex) {
  fn columnCount(self , rsthis: & QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionToSource<RetType, T: QIdentityProxyModel_mapSelectionToSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapSelectionToSource<RetType> {
  fn mapSelectionToSource(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionToSource<QItemSelection> for (&'a QItemSelection) {
  fn mapSelectionToSource(self , rsthis: & QIdentityProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn rowCount<RetType, T: QIdentityProxyModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QIdentityProxyModel) -> RetType;
}

  // proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_rowCount<i32> for (&'a QModelIndex) {
  fn rowCount(self , rsthis: & QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

// <= body block end

