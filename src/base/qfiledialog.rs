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
use super::qbytearray::QByteArray;
use super::qobject::QObject;
use super::qdir::QDir;
use super::qfileiconprovider::QFileIconProvider;
use super::qstringlist::QStringList;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QMetaObject * QFileDialog::metaObject();
  fn _ZNK11QFileDialog10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QFileDialog::setDirectoryUrl(const QUrl & directory);
  fn _ZN11QFileDialog15setDirectoryUrlERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::directoryUrlEntered(const QUrl & directory);
  fn _ZN11QFileDialog19directoryUrlEnteredERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::currentChanged(const QString & path);
  fn _ZN11QFileDialog14currentChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QFileDialog::nameFilters();
  fn _ZNK11QFileDialog11nameFiltersEv(qthis: *mut c_void) ;
  // proto:  void QFileDialog::setConfirmOverwrite(bool enabled);
  fn _ZN11QFileDialog19setConfirmOverwriteEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFileDialog::setDefaultSuffix(const QString & suffix);
  fn _ZN11QFileDialog16setDefaultSuffixERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::filterSelected(const QString & filter);
  fn _ZN11QFileDialog14filterSelectedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QUrl> QFileDialog::sidebarUrls();
  fn _ZNK11QFileDialog11sidebarUrlsEv(qthis: *mut c_void) ;
  // proto:  QString QFileDialog::defaultSuffix();
  fn _ZNK11QFileDialog13defaultSuffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileDialog::selectFile(const QString & filename);
  fn _ZN11QFileDialog10selectFileERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileDialog::resolveSymlinks();
  fn _ZNK11QFileDialog15resolveSymlinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileDialog::setDirectory(const QString & directory);
  fn _ZN11QFileDialog12setDirectoryERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::selectUrl(const QUrl & url);
  fn _ZN11QFileDialog9selectUrlERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFileDialog::selectedNameFilter();
  fn _ZNK11QFileDialog18selectedNameFilterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUrl QFileDialog::directoryUrl();
  fn _ZNK11QFileDialog12directoryUrlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileDialog::isReadOnly();
  fn _ZNK11QFileDialog10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileDialog::directoryEntered(const QString & directory);
  fn _ZN11QFileDialog16directoryEnteredERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::fileSelected(const QString & file);
  fn _ZN11QFileDialog12fileSelectedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QFileDialog::saveState();
  fn _ZNK11QFileDialog9saveStateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileDialog::currentUrlChanged(const QUrl & url);
  fn _ZN11QFileDialog17currentUrlChangedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::open(QObject * receiver, const char * member);
  fn _ZN11QFileDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char) ;
  // proto:  QDir QFileDialog::directory();
  fn _ZNK11QFileDialog9directoryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileDialog::urlSelected(const QUrl & url);
  fn _ZN11QFileDialog11urlSelectedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::setDirectory(const QDir & directory);
  fn _ZN11QFileDialog12setDirectoryERK4QDir(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::setVisible(bool visible);
  fn _ZN11QFileDialog10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFileDialog::setIconProvider(QFileIconProvider * provider);
  fn _ZN11QFileDialog15setIconProviderEP17QFileIconProvider(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::selectMimeTypeFilter(const QString & filter);
  fn _ZN11QFileDialog20selectMimeTypeFilterERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QFileDialog::mimeTypeFilters();
  fn _ZNK11QFileDialog15mimeTypeFiltersEv(qthis: *mut c_void) ;
  // proto:  void QFileDialog::setMimeTypeFilters(const QStringList & filters);
  fn _ZN11QFileDialog18setMimeTypeFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::setResolveSymlinks(bool enabled);
  fn _ZN11QFileDialog18setResolveSymlinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFileDialog::setReadOnly(bool enabled);
  fn _ZN11QFileDialog11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFileDialog::filesSelected(const QStringList & files);
  fn _ZN11QFileDialog13filesSelectedERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::setNameFilterDetailsVisible(bool enabled);
  fn _ZN11QFileDialog27setNameFilterDetailsVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QFileDialog::selectNameFilter(const QString & filter);
  fn _ZN11QFileDialog16selectNameFilterERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileDialog::restoreState(const QByteArray & state);
  fn _ZN11QFileDialog12restoreStateERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QFileIconProvider * QFileDialog::iconProvider();
  fn _ZNK11QFileDialog12iconProviderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QStringList QFileDialog::selectedFiles();
  fn _ZNK11QFileDialog13selectedFilesEv(qthis: *mut c_void) ;
  // proto:  void QFileDialog::FreeQFileDialog();
  fn _ZN11QFileDialogD0Ev(qthis: *mut c_void) ;
  // proto:  QAbstractItemDelegate * QFileDialog::itemDelegate();
  fn _ZNK11QFileDialog12itemDelegateEv(qthis: *mut c_void) ;
  // proto:  bool QFileDialog::confirmOverwrite();
  fn _ZNK11QFileDialog16confirmOverwriteEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileDialog::setHistory(const QStringList & paths);
  fn _ZN11QFileDialog10setHistoryERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::setNameFilter(const QString & filter);
  fn _ZN11QFileDialog13setNameFilterERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QAbstractProxyModel * QFileDialog::proxyModel();
  fn _ZNK11QFileDialog10proxyModelEv(qthis: *mut c_void) ;
  // proto:  void QFileDialog::setNameFilters(const QStringList & filters);
  fn _ZN11QFileDialog14setNameFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QUrl> QFileDialog::selectedUrls();
  fn _ZNK11QFileDialog12selectedUrlsEv(qthis: *mut c_void) ;
  // proto:  QStringList QFileDialog::history();
  fn _ZNK11QFileDialog7historyEv(qthis: *mut c_void) ;
  // proto:  bool QFileDialog::isNameFilterDetailsVisible();
  fn _ZNK11QFileDialog26isNameFilterDetailsVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileDialog::NewQFileDialog(const QFileDialog & );
  fn _ZN11QFileDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDialog::NewQFileDialog(QWidget * parent, const QString & caption, const QString & directory, const QString & filter);
  fn _ZN11QFileDialogC1EP7QWidgetRK7QStringS4_S4_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) ;
}

// body block begin
// class sizeof(QFileDialog)=1
pub struct QFileDialog {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileDialog {
  pub fn metaObject<RetType, T: QFileDialog_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QFileDialog_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  const QMetaObject * QFileDialog::metaObject();
impl<'a> /*trait*/ QFileDialog_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog10metaObjectEv()};
     unsafe {_ZNK11QFileDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setDirectoryUrl<RetType, T: QFileDialog_setDirectoryUrl<RetType>>(&mut self, value: T) -> RetType {
    return value.setDirectoryUrl(self);
    // return 1;
  }
}

pub trait QFileDialog_setDirectoryUrl<RetType> {
  fn setDirectoryUrl(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setDirectoryUrl(const QUrl & directory);
impl<'a> /*trait*/ QFileDialog_setDirectoryUrl<()> for (&'a  QUrl) {
  fn setDirectoryUrl(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog15setDirectoryUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog15setDirectoryUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directoryUrlEntered<RetType, T: QFileDialog_directoryUrlEntered<RetType>>(&mut self, value: T) -> RetType {
    return value.directoryUrlEntered(self);
    // return 1;
  }
}

pub trait QFileDialog_directoryUrlEntered<RetType> {
  fn directoryUrlEntered(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::directoryUrlEntered(const QUrl & directory);
impl<'a> /*trait*/ QFileDialog_directoryUrlEntered<()> for (&'a  QUrl) {
  fn directoryUrlEntered(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog19directoryUrlEnteredERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog19directoryUrlEnteredERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn currentChanged<RetType, T: QFileDialog_currentChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentChanged(self);
    // return 1;
  }
}

pub trait QFileDialog_currentChanged<RetType> {
  fn currentChanged(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::currentChanged(const QString & path);
impl<'a> /*trait*/ QFileDialog_currentChanged<()> for (&'a  QString) {
  fn currentChanged(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog14currentChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog14currentChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn nameFilters<RetType, T: QFileDialog_nameFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.nameFilters(self);
    // return 1;
  }
}

pub trait QFileDialog_nameFilters<RetType> {
  fn nameFilters(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QStringList QFileDialog::nameFilters();
impl<'a> /*trait*/ QFileDialog_nameFilters<()> for () {
  fn nameFilters(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog11nameFiltersEv()};
     unsafe {_ZNK11QFileDialog11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setConfirmOverwrite<RetType, T: QFileDialog_setConfirmOverwrite<RetType>>(&mut self, value: T) -> RetType {
    return value.setConfirmOverwrite(self);
    // return 1;
  }
}

pub trait QFileDialog_setConfirmOverwrite<RetType> {
  fn setConfirmOverwrite(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setConfirmOverwrite(bool enabled);
impl<'a> /*trait*/ QFileDialog_setConfirmOverwrite<()> for (i8) {
  fn setConfirmOverwrite(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog19setConfirmOverwriteEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFileDialog19setConfirmOverwriteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setDefaultSuffix<RetType, T: QFileDialog_setDefaultSuffix<RetType>>(&mut self, value: T) -> RetType {
    return value.setDefaultSuffix(self);
    // return 1;
  }
}

pub trait QFileDialog_setDefaultSuffix<RetType> {
  fn setDefaultSuffix(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setDefaultSuffix(const QString & suffix);
impl<'a> /*trait*/ QFileDialog_setDefaultSuffix<()> for (&'a  QString) {
  fn setDefaultSuffix(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog16setDefaultSuffixERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog16setDefaultSuffixERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn filterSelected<RetType, T: QFileDialog_filterSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.filterSelected(self);
    // return 1;
  }
}

pub trait QFileDialog_filterSelected<RetType> {
  fn filterSelected(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::filterSelected(const QString & filter);
impl<'a> /*trait*/ QFileDialog_filterSelected<()> for (&'a  QString) {
  fn filterSelected(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog14filterSelectedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog14filterSelectedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn sidebarUrls<RetType, T: QFileDialog_sidebarUrls<RetType>>(&mut self, value: T) -> RetType {
    return value.sidebarUrls(self);
    // return 1;
  }
}

pub trait QFileDialog_sidebarUrls<RetType> {
  fn sidebarUrls(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QList<QUrl> QFileDialog::sidebarUrls();
impl<'a> /*trait*/ QFileDialog_sidebarUrls<()> for () {
  fn sidebarUrls(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog11sidebarUrlsEv()};
     unsafe {_ZNK11QFileDialog11sidebarUrlsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn defaultSuffix<RetType, T: QFileDialog_defaultSuffix<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultSuffix(self);
    // return 1;
  }
}

pub trait QFileDialog_defaultSuffix<RetType> {
  fn defaultSuffix(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QString QFileDialog::defaultSuffix();
impl<'a> /*trait*/ QFileDialog_defaultSuffix<QString> for () {
  fn defaultSuffix(self, rsthis: &mut QFileDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog13defaultSuffixEv()};
    let mut ret = unsafe {_ZNK11QFileDialog13defaultSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectFile<RetType, T: QFileDialog_selectFile<RetType>>(&mut self, value: T) -> RetType {
    return value.selectFile(self);
    // return 1;
  }
}

pub trait QFileDialog_selectFile<RetType> {
  fn selectFile(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::selectFile(const QString & filename);
impl<'a> /*trait*/ QFileDialog_selectFile<()> for (&'a  QString) {
  fn selectFile(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog10selectFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog10selectFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn resolveSymlinks<RetType, T: QFileDialog_resolveSymlinks<RetType>>(&mut self, value: T) -> RetType {
    return value.resolveSymlinks(self);
    // return 1;
  }
}

pub trait QFileDialog_resolveSymlinks<RetType> {
  fn resolveSymlinks(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  bool QFileDialog::resolveSymlinks();
impl<'a> /*trait*/ QFileDialog_resolveSymlinks<i8> for () {
  fn resolveSymlinks(self, rsthis: &mut QFileDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog15resolveSymlinksEv()};
    let mut ret = unsafe {_ZNK11QFileDialog15resolveSymlinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setDirectory<RetType, T: QFileDialog_setDirectory<RetType>>(&mut self, value: T) -> RetType {
    return value.setDirectory(self);
    // return 1;
  }
}

pub trait QFileDialog_setDirectory<RetType> {
  fn setDirectory(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setDirectory(const QString & directory);
impl<'a> /*trait*/ QFileDialog_setDirectory<()> for (&'a  QString) {
  fn setDirectory(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12setDirectoryERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog12setDirectoryERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectUrl<RetType, T: QFileDialog_selectUrl<RetType>>(&mut self, value: T) -> RetType {
    return value.selectUrl(self);
    // return 1;
  }
}

pub trait QFileDialog_selectUrl<RetType> {
  fn selectUrl(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::selectUrl(const QUrl & url);
impl<'a> /*trait*/ QFileDialog_selectUrl<()> for (&'a  QUrl) {
  fn selectUrl(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog9selectUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog9selectUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectedNameFilter<RetType, T: QFileDialog_selectedNameFilter<RetType>>(&mut self, value: T) -> RetType {
    return value.selectedNameFilter(self);
    // return 1;
  }
}

pub trait QFileDialog_selectedNameFilter<RetType> {
  fn selectedNameFilter(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QString QFileDialog::selectedNameFilter();
impl<'a> /*trait*/ QFileDialog_selectedNameFilter<QString> for () {
  fn selectedNameFilter(self, rsthis: &mut QFileDialog) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog18selectedNameFilterEv()};
    let mut ret = unsafe {_ZNK11QFileDialog18selectedNameFilterEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directoryUrl<RetType, T: QFileDialog_directoryUrl<RetType>>(&mut self, value: T) -> RetType {
    return value.directoryUrl(self);
    // return 1;
  }
}

pub trait QFileDialog_directoryUrl<RetType> {
  fn directoryUrl(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QUrl QFileDialog::directoryUrl();
impl<'a> /*trait*/ QFileDialog_directoryUrl<QUrl> for () {
  fn directoryUrl(self, rsthis: &mut QFileDialog) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12directoryUrlEv()};
    let mut ret = unsafe {_ZNK11QFileDialog12directoryUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn isReadOnly<RetType, T: QFileDialog_isReadOnly<RetType>>(&mut self, value: T) -> RetType {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QFileDialog_isReadOnly<RetType> {
  fn isReadOnly(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  bool QFileDialog::isReadOnly();
impl<'a> /*trait*/ QFileDialog_isReadOnly<i8> for () {
  fn isReadOnly(self, rsthis: &mut QFileDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK11QFileDialog10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directoryEntered<RetType, T: QFileDialog_directoryEntered<RetType>>(&mut self, value: T) -> RetType {
    return value.directoryEntered(self);
    // return 1;
  }
}

pub trait QFileDialog_directoryEntered<RetType> {
  fn directoryEntered(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::directoryEntered(const QString & directory);
impl<'a> /*trait*/ QFileDialog_directoryEntered<()> for (&'a  QString) {
  fn directoryEntered(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog16directoryEnteredERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog16directoryEnteredERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn fileSelected<RetType, T: QFileDialog_fileSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.fileSelected(self);
    // return 1;
  }
}

pub trait QFileDialog_fileSelected<RetType> {
  fn fileSelected(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::fileSelected(const QString & file);
impl<'a> /*trait*/ QFileDialog_fileSelected<()> for (&'a  QString) {
  fn fileSelected(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12fileSelectedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog12fileSelectedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn saveState<RetType, T: QFileDialog_saveState<RetType>>(&mut self, value: T) -> RetType {
    return value.saveState(self);
    // return 1;
  }
}

pub trait QFileDialog_saveState<RetType> {
  fn saveState(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QByteArray QFileDialog::saveState();
impl<'a> /*trait*/ QFileDialog_saveState<QByteArray> for () {
  fn saveState(self, rsthis: &mut QFileDialog) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog9saveStateEv()};
    let mut ret = unsafe {_ZNK11QFileDialog9saveStateEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn currentUrlChanged<RetType, T: QFileDialog_currentUrlChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentUrlChanged(self);
    // return 1;
  }
}

pub trait QFileDialog_currentUrlChanged<RetType> {
  fn currentUrlChanged(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::currentUrlChanged(const QUrl & url);
impl<'a> /*trait*/ QFileDialog_currentUrlChanged<()> for (&'a  QUrl) {
  fn currentUrlChanged(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog17currentUrlChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog17currentUrlChangedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn open<RetType, T: QFileDialog_open<RetType>>(&mut self, value: T) -> RetType {
    return value.open(self);
    // return 1;
  }
}

pub trait QFileDialog_open<RetType> {
  fn open(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QFileDialog_open<()> for (&'a mut QObject, &'a  String) {
  fn open(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
     unsafe {_ZN11QFileDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn directory<RetType, T: QFileDialog_directory<RetType>>(&mut self, value: T) -> RetType {
    return value.directory(self);
    // return 1;
  }
}

pub trait QFileDialog_directory<RetType> {
  fn directory(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QDir QFileDialog::directory();
impl<'a> /*trait*/ QFileDialog_directory<QDir> for () {
  fn directory(self, rsthis: &mut QFileDialog) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog9directoryEv()};
    let mut ret = unsafe {_ZNK11QFileDialog9directoryEv(rsthis.qclsinst)};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn urlSelected<RetType, T: QFileDialog_urlSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.urlSelected(self);
    // return 1;
  }
}

pub trait QFileDialog_urlSelected<RetType> {
  fn urlSelected(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::urlSelected(const QUrl & url);
impl<'a> /*trait*/ QFileDialog_urlSelected<()> for (&'a  QUrl) {
  fn urlSelected(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog11urlSelectedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog11urlSelectedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QFileDialog::setDirectory(const QDir & directory);
impl<'a> /*trait*/ QFileDialog_setDirectory<()> for (&'a  QDir) {
  fn setDirectory(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12setDirectoryERK4QDir()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog12setDirectoryERK4QDir(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setVisible<RetType, T: QFileDialog_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QFileDialog_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setVisible(bool visible);
impl<'a> /*trait*/ QFileDialog_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFileDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setIconProvider<RetType, T: QFileDialog_setIconProvider<RetType>>(&mut self, value: T) -> RetType {
    return value.setIconProvider(self);
    // return 1;
  }
}

pub trait QFileDialog_setIconProvider<RetType> {
  fn setIconProvider(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QFileDialog_setIconProvider<()> for (&'a mut QFileIconProvider) {
  fn setIconProvider(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog15setIconProviderEP17QFileIconProvider(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectMimeTypeFilter<RetType, T: QFileDialog_selectMimeTypeFilter<RetType>>(&mut self, value: T) -> RetType {
    return value.selectMimeTypeFilter(self);
    // return 1;
  }
}

pub trait QFileDialog_selectMimeTypeFilter<RetType> {
  fn selectMimeTypeFilter(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::selectMimeTypeFilter(const QString & filter);
impl<'a> /*trait*/ QFileDialog_selectMimeTypeFilter<()> for (&'a  QString) {
  fn selectMimeTypeFilter(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog20selectMimeTypeFilterERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog20selectMimeTypeFilterERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn mimeTypeFilters<RetType, T: QFileDialog_mimeTypeFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.mimeTypeFilters(self);
    // return 1;
  }
}

pub trait QFileDialog_mimeTypeFilters<RetType> {
  fn mimeTypeFilters(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QStringList QFileDialog::mimeTypeFilters();
impl<'a> /*trait*/ QFileDialog_mimeTypeFilters<()> for () {
  fn mimeTypeFilters(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog15mimeTypeFiltersEv()};
     unsafe {_ZNK11QFileDialog15mimeTypeFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setMimeTypeFilters<RetType, T: QFileDialog_setMimeTypeFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.setMimeTypeFilters(self);
    // return 1;
  }
}

pub trait QFileDialog_setMimeTypeFilters<RetType> {
  fn setMimeTypeFilters(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setMimeTypeFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileDialog_setMimeTypeFilters<()> for (&'a  QStringList) {
  fn setMimeTypeFilters(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog18setMimeTypeFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog18setMimeTypeFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setResolveSymlinks<RetType, T: QFileDialog_setResolveSymlinks<RetType>>(&mut self, value: T) -> RetType {
    return value.setResolveSymlinks(self);
    // return 1;
  }
}

pub trait QFileDialog_setResolveSymlinks<RetType> {
  fn setResolveSymlinks(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setResolveSymlinks(bool enabled);
impl<'a> /*trait*/ QFileDialog_setResolveSymlinks<()> for (i8) {
  fn setResolveSymlinks(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog18setResolveSymlinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFileDialog18setResolveSymlinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setReadOnly<RetType, T: QFileDialog_setReadOnly<RetType>>(&mut self, value: T) -> RetType {
    return value.setReadOnly(self);
    // return 1;
  }
}

pub trait QFileDialog_setReadOnly<RetType> {
  fn setReadOnly(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setReadOnly(bool enabled);
impl<'a> /*trait*/ QFileDialog_setReadOnly<()> for (i8) {
  fn setReadOnly(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFileDialog11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn filesSelected<RetType, T: QFileDialog_filesSelected<RetType>>(&mut self, value: T) -> RetType {
    return value.filesSelected(self);
    // return 1;
  }
}

pub trait QFileDialog_filesSelected<RetType> {
  fn filesSelected(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::filesSelected(const QStringList & files);
impl<'a> /*trait*/ QFileDialog_filesSelected<()> for (&'a  QStringList) {
  fn filesSelected(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog13filesSelectedERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog13filesSelectedERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setNameFilterDetailsVisible<RetType, T: QFileDialog_setNameFilterDetailsVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setNameFilterDetailsVisible(self);
    // return 1;
  }
}

pub trait QFileDialog_setNameFilterDetailsVisible<RetType> {
  fn setNameFilterDetailsVisible(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setNameFilterDetailsVisible(bool enabled);
impl<'a> /*trait*/ QFileDialog_setNameFilterDetailsVisible<()> for (i8) {
  fn setNameFilterDetailsVisible(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog27setNameFilterDetailsVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QFileDialog27setNameFilterDetailsVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectNameFilter<RetType, T: QFileDialog_selectNameFilter<RetType>>(&mut self, value: T) -> RetType {
    return value.selectNameFilter(self);
    // return 1;
  }
}

pub trait QFileDialog_selectNameFilter<RetType> {
  fn selectNameFilter(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::selectNameFilter(const QString & filter);
impl<'a> /*trait*/ QFileDialog_selectNameFilter<()> for (&'a  QString) {
  fn selectNameFilter(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog16selectNameFilterERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog16selectNameFilterERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn restoreState<RetType, T: QFileDialog_restoreState<RetType>>(&mut self, value: T) -> RetType {
    return value.restoreState(self);
    // return 1;
  }
}

pub trait QFileDialog_restoreState<RetType> {
  fn restoreState(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  bool QFileDialog::restoreState(const QByteArray & state);
impl<'a> /*trait*/ QFileDialog_restoreState<i8> for (&'a  QByteArray) {
  fn restoreState(self, rsthis: &mut QFileDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog12restoreStateERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QFileDialog12restoreStateERK10QByteArray(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn iconProvider<RetType, T: QFileDialog_iconProvider<RetType>>(&mut self, value: T) -> RetType {
    return value.iconProvider(self);
    // return 1;
  }
}

pub trait QFileDialog_iconProvider<RetType> {
  fn iconProvider(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QFileIconProvider * QFileDialog::iconProvider();
impl<'a> /*trait*/ QFileDialog_iconProvider<QFileIconProvider> for () {
  fn iconProvider(self, rsthis: &mut QFileDialog) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12iconProviderEv()};
    let mut ret = unsafe {_ZNK11QFileDialog12iconProviderEv(rsthis.qclsinst)};
    let mut ret1 = QFileIconProvider{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectedFiles<RetType, T: QFileDialog_selectedFiles<RetType>>(&mut self, value: T) -> RetType {
    return value.selectedFiles(self);
    // return 1;
  }
}

pub trait QFileDialog_selectedFiles<RetType> {
  fn selectedFiles(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QStringList QFileDialog::selectedFiles();
impl<'a> /*trait*/ QFileDialog_selectedFiles<()> for () {
  fn selectedFiles(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog13selectedFilesEv()};
     unsafe {_ZNK11QFileDialog13selectedFilesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn FreeQFileDialog<RetType, T: QFileDialog_FreeQFileDialog<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFileDialog(self);
    // return 1;
  }
}

pub trait QFileDialog_FreeQFileDialog<RetType> {
  fn FreeQFileDialog(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::FreeQFileDialog();
impl<'a> /*trait*/ QFileDialog_FreeQFileDialog<()> for () {
  fn FreeQFileDialog(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialogD0Ev()};
     unsafe {_ZN11QFileDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn itemDelegate<RetType, T: QFileDialog_itemDelegate<RetType>>(&mut self, value: T) -> RetType {
    return value.itemDelegate(self);
    // return 1;
  }
}

pub trait QFileDialog_itemDelegate<RetType> {
  fn itemDelegate(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QAbstractItemDelegate * QFileDialog::itemDelegate();
impl<'a> /*trait*/ QFileDialog_itemDelegate<()> for () {
  fn itemDelegate(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12itemDelegateEv()};
     unsafe {_ZNK11QFileDialog12itemDelegateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn confirmOverwrite<RetType, T: QFileDialog_confirmOverwrite<RetType>>(&mut self, value: T) -> RetType {
    return value.confirmOverwrite(self);
    // return 1;
  }
}

pub trait QFileDialog_confirmOverwrite<RetType> {
  fn confirmOverwrite(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  bool QFileDialog::confirmOverwrite();
impl<'a> /*trait*/ QFileDialog_confirmOverwrite<i8> for () {
  fn confirmOverwrite(self, rsthis: &mut QFileDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog16confirmOverwriteEv()};
    let mut ret = unsafe {_ZNK11QFileDialog16confirmOverwriteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setHistory<RetType, T: QFileDialog_setHistory<RetType>>(&mut self, value: T) -> RetType {
    return value.setHistory(self);
    // return 1;
  }
}

pub trait QFileDialog_setHistory<RetType> {
  fn setHistory(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setHistory(const QStringList & paths);
impl<'a> /*trait*/ QFileDialog_setHistory<()> for (&'a  QStringList) {
  fn setHistory(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog10setHistoryERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog10setHistoryERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setNameFilter<RetType, T: QFileDialog_setNameFilter<RetType>>(&mut self, value: T) -> RetType {
    return value.setNameFilter(self);
    // return 1;
  }
}

pub trait QFileDialog_setNameFilter<RetType> {
  fn setNameFilter(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setNameFilter(const QString & filter);
impl<'a> /*trait*/ QFileDialog_setNameFilter<()> for (&'a  QString) {
  fn setNameFilter(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog13setNameFilterERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog13setNameFilterERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn proxyModel<RetType, T: QFileDialog_proxyModel<RetType>>(&mut self, value: T) -> RetType {
    return value.proxyModel(self);
    // return 1;
  }
}

pub trait QFileDialog_proxyModel<RetType> {
  fn proxyModel(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QAbstractProxyModel * QFileDialog::proxyModel();
impl<'a> /*trait*/ QFileDialog_proxyModel<()> for () {
  fn proxyModel(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog10proxyModelEv()};
     unsafe {_ZNK11QFileDialog10proxyModelEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn setNameFilters<RetType, T: QFileDialog_setNameFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.setNameFilters(self);
    // return 1;
  }
}

pub trait QFileDialog_setNameFilters<RetType> {
  fn setNameFilters(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  void QFileDialog::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QFileDialog_setNameFilters<()> for (&'a  QStringList) {
  fn setNameFilters(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDialog14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFileDialog14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn selectedUrls<RetType, T: QFileDialog_selectedUrls<RetType>>(&mut self, value: T) -> RetType {
    return value.selectedUrls(self);
    // return 1;
  }
}

pub trait QFileDialog_selectedUrls<RetType> {
  fn selectedUrls(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QList<QUrl> QFileDialog::selectedUrls();
impl<'a> /*trait*/ QFileDialog_selectedUrls<()> for () {
  fn selectedUrls(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog12selectedUrlsEv()};
     unsafe {_ZNK11QFileDialog12selectedUrlsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn history<RetType, T: QFileDialog_history<RetType>>(&mut self, value: T) -> RetType {
    return value.history(self);
    // return 1;
  }
}

pub trait QFileDialog_history<RetType> {
  fn history(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  QStringList QFileDialog::history();
impl<'a> /*trait*/ QFileDialog_history<()> for () {
  fn history(self, rsthis: &mut QFileDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog7historyEv()};
     unsafe {_ZNK11QFileDialog7historyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDialog {
  pub fn isNameFilterDetailsVisible<RetType, T: QFileDialog_isNameFilterDetailsVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.isNameFilterDetailsVisible(self);
    // return 1;
  }
}

pub trait QFileDialog_isNameFilterDetailsVisible<RetType> {
  fn isNameFilterDetailsVisible(self, rsthis: &mut QFileDialog) -> RetType;
}

// proto:  bool QFileDialog::isNameFilterDetailsVisible();
impl<'a> /*trait*/ QFileDialog_isNameFilterDetailsVisible<i8> for () {
  fn isNameFilterDetailsVisible(self, rsthis: &mut QFileDialog) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDialog26isNameFilterDetailsVisibleEv()};
    let mut ret = unsafe {_ZNK11QFileDialog26isNameFilterDetailsVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN11QFileDialogC1EP7QWidgetRK7QStringS4_S4_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QFileDialog{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

