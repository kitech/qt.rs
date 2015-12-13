// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QWizardPage::NewQWizardPage(const QWizardPage & );
  fn _ZN11QWizardPageC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QWizardPage::metaObject();
  fn _ZNK11QWizardPage10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QString QWizardPage::title();
  fn _ZNK11QWizardPage5titleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QWizardPage::subTitle();
  fn _ZNK11QWizardPage8subTitleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QWizardPage::isFinalPage();
  fn _ZNK11QWizardPage11isFinalPageEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QWizardPage::validatePage();
  fn _ZN11QWizardPage12validatePageEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QWizardPage::nextId();
  fn _ZNK11QWizardPage6nextIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWizardPage::cleanupPage();
  fn _ZN11QWizardPage11cleanupPageEv(qthis: *mut c_void) ;
  // proto:  bool QWizardPage::isComplete();
  fn _ZNK11QWizardPage10isCompleteEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWizardPage::completeChanged();
  fn _ZN11QWizardPage15completeChangedEv(qthis: *mut c_void) ;
  // proto:  bool QWizardPage::isCommitPage();
  fn _ZNK11QWizardPage12isCommitPageEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QWizardPage::NewQWizardPage(QWidget * parent);
  fn _ZN11QWizardPageC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWizardPage::setFinalPage(bool finalPage);
  fn _ZN11QWizardPage12setFinalPageEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QWizardPage::setSubTitle(const QString & subTitle);
  fn _ZN11QWizardPage11setSubTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QWizardPage::FreeQWizardPage();
  fn _ZN11QWizardPageD0Ev(qthis: *mut c_void) ;
  // proto:  void QWizardPage::setCommitPage(bool commitPage);
  fn _ZN11QWizardPage13setCommitPageEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QWizardPage::initializePage();
  fn _ZN11QWizardPage14initializePageEv(qthis: *mut c_void) ;
  // proto:  void QWizardPage::setTitle(const QString & title);
  fn _ZN11QWizardPage8setTitleERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QWizardPage)=1
pub struct QWizardPage {
  pub qclsinst: *mut c_void,
}

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

// proto: void QWizardPage::NewQWizardPage(const QWizardPage & );
impl<'a> /*trait*/ QWizardPage_NewQWizardPage for (&'a  QWizardPage) {
  fn NewQWizardPage(self) -> QWizardPage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWizardPageC1ERKS_(qthis, arg0)};
    let rsthis = QWizardPage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn metaObject<T: QWizardPage_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QWizardPage_metaObject {
  fn metaObject(self, rsthis: &mut QWizardPage) ;
}

// proto:  const QMetaObject * QWizardPage::metaObject();
impl<'a> /*trait*/ QWizardPage_metaObject for () {
  fn metaObject(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10metaObjectEv()};
     unsafe {_ZNK11QWizardPage10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn title<T: QWizardPage_title>(&mut self, value: T) -> QString {
    return value.title(self);
    // return 1;
  }
}

pub trait QWizardPage_title {
  fn title(self, rsthis: &mut QWizardPage) -> QString;
}

// proto:  QString QWizardPage::title();
impl<'a> /*trait*/ QWizardPage_title for () {
  fn title(self, rsthis: &mut QWizardPage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage5titleEv()};
    let mut ret = unsafe {_ZNK11QWizardPage5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn subTitle<T: QWizardPage_subTitle>(&mut self, value: T) -> QString {
    return value.subTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_subTitle {
  fn subTitle(self, rsthis: &mut QWizardPage) -> QString;
}

// proto:  QString QWizardPage::subTitle();
impl<'a> /*trait*/ QWizardPage_subTitle for () {
  fn subTitle(self, rsthis: &mut QWizardPage) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage8subTitleEv()};
    let mut ret = unsafe {_ZNK11QWizardPage8subTitleEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn isFinalPage<T: QWizardPage_isFinalPage>(&mut self, value: T) -> i8 {
    return value.isFinalPage(self);
    // return 1;
  }
}

pub trait QWizardPage_isFinalPage {
  fn isFinalPage(self, rsthis: &mut QWizardPage) -> i8;
}

// proto:  bool QWizardPage::isFinalPage();
impl<'a> /*trait*/ QWizardPage_isFinalPage for () {
  fn isFinalPage(self, rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage11isFinalPageEv()};
    let mut ret = unsafe {_ZNK11QWizardPage11isFinalPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn validatePage<T: QWizardPage_validatePage>(&mut self, value: T) -> i8 {
    return value.validatePage(self);
    // return 1;
  }
}

pub trait QWizardPage_validatePage {
  fn validatePage(self, rsthis: &mut QWizardPage) -> i8;
}

// proto:  bool QWizardPage::validatePage();
impl<'a> /*trait*/ QWizardPage_validatePage for () {
  fn validatePage(self, rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12validatePageEv()};
    let mut ret = unsafe {_ZN11QWizardPage12validatePageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn nextId<T: QWizardPage_nextId>(&mut self, value: T) -> i32 {
    return value.nextId(self);
    // return 1;
  }
}

pub trait QWizardPage_nextId {
  fn nextId(self, rsthis: &mut QWizardPage) -> i32;
}

// proto:  int QWizardPage::nextId();
impl<'a> /*trait*/ QWizardPage_nextId for () {
  fn nextId(self, rsthis: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage6nextIdEv()};
    let mut ret = unsafe {_ZNK11QWizardPage6nextIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn cleanupPage<T: QWizardPage_cleanupPage>(&mut self, value: T)  {
     value.cleanupPage(self);
    // return 1;
  }
}

pub trait QWizardPage_cleanupPage {
  fn cleanupPage(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::cleanupPage();
impl<'a> /*trait*/ QWizardPage_cleanupPage for () {
  fn cleanupPage(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11cleanupPageEv()};
     unsafe {_ZN11QWizardPage11cleanupPageEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn isComplete<T: QWizardPage_isComplete>(&mut self, value: T) -> i8 {
    return value.isComplete(self);
    // return 1;
  }
}

pub trait QWizardPage_isComplete {
  fn isComplete(self, rsthis: &mut QWizardPage) -> i8;
}

// proto:  bool QWizardPage::isComplete();
impl<'a> /*trait*/ QWizardPage_isComplete for () {
  fn isComplete(self, rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10isCompleteEv()};
    let mut ret = unsafe {_ZNK11QWizardPage10isCompleteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn completeChanged<T: QWizardPage_completeChanged>(&mut self, value: T)  {
     value.completeChanged(self);
    // return 1;
  }
}

pub trait QWizardPage_completeChanged {
  fn completeChanged(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::completeChanged();
impl<'a> /*trait*/ QWizardPage_completeChanged for () {
  fn completeChanged(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage15completeChangedEv()};
     unsafe {_ZN11QWizardPage15completeChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn isCommitPage<T: QWizardPage_isCommitPage>(&mut self, value: T) -> i8 {
    return value.isCommitPage(self);
    // return 1;
  }
}

pub trait QWizardPage_isCommitPage {
  fn isCommitPage(self, rsthis: &mut QWizardPage) -> i8;
}

// proto:  bool QWizardPage::isCommitPage();
impl<'a> /*trait*/ QWizardPage_isCommitPage for () {
  fn isCommitPage(self, rsthis: &mut QWizardPage) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage12isCommitPageEv()};
    let mut ret = unsafe {_ZNK11QWizardPage12isCommitPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QWizardPage::NewQWizardPage(QWidget * parent);
impl<'a> /*trait*/ QWizardPage_NewQWizardPage for (&'a mut QWidget) {
  fn NewQWizardPage(self) -> QWizardPage {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QWizardPageC1EP7QWidget(qthis, arg0)};
    let rsthis = QWizardPage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setFinalPage<T: QWizardPage_setFinalPage>(&mut self, value: T)  {
     value.setFinalPage(self);
    // return 1;
  }
}

pub trait QWizardPage_setFinalPage {
  fn setFinalPage(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::setFinalPage(bool finalPage);
impl<'a> /*trait*/ QWizardPage_setFinalPage for (i8) {
  fn setFinalPage(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12setFinalPageEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QWizardPage12setFinalPageEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setSubTitle<T: QWizardPage_setSubTitle>(&mut self, value: T)  {
     value.setSubTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_setSubTitle {
  fn setSubTitle(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::setSubTitle(const QString & subTitle);
impl<'a> /*trait*/ QWizardPage_setSubTitle for (&'a  QString) {
  fn setSubTitle(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11setSubTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWizardPage11setSubTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn FreeQWizardPage<T: QWizardPage_FreeQWizardPage>(&mut self, value: T)  {
     value.FreeQWizardPage(self);
    // return 1;
  }
}

pub trait QWizardPage_FreeQWizardPage {
  fn FreeQWizardPage(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::FreeQWizardPage();
impl<'a> /*trait*/ QWizardPage_FreeQWizardPage for () {
  fn FreeQWizardPage(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageD0Ev()};
     unsafe {_ZN11QWizardPageD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setCommitPage<T: QWizardPage_setCommitPage>(&mut self, value: T)  {
     value.setCommitPage(self);
    // return 1;
  }
}

pub trait QWizardPage_setCommitPage {
  fn setCommitPage(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::setCommitPage(bool commitPage);
impl<'a> /*trait*/ QWizardPage_setCommitPage for (i8) {
  fn setCommitPage(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage13setCommitPageEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QWizardPage13setCommitPageEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn initializePage<T: QWizardPage_initializePage>(&mut self, value: T)  {
     value.initializePage(self);
    // return 1;
  }
}

pub trait QWizardPage_initializePage {
  fn initializePage(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::initializePage();
impl<'a> /*trait*/ QWizardPage_initializePage for () {
  fn initializePage(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage14initializePageEv()};
     unsafe {_ZN11QWizardPage14initializePageEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setTitle<T: QWizardPage_setTitle>(&mut self, value: T)  {
     value.setTitle(self);
    // return 1;
  }
}

pub trait QWizardPage_setTitle {
  fn setTitle(self, rsthis: &mut QWizardPage) ;
}

// proto:  void QWizardPage::setTitle(const QString & title);
impl<'a> /*trait*/ QWizardPage_setTitle for (&'a  QString) {
  fn setTitle(self, rsthis: &mut QWizardPage)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QWizardPage8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

