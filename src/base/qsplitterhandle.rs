// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;
use super::qsplitter::QSplitter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSplitterHandle::FreeQSplitterHandle();
  fn _ZN15QSplitterHandleD0Ev(qthis: *mut c_void) ;
  // proto:  void QSplitterHandle::NewQSplitterHandle(const QSplitterHandle & );
  fn _ZN15QSplitterHandleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QSplitterHandle::sizeHint();
  fn _ZNK15QSplitterHandle8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSplitterHandle::opaqueResize();
  fn _ZNK15QSplitterHandle12opaqueResizeEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSplitter * QSplitterHandle::splitter();
  fn _ZNK15QSplitterHandle8splitterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSplitterHandle::metaObject();
  fn _ZNK15QSplitterHandle10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QSplitterHandle)=1
pub struct QSplitterHandle {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSplitterHandle {
  pub fn FreeQSplitterHandle<T: QSplitterHandle_FreeQSplitterHandle>(&mut self, value: T)  {
     value.FreeQSplitterHandle(self);
    // return 1;
  }
}

pub trait QSplitterHandle_FreeQSplitterHandle {
  fn FreeQSplitterHandle(self, rsthis: &mut QSplitterHandle) ;
}

// proto:  void QSplitterHandle::FreeQSplitterHandle();
impl<'a> /*trait*/ QSplitterHandle_FreeQSplitterHandle for () {
  fn FreeQSplitterHandle(self, rsthis: &mut QSplitterHandle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSplitterHandleD0Ev()};
     unsafe {_ZN15QSplitterHandleD0Ev(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSplitterHandleC1ERKS_(qthis, arg0)};
    let rsthis = QSplitterHandle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn sizeHint<T: QSplitterHandle_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QSplitterHandle_sizeHint {
  fn sizeHint(self, rsthis: &mut QSplitterHandle) -> QSize;
}

// proto:  QSize QSplitterHandle::sizeHint();
impl<'a> /*trait*/ QSplitterHandle_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QSplitterHandle) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QSplitterHandle8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn opaqueResize<T: QSplitterHandle_opaqueResize>(&mut self, value: T) -> i8 {
    return value.opaqueResize(self);
    // return 1;
  }
}

pub trait QSplitterHandle_opaqueResize {
  fn opaqueResize(self, rsthis: &mut QSplitterHandle) -> i8;
}

// proto:  bool QSplitterHandle::opaqueResize();
impl<'a> /*trait*/ QSplitterHandle_opaqueResize for () {
  fn opaqueResize(self, rsthis: &mut QSplitterHandle) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle12opaqueResizeEv()};
    let mut ret = unsafe {_ZNK15QSplitterHandle12opaqueResizeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn splitter<T: QSplitterHandle_splitter>(&mut self, value: T) -> QSplitter {
    return value.splitter(self);
    // return 1;
  }
}

pub trait QSplitterHandle_splitter {
  fn splitter(self, rsthis: &mut QSplitterHandle) -> QSplitter;
}

// proto:  QSplitter * QSplitterHandle::splitter();
impl<'a> /*trait*/ QSplitterHandle_splitter for () {
  fn splitter(self, rsthis: &mut QSplitterHandle) -> QSplitter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8splitterEv()};
    let mut ret = unsafe {_ZNK15QSplitterHandle8splitterEv(rsthis.qclsinst)};
    let mut ret1 = QSplitter{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSplitterHandle {
  pub fn metaObject<T: QSplitterHandle_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSplitterHandle_metaObject {
  fn metaObject(self, rsthis: &mut QSplitterHandle) ;
}

// proto:  const QMetaObject * QSplitterHandle::metaObject();
impl<'a> /*trait*/ QSplitterHandle_metaObject for () {
  fn metaObject(self, rsthis: &mut QSplitterHandle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle10metaObjectEv()};
     unsafe {_ZNK15QSplitterHandle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

