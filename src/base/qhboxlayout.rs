// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QHBoxLayout::QHBoxLayout(QWidget * parent);
  fn _ZN11QHBoxLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QHBoxLayout::metaObject();
  fn _ZNK11QHBoxLayout10metaObjectEv(qthis: *mut c_void);
  // proto:  void QHBoxLayout::~QHBoxLayout();
  fn _ZN11QHBoxLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QHBoxLayout::QHBoxLayout(const QHBoxLayout & );
  fn _ZN11QHBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QHBoxLayout::QHBoxLayout();
  fn _ZN11QHBoxLayoutC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QHBoxLayout)=1
pub struct QHBoxLayout {
  pub qclsinst: *mut c_void,
}

  // proto:  void QHBoxLayout::QHBoxLayout(QWidget * parent);
impl /*struct*/ QHBoxLayout {
  pub fn NewQHBoxLayout<T: QHBoxLayout_NewQHBoxLayout>(value: T) -> QHBoxLayout {
    let rsthis = value.NewQHBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QHBoxLayout_NewQHBoxLayout {
  fn NewQHBoxLayout(self) -> QHBoxLayout;
}

  // proto:  void QHBoxLayout::QHBoxLayout(QWidget * parent);
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for (QWidget) {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QHBoxLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QHBoxLayout::metaObject();
impl /*struct*/ QHBoxLayout {
  pub fn metaObject<RetType, T: QHBoxLayout_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QHBoxLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QHBoxLayout) -> RetType;
}

  // proto:  const QMetaObject * QHBoxLayout::metaObject();
impl<'a> /*trait*/ QHBoxLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QHBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QHBoxLayout10metaObjectEv()};
     unsafe {_ZNK11QHBoxLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHBoxLayout::~QHBoxLayout();
impl /*struct*/ QHBoxLayout {
  pub fn FreeQHBoxLayout<RetType, T: QHBoxLayout_FreeQHBoxLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQHBoxLayout(self);
    // return 1;
  }
}

pub trait QHBoxLayout_FreeQHBoxLayout<RetType> {
  fn FreeQHBoxLayout(self , rsthis: &mut QHBoxLayout) -> RetType;
}

  // proto:  void QHBoxLayout::~QHBoxLayout();
impl<'a> /*trait*/ QHBoxLayout_FreeQHBoxLayout<()> for () {
  fn FreeQHBoxLayout(self , rsthis: &mut QHBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutD0Ev()};
     unsafe {_ZN11QHBoxLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QHBoxLayout::QHBoxLayout(const QHBoxLayout & );
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for (QHBoxLayout) {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QHBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHBoxLayout::QHBoxLayout();
impl<'a> /*trait*/ QHBoxLayout_NewQHBoxLayout for () {
  fn NewQHBoxLayout(self) -> QHBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QHBoxLayoutC1Ev()};
    unsafe {_ZN11QHBoxLayoutC1Ev(qthis)};
    let rsthis = QHBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

