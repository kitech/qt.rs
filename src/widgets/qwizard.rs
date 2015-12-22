// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::gui::qpixmap::QPixmap; // 771
use super::qdialog::QDialog; // 773
// use super::qwizard::QWizardPage; // 773
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QWizardPage::QWizardPage(const QWizardPage & );
  fn _ZN11QWizardPageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QWizardPage::metaObject();
  fn _ZNK11QWizardPage10metaObjectEv(qthis: *mut c_void);
  // proto:  QString QWizardPage::title();
  fn _ZNK11QWizardPage5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWizardPage::subTitle();
  fn _ZNK11QWizardPage8subTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWizardPage::isFinalPage();
  fn _ZNK11QWizardPage11isFinalPageEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QWizardPage::validatePage();
  fn _ZN11QWizardPage12validatePageEv(qthis: *mut c_void) -> c_char;
  // proto:  int QWizardPage::nextId();
  fn _ZNK11QWizardPage6nextIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWizardPage::cleanupPage();
  fn _ZN11QWizardPage11cleanupPageEv(qthis: *mut c_void);
  // proto:  bool QWizardPage::isComplete();
  fn _ZNK11QWizardPage10isCompleteEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWizardPage::completeChanged();
  fn _ZN11QWizardPage15completeChangedEv(qthis: *mut c_void);
  // proto:  bool QWizardPage::isCommitPage();
  fn _ZNK11QWizardPage12isCommitPageEv(qthis: *mut c_void) -> c_char;
  // proto:  void QWizardPage::QWizardPage(QWidget * parent);
  fn _ZN11QWizardPageC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWizardPage::setFinalPage(bool finalPage);
  fn _ZN11QWizardPage12setFinalPageEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
  fn _ZN11QWizardPage11setSubTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWizardPage::~QWizardPage();
  fn _ZN11QWizardPageD0Ev(qthis: *mut c_void);
  // proto:  void QWizardPage::setCommitPage(bool commitPage);
  fn _ZN11QWizardPage13setCommitPageEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QWizardPage::initializePage();
  fn _ZN11QWizardPage14initializePageEv(qthis: *mut c_void);
  // proto:  void QWizardPage::setTitle(const QString & title);
  fn _ZN11QWizardPage8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QWizard::setSideWidget(QWidget * widget);
  fn _ZN7QWizard13setSideWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWizardPage * QWizard::currentPage();
  fn _ZNK7QWizard11currentPageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWizard::next();
  fn _ZN7QWizard4nextEv(qthis: *mut c_void);
  // proto:  void QWizard::helpRequested();
  fn _ZN7QWizard13helpRequestedEv(qthis: *mut c_void);
  // proto:  QWizardPage * QWizard::page(int id);
  fn _ZNK7QWizard4pageEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QWizard::metaObject();
  fn _ZNK7QWizard10metaObjectEv(qthis: *mut c_void);
  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
  fn _ZN7QWizard8setFieldERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QWizard::setPage(int id, QWizardPage * page);
  fn _ZN7QWizard7setPageEiP11QWizardPage(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QWizard::restart();
  fn _ZN7QWizard7restartEv(qthis: *mut c_void);
  // proto:  void QWizard::back();
  fn _ZN7QWizard4backEv(qthis: *mut c_void);
  // proto:  QSize QWizard::sizeHint();
  fn _ZNK7QWizard8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
  fn _ZN7QWizard18setDefaultPropertyEPKcS1_S1_(qthis: *mut c_void, arg0: *mut c_char, arg1: *mut c_char, arg2: *mut c_char);
  // proto:  void QWizard::setStartId(int id);
  fn _ZN7QWizard10setStartIdEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWizard::currentIdChanged(int id);
  fn _ZN7QWizard16currentIdChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWizard::~QWizard();
  fn _ZN7QWizardD0Ev(qthis: *mut c_void);
  // proto:  void QWizard::pageAdded(int id);
  fn _ZN7QWizard9pageAddedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QList<int> QWizard::visitedPages();
  fn _ZNK7QWizard12visitedPagesEv(qthis: *mut c_void);
  // proto:  int QWizard::nextId();
  fn _ZNK7QWizard6nextIdEv(qthis: *mut c_void) -> c_int;
  // proto:  int QWizard::startId();
  fn _ZNK7QWizard7startIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWizard::customButtonClicked(int which);
  fn _ZN7QWizard19customButtonClickedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWizard::QWizard(const QWizard & );
  fn _ZN7QWizardC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QWizard::addPage(QWizardPage * page);
  fn _ZN7QWizard7addPageEP11QWizardPage(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QWizard::removePage(int id);
  fn _ZN7QWizard10removePageEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QList<int> QWizard::pageIds();
  fn _ZNK7QWizard7pageIdsEv(qthis: *mut c_void);
  // proto:  int QWizard::currentId();
  fn _ZNK7QWizard9currentIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWizard::pageRemoved(int id);
  fn _ZN7QWizard11pageRemovedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QWizard::setVisible(bool visible);
  fn _ZN7QWizard10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QWizard::hasVisitedPage(int id);
  fn _ZNK7QWizard14hasVisitedPageEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QVariant QWizard::field(const QString & name);
  fn _ZNK7QWizard5fieldERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QWizard::validateCurrentPage();
  fn _ZN7QWizard19validateCurrentPageEv(qthis: *mut c_void) -> c_char;
  // proto:  QWidget * QWizard::sideWidget();
  fn _ZNK7QWizard10sideWidgetEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QWizardPage)=1
pub struct QWizardPage {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

// class sizeof(QWizard)=1
pub struct QWizard {
  qbase: QDialog,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWizardPage {
  pub fn inheritFrom(qthis: *mut c_void) -> QWizardPage {
    return QWizardPage{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWizardPage {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return &self.qbase;
  }
}
impl AsRef<QWidget> for QWizardPage {
  fn as_ref(&self) -> &QWidget {
    return &self.qbase;
  }
}
  // proto:  void QWizardPage::QWizardPage(const QWizardPage & );
impl /*struct*/ QWizardPage {
  pub fn NewQWizardPage<T: QWizardPage_NewQWizardPage>(value: T) -> QWizardPage {
    let rsthis = value.NewQWizardPage();
    return rsthis;
    // return 1;
  }
}

pub trait QWizardPage_NewQWizardPage {
  fn NewQWizardPage(self) -> QWizardPage;
}

  // proto:  void QWizardPage::QWizardPage(const QWizardPage & );
impl<'a> /*trait*/ QWizardPage_NewQWizardPage for (QWizardPage) {
  fn NewQWizardPage(self) -> QWizardPage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWizardPageC1ERKS_(qthis, arg0)};
    let rsthis = QWizardPage{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QWizardPage::metaObject();
impl /*struct*/ QWizardPage {
  pub fn metaObject<RetType, T: QWizardPage_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWizardPage_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  const QMetaObject * QWizardPage::metaObject();
impl<'a> /*trait*/ QWizardPage_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10metaObjectEv()};
     unsafe {_ZNK11QWizardPage10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QWizardPage::title();
impl /*struct*/ QWizardPage {
  pub fn title<RetType, T: QWizardPage_title<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QWizardPage_title<RetType> {
  fn title(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  QString QWizardPage::title();
impl<'a> /*trait*/ QWizardPage_title<QString> for () {
  fn title(self , rsthis: &mut QWizardPage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage5titleEv()};
    let mut ret = unsafe {_ZNK11QWizardPage5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QWizardPage::subTitle();
impl /*struct*/ QWizardPage {
  pub fn subTitle<RetType, T: QWizardPage_subTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.subTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_subTitle<RetType> {
  fn subTitle(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  QString QWizardPage::subTitle();
impl<'a> /*trait*/ QWizardPage_subTitle<QString> for () {
  fn subTitle(self , rsthis: &mut QWizardPage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage8subTitleEv()};
    let mut ret = unsafe {_ZNK11QWizardPage8subTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWizardPage::isFinalPage();
impl /*struct*/ QWizardPage {
  pub fn isFinalPage<RetType, T: QWizardPage_isFinalPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isFinalPage(self);
    // return 1;
  }
}

pub trait QWizardPage_isFinalPage<RetType> {
  fn isFinalPage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::isFinalPage();
impl<'a> /*trait*/ QWizardPage_isFinalPage<i8> for () {
  fn isFinalPage(self , rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage11isFinalPageEv()};
    let mut ret = unsafe {_ZNK11QWizardPage11isFinalPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QWizardPage::validatePage();
impl /*struct*/ QWizardPage {
  pub fn validatePage<RetType, T: QWizardPage_validatePage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.validatePage(self);
    // return 1;
  }
}

pub trait QWizardPage_validatePage<RetType> {
  fn validatePage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::validatePage();
impl<'a> /*trait*/ QWizardPage_validatePage<i8> for () {
  fn validatePage(self , rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12validatePageEv()};
    let mut ret = unsafe {_ZN11QWizardPage12validatePageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QWizardPage::nextId();
impl /*struct*/ QWizardPage {
  pub fn nextId<RetType, T: QWizardPage_nextId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nextId(self);
    // return 1;
  }
}

pub trait QWizardPage_nextId<RetType> {
  fn nextId(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  int QWizardPage::nextId();
impl<'a> /*trait*/ QWizardPage_nextId<i32> for () {
  fn nextId(self , rsthis: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage6nextIdEv()};
    let mut ret = unsafe {_ZNK11QWizardPage6nextIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWizardPage::cleanupPage();
impl /*struct*/ QWizardPage {
  pub fn cleanupPage<RetType, T: QWizardPage_cleanupPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cleanupPage(self);
    // return 1;
  }
}

pub trait QWizardPage_cleanupPage<RetType> {
  fn cleanupPage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::cleanupPage();
impl<'a> /*trait*/ QWizardPage_cleanupPage<()> for () {
  fn cleanupPage(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11cleanupPageEv()};
     unsafe {_ZN11QWizardPage11cleanupPageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWizardPage::isComplete();
impl /*struct*/ QWizardPage {
  pub fn isComplete<RetType, T: QWizardPage_isComplete<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isComplete(self);
    // return 1;
  }
}

pub trait QWizardPage_isComplete<RetType> {
  fn isComplete(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::isComplete();
impl<'a> /*trait*/ QWizardPage_isComplete<i8> for () {
  fn isComplete(self , rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10isCompleteEv()};
    let mut ret = unsafe {_ZNK11QWizardPage10isCompleteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWizardPage::completeChanged();
impl /*struct*/ QWizardPage {
  pub fn completeChanged<RetType, T: QWizardPage_completeChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.completeChanged(self);
    // return 1;
  }
}

pub trait QWizardPage_completeChanged<RetType> {
  fn completeChanged(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::completeChanged();
impl<'a> /*trait*/ QWizardPage_completeChanged<()> for () {
  fn completeChanged(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage15completeChangedEv()};
     unsafe {_ZN11QWizardPage15completeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QWizardPage::isCommitPage();
impl /*struct*/ QWizardPage {
  pub fn isCommitPage<RetType, T: QWizardPage_isCommitPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isCommitPage(self);
    // return 1;
  }
}

pub trait QWizardPage_isCommitPage<RetType> {
  fn isCommitPage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  bool QWizardPage::isCommitPage();
impl<'a> /*trait*/ QWizardPage_isCommitPage<i8> for () {
  fn isCommitPage(self , rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage12isCommitPageEv()};
    let mut ret = unsafe {_ZNK11QWizardPage12isCommitPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QWizardPage::QWizardPage(QWidget * parent);
impl<'a> /*trait*/ QWizardPage_NewQWizardPage for (QWidget) {
  fn NewQWizardPage(self) -> QWizardPage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWizardPageC1EP7QWidget(qthis, arg0)};
    let rsthis = QWizardPage{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QWizardPage::setFinalPage(bool finalPage);
impl /*struct*/ QWizardPage {
  pub fn setFinalPage<RetType, T: QWizardPage_setFinalPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFinalPage(self);
    // return 1;
  }
}

pub trait QWizardPage_setFinalPage<RetType> {
  fn setFinalPage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setFinalPage(bool finalPage);
impl<'a> /*trait*/ QWizardPage_setFinalPage<()> for (i8) {
  fn setFinalPage(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12setFinalPageEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QWizardPage12setFinalPageEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
impl /*struct*/ QWizardPage {
  pub fn setSubTitle<RetType, T: QWizardPage_setSubTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSubTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_setSubTitle<RetType> {
  fn setSubTitle(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
impl<'a> /*trait*/ QWizardPage_setSubTitle<()> for (QString) {
  fn setSubTitle(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11setSubTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWizardPage11setSubTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizardPage::~QWizardPage();
impl /*struct*/ QWizardPage {
  pub fn FreeQWizardPage<RetType, T: QWizardPage_FreeQWizardPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWizardPage(self);
    // return 1;
  }
}

pub trait QWizardPage_FreeQWizardPage<RetType> {
  fn FreeQWizardPage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::~QWizardPage();
impl<'a> /*trait*/ QWizardPage_FreeQWizardPage<()> for () {
  fn FreeQWizardPage(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageD0Ev()};
     unsafe {_ZN11QWizardPageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizardPage::setCommitPage(bool commitPage);
impl /*struct*/ QWizardPage {
  pub fn setCommitPage<RetType, T: QWizardPage_setCommitPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCommitPage(self);
    // return 1;
  }
}

pub trait QWizardPage_setCommitPage<RetType> {
  fn setCommitPage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setCommitPage(bool commitPage);
impl<'a> /*trait*/ QWizardPage_setCommitPage<()> for (i8) {
  fn setCommitPage(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage13setCommitPageEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QWizardPage13setCommitPageEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizardPage::initializePage();
impl /*struct*/ QWizardPage {
  pub fn initializePage<RetType, T: QWizardPage_initializePage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.initializePage(self);
    // return 1;
  }
}

pub trait QWizardPage_initializePage<RetType> {
  fn initializePage(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::initializePage();
impl<'a> /*trait*/ QWizardPage_initializePage<()> for () {
  fn initializePage(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage14initializePageEv()};
     unsafe {_ZN11QWizardPage14initializePageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizardPage::setTitle(const QString & title);
impl /*struct*/ QWizardPage {
  pub fn setTitle<RetType, T: QWizardPage_setTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_setTitle<RetType> {
  fn setTitle(self , rsthis: &mut QWizardPage) -> RetType;
}

  // proto:  void QWizardPage::setTitle(const QString & title);
impl<'a> /*trait*/ QWizardPage_setTitle<()> for (QString) {
  fn setTitle(self , rsthis: &mut QWizardPage) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWizardPage8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn inheritFrom(qthis: *mut c_void) -> QWizard {
    return QWizard{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QWizard {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return &self.qbase;
  }
}
impl AsRef<QDialog> for QWizard {
  fn as_ref(&self) -> &QDialog {
    return &self.qbase;
  }
}
  // proto:  void QWizard::setSideWidget(QWidget * widget);
impl /*struct*/ QWizard {
  pub fn setSideWidget<RetType, T: QWizard_setSideWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSideWidget(self);
    // return 1;
  }
}

pub trait QWizard_setSideWidget<RetType> {
  fn setSideWidget(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::setSideWidget(QWidget * widget);
impl<'a> /*trait*/ QWizard_setSideWidget<()> for (QWidget) {
  fn setSideWidget(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13setSideWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWizard13setSideWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWizardPage * QWizard::currentPage();
impl /*struct*/ QWizard {
  pub fn currentPage<RetType, T: QWizard_currentPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentPage(self);
    // return 1;
  }
}

pub trait QWizard_currentPage<RetType> {
  fn currentPage(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QWizardPage * QWizard::currentPage();
impl<'a> /*trait*/ QWizard_currentPage<QWizardPage> for () {
  fn currentPage(self , rsthis: &mut QWizard) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard11currentPageEv()};
    let mut ret = unsafe {_ZNK7QWizard11currentPageEv(rsthis.qclsinst)};
    let mut ret1 = QWizardPage::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWizard::next();
impl /*struct*/ QWizard {
  pub fn next<RetType, T: QWizard_next<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QWizard_next<RetType> {
  fn next(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::next();
impl<'a> /*trait*/ QWizard_next<()> for () {
  fn next(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4nextEv()};
     unsafe {_ZN7QWizard4nextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizard::helpRequested();
impl /*struct*/ QWizard {
  pub fn helpRequested<RetType, T: QWizard_helpRequested<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.helpRequested(self);
    // return 1;
  }
}

pub trait QWizard_helpRequested<RetType> {
  fn helpRequested(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::helpRequested();
impl<'a> /*trait*/ QWizard_helpRequested<()> for () {
  fn helpRequested(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13helpRequestedEv()};
     unsafe {_ZN7QWizard13helpRequestedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWizardPage * QWizard::page(int id);
impl /*struct*/ QWizard {
  pub fn page<RetType, T: QWizard_page<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.page(self);
    // return 1;
  }
}

pub trait QWizard_page<RetType> {
  fn page(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QWizardPage * QWizard::page(int id);
impl<'a> /*trait*/ QWizard_page<QWizardPage> for (i32) {
  fn page(self , rsthis: &mut QWizard) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard4pageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QWizard4pageEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWizardPage::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QWizard::metaObject();
impl /*struct*/ QWizard {
  pub fn metaObject<RetType, T: QWizard_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QWizard_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  const QMetaObject * QWizard::metaObject();
impl<'a> /*trait*/ QWizard_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10metaObjectEv()};
     unsafe {_ZNK7QWizard10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
impl /*struct*/ QWizard {
  pub fn setField<RetType, T: QWizard_setField<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setField(self);
    // return 1;
  }
}

pub trait QWizard_setField<RetType> {
  fn setField(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
impl<'a> /*trait*/ QWizard_setField<()> for (QString, QVariant) {
  fn setField(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard8setFieldERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QWizard8setFieldERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWizard::setPage(int id, QWizardPage * page);
impl /*struct*/ QWizard {
  pub fn setPage<RetType, T: QWizard_setPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPage(self);
    // return 1;
  }
}

pub trait QWizard_setPage<RetType> {
  fn setPage(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::setPage(int id, QWizardPage * page);
impl<'a> /*trait*/ QWizard_setPage<()> for (i32, QWizardPage) {
  fn setPage(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7setPageEiP11QWizardPage()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QWizard7setPageEiP11QWizardPage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QWizard::restart();
impl /*struct*/ QWizard {
  pub fn restart<RetType, T: QWizard_restart<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.restart(self);
    // return 1;
  }
}

pub trait QWizard_restart<RetType> {
  fn restart(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::restart();
impl<'a> /*trait*/ QWizard_restart<()> for () {
  fn restart(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7restartEv()};
     unsafe {_ZN7QWizard7restartEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizard::back();
impl /*struct*/ QWizard {
  pub fn back<RetType, T: QWizard_back<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.back(self);
    // return 1;
  }
}

pub trait QWizard_back<RetType> {
  fn back(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::back();
impl<'a> /*trait*/ QWizard_back<()> for () {
  fn back(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4backEv()};
     unsafe {_ZN7QWizard4backEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSize QWizard::sizeHint();
impl /*struct*/ QWizard {
  pub fn sizeHint<RetType, T: QWizard_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QWizard_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QSize QWizard::sizeHint();
impl<'a> /*trait*/ QWizard_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QWizard) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QWizard8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
impl /*struct*/ QWizard {
  pub fn setDefaultProperty<RetType, T: QWizard_setDefaultProperty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultProperty(self);
    // return 1;
  }
}

pub trait QWizard_setDefaultProperty<RetType> {
  fn setDefaultProperty(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
impl<'a> /*trait*/ QWizard_setDefaultProperty<()> for (&'a  String, &'a  String, &'a  String) {
  fn setDefaultProperty(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard18setDefaultPropertyEPKcS1_S1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {_ZN7QWizard18setDefaultPropertyEPKcS1_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QWizard::setStartId(int id);
impl /*struct*/ QWizard {
  pub fn setStartId<RetType, T: QWizard_setStartId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStartId(self);
    // return 1;
  }
}

pub trait QWizard_setStartId<RetType> {
  fn setStartId(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::setStartId(int id);
impl<'a> /*trait*/ QWizard_setStartId<()> for (i32) {
  fn setStartId(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setStartIdEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard10setStartIdEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizard::currentIdChanged(int id);
impl /*struct*/ QWizard {
  pub fn currentIdChanged<RetType, T: QWizard_currentIdChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIdChanged(self);
    // return 1;
  }
}

pub trait QWizard_currentIdChanged<RetType> {
  fn currentIdChanged(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::currentIdChanged(int id);
impl<'a> /*trait*/ QWizard_currentIdChanged<()> for (i32) {
  fn currentIdChanged(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard16currentIdChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard16currentIdChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizard::~QWizard();
impl /*struct*/ QWizard {
  pub fn FreeQWizard<RetType, T: QWizard_FreeQWizard<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQWizard(self);
    // return 1;
  }
}

pub trait QWizard_FreeQWizard<RetType> {
  fn FreeQWizard(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::~QWizard();
impl<'a> /*trait*/ QWizard_FreeQWizard<()> for () {
  fn FreeQWizard(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizardD0Ev()};
     unsafe {_ZN7QWizardD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QWizard::pageAdded(int id);
impl /*struct*/ QWizard {
  pub fn pageAdded<RetType, T: QWizard_pageAdded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageAdded(self);
    // return 1;
  }
}

pub trait QWizard_pageAdded<RetType> {
  fn pageAdded(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::pageAdded(int id);
impl<'a> /*trait*/ QWizard_pageAdded<()> for (i32) {
  fn pageAdded(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard9pageAddedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard9pageAddedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<int> QWizard::visitedPages();
impl /*struct*/ QWizard {
  pub fn visitedPages<RetType, T: QWizard_visitedPages<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.visitedPages(self);
    // return 1;
  }
}

pub trait QWizard_visitedPages<RetType> {
  fn visitedPages(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QList<int> QWizard::visitedPages();
impl<'a> /*trait*/ QWizard_visitedPages<()> for () {
  fn visitedPages(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard12visitedPagesEv()};
     unsafe {_ZNK7QWizard12visitedPagesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QWizard::nextId();
impl /*struct*/ QWizard {
  pub fn nextId<RetType, T: QWizard_nextId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nextId(self);
    // return 1;
  }
}

pub trait QWizard_nextId<RetType> {
  fn nextId(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  int QWizard::nextId();
impl<'a> /*trait*/ QWizard_nextId<i32> for () {
  fn nextId(self , rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard6nextIdEv()};
    let mut ret = unsafe {_ZNK7QWizard6nextIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QWizard::startId();
impl /*struct*/ QWizard {
  pub fn startId<RetType, T: QWizard_startId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.startId(self);
    // return 1;
  }
}

pub trait QWizard_startId<RetType> {
  fn startId(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  int QWizard::startId();
impl<'a> /*trait*/ QWizard_startId<i32> for () {
  fn startId(self , rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7startIdEv()};
    let mut ret = unsafe {_ZNK7QWizard7startIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWizard::customButtonClicked(int which);
impl /*struct*/ QWizard {
  pub fn customButtonClicked<RetType, T: QWizard_customButtonClicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.customButtonClicked(self);
    // return 1;
  }
}

pub trait QWizard_customButtonClicked<RetType> {
  fn customButtonClicked(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::customButtonClicked(int which);
impl<'a> /*trait*/ QWizard_customButtonClicked<()> for (i32) {
  fn customButtonClicked(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19customButtonClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard19customButtonClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizard::QWizard(const QWizard & );
impl /*struct*/ QWizard {
  pub fn NewQWizard<T: QWizard_NewQWizard>(value: T) -> QWizard {
    let rsthis = value.NewQWizard();
    return rsthis;
    // return 1;
  }
}

pub trait QWizard_NewQWizard {
  fn NewQWizard(self) -> QWizard;
}

  // proto:  void QWizard::QWizard(const QWizard & );
impl<'a> /*trait*/ QWizard_NewQWizard for (QWizard) {
  fn NewQWizard(self) -> QWizard {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizardC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWizardC1ERKS_(qthis, arg0)};
    let rsthis = QWizard{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QWizard::addPage(QWizardPage * page);
impl /*struct*/ QWizard {
  pub fn addPage<RetType, T: QWizard_addPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addPage(self);
    // return 1;
  }
}

pub trait QWizard_addPage<RetType> {
  fn addPage(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  int QWizard::addPage(QWizardPage * page);
impl<'a> /*trait*/ QWizard_addPage<i32> for (QWizardPage) {
  fn addPage(self , rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7addPageEP11QWizardPage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QWizard7addPageEP11QWizardPage(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWizard::removePage(int id);
impl /*struct*/ QWizard {
  pub fn removePage<RetType, T: QWizard_removePage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removePage(self);
    // return 1;
  }
}

pub trait QWizard_removePage<RetType> {
  fn removePage(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::removePage(int id);
impl<'a> /*trait*/ QWizard_removePage<()> for (i32) {
  fn removePage(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10removePageEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard10removePageEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<int> QWizard::pageIds();
impl /*struct*/ QWizard {
  pub fn pageIds<RetType, T: QWizard_pageIds<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageIds(self);
    // return 1;
  }
}

pub trait QWizard_pageIds<RetType> {
  fn pageIds(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QList<int> QWizard::pageIds();
impl<'a> /*trait*/ QWizard_pageIds<()> for () {
  fn pageIds(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7pageIdsEv()};
     unsafe {_ZNK7QWizard7pageIdsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QWizard::currentId();
impl /*struct*/ QWizard {
  pub fn currentId<RetType, T: QWizard_currentId<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentId(self);
    // return 1;
  }
}

pub trait QWizard_currentId<RetType> {
  fn currentId(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  int QWizard::currentId();
impl<'a> /*trait*/ QWizard_currentId<i32> for () {
  fn currentId(self , rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard9currentIdEv()};
    let mut ret = unsafe {_ZNK7QWizard9currentIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QWizard::pageRemoved(int id);
impl /*struct*/ QWizard {
  pub fn pageRemoved<RetType, T: QWizard_pageRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageRemoved(self);
    // return 1;
  }
}

pub trait QWizard_pageRemoved<RetType> {
  fn pageRemoved(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::pageRemoved(int id);
impl<'a> /*trait*/ QWizard_pageRemoved<()> for (i32) {
  fn pageRemoved(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard11pageRemovedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard11pageRemovedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QWizard::setVisible(bool visible);
impl /*struct*/ QWizard {
  pub fn setVisible<RetType, T: QWizard_setVisible<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QWizard_setVisible<RetType> {
  fn setVisible(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  void QWizard::setVisible(bool visible);
impl<'a> /*trait*/ QWizard_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QWizard10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QWizard::hasVisitedPage(int id);
impl /*struct*/ QWizard {
  pub fn hasVisitedPage<RetType, T: QWizard_hasVisitedPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasVisitedPage(self);
    // return 1;
  }
}

pub trait QWizard_hasVisitedPage<RetType> {
  fn hasVisitedPage(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  bool QWizard::hasVisitedPage(int id);
impl<'a> /*trait*/ QWizard_hasVisitedPage<i8> for (i32) {
  fn hasVisitedPage(self , rsthis: &mut QWizard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard14hasVisitedPageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QWizard14hasVisitedPageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QVariant QWizard::field(const QString & name);
impl /*struct*/ QWizard {
  pub fn field<RetType, T: QWizard_field<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.field(self);
    // return 1;
  }
}

pub trait QWizard_field<RetType> {
  fn field(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QVariant QWizard::field(const QString & name);
impl<'a> /*trait*/ QWizard_field<QVariant> for (QString) {
  fn field(self , rsthis: &mut QWizard) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard5fieldERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWizard5fieldERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QWizard::validateCurrentPage();
impl /*struct*/ QWizard {
  pub fn validateCurrentPage<RetType, T: QWizard_validateCurrentPage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.validateCurrentPage(self);
    // return 1;
  }
}

pub trait QWizard_validateCurrentPage<RetType> {
  fn validateCurrentPage(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  bool QWizard::validateCurrentPage();
impl<'a> /*trait*/ QWizard_validateCurrentPage<i8> for () {
  fn validateCurrentPage(self , rsthis: &mut QWizard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19validateCurrentPageEv()};
    let mut ret = unsafe {_ZN7QWizard19validateCurrentPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QWidget * QWizard::sideWidget();
impl /*struct*/ QWizard {
  pub fn sideWidget<RetType, T: QWizard_sideWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sideWidget(self);
    // return 1;
  }
}

pub trait QWizard_sideWidget<RetType> {
  fn sideWidget(self , rsthis: &mut QWizard) -> RetType;
}

  // proto:  QWidget * QWizard::sideWidget();
impl<'a> /*trait*/ QWizard_sideWidget<QWidget> for () {
  fn sideWidget(self , rsthis: &mut QWizard) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10sideWidgetEv()};
    let mut ret = unsafe {_ZNK7QWizard10sideWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

