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
  // proto:  void QGraphicsAnchor::FreeQGraphicsAnchor();
  fn _ZN15QGraphicsAnchorD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsAnchor::unsetSpacing();
  fn _ZN15QGraphicsAnchor12unsetSpacingEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
  fn _ZN15QGraphicsAnchor10setSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsAnchor::NewQGraphicsAnchor(QGraphicsAnchorLayout * parent);
  fn _ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
  fn _ZNK15QGraphicsAnchor10metaObjectEv(qthis: *mut c_void) ;
  // proto:  double QGraphicsAnchor::spacing();
  fn _ZNK15QGraphicsAnchor7spacingEv(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QGraphicsAnchor)=1
pub struct QGraphicsAnchor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsAnchor {
  pub fn FreeQGraphicsAnchor<T: QGraphicsAnchor_FreeQGraphicsAnchor>(&mut self, value: T)  {
     value.FreeQGraphicsAnchor(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_FreeQGraphicsAnchor {
  fn FreeQGraphicsAnchor(self, rsthis: &mut QGraphicsAnchor) ;
}

// proto:  void QGraphicsAnchor::FreeQGraphicsAnchor();
impl<'a> /*trait*/ QGraphicsAnchor_FreeQGraphicsAnchor for () {
  fn FreeQGraphicsAnchor(self, rsthis: &mut QGraphicsAnchor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorD0Ev()};
     unsafe {_ZN15QGraphicsAnchorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn unsetSpacing<T: QGraphicsAnchor_unsetSpacing>(&mut self, value: T)  {
     value.unsetSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_unsetSpacing {
  fn unsetSpacing(self, rsthis: &mut QGraphicsAnchor) ;
}

// proto:  void QGraphicsAnchor::unsetSpacing();
impl<'a> /*trait*/ QGraphicsAnchor_unsetSpacing for () {
  fn unsetSpacing(self, rsthis: &mut QGraphicsAnchor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor12unsetSpacingEv()};
     unsafe {_ZN15QGraphicsAnchor12unsetSpacingEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn setSpacing<T: QGraphicsAnchor_setSpacing>(&mut self, value: T)  {
     value.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_setSpacing {
  fn setSpacing(self, rsthis: &mut QGraphicsAnchor) ;
}

// proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchor_setSpacing for (f64) {
  fn setSpacing(self, rsthis: &mut QGraphicsAnchor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QGraphicsAnchor10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

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

// proto: void QGraphicsAnchor::NewQGraphicsAnchor(QGraphicsAnchorLayout * parent);
impl<'a> /*trait*/ QGraphicsAnchor_NewQGraphicsAnchor for (&'a mut QGraphicsAnchorLayout) {
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

impl /*struct*/ QGraphicsAnchor {
  pub fn metaObject<T: QGraphicsAnchor_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsAnchor) ;
}

// proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl<'a> /*trait*/ QGraphicsAnchor_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsAnchor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor10metaObjectEv()};
     unsafe {_ZNK15QGraphicsAnchor10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn spacing<T: QGraphicsAnchor_spacing>(&mut self, value: T) -> f64 {
    return value.spacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_spacing {
  fn spacing(self, rsthis: &mut QGraphicsAnchor) -> f64;
}

// proto:  double QGraphicsAnchor::spacing();
impl<'a> /*trait*/ QGraphicsAnchor_spacing for () {
  fn spacing(self, rsthis: &mut QGraphicsAnchor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor7spacingEv()};
    let mut ret = unsafe {_ZNK15QGraphicsAnchor7spacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

