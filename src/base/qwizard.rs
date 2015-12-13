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
use super::qvariant::QVariant;
use super::qwizardpage::QWizardPage;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QWizard::setSideWidget(QWidget * widget);
  fn _ZN7QWizard13setSideWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QWizardPage * QWizard::currentPage();
  fn _ZNK7QWizard11currentPageEv() -> i32;
  // proto: void QWizard::next();
  fn _ZN7QWizard4nextEv() -> i32;
  // proto: void QWizard::helpRequested();
  fn _ZN7QWizard13helpRequestedEv() -> i32;
  // proto: QWizardPage * QWizard::page(int id);
  fn _ZNK7QWizard4pageEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QWizard::metaObject();
  fn _ZNK7QWizard10metaObjectEv() -> i32;
  // proto: void QWizard::setField(const QString & name, const QVariant & value);
  fn _ZN7QWizard8setFieldERK7QStringRK8QVariant(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QWizard::setPage(int id, QWizardPage * page);
  fn _ZN7QWizard7setPageEiP11QWizardPage(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QWizard::restart();
  fn _ZN7QWizard7restartEv() -> i32;
  // proto: void QWizard::back();
  fn _ZN7QWizard4backEv() -> i32;
  // proto: QSize QWizard::sizeHint();
  fn _ZNK7QWizard8sizeHintEv() -> i32;
  // proto: void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
  fn _ZN7QWizard18setDefaultPropertyEPKcS1_S1_(arg0: *const c_char, arg1: *const c_char, arg2: *const c_char) -> i32;
  // proto: void QWizard::setStartId(int id);
  fn _ZN7QWizard10setStartIdEi(arg0: c_int) -> i32;
  // proto: void QWizard::currentIdChanged(int id);
  fn _ZN7QWizard16currentIdChangedEi(arg0: c_int) -> i32;
  // proto: void QWizard::FreeQWizard();
  fn _ZN7QWizardD0Ev() -> i32;
  // proto: void QWizard::pageAdded(int id);
  fn _ZN7QWizard9pageAddedEi(arg0: c_int) -> i32;
  // proto: QList<int> QWizard::visitedPages();
  fn _ZNK7QWizard12visitedPagesEv() -> i32;
  // proto: int QWizard::nextId();
  fn _ZNK7QWizard6nextIdEv() -> i32;
  // proto: int QWizard::startId();
  fn _ZNK7QWizard7startIdEv() -> i32;
  // proto: void QWizard::customButtonClicked(int which);
  fn _ZN7QWizard19customButtonClickedEi(arg0: c_int) -> i32;
  // proto: void QWizard::NewQWizard(const QWizard & );
  fn _ZN7QWizardC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QWizard::addPage(QWizardPage * page);
  fn _ZN7QWizard7addPageEP11QWizardPage(arg0: *mut c_void) -> i32;
  // proto: void QWizard::removePage(int id);
  fn _ZN7QWizard10removePageEi(arg0: c_int) -> i32;
  // proto: QList<int> QWizard::pageIds();
  fn _ZNK7QWizard7pageIdsEv() -> i32;
  // proto: int QWizard::currentId();
  fn _ZNK7QWizard9currentIdEv() -> i32;
  // proto: void QWizard::pageRemoved(int id);
  fn _ZN7QWizard11pageRemovedEi(arg0: c_int) -> i32;
  // proto: void QWizard::setVisible(bool visible);
  fn _ZN7QWizard10setVisibleEb(arg0: int8_t) -> i32;
  // proto: bool QWizard::hasVisitedPage(int id);
  fn _ZNK7QWizard14hasVisitedPageEi(arg0: c_int) -> i32;
  // proto: QVariant QWizard::field(const QString & name);
  fn _ZNK7QWizard5fieldERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QWizard::validateCurrentPage();
  fn _ZN7QWizard19validateCurrentPageEv() -> i32;
  // proto: QWidget * QWizard::sideWidget();
  fn _ZNK7QWizard10sideWidgetEv() -> i32;
}

// body block begin
// class sizeof(QWizard)=1
pub struct QWizard {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QWizard {
  pub fn setSideWidget<T: QWizard_setSideWidget>(&mut self, value: T) -> i32 {
    value.setSideWidget(self);
    return 1;
  }
}

pub trait QWizard_setSideWidget {
  fn setSideWidget(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::setSideWidget(QWidget * widget);
impl<'a> /*trait*/ QWizard_setSideWidget for (&'a mut QWidget) {
  fn setSideWidget(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13setSideWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWizard13setSideWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn currentPage<T: QWizard_currentPage>(&mut self, value: T) -> i32 {
    value.currentPage(self);
    return 1;
  }
}

pub trait QWizard_currentPage {
  fn currentPage(self, this: &mut QWizard) -> i32;
}

// proto: QWizardPage * QWizard::currentPage();
impl<'a> /*trait*/ QWizard_currentPage for () {
  fn currentPage(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard11currentPageEv()};
    unsafe {_ZNK7QWizard11currentPageEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn next<T: QWizard_next>(&mut self, value: T) -> i32 {
    value.next(self);
    return 1;
  }
}

pub trait QWizard_next {
  fn next(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::next();
impl<'a> /*trait*/ QWizard_next for () {
  fn next(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4nextEv()};
    unsafe {_ZN7QWizard4nextEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn helpRequested<T: QWizard_helpRequested>(&mut self, value: T) -> i32 {
    value.helpRequested(self);
    return 1;
  }
}

pub trait QWizard_helpRequested {
  fn helpRequested(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::helpRequested();
impl<'a> /*trait*/ QWizard_helpRequested for () {
  fn helpRequested(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard13helpRequestedEv()};
    unsafe {_ZN7QWizard13helpRequestedEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn page<T: QWizard_page>(&mut self, value: T) -> i32 {
    value.page(self);
    return 1;
  }
}

pub trait QWizard_page {
  fn page(self, this: &mut QWizard) -> i32;
}

// proto: QWizardPage * QWizard::page(int id);
impl<'a> /*trait*/ QWizard_page for (i32) {
  fn page(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard4pageEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QWizard4pageEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn metaObject<T: QWizard_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QWizard_metaObject {
  fn metaObject(self, this: &mut QWizard) -> i32;
}

// proto: const QMetaObject * QWizard::metaObject();
impl<'a> /*trait*/ QWizard_metaObject for () {
  fn metaObject(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10metaObjectEv()};
    unsafe {_ZNK7QWizard10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setField<T: QWizard_setField>(&mut self, value: T) -> i32 {
    value.setField(self);
    return 1;
  }
}

pub trait QWizard_setField {
  fn setField(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::setField(const QString & name, const QVariant & value);
impl<'a> /*trait*/ QWizard_setField for (&'a  QString, &'a  QVariant) {
  fn setField(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard8setFieldERK7QStringRK8QVariant()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QWizard8setFieldERK7QStringRK8QVariant(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setPage<T: QWizard_setPage>(&mut self, value: T) -> i32 {
    value.setPage(self);
    return 1;
  }
}

pub trait QWizard_setPage {
  fn setPage(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::setPage(int id, QWizardPage * page);
impl<'a> /*trait*/ QWizard_setPage for (i32, &'a mut QWizardPage) {
  fn setPage(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7setPageEiP11QWizardPage()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QWizard7setPageEiP11QWizardPage(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn restart<T: QWizard_restart>(&mut self, value: T) -> i32 {
    value.restart(self);
    return 1;
  }
}

pub trait QWizard_restart {
  fn restart(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::restart();
impl<'a> /*trait*/ QWizard_restart for () {
  fn restart(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7restartEv()};
    unsafe {_ZN7QWizard7restartEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn back<T: QWizard_back>(&mut self, value: T) -> i32 {
    value.back(self);
    return 1;
  }
}

pub trait QWizard_back {
  fn back(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::back();
impl<'a> /*trait*/ QWizard_back for () {
  fn back(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard4backEv()};
    unsafe {_ZN7QWizard4backEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn sizeHint<T: QWizard_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QWizard_sizeHint {
  fn sizeHint(self, this: &mut QWizard) -> i32;
}

// proto: QSize QWizard::sizeHint();
impl<'a> /*trait*/ QWizard_sizeHint for () {
  fn sizeHint(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard8sizeHintEv()};
    unsafe {_ZNK7QWizard8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setDefaultProperty<T: QWizard_setDefaultProperty>(&mut self, value: T) -> i32 {
    value.setDefaultProperty(self);
    return 1;
  }
}

pub trait QWizard_setDefaultProperty {
  fn setDefaultProperty(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::setDefaultProperty(const char * className, const char * property, const char * changedSignal);
impl<'a> /*trait*/ QWizard_setDefaultProperty for (&'a  String, &'a  String, &'a  String) {
  fn setDefaultProperty(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard18setDefaultPropertyEPKcS1_S1_()};
    let arg0 = self.0.as_ptr()  as *const c_char;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN7QWizard18setDefaultPropertyEPKcS1_S1_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setStartId<T: QWizard_setStartId>(&mut self, value: T) -> i32 {
    value.setStartId(self);
    return 1;
  }
}

pub trait QWizard_setStartId {
  fn setStartId(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::setStartId(int id);
impl<'a> /*trait*/ QWizard_setStartId for (i32) {
  fn setStartId(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setStartIdEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWizard10setStartIdEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn currentIdChanged<T: QWizard_currentIdChanged>(&mut self, value: T) -> i32 {
    value.currentIdChanged(self);
    return 1;
  }
}

pub trait QWizard_currentIdChanged {
  fn currentIdChanged(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::currentIdChanged(int id);
impl<'a> /*trait*/ QWizard_currentIdChanged for (i32) {
  fn currentIdChanged(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard16currentIdChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWizard16currentIdChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn FreeQWizard<T: QWizard_FreeQWizard>(&mut self, value: T) -> i32 {
    value.FreeQWizard(self);
    return 1;
  }
}

pub trait QWizard_FreeQWizard {
  fn FreeQWizard(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::FreeQWizard();
impl<'a> /*trait*/ QWizard_FreeQWizard for () {
  fn FreeQWizard(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizardD0Ev()};
    unsafe {_ZN7QWizardD0Ev()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn pageAdded<T: QWizard_pageAdded>(&mut self, value: T) -> i32 {
    value.pageAdded(self);
    return 1;
  }
}

pub trait QWizard_pageAdded {
  fn pageAdded(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::pageAdded(int id);
impl<'a> /*trait*/ QWizard_pageAdded for (i32) {
  fn pageAdded(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard9pageAddedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWizard9pageAddedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn visitedPages<T: QWizard_visitedPages>(&mut self, value: T) -> i32 {
    value.visitedPages(self);
    return 1;
  }
}

pub trait QWizard_visitedPages {
  fn visitedPages(self, this: &mut QWizard) -> i32;
}

// proto: QList<int> QWizard::visitedPages();
impl<'a> /*trait*/ QWizard_visitedPages for () {
  fn visitedPages(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard12visitedPagesEv()};
    unsafe {_ZNK7QWizard12visitedPagesEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn nextId<T: QWizard_nextId>(&mut self, value: T) -> i32 {
    value.nextId(self);
    return 1;
  }
}

pub trait QWizard_nextId {
  fn nextId(self, this: &mut QWizard) -> i32;
}

// proto: int QWizard::nextId();
impl<'a> /*trait*/ QWizard_nextId for () {
  fn nextId(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard6nextIdEv()};
    unsafe {_ZNK7QWizard6nextIdEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn startId<T: QWizard_startId>(&mut self, value: T) -> i32 {
    value.startId(self);
    return 1;
  }
}

pub trait QWizard_startId {
  fn startId(self, this: &mut QWizard) -> i32;
}

// proto: int QWizard::startId();
impl<'a> /*trait*/ QWizard_startId for () {
  fn startId(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7startIdEv()};
    unsafe {_ZNK7QWizard7startIdEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn customButtonClicked<T: QWizard_customButtonClicked>(&mut self, value: T) -> i32 {
    value.customButtonClicked(self);
    return 1;
  }
}

pub trait QWizard_customButtonClicked {
  fn customButtonClicked(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::customButtonClicked(int which);
impl<'a> /*trait*/ QWizard_customButtonClicked for (i32) {
  fn customButtonClicked(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19customButtonClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWizard19customButtonClickedEi(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QWizardC1ERKS_(qthis, arg0)};
    let rsthis = QWizard{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn addPage<T: QWizard_addPage>(&mut self, value: T) -> i32 {
    value.addPage(self);
    return 1;
  }
}

pub trait QWizard_addPage {
  fn addPage(self, this: &mut QWizard) -> i32;
}

// proto: int QWizard::addPage(QWizardPage * page);
impl<'a> /*trait*/ QWizard_addPage for (&'a mut QWizardPage) {
  fn addPage(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard7addPageEP11QWizardPage()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QWizard7addPageEP11QWizardPage(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn removePage<T: QWizard_removePage>(&mut self, value: T) -> i32 {
    value.removePage(self);
    return 1;
  }
}

pub trait QWizard_removePage {
  fn removePage(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::removePage(int id);
impl<'a> /*trait*/ QWizard_removePage for (i32) {
  fn removePage(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10removePageEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWizard10removePageEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn pageIds<T: QWizard_pageIds>(&mut self, value: T) -> i32 {
    value.pageIds(self);
    return 1;
  }
}

pub trait QWizard_pageIds {
  fn pageIds(self, this: &mut QWizard) -> i32;
}

// proto: QList<int> QWizard::pageIds();
impl<'a> /*trait*/ QWizard_pageIds for () {
  fn pageIds(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard7pageIdsEv()};
    unsafe {_ZNK7QWizard7pageIdsEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn currentId<T: QWizard_currentId>(&mut self, value: T) -> i32 {
    value.currentId(self);
    return 1;
  }
}

pub trait QWizard_currentId {
  fn currentId(self, this: &mut QWizard) -> i32;
}

// proto: int QWizard::currentId();
impl<'a> /*trait*/ QWizard_currentId for () {
  fn currentId(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard9currentIdEv()};
    unsafe {_ZNK7QWizard9currentIdEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn pageRemoved<T: QWizard_pageRemoved>(&mut self, value: T) -> i32 {
    value.pageRemoved(self);
    return 1;
  }
}

pub trait QWizard_pageRemoved {
  fn pageRemoved(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::pageRemoved(int id);
impl<'a> /*trait*/ QWizard_pageRemoved for (i32) {
  fn pageRemoved(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard11pageRemovedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QWizard11pageRemovedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn setVisible<T: QWizard_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QWizard_setVisible {
  fn setVisible(self, this: &mut QWizard) -> i32;
}

// proto: void QWizard::setVisible(bool visible);
impl<'a> /*trait*/ QWizard_setVisible for (i8) {
  fn setVisible(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QWizard10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn hasVisitedPage<T: QWizard_hasVisitedPage>(&mut self, value: T) -> i32 {
    value.hasVisitedPage(self);
    return 1;
  }
}

pub trait QWizard_hasVisitedPage {
  fn hasVisitedPage(self, this: &mut QWizard) -> i32;
}

// proto: bool QWizard::hasVisitedPage(int id);
impl<'a> /*trait*/ QWizard_hasVisitedPage for (i32) {
  fn hasVisitedPage(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard14hasVisitedPageEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QWizard14hasVisitedPageEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn field<T: QWizard_field>(&mut self, value: T) -> i32 {
    value.field(self);
    return 1;
  }
}

pub trait QWizard_field {
  fn field(self, this: &mut QWizard) -> i32;
}

// proto: QVariant QWizard::field(const QString & name);
impl<'a> /*trait*/ QWizard_field for (&'a  QString) {
  fn field(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard5fieldERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QWizard5fieldERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn validateCurrentPage<T: QWizard_validateCurrentPage>(&mut self, value: T) -> i32 {
    value.validateCurrentPage(self);
    return 1;
  }
}

pub trait QWizard_validateCurrentPage {
  fn validateCurrentPage(self, this: &mut QWizard) -> i32;
}

// proto: bool QWizard::validateCurrentPage();
impl<'a> /*trait*/ QWizard_validateCurrentPage for () {
  fn validateCurrentPage(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QWizard19validateCurrentPageEv()};
    unsafe {_ZN7QWizard19validateCurrentPageEv()};
    return 1;
  }
}

impl /*struct*/ QWizard {
  pub fn sideWidget<T: QWizard_sideWidget>(&mut self, value: T) -> i32 {
    value.sideWidget(self);
    return 1;
  }
}

pub trait QWizard_sideWidget {
  fn sideWidget(self, this: &mut QWizard) -> i32;
}

// proto: QWidget * QWizard::sideWidget();
impl<'a> /*trait*/ QWizard_sideWidget for () {
  fn sideWidget(self, this: &mut QWizard) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QWizard10sideWidgetEv()};
    unsafe {_ZNK7QWizard10sideWidgetEv()};
    return 1;
  }
}

