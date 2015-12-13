// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qtextdocument::QTextDocument;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTextBlockGroup::NewQTextBlockGroup(const QTextBlockGroup & );
  fn _ZN15QTextBlockGroupC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QTextBlockGroup::metaObject();
  fn _ZNK15QTextBlockGroup10metaObjectEv() -> i32;
  // proto: void QTextBlockGroup::FreeQTextBlockGroup();
  fn _ZN15QTextBlockGroupD0Ev() -> i32;
  // proto: void QTextBlockGroup::NewQTextBlockGroup(QTextDocument * doc);
  fn _ZN15QTextBlockGroupC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QTextBlockGroup)=1
pub struct QTextBlockGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBlockGroup {
  pub fn NewQTextBlockGroup<T: QTextBlockGroup_NewQTextBlockGroup>(value: T) -> QTextBlockGroup {
    let rsthis = value.NewQTextBlockGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBlockGroup_NewQTextBlockGroup {
  fn NewQTextBlockGroup(self) -> QTextBlockGroup;
}

// proto: void QTextBlockGroup::NewQTextBlockGroup(const QTextBlockGroup & );
impl<'a> /*trait*/ QTextBlockGroup_NewQTextBlockGroup for (&'a  QTextBlockGroup) {
  fn NewQTextBlockGroup(self) -> QTextBlockGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QTextBlockGroupC1ERKS_(qthis, arg0)};
    let rsthis = QTextBlockGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBlockGroup {
  pub fn metaObject<T: QTextBlockGroup_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTextBlockGroup_metaObject {
  fn metaObject(self, this: &mut QTextBlockGroup) -> i32;
}

// proto: const QMetaObject * QTextBlockGroup::metaObject();
impl<'a> /*trait*/ QTextBlockGroup_metaObject for () {
  fn metaObject(self, this: &mut QTextBlockGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QTextBlockGroup10metaObjectEv()};
    unsafe {_ZNK15QTextBlockGroup10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTextBlockGroup {
  pub fn FreeQTextBlockGroup<T: QTextBlockGroup_FreeQTextBlockGroup>(&mut self, value: T) -> i32 {
    value.FreeQTextBlockGroup(self);
    return 1;
  }
}

pub trait QTextBlockGroup_FreeQTextBlockGroup {
  fn FreeQTextBlockGroup(self, this: &mut QTextBlockGroup) -> i32;
}

// proto: void QTextBlockGroup::FreeQTextBlockGroup();
impl<'a> /*trait*/ QTextBlockGroup_FreeQTextBlockGroup for () {
  fn FreeQTextBlockGroup(self, this: &mut QTextBlockGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupD0Ev()};
    unsafe {_ZN15QTextBlockGroupD0Ev()};
    return 1;
  }
}

// proto: void QTextBlockGroup::NewQTextBlockGroup(QTextDocument * doc);
impl<'a> /*trait*/ QTextBlockGroup_NewQTextBlockGroup for (&'a mut QTextDocument) {
  fn NewQTextBlockGroup(self) -> QTextBlockGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QTextBlockGroupC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QTextBlockGroupC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QTextBlockGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

