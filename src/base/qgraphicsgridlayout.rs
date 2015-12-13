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
  // proto: void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout21setRowPreferredHeightEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: int QGraphicsGridLayout::columnCount();
  fn _ZNK19QGraphicsGridLayout11columnCountEv() -> i32;
  // proto: QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
  fn _ZNK19QGraphicsGridLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: int QGraphicsGridLayout::count();
  fn _ZNK19QGraphicsGridLayout5countEv() -> i32;
  // proto: void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout19setColumnFixedWidthEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout21setColumnMaximumWidthEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: int QGraphicsGridLayout::rowStretchFactor(int row);
  fn _ZNK19QGraphicsGridLayout16rowStretchFactorEi(arg0: c_int) -> i32;
  // proto: double QGraphicsGridLayout::verticalSpacing();
  fn _ZNK19QGraphicsGridLayout15verticalSpacingEv() -> i32;
  // proto: int QGraphicsGridLayout::columnStretchFactor(int column);
  fn _ZNK19QGraphicsGridLayout19columnStretchFactorEi(arg0: c_int) -> i32;
  // proto: void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout19setRowMaximumHeightEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
  fn _ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsGridLayout::FreeQGraphicsGridLayout();
  fn _ZN19QGraphicsGridLayoutD0Ev() -> i32;
  // proto: double QGraphicsGridLayout::rowMinimumHeight(int row);
  fn _ZNK19QGraphicsGridLayout16rowMinimumHeightEi(arg0: c_int) -> i32;
  // proto: double QGraphicsGridLayout::rowMaximumHeight(int row);
  fn _ZNK19QGraphicsGridLayout16rowMaximumHeightEi(arg0: c_int) -> i32;
  // proto: void QGraphicsGridLayout::NewQGraphicsGridLayout(QGraphicsLayoutItem * parent);
  fn _ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
  fn _ZN19QGraphicsGridLayout16setColumnSpacingEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: double QGraphicsGridLayout::rowSpacing(int row);
  fn _ZNK19QGraphicsGridLayout10rowSpacingEi(arg0: c_int) -> i32;
  // proto: double QGraphicsGridLayout::columnMaximumWidth(int column);
  fn _ZNK19QGraphicsGridLayout18columnMaximumWidthEi(arg0: c_int) -> i32;
  // proto: void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout17setRowFixedHeightEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: double QGraphicsGridLayout::rowPreferredHeight(int row);
  fn _ZNK19QGraphicsGridLayout18rowPreferredHeightEi(arg0: c_int) -> i32;
  // proto: QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int row, int column);
  fn _ZNK19QGraphicsGridLayout6itemAtEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout18setVerticalSpacingEd(arg0: c_double) -> i32;
  // proto: void QGraphicsGridLayout::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsGridLayout11setGeometryERK6QRectF(arg0: *const c_void) -> i32;
  // proto: int QGraphicsGridLayout::rowCount();
  fn _ZNK19QGraphicsGridLayout8rowCountEv() -> i32;
  // proto: void QGraphicsGridLayout::setSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout10setSpacingEd(arg0: c_double) -> i32;
  // proto: void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
  fn _ZN19QGraphicsGridLayout19setRowStretchFactorEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: double QGraphicsGridLayout::columnMinimumWidth(int column);
  fn _ZNK19QGraphicsGridLayout18columnMinimumWidthEi(arg0: c_int) -> i32;
  // proto: void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout21setColumnMinimumWidthEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout19setRowMinimumHeightEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout20setHorizontalSpacingEd(arg0: c_double) -> i32;
  // proto: double QGraphicsGridLayout::horizontalSpacing();
  fn _ZNK19QGraphicsGridLayout17horizontalSpacingEv() -> i32;
  // proto: void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
  fn _ZN19QGraphicsGridLayout22setColumnStretchFactorEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGraphicsGridLayout::invalidate();
  fn _ZN19QGraphicsGridLayout10invalidateEv() -> i32;
  // proto: double QGraphicsGridLayout::columnPreferredWidth(int column);
  fn _ZNK19QGraphicsGridLayout20columnPreferredWidthEi(arg0: c_int) -> i32;
  // proto: void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout23setColumnPreferredWidthEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: double QGraphicsGridLayout::columnSpacing(int column);
  fn _ZNK19QGraphicsGridLayout13columnSpacingEi(arg0: c_int) -> i32;
  // proto: void QGraphicsGridLayout::NewQGraphicsGridLayout(const QGraphicsGridLayout & );
  fn _ZN19QGraphicsGridLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
  fn _ZN19QGraphicsGridLayout13setRowSpacingEid(arg0: c_int, arg1: c_double) -> i32;
  // proto: void QGraphicsGridLayout::removeAt(int index);
  fn _ZN19QGraphicsGridLayout8removeAtEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QGraphicsGridLayout)=1
pub struct QGraphicsGridLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowPreferredHeight<T: QGraphicsGridLayout_setRowPreferredHeight>(&mut self, value: T) -> i32 {
    value.setRowPreferredHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setRowPreferredHeight {
  fn setRowPreferredHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowPreferredHeight for (i32, f64) {
  fn setRowPreferredHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setRowPreferredHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout21setRowPreferredHeightEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnCount<T: QGraphicsGridLayout_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_columnCount {
  fn columnCount(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: int QGraphicsGridLayout::columnCount();
impl<'a> /*trait*/ QGraphicsGridLayout_columnCount for () {
  fn columnCount(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout11columnCountEv()};
    unsafe {_ZNK19QGraphicsGridLayout11columnCountEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn itemAt<T: QGraphicsGridLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_itemAt {
  fn itemAt(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn count<T: QGraphicsGridLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_count {
  fn count(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: int QGraphicsGridLayout::count();
impl<'a> /*trait*/ QGraphicsGridLayout_count for () {
  fn count(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout5countEv()};
    unsafe {_ZNK19QGraphicsGridLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnFixedWidth<T: QGraphicsGridLayout_setColumnFixedWidth>(&mut self, value: T) -> i32 {
    value.setColumnFixedWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnFixedWidth {
  fn setColumnFixedWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnFixedWidth for (i32, f64) {
  fn setColumnFixedWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setColumnFixedWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout19setColumnFixedWidthEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMaximumWidth<T: QGraphicsGridLayout_setColumnMaximumWidth>(&mut self, value: T) -> i32 {
    value.setColumnMaximumWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnMaximumWidth {
  fn setColumnMaximumWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMaximumWidth for (i32, f64) {
  fn setColumnMaximumWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowStretchFactor<T: QGraphicsGridLayout_rowStretchFactor>(&mut self, value: T) -> i32 {
    value.rowStretchFactor(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_rowStretchFactor {
  fn rowStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: int QGraphicsGridLayout::rowStretchFactor(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowStretchFactor for (i32) {
  fn rowStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowStretchFactorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout16rowStretchFactorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn verticalSpacing<T: QGraphicsGridLayout_verticalSpacing>(&mut self, value: T) -> i32 {
    value.verticalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_verticalSpacing {
  fn verticalSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGraphicsGridLayout_verticalSpacing for () {
  fn verticalSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout15verticalSpacingEv()};
    unsafe {_ZNK19QGraphicsGridLayout15verticalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnStretchFactor<T: QGraphicsGridLayout_columnStretchFactor>(&mut self, value: T) -> i32 {
    value.columnStretchFactor(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_columnStretchFactor {
  fn columnStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: int QGraphicsGridLayout::columnStretchFactor(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnStretchFactor for (i32) {
  fn columnStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout19columnStretchFactorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout19columnStretchFactorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMaximumHeight<T: QGraphicsGridLayout_setRowMaximumHeight>(&mut self, value: T) -> i32 {
    value.setRowMaximumHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setRowMaximumHeight {
  fn setRowMaximumHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMaximumHeight for (i32, f64) {
  fn setRowMaximumHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowMaximumHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout19setRowMaximumHeightEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn removeItem<T: QGraphicsGridLayout_removeItem>(&mut self, value: T) -> i32 {
    value.removeItem(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_removeItem {
  fn removeItem(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsGridLayout_removeItem for (&'a mut QGraphicsLayoutItem) {
  fn removeItem(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn FreeQGraphicsGridLayout<T: QGraphicsGridLayout_FreeQGraphicsGridLayout>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsGridLayout(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_FreeQGraphicsGridLayout {
  fn FreeQGraphicsGridLayout(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::FreeQGraphicsGridLayout();
impl<'a> /*trait*/ QGraphicsGridLayout_FreeQGraphicsGridLayout for () {
  fn FreeQGraphicsGridLayout(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutD0Ev()};
    unsafe {_ZN19QGraphicsGridLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMinimumHeight<T: QGraphicsGridLayout_rowMinimumHeight>(&mut self, value: T) -> i32 {
    value.rowMinimumHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_rowMinimumHeight {
  fn rowMinimumHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowMinimumHeight for (i32) {
  fn rowMinimumHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout16rowMinimumHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMaximumHeight<T: QGraphicsGridLayout_rowMaximumHeight>(&mut self, value: T) -> i32 {
    value.rowMaximumHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_rowMaximumHeight {
  fn rowMaximumHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::rowMaximumHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowMaximumHeight for (i32) {
  fn rowMaximumHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowMaximumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout16rowMaximumHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn NewQGraphicsGridLayout<T: QGraphicsGridLayout_NewQGraphicsGridLayout>(value: T) -> QGraphicsGridLayout {
    let rsthis = value.NewQGraphicsGridLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsGridLayout_NewQGraphicsGridLayout {
  fn NewQGraphicsGridLayout(self) -> QGraphicsGridLayout;
}

// proto: void QGraphicsGridLayout::NewQGraphicsGridLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsGridLayout_NewQGraphicsGridLayout for (&'a mut QGraphicsLayoutItem) {
  fn NewQGraphicsGridLayout(self) -> QGraphicsGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnSpacing<T: QGraphicsGridLayout_setColumnSpacing>(&mut self, value: T) -> i32 {
    value.setColumnSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnSpacing {
  fn setColumnSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnSpacing for (i32, f64) {
  fn setColumnSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout16setColumnSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout16setColumnSpacingEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowSpacing<T: QGraphicsGridLayout_rowSpacing>(&mut self, value: T) -> i32 {
    value.rowSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_rowSpacing {
  fn rowSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::rowSpacing(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowSpacing for (i32) {
  fn rowSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout10rowSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout10rowSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMaximumWidth<T: QGraphicsGridLayout_columnMaximumWidth>(&mut self, value: T) -> i32 {
    value.columnMaximumWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_columnMaximumWidth {
  fn columnMaximumWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::columnMaximumWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnMaximumWidth for (i32) {
  fn columnMaximumWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18columnMaximumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout18columnMaximumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowFixedHeight<T: QGraphicsGridLayout_setRowFixedHeight>(&mut self, value: T) -> i32 {
    value.setRowFixedHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setRowFixedHeight {
  fn setRowFixedHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowFixedHeight for (i32, f64) {
  fn setRowFixedHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout17setRowFixedHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout17setRowFixedHeightEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowPreferredHeight<T: QGraphicsGridLayout_rowPreferredHeight>(&mut self, value: T) -> i32 {
    value.rowPreferredHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_rowPreferredHeight {
  fn rowPreferredHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::rowPreferredHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowPreferredHeight for (i32) {
  fn rowPreferredHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18rowPreferredHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout18rowPreferredHeightEi(arg0)};
    return 1;
  }
}

// proto: QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int row, int column);
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt for (i32, i32) {
  fn itemAt(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout6itemAtEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setVerticalSpacing<T: QGraphicsGridLayout_setVerticalSpacing>(&mut self, value: T) -> i32 {
    value.setVerticalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setVerticalSpacing for (f64) {
  fn setVerticalSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout18setVerticalSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsGridLayout18setVerticalSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setGeometry<T: QGraphicsGridLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setGeometry {
  fn setGeometry(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsGridLayout_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsGridLayout11setGeometryERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowCount<T: QGraphicsGridLayout_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_rowCount {
  fn rowCount(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: int QGraphicsGridLayout::rowCount();
impl<'a> /*trait*/ QGraphicsGridLayout_rowCount for () {
  fn rowCount(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout8rowCountEv()};
    unsafe {_ZNK19QGraphicsGridLayout8rowCountEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setSpacing<T: QGraphicsGridLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setSpacing {
  fn setSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setSpacing for (f64) {
  fn setSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10setSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsGridLayout10setSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowStretchFactor<T: QGraphicsGridLayout_setRowStretchFactor>(&mut self, value: T) -> i32 {
    value.setRowStretchFactor(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setRowStretchFactor {
  fn setRowStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowStretchFactor for (i32, i32) {
  fn setRowStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN19QGraphicsGridLayout19setRowStretchFactorEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMinimumWidth<T: QGraphicsGridLayout_columnMinimumWidth>(&mut self, value: T) -> i32 {
    value.columnMinimumWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_columnMinimumWidth {
  fn columnMinimumWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnMinimumWidth for (i32) {
  fn columnMinimumWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout18columnMinimumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMinimumWidth<T: QGraphicsGridLayout_setColumnMinimumWidth>(&mut self, value: T) -> i32 {
    value.setColumnMinimumWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnMinimumWidth {
  fn setColumnMinimumWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMinimumWidth for (i32, f64) {
  fn setColumnMinimumWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMinimumHeight<T: QGraphicsGridLayout_setRowMinimumHeight>(&mut self, value: T) -> i32 {
    value.setRowMinimumHeight(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setRowMinimumHeight {
  fn setRowMinimumHeight(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMinimumHeight for (i32, f64) {
  fn setRowMinimumHeight(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowMinimumHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout19setRowMinimumHeightEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setHorizontalSpacing<T: QGraphicsGridLayout_setHorizontalSpacing>(&mut self, value: T) -> i32 {
    value.setHorizontalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setHorizontalSpacing for (f64) {
  fn setHorizontalSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout20setHorizontalSpacingEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN19QGraphicsGridLayout20setHorizontalSpacingEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn horizontalSpacing<T: QGraphicsGridLayout_horizontalSpacing>(&mut self, value: T) -> i32 {
    value.horizontalSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_horizontalSpacing {
  fn horizontalSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGraphicsGridLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout17horizontalSpacingEv()};
    unsafe {_ZNK19QGraphicsGridLayout17horizontalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnStretchFactor<T: QGraphicsGridLayout_setColumnStretchFactor>(&mut self, value: T) -> i32 {
    value.setColumnStretchFactor(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnStretchFactor {
  fn setColumnStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnStretchFactor for (i32, i32) {
  fn setColumnStretchFactor(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout22setColumnStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN19QGraphicsGridLayout22setColumnStretchFactorEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn invalidate<T: QGraphicsGridLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_invalidate {
  fn invalidate(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::invalidate();
impl<'a> /*trait*/ QGraphicsGridLayout_invalidate for () {
  fn invalidate(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10invalidateEv()};
    unsafe {_ZN19QGraphicsGridLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnPreferredWidth<T: QGraphicsGridLayout_columnPreferredWidth>(&mut self, value: T) -> i32 {
    value.columnPreferredWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_columnPreferredWidth {
  fn columnPreferredWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::columnPreferredWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnPreferredWidth for (i32) {
  fn columnPreferredWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout20columnPreferredWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout20columnPreferredWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnPreferredWidth<T: QGraphicsGridLayout_setColumnPreferredWidth>(&mut self, value: T) -> i32 {
    value.setColumnPreferredWidth(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnPreferredWidth {
  fn setColumnPreferredWidth(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnPreferredWidth for (i32, f64) {
  fn setColumnPreferredWidth(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnSpacing<T: QGraphicsGridLayout_columnSpacing>(&mut self, value: T) -> i32 {
    value.columnSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_columnSpacing {
  fn columnSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: double QGraphicsGridLayout::columnSpacing(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnSpacing for (i32) {
  fn columnSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout13columnSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK19QGraphicsGridLayout13columnSpacingEi(arg0)};
    return 1;
  }
}

// proto: void QGraphicsGridLayout::NewQGraphicsGridLayout(const QGraphicsGridLayout & );
impl<'a> /*trait*/ QGraphicsGridLayout_NewQGraphicsGridLayout for (&'a  QGraphicsGridLayout) {
  fn NewQGraphicsGridLayout(self) -> QGraphicsGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QGraphicsGridLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowSpacing<T: QGraphicsGridLayout_setRowSpacing>(&mut self, value: T) -> i32 {
    value.setRowSpacing(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_setRowSpacing {
  fn setRowSpacing(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowSpacing for (i32, f64) {
  fn setRowSpacing(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout13setRowSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    unsafe {_ZN19QGraphicsGridLayout13setRowSpacingEid(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn removeAt<T: QGraphicsGridLayout_removeAt>(&mut self, value: T) -> i32 {
    value.removeAt(self);
    return 1;
  }
}

pub trait QGraphicsGridLayout_removeAt {
  fn removeAt(self, this: &mut QGraphicsGridLayout) -> i32;
}

// proto: void QGraphicsGridLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsGridLayout_removeAt for (i32) {
  fn removeAt(self, this: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout8removeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QGraphicsGridLayout8removeAtEi(arg0)};
    return 1;
  }
}

