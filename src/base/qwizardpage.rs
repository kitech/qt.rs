// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qpixmap::QPixmap;
use super::qwidget::QWidget;

// ext block begin
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
}

// body block begin
// class sizeof(QWizardPage)=1
pub struct QWizardPage {
  pub qclsinst: *mut c_void,
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
    let rsthis = QWizardPage{qclsinst: qthis};
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
    let mut ret1 = QString{qclsinst: ret};
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
    let mut ret1 = QString{qclsinst: ret};
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
    let rsthis = QWizardPage{qclsinst: qthis};
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

