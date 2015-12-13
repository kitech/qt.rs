// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qlayoutitem::QLayoutItem;
use super::qsize::QSize;
use super::qrect::QRect;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
  fn _ZN11QGridLayout19setRowMinimumHeightEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QLayoutItem * QGridLayout::takeAt(int index);
  fn _ZN11QGridLayout6takeAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
  fn _ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_(qthis: *mut c_void, arg0: c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int, arg4: *mut c_int) ;
  // proto:  int QGridLayout::minimumHeightForWidth(int );
  fn _ZNK11QGridLayout21minimumHeightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QGridLayout::rowMinimumHeight(int row);
  fn _ZNK11QGridLayout16rowMinimumHeightEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QGridLayout::invalidate();
  fn _ZN11QGridLayout10invalidateEv(qthis: *mut c_void) ;
  // proto:  int QGridLayout::count();
  fn _ZNK11QGridLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGridLayout::setColumnStretch(int column, int stretch);
  fn _ZN11QGridLayout16setColumnStretchEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QGridLayout::spacing();
  fn _ZNK11QGridLayout7spacingEv(qthis: *mut c_void) -> c_int;
  // proto:  int QGridLayout::rowStretch(int row);
  fn _ZNK11QGridLayout10rowStretchEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QSize QGridLayout::sizeHint();
  fn _ZNK11QGridLayout8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QGridLayout::rowCount();
  fn _ZNK11QGridLayout8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGridLayout::setGeometry(const QRect & );
  fn _ZN11QGridLayout11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGridLayout::setVerticalSpacing(int spacing);
  fn _ZN11QGridLayout18setVerticalSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGridLayout::setHorizontalSpacing(int spacing);
  fn _ZN11QGridLayout20setHorizontalSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QGridLayout::columnStretch(int column);
  fn _ZNK11QGridLayout13columnStretchEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QGridLayout::NewQGridLayout(const QGridLayout & );
  fn _ZN11QGridLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGridLayout::columnCount();
  fn _ZNK11QGridLayout11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  int QGridLayout::columnMinimumWidth(int column);
  fn _ZNK11QGridLayout18columnMinimumWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  QSize QGridLayout::minimumSize();
  fn _ZNK11QGridLayout11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGridLayout::hasHeightForWidth();
  fn _ZNK11QGridLayout17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QGridLayout::cellRect(int row, int column);
  fn _ZNK11QGridLayout8cellRectEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QGridLayout::setRowStretch(int row, int stretch);
  fn _ZN11QGridLayout13setRowStretchEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
  fn _ZNK11QGridLayout14itemAtPositionEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QGridLayout::metaObject();
  fn _ZNK11QGridLayout10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QGridLayout::verticalSpacing();
  fn _ZNK11QGridLayout15verticalSpacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGridLayout::NewQGridLayout(QWidget * parent);
  fn _ZN11QGridLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGridLayout::horizontalSpacing();
  fn _ZNK11QGridLayout17horizontalSpacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
  fn _ZN11QGridLayout21setColumnMinimumWidthEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QGridLayout::NewQGridLayout();
  fn _ZN11QGridLayoutC1Ev(qthis: *mut c_void) ;
  // proto:  int QGridLayout::heightForWidth(int );
  fn _ZNK11QGridLayout14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QGridLayout::FreeQGridLayout();
  fn _ZN11QGridLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  void QGridLayout::setSpacing(int spacing);
  fn _ZN11QGridLayout10setSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGridLayout::addWidget(QWidget * w);
  fn _ZN11QGridLayout9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QLayoutItem * QGridLayout::itemAt(int index);
  fn _ZNK11QGridLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QSize QGridLayout::maximumSize();
  fn _ZNK11QGridLayout11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QGridLayout)=1
pub struct QGridLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGridLayout {
  pub fn setRowMinimumHeight<T: QGridLayout_setRowMinimumHeight>(&mut self, value: T)  {
     value.setRowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGridLayout_setRowMinimumHeight {
  fn setRowMinimumHeight(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
impl<'a> /*trait*/ QGridLayout_setRowMinimumHeight for (i32, i32) {
  fn setRowMinimumHeight(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout19setRowMinimumHeightEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout19setRowMinimumHeightEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn takeAt<T: QGridLayout_takeAt>(&mut self, value: T) -> QLayoutItem {
    return value.takeAt(self);
    // return 1;
  }
}

pub trait QGridLayout_takeAt {
  fn takeAt(self, rsthis: &mut QGridLayout) -> QLayoutItem;
}

// proto:  QLayoutItem * QGridLayout::takeAt(int index);
impl<'a> /*trait*/ QGridLayout_takeAt for (i32) {
  fn takeAt(self, rsthis: &mut QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QGridLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn getItemPosition<T: QGridLayout_getItemPosition>(&mut self, value: T)  {
     value.getItemPosition(self);
    // return 1;
  }
}

pub trait QGridLayout_getItemPosition {
  fn getItemPosition(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
impl<'a> /*trait*/ QGridLayout_getItemPosition for (i32, &'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getItemPosition(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    let arg4 = self.4  as *mut c_int;
     unsafe {_ZNK11QGridLayout15getItemPositionEiPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn minimumHeightForWidth<T: QGridLayout_minimumHeightForWidth>(&mut self, value: T) -> i32 {
    return value.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_minimumHeightForWidth {
  fn minimumHeightForWidth(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::minimumHeightForWidth(int );
impl<'a> /*trait*/ QGridLayout_minimumHeightForWidth for (i32) {
  fn minimumHeightForWidth(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn rowMinimumHeight<T: QGridLayout_rowMinimumHeight>(&mut self, value: T) -> i32 {
    return value.rowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGridLayout_rowMinimumHeight {
  fn rowMinimumHeight(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGridLayout_rowMinimumHeight for (i32) {
  fn rowMinimumHeight(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout16rowMinimumHeightEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn invalidate<T: QGridLayout_invalidate>(&mut self, value: T)  {
     value.invalidate(self);
    // return 1;
  }
}

pub trait QGridLayout_invalidate {
  fn invalidate(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::invalidate();
impl<'a> /*trait*/ QGridLayout_invalidate for () {
  fn invalidate(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10invalidateEv()};
     unsafe {_ZN11QGridLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn count<T: QGridLayout_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QGridLayout_count {
  fn count(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::count();
impl<'a> /*trait*/ QGridLayout_count for () {
  fn count(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout5countEv()};
    let mut ret = unsafe {_ZNK11QGridLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setColumnStretch<T: QGridLayout_setColumnStretch>(&mut self, value: T)  {
     value.setColumnStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_setColumnStretch {
  fn setColumnStretch(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setColumnStretch(int column, int stretch);
impl<'a> /*trait*/ QGridLayout_setColumnStretch for (i32, i32) {
  fn setColumnStretch(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout16setColumnStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout16setColumnStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn spacing<T: QGridLayout_spacing>(&mut self, value: T) -> i32 {
    return value.spacing(self);
    // return 1;
  }
}

pub trait QGridLayout_spacing {
  fn spacing(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::spacing();
impl<'a> /*trait*/ QGridLayout_spacing for () {
  fn spacing(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout7spacingEv()};
    let mut ret = unsafe {_ZNK11QGridLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn rowStretch<T: QGridLayout_rowStretch>(&mut self, value: T) -> i32 {
    return value.rowStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_rowStretch {
  fn rowStretch(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::rowStretch(int row);
impl<'a> /*trait*/ QGridLayout_rowStretch for (i32) {
  fn rowStretch(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10rowStretchEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout10rowStretchEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn sizeHint<T: QGridLayout_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QGridLayout_sizeHint {
  fn sizeHint(self, rsthis: &mut QGridLayout) -> QSize;
}

// proto:  QSize QGridLayout::sizeHint();
impl<'a> /*trait*/ QGridLayout_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QGridLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn rowCount<T: QGridLayout_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QGridLayout_rowCount {
  fn rowCount(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::rowCount();
impl<'a> /*trait*/ QGridLayout_rowCount for () {
  fn rowCount(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8rowCountEv()};
    let mut ret = unsafe {_ZNK11QGridLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setGeometry<T: QGridLayout_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QGridLayout_setGeometry {
  fn setGeometry(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QGridLayout_setGeometry for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QGridLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setVerticalSpacing<T: QGridLayout_setVerticalSpacing>(&mut self, value: T)  {
     value.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setVerticalSpacing for (i32) {
  fn setVerticalSpacing(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QGridLayout18setVerticalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setHorizontalSpacing<T: QGridLayout_setHorizontalSpacing>(&mut self, value: T)  {
     value.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setHorizontalSpacing for (i32) {
  fn setHorizontalSpacing(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QGridLayout20setHorizontalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn columnStretch<T: QGridLayout_columnStretch>(&mut self, value: T) -> i32 {
    return value.columnStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_columnStretch {
  fn columnStretch(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::columnStretch(int column);
impl<'a> /*trait*/ QGridLayout_columnStretch for (i32) {
  fn columnStretch(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout13columnStretchEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout13columnStretchEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QGridLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn columnCount<T: QGridLayout_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QGridLayout_columnCount {
  fn columnCount(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::columnCount();
impl<'a> /*trait*/ QGridLayout_columnCount for () {
  fn columnCount(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11columnCountEv()};
    let mut ret = unsafe {_ZNK11QGridLayout11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn columnMinimumWidth<T: QGridLayout_columnMinimumWidth>(&mut self, value: T) -> i32 {
    return value.columnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_columnMinimumWidth {
  fn columnMinimumWidth(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGridLayout_columnMinimumWidth for (i32) {
  fn columnMinimumWidth(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout18columnMinimumWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn minimumSize<T: QGridLayout_minimumSize>(&mut self, value: T) -> QSize {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QGridLayout_minimumSize {
  fn minimumSize(self, rsthis: &mut QGridLayout) -> QSize;
}

// proto:  QSize QGridLayout::minimumSize();
impl<'a> /*trait*/ QGridLayout_minimumSize for () {
  fn minimumSize(self, rsthis: &mut QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QGridLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn hasHeightForWidth<T: QGridLayout_hasHeightForWidth>(&mut self, value: T) -> i8 {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_hasHeightForWidth {
  fn hasHeightForWidth(self, rsthis: &mut QGridLayout) -> i8;
}

// proto:  bool QGridLayout::hasHeightForWidth();
impl<'a> /*trait*/ QGridLayout_hasHeightForWidth for () {
  fn hasHeightForWidth(self, rsthis: &mut QGridLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QGridLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn cellRect<T: QGridLayout_cellRect>(&mut self, value: T) -> QRect {
    return value.cellRect(self);
    // return 1;
  }
}

pub trait QGridLayout_cellRect {
  fn cellRect(self, rsthis: &mut QGridLayout) -> QRect;
}

// proto:  QRect QGridLayout::cellRect(int row, int column);
impl<'a> /*trait*/ QGridLayout_cellRect for (i32, i32) {
  fn cellRect(self, rsthis: &mut QGridLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8cellRectEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout8cellRectEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setRowStretch<T: QGridLayout_setRowStretch>(&mut self, value: T)  {
     value.setRowStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_setRowStretch {
  fn setRowStretch(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setRowStretch(int row, int stretch);
impl<'a> /*trait*/ QGridLayout_setRowStretch for (i32, i32) {
  fn setRowStretch(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout13setRowStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout13setRowStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn itemAtPosition<T: QGridLayout_itemAtPosition>(&mut self, value: T) -> QLayoutItem {
    return value.itemAtPosition(self);
    // return 1;
  }
}

pub trait QGridLayout_itemAtPosition {
  fn itemAtPosition(self, rsthis: &mut QGridLayout) -> QLayoutItem;
}

// proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
impl<'a> /*trait*/ QGridLayout_itemAtPosition for (i32, i32) {
  fn itemAtPosition(self, rsthis: &mut QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14itemAtPositionEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout14itemAtPositionEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn metaObject<T: QGridLayout_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGridLayout_metaObject {
  fn metaObject(self, rsthis: &mut QGridLayout) ;
}

// proto:  const QMetaObject * QGridLayout::metaObject();
impl<'a> /*trait*/ QGridLayout_metaObject for () {
  fn metaObject(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10metaObjectEv()};
     unsafe {_ZNK11QGridLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn verticalSpacing<T: QGridLayout_verticalSpacing>(&mut self, value: T) -> i32 {
    return value.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_verticalSpacing {
  fn verticalSpacing(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGridLayout_verticalSpacing for () {
  fn verticalSpacing(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout15verticalSpacingEv()};
    let mut ret = unsafe {_ZNK11QGridLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
    return value.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_horizontalSpacing {
  fn horizontalSpacing(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGridLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK11QGridLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setColumnMinimumWidth<T: QGridLayout_setColumnMinimumWidth>(&mut self, value: T)  {
     value.setColumnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_setColumnMinimumWidth {
  fn setColumnMinimumWidth(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
impl<'a> /*trait*/ QGridLayout_setColumnMinimumWidth for (i32, i32) {
  fn setColumnMinimumWidth(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout21setColumnMinimumWidthEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout21setColumnMinimumWidthEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_heightForWidth {
  fn heightForWidth(self, rsthis: &mut QGridLayout) -> i32;
}

// proto:  int QGridLayout::heightForWidth(int );
impl<'a> /*trait*/ QGridLayout_heightForWidth for (i32) {
  fn heightForWidth(self, rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn FreeQGridLayout<T: QGridLayout_FreeQGridLayout>(&mut self, value: T)  {
     value.FreeQGridLayout(self);
    // return 1;
  }
}

pub trait QGridLayout_FreeQGridLayout {
  fn FreeQGridLayout(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::FreeQGridLayout();
impl<'a> /*trait*/ QGridLayout_FreeQGridLayout for () {
  fn FreeQGridLayout(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutD0Ev()};
     unsafe {_ZN11QGridLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn setSpacing<T: QGridLayout_setSpacing>(&mut self, value: T)  {
     value.setSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setSpacing {
  fn setSpacing(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::setSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setSpacing for (i32) {
  fn setSpacing(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QGridLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn addWidget<T: QGridLayout_addWidget>(&mut self, value: T)  {
     value.addWidget(self);
    // return 1;
  }
}

pub trait QGridLayout_addWidget {
  fn addWidget(self, rsthis: &mut QGridLayout) ;
}

// proto:  void QGridLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QGridLayout_addWidget for (&'a mut QWidget) {
  fn addWidget(self, rsthis: &mut QGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QGridLayout9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn itemAt<T: QGridLayout_itemAt>(&mut self, value: T) -> QLayoutItem {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QGridLayout_itemAt {
  fn itemAt(self, rsthis: &mut QGridLayout) -> QLayoutItem;
}

// proto:  QLayoutItem * QGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGridLayout_itemAt for (i32) {
  fn itemAt(self, rsthis: &mut QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGridLayout {
  pub fn maximumSize<T: QGridLayout_maximumSize>(&mut self, value: T) -> QSize {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QGridLayout_maximumSize {
  fn maximumSize(self, rsthis: &mut QGridLayout) -> QSize;
}

// proto:  QSize QGridLayout::maximumSize();
impl<'a> /*trait*/ QGridLayout_maximumSize for () {
  fn maximumSize(self, rsthis: &mut QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QGridLayout11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

