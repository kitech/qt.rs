// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qwizard.h
// dst-file: /src/widgets/qwizard.rs
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qstring::*; // 771
use super::super::gui::qpixmap::*; // 771
use super::qdialog::*; // 773
// use super::qwizard::QWizardPage; // 773
use super::super::core::qvariant::*; // 771
use super::super::core::qsize::*; // 771
use super::qabstractbutton::*; // 773
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QWizardPage_Class_Size() -> c_int;
  // proto:  const QMetaObject * QWizardPage::metaObject();
  fn C_ZNK11QWizardPage10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QWizardPage::title();
  fn C_ZNK11QWizardPage5titleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QWizardPage::subTitle();
  fn C_ZNK11QWizardPage8subTitleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QWizardPage::isFinalPage();
  fn C_ZNK11QWizardPage11isFinalPageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QWizardPage::validatePage();
  fn C_ZN11QWizardPage12validatePageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QWizardPage::nextId();
  fn C_ZNK11QWizardPage6nextIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWizardPage::cleanupPage();
  fn C_ZN11QWizardPage11cleanupPageEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QWizardPage::isComplete();
  fn C_ZNK11QWizardPage10isCompleteEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QWizardPage::isCommitPage();
  fn C_ZNK11QWizardPage12isCommitPageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QWizardPage::QWizardPage(QWidget * parent);
  fn C_ZN11QWizardPageC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QWizardPage::setFinalPage(bool finalPage);
  fn C_ZN11QWizardPage12setFinalPageEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
  fn C_ZN11QWizardPage11setSubTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QWizardPage::~QWizardPage();
  fn C_ZN11QWizardPageD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QWizardPage::setCommitPage(bool commitPage);
  fn C_ZN11QWizardPage13setCommitPageEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QWizardPage::initializePage();
  fn C_ZN11QWizardPage14initializePageEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWizardPage::setTitle(const QString & title);
  fn C_ZN11QWizardPage8setTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QWizard_Class_Size() -> c_int;
  // proto:  void QWizard::setSideWidget(QWidget * widget);
  fn C_ZN7QWizard13setSideWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWizardPage * QWizard::currentPage();
  fn C_ZNK7QWizard11currentPageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWizard::next();
  fn C_ZN7QWizard4nextEv(qthis: u64 /* *mut c_void*/);
  // proto:  QWizardPage * QWizard::page(int id);
  fn C_ZNK7QWizard4pageEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QWizard::metaObject();
  fn C_ZNK7QWizard10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
  fn C_ZN7QWizard8setFieldERK7QStringRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QWizard::setPage(int id, QWizardPage * page);
  fn C_ZN7QWizard7setPageEiP11QWizardPage(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QWizard::restart();
  fn C_ZN7QWizard7restartEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QWizard::back();
  fn C_ZN7QWizard4backEv(qthis: u64 /* *mut c_void*/);
  // proto:  QSize QWizard::sizeHint();
  fn C_ZNK7QWizard8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
  fn C_ZN7QWizard18setDefaultPropertyEPKcS1_S1_(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: *mut c_char, arg2: *mut c_char);
  // proto:  void QWizard::setStartId(int id);
  fn C_ZN7QWizard10setStartIdEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QWizard::~QWizard();
  fn C_ZN7QWizardD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<int> QWizard::visitedPages();
  fn C_ZNK7QWizard12visitedPagesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWizard::nextId();
  fn C_ZNK7QWizard6nextIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QWizard::startId();
  fn C_ZNK7QWizard7startIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QWizard::addPage(QWizardPage * page);
  fn C_ZN7QWizard7addPageEP11QWizardPage(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QWizard::removePage(int id);
  fn C_ZN7QWizard10removePageEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QList<int> QWizard::pageIds();
  fn C_ZNK7QWizard7pageIdsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QWizard::currentId();
  fn C_ZNK7QWizard9currentIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QWizard::setVisible(bool visible);
  fn C_ZN7QWizard10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QWizard::hasVisitedPage(int id);
  fn C_ZNK7QWizard14hasVisitedPageEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  QVariant QWizard::field(const QString & name);
  fn C_ZNK7QWizard5fieldERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QWizard::validateCurrentPage();
  fn C_ZN7QWizard19validateCurrentPageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QWidget * QWizard::sideWidget();
  fn C_ZNK7QWizard10sideWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QWizardPage_SlotProxy_connect__ZN11QWizardPage15completeChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWizard_SlotProxy_connect__ZN7QWizard13helpRequestedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWizard_SlotProxy_connect__ZN7QWizard16currentIdChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWizard_SlotProxy_connect__ZN7QWizard11pageRemovedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWizard_SlotProxy_connect__ZN7QWizard19customButtonClickedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QWizard_SlotProxy_connect__ZN7QWizard9pageAddedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QWizardPage)=1
#[derive(Default)]
pub struct QWizardPage {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _completeChanged: QWizardPage_completeChanged_signal,
}

// class sizeof(QWizard)=1
#[derive(Default)]
pub struct QWizard {
  qbase: QDialog,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _pageRemoved: QWizard_pageRemoved_signal,
  pub _pageAdded: QWizard_pageAdded_signal,
  pub _helpRequested: QWizard_helpRequested_signal,
  pub _currentIdChanged: QWizard_currentIdChanged_signal,
  pub _customButtonClicked: QWizard_customButtonClicked_signal,
}

impl /*struct*/ QWizardPage {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWizardPage {
    return QWizardPage{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWizardPage {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QWizardPage {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QWizardPage::metaObject();
impl /*struct*/ QWizardPage {
  pub fn metaObject<RetType, T: QWizardPage_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWizardPage_metaObject<RetType> {
  fn metaObject(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  const QMetaObject * QWizardPage::metaObject();
impl<'a> /*trait*/ QWizardPage_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QWizardPage) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWizardPage::title();
impl /*struct*/ QWizardPage {
  pub fn title<RetType, T: QWizardPage_title<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QWizardPage_title<RetType> {
  fn title(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  QString QWizardPage::title();
impl<'a> /*trait*/ QWizardPage_title<QString> for () {
  fn title(self , rsthis: & QWizardPage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage5titleEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWizardPage::subTitle();
impl /*struct*/ QWizardPage {
  pub fn subTitle<RetType, T: QWizardPage_subTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_subTitle<RetType> {
  fn subTitle(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  QString QWizardPage::subTitle();
impl<'a> /*trait*/ QWizardPage_subTitle<QString> for () {
  fn subTitle(self , rsthis: & QWizardPage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage8subTitleEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage8subTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWizardPage::isFinalPage();
impl /*struct*/ QWizardPage {
  pub fn isFinalPage<RetType, T: QWizardPage_isFinalPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isFinalPage(self);
    // return 1;
  }
}

pub trait QWizardPage_isFinalPage<RetType> {
  fn isFinalPage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::isFinalPage();
impl<'a> /*trait*/ QWizardPage_isFinalPage<i8> for () {
  fn isFinalPage(self , rsthis: & QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage11isFinalPageEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage11isFinalPageEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QWizardPage::validatePage();
impl /*struct*/ QWizardPage {
  pub fn validatePage<RetType, T: QWizardPage_validatePage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.validatePage(self);
    // return 1;
  }
}

pub trait QWizardPage_validatePage<RetType> {
  fn validatePage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::validatePage();
impl<'a> /*trait*/ QWizardPage_validatePage<i8> for () {
  fn validatePage(self , rsthis: & QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12validatePageEv()};
    let mut ret = unsafe {C_ZN11QWizardPage12validatePageEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QWizardPage::nextId();
impl /*struct*/ QWizardPage {
  pub fn nextId<RetType, T: QWizardPage_nextId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextId(self);
    // return 1;
  }
}

pub trait QWizardPage_nextId<RetType> {
  fn nextId(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  int QWizardPage::nextId();
impl<'a> /*trait*/ QWizardPage_nextId<i32> for () {
  fn nextId(self , rsthis: & QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage6nextIdEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage6nextIdEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWizardPage::cleanupPage();
impl /*struct*/ QWizardPage {
  pub fn cleanupPage<RetType, T: QWizardPage_cleanupPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanupPage(self);
    // return 1;
  }
}

pub trait QWizardPage_cleanupPage<RetType> {
  fn cleanupPage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::cleanupPage();
impl<'a> /*trait*/ QWizardPage_cleanupPage<()> for () {
  fn cleanupPage(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11cleanupPageEv()};
     unsafe {C_ZN11QWizardPage11cleanupPageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWizardPage::isComplete();
impl /*struct*/ QWizardPage {
  pub fn isComplete<RetType, T: QWizardPage_isComplete<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isComplete(self);
    // return 1;
  }
}

pub trait QWizardPage_isComplete<RetType> {
  fn isComplete(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::isComplete();
impl<'a> /*trait*/ QWizardPage_isComplete<i8> for () {
  fn isComplete(self , rsthis: & QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10isCompleteEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage10isCompleteEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QWizardPage::isCommitPage();
impl /*struct*/ QWizardPage {
  pub fn isCommitPage<RetType, T: QWizardPage_isCommitPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCommitPage(self);
    // return 1;
  }
}

pub trait QWizardPage_isCommitPage<RetType> {
  fn isCommitPage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::isCommitPage();
impl<'a> /*trait*/ QWizardPage_isCommitPage<i8> for () {
  fn isCommitPage(self , rsthis: & QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage12isCommitPageEv()};
    let mut ret = unsafe {C_ZNK11QWizardPage12isCommitPageEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QWizardPage::QWizardPage(QWidget * parent);
impl /*struct*/ QWizardPage {
  pub fn new<T: QWizardPage_new>(value: T) -> QWizardPage {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QWizardPage_new {
  fn new(self) -> QWizardPage;
}

  // proto:  void QWizardPage::QWizardPage(QWidget * parent);
impl<'a> /*trait*/ QWizardPage_new for (Option<&'a QWidget>) {
  fn new(self) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageC2EP7QWidget()};
    let ctysz: c_int = unsafe{QWizardPage_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QWizardPageC2EP7QWidget(arg0)};
    let rsthis = QWizardPage{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWizardPage::setFinalPage(bool finalPage);
impl /*struct*/ QWizardPage {
  pub fn setFinalPage<RetType, T: QWizardPage_setFinalPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFinalPage(self);
    // return 1;
  }
}

pub trait QWizardPage_setFinalPage<RetType> {
  fn setFinalPage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setFinalPage(bool finalPage);
impl<'a> /*trait*/ QWizardPage_setFinalPage<()> for (i8) {
  fn setFinalPage(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12setFinalPageEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QWizardPage12setFinalPageEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
impl /*struct*/ QWizardPage {
  pub fn setSubTitle<RetType, T: QWizardPage_setSubTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSubTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_setSubTitle<RetType> {
  fn setSubTitle(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
impl<'a> /*trait*/ QWizardPage_setSubTitle<()> for (&'a QString) {
  fn setSubTitle(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11setSubTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QWizardPage11setSubTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizardPage::~QWizardPage();
impl /*struct*/ QWizardPage {
  pub fn free<RetType, T: QWizardPage_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWizardPage_free<RetType> {
  fn free(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::~QWizardPage();
impl<'a> /*trait*/ QWizardPage_free<()> for () {
  fn free(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageD2Ev()};
     unsafe {C_ZN11QWizardPageD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizardPage::setCommitPage(bool commitPage);
impl /*struct*/ QWizardPage {
  pub fn setCommitPage<RetType, T: QWizardPage_setCommitPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCommitPage(self);
    // return 1;
  }
}

pub trait QWizardPage_setCommitPage<RetType> {
  fn setCommitPage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setCommitPage(bool commitPage);
impl<'a> /*trait*/ QWizardPage_setCommitPage<()> for (i8) {
  fn setCommitPage(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage13setCommitPageEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN11QWizardPage13setCommitPageEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizardPage::initializePage();
impl /*struct*/ QWizardPage {
  pub fn initializePage<RetType, T: QWizardPage_initializePage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initializePage(self);
    // return 1;
  }
}

pub trait QWizardPage_initializePage<RetType> {
  fn initializePage(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::initializePage();
impl<'a> /*trait*/ QWizardPage_initializePage<()> for () {
  fn initializePage(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage14initializePageEv()};
     unsafe {C_ZN11QWizardPage14initializePageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizardPage::setTitle(const QString & title);
impl /*struct*/ QWizardPage {
  pub fn setTitle<RetType, T: QWizardPage_setTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_setTitle<RetType> {
  fn setTitle(self , rsthis: & QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setTitle(const QString & title);
impl<'a> /*trait*/ QWizardPage_setTitle<()> for (&'a QString) {
  fn setTitle(self , rsthis: & QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QWizardPage8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QWizard {
    return QWizard{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QWizard {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QWizard {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  void QWizard::setSideWidget(QWidget * widget);
impl /*struct*/ QWizard {
  pub fn setSideWidget<RetType, T: QWizard_setSideWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSideWidget(self);
    // return 1;
  }
}

pub trait QWizard_setSideWidget<RetType> {
  fn setSideWidget(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::setSideWidget(QWidget * widget);
impl<'a> /*trait*/ QWizard_setSideWidget<()> for (&'a QWidget) {
  fn setSideWidget(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13setSideWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWizard13setSideWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWizardPage * QWizard::currentPage();
impl /*struct*/ QWizard {
  pub fn currentPage<RetType, T: QWizard_currentPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentPage(self);
    // return 1;
  }
}

pub trait QWizard_currentPage<RetType> {
  fn currentPage(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QWizardPage * QWizard::currentPage();
impl<'a> /*trait*/ QWizard_currentPage<QWizardPage> for () {
  fn currentPage(self , rsthis: & QWizard) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard11currentPageEv()};
    let mut ret = unsafe {C_ZNK7QWizard11currentPageEv(rsthis.qclsinst)};
    let mut ret1 = QWizardPage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWizard::next();
impl /*struct*/ QWizard {
  pub fn next<RetType, T: QWizard_next<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QWizard_next<RetType> {
  fn next(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::next();
impl<'a> /*trait*/ QWizard_next<()> for () {
  fn next(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4nextEv()};
     unsafe {C_ZN7QWizard4nextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWizardPage * QWizard::page(int id);
impl /*struct*/ QWizard {
  pub fn page<RetType, T: QWizard_page<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.page(self);
    // return 1;
  }
}

pub trait QWizard_page<RetType> {
  fn page(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QWizardPage * QWizard::page(int id);
impl<'a> /*trait*/ QWizard_page<QWizardPage> for (i32) {
  fn page(self , rsthis: & QWizard) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard4pageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QWizard4pageEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWizardPage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QWizard::metaObject();
impl /*struct*/ QWizard {
  pub fn metaObject<RetType, T: QWizard_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWizard_metaObject<RetType> {
  fn metaObject(self , rsthis: & QWizard) -> RetType;
}

  // proto:  const QMetaObject * QWizard::metaObject();
impl<'a> /*trait*/ QWizard_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QWizard) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10metaObjectEv()};
    let mut ret = unsafe {C_ZNK7QWizard10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
impl /*struct*/ QWizard {
  pub fn setField<RetType, T: QWizard_setField<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setField(self);
    // return 1;
  }
}

pub trait QWizard_setField<RetType> {
  fn setField(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
impl<'a> /*trait*/ QWizard_setField<()> for (&'a QString, &'a QVariant) {
  fn setField(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard8setFieldERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWizard8setFieldERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWizard::setPage(int id, QWizardPage * page);
impl /*struct*/ QWizard {
  pub fn setPage<RetType, T: QWizard_setPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPage(self);
    // return 1;
  }
}

pub trait QWizard_setPage<RetType> {
  fn setPage(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::setPage(int id, QWizardPage * page);
impl<'a> /*trait*/ QWizard_setPage<()> for (i32, &'a QWizardPage) {
  fn setPage(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7setPageEiP11QWizardPage()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QWizard7setPageEiP11QWizardPage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWizard::restart();
impl /*struct*/ QWizard {
  pub fn restart<RetType, T: QWizard_restart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restart(self);
    // return 1;
  }
}

pub trait QWizard_restart<RetType> {
  fn restart(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::restart();
impl<'a> /*trait*/ QWizard_restart<()> for () {
  fn restart(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7restartEv()};
     unsafe {C_ZN7QWizard7restartEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizard::back();
impl /*struct*/ QWizard {
  pub fn back<RetType, T: QWizard_back<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.back(self);
    // return 1;
  }
}

pub trait QWizard_back<RetType> {
  fn back(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::back();
impl<'a> /*trait*/ QWizard_back<()> for () {
  fn back(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4backEv()};
     unsafe {C_ZN7QWizard4backEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QWizard::sizeHint();
impl /*struct*/ QWizard {
  pub fn sizeHint<RetType, T: QWizard_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWizard_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QSize QWizard::sizeHint();
impl<'a> /*trait*/ QWizard_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QWizard) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard8sizeHintEv()};
    let mut ret = unsafe {C_ZNK7QWizard8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
impl /*struct*/ QWizard {
  pub fn setDefaultProperty<RetType, T: QWizard_setDefaultProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultProperty(self);
    // return 1;
  }
}

pub trait QWizard_setDefaultProperty<RetType> {
  fn setDefaultProperty(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
impl<'a> /*trait*/ QWizard_setDefaultProperty<()> for (&'a  String, &'a  String, &'a  String) {
  fn setDefaultProperty(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard18setDefaultPropertyEPKcS1_S1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {C_ZN7QWizard18setDefaultPropertyEPKcS1_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QWizard::setStartId(int id);
impl /*struct*/ QWizard {
  pub fn setStartId<RetType, T: QWizard_setStartId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartId(self);
    // return 1;
  }
}

pub trait QWizard_setStartId<RetType> {
  fn setStartId(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::setStartId(int id);
impl<'a> /*trait*/ QWizard_setStartId<()> for (i32) {
  fn setStartId(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setStartIdEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWizard10setStartIdEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizard::~QWizard();
impl /*struct*/ QWizard {
  pub fn free<RetType, T: QWizard_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QWizard_free<RetType> {
  fn free(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::~QWizard();
impl<'a> /*trait*/ QWizard_free<()> for () {
  fn free(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizardD2Ev()};
     unsafe {C_ZN7QWizardD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<int> QWizard::visitedPages();
impl /*struct*/ QWizard {
  pub fn visitedPages<RetType, T: QWizard_visitedPages<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.visitedPages(self);
    // return 1;
  }
}

pub trait QWizard_visitedPages<RetType> {
  fn visitedPages(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QList<int> QWizard::visitedPages();
impl<'a> /*trait*/ QWizard_visitedPages<u64> for () {
  fn visitedPages(self , rsthis: & QWizard) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard12visitedPagesEv()};
    let mut ret = unsafe {C_ZNK7QWizard12visitedPagesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QWizard::nextId();
impl /*struct*/ QWizard {
  pub fn nextId<RetType, T: QWizard_nextId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nextId(self);
    // return 1;
  }
}

pub trait QWizard_nextId<RetType> {
  fn nextId(self , rsthis: & QWizard) -> RetType;
}

  // proto:  int QWizard::nextId();
impl<'a> /*trait*/ QWizard_nextId<i32> for () {
  fn nextId(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard6nextIdEv()};
    let mut ret = unsafe {C_ZNK7QWizard6nextIdEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QWizard::startId();
impl /*struct*/ QWizard {
  pub fn startId<RetType, T: QWizard_startId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startId(self);
    // return 1;
  }
}

pub trait QWizard_startId<RetType> {
  fn startId(self , rsthis: & QWizard) -> RetType;
}

  // proto:  int QWizard::startId();
impl<'a> /*trait*/ QWizard_startId<i32> for () {
  fn startId(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7startIdEv()};
    let mut ret = unsafe {C_ZNK7QWizard7startIdEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QWizard::addPage(QWizardPage * page);
impl /*struct*/ QWizard {
  pub fn addPage<RetType, T: QWizard_addPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPage(self);
    // return 1;
  }
}

pub trait QWizard_addPage<RetType> {
  fn addPage(self , rsthis: & QWizard) -> RetType;
}

  // proto:  int QWizard::addPage(QWizardPage * page);
impl<'a> /*trait*/ QWizard_addPage<i32> for (&'a QWizardPage) {
  fn addPage(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7addPageEP11QWizardPage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QWizard7addPageEP11QWizardPage(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWizard::removePage(int id);
impl /*struct*/ QWizard {
  pub fn removePage<RetType, T: QWizard_removePage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removePage(self);
    // return 1;
  }
}

pub trait QWizard_removePage<RetType> {
  fn removePage(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::removePage(int id);
impl<'a> /*trait*/ QWizard_removePage<()> for (i32) {
  fn removePage(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10removePageEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QWizard10removePageEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<int> QWizard::pageIds();
impl /*struct*/ QWizard {
  pub fn pageIds<RetType, T: QWizard_pageIds<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pageIds(self);
    // return 1;
  }
}

pub trait QWizard_pageIds<RetType> {
  fn pageIds(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QList<int> QWizard::pageIds();
impl<'a> /*trait*/ QWizard_pageIds<u64> for () {
  fn pageIds(self , rsthis: & QWizard) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7pageIdsEv()};
    let mut ret = unsafe {C_ZNK7QWizard7pageIdsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  int QWizard::currentId();
impl /*struct*/ QWizard {
  pub fn currentId<RetType, T: QWizard_currentId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentId(self);
    // return 1;
  }
}

pub trait QWizard_currentId<RetType> {
  fn currentId(self , rsthis: & QWizard) -> RetType;
}

  // proto:  int QWizard::currentId();
impl<'a> /*trait*/ QWizard_currentId<i32> for () {
  fn currentId(self , rsthis: & QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard9currentIdEv()};
    let mut ret = unsafe {C_ZNK7QWizard9currentIdEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QWizard::setVisible(bool visible);
impl /*struct*/ QWizard {
  pub fn setVisible<RetType, T: QWizard_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QWizard_setVisible<RetType> {
  fn setVisible(self , rsthis: & QWizard) -> RetType;
}

  // proto:  void QWizard::setVisible(bool visible);
impl<'a> /*trait*/ QWizard_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QWizard10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWizard::hasVisitedPage(int id);
impl /*struct*/ QWizard {
  pub fn hasVisitedPage<RetType, T: QWizard_hasVisitedPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasVisitedPage(self);
    // return 1;
  }
}

pub trait QWizard_hasVisitedPage<RetType> {
  fn hasVisitedPage(self , rsthis: & QWizard) -> RetType;
}

  // proto:  bool QWizard::hasVisitedPage(int id);
impl<'a> /*trait*/ QWizard_hasVisitedPage<i8> for (i32) {
  fn hasVisitedPage(self , rsthis: & QWizard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard14hasVisitedPageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QWizard14hasVisitedPageEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QVariant QWizard::field(const QString & name);
impl /*struct*/ QWizard {
  pub fn field<RetType, T: QWizard_field<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.field(self);
    // return 1;
  }
}

pub trait QWizard_field<RetType> {
  fn field(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QVariant QWizard::field(const QString & name);
impl<'a> /*trait*/ QWizard_field<QVariant> for (&'a QString) {
  fn field(self , rsthis: & QWizard) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard5fieldERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QWizard5fieldERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWizard::validateCurrentPage();
impl /*struct*/ QWizard {
  pub fn validateCurrentPage<RetType, T: QWizard_validateCurrentPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.validateCurrentPage(self);
    // return 1;
  }
}

pub trait QWizard_validateCurrentPage<RetType> {
  fn validateCurrentPage(self , rsthis: & QWizard) -> RetType;
}

  // proto:  bool QWizard::validateCurrentPage();
impl<'a> /*trait*/ QWizard_validateCurrentPage<i8> for () {
  fn validateCurrentPage(self , rsthis: & QWizard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19validateCurrentPageEv()};
    let mut ret = unsafe {C_ZN7QWizard19validateCurrentPageEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QWidget * QWizard::sideWidget();
impl /*struct*/ QWizard {
  pub fn sideWidget<RetType, T: QWizard_sideWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sideWidget(self);
    // return 1;
  }
}

pub trait QWizard_sideWidget<RetType> {
  fn sideWidget(self , rsthis: & QWizard) -> RetType;
}

  // proto:  QWidget * QWizard::sideWidget();
impl<'a> /*trait*/ QWizard_sideWidget<QWidget> for () {
  fn sideWidget(self , rsthis: & QWizard) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10sideWidgetEv()};
    let mut ret = unsafe {C_ZNK7QWizard10sideWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QWizardPage_completeChanged
pub struct QWizardPage_completeChanged_signal{poi:u64}
impl /* struct */ QWizardPage {
  pub fn completeChanged(&self) -> QWizardPage_completeChanged_signal {
     return QWizardPage_completeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWizardPage_completeChanged_signal {
  pub fn connect<T: QWizardPage_completeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWizardPage_completeChanged_signal_connect {
  fn connect(self, sigthis: QWizardPage_completeChanged_signal);
}

// completeChanged()
extern fn QWizardPage_completeChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QWizardPage_completeChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QWizardPage_completeChanged_signal_connect for fn() {
  fn connect(self, sigthis: QWizardPage_completeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizardPage_completeChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWizardPage_SlotProxy_connect__ZN11QWizardPage15completeChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWizardPage_completeChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QWizardPage_completeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizardPage_completeChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWizardPage_SlotProxy_connect__ZN11QWizardPage15completeChangedEv(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QWizard_pageRemoved
pub struct QWizard_pageRemoved_signal{poi:u64}
impl /* struct */ QWizard {
  pub fn pageRemoved(&self) -> QWizard_pageRemoved_signal {
     return QWizard_pageRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWizard_pageRemoved_signal {
  pub fn connect<T: QWizard_pageRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWizard_pageRemoved_signal_connect {
  fn connect(self, sigthis: QWizard_pageRemoved_signal);
}

#[derive(Default)] // for QWizard_pageAdded
pub struct QWizard_pageAdded_signal{poi:u64}
impl /* struct */ QWizard {
  pub fn pageAdded(&self) -> QWizard_pageAdded_signal {
     return QWizard_pageAdded_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWizard_pageAdded_signal {
  pub fn connect<T: QWizard_pageAdded_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWizard_pageAdded_signal_connect {
  fn connect(self, sigthis: QWizard_pageAdded_signal);
}

#[derive(Default)] // for QWizard_helpRequested
pub struct QWizard_helpRequested_signal{poi:u64}
impl /* struct */ QWizard {
  pub fn helpRequested(&self) -> QWizard_helpRequested_signal {
     return QWizard_helpRequested_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWizard_helpRequested_signal {
  pub fn connect<T: QWizard_helpRequested_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWizard_helpRequested_signal_connect {
  fn connect(self, sigthis: QWizard_helpRequested_signal);
}

#[derive(Default)] // for QWizard_currentIdChanged
pub struct QWizard_currentIdChanged_signal{poi:u64}
impl /* struct */ QWizard {
  pub fn currentIdChanged(&self) -> QWizard_currentIdChanged_signal {
     return QWizard_currentIdChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWizard_currentIdChanged_signal {
  pub fn connect<T: QWizard_currentIdChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWizard_currentIdChanged_signal_connect {
  fn connect(self, sigthis: QWizard_currentIdChanged_signal);
}

#[derive(Default)] // for QWizard_customButtonClicked
pub struct QWizard_customButtonClicked_signal{poi:u64}
impl /* struct */ QWizard {
  pub fn customButtonClicked(&self) -> QWizard_customButtonClicked_signal {
     return QWizard_customButtonClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QWizard_customButtonClicked_signal {
  pub fn connect<T: QWizard_customButtonClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QWizard_customButtonClicked_signal_connect {
  fn connect(self, sigthis: QWizard_customButtonClicked_signal);
}

// helpRequested()
extern fn QWizard_helpRequested_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QWizard_helpRequested_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QWizard_helpRequested_signal_connect for fn() {
  fn connect(self, sigthis: QWizard_helpRequested_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_helpRequested_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard13helpRequestedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWizard_helpRequested_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QWizard_helpRequested_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_helpRequested_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard13helpRequestedEv(arg0, arg1, arg2)};
  }
}
// currentIdChanged(int)
extern fn QWizard_currentIdChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWizard_currentIdChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWizard_currentIdChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWizard_currentIdChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_currentIdChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard16currentIdChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWizard_currentIdChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWizard_currentIdChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_currentIdChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard16currentIdChangedEi(arg0, arg1, arg2)};
  }
}
// pageRemoved(int)
extern fn QWizard_pageRemoved_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWizard_pageRemoved_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWizard_pageRemoved_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWizard_pageRemoved_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_pageRemoved_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard11pageRemovedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWizard_pageRemoved_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWizard_pageRemoved_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_pageRemoved_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard11pageRemovedEi(arg0, arg1, arg2)};
  }
}
// customButtonClicked(int)
extern fn QWizard_customButtonClicked_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWizard_customButtonClicked_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWizard_customButtonClicked_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWizard_customButtonClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_customButtonClicked_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard19customButtonClickedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWizard_customButtonClicked_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWizard_customButtonClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_customButtonClicked_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard19customButtonClickedEi(arg0, arg1, arg2)};
  }
}
// pageAdded(int)
extern fn QWizard_pageAdded_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QWizard_pageAdded_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QWizard_pageAdded_signal_connect for fn(i32) {
  fn connect(self, sigthis: QWizard_pageAdded_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_pageAdded_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard9pageAddedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QWizard_pageAdded_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QWizard_pageAdded_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QWizard_pageAdded_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QWizard_SlotProxy_connect__ZN7QWizard9pageAddedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

