// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicsitem::QGraphicsItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsObject::yChanged();
  fn _ZN15QGraphicsObject8yChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::enabledChanged();
  fn _ZN15QGraphicsObject14enabledChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::widthChanged();
  fn _ZN15QGraphicsObject12widthChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::visibleChanged();
  fn _ZN15QGraphicsObject14visibleChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::childrenChanged();
  fn _ZN15QGraphicsObject15childrenChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::zChanged();
  fn _ZN15QGraphicsObject8zChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::opacityChanged();
  fn _ZN15QGraphicsObject14opacityChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::NewQGraphicsObject(QGraphicsItem * parent);
  fn _ZN15QGraphicsObjectC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsObject::xChanged();
  fn _ZN15QGraphicsObject8xChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::rotationChanged();
  fn _ZN15QGraphicsObject15rotationChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::FreeQGraphicsObject();
  fn _ZN15QGraphicsObjectD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::heightChanged();
  fn _ZN15QGraphicsObject13heightChangedEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QGraphicsObject::metaObject();
  fn _ZNK15QGraphicsObject10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::scaleChanged();
  fn _ZN15QGraphicsObject12scaleChangedEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsObject::parentChanged();
  fn _ZN15QGraphicsObject13parentChangedEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsObject)=1
pub struct QGraphicsObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsObject {
  pub fn yChanged<T: QGraphicsObject_yChanged>(&mut self, value: T)  {
     value.yChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_yChanged {
  fn yChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::yChanged();
impl<'a> /*trait*/ QGraphicsObject_yChanged for () {
  fn yChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject8yChangedEv()};
     unsafe {_ZN15QGraphicsObject8yChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn enabledChanged<T: QGraphicsObject_enabledChanged>(&mut self, value: T)  {
     value.enabledChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_enabledChanged {
  fn enabledChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::enabledChanged();
impl<'a> /*trait*/ QGraphicsObject_enabledChanged for () {
  fn enabledChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject14enabledChangedEv()};
     unsafe {_ZN15QGraphicsObject14enabledChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn widthChanged<T: QGraphicsObject_widthChanged>(&mut self, value: T)  {
     value.widthChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_widthChanged {
  fn widthChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::widthChanged();
impl<'a> /*trait*/ QGraphicsObject_widthChanged for () {
  fn widthChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject12widthChangedEv()};
     unsafe {_ZN15QGraphicsObject12widthChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn visibleChanged<T: QGraphicsObject_visibleChanged>(&mut self, value: T)  {
     value.visibleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_visibleChanged {
  fn visibleChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::visibleChanged();
impl<'a> /*trait*/ QGraphicsObject_visibleChanged for () {
  fn visibleChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject14visibleChangedEv()};
     unsafe {_ZN15QGraphicsObject14visibleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn childrenChanged<T: QGraphicsObject_childrenChanged>(&mut self, value: T)  {
     value.childrenChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_childrenChanged {
  fn childrenChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::childrenChanged();
impl<'a> /*trait*/ QGraphicsObject_childrenChanged for () {
  fn childrenChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject15childrenChangedEv()};
     unsafe {_ZN15QGraphicsObject15childrenChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn zChanged<T: QGraphicsObject_zChanged>(&mut self, value: T)  {
     value.zChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_zChanged {
  fn zChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::zChanged();
impl<'a> /*trait*/ QGraphicsObject_zChanged for () {
  fn zChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject8zChangedEv()};
     unsafe {_ZN15QGraphicsObject8zChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn opacityChanged<T: QGraphicsObject_opacityChanged>(&mut self, value: T)  {
     value.opacityChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_opacityChanged {
  fn opacityChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::opacityChanged();
impl<'a> /*trait*/ QGraphicsObject_opacityChanged for () {
  fn opacityChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject14opacityChangedEv()};
     unsafe {_ZN15QGraphicsObject14opacityChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn NewQGraphicsObject<T: QGraphicsObject_NewQGraphicsObject>(value: T) -> QGraphicsObject {
    let rsthis = value.NewQGraphicsObject();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsObject_NewQGraphicsObject {
  fn NewQGraphicsObject(self) -> QGraphicsObject;
}

// proto: void QGraphicsObject::NewQGraphicsObject(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsObject_NewQGraphicsObject for (&'a mut QGraphicsItem) {
  fn NewQGraphicsObject(self) -> QGraphicsObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObjectC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsObjectC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn xChanged<T: QGraphicsObject_xChanged>(&mut self, value: T)  {
     value.xChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_xChanged {
  fn xChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::xChanged();
impl<'a> /*trait*/ QGraphicsObject_xChanged for () {
  fn xChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject8xChangedEv()};
     unsafe {_ZN15QGraphicsObject8xChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn rotationChanged<T: QGraphicsObject_rotationChanged>(&mut self, value: T)  {
     value.rotationChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_rotationChanged {
  fn rotationChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::rotationChanged();
impl<'a> /*trait*/ QGraphicsObject_rotationChanged for () {
  fn rotationChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject15rotationChangedEv()};
     unsafe {_ZN15QGraphicsObject15rotationChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn FreeQGraphicsObject<T: QGraphicsObject_FreeQGraphicsObject>(&mut self, value: T)  {
     value.FreeQGraphicsObject(self);
    // return 1;
  }
}

pub trait QGraphicsObject_FreeQGraphicsObject {
  fn FreeQGraphicsObject(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::FreeQGraphicsObject();
impl<'a> /*trait*/ QGraphicsObject_FreeQGraphicsObject for () {
  fn FreeQGraphicsObject(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObjectD0Ev()};
     unsafe {_ZN15QGraphicsObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn heightChanged<T: QGraphicsObject_heightChanged>(&mut self, value: T)  {
     value.heightChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_heightChanged {
  fn heightChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::heightChanged();
impl<'a> /*trait*/ QGraphicsObject_heightChanged for () {
  fn heightChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject13heightChangedEv()};
     unsafe {_ZN15QGraphicsObject13heightChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn metaObject<T: QGraphicsObject_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsObject_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  const QMetaObject * QGraphicsObject::metaObject();
impl<'a> /*trait*/ QGraphicsObject_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsObject10metaObjectEv()};
     unsafe {_ZNK15QGraphicsObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn scaleChanged<T: QGraphicsObject_scaleChanged>(&mut self, value: T)  {
     value.scaleChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_scaleChanged {
  fn scaleChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::scaleChanged();
impl<'a> /*trait*/ QGraphicsObject_scaleChanged for () {
  fn scaleChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject12scaleChangedEv()};
     unsafe {_ZN15QGraphicsObject12scaleChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn parentChanged<T: QGraphicsObject_parentChanged>(&mut self, value: T)  {
     value.parentChanged(self);
    // return 1;
  }
}

pub trait QGraphicsObject_parentChanged {
  fn parentChanged(self, rsthis: &mut QGraphicsObject) ;
}

// proto:  void QGraphicsObject::parentChanged();
impl<'a> /*trait*/ QGraphicsObject_parentChanged for () {
  fn parentChanged(self, rsthis: &mut QGraphicsObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObject13parentChangedEv()};
     unsafe {_ZN15QGraphicsObject13parentChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

