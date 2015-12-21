// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qstring::QString; // 773
use super::qabstractitemmodel::QModelIndex; // 773
use super::qsize::QSize; // 773
use super::qvariant::QVariant; // 773
use super::qitemselectionmodel::QItemSelection; // 773
use super::qmimedata::QMimeData; // 773
use super::qregexp::QRegExp; // 773
use super::qobject::QObject; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
  fn _ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
  fn _ZNK21QSortFilterProxyModel4spanERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn _ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
  fn _ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
  fn _ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  void QSortFilterProxyModel::setSortRole(int role);
  fn _ZN21QSortFilterProxyModel11setSortRoleEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
  fn _ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::invalidate();
  fn _ZN21QSortFilterProxyModel10invalidateEv(qthis: *mut c_void);
  // proto:  int QSortFilterProxyModel::sortColumn();
  fn _ZNK21QSortFilterProxyModel10sortColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  int QSortFilterProxyModel::filterKeyColumn();
  fn _ZNK21QSortFilterProxyModel15filterKeyColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
  fn _ZNK21QSortFilterProxyModel17isSortLocaleAwareEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
  fn _ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
  fn _ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
  fn _ZNK21QSortFilterProxyModel9mimeTypesEv(qthis: *mut c_void);
  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
  fn _ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QSortFilterProxyModel::filterRole();
  fn _ZNK21QSortFilterProxyModel10filterRoleEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::clear();
  fn _ZN21QSortFilterProxyModel5clearEv(qthis: *mut c_void);
  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
  fn _ZN21QSortFilterProxyModel18setFilterKeyColumnEi(qthis: *mut c_void, arg0: c_int);
  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
  fn _ZNK21QSortFilterProxyModel10metaObjectEv(qthis: *mut c_void);
  // proto:  int QSortFilterProxyModel::sortRole();
  fn _ZNK21QSortFilterProxyModel8sortRoleEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
  fn _ZN21QSortFilterProxyModel18setSortLocaleAwareEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn _ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(const QSortFilterProxyModel & );
  fn _ZN21QSortFilterProxyModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QSortFilterProxyModel::~QSortFilterProxyModel();
  fn _ZN21QSortFilterProxyModelD0Ev(qthis: *mut c_void);
  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
  fn _ZNK21QSortFilterProxyModel17dynamicSortFilterEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
  fn _ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
  fn _ZNK21QSortFilterProxyModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
  fn _ZNK21QSortFilterProxyModel12filterRegExpEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
  fn _ZN21QSortFilterProxyModel13setFilterRoleEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(QObject * parent);
  fn _ZN21QSortFilterProxyModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
  fn _ZN21QSortFilterProxyModel20setDynamicSortFilterEb(qthis: *mut c_void, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QSortFilterProxyModel)=1
pub struct QSortFilterProxyModel {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRegExp<RetType, T: QSortFilterProxyModel_setFilterRegExp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFilterRegExp(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRegExp<RetType> {
  fn setFilterRegExp(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp<()> for (QString) {
  fn setFilterRegExp(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn rowCount<RetType, T: QSortFilterProxyModel_rowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_rowCount<i32> for (QModelIndex) {
  fn rowCount(self , rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QSortFilterProxyModel {
  pub fn sibling<RetType, T: QSortFilterProxyModel_sibling<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sibling<RetType> {
  fn sibling(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QSortFilterProxyModel_sibling<QModelIndex> for (i32, i32, QModelIndex) {
  fn sibling(self , rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
impl /*struct*/ QSortFilterProxyModel {
  pub fn span<RetType, T: QSortFilterProxyModel_span<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.span(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_span<RetType> {
  fn span(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_span<QSize> for (QModelIndex) {
  fn span(self , rsthis: &mut QSortFilterProxyModel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4spanERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel4spanERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapFromSource<RetType, T: QSortFilterProxyModel_mapFromSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapFromSource<RetType> {
  fn mapFromSource(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapFromSource<QModelIndex> for (QModelIndex) {
  fn mapFromSource(self , rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterWildcard<RetType, T: QSortFilterProxyModel_setFilterWildcard<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFilterWildcard(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterWildcard<RetType> {
  fn setFilterWildcard(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterWildcard<()> for (QString) {
  fn setFilterWildcard(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn hasChildren<RetType, T: QSortFilterProxyModel_hasChildren<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_hasChildren<i8> for (QModelIndex) {
  fn hasChildren(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterFixedString<RetType, T: QSortFilterProxyModel_setFilterFixedString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFilterFixedString(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterFixedString<RetType> {
  fn setFilterFixedString(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterFixedString<()> for (QString) {
  fn setFilterFixedString(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setData<RetType, T: QSortFilterProxyModel_setData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setData<RetType> {
  fn setData(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setData<i8> for (QModelIndex, QVariant, i32) {
  fn setData(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setSortRole(int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortRole<RetType, T: QSortFilterProxyModel_setSortRole<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSortRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSortRole<RetType> {
  fn setSortRole(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setSortRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortRole<()> for (i32) {
  fn setSortRole(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel11setSortRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QSortFilterProxyModel11setSortRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn data<RetType, T: QSortFilterProxyModel_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_data<RetType> {
  fn data(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_data<QVariant> for (QModelIndex, i32) {
  fn data(self , rsthis: &mut QSortFilterProxyModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::invalidate();
impl /*struct*/ QSortFilterProxyModel {
  pub fn invalidate<RetType, T: QSortFilterProxyModel_invalidate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::invalidate();
impl<'a> /*trait*/ QSortFilterProxyModel_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10invalidateEv()};
     unsafe {_ZN21QSortFilterProxyModel10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::sortColumn();
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortColumn<RetType, T: QSortFilterProxyModel_sortColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sortColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sortColumn<RetType> {
  fn sortColumn(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::sortColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_sortColumn<i32> for () {
  fn sortColumn(self , rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10sortColumnEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel10sortColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn insertRows<RetType, T: QSortFilterProxyModel_insertRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_insertRows<RetType> {
  fn insertRows(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertRows<i8> for (i32, i32, QModelIndex) {
  fn insertRows(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::filterKeyColumn();
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterKeyColumn<RetType, T: QSortFilterProxyModel_filterKeyColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.filterKeyColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterKeyColumn<RetType> {
  fn filterKeyColumn(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::filterKeyColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_filterKeyColumn<i32> for () {
  fn filterKeyColumn(self , rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel15filterKeyColumnEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel15filterKeyColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn canFetchMore<RetType, T: QSortFilterProxyModel_canFetchMore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_canFetchMore<RetType> {
  fn canFetchMore(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_canFetchMore<i8> for (QModelIndex) {
  fn canFetchMore(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
impl /*struct*/ QSortFilterProxyModel {
  pub fn isSortLocaleAware<RetType, T: QSortFilterProxyModel_isSortLocaleAware<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSortLocaleAware(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_isSortLocaleAware<RetType> {
  fn isSortLocaleAware(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
impl<'a> /*trait*/ QSortFilterProxyModel_isSortLocaleAware<i8> for () {
  fn isSortLocaleAware(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn fetchMore<RetType, T: QSortFilterProxyModel_fetchMore<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fetchMore(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_fetchMore<RetType> {
  fn fetchMore(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_fetchMore<()> for (QModelIndex) {
  fn fetchMore(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionFromSource<RetType, T: QSortFilterProxyModel_mapSelectionFromSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionFromSource<RetType> {
  fn mapSelectionFromSource(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionFromSource<QItemSelection> for (QItemSelection) {
  fn mapSelectionFromSource(self , rsthis: &mut QSortFilterProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionToSource<RetType, T: QSortFilterProxyModel_mapSelectionToSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionToSource<RetType> {
  fn mapSelectionToSource(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionToSource<QItemSelection> for (QItemSelection) {
  fn mapSelectionToSource(self , rsthis: &mut QSortFilterProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
impl /*struct*/ QSortFilterProxyModel {
  pub fn mimeTypes<RetType, T: QSortFilterProxyModel_mimeTypes<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
impl<'a> /*trait*/ QSortFilterProxyModel_mimeTypes<()> for () {
  fn mimeTypes(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel9mimeTypesEv()};
     unsafe {_ZNK21QSortFilterProxyModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
impl /*struct*/ QSortFilterProxyModel {
  pub fn buddy<RetType, T: QSortFilterProxyModel_buddy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.buddy(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_buddy<RetType> {
  fn buddy(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_buddy<QModelIndex> for (QModelIndex) {
  fn buddy(self , rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::filterRole();
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRole<RetType, T: QSortFilterProxyModel_filterRole<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.filterRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterRole<RetType> {
  fn filterRole(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::filterRole();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRole<i32> for () {
  fn filterRole(self , rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10filterRoleEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel10filterRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::clear();
impl /*struct*/ QSortFilterProxyModel {
  pub fn clear<RetType, T: QSortFilterProxyModel_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_clear<RetType> {
  fn clear(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::clear();
impl<'a> /*trait*/ QSortFilterProxyModel_clear<()> for () {
  fn clear(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel5clearEv()};
     unsafe {_ZN21QSortFilterProxyModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterKeyColumn<RetType, T: QSortFilterProxyModel_setFilterKeyColumn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFilterKeyColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterKeyColumn<RetType> {
  fn setFilterKeyColumn(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterKeyColumn<()> for (i32) {
  fn setFilterKeyColumn(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setFilterKeyColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QSortFilterProxyModel18setFilterKeyColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
impl /*struct*/ QSortFilterProxyModel {
  pub fn metaObject<RetType, T: QSortFilterProxyModel_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
impl<'a> /*trait*/ QSortFilterProxyModel_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10metaObjectEv()};
     unsafe {_ZNK21QSortFilterProxyModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::sortRole();
impl /*struct*/ QSortFilterProxyModel {
  pub fn sortRole<RetType, T: QSortFilterProxyModel_sortRole<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sortRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sortRole<RetType> {
  fn sortRole(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::sortRole();
impl<'a> /*trait*/ QSortFilterProxyModel_sortRole<i32> for () {
  fn sortRole(self , rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8sortRoleEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel8sortRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortLocaleAware<RetType, T: QSortFilterProxyModel_setSortLocaleAware<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSortLocaleAware(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSortLocaleAware<RetType> {
  fn setSortLocaleAware(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortLocaleAware<()> for (i8) {
  fn setSortLocaleAware(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setSortLocaleAwareEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN21QSortFilterProxyModel18setSortLocaleAwareEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl /*struct*/ QSortFilterProxyModel {
  pub fn mapToSource<RetType, T: QSortFilterProxyModel_mapToSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapToSource<RetType> {
  fn mapToSource(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapToSource<QModelIndex> for (QModelIndex) {
  fn mapToSource(self , rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(const QSortFilterProxyModel & );
impl /*struct*/ QSortFilterProxyModel {
  pub fn NewQSortFilterProxyModel<T: QSortFilterProxyModel_NewQSortFilterProxyModel>(value: T) -> QSortFilterProxyModel {
    let rsthis = value.NewQSortFilterProxyModel();
    return rsthis;
    // return 1;
  }
}

pub trait QSortFilterProxyModel_NewQSortFilterProxyModel {
  fn NewQSortFilterProxyModel(self) -> QSortFilterProxyModel;
}

  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(const QSortFilterProxyModel & );
impl<'a> /*trait*/ QSortFilterProxyModel_NewQSortFilterProxyModel for (QSortFilterProxyModel) {
  fn NewQSortFilterProxyModel(self) -> QSortFilterProxyModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QSortFilterProxyModelC1ERKS_(qthis, arg0)};
    let rsthis = QSortFilterProxyModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn removeColumns<RetType, T: QSortFilterProxyModel_removeColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeColumns<i8> for (i32, i32, QModelIndex) {
  fn removeColumns(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::~QSortFilterProxyModel();
impl /*struct*/ QSortFilterProxyModel {
  pub fn FreeQSortFilterProxyModel<RetType, T: QSortFilterProxyModel_FreeQSortFilterProxyModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSortFilterProxyModel(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_FreeQSortFilterProxyModel<RetType> {
  fn FreeQSortFilterProxyModel(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::~QSortFilterProxyModel();
impl<'a> /*trait*/ QSortFilterProxyModel_FreeQSortFilterProxyModel<()> for () {
  fn FreeQSortFilterProxyModel(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelD0Ev()};
     unsafe {_ZN21QSortFilterProxyModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
impl /*struct*/ QSortFilterProxyModel {
  pub fn dynamicSortFilter<RetType, T: QSortFilterProxyModel_dynamicSortFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.dynamicSortFilter(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_dynamicSortFilter<RetType> {
  fn dynamicSortFilter(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
impl<'a> /*trait*/ QSortFilterProxyModel_dynamicSortFilter<i8> for () {
  fn dynamicSortFilter(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17dynamicSortFilterEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel17dynamicSortFilterEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn insertColumns<RetType, T: QSortFilterProxyModel_insertColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertColumns<i8> for (i32, i32, QModelIndex) {
  fn insertColumns(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn columnCount<RetType, T: QSortFilterProxyModel_columnCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_columnCount<i32> for (QModelIndex) {
  fn columnCount(self , rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp<()> for (QRegExp) {
  fn setFilterRegExp(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
impl /*struct*/ QSortFilterProxyModel {
  pub fn parent<RetType, T: QSortFilterProxyModel_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_parent<RetType> {
  fn parent(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QSortFilterProxyModel_parent<QModelIndex> for (QModelIndex) {
  fn parent(self , rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRegExp<RetType, T: QSortFilterProxyModel_filterRegExp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.filterRegExp(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterRegExp<RetType> {
  fn filterRegExp(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRegExp<QRegExp> for () {
  fn filterRegExp(self , rsthis: &mut QSortFilterProxyModel) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12filterRegExpEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel12filterRegExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRole<RetType, T: QSortFilterProxyModel_setFilterRole<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFilterRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRole<RetType> {
  fn setFilterRole(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRole<()> for (i32) {
  fn setFilterRole(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13setFilterRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QSortFilterProxyModel13setFilterRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::QSortFilterProxyModel(QObject * parent);
impl<'a> /*trait*/ QSortFilterProxyModel_NewQSortFilterProxyModel for (QObject) {
  fn NewQSortFilterProxyModel(self) -> QSortFilterProxyModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QSortFilterProxyModelC1EP7QObject(qthis, arg0)};
    let rsthis = QSortFilterProxyModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn removeRows<RetType, T: QSortFilterProxyModel_removeRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_removeRows<RetType> {
  fn removeRows(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeRows<i8> for (i32, i32, QModelIndex) {
  fn removeRows(self , rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QSortFilterProxyModel {
  pub fn index<RetType, T: QSortFilterProxyModel_index<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_index<RetType> {
  fn index(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_index<QModelIndex> for (i32, i32, QModelIndex) {
  fn index(self , rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
impl /*struct*/ QSortFilterProxyModel {
  pub fn setDynamicSortFilter<RetType, T: QSortFilterProxyModel_setDynamicSortFilter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDynamicSortFilter(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setDynamicSortFilter<RetType> {
  fn setDynamicSortFilter(self , rsthis: &mut QSortFilterProxyModel) -> RetType;
}

  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
impl<'a> /*trait*/ QSortFilterProxyModel_setDynamicSortFilter<()> for (i8) {
  fn setDynamicSortFilter(self , rsthis: &mut QSortFilterProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setDynamicSortFilterEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN21QSortFilterProxyModel20setDynamicSortFilterEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

