// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qundogroup::QUndoGroup;
use super::qwidget::QWidget;
use super::qundostack::QUndoStack;
use super::qstring::QString;
use super::qicon::QIcon;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QUndoView::NewQUndoView(QUndoGroup * group, QWidget * parent);
  fn _ZN9QUndoViewC1EP10QUndoGroupP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QUndoView::setStack(QUndoStack * stack);
  fn _ZN9QUndoView8setStackEP10QUndoStack(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoView::setEmptyLabel(const QString & label);
  fn _ZN9QUndoView13setEmptyLabelERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoView::setCleanIcon(const QIcon & icon);
  fn _ZN9QUndoView12setCleanIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoView::setGroup(QUndoGroup * group);
  fn _ZN9QUndoView8setGroupEP10QUndoGroup(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QUndoGroup * QUndoView::group();
  fn _ZNK9QUndoView5groupEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QUndoView::metaObject();
  fn _ZNK9QUndoView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QUndoStack * QUndoView::stack();
  fn _ZNK9QUndoView5stackEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIcon QUndoView::cleanIcon();
  fn _ZNK9QUndoView9cleanIconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QUndoView::emptyLabel();
  fn _ZNK9QUndoView10emptyLabelEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUndoView::NewQUndoView(const QUndoView & );
  fn _ZN9QUndoViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoView::NewQUndoView(QWidget * parent);
  fn _ZN9QUndoViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QUndoView::FreeQUndoView();
  fn _ZN9QUndoViewD0Ev(qthis: *mut c_void) ;
  // proto:  void QUndoView::NewQUndoView(QUndoStack * stack, QWidget * parent);
  fn _ZN9QUndoViewC1EP10QUndoStackP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QUndoView)=1
pub struct QUndoView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUndoView {
  pub fn NewQUndoView<T: QUndoView_NewQUndoView>(value: T) -> QUndoView {
    let rsthis = value.NewQUndoView();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoView_NewQUndoView {
  fn NewQUndoView(self) -> QUndoView;
}

// proto: void QUndoView::NewQUndoView(QUndoGroup * group, QWidget * parent);
impl<'a> /*trait*/ QUndoView_NewQUndoView for (&'a mut QUndoGroup, &'a mut QWidget) {
  fn NewQUndoView(self) -> QUndoView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC1EP10QUndoGroupP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QUndoViewC1EP10QUndoGroupP7QWidget(qthis, arg0, arg1)};
    let rsthis = QUndoView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setStack<T: QUndoView_setStack>(&mut self, value: T)  {
     value.setStack(self);
    // return 1;
  }
}

pub trait QUndoView_setStack {
  fn setStack(self, rsthis: &mut QUndoView) ;
}

// proto:  void QUndoView::setStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoView_setStack for (&'a mut QUndoStack) {
  fn setStack(self, rsthis: &mut QUndoView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView8setStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUndoView8setStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setEmptyLabel<T: QUndoView_setEmptyLabel>(&mut self, value: T)  {
     value.setEmptyLabel(self);
    // return 1;
  }
}

pub trait QUndoView_setEmptyLabel {
  fn setEmptyLabel(self, rsthis: &mut QUndoView) ;
}

// proto:  void QUndoView::setEmptyLabel(const QString & label);
impl<'a> /*trait*/ QUndoView_setEmptyLabel for (&'a  QString) {
  fn setEmptyLabel(self, rsthis: &mut QUndoView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView13setEmptyLabelERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUndoView13setEmptyLabelERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setCleanIcon<T: QUndoView_setCleanIcon>(&mut self, value: T)  {
     value.setCleanIcon(self);
    // return 1;
  }
}

pub trait QUndoView_setCleanIcon {
  fn setCleanIcon(self, rsthis: &mut QUndoView) ;
}

// proto:  void QUndoView::setCleanIcon(const QIcon & icon);
impl<'a> /*trait*/ QUndoView_setCleanIcon for (&'a  QIcon) {
  fn setCleanIcon(self, rsthis: &mut QUndoView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView12setCleanIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUndoView12setCleanIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setGroup<T: QUndoView_setGroup>(&mut self, value: T)  {
     value.setGroup(self);
    // return 1;
  }
}

pub trait QUndoView_setGroup {
  fn setGroup(self, rsthis: &mut QUndoView) ;
}

// proto:  void QUndoView::setGroup(QUndoGroup * group);
impl<'a> /*trait*/ QUndoView_setGroup for (&'a mut QUndoGroup) {
  fn setGroup(self, rsthis: &mut QUndoView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView8setGroupEP10QUndoGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QUndoView8setGroupEP10QUndoGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn group<T: QUndoView_group>(&mut self, value: T) -> QUndoGroup {
    return value.group(self);
    // return 1;
  }
}

pub trait QUndoView_group {
  fn group(self, rsthis: &mut QUndoView) -> QUndoGroup;
}

// proto:  QUndoGroup * QUndoView::group();
impl<'a> /*trait*/ QUndoView_group for () {
  fn group(self, rsthis: &mut QUndoView) -> QUndoGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView5groupEv()};
    let mut ret = unsafe {_ZNK9QUndoView5groupEv(rsthis.qclsinst)};
    let mut ret1 = QUndoGroup{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn metaObject<T: QUndoView_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QUndoView_metaObject {
  fn metaObject(self, rsthis: &mut QUndoView) ;
}

// proto:  const QMetaObject * QUndoView::metaObject();
impl<'a> /*trait*/ QUndoView_metaObject for () {
  fn metaObject(self, rsthis: &mut QUndoView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView10metaObjectEv()};
     unsafe {_ZNK9QUndoView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn stack<T: QUndoView_stack>(&mut self, value: T) -> QUndoStack {
    return value.stack(self);
    // return 1;
  }
}

pub trait QUndoView_stack {
  fn stack(self, rsthis: &mut QUndoView) -> QUndoStack;
}

// proto:  QUndoStack * QUndoView::stack();
impl<'a> /*trait*/ QUndoView_stack for () {
  fn stack(self, rsthis: &mut QUndoView) -> QUndoStack {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView5stackEv()};
    let mut ret = unsafe {_ZNK9QUndoView5stackEv(rsthis.qclsinst)};
    let mut ret1 = QUndoStack{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn cleanIcon<T: QUndoView_cleanIcon>(&mut self, value: T) -> QIcon {
    return value.cleanIcon(self);
    // return 1;
  }
}

pub trait QUndoView_cleanIcon {
  fn cleanIcon(self, rsthis: &mut QUndoView) -> QIcon;
}

// proto:  QIcon QUndoView::cleanIcon();
impl<'a> /*trait*/ QUndoView_cleanIcon for () {
  fn cleanIcon(self, rsthis: &mut QUndoView) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView9cleanIconEv()};
    let mut ret = unsafe {_ZNK9QUndoView9cleanIconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn emptyLabel<T: QUndoView_emptyLabel>(&mut self, value: T) -> QString {
    return value.emptyLabel(self);
    // return 1;
  }
}

pub trait QUndoView_emptyLabel {
  fn emptyLabel(self, rsthis: &mut QUndoView) -> QString;
}

// proto:  QString QUndoView::emptyLabel();
impl<'a> /*trait*/ QUndoView_emptyLabel for () {
  fn emptyLabel(self, rsthis: &mut QUndoView) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView10emptyLabelEv()};
    let mut ret = unsafe {_ZNK9QUndoView10emptyLabelEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QUndoView::NewQUndoView(const QUndoView & );
impl<'a> /*trait*/ QUndoView_NewQUndoView for (&'a  QUndoView) {
  fn NewQUndoView(self) -> QUndoView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUndoViewC1ERKS_(qthis, arg0)};
    let rsthis = QUndoView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QUndoView::NewQUndoView(QWidget * parent);
impl<'a> /*trait*/ QUndoView_NewQUndoView for (&'a mut QWidget) {
  fn NewQUndoView(self) -> QUndoView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUndoViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QUndoView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn FreeQUndoView<T: QUndoView_FreeQUndoView>(&mut self, value: T)  {
     value.FreeQUndoView(self);
    // return 1;
  }
}

pub trait QUndoView_FreeQUndoView {
  fn FreeQUndoView(self, rsthis: &mut QUndoView) ;
}

// proto:  void QUndoView::FreeQUndoView();
impl<'a> /*trait*/ QUndoView_FreeQUndoView for () {
  fn FreeQUndoView(self, rsthis: &mut QUndoView)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewD0Ev()};
     unsafe {_ZN9QUndoViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QUndoView::NewQUndoView(QUndoStack * stack, QWidget * parent);
impl<'a> /*trait*/ QUndoView_NewQUndoView for (&'a mut QUndoStack, &'a mut QWidget) {
  fn NewQUndoView(self) -> QUndoView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC1EP10QUndoStackP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QUndoViewC1EP10QUndoStackP7QWidget(qthis, arg0, arg1)};
    let rsthis = QUndoView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

