// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qlayoutitem::QLayoutItem;
use super::qrect::QRect;
use super::qsize::QSize;
use super::qmargins::QMargins;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QLayout::setContentsMargins(int left, int top, int right, int bottom);
  fn _ZN7QLayout18setContentsMarginsEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: int QLayout::spacing();
  fn _ZNK7QLayout7spacingEv() -> i32;
  // proto: void QLayout::NewQLayout();
  fn _ZN7QLayoutC1Ev(qthis: *mut c_void) -> i32;
  // proto: QRect QLayout::geometry();
  fn _ZNK7QLayout8geometryEv() -> i32;
  // proto: int QLayout::count();
  fn _ZNK7QLayout5countEv() -> i32;
  // proto: QSize QLayout::maximumSize();
  fn _ZNK7QLayout11maximumSizeEv() -> i32;
  // proto: void QLayout::setMenuBar(QWidget * w);
  fn _ZN7QLayout10setMenuBarEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: int QLayout::indexOf(QWidget * );
  fn _ZNK7QLayout7indexOfEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QLayout::setEnabled(bool );
  fn _ZN7QLayout10setEnabledEb(arg0: int8_t) -> i32;
  // proto: QSize QLayout::minimumSize();
  fn _ZNK7QLayout11minimumSizeEv() -> i32;
  // proto: QLayoutItem * QLayout::takeAt(int index);
  fn _ZN7QLayout6takeAtEi(arg0: c_int) -> i32;
  // proto: QSize QLayout::totalMaximumSize();
  fn _ZNK7QLayout16totalMaximumSizeEv() -> i32;
  // proto: void QLayout::invalidate();
  fn _ZN7QLayout10invalidateEv() -> i32;
  // proto: void QLayout::update();
  fn _ZN7QLayout6updateEv() -> i32;
  // proto: QRect QLayout::contentsRect();
  fn _ZNK7QLayout12contentsRectEv() -> i32;
  // proto: QSize QLayout::totalSizeHint();
  fn _ZNK7QLayout13totalSizeHintEv() -> i32;
  // proto: void QLayout::NewQLayout(QWidget * parent);
  fn _ZN7QLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QLayout::addItem(QLayoutItem * );
  fn _ZN7QLayout7addItemEP11QLayoutItem(arg0: *mut c_void) -> i32;
  // proto: int QLayout::totalHeightForWidth(int w);
  fn _ZNK7QLayout19totalHeightForWidthEi(arg0: c_int) -> i32;
  // proto: void QLayout::setMargin(int );
  fn _ZN7QLayout9setMarginEi(arg0: c_int) -> i32;
  // proto: bool QLayout::isEmpty();
  fn _ZNK7QLayout7isEmptyEv() -> i32;
  // proto: void QLayout::addWidget(QWidget * w);
  fn _ZN7QLayout9addWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QLayout::getContentsMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  // proto: QLayout * QLayout::layout();
  fn _ZN7QLayout6layoutEv() -> i32;
  // proto: bool QLayout::activate();
  fn _ZN7QLayout8activateEv() -> i32;
  // proto: bool QLayout::isEnabled();
  fn _ZNK7QLayout9isEnabledEv() -> i32;
  // proto: void QLayout::FreeQLayout();
  fn _ZN7QLayoutD0Ev() -> i32;
  // proto: int QLayout::margin();
  fn _ZNK7QLayout6marginEv() -> i32;
  // proto: void QLayout::setSpacing(int );
  fn _ZN7QLayout10setSpacingEi(arg0: c_int) -> i32;
  // proto: QWidget * QLayout::menuBar();
  fn _ZNK7QLayout7menuBarEv() -> i32;
  // proto: void QLayout::NewQLayout(const QLayout & );
  fn _ZN7QLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QLayout::metaObject();
  fn _ZNK7QLayout10metaObjectEv() -> i32;
  // proto: QLayoutItem * QLayout::itemAt(int index);
  fn _ZNK7QLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: QWidget * QLayout::parentWidget();
  fn _ZNK7QLayout12parentWidgetEv() -> i32;
  // proto: void QLayout::removeWidget(QWidget * w);
  fn _ZN7QLayout12removeWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QLayout::removeItem(QLayoutItem * );
  fn _ZN7QLayout10removeItemEP11QLayoutItem(arg0: *mut c_void) -> i32;
  // proto: QMargins QLayout::contentsMargins();
  fn _ZNK7QLayout15contentsMarginsEv() -> i32;
  // proto: QSize QLayout::totalMinimumSize();
  fn _ZNK7QLayout16totalMinimumSizeEv() -> i32;
  // proto: void QLayout::setGeometry(const QRect & );
  fn _ZN7QLayout11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: QSize QLayout::closestAcceptableSize(const QWidget * w, const QSize & s);
  fn _ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QLayout::setContentsMargins(const QMargins & margins);
  fn _ZN7QLayout18setContentsMarginsERK8QMargins(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QLayout)=1
pub struct QLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLayout {
  pub fn setContentsMargins<T: QLayout_setContentsMargins>(&mut self, value: T) -> i32 {
    value.setContentsMargins(self);
    return 1;
  }
}

pub trait QLayout_setContentsMargins {
  fn setContentsMargins(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::setContentsMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QLayout_setContentsMargins for (i32, i32, i32, i32) {
  fn setContentsMargins(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout18setContentsMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QLayout18setContentsMarginsEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn spacing<T: QLayout_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QLayout_spacing {
  fn spacing(self, this: &mut QLayout) -> i32;
}

// proto: int QLayout::spacing();
impl<'a> /*trait*/ QLayout_spacing for () {
  fn spacing(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7spacingEv()};
    unsafe {_ZNK7QLayout7spacingEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn NewQLayout<T: QLayout_NewQLayout>(value: T) -> QLayout {
    let rsthis = value.NewQLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QLayout_NewQLayout {
  fn NewQLayout(self) -> QLayout;
}

// proto: void QLayout::NewQLayout();
impl<'a> /*trait*/ QLayout_NewQLayout for () {
  fn NewQLayout(self) -> QLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayoutC1Ev()};
    unsafe {_ZN7QLayoutC1Ev(qthis)};
    let rsthis = QLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn geometry<T: QLayout_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QLayout_geometry {
  fn geometry(self, this: &mut QLayout) -> i32;
}

// proto: QRect QLayout::geometry();
impl<'a> /*trait*/ QLayout_geometry for () {
  fn geometry(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout8geometryEv()};
    unsafe {_ZNK7QLayout8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn count<T: QLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QLayout_count {
  fn count(self, this: &mut QLayout) -> i32;
}

// proto: int QLayout::count();
impl<'a> /*trait*/ QLayout_count for () {
  fn count(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout5countEv()};
    unsafe {_ZNK7QLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn maximumSize<T: QLayout_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QLayout_maximumSize {
  fn maximumSize(self, this: &mut QLayout) -> i32;
}

// proto: QSize QLayout::maximumSize();
impl<'a> /*trait*/ QLayout_maximumSize for () {
  fn maximumSize(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout11maximumSizeEv()};
    unsafe {_ZNK7QLayout11maximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setMenuBar<T: QLayout_setMenuBar>(&mut self, value: T) -> i32 {
    value.setMenuBar(self);
    return 1;
  }
}

pub trait QLayout_setMenuBar {
  fn setMenuBar(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::setMenuBar(QWidget * w);
impl<'a> /*trait*/ QLayout_setMenuBar for (&'a mut QWidget) {
  fn setMenuBar(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10setMenuBarEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayout10setMenuBarEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn indexOf<T: QLayout_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QLayout_indexOf {
  fn indexOf(self, this: &mut QLayout) -> i32;
}

// proto: int QLayout::indexOf(QWidget * );
impl<'a> /*trait*/ QLayout_indexOf for (&'a mut QWidget) {
  fn indexOf(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK7QLayout7indexOfEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setEnabled<T: QLayout_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QLayout_setEnabled {
  fn setEnabled(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::setEnabled(bool );
impl<'a> /*trait*/ QLayout_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QLayout10setEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn minimumSize<T: QLayout_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QLayout_minimumSize {
  fn minimumSize(self, this: &mut QLayout) -> i32;
}

// proto: QSize QLayout::minimumSize();
impl<'a> /*trait*/ QLayout_minimumSize for () {
  fn minimumSize(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout11minimumSizeEv()};
    unsafe {_ZNK7QLayout11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn takeAt<T: QLayout_takeAt>(&mut self, value: T) -> i32 {
    value.takeAt(self);
    return 1;
  }
}

pub trait QLayout_takeAt {
  fn takeAt(self, this: &mut QLayout) -> i32;
}

// proto: QLayoutItem * QLayout::takeAt(int index);
impl<'a> /*trait*/ QLayout_takeAt for (i32) {
  fn takeAt(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout6takeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QLayout6takeAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalMaximumSize<T: QLayout_totalMaximumSize>(&mut self, value: T) -> i32 {
    value.totalMaximumSize(self);
    return 1;
  }
}

pub trait QLayout_totalMaximumSize {
  fn totalMaximumSize(self, this: &mut QLayout) -> i32;
}

// proto: QSize QLayout::totalMaximumSize();
impl<'a> /*trait*/ QLayout_totalMaximumSize for () {
  fn totalMaximumSize(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout16totalMaximumSizeEv()};
    unsafe {_ZNK7QLayout16totalMaximumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn invalidate<T: QLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QLayout_invalidate {
  fn invalidate(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::invalidate();
impl<'a> /*trait*/ QLayout_invalidate for () {
  fn invalidate(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10invalidateEv()};
    unsafe {_ZN7QLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn update<T: QLayout_update>(&mut self, value: T) -> i32 {
    value.update(self);
    return 1;
  }
}

pub trait QLayout_update {
  fn update(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::update();
impl<'a> /*trait*/ QLayout_update for () {
  fn update(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout6updateEv()};
    unsafe {_ZN7QLayout6updateEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn contentsRect<T: QLayout_contentsRect>(&mut self, value: T) -> i32 {
    value.contentsRect(self);
    return 1;
  }
}

pub trait QLayout_contentsRect {
  fn contentsRect(self, this: &mut QLayout) -> i32;
}

// proto: QRect QLayout::contentsRect();
impl<'a> /*trait*/ QLayout_contentsRect for () {
  fn contentsRect(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout12contentsRectEv()};
    unsafe {_ZNK7QLayout12contentsRectEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalSizeHint<T: QLayout_totalSizeHint>(&mut self, value: T) -> i32 {
    value.totalSizeHint(self);
    return 1;
  }
}

pub trait QLayout_totalSizeHint {
  fn totalSizeHint(self, this: &mut QLayout) -> i32;
}

// proto: QSize QLayout::totalSizeHint();
impl<'a> /*trait*/ QLayout_totalSizeHint for () {
  fn totalSizeHint(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout13totalSizeHintEv()};
    unsafe {_ZNK7QLayout13totalSizeHintEv()};
    return 1;
  }
}

// proto: void QLayout::NewQLayout(QWidget * parent);
impl<'a> /*trait*/ QLayout_NewQLayout for (&'a mut QWidget) {
  fn NewQLayout(self) -> QLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn addItem<T: QLayout_addItem>(&mut self, value: T) -> i32 {
    value.addItem(self);
    return 1;
  }
}

pub trait QLayout_addItem {
  fn addItem(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::addItem(QLayoutItem * );
impl<'a> /*trait*/ QLayout_addItem for (&'a mut QLayoutItem) {
  fn addItem(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayout7addItemEP11QLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalHeightForWidth<T: QLayout_totalHeightForWidth>(&mut self, value: T) -> i32 {
    value.totalHeightForWidth(self);
    return 1;
  }
}

pub trait QLayout_totalHeightForWidth {
  fn totalHeightForWidth(self, this: &mut QLayout) -> i32;
}

// proto: int QLayout::totalHeightForWidth(int w);
impl<'a> /*trait*/ QLayout_totalHeightForWidth for (i32) {
  fn totalHeightForWidth(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout19totalHeightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QLayout19totalHeightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setMargin<T: QLayout_setMargin>(&mut self, value: T) -> i32 {
    value.setMargin(self);
    return 1;
  }
}

pub trait QLayout_setMargin {
  fn setMargin(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::setMargin(int );
impl<'a> /*trait*/ QLayout_setMargin for (i32) {
  fn setMargin(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout9setMarginEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QLayout9setMarginEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn isEmpty<T: QLayout_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QLayout_isEmpty {
  fn isEmpty(self, this: &mut QLayout) -> i32;
}

// proto: bool QLayout::isEmpty();
impl<'a> /*trait*/ QLayout_isEmpty for () {
  fn isEmpty(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7isEmptyEv()};
    unsafe {_ZNK7QLayout7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn addWidget<T: QLayout_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QLayout_addWidget {
  fn addWidget(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QLayout_addWidget for (&'a mut QWidget) {
  fn addWidget(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayout9addWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn getContentsMargins<T: QLayout_getContentsMargins>(&mut self, value: T) -> i32 {
    value.getContentsMargins(self);
    return 1;
  }
}

pub trait QLayout_getContentsMargins {
  fn getContentsMargins(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::getContentsMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QLayout_getContentsMargins for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getContentsMargins(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn layout<T: QLayout_layout>(&mut self, value: T) -> i32 {
    value.layout(self);
    return 1;
  }
}

pub trait QLayout_layout {
  fn layout(self, this: &mut QLayout) -> i32;
}

// proto: QLayout * QLayout::layout();
impl<'a> /*trait*/ QLayout_layout for () {
  fn layout(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout6layoutEv()};
    unsafe {_ZN7QLayout6layoutEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn activate<T: QLayout_activate>(&mut self, value: T) -> i32 {
    value.activate(self);
    return 1;
  }
}

pub trait QLayout_activate {
  fn activate(self, this: &mut QLayout) -> i32;
}

// proto: bool QLayout::activate();
impl<'a> /*trait*/ QLayout_activate for () {
  fn activate(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout8activateEv()};
    unsafe {_ZN7QLayout8activateEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn isEnabled<T: QLayout_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QLayout_isEnabled {
  fn isEnabled(self, this: &mut QLayout) -> i32;
}

// proto: bool QLayout::isEnabled();
impl<'a> /*trait*/ QLayout_isEnabled for () {
  fn isEnabled(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout9isEnabledEv()};
    unsafe {_ZNK7QLayout9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn FreeQLayout<T: QLayout_FreeQLayout>(&mut self, value: T) -> i32 {
    value.FreeQLayout(self);
    return 1;
  }
}

pub trait QLayout_FreeQLayout {
  fn FreeQLayout(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::FreeQLayout();
impl<'a> /*trait*/ QLayout_FreeQLayout for () {
  fn FreeQLayout(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayoutD0Ev()};
    unsafe {_ZN7QLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn margin<T: QLayout_margin>(&mut self, value: T) -> i32 {
    value.margin(self);
    return 1;
  }
}

pub trait QLayout_margin {
  fn margin(self, this: &mut QLayout) -> i32;
}

// proto: int QLayout::margin();
impl<'a> /*trait*/ QLayout_margin for () {
  fn margin(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout6marginEv()};
    unsafe {_ZNK7QLayout6marginEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setSpacing<T: QLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QLayout_setSpacing {
  fn setSpacing(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::setSpacing(int );
impl<'a> /*trait*/ QLayout_setSpacing for (i32) {
  fn setSpacing(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10setSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QLayout10setSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn menuBar<T: QLayout_menuBar>(&mut self, value: T) -> i32 {
    value.menuBar(self);
    return 1;
  }
}

pub trait QLayout_menuBar {
  fn menuBar(self, this: &mut QLayout) -> i32;
}

// proto: QWidget * QLayout::menuBar();
impl<'a> /*trait*/ QLayout_menuBar for () {
  fn menuBar(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7menuBarEv()};
    unsafe {_ZNK7QLayout7menuBarEv()};
    return 1;
  }
}

// proto: void QLayout::NewQLayout(const QLayout & );
impl<'a> /*trait*/ QLayout_NewQLayout for (&'a  QLayout) {
  fn NewQLayout(self) -> QLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn metaObject<T: QLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QLayout_metaObject {
  fn metaObject(self, this: &mut QLayout) -> i32;
}

// proto: const QMetaObject * QLayout::metaObject();
impl<'a> /*trait*/ QLayout_metaObject for () {
  fn metaObject(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout10metaObjectEv()};
    unsafe {_ZNK7QLayout10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn itemAt<T: QLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QLayout_itemAt {
  fn itemAt(self, this: &mut QLayout) -> i32;
}

// proto: QLayoutItem * QLayout::itemAt(int index);
impl<'a> /*trait*/ QLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn parentWidget<T: QLayout_parentWidget>(&mut self, value: T) -> i32 {
    value.parentWidget(self);
    return 1;
  }
}

pub trait QLayout_parentWidget {
  fn parentWidget(self, this: &mut QLayout) -> i32;
}

// proto: QWidget * QLayout::parentWidget();
impl<'a> /*trait*/ QLayout_parentWidget for () {
  fn parentWidget(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout12parentWidgetEv()};
    unsafe {_ZNK7QLayout12parentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn removeWidget<T: QLayout_removeWidget>(&mut self, value: T) -> i32 {
    value.removeWidget(self);
    return 1;
  }
}

pub trait QLayout_removeWidget {
  fn removeWidget(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::removeWidget(QWidget * w);
impl<'a> /*trait*/ QLayout_removeWidget for (&'a mut QWidget) {
  fn removeWidget(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayout12removeWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn removeItem<T: QLayout_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QLayout_removeItem {
  fn removeItem(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::removeItem(QLayoutItem * );
impl<'a> /*trait*/ QLayout_removeItem for (&'a mut QLayoutItem) {
  fn removeItem(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10removeItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayout10removeItemEP11QLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn contentsMargins<T: QLayout_contentsMargins>(&mut self, value: T) -> i32 {
    value.contentsMargins(self);
    return 1;
  }
}

pub trait QLayout_contentsMargins {
  fn contentsMargins(self, this: &mut QLayout) -> i32;
}

// proto: QMargins QLayout::contentsMargins();
impl<'a> /*trait*/ QLayout_contentsMargins for () {
  fn contentsMargins(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout15contentsMarginsEv()};
    unsafe {_ZNK7QLayout15contentsMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalMinimumSize<T: QLayout_totalMinimumSize>(&mut self, value: T) -> i32 {
    value.totalMinimumSize(self);
    return 1;
  }
}

pub trait QLayout_totalMinimumSize {
  fn totalMinimumSize(self, this: &mut QLayout) -> i32;
}

// proto: QSize QLayout::totalMinimumSize();
impl<'a> /*trait*/ QLayout_totalMinimumSize for () {
  fn totalMinimumSize(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout16totalMinimumSizeEv()};
    unsafe {_ZNK7QLayout16totalMinimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setGeometry<T: QLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QLayout_setGeometry {
  fn setGeometry(self, this: &mut QLayout) -> i32;
}

// proto: void QLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QLayout11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn closestAcceptableSize<T: QLayout_closestAcceptableSize>(&mut self, value: T) -> i32 {
    value.closestAcceptableSize(self);
    return 1;
  }
}

pub trait QLayout_closestAcceptableSize {
  fn closestAcceptableSize(self, this: &mut QLayout) -> i32;
}

// proto: QSize QLayout::closestAcceptableSize(const QWidget * w, const QSize & s);
impl<'a> /*trait*/ QLayout_closestAcceptableSize for (&'a  QWidget, &'a  QSize) {
  fn closestAcceptableSize(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize(arg0, arg1)};
    return 1;
  }
}

// proto: void QLayout::setContentsMargins(const QMargins & margins);
impl<'a> /*trait*/ QLayout_setContentsMargins for (&'a  QMargins) {
  fn setContentsMargins(self, this: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout18setContentsMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QLayout18setContentsMarginsERK8QMargins(arg0)};
    return 1;
  }
}

