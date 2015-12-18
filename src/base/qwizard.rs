// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qwizardpage::QWizardPage;
use super::qstring::QString;
use super::qvariant::QVariant;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QWizard::setSideWidget(QWidget * widget);
  fn _ZN7QWizard13setSideWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWizardPage * QWizard::currentPage();
  fn _ZNK7QWizard11currentPageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWizard::next();
  fn _ZN7QWizard4nextEv(qthis: *mut c_void) ;
  // proto:  void QWizard::helpRequested();
  fn _ZN7QWizard13helpRequestedEv(qthis: *mut c_void) ;
  // proto:  QWizardPage * QWizard::page(int id);
  fn _ZNK7QWizard4pageEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QWizard::metaObject();
  fn _ZNK7QWizard10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QWizard::setField(const QString & name, const QVariant & value);
  fn _ZN7QWizard8setFieldERK7QStringRK8QVariant(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QWizard::setPage(int id, QWizardPage * page);
  fn _ZN7QWizard7setPageEiP11QWizardPage(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QWizard::restart();
  fn _ZN7QWizard7restartEv(qthis: *mut c_void) ;
  // proto:  void QWizard::back();
  fn _ZN7QWizard4backEv(qthis: *mut c_void) ;
  // proto:  QSize QWizard::sizeHint();
  fn _ZNK7QWizard8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
  fn _ZN7QWizard18setDefaultPropertyEPKcS1_S1_(qthis: *mut c_void, arg0: *const c_char, arg1: *const c_char, arg2: *const c_char) ;
  // proto:  void QWizard::setStartId(int id);
  fn _ZN7QWizard10setStartIdEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWizard::currentIdChanged(int id);
  fn _ZN7QWizard16currentIdChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWizard::FreeQWizard();
  fn _ZN7QWizardD0Ev(qthis: *mut c_void) ;
  // proto:  void QWizard::pageAdded(int id);
  fn _ZN7QWizard9pageAddedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QList<int> QWizard::visitedPages();
  fn _ZNK7QWizard12visitedPagesEv(qthis: *mut c_void) ;
  // proto:  int QWizard::nextId();
  fn _ZNK7QWizard6nextIdEv(qthis: *mut c_void) -> c_int;
  // proto:  int QWizard::startId();
  fn _ZNK7QWizard7startIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWizard::customButtonClicked(int which);
  fn _ZN7QWizard19customButtonClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWizard::NewQWizard(const QWizard & );
  fn _ZN7QWizardC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QWizard::addPage(QWizardPage * page);
  fn _ZN7QWizard7addPageEP11QWizardPage(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QWizard::removePage(int id);
  fn _ZN7QWizard10removePageEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QList<int> QWizard::pageIds();
  fn _ZNK7QWizard7pageIdsEv(qthis: *mut c_void) ;
  // proto:  int QWizard::currentId();
  fn _ZNK7QWizard9currentIdEv(qthis: *mut c_void) -> c_int;
  // proto:  void QWizard::pageRemoved(int id);
  fn _ZN7QWizard11pageRemovedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QWizard::setVisible(bool visible);
  fn _ZN7QWizard10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QWizard::hasVisitedPage(int id);
  fn _ZNK7QWizard14hasVisitedPageEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QVariant QWizard::field(const QString & name);
  fn _ZNK7QWizard5fieldERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QWizard::validateCurrentPage();
  fn _ZN7QWizard19validateCurrentPageEv(qthis: *mut c_void) -> int8_t;
  // proto:  QWidget * QWizard::sideWidget();
  fn _ZNK7QWizard10sideWidgetEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWizard)=1
pub struct QWizard {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWizard {
  pub fn setSideWidget<RetType, T: QWizard_setSideWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.setSideWidget(self);
    // return 1;
  }
}

pub trait QWizard_setSideWidget<RetType> {
  fn setSideWidget(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::setSideWidget(QWidget * widget);
impl<'a> /*trait*/ QWizard_setSideWidget<()> for (&'a mut QWidget) {
  fn setSideWidget(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13setSideWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QWizard13setSideWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn currentPage<RetType, T: QWizard_currentPage<RetType>>(&mut self, value: T) -> RetType {
    return value.currentPage(self);
    // return 1;
  }
}

pub trait QWizard_currentPage<RetType> {
  fn currentPage(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QWizardPage * QWizard::currentPage();
impl<'a> /*trait*/ QWizard_currentPage<QWizardPage> for () {
  fn currentPage(self, rsthis: &mut QWizard) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard11currentPageEv()};
    let mut ret = unsafe {_ZNK7QWizard11currentPageEv(rsthis.qclsinst)};
    let mut ret1 = QWizardPage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn next<RetType, T: QWizard_next<RetType>>(&mut self, value: T) -> RetType {
    return value.next(self);
    // return 1;
  }
}

pub trait QWizard_next<RetType> {
  fn next(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::next();
impl<'a> /*trait*/ QWizard_next<()> for () {
  fn next(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4nextEv()};
     unsafe {_ZN7QWizard4nextEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn helpRequested<RetType, T: QWizard_helpRequested<RetType>>(&mut self, value: T) -> RetType {
    return value.helpRequested(self);
    // return 1;
  }
}

pub trait QWizard_helpRequested<RetType> {
  fn helpRequested(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::helpRequested();
impl<'a> /*trait*/ QWizard_helpRequested<()> for () {
  fn helpRequested(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13helpRequestedEv()};
     unsafe {_ZN7QWizard13helpRequestedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn page<RetType, T: QWizard_page<RetType>>(&mut self, value: T) -> RetType {
    return value.page(self);
    // return 1;
  }
}

pub trait QWizard_page<RetType> {
  fn page(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QWizardPage * QWizard::page(int id);
impl<'a> /*trait*/ QWizard_page<QWizardPage> for (i32) {
  fn page(self, rsthis: &mut QWizard) -> QWizardPage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard4pageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QWizard4pageEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWizardPage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn metaObject<RetType, T: QWizard_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QWizard_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  const QMetaObject * QWizard::metaObject();
impl<'a> /*trait*/ QWizard_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10metaObjectEv()};
     unsafe {_ZNK7QWizard10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setField<RetType, T: QWizard_setField<RetType>>(&mut self, value: T) -> RetType {
    return value.setField(self);
    // return 1;
  }
}

pub trait QWizard_setField<RetType> {
  fn setField(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::setField(const QString & name, const QVariant & value);
impl<'a> /*trait*/ QWizard_setField<()> for (&'a  QString, &'a  QVariant) {
  fn setField(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard8setFieldERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QWizard8setFieldERK7QStringRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setPage<RetType, T: QWizard_setPage<RetType>>(&mut self, value: T) -> RetType {
    return value.setPage(self);
    // return 1;
  }
}

pub trait QWizard_setPage<RetType> {
  fn setPage(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::setPage(int id, QWizardPage * page);
impl<'a> /*trait*/ QWizard_setPage<()> for (i32, &'a mut QWizardPage) {
  fn setPage(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7setPageEiP11QWizardPage()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QWizard7setPageEiP11QWizardPage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn restart<RetType, T: QWizard_restart<RetType>>(&mut self, value: T) -> RetType {
    return value.restart(self);
    // return 1;
  }
}

pub trait QWizard_restart<RetType> {
  fn restart(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::restart();
impl<'a> /*trait*/ QWizard_restart<()> for () {
  fn restart(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7restartEv()};
     unsafe {_ZN7QWizard7restartEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn back<RetType, T: QWizard_back<RetType>>(&mut self, value: T) -> RetType {
    return value.back(self);
    // return 1;
  }
}

pub trait QWizard_back<RetType> {
  fn back(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::back();
impl<'a> /*trait*/ QWizard_back<()> for () {
  fn back(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4backEv()};
     unsafe {_ZN7QWizard4backEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn sizeHint<RetType, T: QWizard_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QWizard_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QSize QWizard::sizeHint();
impl<'a> /*trait*/ QWizard_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QWizard) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QWizard8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setDefaultProperty<RetType, T: QWizard_setDefaultProperty<RetType>>(&mut self, value: T) -> RetType {
    return value.setDefaultProperty(self);
    // return 1;
  }
}

pub trait QWizard_setDefaultProperty<RetType> {
  fn setDefaultProperty(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
impl<'a> /*trait*/ QWizard_setDefaultProperty<()> for (&'a  String, &'a  String, &'a  String) {
  fn setDefaultProperty(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard18setDefaultPropertyEPKcS1_S1_()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.as_ptr()  as *const c_char;
     unsafe {_ZN7QWizard18setDefaultPropertyEPKcS1_S1_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setStartId<RetType, T: QWizard_setStartId<RetType>>(&mut self, value: T) -> RetType {
    return value.setStartId(self);
    // return 1;
  }
}

pub trait QWizard_setStartId<RetType> {
  fn setStartId(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::setStartId(int id);
impl<'a> /*trait*/ QWizard_setStartId<()> for (i32) {
  fn setStartId(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setStartIdEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard10setStartIdEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn currentIdChanged<RetType, T: QWizard_currentIdChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.currentIdChanged(self);
    // return 1;
  }
}

pub trait QWizard_currentIdChanged<RetType> {
  fn currentIdChanged(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::currentIdChanged(int id);
impl<'a> /*trait*/ QWizard_currentIdChanged<()> for (i32) {
  fn currentIdChanged(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard16currentIdChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard16currentIdChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn FreeQWizard<RetType, T: QWizard_FreeQWizard<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQWizard(self);
    // return 1;
  }
}

pub trait QWizard_FreeQWizard<RetType> {
  fn FreeQWizard(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::FreeQWizard();
impl<'a> /*trait*/ QWizard_FreeQWizard<()> for () {
  fn FreeQWizard(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizardD0Ev()};
     unsafe {_ZN7QWizardD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn pageAdded<RetType, T: QWizard_pageAdded<RetType>>(&mut self, value: T) -> RetType {
    return value.pageAdded(self);
    // return 1;
  }
}

pub trait QWizard_pageAdded<RetType> {
  fn pageAdded(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::pageAdded(int id);
impl<'a> /*trait*/ QWizard_pageAdded<()> for (i32) {
  fn pageAdded(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard9pageAddedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard9pageAddedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn visitedPages<RetType, T: QWizard_visitedPages<RetType>>(&mut self, value: T) -> RetType {
    return value.visitedPages(self);
    // return 1;
  }
}

pub trait QWizard_visitedPages<RetType> {
  fn visitedPages(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QList<int> QWizard::visitedPages();
impl<'a> /*trait*/ QWizard_visitedPages<()> for () {
  fn visitedPages(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard12visitedPagesEv()};
     unsafe {_ZNK7QWizard12visitedPagesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn nextId<RetType, T: QWizard_nextId<RetType>>(&mut self, value: T) -> RetType {
    return value.nextId(self);
    // return 1;
  }
}

pub trait QWizard_nextId<RetType> {
  fn nextId(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  int QWizard::nextId();
impl<'a> /*trait*/ QWizard_nextId<i32> for () {
  fn nextId(self, rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard6nextIdEv()};
    let mut ret = unsafe {_ZNK7QWizard6nextIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn startId<RetType, T: QWizard_startId<RetType>>(&mut self, value: T) -> RetType {
    return value.startId(self);
    // return 1;
  }
}

pub trait QWizard_startId<RetType> {
  fn startId(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  int QWizard::startId();
impl<'a> /*trait*/ QWizard_startId<i32> for () {
  fn startId(self, rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7startIdEv()};
    let mut ret = unsafe {_ZNK7QWizard7startIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn customButtonClicked<RetType, T: QWizard_customButtonClicked<RetType>>(&mut self, value: T) -> RetType {
    return value.customButtonClicked(self);
    // return 1;
  }
}

pub trait QWizard_customButtonClicked<RetType> {
  fn customButtonClicked(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::customButtonClicked(int which);
impl<'a> /*trait*/ QWizard_customButtonClicked<()> for (i32) {
  fn customButtonClicked(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19customButtonClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard19customButtonClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

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

// proto: void QWizard::NewQWizard(const QWizard & );
impl<'a> /*trait*/ QWizard_NewQWizard for (&'a  QWizard) {
  fn NewQWizard(self) -> QWizard {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizardC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWizardC1ERKS_(qthis, arg0)};
    let rsthis = QWizard{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn addPage<RetType, T: QWizard_addPage<RetType>>(&mut self, value: T) -> RetType {
    return value.addPage(self);
    // return 1;
  }
}

pub trait QWizard_addPage<RetType> {
  fn addPage(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  int QWizard::addPage(QWizardPage * page);
impl<'a> /*trait*/ QWizard_addPage<i32> for (&'a mut QWizardPage) {
  fn addPage(self, rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7addPageEP11QWizardPage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QWizard7addPageEP11QWizardPage(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn removePage<RetType, T: QWizard_removePage<RetType>>(&mut self, value: T) -> RetType {
    return value.removePage(self);
    // return 1;
  }
}

pub trait QWizard_removePage<RetType> {
  fn removePage(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::removePage(int id);
impl<'a> /*trait*/ QWizard_removePage<()> for (i32) {
  fn removePage(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10removePageEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard10removePageEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn pageIds<RetType, T: QWizard_pageIds<RetType>>(&mut self, value: T) -> RetType {
    return value.pageIds(self);
    // return 1;
  }
}

pub trait QWizard_pageIds<RetType> {
  fn pageIds(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QList<int> QWizard::pageIds();
impl<'a> /*trait*/ QWizard_pageIds<()> for () {
  fn pageIds(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7pageIdsEv()};
     unsafe {_ZNK7QWizard7pageIdsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn currentId<RetType, T: QWizard_currentId<RetType>>(&mut self, value: T) -> RetType {
    return value.currentId(self);
    // return 1;
  }
}

pub trait QWizard_currentId<RetType> {
  fn currentId(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  int QWizard::currentId();
impl<'a> /*trait*/ QWizard_currentId<i32> for () {
  fn currentId(self, rsthis: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard9currentIdEv()};
    let mut ret = unsafe {_ZNK7QWizard9currentIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn pageRemoved<RetType, T: QWizard_pageRemoved<RetType>>(&mut self, value: T) -> RetType {
    return value.pageRemoved(self);
    // return 1;
  }
}

pub trait QWizard_pageRemoved<RetType> {
  fn pageRemoved(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::pageRemoved(int id);
impl<'a> /*trait*/ QWizard_pageRemoved<()> for (i32) {
  fn pageRemoved(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard11pageRemovedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QWizard11pageRemovedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setVisible<RetType, T: QWizard_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QWizard_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  void QWizard::setVisible(bool visible);
impl<'a> /*trait*/ QWizard_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QWizard) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QWizard10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn hasVisitedPage<RetType, T: QWizard_hasVisitedPage<RetType>>(&mut self, value: T) -> RetType {
    return value.hasVisitedPage(self);
    // return 1;
  }
}

pub trait QWizard_hasVisitedPage<RetType> {
  fn hasVisitedPage(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  bool QWizard::hasVisitedPage(int id);
impl<'a> /*trait*/ QWizard_hasVisitedPage<i8> for (i32) {
  fn hasVisitedPage(self, rsthis: &mut QWizard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard14hasVisitedPageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QWizard14hasVisitedPageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn field<RetType, T: QWizard_field<RetType>>(&mut self, value: T) -> RetType {
    return value.field(self);
    // return 1;
  }
}

pub trait QWizard_field<RetType> {
  fn field(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QVariant QWizard::field(const QString & name);
impl<'a> /*trait*/ QWizard_field<QVariant> for (&'a  QString) {
  fn field(self, rsthis: &mut QWizard) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard5fieldERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QWizard5fieldERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn validateCurrentPage<RetType, T: QWizard_validateCurrentPage<RetType>>(&mut self, value: T) -> RetType {
    return value.validateCurrentPage(self);
    // return 1;
  }
}

pub trait QWizard_validateCurrentPage<RetType> {
  fn validateCurrentPage(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  bool QWizard::validateCurrentPage();
impl<'a> /*trait*/ QWizard_validateCurrentPage<i8> for () {
  fn validateCurrentPage(self, rsthis: &mut QWizard) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19validateCurrentPageEv()};
    let mut ret = unsafe {_ZN7QWizard19validateCurrentPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn sideWidget<RetType, T: QWizard_sideWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.sideWidget(self);
    // return 1;
  }
}

pub trait QWizard_sideWidget<RetType> {
  fn sideWidget(self, rsthis: &mut QWizard) -> RetType;
}

// proto:  QWidget * QWizard::sideWidget();
impl<'a> /*trait*/ QWizard_sideWidget<QWidget> for () {
  fn sideWidget(self, rsthis: &mut QWizard) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10sideWidgetEv()};
    let mut ret = unsafe {_ZNK7QWizard10sideWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

