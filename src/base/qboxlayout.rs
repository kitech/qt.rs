// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qlayoutitem::QLayoutItem;
use super::qlayout::QLayout;
use super::qwidget::QWidget;
use super::qrect::QRect;
use super::qspaceritem::QSpacerItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: int QBoxLayout::spacing();
  fn _ZNK10QBoxLayout7spacingEv() -> i32;
  // proto: bool QBoxLayout::hasHeightForWidth();
  fn _ZNK10QBoxLayout17hasHeightForWidthEv() -> i32;
  // proto: void QBoxLayout::addItem(QLayoutItem * );
  fn _ZN10QBoxLayout7addItemEP11QLayoutItem(arg0: *mut c_void) -> i32;
  // proto: QSize QBoxLayout::sizeHint();
  fn _ZNK10QBoxLayout8sizeHintEv() -> i32;
  // proto: void QBoxLayout::FreeQBoxLayout();
  fn _ZN10QBoxLayoutD0Ev() -> i32;
  // proto: void QBoxLayout::insertSpacing(int index, int size);
  fn _ZN10QBoxLayout13insertSpacingEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QBoxLayout::setStretch(int index, int stretch);
  fn _ZN10QBoxLayout10setStretchEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QBoxLayout::NewQBoxLayout(const QBoxLayout & );
  fn _ZN10QBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QBoxLayout::insertStretch(int index, int stretch);
  fn _ZN10QBoxLayout13insertStretchEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QBoxLayout::addLayout(QLayout * layout, int stretch);
  fn _ZN10QBoxLayout9addLayoutEP7QLayouti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: bool QBoxLayout::setStretchFactor(QWidget * w, int stretch);
  fn _ZN10QBoxLayout16setStretchFactorEP7QWidgeti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QBoxLayout::invalidate();
  fn _ZN10QBoxLayout10invalidateEv() -> i32;
  // proto: void QBoxLayout::setGeometry(const QRect & );
  fn _ZN10QBoxLayout11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QBoxLayout::addStretch(int stretch);
  fn _ZN10QBoxLayout10addStretchEi(arg0: c_int) -> i32;
  // proto: void QBoxLayout::insertLayout(int index, QLayout * layout, int stretch);
  fn _ZN10QBoxLayout12insertLayoutEiP7QLayouti(arg0: c_int, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: bool QBoxLayout::setStretchFactor(QLayout * l, int stretch);
  fn _ZN10QBoxLayout16setStretchFactorEP7QLayouti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: int QBoxLayout::count();
  fn _ZNK10QBoxLayout5countEv() -> i32;
  // proto: QLayoutItem * QBoxLayout::itemAt(int );
  fn _ZNK10QBoxLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QBoxLayout::metaObject();
  fn _ZNK10QBoxLayout10metaObjectEv() -> i32;
  // proto: void QBoxLayout::insertSpacerItem(int index, QSpacerItem * spacerItem);
  fn _ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: int QBoxLayout::heightForWidth(int );
  fn _ZNK10QBoxLayout14heightForWidthEi(arg0: c_int) -> i32;
  // proto: void QBoxLayout::addStrut(int );
  fn _ZN10QBoxLayout8addStrutEi(arg0: c_int) -> i32;
  // proto: QSize QBoxLayout::maximumSize();
  fn _ZNK10QBoxLayout11maximumSizeEv() -> i32;
  // proto: int QBoxLayout::stretch(int index);
  fn _ZNK10QBoxLayout7stretchEi(arg0: c_int) -> i32;
  // proto: void QBoxLayout::addSpacerItem(QSpacerItem * spacerItem);
  fn _ZN10QBoxLayout13addSpacerItemEP11QSpacerItem(arg0: *mut c_void) -> i32;
  // proto: int QBoxLayout::minimumHeightForWidth(int );
  fn _ZNK10QBoxLayout21minimumHeightForWidthEi(arg0: c_int) -> i32;
  // proto: QSize QBoxLayout::minimumSize();
  fn _ZNK10QBoxLayout11minimumSizeEv() -> i32;
  // proto: void QBoxLayout::setSpacing(int spacing);
  fn _ZN10QBoxLayout10setSpacingEi(arg0: c_int) -> i32;
  // proto: QLayoutItem * QBoxLayout::takeAt(int );
  fn _ZN10QBoxLayout6takeAtEi(arg0: c_int) -> i32;
  // proto: void QBoxLayout::insertItem(int index, QLayoutItem * );
  fn _ZN10QBoxLayout10insertItemEiP11QLayoutItem(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: void QBoxLayout::addSpacing(int size);
  fn _ZN10QBoxLayout10addSpacingEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QBoxLayout)=1
pub struct QBoxLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBoxLayout {
  pub fn spacing<T: QBoxLayout_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QBoxLayout_spacing {
  fn spacing(self, this: &mut QBoxLayout) -> i32;
}

// proto: int QBoxLayout::spacing();
impl<'a> /*trait*/ QBoxLayout_spacing for () {
  fn spacing(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout7spacingEv()};
    unsafe {_ZNK10QBoxLayout7spacingEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn hasHeightForWidth<T: QBoxLayout_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QBoxLayout_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QBoxLayout) -> i32;
}

// proto: bool QBoxLayout::hasHeightForWidth();
impl<'a> /*trait*/ QBoxLayout_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout17hasHeightForWidthEv()};
    unsafe {_ZNK10QBoxLayout17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn addItem<T: QBoxLayout_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QBoxLayout_addItem {
  fn addItem(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::addItem(QLayoutItem * );
impl<'a> /*trait*/ QBoxLayout_addItem for (&'a mut QLayoutItem) {
  fn addItem(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QBoxLayout7addItemEP11QLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn sizeHint<T: QBoxLayout_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QBoxLayout_sizeHint {
  fn sizeHint(self, this: &mut QBoxLayout) -> i32;
}

// proto: QSize QBoxLayout::sizeHint();
impl<'a> /*trait*/ QBoxLayout_sizeHint for () {
  fn sizeHint(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout8sizeHintEv()};
    unsafe {_ZNK10QBoxLayout8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn FreeQBoxLayout<T: QBoxLayout_FreeQBoxLayout>(&mut self, value: T) -> i32 {
    value.FreeQBoxLayout(self);
    return 1;
  }
}

pub trait QBoxLayout_FreeQBoxLayout {
  fn FreeQBoxLayout(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::FreeQBoxLayout();
impl<'a> /*trait*/ QBoxLayout_FreeQBoxLayout for () {
  fn FreeQBoxLayout(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayoutD0Ev()};
    unsafe {_ZN10QBoxLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn insertSpacing<T: QBoxLayout_insertSpacing>(&mut self, value: T) -> i32 {
    value.insertSpacing(self);
    return 1;
  }
}

pub trait QBoxLayout_insertSpacing {
  fn insertSpacing(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::insertSpacing(int index, int size);
impl<'a> /*trait*/ QBoxLayout_insertSpacing for (i32, i32) {
  fn insertSpacing(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout13insertSpacingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QBoxLayout13insertSpacingEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn setStretch<T: QBoxLayout_setStretch>(&mut self, value: T) -> i32 {
    value.setStretch(self);
    return 1;
  }
}

pub trait QBoxLayout_setStretch {
  fn setStretch(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::setStretch(int index, int stretch);
impl<'a> /*trait*/ QBoxLayout_setStretch for (i32, i32) {
  fn setStretch(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10setStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QBoxLayout10setStretchEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn NewQBoxLayout<T: QBoxLayout_NewQBoxLayout>(value: T) -> QBoxLayout {
    let rsthis = value.NewQBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QBoxLayout_NewQBoxLayout {
  fn NewQBoxLayout(self) -> QBoxLayout;
}

// proto: void QBoxLayout::NewQBoxLayout(const QBoxLayout & );
impl<'a> /*trait*/ QBoxLayout_NewQBoxLayout for (&'a  QBoxLayout) {
  fn NewQBoxLayout(self) -> QBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn insertStretch<T: QBoxLayout_insertStretch>(&mut self, value: T) -> i32 {
    value.insertStretch(self);
    return 1;
  }
}

pub trait QBoxLayout_insertStretch {
  fn insertStretch(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::insertStretch(int index, int stretch);
impl<'a> /*trait*/ QBoxLayout_insertStretch for (i32, i32) {
  fn insertStretch(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout13insertStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QBoxLayout13insertStretchEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn addLayout<T: QBoxLayout_addLayout>(&mut self, value: T) -> i32 {
    value.addLayout(self);
    return 1;
  }
}

pub trait QBoxLayout_addLayout {
  fn addLayout(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::addLayout(QLayout * layout, int stretch);
impl<'a> /*trait*/ QBoxLayout_addLayout for (&'a mut QLayout, i32) {
  fn addLayout(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout9addLayoutEP7QLayouti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QBoxLayout9addLayoutEP7QLayouti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn setStretchFactor<T: QBoxLayout_setStretchFactor>(&mut self, value: T) -> i32 {
    value.setStretchFactor(self);
    return 1;
  }
}

pub trait QBoxLayout_setStretchFactor {
  fn setStretchFactor(self, this: &mut QBoxLayout) -> i32;
}

// proto: bool QBoxLayout::setStretchFactor(QWidget * w, int stretch);
impl<'a> /*trait*/ QBoxLayout_setStretchFactor for (&'a mut QWidget, i32) {
  fn setStretchFactor(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout16setStretchFactorEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QBoxLayout16setStretchFactorEP7QWidgeti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn invalidate<T: QBoxLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QBoxLayout_invalidate {
  fn invalidate(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::invalidate();
impl<'a> /*trait*/ QBoxLayout_invalidate for () {
  fn invalidate(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10invalidateEv()};
    unsafe {_ZN10QBoxLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn setGeometry<T: QBoxLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QBoxLayout_setGeometry {
  fn setGeometry(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QBoxLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QBoxLayout11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn addStretch<T: QBoxLayout_addStretch>(&mut self, value: T) -> i32 {
    value.addStretch(self);
    return 1;
  }
}

pub trait QBoxLayout_addStretch {
  fn addStretch(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::addStretch(int stretch);
impl<'a> /*trait*/ QBoxLayout_addStretch for (i32) {
  fn addStretch(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10addStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QBoxLayout10addStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn insertLayout<T: QBoxLayout_insertLayout>(&mut self, value: T) -> i32 {
    value.insertLayout(self);
    return 1;
  }
}

pub trait QBoxLayout_insertLayout {
  fn insertLayout(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::insertLayout(int index, QLayout * layout, int stretch);
impl<'a> /*trait*/ QBoxLayout_insertLayout for (i32, &'a mut QLayout, i32) {
  fn insertLayout(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout12insertLayoutEiP7QLayouti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QBoxLayout12insertLayoutEiP7QLayouti(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: bool QBoxLayout::setStretchFactor(QLayout * l, int stretch);
impl<'a> /*trait*/ QBoxLayout_setStretchFactor for (&'a mut QLayout, i32) {
  fn setStretchFactor(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout16setStretchFactorEP7QLayouti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QBoxLayout16setStretchFactorEP7QLayouti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn count<T: QBoxLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QBoxLayout_count {
  fn count(self, this: &mut QBoxLayout) -> i32;
}

// proto: int QBoxLayout::count();
impl<'a> /*trait*/ QBoxLayout_count for () {
  fn count(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout5countEv()};
    unsafe {_ZNK10QBoxLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn itemAt<T: QBoxLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QBoxLayout_itemAt {
  fn itemAt(self, this: &mut QBoxLayout) -> i32;
}

// proto: QLayoutItem * QBoxLayout::itemAt(int );
impl<'a> /*trait*/ QBoxLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QBoxLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn metaObject<T: QBoxLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QBoxLayout_metaObject {
  fn metaObject(self, this: &mut QBoxLayout) -> i32;
}

// proto: const QMetaObject * QBoxLayout::metaObject();
impl<'a> /*trait*/ QBoxLayout_metaObject for () {
  fn metaObject(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout10metaObjectEv()};
    unsafe {_ZNK10QBoxLayout10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn insertSpacerItem<T: QBoxLayout_insertSpacerItem>(&mut self, value: T) -> i32 {
    value.insertSpacerItem(self);
    return 1;
  }
}

pub trait QBoxLayout_insertSpacerItem {
  fn insertSpacerItem(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::insertSpacerItem(int index, QSpacerItem * spacerItem);
impl<'a> /*trait*/ QBoxLayout_insertSpacerItem for (i32, &'a mut QSpacerItem) {
  fn insertSpacerItem(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QBoxLayout16insertSpacerItemEiP11QSpacerItem(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn heightForWidth<T: QBoxLayout_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QBoxLayout_heightForWidth {
  fn heightForWidth(self, this: &mut QBoxLayout) -> i32;
}

// proto: int QBoxLayout::heightForWidth(int );
impl<'a> /*trait*/ QBoxLayout_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QBoxLayout14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn addStrut<T: QBoxLayout_addStrut>(&mut self, value: T) -> i32 {
    value.addStrut(self);
    return 1;
  }
}

pub trait QBoxLayout_addStrut {
  fn addStrut(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::addStrut(int );
impl<'a> /*trait*/ QBoxLayout_addStrut for (i32) {
  fn addStrut(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout8addStrutEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QBoxLayout8addStrutEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn maximumSize<T: QBoxLayout_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QBoxLayout_maximumSize {
  fn maximumSize(self, this: &mut QBoxLayout) -> i32;
}

// proto: QSize QBoxLayout::maximumSize();
impl<'a> /*trait*/ QBoxLayout_maximumSize for () {
  fn maximumSize(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout11maximumSizeEv()};
    unsafe {_ZNK10QBoxLayout11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn stretch<T: QBoxLayout_stretch>(&mut self, value: T) -> i32 {
    value.stretch(self);
    return 1;
  }
}

pub trait QBoxLayout_stretch {
  fn stretch(self, this: &mut QBoxLayout) -> i32;
}

// proto: int QBoxLayout::stretch(int index);
impl<'a> /*trait*/ QBoxLayout_stretch for (i32) {
  fn stretch(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout7stretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QBoxLayout7stretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn addSpacerItem<T: QBoxLayout_addSpacerItem>(&mut self, value: T) -> i32 {
    value.addSpacerItem(self);
    return 1;
  }
}

pub trait QBoxLayout_addSpacerItem {
  fn addSpacerItem(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::addSpacerItem(QSpacerItem * spacerItem);
impl<'a> /*trait*/ QBoxLayout_addSpacerItem for (&'a mut QSpacerItem) {
  fn addSpacerItem(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout13addSpacerItemEP11QSpacerItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QBoxLayout13addSpacerItemEP11QSpacerItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn minimumHeightForWidth<T: QBoxLayout_minimumHeightForWidth>(&mut self, value: T) -> i32 {
    value.minimumHeightForWidth(self);
    return 1;
  }
}

pub trait QBoxLayout_minimumHeightForWidth {
  fn minimumHeightForWidth(self, this: &mut QBoxLayout) -> i32;
}

// proto: int QBoxLayout::minimumHeightForWidth(int );
impl<'a> /*trait*/ QBoxLayout_minimumHeightForWidth for (i32) {
  fn minimumHeightForWidth(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QBoxLayout21minimumHeightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn minimumSize<T: QBoxLayout_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QBoxLayout_minimumSize {
  fn minimumSize(self, this: &mut QBoxLayout) -> i32;
}

// proto: QSize QBoxLayout::minimumSize();
impl<'a> /*trait*/ QBoxLayout_minimumSize for () {
  fn minimumSize(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QBoxLayout11minimumSizeEv()};
    unsafe {_ZNK10QBoxLayout11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn setSpacing<T: QBoxLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QBoxLayout_setSpacing {
  fn setSpacing(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::setSpacing(int spacing);
impl<'a> /*trait*/ QBoxLayout_setSpacing for (i32) {
  fn setSpacing(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10setSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QBoxLayout10setSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn takeAt<T: QBoxLayout_takeAt>(&mut self, value: T) -> i32 {
    value.takeAt(self);
    return 1;
  }
}

pub trait QBoxLayout_takeAt {
  fn takeAt(self, this: &mut QBoxLayout) -> i32;
}

// proto: QLayoutItem * QBoxLayout::takeAt(int );
impl<'a> /*trait*/ QBoxLayout_takeAt for (i32) {
  fn takeAt(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout6takeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QBoxLayout6takeAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn insertItem<T: QBoxLayout_insertItem>(&mut self, value: T) -> i32 {
    value.insertItem(self);
    return 1;
  }
}

pub trait QBoxLayout_insertItem {
  fn insertItem(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::insertItem(int index, QLayoutItem * );
impl<'a> /*trait*/ QBoxLayout_insertItem for (i32, &'a mut QLayoutItem) {
  fn insertItem(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10insertItemEiP11QLayoutItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QBoxLayout10insertItemEiP11QLayoutItem(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QBoxLayout {
  pub fn addSpacing<T: QBoxLayout_addSpacing>(&mut self, value: T) -> i32 {
    value.addSpacing(self);
    return 1;
  }
}

pub trait QBoxLayout_addSpacing {
  fn addSpacing(self, this: &mut QBoxLayout) -> i32;
}

// proto: void QBoxLayout::addSpacing(int size);
impl<'a> /*trait*/ QBoxLayout_addSpacing for (i32) {
  fn addSpacing(self, this: &mut QBoxLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QBoxLayout10addSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QBoxLayout10addSpacingEi(arg0)};
    return 1;
  }
}

