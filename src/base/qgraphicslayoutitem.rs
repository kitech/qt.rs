// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsizepolicy::QSizePolicy;
use super::qrectf::QRectF;
use super::qsizef::QSizeF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
  fn _ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(arg0: *const c_void) -> i32;
  // proto: QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
  fn _ZNK19QGraphicsLayoutItem16parentLayoutItemEv() -> i32;
  // proto: double QGraphicsLayoutItem::minimumWidth();
  fn _ZNK19QGraphicsLayoutItem12minimumWidthEv() -> i32;
  // proto: QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
  fn _ZNK19QGraphicsLayoutItem12graphicsItemEv() -> i32;
  // proto: double QGraphicsLayoutItem::preferredWidth();
  fn _ZNK19QGraphicsLayoutItem14preferredWidthEv() -> i32;
  // proto: bool QGraphicsLayoutItem::ownedByLayout();
  fn _ZNK19QGraphicsLayoutItem13ownedByLayoutEv() -> i32;
  // proto: QSizeF QGraphicsLayoutItem::preferredSize();
  fn _ZNK19QGraphicsLayoutItem13preferredSizeEv() -> i32;
  // proto: QRectF QGraphicsLayoutItem::geometry();
  fn _ZNK19QGraphicsLayoutItem8geometryEv() -> i32;
  // proto: void QGraphicsLayoutItem::NewQGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
  fn _ZN19QGraphicsLayoutItemC1EPS_b(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) -> i32;
  // proto: double QGraphicsLayoutItem::minimumHeight();
  fn _ZNK19QGraphicsLayoutItem13minimumHeightEv() -> i32;
  // proto: double QGraphicsLayoutItem::preferredHeight();
  fn _ZNK19QGraphicsLayoutItem15preferredHeightEv() -> i32;
  // proto: QSizeF QGraphicsLayoutItem::maximumSize();
  fn _ZNK19QGraphicsLayoutItem11maximumSizeEv() -> i32;
  // proto: QSizePolicy QGraphicsLayoutItem::sizePolicy();
  fn _ZNK19QGraphicsLayoutItem10sizePolicyEv() -> i32;
  // proto: double QGraphicsLayoutItem::maximumHeight();
  fn _ZNK19QGraphicsLayoutItem13maximumHeightEv() -> i32;
  // proto: void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLayoutItem::setPreferredWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem17setPreferredWidthEd(arg0: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: double QGraphicsLayoutItem::maximumWidth();
  fn _ZNK19QGraphicsLayoutItem12maximumWidthEv() -> i32;
  // proto: void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::setMaximumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMaximumHeightEd(arg0: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  // proto: void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
  fn _ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsLayoutItem::setMinimumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMinimumWidthEd(arg0: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::setMaximumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMaximumWidthEd(arg0: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::updateGeometry();
  fn _ZN19QGraphicsLayoutItem14updateGeometryEv() -> i32;
  // proto: void QGraphicsLayoutItem::setPreferredHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem18setPreferredHeightEd(arg0: c_double) -> i32;
  // proto: QSizeF QGraphicsLayoutItem::minimumSize();
  fn _ZNK19QGraphicsLayoutItem11minimumSizeEv() -> i32;
  // proto: QRectF QGraphicsLayoutItem::contentsRect();
  fn _ZNK19QGraphicsLayoutItem12contentsRectEv() -> i32;
  // proto: bool QGraphicsLayoutItem::isLayout();
  fn _ZNK19QGraphicsLayoutItem8isLayoutEv() -> i32;
  // proto: void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::FreeQGraphicsLayoutItem();
  fn _ZN19QGraphicsLayoutItemD0Ev() -> i32;
  // proto: void QGraphicsLayoutItem::setMinimumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMinimumHeightEd(arg0: c_double) -> i32;
  // proto: void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeEdd(arg0: c_double, arg1: c_double) -> i32;
}

// body block begin
// class sizeof(QGraphicsLayoutItem)=1
pub struct QGraphicsLayoutItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setSizePolicy<T: QGraphicsLayoutItem_setSizePolicy>(&mut self, value: T) -> i32 {
    value.setSizePolicy(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setSizePolicy {
  fn setSizePolicy(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
impl<'a> /*trait*/ QGraphicsLayoutItem_setSizePolicy for (&'a  QSizePolicy) {
  fn setSizePolicy(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn parentLayoutItem<T: QGraphicsLayoutItem_parentLayoutItem>(&mut self, value: T) -> i32 {
    value.parentLayoutItem(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_parentLayoutItem {
  fn parentLayoutItem(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_parentLayoutItem for () {
  fn parentLayoutItem(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem16parentLayoutItemEv()};
    unsafe {_ZNK19QGraphicsLayoutItem16parentLayoutItemEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumWidth<T: QGraphicsLayoutItem_minimumWidth>(&mut self, value: T) -> i32 {
    value.minimumWidth(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumWidth {
  fn minimumWidth(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: double QGraphicsLayoutItem::minimumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumWidth for () {
  fn minimumWidth(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12minimumWidthEv()};
    unsafe {_ZNK19QGraphicsLayoutItem12minimumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn graphicsItem<T: QGraphicsLayoutItem_graphicsItem>(&mut self, value: T) -> i32 {
    value.graphicsItem(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_graphicsItem {
  fn graphicsItem(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_graphicsItem for () {
  fn graphicsItem(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12graphicsItemEv()};
    unsafe {_ZNK19QGraphicsLayoutItem12graphicsItemEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredWidth<T: QGraphicsLayoutItem_preferredWidth>(&mut self, value: T) -> i32 {
    value.preferredWidth(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredWidth {
  fn preferredWidth(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: double QGraphicsLayoutItem::preferredWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredWidth for () {
  fn preferredWidth(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem14preferredWidthEv()};
    unsafe {_ZNK19QGraphicsLayoutItem14preferredWidthEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn ownedByLayout<T: QGraphicsLayoutItem_ownedByLayout>(&mut self, value: T) -> i32 {
    value.ownedByLayout(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_ownedByLayout {
  fn ownedByLayout(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: bool QGraphicsLayoutItem::ownedByLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_ownedByLayout for () {
  fn ownedByLayout(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13ownedByLayoutEv()};
    unsafe {_ZNK19QGraphicsLayoutItem13ownedByLayoutEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredSize<T: QGraphicsLayoutItem_preferredSize>(&mut self, value: T) -> i32 {
    value.preferredSize(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredSize {
  fn preferredSize(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QSizeF QGraphicsLayoutItem::preferredSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredSize for () {
  fn preferredSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13preferredSizeEv()};
    unsafe {_ZNK19QGraphicsLayoutItem13preferredSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn geometry<T: QGraphicsLayoutItem_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_geometry {
  fn geometry(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QRectF QGraphicsLayoutItem::geometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_geometry for () {
  fn geometry(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8geometryEv()};
    unsafe {_ZNK19QGraphicsLayoutItem8geometryEv()};
    return 1;
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
  pub fn minimumHeight<T: QGraphicsLayoutItem_minimumHeight>(&mut self, value: T) -> i32 {
    value.minimumHeight(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumHeight {
  fn minimumHeight(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: double QGraphicsLayoutItem::minimumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumHeight for () {
  fn minimumHeight(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13minimumHeightEv()};
    unsafe {_ZNK19QGraphicsLayoutItem13minimumHeightEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredHeight<T: QGraphicsLayoutItem_preferredHeight>(&mut self, value: T) -> i32 {
    value.preferredHeight(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredHeight {
  fn preferredHeight(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: double QGraphicsLayoutItem::preferredHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredHeight for () {
  fn preferredHeight(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem15preferredHeightEv()};
    unsafe {_ZNK19QGraphicsLayoutItem15preferredHeightEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumSize<T: QGraphicsLayoutItem_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumSize {
  fn maximumSize(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QSizeF QGraphicsLayoutItem::maximumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumSize for () {
  fn maximumSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11maximumSizeEv()};
    unsafe {_ZNK19QGraphicsLayoutItem11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn sizePolicy<T: QGraphicsLayoutItem_sizePolicy>(&mut self, value: T) -> i32 {
    value.sizePolicy(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_sizePolicy {
  fn sizePolicy(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QSizePolicy QGraphicsLayoutItem::sizePolicy();
impl<'a> /*trait*/ QGraphicsLayoutItem_sizePolicy for () {
  fn sizePolicy(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem10sizePolicyEv()};
    unsafe {_ZNK19QGraphicsLayoutItem10sizePolicyEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumHeight<T: QGraphicsLayoutItem_maximumHeight>(&mut self, value: T) -> i32 {
    value.maximumHeight(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumHeight {
  fn maximumHeight(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: double QGraphicsLayoutItem::maximumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumHeight for () {
  fn maximumHeight(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13maximumHeightEv()};
    unsafe {_ZNK19QGraphicsLayoutItem13maximumHeightEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setGeometry<T: QGraphicsLayoutItem_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setGeometry {
  fn setGeometry(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLayoutItem_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredWidth<T: QGraphicsLayoutItem_setPreferredWidth>(&mut self, value: T) -> i32 {
    value.setPreferredWidth(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredWidth {
  fn setPreferredWidth(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setPreferredWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredWidth for (f64) {
  fn setPreferredWidth(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem17setPreferredWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem17setPreferredWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumSize<T: QGraphicsLayoutItem_setMaximumSize>(&mut self, value: T) -> i32 {
    value.setMaximumSize(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumSize {
  fn setMaximumSize(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize for (&'a  QSizeF) {
  fn setMaximumSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumWidth<T: QGraphicsLayoutItem_maximumWidth>(&mut self, value: T) -> i32 {
    value.maximumWidth(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumWidth {
  fn maximumWidth(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: double QGraphicsLayoutItem::maximumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumWidth for () {
  fn maximumWidth(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12maximumWidthEv()};
    unsafe {_ZNK19QGraphicsLayoutItem12maximumWidthEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumSize<T: QGraphicsLayoutItem_setMinimumSize>(&mut self, value: T) -> i32 {
    value.setMinimumSize(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumSize {
  fn setMinimumSize(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize for (f64, f64) {
  fn setMinimumSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumHeight<T: QGraphicsLayoutItem_setMaximumHeight>(&mut self, value: T) -> i32 {
    value.setMaximumHeight(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumHeight {
  fn setMaximumHeight(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setMaximumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumHeight for (f64) {
  fn setMaximumHeight(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMaximumHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem16setMaximumHeightEd(arg0)};
    return 1;
  }
}

// proto: void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize for (&'a  QSizeF) {
  fn setMinimumSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredSize<T: QGraphicsLayoutItem_setPreferredSize>(&mut self, value: T) -> i32 {
    value.setPreferredSize(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredSize {
  fn setPreferredSize(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize for (&'a  QSizeF) {
  fn setPreferredSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn getContentsMargins<T: QGraphicsLayoutItem_getContentsMargins>(&mut self, value: T) -> i32 {
    value.getContentsMargins(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_getContentsMargins {
  fn getContentsMargins(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayoutItem_getContentsMargins for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getContentsMargins(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setParentLayoutItem<T: QGraphicsLayoutItem_setParentLayoutItem>(&mut self, value: T) -> i32 {
    value.setParentLayoutItem(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setParentLayoutItem {
  fn setParentLayoutItem(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayoutItem_setParentLayoutItem for (&'a mut QGraphicsLayoutItem) {
  fn setParentLayoutItem(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumWidth<T: QGraphicsLayoutItem_setMinimumWidth>(&mut self, value: T) -> i32 {
    value.setMinimumWidth(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumWidth {
  fn setMinimumWidth(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setMinimumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumWidth for (f64) {
  fn setMinimumWidth(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMinimumWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem15setMinimumWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumWidth<T: QGraphicsLayoutItem_setMaximumWidth>(&mut self, value: T) -> i32 {
    value.setMaximumWidth(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumWidth {
  fn setMaximumWidth(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setMaximumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumWidth for (f64) {
  fn setMaximumWidth(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMaximumWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem15setMaximumWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn updateGeometry<T: QGraphicsLayoutItem_updateGeometry>(&mut self, value: T) -> i32 {
    value.updateGeometry(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_updateGeometry {
  fn updateGeometry(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_updateGeometry for () {
  fn updateGeometry(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14updateGeometryEv()};
    unsafe {_ZN19QGraphicsLayoutItem14updateGeometryEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredHeight<T: QGraphicsLayoutItem_setPreferredHeight>(&mut self, value: T) -> i32 {
    value.setPreferredHeight(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredHeight {
  fn setPreferredHeight(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setPreferredHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredHeight for (f64) {
  fn setPreferredHeight(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem18setPreferredHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem18setPreferredHeightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumSize<T: QGraphicsLayoutItem_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumSize {
  fn minimumSize(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QSizeF QGraphicsLayoutItem::minimumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumSize for () {
  fn minimumSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11minimumSizeEv()};
    unsafe {_ZNK19QGraphicsLayoutItem11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn contentsRect<T: QGraphicsLayoutItem_contentsRect>(&mut self, value: T) -> i32 {
    value.contentsRect(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_contentsRect {
  fn contentsRect(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: QRectF QGraphicsLayoutItem::contentsRect();
impl<'a> /*trait*/ QGraphicsLayoutItem_contentsRect for () {
  fn contentsRect(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12contentsRectEv()};
    unsafe {_ZNK19QGraphicsLayoutItem12contentsRectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn isLayout<T: QGraphicsLayoutItem_isLayout>(&mut self, value: T) -> i32 {
    value.isLayout(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_isLayout {
  fn isLayout(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: bool QGraphicsLayoutItem::isLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_isLayout for () {
  fn isLayout(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8isLayoutEv()};
    unsafe {_ZNK19QGraphicsLayoutItem8isLayoutEv()};
    return 1;
  }
}

// proto: void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize for (f64, f64) {
  fn setPreferredSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn FreeQGraphicsLayoutItem<T: QGraphicsLayoutItem_FreeQGraphicsLayoutItem>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsLayoutItem(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_FreeQGraphicsLayoutItem {
  fn FreeQGraphicsLayoutItem(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::FreeQGraphicsLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_FreeQGraphicsLayoutItem for () {
  fn FreeQGraphicsLayoutItem(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemD0Ev()};
    unsafe {_ZN19QGraphicsLayoutItemD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumHeight<T: QGraphicsLayoutItem_setMinimumHeight>(&mut self, value: T) -> i32 {
    value.setMinimumHeight(self);
    return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumHeight {
  fn setMinimumHeight(self, this: &mut QGraphicsLayoutItem) -> i32;
}

// proto: void QGraphicsLayoutItem::setMinimumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumHeight for (f64) {
  fn setMinimumHeight(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMinimumHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem16setMinimumHeightEd(arg0)};
    return 1;
  }
}

// proto: void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize for (f64, f64) {
  fn setMaximumSize(self, this: &mut QGraphicsLayoutItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeEdd(arg0, arg1)};
    return 1;
  }
}

