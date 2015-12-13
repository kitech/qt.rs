// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QWizardPage::NewQWizardPage(const QWizardPage & );
  fn _ZN11QWizardPageC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QWizardPage::metaObject();
  fn _ZNK11QWizardPage10metaObjectEv() -> i32;
  // proto: QString QWizardPage::title();
  fn _ZNK11QWizardPage5titleEv() -> i32;
  // proto: QString QWizardPage::subTitle();
  fn _ZNK11QWizardPage8subTitleEv() -> i32;
  // proto: bool QWizardPage::isFinalPage();
  fn _ZNK11QWizardPage11isFinalPageEv() -> i32;
  // proto: bool QWizardPage::validatePage();
  fn _ZN11QWizardPage12validatePageEv() -> i32;
  // proto: int QWizardPage::nextId();
  fn _ZNK11QWizardPage6nextIdEv() -> i32;
  // proto: void QWizardPage::cleanupPage();
  fn _ZN11QWizardPage11cleanupPageEv() -> i32;
  // proto: bool QWizardPage::isComplete();
  fn _ZNK11QWizardPage10isCompleteEv() -> i32;
  // proto: void QWizardPage::completeChanged();
  fn _ZN11QWizardPage15completeChangedEv() -> i32;
  // proto: bool QWizardPage::isCommitPage();
  fn _ZNK11QWizardPage12isCommitPageEv() -> i32;
  // proto: void QWizardPage::NewQWizardPage(QWidget * parent);
  fn _ZN11QWizardPageC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QWizardPage::setFinalPage(bool finalPage);
  fn _ZN11QWizardPage12setFinalPageEb(arg0: int8_t) -> i32;
  // proto: void QWizardPage::setSubTitle(const QString & subTitle);
  fn _ZN11QWizardPage11setSubTitleERK7QString(arg0: *const c_void) -> i32;
  // proto: void QWizardPage::FreeQWizardPage();
  fn _ZN11QWizardPageD0Ev() -> i32;
  // proto: void QWizardPage::setCommitPage(bool commitPage);
  fn _ZN11QWizardPage13setCommitPageEb(arg0: int8_t) -> i32;
  // proto: void QWizardPage::initializePage();
  fn _ZN11QWizardPage14initializePageEv() -> i32;
  // proto: void QWizardPage::setTitle(const QString & title);
  fn _ZN11QWizardPage8setTitleERK7QString(arg0: *const c_void) -> i32;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QWizardPageC1ERKS_(qthis, arg0)};
    let rsthis = QWizardPage{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn metaObject<T: QWizardPage_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QWizardPage_metaObject {
  fn metaObject(self, this: &mut QWizardPage) -> i32;
}

// proto: const QMetaObject * QWizardPage::metaObject();
impl<'a> /*trait*/ QWizardPage_metaObject for () {
  fn metaObject(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10metaObjectEv()};
    unsafe {_ZNK11QWizardPage10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn title<T: QWizardPage_title>(&mut self, value: T) -> i32 {
    value.title(self);
    return 1;
  }
}

pub trait QWizardPage_title {
  fn title(self, this: &mut QWizardPage) -> i32;
}

// proto: QString QWizardPage::title();
impl<'a> /*trait*/ QWizardPage_title for () {
  fn title(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage5titleEv()};
    unsafe {_ZNK11QWizardPage5titleEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn subTitle<T: QWizardPage_subTitle>(&mut self, value: T) -> i32 {
    value.subTitle(self);
    return 1;
  }
}

pub trait QWizardPage_subTitle {
  fn subTitle(self, this: &mut QWizardPage) -> i32;
}

// proto: QString QWizardPage::subTitle();
impl<'a> /*trait*/ QWizardPage_subTitle for () {
  fn subTitle(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage8subTitleEv()};
    unsafe {_ZNK11QWizardPage8subTitleEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn isFinalPage<T: QWizardPage_isFinalPage>(&mut self, value: T) -> i32 {
    value.isFinalPage(self);
    return 1;
  }
}

pub trait QWizardPage_isFinalPage {
  fn isFinalPage(self, this: &mut QWizardPage) -> i32;
}

// proto: bool QWizardPage::isFinalPage();
impl<'a> /*trait*/ QWizardPage_isFinalPage for () {
  fn isFinalPage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage11isFinalPageEv()};
    unsafe {_ZNK11QWizardPage11isFinalPageEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn validatePage<T: QWizardPage_validatePage>(&mut self, value: T) -> i32 {
    value.validatePage(self);
    return 1;
  }
}

pub trait QWizardPage_validatePage {
  fn validatePage(self, this: &mut QWizardPage) -> i32;
}

// proto: bool QWizardPage::validatePage();
impl<'a> /*trait*/ QWizardPage_validatePage for () {
  fn validatePage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12validatePageEv()};
    unsafe {_ZN11QWizardPage12validatePageEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn nextId<T: QWizardPage_nextId>(&mut self, value: T) -> i32 {
    value.nextId(self);
    return 1;
  }
}

pub trait QWizardPage_nextId {
  fn nextId(self, this: &mut QWizardPage) -> i32;
}

// proto: int QWizardPage::nextId();
impl<'a> /*trait*/ QWizardPage_nextId for () {
  fn nextId(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage6nextIdEv()};
    unsafe {_ZNK11QWizardPage6nextIdEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn cleanupPage<T: QWizardPage_cleanupPage>(&mut self, value: T) -> i32 {
    value.cleanupPage(self);
    return 1;
  }
}

pub trait QWizardPage_cleanupPage {
  fn cleanupPage(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::cleanupPage();
impl<'a> /*trait*/ QWizardPage_cleanupPage for () {
  fn cleanupPage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11cleanupPageEv()};
    unsafe {_ZN11QWizardPage11cleanupPageEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn isComplete<T: QWizardPage_isComplete>(&mut self, value: T) -> i32 {
    value.isComplete(self);
    return 1;
  }
}

pub trait QWizardPage_isComplete {
  fn isComplete(self, this: &mut QWizardPage) -> i32;
}

// proto: bool QWizardPage::isComplete();
impl<'a> /*trait*/ QWizardPage_isComplete for () {
  fn isComplete(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage10isCompleteEv()};
    unsafe {_ZNK11QWizardPage10isCompleteEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn completeChanged<T: QWizardPage_completeChanged>(&mut self, value: T) -> i32 {
    value.completeChanged(self);
    return 1;
  }
}

pub trait QWizardPage_completeChanged {
  fn completeChanged(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::completeChanged();
impl<'a> /*trait*/ QWizardPage_completeChanged for () {
  fn completeChanged(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage15completeChangedEv()};
    unsafe {_ZN11QWizardPage15completeChangedEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn isCommitPage<T: QWizardPage_isCommitPage>(&mut self, value: T) -> i32 {
    value.isCommitPage(self);
    return 1;
  }
}

pub trait QWizardPage_isCommitPage {
  fn isCommitPage(self, this: &mut QWizardPage) -> i32;
}

// proto: bool QWizardPage::isCommitPage();
impl<'a> /*trait*/ QWizardPage_isCommitPage for () {
  fn isCommitPage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QWizardPage12isCommitPageEv()};
    unsafe {_ZNK11QWizardPage12isCommitPageEv()};
    return 1;
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
  pub fn setFinalPage<T: QWizardPage_setFinalPage>(&mut self, value: T) -> i32 {
    value.setFinalPage(self);
    return 1;
  }
}

pub trait QWizardPage_setFinalPage {
  fn setFinalPage(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::setFinalPage(bool finalPage);
impl<'a> /*trait*/ QWizardPage_setFinalPage for (i8) {
  fn setFinalPage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage12setFinalPageEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QWizardPage12setFinalPageEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setSubTitle<T: QWizardPage_setSubTitle>(&mut self, value: T) -> i32 {
    value.setSubTitle(self);
    return 1;
  }
}

pub trait QWizardPage_setSubTitle {
  fn setSubTitle(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::setSubTitle(const QString & subTitle);
impl<'a> /*trait*/ QWizardPage_setSubTitle for (&'a  QString) {
  fn setSubTitle(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage11setSubTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QWizardPage11setSubTitleERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn FreeQWizardPage<T: QWizardPage_FreeQWizardPage>(&mut self, value: T) -> i32 {
    value.FreeQWizardPage(self);
    return 1;
  }
}

pub trait QWizardPage_FreeQWizardPage {
  fn FreeQWizardPage(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::FreeQWizardPage();
impl<'a> /*trait*/ QWizardPage_FreeQWizardPage for () {
  fn FreeQWizardPage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPageD0Ev()};
    unsafe {_ZN11QWizardPageD0Ev()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setCommitPage<T: QWizardPage_setCommitPage>(&mut self, value: T) -> i32 {
    value.setCommitPage(self);
    return 1;
  }
}

pub trait QWizardPage_setCommitPage {
  fn setCommitPage(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::setCommitPage(bool commitPage);
impl<'a> /*trait*/ QWizardPage_setCommitPage for (i8) {
  fn setCommitPage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage13setCommitPageEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QWizardPage13setCommitPageEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn initializePage<T: QWizardPage_initializePage>(&mut self, value: T) -> i32 {
    value.initializePage(self);
    return 1;
  }
}

pub trait QWizardPage_initializePage {
  fn initializePage(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::initializePage();
impl<'a> /*trait*/ QWizardPage_initializePage for () {
  fn initializePage(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage14initializePageEv()};
    unsafe {_ZN11QWizardPage14initializePageEv()};
    return 1;
  }
}

impl /*struct*/ QWizardPage {
  pub fn setTitle<T: QWizardPage_setTitle>(&mut self, value: T) -> i32 {
    value.setTitle(self);
    return 1;
  }
}

pub trait QWizardPage_setTitle {
  fn setTitle(self, this: &mut QWizardPage) -> i32;
}

// proto: void QWizardPage::setTitle(const QString & title);
impl<'a> /*trait*/ QWizardPage_setTitle for (&'a  QString) {
  fn setTitle(self, this: &mut QWizardPage) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QWizardPage8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QWizardPage8setTitleERK7QString(arg0)};
    return 1;
  }
}

