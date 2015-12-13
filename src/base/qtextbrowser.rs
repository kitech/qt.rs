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
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTextBrowser::isBackwardAvailable();
  fn _ZNK12QTextBrowser19isBackwardAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextBrowser::reload();
  fn _ZN12QTextBrowser6reloadEv(qthis: *mut c_void) ;
  // proto:  bool QTextBrowser::openLinks();
  fn _ZNK12QTextBrowser9openLinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextBrowser::clearHistory();
  fn _ZN12QTextBrowser12clearHistoryEv(qthis: *mut c_void) ;
  // proto:  void QTextBrowser::highlighted(const QUrl & );
  fn _ZN12QTextBrowser11highlightedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QTextBrowser::metaObject();
  fn _ZNK12QTextBrowser10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QUrl QTextBrowser::historyUrl(int );
  fn _ZNK12QTextBrowser10historyUrlEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextBrowser::sourceChanged(const QUrl & );
  fn _ZN12QTextBrowser13sourceChangedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTextBrowser::isForwardAvailable();
  fn _ZNK12QTextBrowser18isForwardAvailableEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTextBrowser::openExternalLinks();
  fn _ZNK12QTextBrowser17openExternalLinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextBrowser::NewQTextBrowser(QWidget * parent);
  fn _ZN12QTextBrowserC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTextBrowser::backwardHistoryCount();
  fn _ZNK12QTextBrowser20backwardHistoryCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBrowser::home();
  fn _ZN12QTextBrowser4homeEv(qthis: *mut c_void) ;
  // proto:  void QTextBrowser::FreeQTextBrowser();
  fn _ZN12QTextBrowserD0Ev(qthis: *mut c_void) ;
  // proto:  void QTextBrowser::NewQTextBrowser(const QTextBrowser & );
  fn _ZN12QTextBrowserC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextBrowser::setOpenLinks(bool open);
  fn _ZN12QTextBrowser12setOpenLinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextBrowser::forward();
  fn _ZN12QTextBrowser7forwardEv(qthis: *mut c_void) ;
  // proto:  void QTextBrowser::highlighted(const QString & );
  fn _ZN12QTextBrowser11highlightedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QTextBrowser::historyTitle(int );
  fn _ZNK12QTextBrowser12historyTitleEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextBrowser::forwardAvailable(bool );
  fn _ZN12QTextBrowser16forwardAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
  fn _ZN12QTextBrowser14setSearchPathsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
  fn _ZN12QTextBrowser12loadResourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QUrl QTextBrowser::source();
  fn _ZNK12QTextBrowser6sourceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBrowser::historyChanged();
  fn _ZN12QTextBrowser14historyChangedEv(qthis: *mut c_void) ;
  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
  fn _ZN12QTextBrowser20setOpenExternalLinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTextBrowser::setSource(const QUrl & name);
  fn _ZN12QTextBrowser9setSourceERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QTextBrowser::searchPaths();
  fn _ZNK12QTextBrowser11searchPathsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBrowser::backward();
  fn _ZN12QTextBrowser8backwardEv(qthis: *mut c_void) ;
  // proto:  int QTextBrowser::forwardHistoryCount();
  fn _ZNK12QTextBrowser19forwardHistoryCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBrowser::anchorClicked(const QUrl & );
  fn _ZN12QTextBrowser13anchorClickedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextBrowser::backwardAvailable(bool );
  fn _ZN12QTextBrowser17backwardAvailableEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QTextBrowser)=1
pub struct QTextBrowser {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBrowser {
  pub fn isBackwardAvailable<T: QTextBrowser_isBackwardAvailable>(&mut self, value: T) -> i8 {
    return value.isBackwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_isBackwardAvailable {
  fn isBackwardAvailable(self, rsthis: &mut QTextBrowser) -> i8;
}

// proto:  bool QTextBrowser::isBackwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isBackwardAvailable for () {
  fn isBackwardAvailable(self, rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19isBackwardAvailableEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser19isBackwardAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn reload<T: QTextBrowser_reload>(&mut self, value: T)  {
     value.reload(self);
    // return 1;
  }
}

pub trait QTextBrowser_reload {
  fn reload(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::reload();
impl<'a> /*trait*/ QTextBrowser_reload for () {
  fn reload(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser6reloadEv()};
     unsafe {_ZN12QTextBrowser6reloadEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn openLinks<T: QTextBrowser_openLinks>(&mut self, value: T) -> i8 {
    return value.openLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_openLinks {
  fn openLinks(self, rsthis: &mut QTextBrowser) -> i8;
}

// proto:  bool QTextBrowser::openLinks();
impl<'a> /*trait*/ QTextBrowser_openLinks for () {
  fn openLinks(self, rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser9openLinksEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser9openLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn clearHistory<T: QTextBrowser_clearHistory>(&mut self, value: T)  {
     value.clearHistory(self);
    // return 1;
  }
}

pub trait QTextBrowser_clearHistory {
  fn clearHistory(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::clearHistory();
impl<'a> /*trait*/ QTextBrowser_clearHistory for () {
  fn clearHistory(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12clearHistoryEv()};
     unsafe {_ZN12QTextBrowser12clearHistoryEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn highlighted<T: QTextBrowser_highlighted>(&mut self, value: T)  {
     value.highlighted(self);
    // return 1;
  }
}

pub trait QTextBrowser_highlighted {
  fn highlighted(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::highlighted(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_highlighted for (&'a  QUrl) {
  fn highlighted(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser11highlightedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser11highlightedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn metaObject<T: QTextBrowser_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTextBrowser_metaObject {
  fn metaObject(self, rsthis: &mut QTextBrowser) ;
}

// proto:  const QMetaObject * QTextBrowser::metaObject();
impl<'a> /*trait*/ QTextBrowser_metaObject for () {
  fn metaObject(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10metaObjectEv()};
     unsafe {_ZNK12QTextBrowser10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn historyUrl<T: QTextBrowser_historyUrl>(&mut self, value: T) -> QUrl {
    return value.historyUrl(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyUrl {
  fn historyUrl(self, rsthis: &mut QTextBrowser) -> QUrl;
}

// proto:  QUrl QTextBrowser::historyUrl(int );
impl<'a> /*trait*/ QTextBrowser_historyUrl for (i32) {
  fn historyUrl(self, rsthis: &mut QTextBrowser) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10historyUrlEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTextBrowser10historyUrlEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn sourceChanged<T: QTextBrowser_sourceChanged>(&mut self, value: T)  {
     value.sourceChanged(self);
    // return 1;
  }
}

pub trait QTextBrowser_sourceChanged {
  fn sourceChanged(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::sourceChanged(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_sourceChanged for (&'a  QUrl) {
  fn sourceChanged(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser13sourceChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser13sourceChangedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn isForwardAvailable<T: QTextBrowser_isForwardAvailable>(&mut self, value: T) -> i8 {
    return value.isForwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_isForwardAvailable {
  fn isForwardAvailable(self, rsthis: &mut QTextBrowser) -> i8;
}

// proto:  bool QTextBrowser::isForwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isForwardAvailable for () {
  fn isForwardAvailable(self, rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser18isForwardAvailableEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser18isForwardAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn openExternalLinks<T: QTextBrowser_openExternalLinks>(&mut self, value: T) -> i8 {
    return value.openExternalLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_openExternalLinks {
  fn openExternalLinks(self, rsthis: &mut QTextBrowser) -> i8;
}

// proto:  bool QTextBrowser::openExternalLinks();
impl<'a> /*trait*/ QTextBrowser_openExternalLinks for () {
  fn openExternalLinks(self, rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser17openExternalLinksEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
    return value.backwardHistoryCount(self);
    // return 1;
  }
}

pub trait QTextBrowser_backwardHistoryCount {
  fn backwardHistoryCount(self, rsthis: &mut QTextBrowser) -> i32;
}

// proto:  int QTextBrowser::backwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_backwardHistoryCount for () {
  fn backwardHistoryCount(self, rsthis: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser20backwardHistoryCountEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser20backwardHistoryCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn home<T: QTextBrowser_home>(&mut self, value: T)  {
     value.home(self);
    // return 1;
  }
}

pub trait QTextBrowser_home {
  fn home(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::home();
impl<'a> /*trait*/ QTextBrowser_home for () {
  fn home(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser4homeEv()};
     unsafe {_ZN12QTextBrowser4homeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn FreeQTextBrowser<T: QTextBrowser_FreeQTextBrowser>(&mut self, value: T)  {
     value.FreeQTextBrowser(self);
    // return 1;
  }
}

pub trait QTextBrowser_FreeQTextBrowser {
  fn FreeQTextBrowser(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::FreeQTextBrowser();
impl<'a> /*trait*/ QTextBrowser_FreeQTextBrowser for () {
  fn FreeQTextBrowser(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserD0Ev()};
     unsafe {_ZN12QTextBrowserD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QTextBrowser::NewQTextBrowser(const QTextBrowser & );
impl<'a> /*trait*/ QTextBrowser_NewQTextBrowser for (&'a  QTextBrowser) {
  fn NewQTextBrowser(self) -> QTextBrowser {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextBrowserC1ERKS_(qthis, arg0)};
    let rsthis = QTextBrowser{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setOpenLinks<T: QTextBrowser_setOpenLinks>(&mut self, value: T)  {
     value.setOpenLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_setOpenLinks {
  fn setOpenLinks(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::setOpenLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenLinks for (i8) {
  fn setOpenLinks(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12setOpenLinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QTextBrowser12setOpenLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn forward<T: QTextBrowser_forward>(&mut self, value: T)  {
     value.forward(self);
    // return 1;
  }
}

pub trait QTextBrowser_forward {
  fn forward(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::forward();
impl<'a> /*trait*/ QTextBrowser_forward for () {
  fn forward(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser7forwardEv()};
     unsafe {_ZN12QTextBrowser7forwardEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QTextBrowser::highlighted(const QString & );
impl<'a> /*trait*/ QTextBrowser_highlighted for (&'a  QString) {
  fn highlighted(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser11highlightedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn historyTitle<T: QTextBrowser_historyTitle>(&mut self, value: T) -> QString {
    return value.historyTitle(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyTitle {
  fn historyTitle(self, rsthis: &mut QTextBrowser) -> QString;
}

// proto:  QString QTextBrowser::historyTitle(int );
impl<'a> /*trait*/ QTextBrowser_historyTitle for (i32) {
  fn historyTitle(self, rsthis: &mut QTextBrowser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser12historyTitleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTextBrowser12historyTitleEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn forwardAvailable<T: QTextBrowser_forwardAvailable>(&mut self, value: T)  {
     value.forwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_forwardAvailable {
  fn forwardAvailable(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::forwardAvailable(bool );
impl<'a> /*trait*/ QTextBrowser_forwardAvailable for (i8) {
  fn forwardAvailable(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser16forwardAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QTextBrowser16forwardAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setSearchPaths<T: QTextBrowser_setSearchPaths>(&mut self, value: T)  {
     value.setSearchPaths(self);
    // return 1;
  }
}

pub trait QTextBrowser_setSearchPaths {
  fn setSearchPaths(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
impl<'a> /*trait*/ QTextBrowser_setSearchPaths for (&'a  QStringList) {
  fn setSearchPaths(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14setSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser14setSearchPathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn loadResource<T: QTextBrowser_loadResource>(&mut self, value: T) -> QVariant {
    return value.loadResource(self);
    // return 1;
  }
}

pub trait QTextBrowser_loadResource {
  fn loadResource(self, rsthis: &mut QTextBrowser) -> QVariant;
}

// proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_loadResource for (i32, &'a  QUrl) {
  fn loadResource(self, rsthis: &mut QTextBrowser) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QTextBrowser12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn source<T: QTextBrowser_source>(&mut self, value: T) -> QUrl {
    return value.source(self);
    // return 1;
  }
}

pub trait QTextBrowser_source {
  fn source(self, rsthis: &mut QTextBrowser) -> QUrl;
}

// proto:  QUrl QTextBrowser::source();
impl<'a> /*trait*/ QTextBrowser_source for () {
  fn source(self, rsthis: &mut QTextBrowser) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser6sourceEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn historyChanged<T: QTextBrowser_historyChanged>(&mut self, value: T)  {
     value.historyChanged(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyChanged {
  fn historyChanged(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::historyChanged();
impl<'a> /*trait*/ QTextBrowser_historyChanged for () {
  fn historyChanged(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14historyChangedEv()};
     unsafe {_ZN12QTextBrowser14historyChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setOpenExternalLinks<T: QTextBrowser_setOpenExternalLinks>(&mut self, value: T)  {
     value.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_setOpenExternalLinks {
  fn setOpenExternalLinks(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenExternalLinks for (i8) {
  fn setOpenExternalLinks(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser20setOpenExternalLinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QTextBrowser20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn setSource<T: QTextBrowser_setSource>(&mut self, value: T)  {
     value.setSource(self);
    // return 1;
  }
}

pub trait QTextBrowser_setSource {
  fn setSource(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::setSource(const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_setSource for (&'a  QUrl) {
  fn setSource(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser9setSourceERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser9setSourceERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn searchPaths<T: QTextBrowser_searchPaths>(&mut self, value: T) -> QStringList {
    return value.searchPaths(self);
    // return 1;
  }
}

pub trait QTextBrowser_searchPaths {
  fn searchPaths(self, rsthis: &mut QTextBrowser) -> QStringList;
}

// proto:  QStringList QTextBrowser::searchPaths();
impl<'a> /*trait*/ QTextBrowser_searchPaths for () {
  fn searchPaths(self, rsthis: &mut QTextBrowser) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser11searchPathsEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser11searchPathsEv(rsthis.qclsinst)};
    let mut ret1 = QStringList{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn backward<T: QTextBrowser_backward>(&mut self, value: T)  {
     value.backward(self);
    // return 1;
  }
}

pub trait QTextBrowser_backward {
  fn backward(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::backward();
impl<'a> /*trait*/ QTextBrowser_backward for () {
  fn backward(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser8backwardEv()};
     unsafe {_ZN12QTextBrowser8backwardEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn forwardHistoryCount<T: QTextBrowser_forwardHistoryCount>(&mut self, value: T) -> i32 {
    return value.forwardHistoryCount(self);
    // return 1;
  }
}

pub trait QTextBrowser_forwardHistoryCount {
  fn forwardHistoryCount(self, rsthis: &mut QTextBrowser) -> i32;
}

// proto:  int QTextBrowser::forwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_forwardHistoryCount for () {
  fn forwardHistoryCount(self, rsthis: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19forwardHistoryCountEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser19forwardHistoryCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn anchorClicked<T: QTextBrowser_anchorClicked>(&mut self, value: T)  {
     value.anchorClicked(self);
    // return 1;
  }
}

pub trait QTextBrowser_anchorClicked {
  fn anchorClicked(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::anchorClicked(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_anchorClicked for (&'a  QUrl) {
  fn anchorClicked(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser13anchorClickedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser13anchorClickedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTextBrowser {
  pub fn backwardAvailable<T: QTextBrowser_backwardAvailable>(&mut self, value: T)  {
     value.backwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_backwardAvailable {
  fn backwardAvailable(self, rsthis: &mut QTextBrowser) ;
}

// proto:  void QTextBrowser::backwardAvailable(bool );
impl<'a> /*trait*/ QTextBrowser_backwardAvailable for (i8) {
  fn backwardAvailable(self, rsthis: &mut QTextBrowser)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser17backwardAvailableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QTextBrowser17backwardAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

