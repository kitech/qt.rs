// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsizepolicy::QSizePolicy;
use super::qsizef::QSizeF;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
  fn _ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
  fn _ZNK19QGraphicsLayoutItem16parentLayoutItemEv(qthis: *mut c_void) ;
  // proto:  double QGraphicsLayoutItem::minimumWidth();
  fn _ZNK19QGraphicsLayoutItem12minimumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
  fn _ZNK19QGraphicsLayoutItem12graphicsItemEv(qthis: *mut c_void) ;
  // proto:  double QGraphicsLayoutItem::preferredWidth();
  fn _ZNK19QGraphicsLayoutItem14preferredWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
  fn _ZNK19QGraphicsLayoutItem13ownedByLayoutEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
  fn _ZNK19QGraphicsLayoutItem13preferredSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsLayoutItem::geometry();
  fn _ZNK19QGraphicsLayoutItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsLayoutItem::NewQGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
  fn _ZN19QGraphicsLayoutItemC1EPS_b(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  double QGraphicsLayoutItem::minimumHeight();
  fn _ZNK19QGraphicsLayoutItem13minimumHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  double QGraphicsLayoutItem::preferredHeight();
  fn _ZNK19QGraphicsLayoutItem15preferredHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
  fn _ZNK19QGraphicsLayoutItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
  fn _ZNK19QGraphicsLayoutItem10sizePolicyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QGraphicsLayoutItem::maximumHeight();
  fn _ZNK19QGraphicsLayoutItem13maximumHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem17setPreferredWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsLayoutItem::maximumWidth();
  fn _ZNK19QGraphicsLayoutItem12maximumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMaximumHeightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
  fn _ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMinimumWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMaximumWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsLayoutItem::updateGeometry();
  fn _ZN19QGraphicsLayoutItem14updateGeometryEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem18setPreferredHeightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
  fn _ZNK19QGraphicsLayoutItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
  fn _ZNK19QGraphicsLayoutItem12contentsRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsLayoutItem::isLayout();
  fn _ZNK19QGraphicsLayoutItem8isLayoutEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QGraphicsLayoutItem::FreeQGraphicsLayoutItem();
  fn _ZN19QGraphicsLayoutItemD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMinimumHeightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
}

// body block begin
// class sizeof(QGraphicsLayoutItem)=1
pub struct QGraphicsLayoutItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setSizePolicy<T: QGraphicsLayoutItem_setSizePolicy>(&mut self, value: T)  {
     value.setSizePolicy(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setSizePolicy {
  fn setSizePolicy(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
impl<'a> /*trait*/ QGraphicsLayoutItem_setSizePolicy for (&'a  QSizePolicy) {
  fn setSizePolicy(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn parentLayoutItem<T: QGraphicsLayoutItem_parentLayoutItem>(&mut self, value: T)  {
     value.parentLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_parentLayoutItem {
  fn parentLayoutItem(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_parentLayoutItem for () {
  fn parentLayoutItem(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem16parentLayoutItemEv()};
     unsafe {_ZNK19QGraphicsLayoutItem16parentLayoutItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumWidth<T: QGraphicsLayoutItem_minimumWidth>(&mut self, value: T) -> f64 {
    return value.minimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumWidth {
  fn minimumWidth(self, rsthis: &mut QGraphicsLayoutItem) -> f64;
}

// proto:  double QGraphicsLayoutItem::minimumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumWidth for () {
  fn minimumWidth(self, rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12minimumWidthEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12minimumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn graphicsItem<T: QGraphicsLayoutItem_graphicsItem>(&mut self, value: T)  {
     value.graphicsItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_graphicsItem {
  fn graphicsItem(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_graphicsItem for () {
  fn graphicsItem(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12graphicsItemEv()};
     unsafe {_ZNK19QGraphicsLayoutItem12graphicsItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredWidth<T: QGraphicsLayoutItem_preferredWidth>(&mut self, value: T) -> f64 {
    return value.preferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredWidth {
  fn preferredWidth(self, rsthis: &mut QGraphicsLayoutItem) -> f64;
}

// proto:  double QGraphicsLayoutItem::preferredWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredWidth for () {
  fn preferredWidth(self, rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem14preferredWidthEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem14preferredWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn ownedByLayout<T: QGraphicsLayoutItem_ownedByLayout>(&mut self, value: T) -> i8 {
    return value.ownedByLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_ownedByLayout {
  fn ownedByLayout(self, rsthis: &mut QGraphicsLayoutItem) -> i8;
}

// proto:  bool QGraphicsLayoutItem::ownedByLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_ownedByLayout for () {
  fn ownedByLayout(self, rsthis: &mut QGraphicsLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13ownedByLayoutEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13ownedByLayoutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredSize<T: QGraphicsLayoutItem_preferredSize>(&mut self, value: T) -> QSizeF {
    return value.preferredSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredSize {
  fn preferredSize(self, rsthis: &mut QGraphicsLayoutItem) -> QSizeF;
}

// proto:  QSizeF QGraphicsLayoutItem::preferredSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredSize for () {
  fn preferredSize(self, rsthis: &mut QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13preferredSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13preferredSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn geometry<T: QGraphicsLayoutItem_geometry>(&mut self, value: T) -> QRectF {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_geometry {
  fn geometry(self, rsthis: &mut QGraphicsLayoutItem) -> QRectF;
}

// proto:  QRectF QGraphicsLayoutItem::geometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_geometry for () {
  fn geometry(self, rsthis: &mut QGraphicsLayoutItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn NewQGraphicsLayoutItem<T: QGraphicsLayoutItem_NewQGraphicsLayoutItem>(value: T) -> QGraphicsLayoutItem {
    let rsthis = value.NewQGraphicsLayoutItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_NewQGraphicsLayoutItem {
  fn NewQGraphicsLayoutItem(self) -> QGraphicsLayoutItem;
}

// proto: void QGraphicsLayoutItem::NewQGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
impl<'a> /*trait*/ QGraphicsLayoutItem_NewQGraphicsLayoutItem for (&'a mut QGraphicsLayoutItem, i8) {
  fn NewQGraphicsLayoutItem(self) -> QGraphicsLayoutItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemC1EPS_b()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN19QGraphicsLayoutItemC1EPS_b(qthis, arg0, arg1)};
    let rsthis = QGraphicsLayoutItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumHeight<T: QGraphicsLayoutItem_minimumHeight>(&mut self, value: T) -> f64 {
    return value.minimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumHeight {
  fn minimumHeight(self, rsthis: &mut QGraphicsLayoutItem) -> f64;
}

// proto:  double QGraphicsLayoutItem::minimumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumHeight for () {
  fn minimumHeight(self, rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13minimumHeightEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13minimumHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredHeight<T: QGraphicsLayoutItem_preferredHeight>(&mut self, value: T) -> f64 {
    return value.preferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredHeight {
  fn preferredHeight(self, rsthis: &mut QGraphicsLayoutItem) -> f64;
}

// proto:  double QGraphicsLayoutItem::preferredHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredHeight for () {
  fn preferredHeight(self, rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem15preferredHeightEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem15preferredHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumSize<T: QGraphicsLayoutItem_maximumSize>(&mut self, value: T) -> QSizeF {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumSize {
  fn maximumSize(self, rsthis: &mut QGraphicsLayoutItem) -> QSizeF;
}

// proto:  QSizeF QGraphicsLayoutItem::maximumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumSize for () {
  fn maximumSize(self, rsthis: &mut QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn sizePolicy<T: QGraphicsLayoutItem_sizePolicy>(&mut self, value: T) -> QSizePolicy {
    return value.sizePolicy(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_sizePolicy {
  fn sizePolicy(self, rsthis: &mut QGraphicsLayoutItem) -> QSizePolicy;
}

// proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
impl<'a> /*trait*/ QGraphicsLayoutItem_sizePolicy for () {
  fn sizePolicy(self, rsthis: &mut QGraphicsLayoutItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumHeight<T: QGraphicsLayoutItem_maximumHeight>(&mut self, value: T) -> f64 {
    return value.maximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumHeight {
  fn maximumHeight(self, rsthis: &mut QGraphicsLayoutItem) -> f64;
}

// proto:  double QGraphicsLayoutItem::maximumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumHeight for () {
  fn maximumHeight(self, rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13maximumHeightEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13maximumHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setGeometry<T: QGraphicsLayoutItem_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setGeometry {
  fn setGeometry(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLayoutItem_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredWidth<T: QGraphicsLayoutItem_setPreferredWidth>(&mut self, value: T)  {
     value.setPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredWidth {
  fn setPreferredWidth(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredWidth for (f64) {
  fn setPreferredWidth(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem17setPreferredWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem17setPreferredWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumSize<T: QGraphicsLayoutItem_setMaximumSize>(&mut self, value: T)  {
     value.setMaximumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumSize {
  fn setMaximumSize(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize for (&'a  QSizeF) {
  fn setMaximumSize(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumWidth<T: QGraphicsLayoutItem_maximumWidth>(&mut self, value: T) -> f64 {
    return value.maximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumWidth {
  fn maximumWidth(self, rsthis: &mut QGraphicsLayoutItem) -> f64;
}

// proto:  double QGraphicsLayoutItem::maximumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumWidth for () {
  fn maximumWidth(self, rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12maximumWidthEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12maximumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumSize<T: QGraphicsLayoutItem_setMinimumSize>(&mut self, value: T)  {
     value.setMinimumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumSize {
  fn setMinimumSize(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize for (f64, f64) {
  fn setMinimumSize(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumHeight<T: QGraphicsLayoutItem_setMaximumHeight>(&mut self, value: T)  {
     value.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumHeight {
  fn setMaximumHeight(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumHeight for (f64) {
  fn setMaximumHeight(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMaximumHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setMaximumHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize for (&'a  QSizeF) {
  fn setMinimumSize(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredSize<T: QGraphicsLayoutItem_setPreferredSize>(&mut self, value: T)  {
     value.setPreferredSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredSize {
  fn setPreferredSize(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize for (&'a  QSizeF) {
  fn setPreferredSize(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn getContentsMargins<T: QGraphicsLayoutItem_getContentsMargins>(&mut self, value: T)  {
     value.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_getContentsMargins {
  fn getContentsMargins(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayoutItem_getContentsMargins for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getContentsMargins(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setParentLayoutItem<T: QGraphicsLayoutItem_setParentLayoutItem>(&mut self, value: T)  {
     value.setParentLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setParentLayoutItem {
  fn setParentLayoutItem(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayoutItem_setParentLayoutItem for (&'a mut QGraphicsLayoutItem) {
  fn setParentLayoutItem(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumWidth<T: QGraphicsLayoutItem_setMinimumWidth>(&mut self, value: T)  {
     value.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumWidth {
  fn setMinimumWidth(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumWidth for (f64) {
  fn setMinimumWidth(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMinimumWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem15setMinimumWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumWidth<T: QGraphicsLayoutItem_setMaximumWidth>(&mut self, value: T)  {
     value.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumWidth {
  fn setMaximumWidth(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumWidth for (f64) {
  fn setMaximumWidth(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMaximumWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem15setMaximumWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn updateGeometry<T: QGraphicsLayoutItem_updateGeometry>(&mut self, value: T)  {
     value.updateGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_updateGeometry {
  fn updateGeometry(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_updateGeometry for () {
  fn updateGeometry(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14updateGeometryEv()};
     unsafe {_ZN19QGraphicsLayoutItem14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredHeight<T: QGraphicsLayoutItem_setPreferredHeight>(&mut self, value: T)  {
     value.setPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredHeight {
  fn setPreferredHeight(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredHeight for (f64) {
  fn setPreferredHeight(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem18setPreferredHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem18setPreferredHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumSize<T: QGraphicsLayoutItem_minimumSize>(&mut self, value: T) -> QSizeF {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumSize {
  fn minimumSize(self, rsthis: &mut QGraphicsLayoutItem) -> QSizeF;
}

// proto:  QSizeF QGraphicsLayoutItem::minimumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn contentsRect<T: QGraphicsLayoutItem_contentsRect>(&mut self, value: T) -> QRectF {
    return value.contentsRect(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_contentsRect {
  fn contentsRect(self, rsthis: &mut QGraphicsLayoutItem) -> QRectF;
}

// proto:  QRectF QGraphicsLayoutItem::contentsRect();
impl<'a> /*trait*/ QGraphicsLayoutItem_contentsRect for () {
  fn contentsRect(self, rsthis: &mut QGraphicsLayoutItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12contentsRectEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12contentsRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn isLayout<T: QGraphicsLayoutItem_isLayout>(&mut self, value: T) -> i8 {
    return value.isLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_isLayout {
  fn isLayout(self, rsthis: &mut QGraphicsLayoutItem) -> i8;
}

// proto:  bool QGraphicsLayoutItem::isLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_isLayout for () {
  fn isLayout(self, rsthis: &mut QGraphicsLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8isLayoutEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem8isLayoutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize for (f64, f64) {
  fn setPreferredSize(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn FreeQGraphicsLayoutItem<T: QGraphicsLayoutItem_FreeQGraphicsLayoutItem>(&mut self, value: T)  {
     value.FreeQGraphicsLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_FreeQGraphicsLayoutItem {
  fn FreeQGraphicsLayoutItem(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::FreeQGraphicsLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_FreeQGraphicsLayoutItem for () {
  fn FreeQGraphicsLayoutItem(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemD0Ev()};
     unsafe {_ZN19QGraphicsLayoutItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumHeight<T: QGraphicsLayoutItem_setMinimumHeight>(&mut self, value: T)  {
     value.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumHeight {
  fn setMinimumHeight(self, rsthis: &mut QGraphicsLayoutItem) ;
}

// proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumHeight for (f64) {
  fn setMinimumHeight(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMinimumHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setMinimumHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize for (f64, f64) {
  fn setMaximumSize(self, rsthis: &mut QGraphicsLayoutItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

