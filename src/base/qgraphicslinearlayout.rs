// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicslayoutitem::QGraphicsLayoutItem;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: double QGraphicsLinearLayout::spacing();
  fn _ZNK21QGraphicsLinearLayout7spacingEv() -> i32;
  // proto: void QGraphicsLinearLayout::NewQGraphicsLinearLayout(QGraphicsLayoutItem * parent);
  fn _ZN21QGraphicsLinearLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsLinearLayout::NewQGraphicsLinearLayout(const QGraphicsLinearLayout & );
  fn _ZN21QGraphicsLinearLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
  fn _ZNK21QGraphicsLinearLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: void QGraphicsLinearLayout::invalidate();
  fn _ZN21QGraphicsLinearLayout10invalidateEv() -> i32;
  // proto: void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
  fn _ZN21QGraphicsLinearLayout11setGeometryERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QGraphicsLinearLayout::addStretch(int stretch);
  fn _ZN21QGraphicsLinearLayout10addStretchEi(arg0: c_int) -> i32;
  // proto: int QGraphicsLinearLayout::count();
  fn _ZNK21QGraphicsLinearLayout5countEv() -> i32;
  // proto: void QGraphicsLinearLayout::setSpacing(qreal spacing);
  fn _ZN21QGraphicsLinearLayout10setSpacingEd(arg0: c_double) -> i32;
  // proto: void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
  fn _ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QGraphicsLinearLayout::FreeQGraphicsLinearLayout();
  fn _ZN21QGraphicsLinearLayoutD0Ev() -> i32;
  // proto: void QGraphicsLinearLayout::dump(int indent);
  fn _ZNK21QGraphicsLinearLayout4dumpEi(arg0: c_int) -> i32;
  // proto: void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
  fn _ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
  fn _ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem(arg0: *mut c_void) -> i32;
  // proto: double QGraphicsLinearLayout::itemSpacing(int index);
  fn _ZNK21QGraphicsLinearLayout11itemSpacingEi(arg0: c_int) -> i32;
  // proto: void QGraphicsLinearLayout::removeAt(int index);
  fn _ZN21QGraphicsLinearLayout8removeAtEi(arg0: c_int) -> i32;
  // proto: void QGraphicsLinearLayout::insertStretch(int index, int stretch);
  fn _ZN21QGraphicsLinearLayout13insertStretchEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
  fn _ZN21QGraphicsLinearLayout14setItemSpacingEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
  fn _ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem(arg0: *mut c_void) -> i32;
  // proto: int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
  fn _ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QGraphicsLinearLayout)=1
pub struct QGraphicsLinearLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn spacing<T: QGraphicsLinearLayout_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_spacing {
  fn spacing(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: double QGraphicsLinearLayout::spacing();
impl<'a> /*trait*/ QGraphicsLinearLayout_spacing for () {
  fn spacing(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout7spacingEv()};
    unsafe {_ZNK21QGraphicsLinearLayout7spacingEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn NewQGraphicsLinearLayout<T: QGraphicsLinearLayout_NewQGraphicsLinearLayout>(value: T) -> QGraphicsLinearLayout {
    let rsthis = value.NewQGraphicsLinearLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_NewQGraphicsLinearLayout {
  fn NewQGraphicsLinearLayout(self) -> QGraphicsLinearLayout;
}

// proto: void QGraphicsLinearLayout::NewQGraphicsLinearLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLinearLayout_NewQGraphicsLinearLayout for (&'a mut QGraphicsLayoutItem) {
  fn NewQGraphicsLinearLayout(self) -> QGraphicsLinearLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsLinearLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsLinearLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsLinearLayout::NewQGraphicsLinearLayout(const QGraphicsLinearLayout & );
impl<'a> /*trait*/ QGraphicsLinearLayout_NewQGraphicsLinearLayout for (&'a  QGraphicsLinearLayout) {
  fn NewQGraphicsLinearLayout(self) -> QGraphicsLinearLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QGraphicsLinearLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsLinearLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemAt<T: QGraphicsLinearLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_itemAt {
  fn itemAt(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK21QGraphicsLinearLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn invalidate<T: QGraphicsLinearLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_invalidate {
  fn invalidate(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLinearLayout_invalidate for () {
  fn invalidate(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10invalidateEv()};
    unsafe {_ZN21QGraphicsLinearLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setGeometry<T: QGraphicsLinearLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_setGeometry {
  fn setGeometry(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLinearLayout_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn addStretch<T: QGraphicsLinearLayout_addStretch>(&mut self, value: T) -> i32 {
    value.addStretch(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_addStretch {
  fn addStretch(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::addStretch(int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_addStretch for (i32) {
  fn addStretch(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10addStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QGraphicsLinearLayout10addStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn count<T: QGraphicsLinearLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_count {
  fn count(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: int QGraphicsLinearLayout::count();
impl<'a> /*trait*/ QGraphicsLinearLayout_count for () {
  fn count(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout5countEv()};
    unsafe {_ZNK21QGraphicsLinearLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setSpacing<T: QGraphicsLinearLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_setSpacing {
  fn setSpacing(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsLinearLayout_setSpacing for (f64) {
  fn setSpacing(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10setSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN21QGraphicsLinearLayout10setSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertItem<T: QGraphicsLinearLayout_insertItem>(&mut self, value: T) -> i32 {
    value.insertItem(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_insertItem {
  fn insertItem(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_insertItem for (i32, &'a mut QGraphicsLayoutItem) {
  fn insertItem(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn FreeQGraphicsLinearLayout<T: QGraphicsLinearLayout_FreeQGraphicsLinearLayout>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsLinearLayout(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_FreeQGraphicsLinearLayout {
  fn FreeQGraphicsLinearLayout(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::FreeQGraphicsLinearLayout();
impl<'a> /*trait*/ QGraphicsLinearLayout_FreeQGraphicsLinearLayout for () {
  fn FreeQGraphicsLinearLayout(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutD0Ev()};
    unsafe {_ZN21QGraphicsLinearLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn dump<T: QGraphicsLinearLayout_dump>(&mut self, value: T) -> i32 {
    value.dump(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_dump {
  fn dump(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::dump(int indent);
impl<'a> /*trait*/ QGraphicsLinearLayout_dump for (i32) {
  fn dump(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout4dumpEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK21QGraphicsLinearLayout4dumpEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setStretchFactor<T: QGraphicsLinearLayout_setStretchFactor>(&mut self, value: T) -> i32 {
    value.setStretchFactor(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_setStretchFactor {
  fn setStretchFactor(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_setStretchFactor for (&'a mut QGraphicsLayoutItem, i32) {
  fn setStretchFactor(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn addItem<T: QGraphicsLinearLayout_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_addItem {
  fn addItem(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_addItem for (&'a mut QGraphicsLayoutItem) {
  fn addItem(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemSpacing<T: QGraphicsLinearLayout_itemSpacing>(&mut self, value: T) -> i32 {
    value.itemSpacing(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_itemSpacing {
  fn itemSpacing(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: double QGraphicsLinearLayout::itemSpacing(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_itemSpacing for (i32) {
  fn itemSpacing(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout11itemSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK21QGraphicsLinearLayout11itemSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeAt<T: QGraphicsLinearLayout_removeAt>(&mut self, value: T) -> i32 {
    value.removeAt(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_removeAt {
  fn removeAt(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_removeAt for (i32) {
  fn removeAt(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout8removeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QGraphicsLinearLayout8removeAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertStretch<T: QGraphicsLinearLayout_insertStretch>(&mut self, value: T) -> i32 {
    value.insertStretch(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_insertStretch {
  fn insertStretch(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::insertStretch(int index, int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_insertStretch for (i32, i32) {
  fn insertStretch(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout13insertStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN21QGraphicsLinearLayout13insertStretchEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setItemSpacing<T: QGraphicsLinearLayout_setItemSpacing>(&mut self, value: T) -> i32 {
    value.setItemSpacing(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_setItemSpacing {
  fn setItemSpacing(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
impl<'a> /*trait*/ QGraphicsLinearLayout_setItemSpacing for (i32, f64) {
  fn setItemSpacing(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout14setItemSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN21QGraphicsLinearLayout14setItemSpacingEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeItem<T: QGraphicsLinearLayout_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_removeItem {
  fn removeItem(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_removeItem for (&'a mut QGraphicsLayoutItem) {
  fn removeItem(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn stretchFactor<T: QGraphicsLinearLayout_stretchFactor>(&mut self, value: T) -> i32 {
    value.stretchFactor(self);
    return 1;
  }
}

pub trait QGraphicsLinearLayout_stretchFactor {
  fn stretchFactor(self, this: &mut QGraphicsLinearLayout) -> i32;
}

// proto: int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_stretchFactor for (&'a mut QGraphicsLayoutItem) {
  fn stretchFactor(self, this: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem(arg0)};
    return 1;
  }
}

