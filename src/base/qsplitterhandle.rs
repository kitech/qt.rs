// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QSplitterHandle::FreeQSplitterHandle();
  fn _ZN15QSplitterHandleD0Ev() -> i32;
  // proto: void QSplitterHandle::NewQSplitterHandle(const QSplitterHandle & );
  fn _ZN15QSplitterHandleC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QSize QSplitterHandle::sizeHint();
  fn _ZNK15QSplitterHandle8sizeHintEv() -> i32;
  // proto: bool QSplitterHandle::opaqueResize();
  fn _ZNK15QSplitterHandle12opaqueResizeEv() -> i32;
  // proto: QSplitter * QSplitterHandle::splitter();
  fn _ZNK15QSplitterHandle8splitterEv() -> i32;
  // proto: const QMetaObject * QSplitterHandle::metaObject();
  fn _ZNK15QSplitterHandle10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QSplitterHandle)=1
pub struct QSplitterHandle {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSplitterHandle {
  pub fn FreeQSplitterHandle<T: QSplitterHandle_FreeQSplitterHandle>(&mut self, value: T) -> i32 {
    value.FreeQSplitterHandle(self);
    return 1;
  }
}

pub trait QSplitterHandle_FreeQSplitterHandle {
  fn FreeQSplitterHandle(self, this: &mut QSplitterHandle) -> i32;
}

// proto: void QSplitterHandle::FreeQSplitterHandle();
impl<'a> /*trait*/ QSplitterHandle_FreeQSplitterHandle for () {
  fn FreeQSplitterHandle(self, this: &mut QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSplitterHandleD0Ev()};
    unsafe {_ZN15QSplitterHandleD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn NewQSplitterHandle<T: QSplitterHandle_NewQSplitterHandle>(value: T) -> QSplitterHandle {
    let rsthis = value.NewQSplitterHandle();
    return rsthis;
    // return 1;
  }
}

pub trait QSplitterHandle_NewQSplitterHandle {
  fn NewQSplitterHandle(self) -> QSplitterHandle;
}

// proto: void QSplitterHandle::NewQSplitterHandle(const QSplitterHandle & );
impl<'a> /*trait*/ QSplitterHandle_NewQSplitterHandle for (&'a  QSplitterHandle) {
  fn NewQSplitterHandle(self) -> QSplitterHandle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSplitterHandleC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSplitterHandleC1ERKS_(qthis, arg0)};
    let rsthis = QSplitterHandle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn sizeHint<T: QSplitterHandle_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QSplitterHandle_sizeHint {
  fn sizeHint(self, this: &mut QSplitterHandle) -> i32;
}

// proto: QSize QSplitterHandle::sizeHint();
impl<'a> /*trait*/ QSplitterHandle_sizeHint for () {
  fn sizeHint(self, this: &mut QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8sizeHintEv()};
    unsafe {_ZNK15QSplitterHandle8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn opaqueResize<T: QSplitterHandle_opaqueResize>(&mut self, value: T) -> i32 {
    value.opaqueResize(self);
    return 1;
  }
}

pub trait QSplitterHandle_opaqueResize {
  fn opaqueResize(self, this: &mut QSplitterHandle) -> i32;
}

// proto: bool QSplitterHandle::opaqueResize();
impl<'a> /*trait*/ QSplitterHandle_opaqueResize for () {
  fn opaqueResize(self, this: &mut QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle12opaqueResizeEv()};
    unsafe {_ZNK15QSplitterHandle12opaqueResizeEv()};
    return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn splitter<T: QSplitterHandle_splitter>(&mut self, value: T) -> i32 {
    value.splitter(self);
    return 1;
  }
}

pub trait QSplitterHandle_splitter {
  fn splitter(self, this: &mut QSplitterHandle) -> i32;
}

// proto: QSplitter * QSplitterHandle::splitter();
impl<'a> /*trait*/ QSplitterHandle_splitter for () {
  fn splitter(self, this: &mut QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8splitterEv()};
    unsafe {_ZNK15QSplitterHandle8splitterEv()};
    return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn metaObject<T: QSplitterHandle_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSplitterHandle_metaObject {
  fn metaObject(self, this: &mut QSplitterHandle) -> i32;
}

// proto: const QMetaObject * QSplitterHandle::metaObject();
impl<'a> /*trait*/ QSplitterHandle_metaObject for () {
  fn metaObject(self, this: &mut QSplitterHandle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle10metaObjectEv()};
    unsafe {_ZNK15QSplitterHandle10metaObjectEv()};
    return 1;
  }
}

