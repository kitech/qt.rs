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
use super::qvariant::QVariant;
use super::qobject::QObject;
use super::qfileiconprovider::QFileIconProvider;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QFileSystemModel::fileName(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8fileNameERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QFileSystemModel::hasChildren(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel11hasChildrenERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: QStringList QFileSystemModel::mimeTypes();
  fn _ZNK16QFileSystemModel9mimeTypesEv() -> i32;
  // proto: void QFileSystemModel::FreeQFileSystemModel();
  fn _ZN16QFileSystemModelD0Ev() -> i32;
  // proto: QModelIndex QFileSystemModel::index(const QString & path, int column);
  fn _ZNK16QFileSystemModel5indexERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QFileSystemModel::setNameFilterDisables(bool enable);
  fn _ZN16QFileSystemModel21setNameFilterDisablesEb(arg0: int8_t) -> i32;
  // proto: QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8fileIconERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QFileSystemModel::resolveSymlinks();
  fn _ZNK16QFileSystemModel15resolveSymlinksEv() -> i32;
  // proto: QString QFileSystemModel::filePath(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8filePathERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: QModelIndex QFileSystemModel::parent(const QModelIndex & child);
  fn _ZNK16QFileSystemModel6parentERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QFileSystemModel::nameFilterDisables();
  fn _ZNK16QFileSystemModel18nameFilterDisablesEv() -> i32;
  // proto: const QMetaObject * QFileSystemModel::metaObject();
  fn _ZNK16QFileSystemModel10metaObjectEv() -> i32;
  // proto: void QFileSystemModel::fetchMore(const QModelIndex & parent);
  fn _ZN16QFileSystemModel9fetchMoreERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QFileSystemModel::NewQFileSystemModel(const QFileSystemModel & );
  fn _ZN16QFileSystemModelC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: long long QFileSystemModel::size(const QModelIndex & index);
  fn _ZNK16QFileSystemModel4sizeERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: QFileIconProvider * QFileSystemModel::iconProvider();
  fn _ZNK16QFileSystemModel12iconProviderEv() -> i32;
  // proto: void QFileSystemModel::setNameFilters(const QStringList & filters);
  fn _ZN16QFileSystemModel14setNameFiltersERK11QStringList(arg0: *const c_void) -> i32;
  // proto: QVariant QFileSystemModel::data(const QModelIndex & index, int role);
  fn _ZNK16QFileSystemModel4dataERK11QModelIndexi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QDir QFileSystemModel::rootDirectory();
  fn _ZNK16QFileSystemModel13rootDirectoryEv() -> i32;
  // proto: QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
  fn _ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: int QFileSystemModel::rowCount(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel8rowCountERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti(arg0: *const c_void, arg1: *const c_void, arg2: c_int) -> i32;
  // proto: int QFileSystemModel::columnCount(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel11columnCountERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK16QFileSystemModel5indexEiiRK11QModelIndex(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QFileSystemModel::remove(const QModelIndex & index);
  fn _ZN16QFileSystemModel6removeERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QFileSystemModel::rootPathChanged(const QString & newPath);
  fn _ZN16QFileSystemModel15rootPathChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: QModelIndex QFileSystemModel::setRootPath(const QString & path);
  fn _ZN16QFileSystemModel11setRootPathERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFileSystemModel::directoryLoaded(const QString & path);
  fn _ZN16QFileSystemModel15directoryLoadedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFileSystemModel::setResolveSymlinks(bool enable);
  fn _ZN16QFileSystemModel18setResolveSymlinksEb(arg0: int8_t) -> i32;
  // proto: void QFileSystemModel::setReadOnly(bool enable);
  fn _ZN16QFileSystemModel11setReadOnlyEb(arg0: int8_t) -> i32;
  // proto: QStringList QFileSystemModel::nameFilters();
  fn _ZNK16QFileSystemModel11nameFiltersEv() -> i32;
  // proto: void QFileSystemModel::NewQFileSystemModel(QObject * parent);
  fn _ZN16QFileSystemModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
  fn _ZNK16QFileSystemModel8fileInfoERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: QVariant QFileSystemModel::myComputer(int role);
  fn _ZNK16QFileSystemModel10myComputerEi(arg0: c_int) -> i32;
  // proto: bool QFileSystemModel::isReadOnly();
  fn _ZNK16QFileSystemModel10isReadOnlyEv() -> i32;
  // proto: QString QFileSystemModel::type_(const QModelIndex & index);
  fn _ZNK16QFileSystemModel4typeERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: QString QFileSystemModel::rootPath();
  fn _ZNK16QFileSystemModel8rootPathEv() -> i32;
  // proto: QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
  fn _ZNK16QFileSystemModel12lastModifiedERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
  fn _ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: bool QFileSystemModel::isDir(const QModelIndex & index);
  fn _ZNK16QFileSystemModel5isDirERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: bool QFileSystemModel::rmdir(const QModelIndex & index);
  fn _ZN16QFileSystemModel5rmdirERK11QModelIndex(arg0: *const c_void) -> i32;
  // proto: void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
  fn _ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QFileSystemModel)=1
pub struct QFileSystemModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileSystemModel {
  pub fn fileName<T: QFileSystemModel_fileName>(&mut self, value: T) -> i32 {
    value.fileName(self);
    return 1;
  }
}

pub trait QFileSystemModel_fileName {
  fn fileName(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QString QFileSystemModel::fileName(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileName for (&'a  QModelIndex) {
  fn fileName(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileNameERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel8fileNameERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn hasChildren<T: QFileSystemModel_hasChildren>(&mut self, value: T) -> i32 {
    value.hasChildren(self);
    return 1;
  }
}

pub trait QFileSystemModel_hasChildren {
  fn hasChildren(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_hasChildren for (&'a  QModelIndex) {
  fn hasChildren(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel11hasChildrenERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn mimeTypes<T: QFileSystemModel_mimeTypes>(&mut self, value: T) -> i32 {
    value.mimeTypes(self);
    return 1;
  }
}

pub trait QFileSystemModel_mimeTypes {
  fn mimeTypes(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QStringList QFileSystemModel::mimeTypes();
impl<'a> /*trait*/ QFileSystemModel_mimeTypes for () {
  fn mimeTypes(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel9mimeTypesEv()};
    unsafe {_ZNK16QFileSystemModel9mimeTypesEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn FreeQFileSystemModel<T: QFileSystemModel_FreeQFileSystemModel>(&mut self, value: T) -> i32 {
    value.FreeQFileSystemModel(self);
    return 1;
  }
}

pub trait QFileSystemModel_FreeQFileSystemModel {
  fn FreeQFileSystemModel(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::FreeQFileSystemModel();
impl<'a> /*trait*/ QFileSystemModel_FreeQFileSystemModel for () {
  fn FreeQFileSystemModel(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModelD0Ev()};
    unsafe {_ZN16QFileSystemModelD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn index<T: QFileSystemModel_index>(&mut self, value: T) -> i32 {
    value.index(self);
    return 1;
  }
}

pub trait QFileSystemModel_index {
  fn index(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QModelIndex QFileSystemModel::index(const QString & path, int column);
impl<'a> /*trait*/ QFileSystemModel_index for (&'a  QString, i32) {
  fn index(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5indexERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK16QFileSystemModel5indexERK7QStringi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setNameFilterDisables<T: QFileSystemModel_setNameFilterDisables>(&mut self, value: T) -> i32 {
    value.setNameFilterDisables(self);
    return 1;
  }
}

pub trait QFileSystemModel_setNameFilterDisables {
  fn setNameFilterDisables(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::setNameFilterDisables(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setNameFilterDisables for (i8) {
  fn setNameFilterDisables(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel21setNameFilterDisablesEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QFileSystemModel21setNameFilterDisablesEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fileIcon<T: QFileSystemModel_fileIcon>(&mut self, value: T) -> i32 {
    value.fileIcon(self);
    return 1;
  }
}

pub trait QFileSystemModel_fileIcon {
  fn fileIcon(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QIcon QFileSystemModel::fileIcon(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileIcon for (&'a  QModelIndex) {
  fn fileIcon(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileIconERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel8fileIconERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn resolveSymlinks<T: QFileSystemModel_resolveSymlinks>(&mut self, value: T) -> i32 {
    value.resolveSymlinks(self);
    return 1;
  }
}

pub trait QFileSystemModel_resolveSymlinks {
  fn resolveSymlinks(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::resolveSymlinks();
impl<'a> /*trait*/ QFileSystemModel_resolveSymlinks for () {
  fn resolveSymlinks(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel15resolveSymlinksEv()};
    unsafe {_ZNK16QFileSystemModel15resolveSymlinksEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn filePath<T: QFileSystemModel_filePath>(&mut self, value: T) -> i32 {
    value.filePath(self);
    return 1;
  }
}

pub trait QFileSystemModel_filePath {
  fn filePath(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QString QFileSystemModel::filePath(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_filePath for (&'a  QModelIndex) {
  fn filePath(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8filePathERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel8filePathERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn parent<T: QFileSystemModel_parent>(&mut self, value: T) -> i32 {
    value.parent(self);
    return 1;
  }
}

pub trait QFileSystemModel_parent {
  fn parent(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QModelIndex QFileSystemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QFileSystemModel_parent for (&'a  QModelIndex) {
  fn parent(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel6parentERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn nameFilterDisables<T: QFileSystemModel_nameFilterDisables>(&mut self, value: T) -> i32 {
    value.nameFilterDisables(self);
    return 1;
  }
}

pub trait QFileSystemModel_nameFilterDisables {
  fn nameFilterDisables(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::nameFilterDisables();
impl<'a> /*trait*/ QFileSystemModel_nameFilterDisables for () {
  fn nameFilterDisables(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel18nameFilterDisablesEv()};
    unsafe {_ZNK16QFileSystemModel18nameFilterDisablesEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn metaObject<T: QFileSystemModel_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFileSystemModel_metaObject {
  fn metaObject(self, this: &mut QFileSystemModel) -> i32;
}

// proto: const QMetaObject * QFileSystemModel::metaObject();
impl<'a> /*trait*/ QFileSystemModel_metaObject for () {
  fn metaObject(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10metaObjectEv()};
    unsafe {_ZNK16QFileSystemModel10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fetchMore<T: QFileSystemModel_fetchMore>(&mut self, value: T) -> i32 {
    value.fetchMore(self);
    return 1;
  }
}

pub trait QFileSystemModel_fetchMore {
  fn fetchMore(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_fetchMore for (&'a  QModelIndex) {
  fn fetchMore(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel9fetchMoreERK11QModelIndex(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModelC1ERKS_(qthis, arg0)};
    let rsthis = QFileSystemModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn size<T: QFileSystemModel_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QFileSystemModel_size {
  fn size(self, this: &mut QFileSystemModel) -> i32;
}

// proto: long long QFileSystemModel::size(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_size for (&'a  QModelIndex) {
  fn size(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4sizeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel4sizeERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn iconProvider<T: QFileSystemModel_iconProvider>(&mut self, value: T) -> i32 {
    value.iconProvider(self);
    return 1;
  }
}

pub trait QFileSystemModel_iconProvider {
  fn iconProvider(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QFileIconProvider * QFileSystemModel::iconProvider();
impl<'a> /*trait*/ QFileSystemModel_iconProvider for () {
  fn iconProvider(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12iconProviderEv()};
    unsafe {_ZNK16QFileSystemModel12iconProviderEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setNameFilters<T: QFileSystemModel_setNameFilters>(&mut self, value: T) -> i32 {
    value.setNameFilters(self);
    return 1;
  }
}

pub trait QFileSystemModel_setNameFilters {
  fn setNameFilters(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileSystemModel_setNameFilters for (&'a  QStringList) {
  fn setNameFilters(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel14setNameFiltersERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn data<T: QFileSystemModel_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QFileSystemModel_data {
  fn data(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QVariant QFileSystemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QFileSystemModel_data for (&'a  QModelIndex, i32) {
  fn data(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK16QFileSystemModel4dataERK11QModelIndexi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rootDirectory<T: QFileSystemModel_rootDirectory>(&mut self, value: T) -> i32 {
    value.rootDirectory(self);
    return 1;
  }
}

pub trait QFileSystemModel_rootDirectory {
  fn rootDirectory(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QDir QFileSystemModel::rootDirectory();
impl<'a> /*trait*/ QFileSystemModel_rootDirectory for () {
  fn rootDirectory(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel13rootDirectoryEv()};
    unsafe {_ZNK16QFileSystemModel13rootDirectoryEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn mkdir<T: QFileSystemModel_mkdir>(&mut self, value: T) -> i32 {
    value.mkdir(self);
    return 1;
  }
}

pub trait QFileSystemModel_mkdir {
  fn mkdir(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QModelIndex QFileSystemModel::mkdir(const QModelIndex & parent, const QString & name);
impl<'a> /*trait*/ QFileSystemModel_mkdir for (&'a  QModelIndex, &'a  QString) {
  fn mkdir(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel5mkdirERK11QModelIndexRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rowCount<T: QFileSystemModel_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QFileSystemModel_rowCount {
  fn rowCount(self, this: &mut QFileSystemModel) -> i32;
}

// proto: int QFileSystemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel8rowCountERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setData<T: QFileSystemModel_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QFileSystemModel_setData {
  fn setData(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QFileSystemModel_setData for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN16QFileSystemModel7setDataERK11QModelIndexRK8QVarianti(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn columnCount<T: QFileSystemModel_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QFileSystemModel_columnCount {
  fn columnCount(self, this: &mut QFileSystemModel) -> i32;
}

// proto: int QFileSystemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel11columnCountERK11QModelIndex(arg0)};
    return 1;
  }
}

// proto: QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel5indexEiiRK11QModelIndex(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn canFetchMore<T: QFileSystemModel_canFetchMore>(&mut self, value: T) -> i32 {
    value.canFetchMore(self);
    return 1;
  }
}

pub trait QFileSystemModel_canFetchMore {
  fn canFetchMore(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QFileSystemModel_canFetchMore for (&'a  QModelIndex) {
  fn canFetchMore(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel12canFetchMoreERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn remove<T: QFileSystemModel_remove>(&mut self, value: T) -> i32 {
    value.remove(self);
    return 1;
  }
}

pub trait QFileSystemModel_remove {
  fn remove(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::remove(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_remove for (&'a  QModelIndex) {
  fn remove(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel6removeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel6removeERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rootPathChanged<T: QFileSystemModel_rootPathChanged>(&mut self, value: T) -> i32 {
    value.rootPathChanged(self);
    return 1;
  }
}

pub trait QFileSystemModel_rootPathChanged {
  fn rootPathChanged(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::rootPathChanged(const QString & newPath);
impl<'a> /*trait*/ QFileSystemModel_rootPathChanged for (&'a  QString) {
  fn rootPathChanged(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15rootPathChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel15rootPathChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setRootPath<T: QFileSystemModel_setRootPath>(&mut self, value: T) -> i32 {
    value.setRootPath(self);
    return 1;
  }
}

pub trait QFileSystemModel_setRootPath {
  fn setRootPath(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QModelIndex QFileSystemModel::setRootPath(const QString & path);
impl<'a> /*trait*/ QFileSystemModel_setRootPath for (&'a  QString) {
  fn setRootPath(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11setRootPathERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel11setRootPathERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn directoryLoaded<T: QFileSystemModel_directoryLoaded>(&mut self, value: T) -> i32 {
    value.directoryLoaded(self);
    return 1;
  }
}

pub trait QFileSystemModel_directoryLoaded {
  fn directoryLoaded(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::directoryLoaded(const QString & path);
impl<'a> /*trait*/ QFileSystemModel_directoryLoaded for (&'a  QString) {
  fn directoryLoaded(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15directoryLoadedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel15directoryLoadedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setResolveSymlinks<T: QFileSystemModel_setResolveSymlinks>(&mut self, value: T) -> i32 {
    value.setResolveSymlinks(self);
    return 1;
  }
}

pub trait QFileSystemModel_setResolveSymlinks {
  fn setResolveSymlinks(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::setResolveSymlinks(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setResolveSymlinks for (i8) {
  fn setResolveSymlinks(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel18setResolveSymlinksEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QFileSystemModel18setResolveSymlinksEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setReadOnly<T: QFileSystemModel_setReadOnly>(&mut self, value: T) -> i32 {
    value.setReadOnly(self);
    return 1;
  }
}

pub trait QFileSystemModel_setReadOnly {
  fn setReadOnly(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::setReadOnly(bool enable);
impl<'a> /*trait*/ QFileSystemModel_setReadOnly for (i8) {
  fn setReadOnly(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11setReadOnlyEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN16QFileSystemModel11setReadOnlyEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn nameFilters<T: QFileSystemModel_nameFilters>(&mut self, value: T) -> i32 {
    value.nameFilters(self);
    return 1;
  }
}

pub trait QFileSystemModel_nameFilters {
  fn nameFilters(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QStringList QFileSystemModel::nameFilters();
impl<'a> /*trait*/ QFileSystemModel_nameFilters for () {
  fn nameFilters(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel11nameFiltersEv()};
    unsafe {_ZNK16QFileSystemModel11nameFiltersEv()};
    return 1;
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
  pub fn fileInfo<T: QFileSystemModel_fileInfo>(&mut self, value: T) -> i32 {
    value.fileInfo(self);
    return 1;
  }
}

pub trait QFileSystemModel_fileInfo {
  fn fileInfo(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QFileInfo QFileSystemModel::fileInfo(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_fileInfo for (&'a  QModelIndex) {
  fn fileInfo(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8fileInfoERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel8fileInfoERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn myComputer<T: QFileSystemModel_myComputer>(&mut self, value: T) -> i32 {
    value.myComputer(self);
    return 1;
  }
}

pub trait QFileSystemModel_myComputer {
  fn myComputer(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QVariant QFileSystemModel::myComputer(int role);
impl<'a> /*trait*/ QFileSystemModel_myComputer for (i32) {
  fn myComputer(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10myComputerEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK16QFileSystemModel10myComputerEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn isReadOnly<T: QFileSystemModel_isReadOnly>(&mut self, value: T) -> i32 {
    value.isReadOnly(self);
    return 1;
  }
}

pub trait QFileSystemModel_isReadOnly {
  fn isReadOnly(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::isReadOnly();
impl<'a> /*trait*/ QFileSystemModel_isReadOnly for () {
  fn isReadOnly(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel10isReadOnlyEv()};
    unsafe {_ZNK16QFileSystemModel10isReadOnlyEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn type_<T: QFileSystemModel_type_>(&mut self, value: T) -> i32 {
    value.type_(self);
    return 1;
  }
}

pub trait QFileSystemModel_type_ {
  fn type_(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QString QFileSystemModel::type_(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_type_ for (&'a  QModelIndex) {
  fn type_(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel4typeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel4typeERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rootPath<T: QFileSystemModel_rootPath>(&mut self, value: T) -> i32 {
    value.rootPath(self);
    return 1;
  }
}

pub trait QFileSystemModel_rootPath {
  fn rootPath(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QString QFileSystemModel::rootPath();
impl<'a> /*trait*/ QFileSystemModel_rootPath for () {
  fn rootPath(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel8rootPathEv()};
    unsafe {_ZNK16QFileSystemModel8rootPathEv()};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn lastModified<T: QFileSystemModel_lastModified>(&mut self, value: T) -> i32 {
    value.lastModified(self);
    return 1;
  }
}

pub trait QFileSystemModel_lastModified {
  fn lastModified(self, this: &mut QFileSystemModel) -> i32;
}

// proto: QDateTime QFileSystemModel::lastModified(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_lastModified for (&'a  QModelIndex) {
  fn lastModified(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel12lastModifiedERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn fileRenamed<T: QFileSystemModel_fileRenamed>(&mut self, value: T) -> i32 {
    value.fileRenamed(self);
    return 1;
  }
}

pub trait QFileSystemModel_fileRenamed {
  fn fileRenamed(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::fileRenamed(const QString & path, const QString & oldName, const QString & newName);
impl<'a> /*trait*/ QFileSystemModel_fileRenamed for (&'a  QString, &'a  QString, &'a  QString) {
  fn fileRenamed(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel11fileRenamedERK7QStringS2_S2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn isDir<T: QFileSystemModel_isDir>(&mut self, value: T) -> i32 {
    value.isDir(self);
    return 1;
  }
}

pub trait QFileSystemModel_isDir {
  fn isDir(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::isDir(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_isDir for (&'a  QModelIndex) {
  fn isDir(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QFileSystemModel5isDirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK16QFileSystemModel5isDirERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn rmdir<T: QFileSystemModel_rmdir>(&mut self, value: T) -> i32 {
    value.rmdir(self);
    return 1;
  }
}

pub trait QFileSystemModel_rmdir {
  fn rmdir(self, this: &mut QFileSystemModel) -> i32;
}

// proto: bool QFileSystemModel::rmdir(const QModelIndex & index);
impl<'a> /*trait*/ QFileSystemModel_rmdir for (&'a  QModelIndex) {
  fn rmdir(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel5rmdirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QFileSystemModel5rmdirERK11QModelIndex(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileSystemModel {
  pub fn setIconProvider<T: QFileSystemModel_setIconProvider>(&mut self, value: T) -> i32 {
    value.setIconProvider(self);
    return 1;
  }
}

pub trait QFileSystemModel_setIconProvider {
  fn setIconProvider(self, this: &mut QFileSystemModel) -> i32;
}

// proto: void QFileSystemModel::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QFileSystemModel_setIconProvider for (&'a mut QFileIconProvider) {
  fn setIconProvider(self, this: &mut QFileSystemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QFileSystemModel15setIconProviderEP17QFileIconProvider(arg0)};
    return 1;
  }
}

