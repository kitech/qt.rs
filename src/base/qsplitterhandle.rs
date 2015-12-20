// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsplitter::QSplitter;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSplitterHandle::~QSplitterHandle();
  fn _ZN15QSplitterHandleD0Ev(qthis: *mut c_void);
  // proto:  void QSplitterHandle::QSplitterHandle(const QSplitterHandle & );
  fn _ZN15QSplitterHandleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QSplitterHandle::sizeHint();
  fn _ZNK15QSplitterHandle8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSplitterHandle::opaqueResize();
  fn _ZNK15QSplitterHandle12opaqueResizeEv(qthis: *mut c_void) -> c_char;
  // proto:  QSplitter * QSplitterHandle::splitter();
  fn _ZNK15QSplitterHandle8splitterEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QSplitterHandle::metaObject();
  fn _ZNK15QSplitterHandle10metaObjectEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QSplitterHandle)=1
pub struct QSplitterHandle {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSplitterHandle::~QSplitterHandle();
impl /*struct*/ QSplitterHandle {
  pub fn FreeQSplitterHandle<RetType, T: QSplitterHandle_FreeQSplitterHandle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSplitterHandle(self);
    // return 1;
  }
}

pub trait QSplitterHandle_FreeQSplitterHandle<RetType> {
  fn FreeQSplitterHandle(self , rsthis: &mut QSplitterHandle) -> RetType;
}

  // proto:  void QSplitterHandle::~QSplitterHandle();
impl<'a> /*trait*/ QSplitterHandle_FreeQSplitterHandle<()> for () {
  fn FreeQSplitterHandle(self , rsthis: &mut QSplitterHandle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSplitterHandleD0Ev()};
     unsafe {_ZN15QSplitterHandleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSplitterHandle::QSplitterHandle(const QSplitterHandle & );
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

  // proto:  void QSplitterHandle::QSplitterHandle(const QSplitterHandle & );
impl<'a> /*trait*/ QSplitterHandle_NewQSplitterHandle for (QSplitterHandle) {
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

  // proto:  QSize QSplitterHandle::sizeHint();
impl /*struct*/ QSplitterHandle {
  pub fn sizeHint<RetType, T: QSplitterHandle_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSplitterHandle_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QSplitterHandle) -> RetType;
}

  // proto:  QSize QSplitterHandle::sizeHint();
impl<'a> /*trait*/ QSplitterHandle_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QSplitterHandle) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8sizeHintEv()};
    let mut ret = unsafe {_ZNK15QSplitterHandle8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSplitterHandle::opaqueResize();
impl /*struct*/ QSplitterHandle {
  pub fn opaqueResize<RetType, T: QSplitterHandle_opaqueResize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opaqueResize(self);
    // return 1;
  }
}

pub trait QSplitterHandle_opaqueResize<RetType> {
  fn opaqueResize(self , rsthis: &mut QSplitterHandle) -> RetType;
}

  // proto:  bool QSplitterHandle::opaqueResize();
impl<'a> /*trait*/ QSplitterHandle_opaqueResize<i8> for () {
  fn opaqueResize(self , rsthis: &mut QSplitterHandle) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle12opaqueResizeEv()};
    let mut ret = unsafe {_ZNK15QSplitterHandle12opaqueResizeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSplitter * QSplitterHandle::splitter();
impl /*struct*/ QSplitterHandle {
  pub fn splitter<RetType, T: QSplitterHandle_splitter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.splitter(self);
    // return 1;
  }
}

pub trait QSplitterHandle_splitter<RetType> {
  fn splitter(self , rsthis: &mut QSplitterHandle) -> RetType;
}

  // proto:  QSplitter * QSplitterHandle::splitter();
impl<'a> /*trait*/ QSplitterHandle_splitter<QSplitter> for () {
  fn splitter(self , rsthis: &mut QSplitterHandle) -> QSplitter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle8splitterEv()};
    let mut ret = unsafe {_ZNK15QSplitterHandle8splitterEv(rsthis.qclsinst)};
    let mut ret1 = QSplitter{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSplitterHandle::metaObject();
impl /*struct*/ QSplitterHandle {
  pub fn metaObject<RetType, T: QSplitterHandle_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSplitterHandle_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSplitterHandle) -> RetType;
}

  // proto:  const QMetaObject * QSplitterHandle::metaObject();
impl<'a> /*trait*/ QSplitterHandle_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSplitterHandle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSplitterHandle10metaObjectEv()};
     unsafe {_ZNK15QSplitterHandle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

