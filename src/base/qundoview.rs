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
  // proto: void QUndoView::NewQUndoView(QUndoGroup * group, QWidget * parent);
  fn _ZN9QUndoViewC1EP10QUndoGroupP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> i32;
  // proto: void QUndoView::setStack(QUndoStack * stack);
  fn _ZN9QUndoView8setStackEP10QUndoStack(arg0: *mut c_void) -> i32;
  // proto: void QUndoView::setEmptyLabel(const QString & label);
  fn _ZN9QUndoView13setEmptyLabelERK7QString(arg0: *const c_void) -> i32;
  // proto: void QUndoView::setCleanIcon(const QIcon & icon);
  fn _ZN9QUndoView12setCleanIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: void QUndoView::setGroup(QUndoGroup * group);
  fn _ZN9QUndoView8setGroupEP10QUndoGroup(arg0: *mut c_void) -> i32;
  // proto: QUndoGroup * QUndoView::group();
  fn _ZNK9QUndoView5groupEv() -> i32;
  // proto: const QMetaObject * QUndoView::metaObject();
  fn _ZNK9QUndoView10metaObjectEv() -> i32;
  // proto: QUndoStack * QUndoView::stack();
  fn _ZNK9QUndoView5stackEv() -> i32;
  // proto: QIcon QUndoView::cleanIcon();
  fn _ZNK9QUndoView9cleanIconEv() -> i32;
  // proto: QString QUndoView::emptyLabel();
  fn _ZNK9QUndoView10emptyLabelEv() -> i32;
  // proto: void QUndoView::NewQUndoView(const QUndoView & );
  fn _ZN9QUndoViewC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QUndoView::NewQUndoView(QWidget * parent);
  fn _ZN9QUndoViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QUndoView::FreeQUndoView();
  fn _ZN9QUndoViewD0Ev() -> i32;
  // proto: void QUndoView::NewQUndoView(QUndoStack * stack, QWidget * parent);
  fn _ZN9QUndoViewC1EP10QUndoStackP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> i32;
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
  pub fn setStack<T: QUndoView_setStack>(&mut self, value: T) -> i32 {
    value.setStack(self);
    return 1;
  }
}

pub trait QUndoView_setStack {
  fn setStack(self, this: &mut QUndoView) -> i32;
}

// proto: void QUndoView::setStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoView_setStack for (&'a mut QUndoStack) {
  fn setStack(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView8setStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUndoView8setStackEP10QUndoStack(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setEmptyLabel<T: QUndoView_setEmptyLabel>(&mut self, value: T) -> i32 {
    value.setEmptyLabel(self);
    return 1;
  }
}

pub trait QUndoView_setEmptyLabel {
  fn setEmptyLabel(self, this: &mut QUndoView) -> i32;
}

// proto: void QUndoView::setEmptyLabel(const QString & label);
impl<'a> /*trait*/ QUndoView_setEmptyLabel for (&'a  QString) {
  fn setEmptyLabel(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView13setEmptyLabelERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUndoView13setEmptyLabelERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setCleanIcon<T: QUndoView_setCleanIcon>(&mut self, value: T) -> i32 {
    value.setCleanIcon(self);
    return 1;
  }
}

pub trait QUndoView_setCleanIcon {
  fn setCleanIcon(self, this: &mut QUndoView) -> i32;
}

// proto: void QUndoView::setCleanIcon(const QIcon & icon);
impl<'a> /*trait*/ QUndoView_setCleanIcon for (&'a  QIcon) {
  fn setCleanIcon(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView12setCleanIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN9QUndoView12setCleanIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn setGroup<T: QUndoView_setGroup>(&mut self, value: T) -> i32 {
    value.setGroup(self);
    return 1;
  }
}

pub trait QUndoView_setGroup {
  fn setGroup(self, this: &mut QUndoView) -> i32;
}

// proto: void QUndoView::setGroup(QUndoGroup * group);
impl<'a> /*trait*/ QUndoView_setGroup for (&'a mut QUndoGroup) {
  fn setGroup(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView8setGroupEP10QUndoGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QUndoView8setGroupEP10QUndoGroup(arg0)};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn group<T: QUndoView_group>(&mut self, value: T) -> i32 {
    value.group(self);
    return 1;
  }
}

pub trait QUndoView_group {
  fn group(self, this: &mut QUndoView) -> i32;
}

// proto: QUndoGroup * QUndoView::group();
impl<'a> /*trait*/ QUndoView_group for () {
  fn group(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView5groupEv()};
    unsafe {_ZNK9QUndoView5groupEv()};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn metaObject<T: QUndoView_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QUndoView_metaObject {
  fn metaObject(self, this: &mut QUndoView) -> i32;
}

// proto: const QMetaObject * QUndoView::metaObject();
impl<'a> /*trait*/ QUndoView_metaObject for () {
  fn metaObject(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView10metaObjectEv()};
    unsafe {_ZNK9QUndoView10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn stack<T: QUndoView_stack>(&mut self, value: T) -> i32 {
    value.stack(self);
    return 1;
  }
}

pub trait QUndoView_stack {
  fn stack(self, this: &mut QUndoView) -> i32;
}

// proto: QUndoStack * QUndoView::stack();
impl<'a> /*trait*/ QUndoView_stack for () {
  fn stack(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView5stackEv()};
    unsafe {_ZNK9QUndoView5stackEv()};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn cleanIcon<T: QUndoView_cleanIcon>(&mut self, value: T) -> i32 {
    value.cleanIcon(self);
    return 1;
  }
}

pub trait QUndoView_cleanIcon {
  fn cleanIcon(self, this: &mut QUndoView) -> i32;
}

// proto: QIcon QUndoView::cleanIcon();
impl<'a> /*trait*/ QUndoView_cleanIcon for () {
  fn cleanIcon(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView9cleanIconEv()};
    unsafe {_ZNK9QUndoView9cleanIconEv()};
    return 1;
  }
}

impl /*struct*/ QUndoView {
  pub fn emptyLabel<T: QUndoView_emptyLabel>(&mut self, value: T) -> i32 {
    value.emptyLabel(self);
    return 1;
  }
}

pub trait QUndoView_emptyLabel {
  fn emptyLabel(self, this: &mut QUndoView) -> i32;
}

// proto: QString QUndoView::emptyLabel();
impl<'a> /*trait*/ QUndoView_emptyLabel for () {
  fn emptyLabel(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView10emptyLabelEv()};
    unsafe {_ZNK9QUndoView10emptyLabelEv()};
    return 1;
  }
}

// proto: void QUndoView::NewQUndoView(const QUndoView & );
impl<'a> /*trait*/ QUndoView_NewQUndoView for (&'a  QUndoView) {
  fn NewQUndoView(self) -> QUndoView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
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
  pub fn FreeQUndoView<T: QUndoView_FreeQUndoView>(&mut self, value: T) -> i32 {
    value.FreeQUndoView(self);
    return 1;
  }
}

pub trait QUndoView_FreeQUndoView {
  fn FreeQUndoView(self, this: &mut QUndoView) -> i32;
}

// proto: void QUndoView::FreeQUndoView();
impl<'a> /*trait*/ QUndoView_FreeQUndoView for () {
  fn FreeQUndoView(self, this: &mut QUndoView) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewD0Ev()};
    unsafe {_ZN9QUndoViewD0Ev()};
    return 1;
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

