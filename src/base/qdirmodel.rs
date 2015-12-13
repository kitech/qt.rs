// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfileiconprovider::QFileIconProvider;
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;
use super::qstringlist::QStringList;
use super::qstring::QString;
use super::qobject::QObject;
use super::qicon::QIcon;
use super::qfileinfo::QFileInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QFileIconProvider * QDirModel::iconProvider();
  fn _ZNK9QDirModel12iconProviderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
  fn _ZNK9QDirModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
  fn _ZNK9QDirModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QStringList QDirModel::nameFilters();
  fn _ZNK9QDirModel11nameFiltersEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::isReadOnly();
  fn _ZNK9QDirModel10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QDirModel::columnCount(const QModelIndex & parent);
  fn _ZNK9QDirModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStringList QDirModel::mimeTypes();
  fn _ZNK9QDirModel9mimeTypesEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QDirModel::FreeQDirModel();
  fn _ZN9QDirModelD0Ev(qthis: *mut c_void) ;
  // proto:  bool QDirModel::remove(const QModelIndex & index);
  fn _ZN9QDirModel6removeERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QDirModel::fileName(const QModelIndex & index);
  fn _ZNK9QDirModel8fileNameERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QDirModel::metaObject();
  fn _ZNK9QDirModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QDirModel::resolveSymlinks();
  fn _ZNK9QDirModel15resolveSymlinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDirModel::refresh(const QModelIndex & parent);
  fn _ZN9QDirModel7refreshERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDirModel::setNameFilters(const QStringList & filters);
  fn _ZN9QDirModel14setNameFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
  fn _ZN9QDirModel15setIconProviderEP17QFileIconProvider(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK9QDirModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QDirModel::NewQDirModel(QObject * parent);
  fn _ZN9QDirModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QDirModel::index(const QString & path, int column);
  fn _ZNK9QDirModel5indexERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  void QDirModel::setLazyChildCount(bool enable);
  fn _ZN9QDirModel17setLazyChildCountEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
  fn _ZNK9QDirModel8fileIconERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::hasChildren(const QModelIndex & index);
  fn _ZNK9QDirModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QDirModel::isDir(const QModelIndex & index);
  fn _ZNK9QDirModel5isDirERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
  fn _ZN9QDirModel5mkdirERK11QModelIndexRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::rmdir(const QModelIndex & index);
  fn _ZN9QDirModel5rmdirERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QDirModel::filePath(const QModelIndex & index);
  fn _ZNK9QDirModel8filePathERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QDirModel::rowCount(const QModelIndex & parent);
  fn _ZNK9QDirModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QDirModel::setReadOnly(bool enable);
  fn _ZN9QDirModel11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QDirModel::NewQDirModel(const QDirModel & );
  fn _ZN9QDirModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDirModel::setResolveSymlinks(bool enable);
  fn _ZN9QDirModel18setResolveSymlinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QDirModel::lazyChildCount();
  fn _ZNK9QDirModel14lazyChildCountEv(qthis: *mut c_void) -> int8_t;
  // proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
  fn _ZNK9QDirModel8fileInfoERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDirModel)=1
pub struct QDirModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDirModel {
  pub fn iconProvider<T: QDirModel_iconProvider>(&mut self, value: T) -> QFileIconProvider {
    return value.iconProvider(self);
    // return 1;
  }
}

pub trait QDirModel_iconProvider {
  fn iconProvider(self, rsthis: &mut QDirModel) -> QFileIconProvider;
}

// proto:  QFileIconProvider * QDirModel::iconProvider();
impl<'a> /*trait*/ QDirModel_iconProvider for () {
  fn iconProvider(self, rsthis: &mut QDirModel) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel12iconProviderEv()};
    let mut ret = unsafe {_ZNK9QDirModel12iconProviderEv(rsthis.qclsinst)};
    let mut ret1 = QFileIconProvider{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn parent<T: QDirModel_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QDirModel_parent {
  fn parent(self, rsthis: &mut QDirModel) -> QModelIndex;
}

// proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QDirModel_parent for (&'a  QModelIndex) {
  fn parent(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn data<T: QDirModel_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QDirModel_data {
  fn data(self, rsthis: &mut QDirModel) -> QVariant;
}

// proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QDirModel_data for (&'a  QModelIndex, i32) {
  fn data(self, rsthis: &mut QDirModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QDirModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn nameFilters<T: QDirModel_nameFilters>(&mut self, value: T) -> QStringList {
    return value.nameFilters(self);
    // return 1;
  }
}

pub trait QDirModel_nameFilters {
  fn nameFilters(self, rsthis: &mut QDirModel) -> QStringList;
}

// proto:  QStringList QDirModel::nameFilters();
impl<'a> /*trait*/ QDirModel_nameFilters for () {
  fn nameFilters(self, rsthis: &mut QDirModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11nameFiltersEv()};
    let mut ret = unsafe {_ZNK9QDirModel11nameFiltersEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn isReadOnly<T: QDirModel_isReadOnly>(&mut self, value: T) -> i8 {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QDirModel_isReadOnly {
  fn isReadOnly(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::isReadOnly();
impl<'a> /*trait*/ QDirModel_isReadOnly for () {
  fn isReadOnly(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QDirModel10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn columnCount<T: QDirModel_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QDirModel_columnCount {
  fn columnCount(self, rsthis: &mut QDirModel) -> i32;
}

// proto:  int QDirModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, rsthis: &mut QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn mimeTypes<T: QDirModel_mimeTypes>(&mut self, value: T) -> QStringList {
    return value.mimeTypes(self);
    // return 1;
  }
}

pub trait QDirModel_mimeTypes {
  fn mimeTypes(self, rsthis: &mut QDirModel) -> QStringList;
}

// proto:  QStringList QDirModel::mimeTypes();
impl<'a> /*trait*/ QDirModel_mimeTypes for () {
  fn mimeTypes(self, rsthis: &mut QDirModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel9mimeTypesEv()};
    let mut ret = unsafe {_ZNK9QDirModel9mimeTypesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn FreeQDirModel<T: QDirModel_FreeQDirModel>(&mut self, value: T)  {
     value.FreeQDirModel(self);
    // return 1;
  }
}

pub trait QDirModel_FreeQDirModel {
  fn FreeQDirModel(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::FreeQDirModel();
impl<'a> /*trait*/ QDirModel_FreeQDirModel for () {
  fn FreeQDirModel(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelD0Ev()};
     unsafe {_ZN9QDirModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn remove<T: QDirModel_remove>(&mut self, value: T) -> i8 {
    return value.remove(self);
    // return 1;
  }
}

pub trait QDirModel_remove {
  fn remove(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::remove(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_remove for (&'a  QModelIndex) {
  fn remove(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel6removeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel6removeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn fileName<T: QDirModel_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QDirModel_fileName {
  fn fileName(self, rsthis: &mut QDirModel) -> QString;
}

// proto:  QString QDirModel::fileName(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileName for (&'a  QModelIndex) {
  fn fileName(self, rsthis: &mut QDirModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileNameERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileNameERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn metaObject<T: QDirModel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QDirModel_metaObject {
  fn metaObject(self, rsthis: &mut QDirModel) ;
}

// proto:  const QMetaObject * QDirModel::metaObject();
impl<'a> /*trait*/ QDirModel_metaObject for () {
  fn metaObject(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel10metaObjectEv()};
     unsafe {_ZNK9QDirModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn resolveSymlinks<T: QDirModel_resolveSymlinks>(&mut self, value: T) -> i8 {
    return value.resolveSymlinks(self);
    // return 1;
  }
}

pub trait QDirModel_resolveSymlinks {
  fn resolveSymlinks(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::resolveSymlinks();
impl<'a> /*trait*/ QDirModel_resolveSymlinks for () {
  fn resolveSymlinks(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel15resolveSymlinksEv()};
    let mut ret = unsafe {_ZNK9QDirModel15resolveSymlinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn refresh<T: QDirModel_refresh>(&mut self, value: T)  {
     value.refresh(self);
    // return 1;
  }
}

pub trait QDirModel_refresh {
  fn refresh(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::refresh(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_refresh for (&'a  QModelIndex) {
  fn refresh(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel7refreshERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel7refreshERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setNameFilters<T: QDirModel_setNameFilters>(&mut self, value: T)  {
     value.setNameFilters(self);
    // return 1;
  }
}

pub trait QDirModel_setNameFilters {
  fn setNameFilters(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QDirModel_setNameFilters for (&'a  QStringList) {
  fn setNameFilters(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setIconProvider<T: QDirModel_setIconProvider>(&mut self, value: T)  {
     value.setIconProvider(self);
    // return 1;
  }
}

pub trait QDirModel_setIconProvider {
  fn setIconProvider(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QDirModel_setIconProvider for (&'a mut QFileIconProvider) {
  fn setIconProvider(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel15setIconProviderEP17QFileIconProvider(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn index<T: QDirModel_index>(&mut self, value: T) -> QModelIndex {
    return value.index(self);
    // return 1;
  }
}

pub trait QDirModel_index {
  fn index(self, rsthis: &mut QDirModel) -> QModelIndex;
}

// proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn NewQDirModel<T: QDirModel_NewQDirModel>(value: T) -> QDirModel {
    let rsthis = value.NewQDirModel();
    return rsthis;
    // return 1;
  }
}

pub trait QDirModel_NewQDirModel {
  fn NewQDirModel(self) -> QDirModel;
}

// proto: void QDirModel::NewQDirModel(QObject * parent);
impl<'a> /*trait*/ QDirModel_NewQDirModel for (&'a mut QObject) {
  fn NewQDirModel(self) -> QDirModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDirModelC1EP7QObject(qthis, arg0)};
    let rsthis = QDirModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QModelIndex QDirModel::index(const QString & path, int column);
impl<'a> /*trait*/ QDirModel_index for (&'a  QString, i32) {
  fn index(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5indexERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QDirModel5indexERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setData<T: QDirModel_setData>(&mut self, value: T) -> i8 {
    return value.setData(self);
    // return 1;
  }
}

pub trait QDirModel_setData {
  fn setData(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QDirModel_setData for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setLazyChildCount<T: QDirModel_setLazyChildCount>(&mut self, value: T)  {
     value.setLazyChildCount(self);
    // return 1;
  }
}

pub trait QDirModel_setLazyChildCount {
  fn setLazyChildCount(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::setLazyChildCount(bool enable);
impl<'a> /*trait*/ QDirModel_setLazyChildCount for (i8) {
  fn setLazyChildCount(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel17setLazyChildCountEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QDirModel17setLazyChildCountEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn fileIcon<T: QDirModel_fileIcon>(&mut self, value: T) -> QIcon {
    return value.fileIcon(self);
    // return 1;
  }
}

pub trait QDirModel_fileIcon {
  fn fileIcon(self, rsthis: &mut QDirModel) -> QIcon;
}

// proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileIcon for (&'a  QModelIndex) {
  fn fileIcon(self, rsthis: &mut QDirModel) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileIconERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileIconERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn hasChildren<T: QDirModel_hasChildren>(&mut self, value: T) -> i8 {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QDirModel_hasChildren {
  fn hasChildren(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::hasChildren(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_hasChildren for (&'a  QModelIndex) {
  fn hasChildren(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn isDir<T: QDirModel_isDir>(&mut self, value: T) -> i8 {
    return value.isDir(self);
    // return 1;
  }
}

pub trait QDirModel_isDir {
  fn isDir(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::isDir(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_isDir for (&'a  QModelIndex) {
  fn isDir(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5isDirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel5isDirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn mkdir<T: QDirModel_mkdir>(&mut self, value: T) -> QModelIndex {
    return value.mkdir(self);
    // return 1;
  }
}

pub trait QDirModel_mkdir {
  fn mkdir(self, rsthis: &mut QDirModel) -> QModelIndex;
}

// proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
impl<'a> /*trait*/ QDirModel_mkdir for (&'a  QModelIndex, &'a  QString) {
  fn mkdir(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel5mkdirERK11QModelIndexRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel5mkdirERK11QModelIndexRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn rmdir<T: QDirModel_rmdir>(&mut self, value: T) -> i8 {
    return value.rmdir(self);
    // return 1;
  }
}

pub trait QDirModel_rmdir {
  fn rmdir(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::rmdir(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_rmdir for (&'a  QModelIndex) {
  fn rmdir(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel5rmdirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel5rmdirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn filePath<T: QDirModel_filePath>(&mut self, value: T) -> QString {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QDirModel_filePath {
  fn filePath(self, rsthis: &mut QDirModel) -> QString;
}

// proto:  QString QDirModel::filePath(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_filePath for (&'a  QModelIndex) {
  fn filePath(self, rsthis: &mut QDirModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8filePathERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8filePathERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn rowCount<T: QDirModel_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QDirModel_rowCount {
  fn rowCount(self, rsthis: &mut QDirModel) -> i32;
}

// proto:  int QDirModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setReadOnly<T: QDirModel_setReadOnly>(&mut self, value: T)  {
     value.setReadOnly(self);
    // return 1;
  }
}

pub trait QDirModel_setReadOnly {
  fn setReadOnly(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::setReadOnly(bool enable);
impl<'a> /*trait*/ QDirModel_setReadOnly for (i8) {
  fn setReadOnly(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QDirModel11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QDirModel::NewQDirModel(const QDirModel & );
impl<'a> /*trait*/ QDirModel_NewQDirModel for (&'a  QDirModel) {
  fn NewQDirModel(self) -> QDirModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDirModelC1ERKS_(qthis, arg0)};
    let rsthis = QDirModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setResolveSymlinks<T: QDirModel_setResolveSymlinks>(&mut self, value: T)  {
     value.setResolveSymlinks(self);
    // return 1;
  }
}

pub trait QDirModel_setResolveSymlinks {
  fn setResolveSymlinks(self, rsthis: &mut QDirModel) ;
}

// proto:  void QDirModel::setResolveSymlinks(bool enable);
impl<'a> /*trait*/ QDirModel_setResolveSymlinks for (i8) {
  fn setResolveSymlinks(self, rsthis: &mut QDirModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel18setResolveSymlinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QDirModel18setResolveSymlinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn lazyChildCount<T: QDirModel_lazyChildCount>(&mut self, value: T) -> i8 {
    return value.lazyChildCount(self);
    // return 1;
  }
}

pub trait QDirModel_lazyChildCount {
  fn lazyChildCount(self, rsthis: &mut QDirModel) -> i8;
}

// proto:  bool QDirModel::lazyChildCount();
impl<'a> /*trait*/ QDirModel_lazyChildCount for () {
  fn lazyChildCount(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel14lazyChildCountEv()};
    let mut ret = unsafe {_ZNK9QDirModel14lazyChildCountEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn fileInfo<T: QDirModel_fileInfo>(&mut self, value: T) -> QFileInfo {
    return value.fileInfo(self);
    // return 1;
  }
}

pub trait QDirModel_fileInfo {
  fn fileInfo(self, rsthis: &mut QDirModel) -> QFileInfo;
}

// proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileInfo for (&'a  QModelIndex) {
  fn fileInfo(self, rsthis: &mut QDirModel) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileInfoERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileInfoERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QFileInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

