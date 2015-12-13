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
use super::qvariant::QVariant;
use super::qitemselection::QItemSelection;
use super::qregexp::QRegExp;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel4spanERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(arg0: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(arg0: *const c_void, arg1: *const c_void, arg2: c_int) -> i32;
  fn _ZN21QSortFilterProxyModel11setSortRoleEi(arg0: c_int) -> i32;
  fn _ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(arg0: *const c_void, arg1: c_int) -> i32;
  fn _ZN21QSortFilterProxyModel10invalidateEv() -> i32;
  fn _ZNK21QSortFilterProxyModel10sortColumnEv() -> i32;
  fn _ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel15filterKeyColumnEv() -> i32;
  fn _ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel17isSortLocaleAwareEv() -> i32;
  fn _ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel9mimeTypesEv() -> i32;
  fn _ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel10filterRoleEv() -> i32;
  fn _ZN21QSortFilterProxyModel5clearEv() -> i32;
  fn _ZN21QSortFilterProxyModel18setFilterKeyColumnEi(arg0: c_int) -> i32;
  fn _ZNK21QSortFilterProxyModel10metaObjectEv() -> i32;
  fn _ZNK21QSortFilterProxyModel8sortRoleEv() -> i32;
  fn _ZN21QSortFilterProxyModel18setSortLocaleAwareEb(arg0: int8_t) -> i32;
  fn _ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModelC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModelD0Ev() -> i32;
  fn _ZNK21QSortFilterProxyModel17dynamicSortFilterEv() -> i32;
  fn _ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel6parentERK11QModelIndex(arg0: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel12filterRegExpEv() -> i32;
  fn _ZN21QSortFilterProxyModel13setFilterRoleEi(arg0: c_int) -> i32;
  fn _ZN21QSortFilterProxyModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  fn _ZN21QSortFilterProxyModel20setDynamicSortFilterEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QSortFilterProxyModel)=1
pub struct QSortFilterProxyModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRegExp<T: QSortFilterProxyModel_setFilterRegExp>(&mut self, value: T) -> i32 {
    value.setFilterRegExp(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRegExp {
  fn setFilterRegExp(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setFilterRegExp(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp for (&'a  QString) {
  fn setFilterRegExp(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel15setFilterRegExpERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn rowCount<T: QSortFilterProxyModel_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_rowCount {
  fn rowCount(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: int QSortFilterProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel8rowCountERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn sibling<T: QSortFilterProxyModel_sibling>(&mut self, value: T) -> i32 {
    value.sibling(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_sibling {
  fn sibling(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QSortFilterProxyModel_sibling for (i32, i32, &'a  QModelIndex) {
  fn sibling(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel7siblingEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn span<T: QSortFilterProxyModel_span>(&mut self, value: T) -> i32 {
    value.span(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_span {
  fn span(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QSize QSortFilterProxyModel::span(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_span for (&'a  QModelIndex) {
  fn span(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4spanERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel4spanERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapFromSource<T: QSortFilterProxyModel_mapFromSource>(&mut self, value: T) -> i32 {
    value.mapFromSource(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_mapFromSource {
  fn mapFromSource(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapFromSource for (&'a  QModelIndex) {
  fn mapFromSource(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel13mapFromSourceERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterWildcard<T: QSortFilterProxyModel_setFilterWildcard>(&mut self, value: T) -> i32 {
    value.setFilterWildcard(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterWildcard {
  fn setFilterWildcard(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setFilterWildcard(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterWildcard for (&'a  QString) {
  fn setFilterWildcard(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel17setFilterWildcardERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn hasChildren<T: QSortFilterProxyModel_hasChildren>(&mut self, value: T) -> i32 {
    value.hasChildren(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_hasChildren {
  fn hasChildren(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_hasChildren for (&'a  QModelIndex) {
  fn hasChildren(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel11hasChildrenERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterFixedString<T: QSortFilterProxyModel_setFilterFixedString>(&mut self, value: T) -> i32 {
    value.setFilterFixedString(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterFixedString {
  fn setFilterFixedString(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setFilterFixedString(const QString & pattern);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterFixedString for (&'a  QString) {
  fn setFilterFixedString(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel20setFilterFixedStringERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setData<T: QSortFilterProxyModel_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setData {
  fn setData(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setData for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN21QSortFilterProxyModel7setDataERK11QModelIndexRK8QVarianti(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortRole<T: QSortFilterProxyModel_setSortRole>(&mut self, value: T) -> i32 {
    value.setSortRole(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setSortRole {
  fn setSortRole(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setSortRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortRole for (i32) {
  fn setSortRole(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel11setSortRoleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QSortFilterProxyModel11setSortRoleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn data<T: QSortFilterProxyModel_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_data {
  fn data(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QVariant QSortFilterProxyModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QSortFilterProxyModel_data for (&'a  QModelIndex, i32) {
  fn data(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK21QSortFilterProxyModel4dataERK11QModelIndexi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn invalidate<T: QSortFilterProxyModel_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_invalidate {
  fn invalidate(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::invalidate();
impl<'a> /*trait*/ QSortFilterProxyModel_invalidate for () {
  fn invalidate(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10invalidateEv()};
    unsafe {_ZN21QSortFilterProxyModel10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn sortColumn<T: QSortFilterProxyModel_sortColumn>(&mut self, value: T) -> i32 {
    value.sortColumn(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_sortColumn {
  fn sortColumn(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: int QSortFilterProxyModel::sortColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_sortColumn for () {
  fn sortColumn(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10sortColumnEv()};
    unsafe {_ZNK21QSortFilterProxyModel10sortColumnEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn insertRows<T: QSortFilterProxyModel_insertRows>(&mut self, value: T) -> i32 {
    value.insertRows(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_insertRows {
  fn insertRows(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertRows for (i32, i32, &'a  QModelIndex) {
  fn insertRows(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel10insertRowsEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn filterKeyColumn<T: QSortFilterProxyModel_filterKeyColumn>(&mut self, value: T) -> i32 {
    value.filterKeyColumn(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_filterKeyColumn {
  fn filterKeyColumn(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: int QSortFilterProxyModel::filterKeyColumn();
impl<'a> /*trait*/ QSortFilterProxyModel_filterKeyColumn for () {
  fn filterKeyColumn(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel15filterKeyColumnEv()};
    unsafe {_ZNK21QSortFilterProxyModel15filterKeyColumnEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn canFetchMore<T: QSortFilterProxyModel_canFetchMore>(&mut self, value: T) -> i32 {
    value.canFetchMore(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_canFetchMore {
  fn canFetchMore(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_canFetchMore for (&'a  QModelIndex) {
  fn canFetchMore(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel12canFetchMoreERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn isSortLocaleAware<T: QSortFilterProxyModel_isSortLocaleAware>(&mut self, value: T) -> i32 {
    value.isSortLocaleAware(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_isSortLocaleAware {
  fn isSortLocaleAware(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::isSortLocaleAware();
impl<'a> /*trait*/ QSortFilterProxyModel_isSortLocaleAware for () {
  fn isSortLocaleAware(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv()};
    unsafe {_ZNK21QSortFilterProxyModel17isSortLocaleAwareEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn fetchMore<T: QSortFilterProxyModel_fetchMore>(&mut self, value: T) -> i32 {
    value.fetchMore(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_fetchMore {
  fn fetchMore(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_fetchMore for (&'a  QModelIndex) {
  fn fetchMore(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel9fetchMoreERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionFromSource<T: QSortFilterProxyModel_mapSelectionFromSource>(&mut self, value: T) -> i32 {
    value.mapSelectionFromSource(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionFromSource {
  fn mapSelectionFromSource(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection & sourceSelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionFromSource for (&'a  QItemSelection) {
  fn mapSelectionFromSource(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel22mapSelectionFromSourceERK14QItemSelection(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapSelectionToSource<T: QSortFilterProxyModel_mapSelectionToSource>(&mut self, value: T) -> i32 {
    value.mapSelectionToSource(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_mapSelectionToSource {
  fn mapSelectionToSource(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection & proxySelection);
impl<'a> /*trait*/ QSortFilterProxyModel_mapSelectionToSource for (&'a  QItemSelection) {
  fn mapSelectionToSource(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel20mapSelectionToSourceERK14QItemSelection(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mimeTypes<T: QSortFilterProxyModel_mimeTypes>(&mut self, value: T) -> i32 {
    value.mimeTypes(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_mimeTypes {
  fn mimeTypes(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QStringList QSortFilterProxyModel::mimeTypes();
impl<'a> /*trait*/ QSortFilterProxyModel_mimeTypes for () {
  fn mimeTypes(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel9mimeTypesEv()};
    unsafe {_ZNK21QSortFilterProxyModel9mimeTypesEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn buddy<T: QSortFilterProxyModel_buddy>(&mut self, value: T) -> i32 {
    value.buddy(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_buddy {
  fn buddy(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QModelIndex QSortFilterProxyModel::buddy(const QModelIndex & index);
impl<'a> /*trait*/ QSortFilterProxyModel_buddy for (&'a  QModelIndex) {
  fn buddy(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel5buddyERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRole<T: QSortFilterProxyModel_filterRole>(&mut self, value: T) -> i32 {
    value.filterRole(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_filterRole {
  fn filterRole(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: int QSortFilterProxyModel::filterRole();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRole for () {
  fn filterRole(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10filterRoleEv()};
    unsafe {_ZNK21QSortFilterProxyModel10filterRoleEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn clear<T: QSortFilterProxyModel_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_clear {
  fn clear(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::clear();
impl<'a> /*trait*/ QSortFilterProxyModel_clear for () {
  fn clear(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel5clearEv()};
    unsafe {_ZN21QSortFilterProxyModel5clearEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterKeyColumn<T: QSortFilterProxyModel_setFilterKeyColumn>(&mut self, value: T) -> i32 {
    value.setFilterKeyColumn(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterKeyColumn {
  fn setFilterKeyColumn(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setFilterKeyColumn(int column);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterKeyColumn for (i32) {
  fn setFilterKeyColumn(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setFilterKeyColumnEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QSortFilterProxyModel18setFilterKeyColumnEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn metaObject<T: QSortFilterProxyModel_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_metaObject {
  fn metaObject(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: const QMetaObject * QSortFilterProxyModel::metaObject();
impl<'a> /*trait*/ QSortFilterProxyModel_metaObject for () {
  fn metaObject(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel10metaObjectEv()};
    unsafe {_ZNK21QSortFilterProxyModel10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn sortRole<T: QSortFilterProxyModel_sortRole>(&mut self, value: T) -> i32 {
    value.sortRole(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_sortRole {
  fn sortRole(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: int QSortFilterProxyModel::sortRole();
impl<'a> /*trait*/ QSortFilterProxyModel_sortRole for () {
  fn sortRole(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel8sortRoleEv()};
    unsafe {_ZNK21QSortFilterProxyModel8sortRoleEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setSortLocaleAware<T: QSortFilterProxyModel_setSortLocaleAware>(&mut self, value: T) -> i32 {
    value.setSortLocaleAware(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setSortLocaleAware {
  fn setSortLocaleAware(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setSortLocaleAware(bool on);
impl<'a> /*trait*/ QSortFilterProxyModel_setSortLocaleAware for (i8) {
  fn setSortLocaleAware(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel18setSortLocaleAwareEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN21QSortFilterProxyModel18setSortLocaleAwareEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn mapToSource<T: QSortFilterProxyModel_mapToSource>(&mut self, value: T) -> i32 {
    value.mapToSource(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_mapToSource {
  fn mapToSource(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QSortFilterProxyModel_mapToSource for (&'a  QModelIndex) {
  fn mapToSource(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel11mapToSourceERK11QModelIndex(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModelC1ERKS_(qthis, arg0)};
    let rsthis = QSortFilterProxyModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn removeColumns<T: QSortFilterProxyModel_removeColumns>(&mut self, value: T) -> i32 {
    value.removeColumns(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_removeColumns {
  fn removeColumns(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeColumns for (i32, i32, &'a  QModelIndex) {
  fn removeColumns(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel13removeColumnsEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn FreeQSortFilterProxyModel<T: QSortFilterProxyModel_FreeQSortFilterProxyModel>(&mut self, value: T) -> i32 {
    value.FreeQSortFilterProxyModel(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_FreeQSortFilterProxyModel {
  fn FreeQSortFilterProxyModel(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::FreeQSortFilterProxyModel();
impl<'a> /*trait*/ QSortFilterProxyModel_FreeQSortFilterProxyModel for () {
  fn FreeQSortFilterProxyModel(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModelD0Ev()};
    unsafe {_ZN21QSortFilterProxyModelD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn dynamicSortFilter<T: QSortFilterProxyModel_dynamicSortFilter>(&mut self, value: T) -> i32 {
    value.dynamicSortFilter(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_dynamicSortFilter {
  fn dynamicSortFilter(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::dynamicSortFilter();
impl<'a> /*trait*/ QSortFilterProxyModel_dynamicSortFilter for () {
  fn dynamicSortFilter(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel17dynamicSortFilterEv()};
    unsafe {_ZNK21QSortFilterProxyModel17dynamicSortFilterEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn insertColumns<T: QSortFilterProxyModel_insertColumns>(&mut self, value: T) -> i32 {
    value.insertColumns(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_insertColumns {
  fn insertColumns(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_insertColumns for (i32, i32, &'a  QModelIndex) {
  fn insertColumns(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel13insertColumnsEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn columnCount<T: QSortFilterProxyModel_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_columnCount {
  fn columnCount(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: int QSortFilterProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel11columnCountERK11QModelIndex(arg0)};
    return 1;
  }
}

// proto: void QSortFilterProxyModel::setFilterRegExp(const QRegExp & regExp);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRegExp for (&'a  QRegExp) {
  fn setFilterRegExp(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel15setFilterRegExpERK7QRegExp(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn parent<T: QSortFilterProxyModel_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_parent {
  fn parent(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QModelIndex QSortFilterProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QSortFilterProxyModel_parent for (&'a  QModelIndex) {
  fn parent(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel6parentERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn filterRegExp<T: QSortFilterProxyModel_filterRegExp>(&mut self, value: T) -> i32 {
    value.filterRegExp(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_filterRegExp {
  fn filterRegExp(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QRegExp QSortFilterProxyModel::filterRegExp();
impl<'a> /*trait*/ QSortFilterProxyModel_filterRegExp for () {
  fn filterRegExp(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel12filterRegExpEv()};
    unsafe {_ZNK21QSortFilterProxyModel12filterRegExpEv()};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setFilterRole<T: QSortFilterProxyModel_setFilterRole>(&mut self, value: T) -> i32 {
    value.setFilterRole(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setFilterRole {
  fn setFilterRole(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setFilterRole(int role);
impl<'a> /*trait*/ QSortFilterProxyModel_setFilterRole for (i32) {
  fn setFilterRole(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel13setFilterRoleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QSortFilterProxyModel13setFilterRoleEi(arg0)};
    return 1;
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
  pub fn removeRows<T: QSortFilterProxyModel_removeRows>(&mut self, value: T) -> i32 {
    value.removeRows(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_removeRows {
  fn removeRows(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_removeRows for (i32, i32, &'a  QModelIndex) {
  fn removeRows(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN21QSortFilterProxyModel10removeRowsEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn index<T: QSortFilterProxyModel_index>(&mut self, value: T) -> i32 {
    value.index(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_index {
  fn index(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QSortFilterProxyModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK21QSortFilterProxyModel5indexEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QSortFilterProxyModel {
  pub fn setDynamicSortFilter<T: QSortFilterProxyModel_setDynamicSortFilter>(&mut self, value: T) -> i32 {
    value.setDynamicSortFilter(self);
    return 1;
  }
}

pub trait QSortFilterProxyModel_setDynamicSortFilter {
  fn setDynamicSortFilter(self, this: &mut QSortFilterProxyModel) -> i32;
}

// proto: void QSortFilterProxyModel::setDynamicSortFilter(bool enable);
impl<'a> /*trait*/ QSortFilterProxyModel_setDynamicSortFilter for (i8) {
  fn setDynamicSortFilter(self, this: &mut QSortFilterProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QSortFilterProxyModel20setDynamicSortFilterEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN21QSortFilterProxyModel20setDynamicSortFilterEb(arg0)};
    return 1;
  }
}

