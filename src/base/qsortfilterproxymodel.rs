// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qmodelindex::QModelIndex;
use super::qsize::QSize;
use super::qvariant::QVariant;
use super::qitemselection::QItemSelection;
use super::qregexp::QRegExp;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
  fn _ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
  fn _ZNK21QSortFilterProxyModel4spanERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn _ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
  fn _ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
  fn _ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  void QSortFilterProxyModel::setSortRole(int role);
  fn _ZN21QSortFilterProxyModel11setSortRoleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
  fn _ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::invalidate();
  fn _ZN21QSortFilterProxyModel10invalidateEv(qthis: *mut c_void) ;
  // proto:  int QSortFilterProxyModel::sortColumn();
  fn _ZNK21QSortFilterProxyModel10sortColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  int QSortFilterProxyModel::filterKeyColumn();
  fn _ZNK21QSortFilterProxyModel15filterKeyColumnEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QSortFilterProxyModel::isSortLocaleAware();
  fn _ZNK21QSortFilterProxyModel17isSortLocaleAwareEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
  fn _ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
  fn _ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QStringList QSortFilterProxyModel::mimeTypes();
  fn _ZNK21QSortFilterProxyModel9mimeTypesEv(qthis: *mut c_void) ;
  // proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
  fn _ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QSortFilterProxyModel::filterRole();
  fn _ZNK21QSortFilterProxyModel10filterRoleEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::clear();
  fn _ZN21QSortFilterProxyModel5clearEv(qthis: *mut c_void) ;
  // proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
  fn _ZN21QSortFilterProxyModel18setFilterKeyColumnEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
  fn _ZNK21QSortFilterProxyModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QSortFilterProxyModel::sortRole();
  fn _ZNK21QSortFilterProxyModel8sortRoleEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
  fn _ZN21QSortFilterProxyModel18setSortLocaleAwareEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn _ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::NewQSortFilterProxyModel(const QSortFilterProxyModel & );
  fn _ZN21QSortFilterProxyModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  void QSortFilterProxyModel::FreeQSortFilterProxyModel();
  fn _ZN21QSortFilterProxyModelD0Ev(qthis: *mut c_void) ;
  // proto:  bool QSortFilterProxyModel::dynamicSortFilter();
  fn _ZNK21QSortFilterProxyModel17dynamicSortFilterEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
  fn _ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
  fn _ZNK21QSortFilterProxyModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegExp QSortFilterProxyModel::filterRegExp();
  fn _ZNK21QSortFilterProxyModel12filterRegExpEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setFilterRole(int role);
  fn _ZN21QSortFilterProxyModel13setFilterRoleEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSortFilterProxyModel::NewQSortFilterProxyModel(QObject * parent);
  fn _ZN21QSortFilterProxyModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
  fn _ZN21QSortFilterProxyModel20setDynamicSortFilterEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QSortFilterProxyModel)=1
pub struct QSortFilterProxyModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRegExp<T: QSortFilterProxyModel_setFilterRegExp>(&mut self, value: T)  {
     value.setFilterRegExp(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRegExp {
  fn setFilterRegExp(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp for (&'a  QString) {
  fn setFilterRegExp(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn rowCount<T: QSortFilterProxyModel_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_rowCount {
  fn rowCount(self, rsthis: &mut QSortFilterProxyModel) -> i32;
}

// proto:  int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn sibling<T: QSortFilterProxyModel_sibling>(&mut self, value: T) -> QModelIndex {
    return value.sibling(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sibling {
  fn sibling(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QSortFilterProxyModel_sibling for (i32, i32, &'a  QModelIndex) {
  fn sibling(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn span<T: QSortFilterProxyModel_span>(&mut self, value: T) -> QSize {
    return value.span(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_span {
  fn span(self, rsthis: &mut QSortFilterProxyModel) -> QSize;
}

// proto:  QSize QSortFilterProxyModel::span(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_span for (&'a  QModelIndex) {
  fn span(self, rsthis: &mut QSortFilterProxyModel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4spanERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel4spanERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapFromSource<T: QSortFilterProxyModel_mapFromSource>(&mut self, value: T) -> QModelIndex {
    return value.mapFromSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapFromSource {
  fn mapFromSource(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapFromSource for (&'a  QModelIndex) {
  fn mapFromSource(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterWildcard<T: QSortFilterProxyModel_setFilterWildcard>(&mut self, value: T)  {
     value.setFilterWildcard(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterWildcard {
  fn setFilterWildcard(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterWildcard for (&'a  QString) {
  fn setFilterWildcard(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn hasChildren<T: QSortFilterProxyModel_hasChildren>(&mut self, value: T) -> i8 {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_hasChildren {
  fn hasChildren(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_hasChildren for (&'a  QModelIndex) {
  fn hasChildren(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterFixedString<T: QSortFilterProxyModel_setFilterFixedString>(&mut self, value: T)  {
     value.setFilterFixedString(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterFixedString {
  fn setFilterFixedString(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterFixedString for (&'a  QString) {
  fn setFilterFixedString(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setData<T: QSortFilterProxyModel_setData>(&mut self, value: T) -> i8 {
    return value.setData(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setData {
  fn setData(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setData for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortRole<T: QSortFilterProxyModel_setSortRole>(&mut self, value: T)  {
     value.setSortRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSortRole {
  fn setSortRole(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setSortRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortRole for (i32) {
  fn setSortRole(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel11setSortRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QSortFilterProxyModel11setSortRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn data<T: QSortFilterProxyModel_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_data {
  fn data(self, rsthis: &mut QSortFilterProxyModel) -> QVariant;
}

// proto:  QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_data for (&'a  QModelIndex, i32) {
  fn data(self, rsthis: &mut QSortFilterProxyModel) -> QVariant {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn invalidate<T: QSortFilterProxyModel_invalidate>(&mut self, value: T)  {
     value.invalidate(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_invalidate {
  fn invalidate(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::invalidate();
impl<'a> /*trait*/ QSortFilterProxyModel_invalidate for () {
  fn invalidate(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10invalidateEv()};
     unsafe {_ZN21QSortFilterProxyModel10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn sortColumn<T: QSortFilterProxyModel_sortColumn>(&mut self, value: T) -> i32 {
    return value.sortColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sortColumn {
  fn sortColumn(self, rsthis: &mut QSortFilterProxyModel) -> i32;
}

// proto:  int QSortFilterProxyModel::sortColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_sortColumn for () {
  fn sortColumn(self, rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10sortColumnEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel10sortColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn insertRows<T: QSortFilterProxyModel_insertRows>(&mut self, value: T) -> i8 {
    return value.insertRows(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_insertRows {
  fn insertRows(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertRows for (i32, i32, &'a  QModelIndex) {
  fn insertRows(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn filterKeyColumn<T: QSortFilterProxyModel_filterKeyColumn>(&mut self, value: T) -> i32 {
    return value.filterKeyColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterKeyColumn {
  fn filterKeyColumn(self, rsthis: &mut QSortFilterProxyModel) -> i32;
}

// proto:  int QSortFilterProxyModel::filterKeyColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_filterKeyColumn for () {
  fn filterKeyColumn(self, rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel15filterKeyColumnEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel15filterKeyColumnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn canFetchMore<T: QSortFilterProxyModel_canFetchMore>(&mut self, value: T) -> i8 {
    return value.canFetchMore(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_canFetchMore {
  fn canFetchMore(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_canFetchMore for (&'a  QModelIndex) {
  fn canFetchMore(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn isSortLocaleAware<T: QSortFilterProxyModel_isSortLocaleAware>(&mut self, value: T) -> i8 {
    return value.isSortLocaleAware(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_isSortLocaleAware {
  fn isSortLocaleAware(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::isSortLocaleAware();
impl<'a> /*trait*/ QSortFilterProxyModel_isSortLocaleAware for () {
  fn isSortLocaleAware(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn fetchMore<T: QSortFilterProxyModel_fetchMore>(&mut self, value: T)  {
     value.fetchMore(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_fetchMore {
  fn fetchMore(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_fetchMore for (&'a  QModelIndex) {
  fn fetchMore(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionFromSource<T: QSortFilterProxyModel_mapSelectionFromSource>(&mut self, value: T) -> QItemSelection {
    return value.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionFromSource {
  fn mapSelectionFromSource(self, rsthis: &mut QSortFilterProxyModel) -> QItemSelection;
}

// proto:  QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionFromSource for (&'a  QItemSelection) {
  fn mapSelectionFromSource(self, rsthis: &mut QSortFilterProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionToSource<T: QSortFilterProxyModel_mapSelectionToSource>(&mut self, value: T) -> QItemSelection {
    return value.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionToSource {
  fn mapSelectionToSource(self, rsthis: &mut QSortFilterProxyModel) -> QItemSelection;
}

// proto:  QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionToSource for (&'a  QItemSelection) {
  fn mapSelectionToSource(self, rsthis: &mut QSortFilterProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mimeTypes<T: QSortFilterProxyModel_mimeTypes>(&mut self, value: T)  {
     value.mimeTypes(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mimeTypes {
  fn mimeTypes(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  QStringList QSortFilterProxyModel::mimeTypes();
impl<'a> /*trait*/ QSortFilterProxyModel_mimeTypes for () {
  fn mimeTypes(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel9mimeTypesEv()};
     unsafe {_ZNK21QSortFilterProxyModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn buddy<T: QSortFilterProxyModel_buddy>(&mut self, value: T) -> QModelIndex {
    return value.buddy(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_buddy {
  fn buddy(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_buddy for (&'a  QModelIndex) {
  fn buddy(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRole<T: QSortFilterProxyModel_filterRole>(&mut self, value: T) -> i32 {
    return value.filterRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterRole {
  fn filterRole(self, rsthis: &mut QSortFilterProxyModel) -> i32;
}

// proto:  int QSortFilterProxyModel::filterRole();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRole for () {
  fn filterRole(self, rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10filterRoleEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel10filterRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn clear<T: QSortFilterProxyModel_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_clear {
  fn clear(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::clear();
impl<'a> /*trait*/ QSortFilterProxyModel_clear for () {
  fn clear(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel5clearEv()};
     unsafe {_ZN21QSortFilterProxyModel5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterKeyColumn<T: QSortFilterProxyModel_setFilterKeyColumn>(&mut self, value: T)  {
     value.setFilterKeyColumn(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterKeyColumn {
  fn setFilterKeyColumn(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setFilterKeyColumn(int column);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterKeyColumn for (i32) {
  fn setFilterKeyColumn(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setFilterKeyColumnEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QSortFilterProxyModel18setFilterKeyColumnEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn metaObject<T: QSortFilterProxyModel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_metaObject {
  fn metaObject(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  const QMetaObject * QSortFilterProxyModel::metaObject();
impl<'a> /*trait*/ QSortFilterProxyModel_metaObject for () {
  fn metaObject(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10metaObjectEv()};
     unsafe {_ZNK21QSortFilterProxyModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn sortRole<T: QSortFilterProxyModel_sortRole>(&mut self, value: T) -> i32 {
    return value.sortRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_sortRole {
  fn sortRole(self, rsthis: &mut QSortFilterProxyModel) -> i32;
}

// proto:  int QSortFilterProxyModel::sortRole();
impl<'a> /*trait*/ QSortFilterProxyModel_sortRole for () {
  fn sortRole(self, rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8sortRoleEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel8sortRoleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortLocaleAware<T: QSortFilterProxyModel_setSortLocaleAware>(&mut self, value: T)  {
     value.setSortLocaleAware(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setSortLocaleAware {
  fn setSortLocaleAware(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setSortLocaleAware(bool on);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortLocaleAware for (i8) {
  fn setSortLocaleAware(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setSortLocaleAwareEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN21QSortFilterProxyModel18setSortLocaleAwareEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapToSource<T: QSortFilterProxyModel_mapToSource>(&mut self, value: T) -> QModelIndex {
    return value.mapToSource(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_mapToSource {
  fn mapToSource(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapToSource for (&'a  QModelIndex) {
  fn mapToSource(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

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

// proto: void QSortFilterProxyModel::NewQSortFilterProxyModel(const QSortFilterProxyModel & );
impl<'a> /*trait*/ QSortFilterProxyModel_NewQSortFilterProxyModel for (&'a  QSortFilterProxyModel) {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn removeColumns<T: QSortFilterProxyModel_removeColumns>(&mut self, value: T) -> i8 {
    return value.removeColumns(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_removeColumns {
  fn removeColumns(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeColumns for (i32, i32, &'a  QModelIndex) {
  fn removeColumns(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn FreeQSortFilterProxyModel<T: QSortFilterProxyModel_FreeQSortFilterProxyModel>(&mut self, value: T)  {
     value.FreeQSortFilterProxyModel(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_FreeQSortFilterProxyModel {
  fn FreeQSortFilterProxyModel(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::FreeQSortFilterProxyModel();
impl<'a> /*trait*/ QSortFilterProxyModel_FreeQSortFilterProxyModel for () {
  fn FreeQSortFilterProxyModel(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelD0Ev()};
     unsafe {_ZN21QSortFilterProxyModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn dynamicSortFilter<T: QSortFilterProxyModel_dynamicSortFilter>(&mut self, value: T) -> i8 {
    return value.dynamicSortFilter(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_dynamicSortFilter {
  fn dynamicSortFilter(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::dynamicSortFilter();
impl<'a> /*trait*/ QSortFilterProxyModel_dynamicSortFilter for () {
  fn dynamicSortFilter(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17dynamicSortFilterEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel17dynamicSortFilterEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn insertColumns<T: QSortFilterProxyModel_insertColumns>(&mut self, value: T) -> i8 {
    return value.insertColumns(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_insertColumns {
  fn insertColumns(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertColumns for (i32, i32, &'a  QModelIndex) {
  fn insertColumns(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn columnCount<T: QSortFilterProxyModel_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_columnCount {
  fn columnCount(self, rsthis: &mut QSortFilterProxyModel) -> i32;
}

// proto:  int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, rsthis: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp for (&'a  QRegExp) {
  fn setFilterRegExp(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn parent<T: QSortFilterProxyModel_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_parent {
  fn parent(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QSortFilterProxyModel_parent for (&'a  QModelIndex) {
  fn parent(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRegExp<T: QSortFilterProxyModel_filterRegExp>(&mut self, value: T) -> QRegExp {
    return value.filterRegExp(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_filterRegExp {
  fn filterRegExp(self, rsthis: &mut QSortFilterProxyModel) -> QRegExp;
}

// proto:  QRegExp QSortFilterProxyModel::filterRegExp();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRegExp for () {
  fn filterRegExp(self, rsthis: &mut QSortFilterProxyModel) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12filterRegExpEv()};
    let mut ret = unsafe {_ZNK21QSortFilterProxyModel12filterRegExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRole<T: QSortFilterProxyModel_setFilterRole>(&mut self, value: T)  {
     value.setFilterRole(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRole {
  fn setFilterRole(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setFilterRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRole for (i32) {
  fn setFilterRole(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13setFilterRoleEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QSortFilterProxyModel13setFilterRoleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QSortFilterProxyModel::NewQSortFilterProxyModel(QObject * parent);
impl<'a> /*trait*/ QSortFilterProxyModel_NewQSortFilterProxyModel for (&'a mut QObject) {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn removeRows<T: QSortFilterProxyModel_removeRows>(&mut self, value: T) -> i8 {
    return value.removeRows(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_removeRows {
  fn removeRows(self, rsthis: &mut QSortFilterProxyModel) -> i8;
}

// proto:  bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeRows for (i32, i32, &'a  QModelIndex) {
  fn removeRows(self, rsthis: &mut QSortFilterProxyModel) -> i8 {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn index<T: QSortFilterProxyModel_index>(&mut self, value: T) -> QModelIndex {
    return value.index(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_index {
  fn index(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, rsthis: &mut QSortFilterProxyModel) -> QModelIndex {
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

impl /*struct*/ QSortFilterProxyModel {
  pub fn setDynamicSortFilter<T: QSortFilterProxyModel_setDynamicSortFilter>(&mut self, value: T)  {
     value.setDynamicSortFilter(self);
    // return 1;
  }
}

pub trait QSortFilterProxyModel_setDynamicSortFilter {
  fn setDynamicSortFilter(self, rsthis: &mut QSortFilterProxyModel) ;
}

// proto:  void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
impl<'a> /*trait*/ QSortFilterProxyModel_setDynamicSortFilter for (i8) {
  fn setDynamicSortFilter(self, rsthis: &mut QSortFilterProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setDynamicSortFilterEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN21QSortFilterProxyModel20setDynamicSortFilterEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

