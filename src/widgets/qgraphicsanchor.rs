// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsanchorlayout::QGraphicsAnchorLayout;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
  fn _ZN15QGraphicsAnchorD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsAnchor::unsetSpacing();
  fn _ZN15QGraphicsAnchor12unsetSpacingEv(qthis: *mut c_void);
  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
  fn _ZN15QGraphicsAnchor10setSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsAnchor::QGraphicsAnchor(QGraphicsAnchorLayout * parent);
  fn _ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
  fn _ZNK15QGraphicsAnchor10metaObjectEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsAnchor::spacing();
  fn _ZNK15QGraphicsAnchor7spacingEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QGraphicsAnchor)=1
pub struct QGraphicsAnchor {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
impl /*struct*/ QGraphicsAnchor {
  pub fn FreeQGraphicsAnchor<RetType, T: QGraphicsAnchor_FreeQGraphicsAnchor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsAnchor(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_FreeQGraphicsAnchor<RetType> {
  fn FreeQGraphicsAnchor(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
impl<'a> /*trait*/ QGraphicsAnchor_FreeQGraphicsAnchor<()> for () {
  fn FreeQGraphicsAnchor(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorD0Ev()};
     unsafe {_ZN15QGraphicsAnchorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::unsetSpacing();
impl /*struct*/ QGraphicsAnchor {
  pub fn unsetSpacing<RetType, T: QGraphicsAnchor_unsetSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unsetSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_unsetSpacing<RetType> {
  fn unsetSpacing(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::unsetSpacing();
impl<'a> /*trait*/ QGraphicsAnchor_unsetSpacing<()> for () {
  fn unsetSpacing(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor12unsetSpacingEv()};
     unsafe {_ZN15QGraphicsAnchor12unsetSpacingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchor {
  pub fn setSpacing<RetType, T: QGraphicsAnchor_setSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_setSpacing<RetType> {
  fn setSpacing(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchor_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QGraphicsAnchor10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::QGraphicsAnchor(QGraphicsAnchorLayout * parent);
impl /*struct*/ QGraphicsAnchor {
  pub fn NewQGraphicsAnchor<T: QGraphicsAnchor_NewQGraphicsAnchor>(value: T) -> QGraphicsAnchor {
    let rsthis = value.NewQGraphicsAnchor();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchor_NewQGraphicsAnchor {
  fn NewQGraphicsAnchor(self) -> QGraphicsAnchor;
}

  // proto:  void QGraphicsAnchor::QGraphicsAnchor(QGraphicsAnchorLayout * parent);
impl<'a> /*trait*/ QGraphicsAnchor_NewQGraphicsAnchor for (QGraphicsAnchorLayout) {
  fn NewQGraphicsAnchor(self) -> QGraphicsAnchor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis, arg0)};
    let rsthis = QGraphicsAnchor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl /*struct*/ QGraphicsAnchor {
  pub fn metaObject<RetType, T: QGraphicsAnchor_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl<'a> /*trait*/ QGraphicsAnchor_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor10metaObjectEv()};
     unsafe {_ZNK15QGraphicsAnchor10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchor::spacing();
impl /*struct*/ QGraphicsAnchor {
  pub fn spacing<RetType, T: QGraphicsAnchor_spacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_spacing<RetType> {
  fn spacing(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  qreal QGraphicsAnchor::spacing();
impl<'a> /*trait*/ QGraphicsAnchor_spacing<f64> for () {
  fn spacing(self , rsthis: &mut QGraphicsAnchor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor7spacingEv()};
    let mut ret = unsafe {_ZNK15QGraphicsAnchor7spacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

