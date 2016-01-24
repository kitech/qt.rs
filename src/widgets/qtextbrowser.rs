// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qtextbrowser.h
// dst-file: /src/widgets/qtextbrowser.rs
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
use super::qtextedit::QTextEdit; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qurl::QUrl; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qstring::QString; // 771
use super::super::core::qstringlist::QStringList; // 771
use super::super::core::qvariant::QVariant; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextBrowser_Class_Size() -> c_int;
  // proto:  bool QTextBrowser::isBackwardAvailable();
  fn C_ZNK12QTextBrowser19isBackwardAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextBrowser::reload();
  fn C_ZN12QTextBrowser6reloadEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextBrowser::openLinks();
  fn C_ZNK12QTextBrowser9openLinksEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextBrowser::clearHistory();
  fn C_ZN12QTextBrowser12clearHistoryEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QTextBrowser::metaObject();
  fn C_ZNK12QTextBrowser10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUrl QTextBrowser::historyUrl(int );
  fn C_ZNK12QTextBrowser10historyUrlEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTextBrowser::isForwardAvailable();
  fn C_ZNK12QTextBrowser18isForwardAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTextBrowser::openExternalLinks();
  fn C_ZNK12QTextBrowser17openExternalLinksEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextBrowser::QTextBrowser(QWidget * parent);
  fn C_ZN12QTextBrowserC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  int QTextBrowser::backwardHistoryCount();
  fn C_ZNK12QTextBrowser20backwardHistoryCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextBrowser::home();
  fn C_ZN12QTextBrowser4homeEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextBrowser::~QTextBrowser();
  fn C_ZN12QTextBrowserD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextBrowser::setOpenLinks(bool open);
  fn C_ZN12QTextBrowser12setOpenLinksEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextBrowser::forward();
  fn C_ZN12QTextBrowser7forwardEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QTextBrowser::historyTitle(int );
  fn C_ZNK12QTextBrowser12historyTitleEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
  fn C_ZN12QTextBrowser14setSearchPathsERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
  fn C_ZN12QTextBrowser12loadResourceEiRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QUrl QTextBrowser::source();
  fn C_ZNK12QTextBrowser6sourceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
  fn C_ZN12QTextBrowser20setOpenExternalLinksEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextBrowser::setSource(const QUrl & name);
  fn C_ZN12QTextBrowser9setSourceERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QTextBrowser::searchPaths();
  fn C_ZNK12QTextBrowser11searchPathsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextBrowser::backward();
  fn C_ZN12QTextBrowser8backwardEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextBrowser::forwardHistoryCount();
  fn C_ZNK12QTextBrowser19forwardHistoryCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser14historyChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser11highlightedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser13sourceChangedERK4QUrl(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser11highlightedERK4QUrl(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser16forwardAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser13anchorClickedERK4QUrl(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextBrowser_SlotProxy_connect__ZN12QTextBrowser17backwardAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTextBrowser)=1
#[derive(Default)]
pub struct QTextBrowser {
  qbase: QTextEdit,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _forwardAvailable: QTextBrowser_forwardAvailable_signal,
  pub _sourceChanged: QTextBrowser_sourceChanged_signal,
  pub _highlighted: QTextBrowser_highlighted_signal,
  pub _anchorClicked: QTextBrowser_anchorClicked_signal,
  pub _historyChanged: QTextBrowser_historyChanged_signal,
  pub _backwardAvailable: QTextBrowser_backwardAvailable_signal,
}

impl /*struct*/ QTextBrowser {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextBrowser {
    return QTextBrowser{qbase: QTextEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextBrowser {
  type Target = QTextEdit;

  fn deref(&self) -> &QTextEdit {
    return & self.qbase;
  }
}
impl AsRef<QTextEdit> for QTextBrowser {
  fn as_ref(& self) -> & QTextEdit {
    return & self.qbase;
  }
}
  // proto:  bool QTextBrowser::isBackwardAvailable();
impl /*struct*/ QTextBrowser {
  pub fn isBackwardAvailable<RetType, T: QTextBrowser_isBackwardAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBackwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_isBackwardAvailable<RetType> {
  fn isBackwardAvailable(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::isBackwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isBackwardAvailable<i8> for () {
  fn isBackwardAvailable(self , rsthis: & QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19isBackwardAvailableEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser19isBackwardAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBrowser::reload();
impl /*struct*/ QTextBrowser {
  pub fn reload<RetType, T: QTextBrowser_reload<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reload(self);
    // return 1;
  }
}

pub trait QTextBrowser_reload<RetType> {
  fn reload(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::reload();
impl<'a> /*trait*/ QTextBrowser_reload<()> for () {
  fn reload(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser6reloadEv()};
     unsafe {C_ZN12QTextBrowser6reloadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextBrowser::openLinks();
impl /*struct*/ QTextBrowser {
  pub fn openLinks<RetType, T: QTextBrowser_openLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_openLinks<RetType> {
  fn openLinks(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::openLinks();
impl<'a> /*trait*/ QTextBrowser_openLinks<i8> for () {
  fn openLinks(self , rsthis: & QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser9openLinksEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser9openLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBrowser::clearHistory();
impl /*struct*/ QTextBrowser {
  pub fn clearHistory<RetType, T: QTextBrowser_clearHistory<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearHistory(self);
    // return 1;
  }
}

pub trait QTextBrowser_clearHistory<RetType> {
  fn clearHistory(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::clearHistory();
impl<'a> /*trait*/ QTextBrowser_clearHistory<()> for () {
  fn clearHistory(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12clearHistoryEv()};
     unsafe {C_ZN12QTextBrowser12clearHistoryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextBrowser::metaObject();
impl /*struct*/ QTextBrowser {
  pub fn metaObject<RetType, T: QTextBrowser_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextBrowser_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  const QMetaObject * QTextBrowser::metaObject();
impl<'a> /*trait*/ QTextBrowser_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTextBrowser) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QTextBrowser::historyUrl(int );
impl /*struct*/ QTextBrowser {
  pub fn historyUrl<RetType, T: QTextBrowser_historyUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.historyUrl(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyUrl<RetType> {
  fn historyUrl(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  QUrl QTextBrowser::historyUrl(int );
impl<'a> /*trait*/ QTextBrowser_historyUrl<QUrl> for (i32) {
  fn historyUrl(self , rsthis: & QTextBrowser) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10historyUrlEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK12QTextBrowser10historyUrlEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextBrowser::isForwardAvailable();
impl /*struct*/ QTextBrowser {
  pub fn isForwardAvailable<RetType, T: QTextBrowser_isForwardAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isForwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_isForwardAvailable<RetType> {
  fn isForwardAvailable(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::isForwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isForwardAvailable<i8> for () {
  fn isForwardAvailable(self , rsthis: & QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser18isForwardAvailableEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser18isForwardAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextBrowser::openExternalLinks();
impl /*struct*/ QTextBrowser {
  pub fn openExternalLinks<RetType, T: QTextBrowser_openExternalLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_openExternalLinks<RetType> {
  fn openExternalLinks(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::openExternalLinks();
impl<'a> /*trait*/ QTextBrowser_openExternalLinks<i8> for () {
  fn openExternalLinks(self , rsthis: & QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser17openExternalLinksEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBrowser::QTextBrowser(QWidget * parent);
impl /*struct*/ QTextBrowser {
  pub fn new<T: QTextBrowser_new>(value: T) -> QTextBrowser {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBrowser_new {
  fn new(self) -> QTextBrowser;
}

  // proto:  void QTextBrowser::QTextBrowser(QWidget * parent);
impl<'a> /*trait*/ QTextBrowser_new for (&'a QWidget) {
  fn new(self) -> QTextBrowser {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserC2EP7QWidget()};
    let ctysz: c_int = unsafe{QTextBrowser_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QTextBrowserC2EP7QWidget(arg0)};
    let rsthis = QTextBrowser{qbase: QTextEdit::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextBrowser::backwardHistoryCount();
impl /*struct*/ QTextBrowser {
  pub fn backwardHistoryCount<RetType, T: QTextBrowser_backwardHistoryCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backwardHistoryCount(self);
    // return 1;
  }
}

pub trait QTextBrowser_backwardHistoryCount<RetType> {
  fn backwardHistoryCount(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  int QTextBrowser::backwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_backwardHistoryCount<i32> for () {
  fn backwardHistoryCount(self , rsthis: & QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser20backwardHistoryCountEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser20backwardHistoryCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBrowser::home();
impl /*struct*/ QTextBrowser {
  pub fn home<RetType, T: QTextBrowser_home<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.home(self);
    // return 1;
  }
}

pub trait QTextBrowser_home<RetType> {
  fn home(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::home();
impl<'a> /*trait*/ QTextBrowser_home<()> for () {
  fn home(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser4homeEv()};
     unsafe {C_ZN12QTextBrowser4homeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::~QTextBrowser();
impl /*struct*/ QTextBrowser {
  pub fn free<RetType, T: QTextBrowser_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextBrowser_free<RetType> {
  fn free(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::~QTextBrowser();
impl<'a> /*trait*/ QTextBrowser_free<()> for () {
  fn free(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserD2Ev()};
     unsafe {C_ZN12QTextBrowserD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::setOpenLinks(bool open);
impl /*struct*/ QTextBrowser {
  pub fn setOpenLinks<RetType, T: QTextBrowser_setOpenLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpenLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_setOpenLinks<RetType> {
  fn setOpenLinks(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setOpenLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenLinks<()> for (i8) {
  fn setOpenLinks(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12setOpenLinksEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QTextBrowser12setOpenLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::forward();
impl /*struct*/ QTextBrowser {
  pub fn forward<RetType, T: QTextBrowser_forward<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.forward(self);
    // return 1;
  }
}

pub trait QTextBrowser_forward<RetType> {
  fn forward(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::forward();
impl<'a> /*trait*/ QTextBrowser_forward<()> for () {
  fn forward(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser7forwardEv()};
     unsafe {C_ZN12QTextBrowser7forwardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTextBrowser::historyTitle(int );
impl /*struct*/ QTextBrowser {
  pub fn historyTitle<RetType, T: QTextBrowser_historyTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.historyTitle(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyTitle<RetType> {
  fn historyTitle(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  QString QTextBrowser::historyTitle(int );
impl<'a> /*trait*/ QTextBrowser_historyTitle<QString> for (i32) {
  fn historyTitle(self , rsthis: & QTextBrowser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser12historyTitleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK12QTextBrowser12historyTitleEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
impl /*struct*/ QTextBrowser {
  pub fn setSearchPaths<RetType, T: QTextBrowser_setSearchPaths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSearchPaths(self);
    // return 1;
  }
}

pub trait QTextBrowser_setSearchPaths<RetType> {
  fn setSearchPaths(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
impl<'a> /*trait*/ QTextBrowser_setSearchPaths<()> for (&'a QStringList) {
  fn setSearchPaths(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14setSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTextBrowser14setSearchPathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
impl /*struct*/ QTextBrowser {
  pub fn loadResource<RetType, T: QTextBrowser_loadResource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loadResource(self);
    // return 1;
  }
}

pub trait QTextBrowser_loadResource<RetType> {
  fn loadResource(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_loadResource<QVariant> for (i32, &'a QUrl) {
  fn loadResource(self , rsthis: & QTextBrowser) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN12QTextBrowser12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QTextBrowser::source();
impl /*struct*/ QTextBrowser {
  pub fn source<RetType, T: QTextBrowser_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QTextBrowser_source<RetType> {
  fn source(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  QUrl QTextBrowser::source();
impl<'a> /*trait*/ QTextBrowser_source<QUrl> for () {
  fn source(self , rsthis: & QTextBrowser) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser6sourceEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
impl /*struct*/ QTextBrowser {
  pub fn setOpenExternalLinks<RetType, T: QTextBrowser_setOpenExternalLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_setOpenExternalLinks<RetType> {
  fn setOpenExternalLinks(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenExternalLinks<()> for (i8) {
  fn setOpenExternalLinks(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser20setOpenExternalLinksEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QTextBrowser20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::setSource(const QUrl & name);
impl /*struct*/ QTextBrowser {
  pub fn setSource<RetType, T: QTextBrowser_setSource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSource(self);
    // return 1;
  }
}

pub trait QTextBrowser_setSource<RetType> {
  fn setSource(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setSource(const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_setSource<()> for (&'a QUrl) {
  fn setSource(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser9setSourceERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QTextBrowser9setSourceERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QTextBrowser::searchPaths();
impl /*struct*/ QTextBrowser {
  pub fn searchPaths<RetType, T: QTextBrowser_searchPaths<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.searchPaths(self);
    // return 1;
  }
}

pub trait QTextBrowser_searchPaths<RetType> {
  fn searchPaths(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  QStringList QTextBrowser::searchPaths();
impl<'a> /*trait*/ QTextBrowser_searchPaths<()> for () {
  fn searchPaths(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser11searchPathsEv()};
     unsafe {C_ZNK12QTextBrowser11searchPathsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::backward();
impl /*struct*/ QTextBrowser {
  pub fn backward<RetType, T: QTextBrowser_backward<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.backward(self);
    // return 1;
  }
}

pub trait QTextBrowser_backward<RetType> {
  fn backward(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::backward();
impl<'a> /*trait*/ QTextBrowser_backward<()> for () {
  fn backward(self , rsthis: & QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser8backwardEv()};
     unsafe {C_ZN12QTextBrowser8backwardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextBrowser::forwardHistoryCount();
impl /*struct*/ QTextBrowser {
  pub fn forwardHistoryCount<RetType, T: QTextBrowser_forwardHistoryCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.forwardHistoryCount(self);
    // return 1;
  }
}

pub trait QTextBrowser_forwardHistoryCount<RetType> {
  fn forwardHistoryCount(self , rsthis: & QTextBrowser) -> RetType;
}

  // proto:  int QTextBrowser::forwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_forwardHistoryCount<i32> for () {
  fn forwardHistoryCount(self , rsthis: & QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19forwardHistoryCountEv()};
    let mut ret = unsafe {C_ZNK12QTextBrowser19forwardHistoryCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

#[derive(Default)] // for QTextBrowser_forwardAvailable
pub struct QTextBrowser_forwardAvailable_signal{poi:u64}
impl /* struct */ QTextBrowser {
  pub fn forwardAvailable(&self) -> QTextBrowser_forwardAvailable_signal {
     return QTextBrowser_forwardAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextBrowser_forwardAvailable_signal {
  pub fn connect<T: QTextBrowser_forwardAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextBrowser_forwardAvailable_signal_connect {
  fn connect(self, sigthis: QTextBrowser_forwardAvailable_signal);
}

#[derive(Default)] // for QTextBrowser_sourceChanged
pub struct QTextBrowser_sourceChanged_signal{poi:u64}
impl /* struct */ QTextBrowser {
  pub fn sourceChanged(&self) -> QTextBrowser_sourceChanged_signal {
     return QTextBrowser_sourceChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextBrowser_sourceChanged_signal {
  pub fn connect<T: QTextBrowser_sourceChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextBrowser_sourceChanged_signal_connect {
  fn connect(self, sigthis: QTextBrowser_sourceChanged_signal);
}

#[derive(Default)] // for QTextBrowser_highlighted
pub struct QTextBrowser_highlighted_signal{poi:u64}
impl /* struct */ QTextBrowser {
  pub fn highlighted(&self) -> QTextBrowser_highlighted_signal {
     return QTextBrowser_highlighted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextBrowser_highlighted_signal {
  pub fn connect<T: QTextBrowser_highlighted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextBrowser_highlighted_signal_connect {
  fn connect(self, sigthis: QTextBrowser_highlighted_signal);
}

#[derive(Default)] // for QTextBrowser_anchorClicked
pub struct QTextBrowser_anchorClicked_signal{poi:u64}
impl /* struct */ QTextBrowser {
  pub fn anchorClicked(&self) -> QTextBrowser_anchorClicked_signal {
     return QTextBrowser_anchorClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextBrowser_anchorClicked_signal {
  pub fn connect<T: QTextBrowser_anchorClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextBrowser_anchorClicked_signal_connect {
  fn connect(self, sigthis: QTextBrowser_anchorClicked_signal);
}

#[derive(Default)] // for QTextBrowser_historyChanged
pub struct QTextBrowser_historyChanged_signal{poi:u64}
impl /* struct */ QTextBrowser {
  pub fn historyChanged(&self) -> QTextBrowser_historyChanged_signal {
     return QTextBrowser_historyChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextBrowser_historyChanged_signal {
  pub fn connect<T: QTextBrowser_historyChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextBrowser_historyChanged_signal_connect {
  fn connect(self, sigthis: QTextBrowser_historyChanged_signal);
}

#[derive(Default)] // for QTextBrowser_backwardAvailable
pub struct QTextBrowser_backwardAvailable_signal{poi:u64}
impl /* struct */ QTextBrowser {
  pub fn backwardAvailable(&self) -> QTextBrowser_backwardAvailable_signal {
     return QTextBrowser_backwardAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextBrowser_backwardAvailable_signal {
  pub fn connect<T: QTextBrowser_backwardAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextBrowser_backwardAvailable_signal_connect {
  fn connect(self, sigthis: QTextBrowser_backwardAvailable_signal);
}

// historyChanged()
extern fn QTextBrowser_historyChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextBrowser_historyChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QTextBrowser_historyChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTextBrowser_historyChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_historyChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser14historyChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_historyChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextBrowser_historyChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_historyChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser14historyChangedEv(arg0, arg1, arg2)};
  }
}
// highlighted(const class QString &)
extern fn QTextBrowser_highlighted_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextBrowser_highlighted_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextBrowser_highlighted_signal_connect for fn(QString) {
  fn connect(self, sigthis: QTextBrowser_highlighted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_highlighted_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser11highlightedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_highlighted_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QTextBrowser_highlighted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_highlighted_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser11highlightedERK7QString(arg0, arg1, arg2)};
  }
}
// sourceChanged(const class QUrl &)
extern fn QTextBrowser_sourceChanged_signal_connect_cb_2(rsfptr:fn(QUrl), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextBrowser_sourceChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QUrl)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextBrowser_sourceChanged_signal_connect for fn(QUrl) {
  fn connect(self, sigthis: QTextBrowser_sourceChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_sourceChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser13sourceChangedERK4QUrl(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_sourceChanged_signal_connect for Box<Fn(QUrl)> {
  fn connect(self, sigthis: QTextBrowser_sourceChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_sourceChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser13sourceChangedERK4QUrl(arg0, arg1, arg2)};
  }
}
// highlighted(const class QUrl &)
extern fn QTextBrowser_highlighted_signal_connect_cb_3(rsfptr:fn(QUrl), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextBrowser_highlighted_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(QUrl)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextBrowser_highlighted_signal_connect for fn(QUrl) {
  fn connect(self, sigthis: QTextBrowser_highlighted_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_highlighted_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser11highlightedERK4QUrl(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_highlighted_signal_connect for Box<Fn(QUrl)> {
  fn connect(self, sigthis: QTextBrowser_highlighted_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_highlighted_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser11highlightedERK4QUrl(arg0, arg1, arg2)};
  }
}
// forwardAvailable(_Bool)
extern fn QTextBrowser_forwardAvailable_signal_connect_cb_4(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextBrowser_forwardAvailable_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextBrowser_forwardAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextBrowser_forwardAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_forwardAvailable_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser16forwardAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_forwardAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextBrowser_forwardAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_forwardAvailable_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser16forwardAvailableEb(arg0, arg1, arg2)};
  }
}
// anchorClicked(const class QUrl &)
extern fn QTextBrowser_anchorClicked_signal_connect_cb_5(rsfptr:fn(QUrl), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextBrowser_anchorClicked_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(QUrl)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextBrowser_anchorClicked_signal_connect for fn(QUrl) {
  fn connect(self, sigthis: QTextBrowser_anchorClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_anchorClicked_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser13anchorClickedERK4QUrl(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_anchorClicked_signal_connect for Box<Fn(QUrl)> {
  fn connect(self, sigthis: QTextBrowser_anchorClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_anchorClicked_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser13anchorClickedERK4QUrl(arg0, arg1, arg2)};
  }
}
// backwardAvailable(_Bool)
extern fn QTextBrowser_backwardAvailable_signal_connect_cb_6(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextBrowser_backwardAvailable_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextBrowser_backwardAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextBrowser_backwardAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_backwardAvailable_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser17backwardAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextBrowser_backwardAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextBrowser_backwardAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextBrowser_backwardAvailable_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextBrowser_SlotProxy_connect__ZN12QTextBrowser17backwardAvailableEb(arg0, arg1, arg2)};
  }
}
// <= body block end

