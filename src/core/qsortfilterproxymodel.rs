// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qsortfilterproxymodel.h
// dst-file: /src/core/qsortfilterproxymodel.rs
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
use super::qstring::*; // 773
use super::qabstractitemmodel::*; // 773
use super::qsize::*; // 773
use super::qvariant::*; // 773
use super::qitemselectionmodel::*; // 773
use super::qstringlist::*; // 773
use super::qobjectdefs::*; // 773
use super::qmimedata::*; // 773
use super::qregexp::*; // 773
use super::qobject::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSortFilterProxyModel_Class_Size() -> c_int;
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
  fn C_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
  fn C_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn C_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
  fn C_ZNK21QSortFilterProxyModel4spanERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn C_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
  fn C_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
  fn C_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
  fn C_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn C_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  void QSortFilterProxyModel::setSortRole(int role);
  fn C_ZN21QSortFilterProxyModel11setSortRoleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
  fn C_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::invalidate();
  fn C_ZN21QSortFilterProxyModel10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QSortFilterProxyModel::sortColumn();
  fn C_ZNK21QSortFilterProxyModel10sortColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
  fn C_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  int QSortFilterProxyModel::filterKeyColumn();
  fn C_ZNK21QSortFilterProxyModel15filterKeyColumnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
  fn C_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
  fn C_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
  fn C_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
  fn C_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
  fn C_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
  fn C_ZNK21QSortFilterProxyModel9mimeTypesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
  fn C_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QSortFilterProxyModel::filterRole();
  fn C_ZNK21QSortFilterProxyModel10filterRoleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSortFilterProxyModel::clear();
  fn C_ZN21QSortFilterProxyModel5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
  fn C_ZN21QSortFilterProxyModel18setFilterKeyColumnEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
  fn C_ZNK21QSortFilterProxyModel10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QSortFilterProxyModel::sortRole();
  fn C_ZNK21QSortFilterProxyModel8sortRoleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
  fn C_ZN21QSortFilterProxyModel18setSortLocaleAwareEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn C_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setSourceModel(QAbstractItemModel * sourceModel);
  fn C_ZN21QSortFilterProxyModel14setSourceModelEP18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn C_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QSortFilterProxyModel::~QSortFilterProxyModel();
  fn C_ZN21QSortFilterProxyModelD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
  fn C_ZNK21QSortFilterProxyModel17dynamicSortFilterEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn C_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
  fn C_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
  fn C_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
  fn C_ZNK21QSortFilterProxyModel6parentERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
  fn C_ZNK21QSortFilterProxyModel12filterRegExpEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
  fn C_ZN21QSortFilterProxyModel13setFilterRoleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(QObject * parent);
  fn C_ZN21QSortFilterProxyModelC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
  fn C_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
  fn C_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
  fn C_ZN21QSortFilterProxyModel20setDynamicSortFilterEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QSortFilterProxyModel)=1
#[derive(Default)]
pub struct QSortFilterProxyModel {
  qbase: QAbstractProxyModel,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSortFilterProxyModel {
    return QSortFilterProxyModel{qbase: QAbstractProxyModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSortFilterProxyModel {
  type Target = QAbstractProxyModel;

  fn deref(&self) -> &QAbstractProxyModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractProxyModel> for QSortFilterProxyModel {
  fn as_ref(& self) -> & QAbstractProxyModel {
    return & self.qbase;
  }
}
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRegExp<RetType, T: QSortFilterProxyModel_setFilterRegExp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilterRegExp(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRegExp<RetType> {
  fn setFilterRegExp(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp<()> for (&'a QString) {
  fn setFilterRegExp(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn rowCount<RetType, T: QSortFilterProxyModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_rowCount<i32> for (&'a QModelIndex) {
  fn rowCount(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QSortFilterProxyModel {
  pub fn sibling<RetType, T: QSortFilterProxyModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sibling<RetType> {
  fn sibling(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QSortFilterProxyModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
impl /*struct*/ QSortFilterProxyModel {
  pub fn span<RetType, T: QSortFilterProxyModel_span<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.span(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_span<RetType> {
  fn span(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_span<QSize> for (&'a QModelIndex) {
  fn span(self , rsthis: & QSortFilterProxyModel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4spanERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel4spanERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapFromSource<RetType, T: QSortFilterProxyModel_mapFromSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapFromSource<RetType> {
  fn mapFromSource(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapFromSource<QModelIndex> for (&'a QModelIndex) {
  fn mapFromSource(self , rsthis: & QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterWildcard<RetType, T: QSortFilterProxyModel_setFilterWildcard<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilterWildcard(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterWildcard<RetType> {
  fn setFilterWildcard(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterWildcard<()> for (&'a QString) {
  fn setFilterWildcard(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn hasChildren<RetType, T: QSortFilterProxyModel_hasChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_hasChildren<i8> for (&'a QModelIndex) {
  fn hasChildren(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterFixedString<RetType, T: QSortFilterProxyModel_setFilterFixedString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilterFixedString(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterFixedString<RetType> {
  fn setFilterFixedString(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterFixedString<()> for (&'a QString) {
  fn setFilterFixedString(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setData<RetType, T: QSortFilterProxyModel_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setData<RetType> {
  fn setData(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setData<i8> for (&'a QModelIndex, &'a QVariant, i32) {
  fn setData(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setSortRole(int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortRole<RetType, T: QSortFilterProxyModel_setSortRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSortRole<RetType> {
  fn setSortRole(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setSortRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortRole<()> for (i32) {
  fn setSortRole(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel11setSortRoleEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN21QSortFilterProxyModel11setSortRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn data<RetType, T: QSortFilterProxyModel_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_data<RetType> {
  fn data(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_data<QVariant> for (&'a QModelIndex, i32) {
  fn data(self , rsthis: & QSortFilterProxyModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::invalidate();
impl /*struct*/ QSortFilterProxyModel {
  pub fn invalidate<RetType, T: QSortFilterProxyModel_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_invalidate<RetType> {
  fn invalidate(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::invalidate();
impl<'a> /*trait*/ QSortFilterProxyModel_invalidate<()> for () {
  fn invalidate(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10invalidateEv()};
     unsafe {C_ZN21QSortFilterProxyModel10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::sortColumn();
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortColumn<RetType, T: QSortFilterProxyModel_sortColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sortColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sortColumn<RetType> {
  fn sortColumn(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::sortColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_sortColumn<i32> for () {
  fn sortColumn(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10sortColumnEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel10sortColumnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn insertRows<RetType, T: QSortFilterProxyModel_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_insertRows<RetType> {
  fn insertRows(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertRows<i8> for (i32, i32, &'a QModelIndex) {
  fn insertRows(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::filterKeyColumn();
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterKeyColumn<RetType, T: QSortFilterProxyModel_filterKeyColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filterKeyColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterKeyColumn<RetType> {
  fn filterKeyColumn(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::filterKeyColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_filterKeyColumn<i32> for () {
  fn filterKeyColumn(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel15filterKeyColumnEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel15filterKeyColumnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn canFetchMore<RetType, T: QSortFilterProxyModel_canFetchMore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_canFetchMore<RetType> {
  fn canFetchMore(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_canFetchMore<i8> for (&'a QModelIndex) {
  fn canFetchMore(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
impl /*struct*/ QSortFilterProxyModel {
  pub fn isSortLocaleAware<RetType, T: QSortFilterProxyModel_isSortLocaleAware<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSortLocaleAware(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_isSortLocaleAware<RetType> {
  fn isSortLocaleAware(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
impl<'a> /*trait*/ QSortFilterProxyModel_isSortLocaleAware<i8> for () {
  fn isSortLocaleAware(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn fetchMore<RetType, T: QSortFilterProxyModel_fetchMore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fetchMore(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_fetchMore<RetType> {
  fn fetchMore(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_fetchMore<()> for (&'a QModelIndex) {
  fn fetchMore(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionFromSource<RetType, T: QSortFilterProxyModel_mapSelectionFromSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionFromSource<RetType> {
  fn mapSelectionFromSource(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionFromSource<QItemSelection> for (&'a QItemSelection) {
  fn mapSelectionFromSource(self , rsthis: & QSortFilterProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionToSource<RetType, T: QSortFilterProxyModel_mapSelectionToSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionToSource<RetType> {
  fn mapSelectionToSource(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionToSource<QItemSelection> for (&'a QItemSelection) {
  fn mapSelectionToSource(self , rsthis: & QSortFilterProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
impl /*struct*/ QSortFilterProxyModel {
  pub fn mimeTypes<RetType, T: QSortFilterProxyModel_mimeTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
impl<'a> /*trait*/ QSortFilterProxyModel_mimeTypes<QStringList> for () {
  fn mimeTypes(self , rsthis: & QSortFilterProxyModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel9mimeTypesEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel9mimeTypesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
impl /*struct*/ QSortFilterProxyModel {
  pub fn buddy<RetType, T: QSortFilterProxyModel_buddy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buddy(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_buddy<RetType> {
  fn buddy(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_buddy<QModelIndex> for (&'a QModelIndex) {
  fn buddy(self , rsthis: & QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::filterRole();
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRole<RetType, T: QSortFilterProxyModel_filterRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filterRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterRole<RetType> {
  fn filterRole(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::filterRole();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRole<i32> for () {
  fn filterRole(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10filterRoleEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel10filterRoleEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::clear();
impl /*struct*/ QSortFilterProxyModel {
  pub fn clear<RetType, T: QSortFilterProxyModel_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_clear<RetType> {
  fn clear(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::clear();
impl<'a> /*trait*/ QSortFilterProxyModel_clear<()> for () {
  fn clear(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel5clearEv()};
     unsafe {C_ZN21QSortFilterProxyModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterKeyColumn<RetType, T: QSortFilterProxyModel_setFilterKeyColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilterKeyColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterKeyColumn<RetType> {
  fn setFilterKeyColumn(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterKeyColumn<()> for (i32) {
  fn setFilterKeyColumn(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setFilterKeyColumnEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN21QSortFilterProxyModel18setFilterKeyColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
impl /*struct*/ QSortFilterProxyModel {
  pub fn metaObject<RetType, T: QSortFilterProxyModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
impl<'a> /*trait*/ QSortFilterProxyModel_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSortFilterProxyModel) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10metaObjectEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::sortRole();
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortRole<RetType, T: QSortFilterProxyModel_sortRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sortRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sortRole<RetType> {
  fn sortRole(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::sortRole();
impl<'a> /*trait*/ QSortFilterProxyModel_sortRole<i32> for () {
  fn sortRole(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8sortRoleEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel8sortRoleEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortLocaleAware<RetType, T: QSortFilterProxyModel_setSortLocaleAware<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSortLocaleAware(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSortLocaleAware<RetType> {
  fn setSortLocaleAware(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortLocaleAware<()> for (i8) {
  fn setSortLocaleAware(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setSortLocaleAwareEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN21QSortFilterProxyModel18setSortLocaleAwareEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapToSource<RetType, T: QSortFilterProxyModel_mapToSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapToSource<RetType> {
  fn mapToSource(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapToSource<QModelIndex> for (&'a QModelIndex) {
  fn mapToSource(self , rsthis: & QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setSourceModel(QAbstractItemModel * sourceModel);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSourceModel<RetType, T: QSortFilterProxyModel_setSourceModel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSourceModel(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSourceModel<RetType> {
  fn setSourceModel(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setSourceModel(QAbstractItemModel * sourceModel);
impl<'a> /*trait*/ QSortFilterProxyModel_setSourceModel<()> for (&'a QAbstractItemModel) {
  fn setSourceModel(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel14setSourceModelEP18QAbstractItemModel()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QSortFilterProxyModel14setSourceModelEP18QAbstractItemModel(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn removeColumns<RetType, T: QSortFilterProxyModel_removeColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeColumns<i8> for (i32, i32, &'a QModelIndex) {
  fn removeColumns(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::~QSortFilterProxyModel();
impl /*struct*/ QSortFilterProxyModel {
  pub fn free<RetType, T: QSortFilterProxyModel_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_free<RetType> {
  fn free(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::~QSortFilterProxyModel();
impl<'a> /*trait*/ QSortFilterProxyModel_free<()> for () {
  fn free(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelD2Ev()};
     unsafe {C_ZN21QSortFilterProxyModelD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
impl /*struct*/ QSortFilterProxyModel {
  pub fn dynamicSortFilter<RetType, T: QSortFilterProxyModel_dynamicSortFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dynamicSortFilter(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_dynamicSortFilter<RetType> {
  fn dynamicSortFilter(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
impl<'a> /*trait*/ QSortFilterProxyModel_dynamicSortFilter<i8> for () {
  fn dynamicSortFilter(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17dynamicSortFilterEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel17dynamicSortFilterEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn insertColumns<RetType, T: QSortFilterProxyModel_insertColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertColumns<i8> for (i32, i32, &'a QModelIndex) {
  fn insertColumns(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn columnCount<RetType, T: QSortFilterProxyModel_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_columnCount<RetType> {
  fn columnCount(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_columnCount<i32> for (&'a QModelIndex) {
  fn columnCount(self , rsthis: & QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp<()> for (&'a QRegExp) {
  fn setFilterRegExp(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
impl /*struct*/ QSortFilterProxyModel {
  pub fn parent<RetType, T: QSortFilterProxyModel_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_parent<RetType> {
  fn parent(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QSortFilterProxyModel_parent<QModelIndex> for (&'a QModelIndex) {
  fn parent(self , rsthis: & QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRegExp<RetType, T: QSortFilterProxyModel_filterRegExp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filterRegExp(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterRegExp<RetType> {
  fn filterRegExp(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRegExp<QRegExp> for () {
  fn filterRegExp(self , rsthis: & QSortFilterProxyModel) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12filterRegExpEv()};
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel12filterRegExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRole<RetType, T: QSortFilterProxyModel_setFilterRole<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFilterRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRole<RetType> {
  fn setFilterRole(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRole<()> for (i32) {
  fn setFilterRole(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13setFilterRoleEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN21QSortFilterProxyModel13setFilterRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(QObject * parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn new<T: QSortFilterProxyModel_new>(value: T) -> QSortFilterProxyModel {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSortFilterProxyModel_new {
  fn new(self) -> QSortFilterProxyModel;
}

  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(QObject * parent);
impl<'a> /*trait*/ QSortFilterProxyModel_new for (&'a QObject) {
  fn new(self) -> QSortFilterProxyModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelC2EP7QObject()};
    let ctysz: c_int = unsafe{QSortFilterProxyModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QSortFilterProxyModelC2EP7QObject(arg0)};
    let rsthis = QSortFilterProxyModel{qbase: QAbstractProxyModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn removeRows<RetType, T: QSortFilterProxyModel_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_removeRows<RetType> {
  fn removeRows(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeRows<i8> for (i32, i32, &'a QModelIndex) {
  fn removeRows(self , rsthis: & QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn index<RetType, T: QSortFilterProxyModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_index<RetType> {
  fn index(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setDynamicSortFilter<RetType, T: QSortFilterProxyModel_setDynamicSortFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDynamicSortFilter(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setDynamicSortFilter<RetType> {
  fn setDynamicSortFilter(self , rsthis: & QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
impl<'a> /*trait*/ QSortFilterProxyModel_setDynamicSortFilter<()> for (i8) {
  fn setDynamicSortFilter(self , rsthis: & QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setDynamicSortFilterEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN21QSortFilterProxyModel20setDynamicSortFilterEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

