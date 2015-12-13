// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurl::QUrl;
use super::qstring::QString;
use super::qobject::QObject;
use super::qdir::QDir;
use super::qfileiconprovider::QFileIconProvider;
use super::qstringlist::QStringList;
use super::qbytearray::QByteArray;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: const QMetaObject * QFileDialog::metaObject();
  fn _ZNK11QFileDialog10metaObjectEv() -> i32;
  // proto: void QFileDialog::setDirectoryUrl(const QUrl & directory);
  fn _ZN11QFileDialog15setDirectoryUrlERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::directoryUrlEntered(const QUrl & directory);
  fn _ZN11QFileDialog19directoryUrlEnteredERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::currentChanged(const QString & path);
  fn _ZN11QFileDialog14currentChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: QStringList QFileDialog::nameFilters();
  fn _ZNK11QFileDialog11nameFiltersEv() -> i32;
  // proto: void QFileDialog::setConfirmOverwrite(bool enabled);
  fn _ZN11QFileDialog19setConfirmOverwriteEb(arg0: int8_t) -> i32;
  // proto: void QFileDialog::setDefaultSuffix(const QString & suffix);
  fn _ZN11QFileDialog16setDefaultSuffixERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::filterSelected(const QString & filter);
  fn _ZN11QFileDialog14filterSelectedERK7QString(arg0: *const c_void) -> i32;
  // proto: QList<QUrl> QFileDialog::sidebarUrls();
  fn _ZNK11QFileDialog11sidebarUrlsEv() -> i32;
  // proto: QString QFileDialog::defaultSuffix();
  fn _ZNK11QFileDialog13defaultSuffixEv() -> i32;
  // proto: void QFileDialog::selectFile(const QString & filename);
  fn _ZN11QFileDialog10selectFileERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QFileDialog::resolveSymlinks();
  fn _ZNK11QFileDialog15resolveSymlinksEv() -> i32;
  // proto: void QFileDialog::setDirectory(const QString & directory);
  fn _ZN11QFileDialog12setDirectoryERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::selectUrl(const QUrl & url);
  fn _ZN11QFileDialog9selectUrlERK4QUrl(arg0: *const c_void) -> i32;
  // proto: QString QFileDialog::selectedNameFilter();
  fn _ZNK11QFileDialog18selectedNameFilterEv() -> i32;
  // proto: QUrl QFileDialog::directoryUrl();
  fn _ZNK11QFileDialog12directoryUrlEv() -> i32;
  // proto: bool QFileDialog::isReadOnly();
  fn _ZNK11QFileDialog10isReadOnlyEv() -> i32;
  // proto: void QFileDialog::directoryEntered(const QString & directory);
  fn _ZN11QFileDialog16directoryEnteredERK7QString(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::fileSelected(const QString & file);
  fn _ZN11QFileDialog12fileSelectedERK7QString(arg0: *const c_void) -> i32;
  // proto: QByteArray QFileDialog::saveState();
  fn _ZNK11QFileDialog9saveStateEv() -> i32;
  // proto: void QFileDialog::currentUrlChanged(const QUrl & url);
  fn _ZN11QFileDialog17currentUrlChangedERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::open(QObject * receiver, const char * member);
  fn _ZN11QFileDialog4openEP7QObjectPKc(arg0: *mut c_void, arg1: *const c_char) -> i32;
  // proto: QDir QFileDialog::directory();
  fn _ZNK11QFileDialog9directoryEv() -> i32;
  // proto: void QFileDialog::urlSelected(const QUrl & url);
  fn _ZN11QFileDialog11urlSelectedERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::setDirectory(const QDir & directory);
  fn _ZN11QFileDialog12setDirectoryERK4QDir(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::setVisible(bool visible);
  fn _ZN11QFileDialog10setVisibleEb(arg0: int8_t) -> i32;
  // proto: void QFileDialog::setIconProvider(QFileIconProvider * provider);
  fn _ZN11QFileDialog15setIconProviderEP17QFileIconProvider(arg0: *mut c_void) -> i32;
  // proto: void QFileDialog::selectMimeTypeFilter(const QString & filter);
  fn _ZN11QFileDialog20selectMimeTypeFilterERK7QString(arg0: *const c_void) -> i32;
  // proto: QStringList QFileDialog::mimeTypeFilters();
  fn _ZNK11QFileDialog15mimeTypeFiltersEv() -> i32;
  // proto: void QFileDialog::setMimeTypeFilters(const QStringList & filters);
  fn _ZN11QFileDialog18setMimeTypeFiltersERK11QStringList(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::setResolveSymlinks(bool enabled);
  fn _ZN11QFileDialog18setResolveSymlinksEb(arg0: int8_t) -> i32;
  // proto: void QFileDialog::setReadOnly(bool enabled);
  fn _ZN11QFileDialog11setReadOnlyEb(arg0: int8_t) -> i32;
  // proto: void QFileDialog::filesSelected(const QStringList & files);
  fn _ZN11QFileDialog13filesSelectedERK11QStringList(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::setNameFilterDetailsVisible(bool enabled);
  fn _ZN11QFileDialog27setNameFilterDetailsVisibleEb(arg0: int8_t) -> i32;
  // proto: void QFileDialog::selectNameFilter(const QString & filter);
  fn _ZN11QFileDialog16selectNameFilterERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QFileDialog::restoreState(const QByteArray & state);
  fn _ZN11QFileDialog12restoreStateERK10QByteArray(arg0: *const c_void) -> i32;
  // proto: QFileIconProvider * QFileDialog::iconProvider();
  fn _ZNK11QFileDialog12iconProviderEv() -> i32;
  // proto: QStringList QFileDialog::selectedFiles();
  fn _ZNK11QFileDialog13selectedFilesEv() -> i32;
  // proto: void QFileDialog::FreeQFileDialog();
  fn _ZN11QFileDialogD0Ev() -> i32;
  // proto: QAbstractItemDelegate * QFileDialog::itemDelegate();
  fn _ZNK11QFileDialog12itemDelegateEv() -> i32;
  // proto: bool QFileDialog::confirmOverwrite();
  fn _ZNK11QFileDialog16confirmOverwriteEv() -> i32;
  // proto: void QFileDialog::setHistory(const QStringList & paths);
  fn _ZN11QFileDialog10setHistoryERK11QStringList(arg0: *const c_void) -> i32;
  // proto: void QFileDialog::setNameFilter(const QString & filter);
  fn _ZN11QFileDialog13setNameFilterERK7QString(arg0: *const c_void) -> i32;
  // proto: QAbstractProxyModel * QFileDialog::proxyModel();
  fn _ZNK11QFileDialog10proxyModelEv() -> i32;
  // proto: void QFileDialog::setNameFilters(const QStringList & filters);
  fn _ZN11QFileDialog14setNameFiltersERK11QStringList(arg0: *const c_void) -> i32;
  // proto: QList<QUrl> QFileDialog::selectedUrls();
  fn _ZNK11QFileDialog12selectedUrlsEv() -> i32;
  // proto: QStringList QFileDialog::history();
  fn _ZNK11QFileDialog7historyEv() -> i32;
  // proto: bool QFileDialog::isNameFilterDetailsVisible();
  fn _ZNK11QFileDialog26isNameFilterDetailsVisibleEv() -> i32;
  // proto: void QFileDialog::NewQFileDialog(const QFileDialog & );
  fn _ZN11QFileDialogC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QFileDialog::NewQFileDialog(QWidget * parent, const QString & caption, const QString & directory, const QString & filter);
  fn _ZN11QFileDialogC1EP7QWidgetRK7QStringS4_S4_(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
}

// body block begin
// class sizeof(QFileDialog)=1
pub struct QFileDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileDialog {
  pub fn metaObject<T: QFileDialog_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QFileDialog_metaObject {
  fn metaObject(self, this: &mut QFileDialog) -> i32;
}

// proto: const QMetaObject * QFileDialog::metaObject();
impl<'a> /*trait*/ QFileDialog_metaObject for () {
  fn metaObject(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog10metaObjectEv()};
    unsafe {_ZNK11QFileDialog10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setDirectoryUrl<T: QFileDialog_setDirectoryUrl>(&mut self, value: T) -> i32 {
    value.setDirectoryUrl(self);
    return 1;
  }
}

pub trait QFileDialog_setDirectoryUrl {
  fn setDirectoryUrl(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setDirectoryUrl(const QUrl & directory);
impl<'a> /*trait*/ QFileDialog_setDirectoryUrl for (&'a  QUrl) {
  fn setDirectoryUrl(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog15setDirectoryUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog15setDirectoryUrlERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directoryUrlEntered<T: QFileDialog_directoryUrlEntered>(&mut self, value: T) -> i32 {
    value.directoryUrlEntered(self);
    return 1;
  }
}

pub trait QFileDialog_directoryUrlEntered {
  fn directoryUrlEntered(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::directoryUrlEntered(const QUrl & directory);
impl<'a> /*trait*/ QFileDialog_directoryUrlEntered for (&'a  QUrl) {
  fn directoryUrlEntered(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog19directoryUrlEnteredERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog19directoryUrlEnteredERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn currentChanged<T: QFileDialog_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QFileDialog_currentChanged {
  fn currentChanged(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::currentChanged(const QString & path);
impl<'a> /*trait*/ QFileDialog_currentChanged for (&'a  QString) {
  fn currentChanged(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog14currentChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog14currentChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn nameFilters<T: QFileDialog_nameFilters>(&mut self, value: T) -> i32 {
    value.nameFilters(self);
    return 1;
  }
}

pub trait QFileDialog_nameFilters {
  fn nameFilters(self, this: &mut QFileDialog) -> i32;
}

// proto: QStringList QFileDialog::nameFilters();
impl<'a> /*trait*/ QFileDialog_nameFilters for () {
  fn nameFilters(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog11nameFiltersEv()};
    unsafe {_ZNK11QFileDialog11nameFiltersEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setConfirmOverwrite<T: QFileDialog_setConfirmOverwrite>(&mut self, value: T) -> i32 {
    value.setConfirmOverwrite(self);
    return 1;
  }
}

pub trait QFileDialog_setConfirmOverwrite {
  fn setConfirmOverwrite(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setConfirmOverwrite(bool enabled);
impl<'a> /*trait*/ QFileDialog_setConfirmOverwrite for (i8) {
  fn setConfirmOverwrite(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog19setConfirmOverwriteEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QFileDialog19setConfirmOverwriteEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setDefaultSuffix<T: QFileDialog_setDefaultSuffix>(&mut self, value: T) -> i32 {
    value.setDefaultSuffix(self);
    return 1;
  }
}

pub trait QFileDialog_setDefaultSuffix {
  fn setDefaultSuffix(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setDefaultSuffix(const QString & suffix);
impl<'a> /*trait*/ QFileDialog_setDefaultSuffix for (&'a  QString) {
  fn setDefaultSuffix(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog16setDefaultSuffixERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog16setDefaultSuffixERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn filterSelected<T: QFileDialog_filterSelected>(&mut self, value: T) -> i32 {
    value.filterSelected(self);
    return 1;
  }
}

pub trait QFileDialog_filterSelected {
  fn filterSelected(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::filterSelected(const QString & filter);
impl<'a> /*trait*/ QFileDialog_filterSelected for (&'a  QString) {
  fn filterSelected(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog14filterSelectedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog14filterSelectedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn sidebarUrls<T: QFileDialog_sidebarUrls>(&mut self, value: T) -> i32 {
    value.sidebarUrls(self);
    return 1;
  }
}

pub trait QFileDialog_sidebarUrls {
  fn sidebarUrls(self, this: &mut QFileDialog) -> i32;
}

// proto: QList<QUrl> QFileDialog::sidebarUrls();
impl<'a> /*trait*/ QFileDialog_sidebarUrls for () {
  fn sidebarUrls(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog11sidebarUrlsEv()};
    unsafe {_ZNK11QFileDialog11sidebarUrlsEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn defaultSuffix<T: QFileDialog_defaultSuffix>(&mut self, value: T) -> i32 {
    value.defaultSuffix(self);
    return 1;
  }
}

pub trait QFileDialog_defaultSuffix {
  fn defaultSuffix(self, this: &mut QFileDialog) -> i32;
}

// proto: QString QFileDialog::defaultSuffix();
impl<'a> /*trait*/ QFileDialog_defaultSuffix for () {
  fn defaultSuffix(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog13defaultSuffixEv()};
    unsafe {_ZNK11QFileDialog13defaultSuffixEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectFile<T: QFileDialog_selectFile>(&mut self, value: T) -> i32 {
    value.selectFile(self);
    return 1;
  }
}

pub trait QFileDialog_selectFile {
  fn selectFile(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::selectFile(const QString & filename);
impl<'a> /*trait*/ QFileDialog_selectFile for (&'a  QString) {
  fn selectFile(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog10selectFileERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog10selectFileERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn resolveSymlinks<T: QFileDialog_resolveSymlinks>(&mut self, value: T) -> i32 {
    value.resolveSymlinks(self);
    return 1;
  }
}

pub trait QFileDialog_resolveSymlinks {
  fn resolveSymlinks(self, this: &mut QFileDialog) -> i32;
}

// proto: bool QFileDialog::resolveSymlinks();
impl<'a> /*trait*/ QFileDialog_resolveSymlinks for () {
  fn resolveSymlinks(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog15resolveSymlinksEv()};
    unsafe {_ZNK11QFileDialog15resolveSymlinksEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setDirectory<T: QFileDialog_setDirectory>(&mut self, value: T) -> i32 {
    value.setDirectory(self);
    return 1;
  }
}

pub trait QFileDialog_setDirectory {
  fn setDirectory(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setDirectory(const QString & directory);
impl<'a> /*trait*/ QFileDialog_setDirectory for (&'a  QString) {
  fn setDirectory(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12setDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog12setDirectoryERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectUrl<T: QFileDialog_selectUrl>(&mut self, value: T) -> i32 {
    value.selectUrl(self);
    return 1;
  }
}

pub trait QFileDialog_selectUrl {
  fn selectUrl(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::selectUrl(const QUrl & url);
impl<'a> /*trait*/ QFileDialog_selectUrl for (&'a  QUrl) {
  fn selectUrl(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog9selectUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog9selectUrlERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectedNameFilter<T: QFileDialog_selectedNameFilter>(&mut self, value: T) -> i32 {
    value.selectedNameFilter(self);
    return 1;
  }
}

pub trait QFileDialog_selectedNameFilter {
  fn selectedNameFilter(self, this: &mut QFileDialog) -> i32;
}

// proto: QString QFileDialog::selectedNameFilter();
impl<'a> /*trait*/ QFileDialog_selectedNameFilter for () {
  fn selectedNameFilter(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog18selectedNameFilterEv()};
    unsafe {_ZNK11QFileDialog18selectedNameFilterEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directoryUrl<T: QFileDialog_directoryUrl>(&mut self, value: T) -> i32 {
    value.directoryUrl(self);
    return 1;
  }
}

pub trait QFileDialog_directoryUrl {
  fn directoryUrl(self, this: &mut QFileDialog) -> i32;
}

// proto: QUrl QFileDialog::directoryUrl();
impl<'a> /*trait*/ QFileDialog_directoryUrl for () {
  fn directoryUrl(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12directoryUrlEv()};
    unsafe {_ZNK11QFileDialog12directoryUrlEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn isReadOnly<T: QFileDialog_isReadOnly>(&mut self, value: T) -> i32 {
    value.isReadOnly(self);
    return 1;
  }
}

pub trait QFileDialog_isReadOnly {
  fn isReadOnly(self, this: &mut QFileDialog) -> i32;
}

// proto: bool QFileDialog::isReadOnly();
impl<'a> /*trait*/ QFileDialog_isReadOnly for () {
  fn isReadOnly(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog10isReadOnlyEv()};
    unsafe {_ZNK11QFileDialog10isReadOnlyEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directoryEntered<T: QFileDialog_directoryEntered>(&mut self, value: T) -> i32 {
    value.directoryEntered(self);
    return 1;
  }
}

pub trait QFileDialog_directoryEntered {
  fn directoryEntered(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::directoryEntered(const QString & directory);
impl<'a> /*trait*/ QFileDialog_directoryEntered for (&'a  QString) {
  fn directoryEntered(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog16directoryEnteredERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog16directoryEnteredERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn fileSelected<T: QFileDialog_fileSelected>(&mut self, value: T) -> i32 {
    value.fileSelected(self);
    return 1;
  }
}

pub trait QFileDialog_fileSelected {
  fn fileSelected(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::fileSelected(const QString & file);
impl<'a> /*trait*/ QFileDialog_fileSelected for (&'a  QString) {
  fn fileSelected(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12fileSelectedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog12fileSelectedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn saveState<T: QFileDialog_saveState>(&mut self, value: T) -> i32 {
    value.saveState(self);
    return 1;
  }
}

pub trait QFileDialog_saveState {
  fn saveState(self, this: &mut QFileDialog) -> i32;
}

// proto: QByteArray QFileDialog::saveState();
impl<'a> /*trait*/ QFileDialog_saveState for () {
  fn saveState(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog9saveStateEv()};
    unsafe {_ZNK11QFileDialog9saveStateEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn currentUrlChanged<T: QFileDialog_currentUrlChanged>(&mut self, value: T) -> i32 {
    value.currentUrlChanged(self);
    return 1;
  }
}

pub trait QFileDialog_currentUrlChanged {
  fn currentUrlChanged(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::currentUrlChanged(const QUrl & url);
impl<'a> /*trait*/ QFileDialog_currentUrlChanged for (&'a  QUrl) {
  fn currentUrlChanged(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog17currentUrlChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog17currentUrlChangedERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn open<T: QFileDialog_open>(&mut self, value: T) -> i32 {
    value.open(self);
    return 1;
  }
}

pub trait QFileDialog_open {
  fn open(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QFileDialog_open for (&'a mut QObject, &'a  String) {
  fn open(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    unsafe {_ZN11QFileDialog4openEP7QObjectPKc(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directory<T: QFileDialog_directory>(&mut self, value: T) -> i32 {
    value.directory(self);
    return 1;
  }
}

pub trait QFileDialog_directory {
  fn directory(self, this: &mut QFileDialog) -> i32;
}

// proto: QDir QFileDialog::directory();
impl<'a> /*trait*/ QFileDialog_directory for () {
  fn directory(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog9directoryEv()};
    unsafe {_ZNK11QFileDialog9directoryEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn urlSelected<T: QFileDialog_urlSelected>(&mut self, value: T) -> i32 {
    value.urlSelected(self);
    return 1;
  }
}

pub trait QFileDialog_urlSelected {
  fn urlSelected(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::urlSelected(const QUrl & url);
impl<'a> /*trait*/ QFileDialog_urlSelected for (&'a  QUrl) {
  fn urlSelected(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog11urlSelectedERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog11urlSelectedERK4QUrl(arg0)};
    return 1;
  }
}

// proto: void QFileDialog::setDirectory(const QDir & directory);
impl<'a> /*trait*/ QFileDialog_setDirectory for (&'a  QDir) {
  fn setDirectory(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12setDirectoryERK4QDir()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog12setDirectoryERK4QDir(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setVisible<T: QFileDialog_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QFileDialog_setVisible {
  fn setVisible(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setVisible(bool visible);
impl<'a> /*trait*/ QFileDialog_setVisible for (i8) {
  fn setVisible(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QFileDialog10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setIconProvider<T: QFileDialog_setIconProvider>(&mut self, value: T) -> i32 {
    value.setIconProvider(self);
    return 1;
  }
}

pub trait QFileDialog_setIconProvider {
  fn setIconProvider(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QFileDialog_setIconProvider for (&'a mut QFileIconProvider) {
  fn setIconProvider(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFileDialog15setIconProviderEP17QFileIconProvider(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectMimeTypeFilter<T: QFileDialog_selectMimeTypeFilter>(&mut self, value: T) -> i32 {
    value.selectMimeTypeFilter(self);
    return 1;
  }
}

pub trait QFileDialog_selectMimeTypeFilter {
  fn selectMimeTypeFilter(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::selectMimeTypeFilter(const QString & filter);
impl<'a> /*trait*/ QFileDialog_selectMimeTypeFilter for (&'a  QString) {
  fn selectMimeTypeFilter(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog20selectMimeTypeFilterERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog20selectMimeTypeFilterERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn mimeTypeFilters<T: QFileDialog_mimeTypeFilters>(&mut self, value: T) -> i32 {
    value.mimeTypeFilters(self);
    return 1;
  }
}

pub trait QFileDialog_mimeTypeFilters {
  fn mimeTypeFilters(self, this: &mut QFileDialog) -> i32;
}

// proto: QStringList QFileDialog::mimeTypeFilters();
impl<'a> /*trait*/ QFileDialog_mimeTypeFilters for () {
  fn mimeTypeFilters(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog15mimeTypeFiltersEv()};
    unsafe {_ZNK11QFileDialog15mimeTypeFiltersEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setMimeTypeFilters<T: QFileDialog_setMimeTypeFilters>(&mut self, value: T) -> i32 {
    value.setMimeTypeFilters(self);
    return 1;
  }
}

pub trait QFileDialog_setMimeTypeFilters {
  fn setMimeTypeFilters(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setMimeTypeFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileDialog_setMimeTypeFilters for (&'a  QStringList) {
  fn setMimeTypeFilters(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog18setMimeTypeFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog18setMimeTypeFiltersERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setResolveSymlinks<T: QFileDialog_setResolveSymlinks>(&mut self, value: T) -> i32 {
    value.setResolveSymlinks(self);
    return 1;
  }
}

pub trait QFileDialog_setResolveSymlinks {
  fn setResolveSymlinks(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setResolveSymlinks(bool enabled);
impl<'a> /*trait*/ QFileDialog_setResolveSymlinks for (i8) {
  fn setResolveSymlinks(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog18setResolveSymlinksEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QFileDialog18setResolveSymlinksEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setReadOnly<T: QFileDialog_setReadOnly>(&mut self, value: T) -> i32 {
    value.setReadOnly(self);
    return 1;
  }
}

pub trait QFileDialog_setReadOnly {
  fn setReadOnly(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setReadOnly(bool enabled);
impl<'a> /*trait*/ QFileDialog_setReadOnly for (i8) {
  fn setReadOnly(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog11setReadOnlyEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QFileDialog11setReadOnlyEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn filesSelected<T: QFileDialog_filesSelected>(&mut self, value: T) -> i32 {
    value.filesSelected(self);
    return 1;
  }
}

pub trait QFileDialog_filesSelected {
  fn filesSelected(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::filesSelected(const QStringList & files);
impl<'a> /*trait*/ QFileDialog_filesSelected for (&'a  QStringList) {
  fn filesSelected(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog13filesSelectedERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog13filesSelectedERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setNameFilterDetailsVisible<T: QFileDialog_setNameFilterDetailsVisible>(&mut self, value: T) -> i32 {
    value.setNameFilterDetailsVisible(self);
    return 1;
  }
}

pub trait QFileDialog_setNameFilterDetailsVisible {
  fn setNameFilterDetailsVisible(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setNameFilterDetailsVisible(bool enabled);
impl<'a> /*trait*/ QFileDialog_setNameFilterDetailsVisible for (i8) {
  fn setNameFilterDetailsVisible(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog27setNameFilterDetailsVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QFileDialog27setNameFilterDetailsVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectNameFilter<T: QFileDialog_selectNameFilter>(&mut self, value: T) -> i32 {
    value.selectNameFilter(self);
    return 1;
  }
}

pub trait QFileDialog_selectNameFilter {
  fn selectNameFilter(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::selectNameFilter(const QString & filter);
impl<'a> /*trait*/ QFileDialog_selectNameFilter for (&'a  QString) {
  fn selectNameFilter(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog16selectNameFilterERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog16selectNameFilterERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn restoreState<T: QFileDialog_restoreState>(&mut self, value: T) -> i32 {
    value.restoreState(self);
    return 1;
  }
}

pub trait QFileDialog_restoreState {
  fn restoreState(self, this: &mut QFileDialog) -> i32;
}

// proto: bool QFileDialog::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QFileDialog_restoreState for (&'a  QByteArray) {
  fn restoreState(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog12restoreStateERK10QByteArray(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn iconProvider<T: QFileDialog_iconProvider>(&mut self, value: T) -> i32 {
    value.iconProvider(self);
    return 1;
  }
}

pub trait QFileDialog_iconProvider {
  fn iconProvider(self, this: &mut QFileDialog) -> i32;
}

// proto: QFileIconProvider * QFileDialog::iconProvider();
impl<'a> /*trait*/ QFileDialog_iconProvider for () {
  fn iconProvider(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12iconProviderEv()};
    unsafe {_ZNK11QFileDialog12iconProviderEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectedFiles<T: QFileDialog_selectedFiles>(&mut self, value: T) -> i32 {
    value.selectedFiles(self);
    return 1;
  }
}

pub trait QFileDialog_selectedFiles {
  fn selectedFiles(self, this: &mut QFileDialog) -> i32;
}

// proto: QStringList QFileDialog::selectedFiles();
impl<'a> /*trait*/ QFileDialog_selectedFiles for () {
  fn selectedFiles(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog13selectedFilesEv()};
    unsafe {_ZNK11QFileDialog13selectedFilesEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn FreeQFileDialog<T: QFileDialog_FreeQFileDialog>(&mut self, value: T) -> i32 {
    value.FreeQFileDialog(self);
    return 1;
  }
}

pub trait QFileDialog_FreeQFileDialog {
  fn FreeQFileDialog(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::FreeQFileDialog();
impl<'a> /*trait*/ QFileDialog_FreeQFileDialog for () {
  fn FreeQFileDialog(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialogD0Ev()};
    unsafe {_ZN11QFileDialogD0Ev()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn itemDelegate<T: QFileDialog_itemDelegate>(&mut self, value: T) -> i32 {
    value.itemDelegate(self);
    return 1;
  }
}

pub trait QFileDialog_itemDelegate {
  fn itemDelegate(self, this: &mut QFileDialog) -> i32;
}

// proto: QAbstractItemDelegate * QFileDialog::itemDelegate();
impl<'a> /*trait*/ QFileDialog_itemDelegate for () {
  fn itemDelegate(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12itemDelegateEv()};
    unsafe {_ZNK11QFileDialog12itemDelegateEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn confirmOverwrite<T: QFileDialog_confirmOverwrite>(&mut self, value: T) -> i32 {
    value.confirmOverwrite(self);
    return 1;
  }
}

pub trait QFileDialog_confirmOverwrite {
  fn confirmOverwrite(self, this: &mut QFileDialog) -> i32;
}

// proto: bool QFileDialog::confirmOverwrite();
impl<'a> /*trait*/ QFileDialog_confirmOverwrite for () {
  fn confirmOverwrite(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog16confirmOverwriteEv()};
    unsafe {_ZNK11QFileDialog16confirmOverwriteEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setHistory<T: QFileDialog_setHistory>(&mut self, value: T) -> i32 {
    value.setHistory(self);
    return 1;
  }
}

pub trait QFileDialog_setHistory {
  fn setHistory(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setHistory(const QStringList & paths);
impl<'a> /*trait*/ QFileDialog_setHistory for (&'a  QStringList) {
  fn setHistory(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog10setHistoryERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog10setHistoryERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setNameFilter<T: QFileDialog_setNameFilter>(&mut self, value: T) -> i32 {
    value.setNameFilter(self);
    return 1;
  }
}

pub trait QFileDialog_setNameFilter {
  fn setNameFilter(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setNameFilter(const QString & filter);
impl<'a> /*trait*/ QFileDialog_setNameFilter for (&'a  QString) {
  fn setNameFilter(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog13setNameFilterERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog13setNameFilterERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn proxyModel<T: QFileDialog_proxyModel>(&mut self, value: T) -> i32 {
    value.proxyModel(self);
    return 1;
  }
}

pub trait QFileDialog_proxyModel {
  fn proxyModel(self, this: &mut QFileDialog) -> i32;
}

// proto: QAbstractProxyModel * QFileDialog::proxyModel();
impl<'a> /*trait*/ QFileDialog_proxyModel for () {
  fn proxyModel(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog10proxyModelEv()};
    unsafe {_ZNK11QFileDialog10proxyModelEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setNameFilters<T: QFileDialog_setNameFilters>(&mut self, value: T) -> i32 {
    value.setNameFilters(self);
    return 1;
  }
}

pub trait QFileDialog_setNameFilters {
  fn setNameFilters(self, this: &mut QFileDialog) -> i32;
}

// proto: void QFileDialog::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileDialog_setNameFilters for (&'a  QStringList) {
  fn setNameFilters(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialog14setNameFiltersERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectedUrls<T: QFileDialog_selectedUrls>(&mut self, value: T) -> i32 {
    value.selectedUrls(self);
    return 1;
  }
}

pub trait QFileDialog_selectedUrls {
  fn selectedUrls(self, this: &mut QFileDialog) -> i32;
}

// proto: QList<QUrl> QFileDialog::selectedUrls();
impl<'a> /*trait*/ QFileDialog_selectedUrls for () {
  fn selectedUrls(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12selectedUrlsEv()};
    unsafe {_ZNK11QFileDialog12selectedUrlsEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn history<T: QFileDialog_history>(&mut self, value: T) -> i32 {
    value.history(self);
    return 1;
  }
}

pub trait QFileDialog_history {
  fn history(self, this: &mut QFileDialog) -> i32;
}

// proto: QStringList QFileDialog::history();
impl<'a> /*trait*/ QFileDialog_history for () {
  fn history(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog7historyEv()};
    unsafe {_ZNK11QFileDialog7historyEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn isNameFilterDetailsVisible<T: QFileDialog_isNameFilterDetailsVisible>(&mut self, value: T) -> i32 {
    value.isNameFilterDetailsVisible(self);
    return 1;
  }
}

pub trait QFileDialog_isNameFilterDetailsVisible {
  fn isNameFilterDetailsVisible(self, this: &mut QFileDialog) -> i32;
}

// proto: bool QFileDialog::isNameFilterDetailsVisible();
impl<'a> /*trait*/ QFileDialog_isNameFilterDetailsVisible for () {
  fn isNameFilterDetailsVisible(self, this: &mut QFileDialog) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog26isNameFilterDetailsVisibleEv()};
    unsafe {_ZNK11QFileDialog26isNameFilterDetailsVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn NewQFileDialog<T: QFileDialog_NewQFileDialog>(value: T) -> QFileDialog {
    let rsthis = value.NewQFileDialog();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDialog_NewQFileDialog {
  fn NewQFileDialog(self) -> QFileDialog;
}

// proto: void QFileDialog::NewQFileDialog(const QFileDialog & );
impl<'a> /*trait*/ QFileDialog_NewQFileDialog for (&'a  QFileDialog) {
  fn NewQFileDialog(self) -> QFileDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialogC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialogC1ERKS_(qthis, arg0)};
    let rsthis = QFileDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QFileDialog::NewQFileDialog(QWidget * parent, const QString & caption, const QString & directory, const QString & filter);
impl<'a> /*trait*/ QFileDialog_NewQFileDialog for (&'a mut QWidget, &'a  QString, &'a  QString, &'a  QString) {
  fn NewQFileDialog(self) -> QFileDialog {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialogC1EP7QWidgetRK7QStringS4_S4_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN11QFileDialogC1EP7QWidgetRK7QStringS4_S4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QFileDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

