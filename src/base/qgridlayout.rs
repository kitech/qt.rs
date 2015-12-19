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

// proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
impl /*struct*/ QGridLayout {
  pub fn setRowMinimumHeight<RetType, T: QGridLayout_setRowMinimumHeight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGridLayout_setRowMinimumHeight<RetType> {
  fn setRowMinimumHeight(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setRowMinimumHeight(int row, int minSize);
impl<'a> /*trait*/ QGridLayout_setRowMinimumHeight<()> for (i32, i32) {
  fn setRowMinimumHeight(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout19setRowMinimumHeightEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout19setRowMinimumHeightEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QLayoutItem * QGridLayout::takeAt(int index);
impl /*struct*/ QGridLayout {
  pub fn takeAt<RetType, T: QGridLayout_takeAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QGridLayout_takeAt<RetType> {
  fn takeAt(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QLayoutItem * QGridLayout::takeAt(int index);
impl<'a> /*trait*/ QGridLayout_takeAt<QLayoutItem> for (i32) {
  fn takeAt(self , rsthis: &mut QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN11QGridLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
impl /*struct*/ QGridLayout {
  pub fn getItemPosition<RetType, T: QGridLayout_getItemPosition<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.getItemPosition(self);
    // return 1;
  }
}

pub trait QGridLayout_getItemPosition<RetType> {
  fn getItemPosition(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::getItemPosition(int idx, int * row, int * column, int * rowSpan, int * columnSpan);
impl<'a> /*trait*/ QGridLayout_getItemPosition<()> for (i32, &'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getItemPosition(self , rsthis: &mut QGridLayout) -> () {
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

// proto:  int QGridLayout::minimumHeightForWidth(int );
impl /*struct*/ QGridLayout {
  pub fn minimumHeightForWidth<RetType, T: QGridLayout_minimumHeightForWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumHeightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_minimumHeightForWidth<RetType> {
  fn minimumHeightForWidth(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::minimumHeightForWidth(int );
impl<'a> /*trait*/ QGridLayout_minimumHeightForWidth<i32> for (i32) {
  fn minimumHeightForWidth(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout21minimumHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout21minimumHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QGridLayout::rowMinimumHeight(int row);
impl /*struct*/ QGridLayout {
  pub fn rowMinimumHeight<RetType, T: QGridLayout_rowMinimumHeight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGridLayout_rowMinimumHeight<RetType> {
  fn rowMinimumHeight(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGridLayout_rowMinimumHeight<i32> for (i32) {
  fn rowMinimumHeight(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout16rowMinimumHeightEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QGridLayout::invalidate();
impl /*struct*/ QGridLayout {
  pub fn invalidate<RetType, T: QGridLayout_invalidate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGridLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::invalidate();
impl<'a> /*trait*/ QGridLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10invalidateEv()};
     unsafe {_ZN11QGridLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QGridLayout::count();
impl /*struct*/ QGridLayout {
  pub fn count<RetType, T: QGridLayout_count<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGridLayout_count<RetType> {
  fn count(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::count();
impl<'a> /*trait*/ QGridLayout_count<i32> for () {
  fn count(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout5countEv()};
    let mut ret = unsafe {_ZNK11QGridLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QGridLayout::setColumnStretch(int column, int stretch);
impl /*struct*/ QGridLayout {
  pub fn setColumnStretch<RetType, T: QGridLayout_setColumnStretch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setColumnStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_setColumnStretch<RetType> {
  fn setColumnStretch(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setColumnStretch(int column, int stretch);
impl<'a> /*trait*/ QGridLayout_setColumnStretch<()> for (i32, i32) {
  fn setColumnStretch(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout16setColumnStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout16setColumnStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QGridLayout::spacing();
impl /*struct*/ QGridLayout {
  pub fn spacing<RetType, T: QGridLayout_spacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QGridLayout_spacing<RetType> {
  fn spacing(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::spacing();
impl<'a> /*trait*/ QGridLayout_spacing<i32> for () {
  fn spacing(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout7spacingEv()};
    let mut ret = unsafe {_ZNK11QGridLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QGridLayout::rowStretch(int row);
impl /*struct*/ QGridLayout {
  pub fn rowStretch<RetType, T: QGridLayout_rowStretch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_rowStretch<RetType> {
  fn rowStretch(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::rowStretch(int row);
impl<'a> /*trait*/ QGridLayout_rowStretch<i32> for (i32) {
  fn rowStretch(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10rowStretchEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout10rowStretchEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QSize QGridLayout::sizeHint();
impl /*struct*/ QGridLayout {
  pub fn sizeHint<RetType, T: QGridLayout_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QGridLayout_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QSize QGridLayout::sizeHint();
impl<'a> /*trait*/ QGridLayout_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QGridLayout8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  int QGridLayout::rowCount();
impl /*struct*/ QGridLayout {
  pub fn rowCount<RetType, T: QGridLayout_rowCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QGridLayout_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::rowCount();
impl<'a> /*trait*/ QGridLayout_rowCount<i32> for () {
  fn rowCount(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout8rowCountEv()};
    let mut ret = unsafe {_ZNK11QGridLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QGridLayout::setGeometry(const QRect & );
impl /*struct*/ QGridLayout {
  pub fn setGeometry<RetType, T: QGridLayout_setGeometry<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGridLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QGridLayout_setGeometry<()> for (&'a  QRect) {
  fn setGeometry(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QGridLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGridLayout::setVerticalSpacing(int spacing);
impl /*struct*/ QGridLayout {
  pub fn setVerticalSpacing<RetType, T: QGridLayout_setVerticalSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setVerticalSpacing<RetType> {
  fn setVerticalSpacing(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setVerticalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setVerticalSpacing<()> for (i32) {
  fn setVerticalSpacing(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout18setVerticalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QGridLayout18setVerticalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGridLayout::setHorizontalSpacing(int spacing);
impl /*struct*/ QGridLayout {
  pub fn setHorizontalSpacing<RetType, T: QGridLayout_setHorizontalSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setHorizontalSpacing<RetType> {
  fn setHorizontalSpacing(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setHorizontalSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setHorizontalSpacing<()> for (i32) {
  fn setHorizontalSpacing(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout20setHorizontalSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QGridLayout20setHorizontalSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QGridLayout::columnStretch(int column);
impl /*struct*/ QGridLayout {
  pub fn columnStretch<RetType, T: QGridLayout_columnStretch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_columnStretch<RetType> {
  fn columnStretch(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::columnStretch(int column);
impl<'a> /*trait*/ QGridLayout_columnStretch<i32> for (i32) {
  fn columnStretch(self , rsthis: &mut QGridLayout) -> i32 {
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

// proto:  int QGridLayout::columnCount();
impl /*struct*/ QGridLayout {
  pub fn columnCount<RetType, T: QGridLayout_columnCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QGridLayout_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::columnCount();
impl<'a> /*trait*/ QGridLayout_columnCount<i32> for () {
  fn columnCount(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11columnCountEv()};
    let mut ret = unsafe {_ZNK11QGridLayout11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QGridLayout::columnMinimumWidth(int column);
impl /*struct*/ QGridLayout {
  pub fn columnMinimumWidth<RetType, T: QGridLayout_columnMinimumWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.columnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_columnMinimumWidth<RetType> {
  fn columnMinimumWidth(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGridLayout_columnMinimumWidth<i32> for (i32) {
  fn columnMinimumWidth(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout18columnMinimumWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QSize QGridLayout::minimumSize();
impl /*struct*/ QGridLayout {
  pub fn minimumSize<RetType, T: QGridLayout_minimumSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QGridLayout_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QSize QGridLayout::minimumSize();
impl<'a> /*trait*/ QGridLayout_minimumSize<QSize> for () {
  fn minimumSize(self , rsthis: &mut QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK11QGridLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QGridLayout::hasHeightForWidth();
impl /*struct*/ QGridLayout {
  pub fn hasHeightForWidth<RetType, T: QGridLayout_hasHeightForWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  bool QGridLayout::hasHeightForWidth();
impl<'a> /*trait*/ QGridLayout_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QGridLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QGridLayout17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QRect QGridLayout::cellRect(int row, int column);
impl /*struct*/ QGridLayout {
  pub fn cellRect<RetType, T: QGridLayout_cellRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cellRect(self);
    // return 1;
  }
}

pub trait QGridLayout_cellRect<RetType> {
  fn cellRect(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QRect QGridLayout::cellRect(int row, int column);
impl<'a> /*trait*/ QGridLayout_cellRect<QRect> for (i32, i32) {
  fn cellRect(self , rsthis: &mut QGridLayout) -> QRect {
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

// proto:  void QGridLayout::setRowStretch(int row, int stretch);
impl /*struct*/ QGridLayout {
  pub fn setRowStretch<RetType, T: QGridLayout_setRowStretch<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRowStretch(self);
    // return 1;
  }
}

pub trait QGridLayout_setRowStretch<RetType> {
  fn setRowStretch(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setRowStretch(int row, int stretch);
impl<'a> /*trait*/ QGridLayout_setRowStretch<()> for (i32, i32) {
  fn setRowStretch(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout13setRowStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QGridLayout13setRowStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
impl /*struct*/ QGridLayout {
  pub fn itemAtPosition<RetType, T: QGridLayout_itemAtPosition<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemAtPosition(self);
    // return 1;
  }
}

pub trait QGridLayout_itemAtPosition<RetType> {
  fn itemAtPosition(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QLayoutItem * QGridLayout::itemAtPosition(int row, int column);
impl<'a> /*trait*/ QGridLayout_itemAtPosition<QLayoutItem> for (i32, i32) {
  fn itemAtPosition(self , rsthis: &mut QGridLayout) -> QLayoutItem {
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

// proto:  const QMetaObject * QGridLayout::metaObject();
impl /*struct*/ QGridLayout {
  pub fn metaObject<RetType, T: QGridLayout_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGridLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  const QMetaObject * QGridLayout::metaObject();
impl<'a> /*trait*/ QGridLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout10metaObjectEv()};
     unsafe {_ZNK11QGridLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QGridLayout::verticalSpacing();
impl /*struct*/ QGridLayout {
  pub fn verticalSpacing<RetType, T: QGridLayout_verticalSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_verticalSpacing<RetType> {
  fn verticalSpacing(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGridLayout_verticalSpacing<i32> for () {
  fn verticalSpacing(self , rsthis: &mut QGridLayout) -> i32 {
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

// proto:  int QGridLayout::horizontalSpacing();
impl /*struct*/ QGridLayout {
  pub fn horizontalSpacing<RetType, T: QGridLayout_horizontalSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_horizontalSpacing<RetType> {
  fn horizontalSpacing(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGridLayout_horizontalSpacing<i32> for () {
  fn horizontalSpacing(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK11QGridLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
impl /*struct*/ QGridLayout {
  pub fn setColumnMinimumWidth<RetType, T: QGridLayout_setColumnMinimumWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setColumnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_setColumnMinimumWidth<RetType> {
  fn setColumnMinimumWidth(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setColumnMinimumWidth(int column, int minSize);
impl<'a> /*trait*/ QGridLayout_setColumnMinimumWidth<()> for (i32, i32) {
  fn setColumnMinimumWidth(self , rsthis: &mut QGridLayout) -> () {
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

// proto:  int QGridLayout::heightForWidth(int );
impl /*struct*/ QGridLayout {
  pub fn heightForWidth<RetType, T: QGridLayout_heightForWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QGridLayout_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  int QGridLayout::heightForWidth(int );
impl<'a> /*trait*/ QGridLayout_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QGridLayout::FreeQGridLayout();
impl /*struct*/ QGridLayout {
  pub fn FreeQGridLayout<RetType, T: QGridLayout_FreeQGridLayout<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQGridLayout(self);
    // return 1;
  }
}

pub trait QGridLayout_FreeQGridLayout<RetType> {
  fn FreeQGridLayout(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::FreeQGridLayout();
impl<'a> /*trait*/ QGridLayout_FreeQGridLayout<()> for () {
  fn FreeQGridLayout(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayoutD0Ev()};
     unsafe {_ZN11QGridLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QGridLayout::setSpacing(int spacing);
impl /*struct*/ QGridLayout {
  pub fn setSpacing<RetType, T: QGridLayout_setSpacing<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGridLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::setSpacing(int spacing);
impl<'a> /*trait*/ QGridLayout_setSpacing<()> for (i32) {
  fn setSpacing(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QGridLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QGridLayout::addWidget(QWidget * w);
impl /*struct*/ QGridLayout {
  pub fn addWidget<RetType, T: QGridLayout_addWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QGridLayout_addWidget<RetType> {
  fn addWidget(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  void QGridLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QGridLayout_addWidget<()> for (&'a mut QWidget) {
  fn addWidget(self , rsthis: &mut QGridLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QGridLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QGridLayout9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QLayoutItem * QGridLayout::itemAt(int index);
impl /*struct*/ QGridLayout {
  pub fn itemAt<RetType, T: QGridLayout_itemAt<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGridLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QLayoutItem * QGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGridLayout_itemAt<QLayoutItem> for (i32) {
  fn itemAt(self , rsthis: &mut QGridLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QGridLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QSize QGridLayout::maximumSize();
impl /*struct*/ QGridLayout {
  pub fn maximumSize<RetType, T: QGridLayout_maximumSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QGridLayout_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QGridLayout) -> RetType;
}

// proto:  QSize QGridLayout::maximumSize();
impl<'a> /*trait*/ QGridLayout_maximumSize<QSize> for () {
  fn maximumSize(self , rsthis: &mut QGridLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QGridLayout11maximumSizeEv()};
    let mut ret = unsafe {_ZNK11QGridLayout11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

