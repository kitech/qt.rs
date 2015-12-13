// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurl::QUrl;
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QTextBrowser::isBackwardAvailable();
  fn _ZNK12QTextBrowser19isBackwardAvailableEv() -> i32;
  // proto: void QTextBrowser::reload();
  fn _ZN12QTextBrowser6reloadEv() -> i32;
  // proto: bool QTextBrowser::openLinks();
  fn _ZNK12QTextBrowser9openLinksEv() -> i32;
  // proto: void QTextBrowser::clearHistory();
  fn _ZN12QTextBrowser12clearHistoryEv() -> i32;
  // proto: void QTextBrowser::highlighted(const QUrl & );
  fn _ZN12QTextBrowser11highlightedERK4QUrl(arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QTextBrowser::metaObject();
  fn _ZNK12QTextBrowser10metaObjectEv() -> i32;
  // proto: QUrl QTextBrowser::historyUrl(int );
  fn _ZNK12QTextBrowser10historyUrlEi(arg0: c_int) -> i32;
  // proto: void QTextBrowser::sourceChanged(const QUrl & );
  fn _ZN12QTextBrowser13sourceChangedERK4QUrl(arg0: *const c_void) -> i32;
  // proto: bool QTextBrowser::isForwardAvailable();
  fn _ZNK12QTextBrowser18isForwardAvailableEv() -> i32;
  // proto: bool QTextBrowser::openExternalLinks();
  fn _ZNK12QTextBrowser17openExternalLinksEv() -> i32;
  // proto: void QTextBrowser::NewQTextBrowser(QWidget * parent);
  fn _ZN12QTextBrowserC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QTextBrowser::backwardHistoryCount();
  fn _ZNK12QTextBrowser20backwardHistoryCountEv() -> i32;
  // proto: void QTextBrowser::home();
  fn _ZN12QTextBrowser4homeEv() -> i32;
  // proto: void QTextBrowser::FreeQTextBrowser();
  fn _ZN12QTextBrowserD0Ev() -> i32;
  // proto: void QTextBrowser::NewQTextBrowser(const QTextBrowser & );
  fn _ZN12QTextBrowserC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTextBrowser::setOpenLinks(bool open);
  fn _ZN12QTextBrowser12setOpenLinksEb(arg0: int8_t) -> i32;
  // proto: void QTextBrowser::forward();
  fn _ZN12QTextBrowser7forwardEv() -> i32;
  // proto: void QTextBrowser::highlighted(const QString & );
  fn _ZN12QTextBrowser11highlightedERK7QString(arg0: *const c_void) -> i32;
  // proto: QString QTextBrowser::historyTitle(int );
  fn _ZNK12QTextBrowser12historyTitleEi(arg0: c_int) -> i32;
  // proto: void QTextBrowser::forwardAvailable(bool );
  fn _ZN12QTextBrowser16forwardAvailableEb(arg0: int8_t) -> i32;
  // proto: void QTextBrowser::setSearchPaths(const QStringList & paths);
  fn _ZN12QTextBrowser14setSearchPathsERK11QStringList(arg0: *const c_void) -> i32;
  // proto: QVariant QTextBrowser::loadResource(int type, const QUrl & name);
  fn _ZN12QTextBrowser12loadResourceEiRK4QUrl(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QUrl QTextBrowser::source();
  fn _ZNK12QTextBrowser6sourceEv() -> i32;
  // proto: void QTextBrowser::historyChanged();
  fn _ZN12QTextBrowser14historyChangedEv() -> i32;
  // proto: void QTextBrowser::setOpenExternalLinks(bool open);
  fn _ZN12QTextBrowser20setOpenExternalLinksEb(arg0: int8_t) -> i32;
  // proto: void QTextBrowser::setSource(const QUrl & name);
  fn _ZN12QTextBrowser9setSourceERK4QUrl(arg0: *const c_void) -> i32;
  // proto: QStringList QTextBrowser::searchPaths();
  fn _ZNK12QTextBrowser11searchPathsEv() -> i32;
  // proto: void QTextBrowser::backward();
  fn _ZN12QTextBrowser8backwardEv() -> i32;
  // proto: int QTextBrowser::forwardHistoryCount();
  fn _ZNK12QTextBrowser19forwardHistoryCountEv() -> i32;
  // proto: void QTextBrowser::anchorClicked(const QUrl & );
  fn _ZN12QTextBrowser13anchorClickedERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QTextBrowser::backwardAvailable(bool );
  fn _ZN12QTextBrowser17backwardAvailableEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QTextBrowser)=1
pub struct QTextBrowser {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBrowser {
  pub fn isBackwardAvailable<T: QTextBrowser_isBackwardAvailable>(&mut self, value: T) -> i32 {
    value.isBackwardAvailable(self);
    return 1;
  }
}

pub trait QTextBrowser_isBackwardAvailable {
  fn isBackwardAvailable(self, this: &mut QTextBrowser) -> i32;
}

// proto: bool QTextBrowser::isBackwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isBackwardAvailable for () {
  fn isBackwardAvailable(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19isBackwardAvailableEv()};
    unsafe {_ZNK12QTextBrowser19isBackwardAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn reload<T: QTextBrowser_reload>(&mut self, value: T) -> i32 {
    value.reload(self);
    return 1;
  }
}

pub trait QTextBrowser_reload {
  fn reload(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::reload();
impl<'a> /*trait*/ QTextBrowser_reload for () {
  fn reload(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser6reloadEv()};
    unsafe {_ZN12QTextBrowser6reloadEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn openLinks<T: QTextBrowser_openLinks>(&mut self, value: T) -> i32 {
    value.openLinks(self);
    return 1;
  }
}

pub trait QTextBrowser_openLinks {
  fn openLinks(self, this: &mut QTextBrowser) -> i32;
}

// proto: bool QTextBrowser::openLinks();
impl<'a> /*trait*/ QTextBrowser_openLinks for () {
  fn openLinks(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser9openLinksEv()};
    unsafe {_ZNK12QTextBrowser9openLinksEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn clearHistory<T: QTextBrowser_clearHistory>(&mut self, value: T) -> i32 {
    value.clearHistory(self);
    return 1;
  }
}

pub trait QTextBrowser_clearHistory {
  fn clearHistory(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::clearHistory();
impl<'a> /*trait*/ QTextBrowser_clearHistory for () {
  fn clearHistory(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12clearHistoryEv()};
    unsafe {_ZN12QTextBrowser12clearHistoryEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn highlighted<T: QTextBrowser_highlighted>(&mut self, value: T) -> i32 {
    value.highlighted(self);
    return 1;
  }
}

pub trait QTextBrowser_highlighted {
  fn highlighted(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::highlighted(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_highlighted for (&'a  QUrl) {
  fn highlighted(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser11highlightedERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser11highlightedERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn metaObject<T: QTextBrowser_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextBrowser_metaObject {
  fn metaObject(self, this: &mut QTextBrowser) -> i32;
}

// proto: const QMetaObject * QTextBrowser::metaObject();
impl<'a> /*trait*/ QTextBrowser_metaObject for () {
  fn metaObject(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10metaObjectEv()};
    unsafe {_ZNK12QTextBrowser10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn historyUrl<T: QTextBrowser_historyUrl>(&mut self, value: T) -> i32 {
    value.historyUrl(self);
    return 1;
  }
}

pub trait QTextBrowser_historyUrl {
  fn historyUrl(self, this: &mut QTextBrowser) -> i32;
}

// proto: QUrl QTextBrowser::historyUrl(int );
impl<'a> /*trait*/ QTextBrowser_historyUrl for (i32) {
  fn historyUrl(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10historyUrlEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK12QTextBrowser10historyUrlEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn sourceChanged<T: QTextBrowser_sourceChanged>(&mut self, value: T) -> i32 {
    value.sourceChanged(self);
    return 1;
  }
}

pub trait QTextBrowser_sourceChanged {
  fn sourceChanged(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::sourceChanged(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_sourceChanged for (&'a  QUrl) {
  fn sourceChanged(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser13sourceChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser13sourceChangedERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn isForwardAvailable<T: QTextBrowser_isForwardAvailable>(&mut self, value: T) -> i32 {
    value.isForwardAvailable(self);
    return 1;
  }
}

pub trait QTextBrowser_isForwardAvailable {
  fn isForwardAvailable(self, this: &mut QTextBrowser) -> i32;
}

// proto: bool QTextBrowser::isForwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isForwardAvailable for () {
  fn isForwardAvailable(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser18isForwardAvailableEv()};
    unsafe {_ZNK12QTextBrowser18isForwardAvailableEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn openExternalLinks<T: QTextBrowser_openExternalLinks>(&mut self, value: T) -> i32 {
    value.openExternalLinks(self);
    return 1;
  }
}

pub trait QTextBrowser_openExternalLinks {
  fn openExternalLinks(self, this: &mut QTextBrowser) -> i32;
}

// proto: bool QTextBrowser::openExternalLinks();
impl<'a> /*trait*/ QTextBrowser_openExternalLinks for () {
  fn openExternalLinks(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser17openExternalLinksEv()};
    unsafe {_ZNK12QTextBrowser17openExternalLinksEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn NewQTextBrowser<T: QTextBrowser_NewQTextBrowser>(value: T) -> QTextBrowser {
    let rsthis = value.NewQTextBrowser();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBrowser_NewQTextBrowser {
  fn NewQTextBrowser(self) -> QTextBrowser;
}

// proto: void QTextBrowser::NewQTextBrowser(QWidget * parent);
impl<'a> /*trait*/ QTextBrowser_NewQTextBrowser for (&'a mut QWidget) {
  fn NewQTextBrowser(self) -> QTextBrowser {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextBrowserC1EP7QWidget(qthis, arg0)};
    let rsthis = QTextBrowser{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn backwardHistoryCount<T: QTextBrowser_backwardHistoryCount>(&mut self, value: T) -> i32 {
    value.backwardHistoryCount(self);
    return 1;
  }
}

pub trait QTextBrowser_backwardHistoryCount {
  fn backwardHistoryCount(self, this: &mut QTextBrowser) -> i32;
}

// proto: int QTextBrowser::backwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_backwardHistoryCount for () {
  fn backwardHistoryCount(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser20backwardHistoryCountEv()};
    unsafe {_ZNK12QTextBrowser20backwardHistoryCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn home<T: QTextBrowser_home>(&mut self, value: T) -> i32 {
    value.home(self);
    return 1;
  }
}

pub trait QTextBrowser_home {
  fn home(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::home();
impl<'a> /*trait*/ QTextBrowser_home for () {
  fn home(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser4homeEv()};
    unsafe {_ZN12QTextBrowser4homeEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn FreeQTextBrowser<T: QTextBrowser_FreeQTextBrowser>(&mut self, value: T) -> i32 {
    value.FreeQTextBrowser(self);
    return 1;
  }
}

pub trait QTextBrowser_FreeQTextBrowser {
  fn FreeQTextBrowser(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::FreeQTextBrowser();
impl<'a> /*trait*/ QTextBrowser_FreeQTextBrowser for () {
  fn FreeQTextBrowser(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserD0Ev()};
    unsafe {_ZN12QTextBrowserD0Ev()};
    return 1;
  }
}

// proto: void QTextBrowser::NewQTextBrowser(const QTextBrowser & );
impl<'a> /*trait*/ QTextBrowser_NewQTextBrowser for (&'a  QTextBrowser) {
  fn NewQTextBrowser(self) -> QTextBrowser {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowserC1ERKS_(qthis, arg0)};
    let rsthis = QTextBrowser{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setOpenLinks<T: QTextBrowser_setOpenLinks>(&mut self, value: T) -> i32 {
    value.setOpenLinks(self);
    return 1;
  }
}

pub trait QTextBrowser_setOpenLinks {
  fn setOpenLinks(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::setOpenLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenLinks for (i8) {
  fn setOpenLinks(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12setOpenLinksEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QTextBrowser12setOpenLinksEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn forward<T: QTextBrowser_forward>(&mut self, value: T) -> i32 {
    value.forward(self);
    return 1;
  }
}

pub trait QTextBrowser_forward {
  fn forward(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::forward();
impl<'a> /*trait*/ QTextBrowser_forward for () {
  fn forward(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser7forwardEv()};
    unsafe {_ZN12QTextBrowser7forwardEv()};
    return 1;
  }
}

// proto: void QTextBrowser::highlighted(const QString & );
impl<'a> /*trait*/ QTextBrowser_highlighted for (&'a  QString) {
  fn highlighted(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser11highlightedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn historyTitle<T: QTextBrowser_historyTitle>(&mut self, value: T) -> i32 {
    value.historyTitle(self);
    return 1;
  }
}

pub trait QTextBrowser_historyTitle {
  fn historyTitle(self, this: &mut QTextBrowser) -> i32;
}

// proto: QString QTextBrowser::historyTitle(int );
impl<'a> /*trait*/ QTextBrowser_historyTitle for (i32) {
  fn historyTitle(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser12historyTitleEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK12QTextBrowser12historyTitleEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn forwardAvailable<T: QTextBrowser_forwardAvailable>(&mut self, value: T) -> i32 {
    value.forwardAvailable(self);
    return 1;
  }
}

pub trait QTextBrowser_forwardAvailable {
  fn forwardAvailable(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::forwardAvailable(bool );
impl<'a> /*trait*/ QTextBrowser_forwardAvailable for (i8) {
  fn forwardAvailable(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser16forwardAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QTextBrowser16forwardAvailableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setSearchPaths<T: QTextBrowser_setSearchPaths>(&mut self, value: T) -> i32 {
    value.setSearchPaths(self);
    return 1;
  }
}

pub trait QTextBrowser_setSearchPaths {
  fn setSearchPaths(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::setSearchPaths(const QStringList & paths);
impl<'a> /*trait*/ QTextBrowser_setSearchPaths for (&'a  QStringList) {
  fn setSearchPaths(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14setSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser14setSearchPathsERK11QStringList(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn loadResource<T: QTextBrowser_loadResource>(&mut self, value: T) -> i32 {
    value.loadResource(self);
    return 1;
  }
}

pub trait QTextBrowser_loadResource {
  fn loadResource(self, this: &mut QTextBrowser) -> i32;
}

// proto: QVariant QTextBrowser::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_loadResource for (i32, &'a  QUrl) {
  fn loadResource(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser12loadResourceEiRK4QUrl(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn source<T: QTextBrowser_source>(&mut self, value: T) -> i32 {
    value.source(self);
    return 1;
  }
}

pub trait QTextBrowser_source {
  fn source(self, this: &mut QTextBrowser) -> i32;
}

// proto: QUrl QTextBrowser::source();
impl<'a> /*trait*/ QTextBrowser_source for () {
  fn source(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser6sourceEv()};
    unsafe {_ZNK12QTextBrowser6sourceEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn historyChanged<T: QTextBrowser_historyChanged>(&mut self, value: T) -> i32 {
    value.historyChanged(self);
    return 1;
  }
}

pub trait QTextBrowser_historyChanged {
  fn historyChanged(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::historyChanged();
impl<'a> /*trait*/ QTextBrowser_historyChanged for () {
  fn historyChanged(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14historyChangedEv()};
    unsafe {_ZN12QTextBrowser14historyChangedEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setOpenExternalLinks<T: QTextBrowser_setOpenExternalLinks>(&mut self, value: T) -> i32 {
    value.setOpenExternalLinks(self);
    return 1;
  }
}

pub trait QTextBrowser_setOpenExternalLinks {
  fn setOpenExternalLinks(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenExternalLinks for (i8) {
  fn setOpenExternalLinks(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser20setOpenExternalLinksEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QTextBrowser20setOpenExternalLinksEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setSource<T: QTextBrowser_setSource>(&mut self, value: T) -> i32 {
    value.setSource(self);
    return 1;
  }
}

pub trait QTextBrowser_setSource {
  fn setSource(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::setSource(const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_setSource for (&'a  QUrl) {
  fn setSource(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser9setSourceERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser9setSourceERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn searchPaths<T: QTextBrowser_searchPaths>(&mut self, value: T) -> i32 {
    value.searchPaths(self);
    return 1;
  }
}

pub trait QTextBrowser_searchPaths {
  fn searchPaths(self, this: &mut QTextBrowser) -> i32;
}

// proto: QStringList QTextBrowser::searchPaths();
impl<'a> /*trait*/ QTextBrowser_searchPaths for () {
  fn searchPaths(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser11searchPathsEv()};
    unsafe {_ZNK12QTextBrowser11searchPathsEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn backward<T: QTextBrowser_backward>(&mut self, value: T) -> i32 {
    value.backward(self);
    return 1;
  }
}

pub trait QTextBrowser_backward {
  fn backward(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::backward();
impl<'a> /*trait*/ QTextBrowser_backward for () {
  fn backward(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser8backwardEv()};
    unsafe {_ZN12QTextBrowser8backwardEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn forwardHistoryCount<T: QTextBrowser_forwardHistoryCount>(&mut self, value: T) -> i32 {
    value.forwardHistoryCount(self);
    return 1;
  }
}

pub trait QTextBrowser_forwardHistoryCount {
  fn forwardHistoryCount(self, this: &mut QTextBrowser) -> i32;
}

// proto: int QTextBrowser::forwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_forwardHistoryCount for () {
  fn forwardHistoryCount(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19forwardHistoryCountEv()};
    unsafe {_ZNK12QTextBrowser19forwardHistoryCountEv()};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn anchorClicked<T: QTextBrowser_anchorClicked>(&mut self, value: T) -> i32 {
    value.anchorClicked(self);
    return 1;
  }
}

pub trait QTextBrowser_anchorClicked {
  fn anchorClicked(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::anchorClicked(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_anchorClicked for (&'a  QUrl) {
  fn anchorClicked(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser13anchorClickedERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QTextBrowser13anchorClickedERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn backwardAvailable<T: QTextBrowser_backwardAvailable>(&mut self, value: T) -> i32 {
    value.backwardAvailable(self);
    return 1;
  }
}

pub trait QTextBrowser_backwardAvailable {
  fn backwardAvailable(self, this: &mut QTextBrowser) -> i32;
}

// proto: void QTextBrowser::backwardAvailable(bool );
impl<'a> /*trait*/ QTextBrowser_backwardAvailable for (i8) {
  fn backwardAvailable(self, this: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser17backwardAvailableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QTextBrowser17backwardAvailableEb(arg0)};
    return 1;
  }
}

