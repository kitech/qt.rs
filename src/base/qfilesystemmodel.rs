// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qicon::QIcon;
use super::qfileiconprovider::QFileIconProvider;
use super::qvariant::QVariant;
use super::qdir::QDir;
use super::qobject::QObject;
use super::qfileinfo::QFileInfo;
use super::qdatetime::QDateTime;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QFileSystemModel::fileName(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8fileNameERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::hasChildren(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QStringList QFileSystemModel::mimeTypes();
  fn _ZNK16QFileSystemModel9mimeTypesEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::FreeQFileSystemModel();
  fn _ZN16QFileSystemModelD0Ev(qthis: *mut c_void) ;
  // proto:  QModelIndex QFileSystemModel::index(const QString & path, int column);
  fn _ZNK16QFileSystemModel5indexERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  void QFileSystemModel::setNameFilterDisables(bool enable);
  fn _ZN16QFileSystemModel21setNameFilterDisablesEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8fileIconERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::resolveSymlinks();
  fn _ZNK16QFileSystemModel15resolveSymlinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileSystemModel::filePath(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8filePathERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QFileSystemModel::parent(const QModelIndex & child);
  fn _ZNK16QFileSystemModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::nameFilterDisables();
  fn _ZNK16QFileSystemModel18nameFilterDisablesEv(qthis: *mut c_void) -> int8_t;
  // proto:  const QMetaObject * QFileSystemModel::metaObject();
  fn _ZNK16QFileSystemModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFileSystemModel::fetchMore(const QModelIndex & parent);
  fn _ZN16QFileSystemModel9fetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileSystemModel::NewQFileSystemModel(const QFileSystemModel & );
  fn _ZN16QFileSystemModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  long long QFileSystemModel::size(const QModelIndex & index);
  fn _ZNK16QFileSystemModel4sizeERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  QFileIconProvider * QFileSystemModel::iconProvider();
  fn _ZNK16QFileSystemModel12iconProviderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::setNameFilters(const QStringList & filters);
  fn _ZN16QFileSystemModel14setNameFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVariant QFileSystemModel::data(const QModelIndex & index, int role);
  fn _ZNK16QFileSystemModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QDir QFileSystemModel::rootDirectory();
  fn _ZNK16QFileSystemModel13rootDirectoryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
  fn _ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QFileSystemModel::rowCount(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  int QFileSystemModel::columnCount(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK16QFileSystemModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QFileSystemModel::remove(const QModelIndex & index);
  fn _ZN16QFileSystemModel6removeERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QFileSystemModel::rootPathChanged(const QString & newPath);
  fn _ZN16QFileSystemModel15rootPathChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QFileSystemModel::setRootPath(const QString & path);
  fn _ZN16QFileSystemModel11setRootPathERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::directoryLoaded(const QString & path);
  fn _ZN16QFileSystemModel15directoryLoadedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileSystemModel::setResolveSymlinks(bool enable);
  fn _ZN16QFileSystemModel18setResolveSymlinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFileSystemModel::setReadOnly(bool enable);
  fn _ZN16QFileSystemModel11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QStringList QFileSystemModel::nameFilters();
  fn _ZNK16QFileSystemModel11nameFiltersEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::NewQFileSystemModel(QObject * parent);
  fn _ZN16QFileSystemModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8fileInfoERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariant QFileSystemModel::myComputer(int role);
  fn _ZNK16QFileSystemModel10myComputerEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QFileSystemModel::isReadOnly();
  fn _ZNK16QFileSystemModel10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileSystemModel::type_(const QModelIndex & index);
  fn _ZNK16QFileSystemModel4typeERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QString QFileSystemModel::rootPath();
  fn _ZNK16QFileSystemModel8rootPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
  fn _ZNK16QFileSystemModel12lastModifiedERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
  fn _ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QFileSystemModel::isDir(const QModelIndex & index);
  fn _ZNK16QFileSystemModel5isDirERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QFileSystemModel::rmdir(const QModelIndex & index);
  fn _ZN16QFileSystemModel5rmdirERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
  fn _ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QFileSystemModel)=1
pub struct QFileSystemModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileSystemModel {
  pub fn fileName<T: QFileSystemModel_fileName>(&mut self, value: T) -> QString {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileName {
  fn fileName(self, rsthis: &mut QFileSystemModel) -> QString;
}

// proto:  QString QFileSystemModel::fileName(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileName for (&'a  QModelIndex) {
  fn fileName(self, rsthis: &mut QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileNameERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8fileNameERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn hasChildren<T: QFileSystemModel_hasChildren>(&mut self, value: T) -> i8 {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QFileSystemModel_hasChildren {
  fn hasChildren(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_hasChildren for (&'a  QModelIndex) {
  fn hasChildren(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn mimeTypes<T: QFileSystemModel_mimeTypes>(&mut self, value: T) -> QStringList {
    return value.mimeTypes(self);
    // return 1;
  }
}

pub trait QFileSystemModel_mimeTypes {
  fn mimeTypes(self, rsthis: &mut QFileSystemModel) -> QStringList;
}

// proto:  QStringList QFileSystemModel::mimeTypes();
impl<'a> /*trait*/ QFileSystemModel_mimeTypes for () {
  fn mimeTypes(self, rsthis: &mut QFileSystemModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel9mimeTypesEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel9mimeTypesEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn FreeQFileSystemModel<T: QFileSystemModel_FreeQFileSystemModel>(&mut self, value: T)  {
     value.FreeQFileSystemModel(self);
    // return 1;
  }
}

pub trait QFileSystemModel_FreeQFileSystemModel {
  fn FreeQFileSystemModel(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::FreeQFileSystemModel();
impl<'a> /*trait*/ QFileSystemModel_FreeQFileSystemModel for () {
  fn FreeQFileSystemModel(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelD0Ev()};
     unsafe {_ZN16QFileSystemModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn index<T: QFileSystemModel_index>(&mut self, value: T) -> QModelIndex {
    return value.index(self);
    // return 1;
  }
}

pub trait QFileSystemModel_index {
  fn index(self, rsthis: &mut QFileSystemModel) -> QModelIndex;
}

// proto:  QModelIndex QFileSystemModel::index(const QString & path, int column);
impl<'a> /*trait*/ QFileSystemModel_index for (&'a  QString, i32) {
  fn index(self, rsthis: &mut QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5indexERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK16QFileSystemModel5indexERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setNameFilterDisables<T: QFileSystemModel_setNameFilterDisables>(&mut self, value: T)  {
     value.setNameFilterDisables(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setNameFilterDisables {
  fn setNameFilterDisables(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::setNameFilterDisables(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setNameFilterDisables for (i8) {
  fn setNameFilterDisables(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel21setNameFilterDisablesEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QFileSystemModel21setNameFilterDisablesEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fileIcon<T: QFileSystemModel_fileIcon>(&mut self, value: T) -> QIcon {
    return value.fileIcon(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileIcon {
  fn fileIcon(self, rsthis: &mut QFileSystemModel) -> QIcon;
}

// proto:  QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileIcon for (&'a  QModelIndex) {
  fn fileIcon(self, rsthis: &mut QFileSystemModel) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileIconERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8fileIconERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn resolveSymlinks<T: QFileSystemModel_resolveSymlinks>(&mut self, value: T) -> i8 {
    return value.resolveSymlinks(self);
    // return 1;
  }
}

pub trait QFileSystemModel_resolveSymlinks {
  fn resolveSymlinks(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::resolveSymlinks();
impl<'a> /*trait*/ QFileSystemModel_resolveSymlinks for () {
  fn resolveSymlinks(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel15resolveSymlinksEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel15resolveSymlinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn filePath<T: QFileSystemModel_filePath>(&mut self, value: T) -> QString {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QFileSystemModel_filePath {
  fn filePath(self, rsthis: &mut QFileSystemModel) -> QString;
}

// proto:  QString QFileSystemModel::filePath(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_filePath for (&'a  QModelIndex) {
  fn filePath(self, rsthis: &mut QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8filePathERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8filePathERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn parent<T: QFileSystemModel_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QFileSystemModel_parent {
  fn parent(self, rsthis: &mut QFileSystemModel) -> QModelIndex;
}

// proto:  QModelIndex QFileSystemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QFileSystemModel_parent for (&'a  QModelIndex) {
  fn parent(self, rsthis: &mut QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn nameFilterDisables<T: QFileSystemModel_nameFilterDisables>(&mut self, value: T) -> i8 {
    return value.nameFilterDisables(self);
    // return 1;
  }
}

pub trait QFileSystemModel_nameFilterDisables {
  fn nameFilterDisables(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::nameFilterDisables();
impl<'a> /*trait*/ QFileSystemModel_nameFilterDisables for () {
  fn nameFilterDisables(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel18nameFilterDisablesEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel18nameFilterDisablesEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn metaObject<T: QFileSystemModel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QFileSystemModel_metaObject {
  fn metaObject(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  const QMetaObject * QFileSystemModel::metaObject();
impl<'a> /*trait*/ QFileSystemModel_metaObject for () {
  fn metaObject(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10metaObjectEv()};
     unsafe {_ZNK16QFileSystemModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fetchMore<T: QFileSystemModel_fetchMore>(&mut self, value: T)  {
     value.fetchMore(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fetchMore {
  fn fetchMore(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_fetchMore for (&'a  QModelIndex) {
  fn fetchMore(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn NewQFileSystemModel<T: QFileSystemModel_NewQFileSystemModel>(value: T) -> QFileSystemModel {
    let rsthis = value.NewQFileSystemModel();
    return rsthis;
    // return 1;
  }
}

pub trait QFileSystemModel_NewQFileSystemModel {
  fn NewQFileSystemModel(self) -> QFileSystemModel;
}

// proto: void QFileSystemModel::NewQFileSystemModel(const QFileSystemModel & );
impl<'a> /*trait*/ QFileSystemModel_NewQFileSystemModel for (&'a  QFileSystemModel) {
  fn NewQFileSystemModel(self) -> QFileSystemModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QFileSystemModelC1ERKS_(qthis, arg0)};
    let rsthis = QFileSystemModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn size<T: QFileSystemModel_size>(&mut self, value: T) -> i64 {
    return value.size(self);
    // return 1;
  }
}

pub trait QFileSystemModel_size {
  fn size(self, rsthis: &mut QFileSystemModel) -> i64;
}

// proto:  long long QFileSystemModel::size(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_size for (&'a  QModelIndex) {
  fn size(self, rsthis: &mut QFileSystemModel) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4sizeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel4sizeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn iconProvider<T: QFileSystemModel_iconProvider>(&mut self, value: T) -> QFileIconProvider {
    return value.iconProvider(self);
    // return 1;
  }
}

pub trait QFileSystemModel_iconProvider {
  fn iconProvider(self, rsthis: &mut QFileSystemModel) -> QFileIconProvider;
}

// proto:  QFileIconProvider * QFileSystemModel::iconProvider();
impl<'a> /*trait*/ QFileSystemModel_iconProvider for () {
  fn iconProvider(self, rsthis: &mut QFileSystemModel) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12iconProviderEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel12iconProviderEv(rsthis.qclsinst)};
    let mut ret1 = QFileIconProvider{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setNameFilters<T: QFileSystemModel_setNameFilters>(&mut self, value: T)  {
     value.setNameFilters(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setNameFilters {
  fn setNameFilters(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileSystemModel_setNameFilters for (&'a  QStringList) {
  fn setNameFilters(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn data<T: QFileSystemModel_data>(&mut self, value: T) -> QVariant {
    return value.data(self);
    // return 1;
  }
}

pub trait QFileSystemModel_data {
  fn data(self, rsthis: &mut QFileSystemModel) -> QVariant;
}

// proto:  QVariant QFileSystemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QFileSystemModel_data for (&'a  QModelIndex, i32) {
  fn data(self, rsthis: &mut QFileSystemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK16QFileSystemModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rootDirectory<T: QFileSystemModel_rootDirectory>(&mut self, value: T) -> QDir {
    return value.rootDirectory(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rootDirectory {
  fn rootDirectory(self, rsthis: &mut QFileSystemModel) -> QDir;
}

// proto:  QDir QFileSystemModel::rootDirectory();
impl<'a> /*trait*/ QFileSystemModel_rootDirectory for () {
  fn rootDirectory(self, rsthis: &mut QFileSystemModel) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel13rootDirectoryEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel13rootDirectoryEv(rsthis.qclsinst)};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn mkdir<T: QFileSystemModel_mkdir>(&mut self, value: T) -> QModelIndex {
    return value.mkdir(self);
    // return 1;
  }
}

pub trait QFileSystemModel_mkdir {
  fn mkdir(self, rsthis: &mut QFileSystemModel) -> QModelIndex;
}

// proto:  QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
impl<'a> /*trait*/ QFileSystemModel_mkdir for (&'a  QModelIndex, &'a  QString) {
  fn mkdir(self, rsthis: &mut QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rowCount<T: QFileSystemModel_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rowCount {
  fn rowCount(self, rsthis: &mut QFileSystemModel) -> i32;
}

// proto:  int QFileSystemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setData<T: QFileSystemModel_setData>(&mut self, value: T) -> i8 {
    return value.setData(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setData {
  fn setData(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QFileSystemModel_setData for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn columnCount<T: QFileSystemModel_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QFileSystemModel_columnCount {
  fn columnCount(self, rsthis: &mut QFileSystemModel) -> i32;
}

// proto:  int QFileSystemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, rsthis: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, rsthis: &mut QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn canFetchMore<T: QFileSystemModel_canFetchMore>(&mut self, value: T) -> i8 {
    return value.canFetchMore(self);
    // return 1;
  }
}

pub trait QFileSystemModel_canFetchMore {
  fn canFetchMore(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_canFetchMore for (&'a  QModelIndex) {
  fn canFetchMore(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn remove<T: QFileSystemModel_remove>(&mut self, value: T) -> i8 {
    return value.remove(self);
    // return 1;
  }
}

pub trait QFileSystemModel_remove {
  fn remove(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::remove(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_remove for (&'a  QModelIndex) {
  fn remove(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel6removeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel6removeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rootPathChanged<T: QFileSystemModel_rootPathChanged>(&mut self, value: T)  {
     value.rootPathChanged(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rootPathChanged {
  fn rootPathChanged(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::rootPathChanged(const QString & newPath);
impl<'a> /*trait*/ QFileSystemModel_rootPathChanged for (&'a  QString) {
  fn rootPathChanged(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15rootPathChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel15rootPathChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setRootPath<T: QFileSystemModel_setRootPath>(&mut self, value: T) -> QModelIndex {
    return value.setRootPath(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setRootPath {
  fn setRootPath(self, rsthis: &mut QFileSystemModel) -> QModelIndex;
}

// proto:  QModelIndex QFileSystemModel::setRootPath(const QString & path);
impl<'a> /*trait*/ QFileSystemModel_setRootPath for (&'a  QString) {
  fn setRootPath(self, rsthis: &mut QFileSystemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11setRootPathERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel11setRootPathERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn directoryLoaded<T: QFileSystemModel_directoryLoaded>(&mut self, value: T)  {
     value.directoryLoaded(self);
    // return 1;
  }
}

pub trait QFileSystemModel_directoryLoaded {
  fn directoryLoaded(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::directoryLoaded(const QString & path);
impl<'a> /*trait*/ QFileSystemModel_directoryLoaded for (&'a  QString) {
  fn directoryLoaded(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15directoryLoadedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel15directoryLoadedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setResolveSymlinks<T: QFileSystemModel_setResolveSymlinks>(&mut self, value: T)  {
     value.setResolveSymlinks(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setResolveSymlinks {
  fn setResolveSymlinks(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::setResolveSymlinks(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setResolveSymlinks for (i8) {
  fn setResolveSymlinks(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel18setResolveSymlinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QFileSystemModel18setResolveSymlinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setReadOnly<T: QFileSystemModel_setReadOnly>(&mut self, value: T)  {
     value.setReadOnly(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setReadOnly {
  fn setReadOnly(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::setReadOnly(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setReadOnly for (i8) {
  fn setReadOnly(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN16QFileSystemModel11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn nameFilters<T: QFileSystemModel_nameFilters>(&mut self, value: T) -> QStringList {
    return value.nameFilters(self);
    // return 1;
  }
}

pub trait QFileSystemModel_nameFilters {
  fn nameFilters(self, rsthis: &mut QFileSystemModel) -> QStringList;
}

// proto:  QStringList QFileSystemModel::nameFilters();
impl<'a> /*trait*/ QFileSystemModel_nameFilters for () {
  fn nameFilters(self, rsthis: &mut QFileSystemModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11nameFiltersEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel11nameFiltersEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QFileSystemModel::NewQFileSystemModel(QObject * parent);
impl<'a> /*trait*/ QFileSystemModel_NewQFileSystemModel for (&'a mut QObject) {
  fn NewQFileSystemModel(self) -> QFileSystemModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QFileSystemModelC1EP7QObject(qthis, arg0)};
    let rsthis = QFileSystemModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fileInfo<T: QFileSystemModel_fileInfo>(&mut self, value: T) -> QFileInfo {
    return value.fileInfo(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileInfo {
  fn fileInfo(self, rsthis: &mut QFileSystemModel) -> QFileInfo;
}

// proto:  QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileInfo for (&'a  QModelIndex) {
  fn fileInfo(self, rsthis: &mut QFileSystemModel) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileInfoERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel8fileInfoERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QFileInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn myComputer<T: QFileSystemModel_myComputer>(&mut self, value: T) -> QVariant {
    return value.myComputer(self);
    // return 1;
  }
}

pub trait QFileSystemModel_myComputer {
  fn myComputer(self, rsthis: &mut QFileSystemModel) -> QVariant;
}

// proto:  QVariant QFileSystemModel::myComputer(int role);
impl<'a> /*trait*/ QFileSystemModel_myComputer for (i32) {
  fn myComputer(self, rsthis: &mut QFileSystemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10myComputerEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK16QFileSystemModel10myComputerEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn isReadOnly<T: QFileSystemModel_isReadOnly>(&mut self, value: T) -> i8 {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QFileSystemModel_isReadOnly {
  fn isReadOnly(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::isReadOnly();
impl<'a> /*trait*/ QFileSystemModel_isReadOnly for () {
  fn isReadOnly(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn type_<T: QFileSystemModel_type_>(&mut self, value: T) -> QString {
    return value.type_(self);
    // return 1;
  }
}

pub trait QFileSystemModel_type_ {
  fn type_(self, rsthis: &mut QFileSystemModel) -> QString;
}

// proto:  QString QFileSystemModel::type_(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_type_ for (&'a  QModelIndex) {
  fn type_(self, rsthis: &mut QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4typeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel4typeERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rootPath<T: QFileSystemModel_rootPath>(&mut self, value: T) -> QString {
    return value.rootPath(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rootPath {
  fn rootPath(self, rsthis: &mut QFileSystemModel) -> QString;
}

// proto:  QString QFileSystemModel::rootPath();
impl<'a> /*trait*/ QFileSystemModel_rootPath for () {
  fn rootPath(self, rsthis: &mut QFileSystemModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8rootPathEv()};
    let mut ret = unsafe {_ZNK16QFileSystemModel8rootPathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn lastModified<T: QFileSystemModel_lastModified>(&mut self, value: T) -> QDateTime {
    return value.lastModified(self);
    // return 1;
  }
}

pub trait QFileSystemModel_lastModified {
  fn lastModified(self, rsthis: &mut QFileSystemModel) -> QDateTime;
}

// proto:  QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_lastModified for (&'a  QModelIndex) {
  fn lastModified(self, rsthis: &mut QFileSystemModel) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fileRenamed<T: QFileSystemModel_fileRenamed>(&mut self, value: T)  {
     value.fileRenamed(self);
    // return 1;
  }
}

pub trait QFileSystemModel_fileRenamed {
  fn fileRenamed(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QFileSystemModel_fileRenamed for (&'a  QString, &'a  QString, &'a  QString) {
  fn fileRenamed(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn isDir<T: QFileSystemModel_isDir>(&mut self, value: T) -> i8 {
    return value.isDir(self);
    // return 1;
  }
}

pub trait QFileSystemModel_isDir {
  fn isDir(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::isDir(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_isDir for (&'a  QModelIndex) {
  fn isDir(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5isDirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QFileSystemModel5isDirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rmdir<T: QFileSystemModel_rmdir>(&mut self, value: T) -> i8 {
    return value.rmdir(self);
    // return 1;
  }
}

pub trait QFileSystemModel_rmdir {
  fn rmdir(self, rsthis: &mut QFileSystemModel) -> i8;
}

// proto:  bool QFileSystemModel::rmdir(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_rmdir for (&'a  QModelIndex) {
  fn rmdir(self, rsthis: &mut QFileSystemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel5rmdirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QFileSystemModel5rmdirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setIconProvider<T: QFileSystemModel_setIconProvider>(&mut self, value: T)  {
     value.setIconProvider(self);
    // return 1;
  }
}

pub trait QFileSystemModel_setIconProvider {
  fn setIconProvider(self, rsthis: &mut QFileSystemModel) ;
}

// proto:  void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QFileSystemModel_setIconProvider for (&'a mut QFileIconProvider) {
  fn setIconProvider(self, rsthis: &mut QFileSystemModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

