// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGridLayout::setRowMinimumHeight(int row, int minSize);
  fn _ZN11QGridLayout19setRowMinimumHeightEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QLayoutItem * QGridLayout::takeAt(int index);
  fn _ZN11QGridLayout6takeAtEi(arg0: c_int) -> i32;
  // proto: void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
  fn _ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_(arg0: c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int) -> i32;
  // proto: int QGridLayout::minimumHeightForWidth(int );
  fn _ZNK11QGridLayout21minimumHeightForWidthEi(arg0: c_int) -> i32;
  // proto: int QGridLayout::rowMinimumHeight(int row);
  fn _ZNK11QGridLayout16rowMinimumHeightEi(arg0: c_int) -> i32;
  // proto: void QGridLayout::invalidate();
  fn _ZN11QGridLayout10invalidateEv() -> i32;
  // proto: int QGridLayout::count();
  fn _ZNK11QGridLayout5countEv() -> i32;
  // proto: void QGridLayout::setColumnStretch(int column, int stretch);
  fn _ZN11QGridLayout16setColumnStretchEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: int QGridLayout::spacing();
  fn _ZNK11QGridLayout7spacingEv() -> i32;
  // proto: int QGridLayout::rowStretch(int row);
  fn _ZNK11QGridLayout10rowStretchEi(arg0: c_int) -> i32;
  // proto: QSize QGridLayout::sizeHint();
  fn _ZNK11QGridLayout8sizeHintEv() -> i32;
  // proto: int QGridLayout::rowCount();
  fn _ZNK11QGridLayout8rowCountEv() -> i32;
  // proto: void QGridLayout::setGeometry(const QRect & );
  fn _ZN11QGridLayout11setGeometryERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QGridLayout::setVerticalSpacing(int spacing);
  fn _ZN11QGridLayout18setVerticalSpacingEi(arg0: c_int) -> i32;
  // proto: void QGridLayout::setHorizontalSpacing(int spacing);
  fn _ZN11QGridLayout20setHorizontalSpacingEi(arg0: c_int) -> i32;
  // proto: int QGridLayout::columnStretch(int column);
  fn _ZNK11QGridLayout13columnStretchEi(arg0: c_int) -> i32;
  // proto: void QGridLayout::NewQGridLayout(const QGridLayout & );
  fn _ZN11QGridLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QGridLayout::columnCount();
  fn _ZNK11QGridLayout11columnCountEv() -> i32;
  // proto: int QGridLayout::columnMinimumWidth(int column);
  fn _ZNK11QGridLayout18columnMinimumWidthEi(arg0: c_int) -> i32;
  // proto: QSize QGridLayout::minimumSize();
  fn _ZNK11QGridLayout11minimumSizeEv() -> i32;
  // proto: bool QGridLayout::hasHeightForWidth();
  fn _ZNK11QGridLayout17hasHeightForWidthEv() -> i32;
  // proto: QRect QGridLayout::cellRect(int row, int column);
  fn _ZNK11QGridLayout8cellRectEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGridLayout::setRowStretch(int row, int stretch);
  fn _ZN11QGridLayout13setRowStretchEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
  fn _ZNK11QGridLayout14itemAtPositionEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QMetaObject * QGridLayout::metaObject();
  fn _ZNK11QGridLayout10metaObjectEv() -> i32;
  // proto: int QGridLayout::verticalSpacing();
  fn _ZNK11QGridLayout15verticalSpacingEv() -> i32;
  // proto: void QGridLayout::NewQGridLayout(QWidget * parent);
  fn _ZN11QGridLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: int QGridLayout::horizontalSpacing();
  fn _ZNK11QGridLayout17horizontalSpacingEv() -> i32;
  // proto: void QGridLayout::setColumnMinimumWidth(int column, int minSize);
  fn _ZN11QGridLayout21setColumnMinimumWidthEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: void QGridLayout::NewQGridLayout();
  fn _ZN11QGridLayoutC1Ev(qthis: *mut c_void) -> i32;
  // proto: int QGridLayout::heightForWidth(int );
  fn _ZNK11QGridLayout14heightForWidthEi(arg0: c_int) -> i32;
  // proto: void QGridLayout::FreeQGridLayout();
  fn _ZN11QGridLayoutD0Ev() -> i32;
  // proto: void QGridLayout::setSpacing(int spacing);
  fn _ZN11QGridLayout10setSpacingEi(arg0: c_int) -> i32;
  // proto: void QGridLayout::addWidget(QWidget * w);
  fn _ZN11QGridLayout9addWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QLayoutItem * QGridLayout::itemAt(int index);
  fn _ZNK11QGridLayout6itemAtEi(arg0: c_int) -> i32;
  // proto: QSize QGridLayout::maximumSize();
  fn _ZNK11QGridLayout11maximumSizeEv() -> i32;
}

// body block begin
// class sizeof(QGridLayout)=1
pub struct QGridLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGridLayout {
  pub fn setRowMinimumHeight<T: QGridLayout_setRowMinimumHeight>(&mut self, value: T) -> i32 {
    value.setRowMinimumHeight(self);
    return 1;
  }
}

pub trait QGridLayout_setRowMinimumHeight {
  fn setRowMinimumHeight(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setRowMinimumHeight(int row, int minSize);
impl<'a> /*trait*/ QGridLayout_setRowMinimumHeight for (i32, i32) {
  fn setRowMinimumHeight(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout19setRowMinimumHeightEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QGridLayout19setRowMinimumHeightEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn takeAt<T: QGridLayout_takeAt>(&mut self, value: T) -> i32 {
    value.takeAt(self);
    return 1;
  }
}

pub trait QGridLayout_takeAt {
  fn takeAt(self, this: &mut QGridLayout) -> i32;
}

// proto: QLayoutItem * QGridLayout::takeAt(int index);
impl<'a> /*trait*/ QGridLayout_takeAt for (i32) {
  fn takeAt(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout6takeAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QGridLayout6takeAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn getItemPosition<T: QGridLayout_getItemPosition>(&mut self, value: T) -> i32 {
    value.getItemPosition(self);
    return 1;
  }
}

pub trait QGridLayout_getItemPosition {
  fn getItemPosition(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
impl<'a> /*trait*/ QGridLayout_getItemPosition for (i32, &'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getItemPosition(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
    unsafe {_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn minimumHeightForWidth<T: QGridLayout_minimumHeightForWidth>(&mut self, value: T) -> i32 {
    value.minimumHeightForWidth(self);
    return 1;
  }
}

pub trait QGridLayout_minimumHeightForWidth {
  fn minimumHeightForWidth(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::minimumHeightForWidth(int );
impl<'a> /*trait*/ QGridLayout_minimumHeightForWidth for (i32) {
  fn minimumHeightForWidth(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout21minimumHeightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn rowMinimumHeight<T: QGridLayout_rowMinimumHeight>(&mut self, value: T) -> i32 {
    value.rowMinimumHeight(self);
    return 1;
  }
}

pub trait QGridLayout_rowMinimumHeight {
  fn rowMinimumHeight(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGridLayout_rowMinimumHeight for (i32) {
  fn rowMinimumHeight(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout16rowMinimumHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn invalidate<T: QGridLayout_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QGridLayout_invalidate {
  fn invalidate(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::invalidate();
impl<'a> /*trait*/ QGridLayout_invalidate for () {
  fn invalidate(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10invalidateEv()};
    unsafe {_ZN11QGridLayout10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn count<T: QGridLayout_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QGridLayout_count {
  fn count(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::count();
impl<'a> /*trait*/ QGridLayout_count for () {
  fn count(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout5countEv()};
    unsafe {_ZNK11QGridLayout5countEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setColumnStretch<T: QGridLayout_setColumnStretch>(&mut self, value: T) -> i32 {
    value.setColumnStretch(self);
    return 1;
  }
}

pub trait QGridLayout_setColumnStretch {
  fn setColumnStretch(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setColumnStretch(int column, int stretch);
impl<'a> /*trait*/ QGridLayout_setColumnStretch for (i32, i32) {
  fn setColumnStretch(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout16setColumnStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QGridLayout16setColumnStretchEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn spacing<T: QGridLayout_spacing>(&mut self, value: T) -> i32 {
    value.spacing(self);
    return 1;
  }
}

pub trait QGridLayout_spacing {
  fn spacing(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::spacing();
impl<'a> /*trait*/ QGridLayout_spacing for () {
  fn spacing(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout7spacingEv()};
    unsafe {_ZNK11QGridLayout7spacingEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn rowStretch<T: QGridLayout_rowStretch>(&mut self, value: T) -> i32 {
    value.rowStretch(self);
    return 1;
  }
}

pub trait QGridLayout_rowStretch {
  fn rowStretch(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::rowStretch(int row);
impl<'a> /*trait*/ QGridLayout_rowStretch for (i32) {
  fn rowStretch(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10rowStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout10rowStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn sizeHint<T: QGridLayout_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QGridLayout_sizeHint {
  fn sizeHint(self, this: &mut QGridLayout) -> i32;
}

// proto: QSize QGridLayout::sizeHint();
impl<'a> /*trait*/ QGridLayout_sizeHint for () {
  fn sizeHint(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8sizeHintEv()};
    unsafe {_ZNK11QGridLayout8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn rowCount<T: QGridLayout_rowCount>(&mut self, value: T) -> i32 {
    value.rowCount(self);
    return 1;
  }
}

pub trait QGridLayout_rowCount {
  fn rowCount(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::rowCount();
impl<'a> /*trait*/ QGridLayout_rowCount for () {
  fn rowCount(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8rowCountEv()};
    unsafe {_ZNK11QGridLayout8rowCountEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setGeometry<T: QGridLayout_setGeometry>(&mut self, value: T) -> i32 {
    value.setGeometry(self);
    return 1;
  }
}

pub trait QGridLayout_setGeometry {
  fn setGeometry(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QGridLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QGridLayout11setGeometryERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setVerticalSpacing<T: QGridLayout_setVerticalSpacing>(&mut self, value: T) -> i32 {
    value.setVerticalSpacing(self);
    return 1;
  }
}

pub trait QGridLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setVerticalSpacing for (i32) {
  fn setVerticalSpacing(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QGridLayout18setVerticalSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setHorizontalSpacing<T: QGridLayout_setHorizontalSpacing>(&mut self, value: T) -> i32 {
    value.setHorizontalSpacing(self);
    return 1;
  }
}

pub trait QGridLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setHorizontalSpacing for (i32) {
  fn setHorizontalSpacing(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QGridLayout20setHorizontalSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn columnStretch<T: QGridLayout_columnStretch>(&mut self, value: T) -> i32 {
    value.columnStretch(self);
    return 1;
  }
}

pub trait QGridLayout_columnStretch {
  fn columnStretch(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::columnStretch(int column);
impl<'a> /*trait*/ QGridLayout_columnStretch for (i32) {
  fn columnStretch(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout13columnStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout13columnStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn NewQGridLayout<T: QGridLayout_NewQGridLayout>(value: T) -> QGridLayout {
    let rsthis = value.NewQGridLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGridLayout_NewQGridLayout {
  fn NewQGridLayout(self) -> QGridLayout;
}

// proto: void QGridLayout::NewQGridLayout(const QGridLayout & );
impl<'a> /*trait*/ QGridLayout_NewQGridLayout for (&'a  QGridLayout) {
  fn NewQGridLayout(self) -> QGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QGridLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn columnCount<T: QGridLayout_columnCount>(&mut self, value: T) -> i32 {
    value.columnCount(self);
    return 1;
  }
}

pub trait QGridLayout_columnCount {
  fn columnCount(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::columnCount();
impl<'a> /*trait*/ QGridLayout_columnCount for () {
  fn columnCount(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11columnCountEv()};
    unsafe {_ZNK11QGridLayout11columnCountEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn columnMinimumWidth<T: QGridLayout_columnMinimumWidth>(&mut self, value: T) -> i32 {
    value.columnMinimumWidth(self);
    return 1;
  }
}

pub trait QGridLayout_columnMinimumWidth {
  fn columnMinimumWidth(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGridLayout_columnMinimumWidth for (i32) {
  fn columnMinimumWidth(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout18columnMinimumWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn minimumSize<T: QGridLayout_minimumSize>(&mut self, value: T) -> i32 {
    value.minimumSize(self);
    return 1;
  }
}

pub trait QGridLayout_minimumSize {
  fn minimumSize(self, this: &mut QGridLayout) -> i32;
}

// proto: QSize QGridLayout::minimumSize();
impl<'a> /*trait*/ QGridLayout_minimumSize for () {
  fn minimumSize(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11minimumSizeEv()};
    unsafe {_ZNK11QGridLayout11minimumSizeEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn hasHeightForWidth<T: QGridLayout_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QGridLayout_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QGridLayout) -> i32;
}

// proto: bool QGridLayout::hasHeightForWidth();
impl<'a> /*trait*/ QGridLayout_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17hasHeightForWidthEv()};
    unsafe {_ZNK11QGridLayout17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn cellRect<T: QGridLayout_cellRect>(&mut self, value: T) -> i32 {
    value.cellRect(self);
    return 1;
  }
}

pub trait QGridLayout_cellRect {
  fn cellRect(self, this: &mut QGridLayout) -> i32;
}

// proto: QRect QGridLayout::cellRect(int row, int column);
impl<'a> /*trait*/ QGridLayout_cellRect for (i32, i32) {
  fn cellRect(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8cellRectEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QGridLayout8cellRectEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setRowStretch<T: QGridLayout_setRowStretch>(&mut self, value: T) -> i32 {
    value.setRowStretch(self);
    return 1;
  }
}

pub trait QGridLayout_setRowStretch {
  fn setRowStretch(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setRowStretch(int row, int stretch);
impl<'a> /*trait*/ QGridLayout_setRowStretch for (i32, i32) {
  fn setRowStretch(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout13setRowStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QGridLayout13setRowStretchEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn itemAtPosition<T: QGridLayout_itemAtPosition>(&mut self, value: T) -> i32 {
    value.itemAtPosition(self);
    return 1;
  }
}

pub trait QGridLayout_itemAtPosition {
  fn itemAtPosition(self, this: &mut QGridLayout) -> i32;
}

// proto: QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
impl<'a> /*trait*/ QGridLayout_itemAtPosition for (i32, i32) {
  fn itemAtPosition(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14itemAtPositionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK11QGridLayout14itemAtPositionEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn metaObject<T: QGridLayout_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGridLayout_metaObject {
  fn metaObject(self, this: &mut QGridLayout) -> i32;
}

// proto: const QMetaObject * QGridLayout::metaObject();
impl<'a> /*trait*/ QGridLayout_metaObject for () {
  fn metaObject(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10metaObjectEv()};
    unsafe {_ZNK11QGridLayout10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn verticalSpacing<T: QGridLayout_verticalSpacing>(&mut self, value: T) -> i32 {
    value.verticalSpacing(self);
    return 1;
  }
}

pub trait QGridLayout_verticalSpacing {
  fn verticalSpacing(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGridLayout_verticalSpacing for () {
  fn verticalSpacing(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout15verticalSpacingEv()};
    unsafe {_ZNK11QGridLayout15verticalSpacingEv()};
    return 1;
  }
}

// proto: void QGridLayout::NewQGridLayout(QWidget * parent);
impl<'a> /*trait*/ QGridLayout_NewQGridLayout for (&'a mut QWidget) {
  fn NewQGridLayout(self) -> QGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QGridLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn horizontalSpacing<T: QGridLayout_horizontalSpacing>(&mut self, value: T) -> i32 {
    value.horizontalSpacing(self);
    return 1;
  }
}

pub trait QGridLayout_horizontalSpacing {
  fn horizontalSpacing(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGridLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17horizontalSpacingEv()};
    unsafe {_ZNK11QGridLayout17horizontalSpacingEv()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setColumnMinimumWidth<T: QGridLayout_setColumnMinimumWidth>(&mut self, value: T) -> i32 {
    value.setColumnMinimumWidth(self);
    return 1;
  }
}

pub trait QGridLayout_setColumnMinimumWidth {
  fn setColumnMinimumWidth(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setColumnMinimumWidth(int column, int minSize);
impl<'a> /*trait*/ QGridLayout_setColumnMinimumWidth for (i32, i32) {
  fn setColumnMinimumWidth(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout21setColumnMinimumWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN11QGridLayout21setColumnMinimumWidthEii(arg0, arg1)};
    return 1;
  }
}

// proto: void QGridLayout::NewQGridLayout();
impl<'a> /*trait*/ QGridLayout_NewQGridLayout for () {
  fn NewQGridLayout(self) -> QGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutC1Ev()};
    unsafe {_ZN11QGridLayoutC1Ev(qthis)};
    let rsthis = QGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn heightForWidth<T: QGridLayout_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QGridLayout_heightForWidth {
  fn heightForWidth(self, this: &mut QGridLayout) -> i32;
}

// proto: int QGridLayout::heightForWidth(int );
impl<'a> /*trait*/ QGridLayout_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn FreeQGridLayout<T: QGridLayout_FreeQGridLayout>(&mut self, value: T) -> i32 {
    value.FreeQGridLayout(self);
    return 1;
  }
}

pub trait QGridLayout_FreeQGridLayout {
  fn FreeQGridLayout(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::FreeQGridLayout();
impl<'a> /*trait*/ QGridLayout_FreeQGridLayout for () {
  fn FreeQGridLayout(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutD0Ev()};
    unsafe {_ZN11QGridLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setSpacing<T: QGridLayout_setSpacing>(&mut self, value: T) -> i32 {
    value.setSpacing(self);
    return 1;
  }
}

pub trait QGridLayout_setSpacing {
  fn setSpacing(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::setSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setSpacing for (i32) {
  fn setSpacing(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10setSpacingEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QGridLayout10setSpacingEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn addWidget<T: QGridLayout_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QGridLayout_addWidget {
  fn addWidget(self, this: &mut QGridLayout) -> i32;
}

// proto: void QGridLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QGridLayout_addWidget for (&'a mut QWidget) {
  fn addWidget(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QGridLayout9addWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn itemAt<T: QGridLayout_itemAt>(&mut self, value: T) -> i32 {
    value.itemAt(self);
    return 1;
  }
}

pub trait QGridLayout_itemAt {
  fn itemAt(self, this: &mut QGridLayout) -> i32;
}

// proto: QLayoutItem * QGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGridLayout_itemAt for (i32) {
  fn itemAt(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QGridLayout6itemAtEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn maximumSize<T: QGridLayout_maximumSize>(&mut self, value: T) -> i32 {
    value.maximumSize(self);
    return 1;
  }
}

pub trait QGridLayout_maximumSize {
  fn maximumSize(self, this: &mut QGridLayout) -> i32;
}

// proto: QSize QGridLayout::maximumSize();
impl<'a> /*trait*/ QGridLayout_maximumSize for () {
  fn maximumSize(self, this: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11maximumSizeEv()};
    unsafe {_ZNK11QGridLayout11maximumSizeEv()};
    return 1;
  }
}

