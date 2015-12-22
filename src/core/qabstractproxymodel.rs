// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qabstractproxymodel.h
// dst-file: /src/core/qabstractproxymodel.rs
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
use super::qabstractitemmodel::QAbstractItemModel; // 773
use std::ops::Deref;
use super::qmimedata::QMimeData; // 773
use super::qabstractitemmodel::QModelIndex; // 773
use super::qitemselectionmodel::QItemSelection; // 773
use super::qvariant::QVariant; // 773
use super::qsize::QSize; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QStringList QAbstractProxyModel::mimeTypes();
  fn _ZNK19QAbstractProxyModel9mimeTypesEv(qthis: *mut c_void);
  // proto:  void QAbstractProxyModel::revert();
  fn _ZN19QAbstractProxyModel6revertEv(qthis: *mut c_void);
  // proto:  QItemSelection QAbstractProxyModel::mapSelectionToSource(const QItemSelection & selection);
  fn _ZNK19QAbstractProxyModel20mapSelectionToSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QItemSelection QAbstractProxyModel::mapSelectionFromSource(const QItemSelection & selection);
  fn _ZNK19QAbstractProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariant QAbstractProxyModel::data(const QModelIndex & proxyIndex, int role);
  fn _ZNK19QAbstractProxyModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QAbstractProxyModel::submit();
  fn _ZN19QAbstractProxyModel6submitEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QAbstractProxyModel::span(const QModelIndex & index);
  fn _ZNK19QAbstractProxyModel4spanERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractProxyModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK19QAbstractProxyModel12canFetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QAbstractProxyModel::metaObject();
  fn _ZNK19QAbstractProxyModel10metaObjectEv(qthis: *mut c_void);
  // proto:  QModelIndex QAbstractProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn _ZNK19QAbstractProxyModel11mapToSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QAbstractProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn _ZNK19QAbstractProxyModel13mapFromSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMap<int, QVariant> QAbstractProxyModel::itemData(const QModelIndex & index);
  fn _ZNK19QAbstractProxyModel8itemDataERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QModelIndex QAbstractProxyModel::buddy(const QModelIndex & index);
  fn _ZNK19QAbstractProxyModel5buddyERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QAbstractProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK19QAbstractProxyModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN19QAbstractProxyModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  void QAbstractProxyModel::fetchMore(const QModelIndex & parent);
  fn _ZN19QAbstractProxyModel9fetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractProxyModel::~QAbstractProxyModel();
  fn _ZN19QAbstractProxyModelD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractProxyModel::QAbstractProxyModel(QObject * parent);
  fn _ZN19QAbstractProxyModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QAbstractProxyModel::hasChildren(const QModelIndex & parent);
  fn _ZNK19QAbstractProxyModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QAbstractItemModel * QAbstractProxyModel::sourceModel();
  fn _ZNK19QAbstractProxyModel11sourceModelEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractProxyModel)=1
pub struct QAbstractProxyModel {
  qbase: QAbstractItemModel,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAbstractProxyModel {
  pub fn inheritFrom(qthis: *mut c_void) -> QAbstractProxyModel {
    return QAbstractProxyModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAbstractProxyModel {
  type Target = QAbstractItemModel;

  fn deref(&self) -> &QAbstractItemModel {
    return &self.qbase;
  }
}
impl AsRef<QAbstractItemModel> for QAbstractProxyModel {
  fn as_ref(&self) -> &QAbstractItemModel {
    return &self.qbase;
  }
}
  // proto:  QStringList QAbstractProxyModel::mimeTypes();
impl /*struct*/ QAbstractProxyModel {
  pub fn mimeTypes<RetType, T: QAbstractProxyModel_mimeTypes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QStringList QAbstractProxyModel::mimeTypes();
impl<'a> /*trait*/ QAbstractProxyModel_mimeTypes<()> for () {
  fn mimeTypes(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel9mimeTypesEv()};
     unsafe {_ZNK19QAbstractProxyModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractProxyModel::revert();
impl /*struct*/ QAbstractProxyModel {
  pub fn revert<RetType, T: QAbstractProxyModel_revert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.revert(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_revert<RetType> {
  fn revert(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  void QAbstractProxyModel::revert();
impl<'a> /*trait*/ QAbstractProxyModel_revert<()> for () {
  fn revert(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractProxyModel6revertEv()};
     unsafe {_ZN19QAbstractProxyModel6revertEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QItemSelection QAbstractProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl /*struct*/ QAbstractProxyModel {
  pub fn mapSelectionToSource<RetType, T: QAbstractProxyModel_mapSelectionToSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_mapSelectionToSource<RetType> {
  fn mapSelectionToSource(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QItemSelection QAbstractProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl<'a> /*trait*/ QAbstractProxyModel_mapSelectionToSource<QItemSelection> for (QItemSelection) {
  fn mapSelectionToSource(self , rsthis: &mut QAbstractProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QItemSelection QAbstractProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl /*struct*/ QAbstractProxyModel {
  pub fn mapSelectionFromSource<RetType, T: QAbstractProxyModel_mapSelectionFromSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_mapSelectionFromSource<RetType> {
  fn mapSelectionFromSource(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QItemSelection QAbstractProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl<'a> /*trait*/ QAbstractProxyModel_mapSelectionFromSource<QItemSelection> for (QItemSelection) {
  fn mapSelectionFromSource(self , rsthis: &mut QAbstractProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QAbstractProxyModel::data(const QModelIndex & proxyIndex, int role);
impl /*struct*/ QAbstractProxyModel {
  pub fn data<RetType, T: QAbstractProxyModel_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_data<RetType> {
  fn data(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QVariant QAbstractProxyModel::data(const QModelIndex & proxyIndex, int role);
impl<'a> /*trait*/ QAbstractProxyModel_data<QVariant> for (QModelIndex, i32) {
  fn data(self , rsthis: &mut QAbstractProxyModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractProxyModel::submit();
impl /*struct*/ QAbstractProxyModel {
  pub fn submit<RetType, T: QAbstractProxyModel_submit<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.submit(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_submit<RetType> {
  fn submit(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  bool QAbstractProxyModel::submit();
impl<'a> /*trait*/ QAbstractProxyModel_submit<i8> for () {
  fn submit(self , rsthis: &mut QAbstractProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractProxyModel6submitEv()};
    let mut ret = unsafe {_ZN19QAbstractProxyModel6submitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QAbstractProxyModel::span(const QModelIndex & index);
impl /*struct*/ QAbstractProxyModel {
  pub fn span<RetType, T: QAbstractProxyModel_span<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.span(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_span<RetType> {
  fn span(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QSize QAbstractProxyModel::span(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractProxyModel_span<QSize> for (QModelIndex) {
  fn span(self , rsthis: &mut QAbstractProxyModel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel4spanERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel4spanERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractProxyModel::canFetchMore(const QModelIndex & parent);
impl /*struct*/ QAbstractProxyModel {
  pub fn canFetchMore<RetType, T: QAbstractProxyModel_canFetchMore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_canFetchMore<RetType> {
  fn canFetchMore(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  bool QAbstractProxyModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractProxyModel_canFetchMore<i8> for (QModelIndex) {
  fn canFetchMore(self , rsthis: &mut QAbstractProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractProxyModel::metaObject();
impl /*struct*/ QAbstractProxyModel {
  pub fn metaObject<RetType, T: QAbstractProxyModel_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  const QMetaObject * QAbstractProxyModel::metaObject();
impl<'a> /*trait*/ QAbstractProxyModel_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel10metaObjectEv()};
     unsafe {_ZNK19QAbstractProxyModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl /*struct*/ QAbstractProxyModel {
  pub fn mapToSource<RetType, T: QAbstractProxyModel_mapToSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToSource(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_mapToSource<RetType> {
  fn mapToSource(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QModelIndex QAbstractProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QAbstractProxyModel_mapToSource<QModelIndex> for (QModelIndex) {
  fn mapToSource(self , rsthis: &mut QAbstractProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl /*struct*/ QAbstractProxyModel {
  pub fn mapFromSource<RetType, T: QAbstractProxyModel_mapFromSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_mapFromSource<RetType> {
  fn mapFromSource(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QModelIndex QAbstractProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QAbstractProxyModel_mapFromSource<QModelIndex> for (QModelIndex) {
  fn mapFromSource(self , rsthis: &mut QAbstractProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QMap<int, QVariant> QAbstractProxyModel::itemData(const QModelIndex & index);
impl /*struct*/ QAbstractProxyModel {
  pub fn itemData<RetType, T: QAbstractProxyModel_itemData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemData(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_itemData<RetType> {
  fn itemData(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QMap<int, QVariant> QAbstractProxyModel::itemData(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractProxyModel_itemData<()> for (QModelIndex) {
  fn itemData(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel8itemDataERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK19QAbstractProxyModel8itemDataERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractProxyModel::buddy(const QModelIndex & index);
impl /*struct*/ QAbstractProxyModel {
  pub fn buddy<RetType, T: QAbstractProxyModel_buddy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.buddy(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_buddy<RetType> {
  fn buddy(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QModelIndex QAbstractProxyModel::buddy(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractProxyModel_buddy<QModelIndex> for (QModelIndex) {
  fn buddy(self , rsthis: &mut QAbstractProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel5buddyERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel5buddyERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QAbstractProxyModel {
  pub fn sibling<RetType, T: QAbstractProxyModel_sibling<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_sibling<RetType> {
  fn sibling(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QModelIndex QAbstractProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QAbstractProxyModel_sibling<QModelIndex> for (i32, i32, QModelIndex) {
  fn sibling(self , rsthis: &mut QAbstractProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QAbstractProxyModel {
  pub fn setData<RetType, T: QAbstractProxyModel_setData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_setData<RetType> {
  fn setData(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  bool QAbstractProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QAbstractProxyModel_setData<i8> for (QModelIndex, QVariant, i32) {
  fn setData(self , rsthis: &mut QAbstractProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractProxyModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN19QAbstractProxyModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractProxyModel::fetchMore(const QModelIndex & parent);
impl /*struct*/ QAbstractProxyModel {
  pub fn fetchMore<RetType, T: QAbstractProxyModel_fetchMore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fetchMore(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_fetchMore<RetType> {
  fn fetchMore(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  void QAbstractProxyModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractProxyModel_fetchMore<()> for (QModelIndex) {
  fn fetchMore(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractProxyModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QAbstractProxyModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractProxyModel::~QAbstractProxyModel();
impl /*struct*/ QAbstractProxyModel {
  pub fn FreeQAbstractProxyModel<RetType, T: QAbstractProxyModel_FreeQAbstractProxyModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractProxyModel(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_FreeQAbstractProxyModel<RetType> {
  fn FreeQAbstractProxyModel(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  void QAbstractProxyModel::~QAbstractProxyModel();
impl<'a> /*trait*/ QAbstractProxyModel_FreeQAbstractProxyModel<()> for () {
  fn FreeQAbstractProxyModel(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractProxyModelD0Ev()};
     unsafe {_ZN19QAbstractProxyModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractProxyModel::QAbstractProxyModel(QObject * parent);
impl /*struct*/ QAbstractProxyModel {
  pub fn NewQAbstractProxyModel<T: QAbstractProxyModel_NewQAbstractProxyModel>(value: T) -> QAbstractProxyModel {
    let rsthis = value.NewQAbstractProxyModel();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractProxyModel_NewQAbstractProxyModel {
  fn NewQAbstractProxyModel(self) -> QAbstractProxyModel;
}

  // proto:  void QAbstractProxyModel::QAbstractProxyModel(QObject * parent);
impl<'a> /*trait*/ QAbstractProxyModel_NewQAbstractProxyModel for (QObject) {
  fn NewQAbstractProxyModel(self) -> QAbstractProxyModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractProxyModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QAbstractProxyModelC1EP7QObject(qthis, arg0)};
    let rsthis = QAbstractProxyModel{/**/qbase: QAbstractItemModel::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAbstractProxyModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QAbstractProxyModel {
  pub fn hasChildren<RetType, T: QAbstractProxyModel_hasChildren<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  bool QAbstractProxyModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractProxyModel_hasChildren<i8> for (QModelIndex) {
  fn hasChildren(self , rsthis: &mut QAbstractProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractProxyModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QAbstractItemModel * QAbstractProxyModel::sourceModel();
impl /*struct*/ QAbstractProxyModel {
  pub fn sourceModel<RetType, T: QAbstractProxyModel_sourceModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sourceModel(self);
    // return 1;
  }
}

pub trait QAbstractProxyModel_sourceModel<RetType> {
  fn sourceModel(self , rsthis: &mut QAbstractProxyModel) -> RetType;
}

  // proto:  QAbstractItemModel * QAbstractProxyModel::sourceModel();
impl<'a> /*trait*/ QAbstractProxyModel_sourceModel<()> for () {
  fn sourceModel(self , rsthis: &mut QAbstractProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractProxyModel11sourceModelEv()};
     unsafe {_ZNK19QAbstractProxyModel11sourceModelEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

