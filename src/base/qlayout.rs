// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qsize::QSize;
use super::qwidget::QWidget;
use super::qlayoutitem::QLayoutItem;
use super::qmargins::QMargins;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QLayout::setContentsMargins(int left, int top, int right, int bottom);
  fn _ZN7QLayout18setContentsMarginsEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  int QLayout::spacing();
  fn _ZNK7QLayout7spacingEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLayout::NewQLayout();
  fn _ZN7QLayoutC1Ev(qthis: *mut c_void) ;
  // proto:  QRect QLayout::geometry();
  fn _ZNK7QLayout8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QLayout::count();
  fn _ZNK7QLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QLayout::maximumSize();
  fn _ZNK7QLayout11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::setMenuBar(QWidget * w);
  fn _ZN7QLayout10setMenuBarEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLayout::indexOf(QWidget * );
  fn _ZNK7QLayout7indexOfEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QLayout::setEnabled(bool );
  fn _ZN7QLayout10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QLayout::minimumSize();
  fn _ZNK7QLayout11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLayoutItem * QLayout::takeAt(int index);
  fn _ZN7QLayout6takeAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QSize QLayout::totalMaximumSize();
  fn _ZNK7QLayout16totalMaximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::invalidate();
  fn _ZN7QLayout10invalidateEv(qthis: *mut c_void) ;
  // proto:  void QLayout::update();
  fn _ZN7QLayout6updateEv(qthis: *mut c_void) ;
  // proto:  QRect QLayout::contentsRect();
  fn _ZNK7QLayout12contentsRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QLayout::totalSizeHint();
  fn _ZNK7QLayout13totalSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::NewQLayout(QWidget * parent);
  fn _ZN7QLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLayout::addItem(QLayoutItem * );
  fn _ZN7QLayout7addItemEP11QLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLayout::totalHeightForWidth(int w);
  fn _ZNK7QLayout19totalHeightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QLayout::setMargin(int );
  fn _ZN7QLayout9setMarginEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QLayout::isEmpty();
  fn _ZNK7QLayout7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLayout::addWidget(QWidget * w);
  fn _ZN7QLayout9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLayout::getContentsMargins(int * left, int * top, int * right, int * bottom);
  fn _ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  QLayout * QLayout::layout();
  fn _ZN7QLayout6layoutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLayout::activate();
  fn _ZN7QLayout8activateEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QLayout::isEnabled();
  fn _ZNK7QLayout9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QLayout::FreeQLayout();
  fn _ZN7QLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  int QLayout::margin();
  fn _ZNK7QLayout6marginEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLayout::setSpacing(int );
  fn _ZN7QLayout10setSpacingEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QWidget * QLayout::menuBar();
  fn _ZNK7QLayout7menuBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::NewQLayout(const QLayout & );
  fn _ZN7QLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QLayout::metaObject();
  fn _ZNK7QLayout10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QLayoutItem * QLayout::itemAt(int index);
  fn _ZNK7QLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QLayout::parentWidget();
  fn _ZNK7QLayout12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::removeWidget(QWidget * w);
  fn _ZN7QLayout12removeWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLayout::removeItem(QLayoutItem * );
  fn _ZN7QLayout10removeItemEP11QLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMargins QLayout::contentsMargins();
  fn _ZNK7QLayout15contentsMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QLayout::totalMinimumSize();
  fn _ZNK7QLayout16totalMinimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::setGeometry(const QRect & );
  fn _ZN7QLayout11setGeometryERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QSize QLayout::closestAcceptableSize(const QWidget * w, const QSize & s);
  fn _ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QLayout::setContentsMargins(const QMargins & margins);
  fn _ZN7QLayout18setContentsMarginsERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QLayout)=1
pub struct QLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLayout {
  pub fn setContentsMargins<RetType, T: QLayout_setContentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.setContentsMargins(self);
    // return 1;
  }
}

pub trait QLayout_setContentsMargins<RetType> {
  fn setContentsMargins(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::setContentsMargins(int left, int top, int right, int bottom);
impl<'a> /*trait*/ QLayout_setContentsMargins<()> for (i32, i32, i32, i32) {
  fn setContentsMargins(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout18setContentsMarginsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN7QLayout18setContentsMarginsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn spacing<RetType, T: QLayout_spacing<RetType>>(&mut self, value: T) -> RetType {
    return value.spacing(self);
    // return 1;
  }
}

pub trait QLayout_spacing<RetType> {
  fn spacing(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  int QLayout::spacing();
impl<'a> /*trait*/ QLayout_spacing<i32> for () {
  fn spacing(self, rsthis: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7spacingEv()};
    let mut ret = unsafe {_ZNK7QLayout7spacingEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
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
  pub fn geometry<RetType, T: QLayout_geometry<RetType>>(&mut self, value: T) -> RetType {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QLayout_geometry<RetType> {
  fn geometry(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QRect QLayout::geometry();
impl<'a> /*trait*/ QLayout_geometry<QRect> for () {
  fn geometry(self, rsthis: &mut QLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout8geometryEv()};
    let mut ret = unsafe {_ZNK7QLayout8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn count<RetType, T: QLayout_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QLayout_count<RetType> {
  fn count(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  int QLayout::count();
impl<'a> /*trait*/ QLayout_count<i32> for () {
  fn count(self, rsthis: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout5countEv()};
    let mut ret = unsafe {_ZNK7QLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn maximumSize<RetType, T: QLayout_maximumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.maximumSize(self);
    // return 1;
  }
}

pub trait QLayout_maximumSize<RetType> {
  fn maximumSize(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QSize QLayout::maximumSize();
impl<'a> /*trait*/ QLayout_maximumSize<QSize> for () {
  fn maximumSize(self, rsthis: &mut QLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout11maximumSizeEv()};
    let mut ret = unsafe {_ZNK7QLayout11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setMenuBar<RetType, T: QLayout_setMenuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.setMenuBar(self);
    // return 1;
  }
}

pub trait QLayout_setMenuBar<RetType> {
  fn setMenuBar(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::setMenuBar(QWidget * w);
impl<'a> /*trait*/ QLayout_setMenuBar<()> for (&'a mut QWidget) {
  fn setMenuBar(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10setMenuBarEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout10setMenuBarEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn indexOf<RetType, T: QLayout_indexOf<RetType>>(&mut self, value: T) -> RetType {
    return value.indexOf(self);
    // return 1;
  }
}

pub trait QLayout_indexOf<RetType> {
  fn indexOf(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  int QLayout::indexOf(QWidget * );
impl<'a> /*trait*/ QLayout_indexOf<i32> for (&'a mut QWidget) {
  fn indexOf(self, rsthis: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QLayout7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setEnabled<RetType, T: QLayout_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QLayout_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::setEnabled(bool );
impl<'a> /*trait*/ QLayout_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QLayout10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn minimumSize<RetType, T: QLayout_minimumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumSize(self);
    // return 1;
  }
}

pub trait QLayout_minimumSize<RetType> {
  fn minimumSize(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QSize QLayout::minimumSize();
impl<'a> /*trait*/ QLayout_minimumSize<QSize> for () {
  fn minimumSize(self, rsthis: &mut QLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout11minimumSizeEv()};
    let mut ret = unsafe {_ZNK7QLayout11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn takeAt<RetType, T: QLayout_takeAt<RetType>>(&mut self, value: T) -> RetType {
    return value.takeAt(self);
    // return 1;
  }
}

pub trait QLayout_takeAt<RetType> {
  fn takeAt(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QLayoutItem * QLayout::takeAt(int index);
impl<'a> /*trait*/ QLayout_takeAt<QLayoutItem> for (i32) {
  fn takeAt(self, rsthis: &mut QLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout6takeAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN7QLayout6takeAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalMaximumSize<RetType, T: QLayout_totalMaximumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.totalMaximumSize(self);
    // return 1;
  }
}

pub trait QLayout_totalMaximumSize<RetType> {
  fn totalMaximumSize(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QSize QLayout::totalMaximumSize();
impl<'a> /*trait*/ QLayout_totalMaximumSize<QSize> for () {
  fn totalMaximumSize(self, rsthis: &mut QLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout16totalMaximumSizeEv()};
    let mut ret = unsafe {_ZNK7QLayout16totalMaximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn invalidate<RetType, T: QLayout_invalidate<RetType>>(&mut self, value: T) -> RetType {
    return value.invalidate(self);
    // return 1;
  }
}

pub trait QLayout_invalidate<RetType> {
  fn invalidate(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::invalidate();
impl<'a> /*trait*/ QLayout_invalidate<()> for () {
  fn invalidate(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10invalidateEv()};
     unsafe {_ZN7QLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn update<RetType, T: QLayout_update<RetType>>(&mut self, value: T) -> RetType {
    return value.update(self);
    // return 1;
  }
}

pub trait QLayout_update<RetType> {
  fn update(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::update();
impl<'a> /*trait*/ QLayout_update<()> for () {
  fn update(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout6updateEv()};
     unsafe {_ZN7QLayout6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn contentsRect<RetType, T: QLayout_contentsRect<RetType>>(&mut self, value: T) -> RetType {
    return value.contentsRect(self);
    // return 1;
  }
}

pub trait QLayout_contentsRect<RetType> {
  fn contentsRect(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QRect QLayout::contentsRect();
impl<'a> /*trait*/ QLayout_contentsRect<QRect> for () {
  fn contentsRect(self, rsthis: &mut QLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout12contentsRectEv()};
    let mut ret = unsafe {_ZNK7QLayout12contentsRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalSizeHint<RetType, T: QLayout_totalSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.totalSizeHint(self);
    // return 1;
  }
}

pub trait QLayout_totalSizeHint<RetType> {
  fn totalSizeHint(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QSize QLayout::totalSizeHint();
impl<'a> /*trait*/ QLayout_totalSizeHint<QSize> for () {
  fn totalSizeHint(self, rsthis: &mut QLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout13totalSizeHintEv()};
    let mut ret = unsafe {_ZNK7QLayout13totalSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn addItem<RetType, T: QLayout_addItem<RetType>>(&mut self, value: T) -> RetType {
    return value.addItem(self);
    // return 1;
  }
}

pub trait QLayout_addItem<RetType> {
  fn addItem(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::addItem(QLayoutItem * );
impl<'a> /*trait*/ QLayout_addItem<()> for (&'a mut QLayoutItem) {
  fn addItem(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout7addItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout7addItemEP11QLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalHeightForWidth<RetType, T: QLayout_totalHeightForWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.totalHeightForWidth(self);
    // return 1;
  }
}

pub trait QLayout_totalHeightForWidth<RetType> {
  fn totalHeightForWidth(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  int QLayout::totalHeightForWidth(int w);
impl<'a> /*trait*/ QLayout_totalHeightForWidth<i32> for (i32) {
  fn totalHeightForWidth(self, rsthis: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout19totalHeightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QLayout19totalHeightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setMargin<RetType, T: QLayout_setMargin<RetType>>(&mut self, value: T) -> RetType {
    return value.setMargin(self);
    // return 1;
  }
}

pub trait QLayout_setMargin<RetType> {
  fn setMargin(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::setMargin(int );
impl<'a> /*trait*/ QLayout_setMargin<()> for (i32) {
  fn setMargin(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout9setMarginEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QLayout9setMarginEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn isEmpty<RetType, T: QLayout_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QLayout_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  bool QLayout::isEmpty();
impl<'a> /*trait*/ QLayout_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QLayout7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn addWidget<RetType, T: QLayout_addWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.addWidget(self);
    // return 1;
  }
}

pub trait QLayout_addWidget<RetType> {
  fn addWidget(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::addWidget(QWidget * w);
impl<'a> /*trait*/ QLayout_addWidget<()> for (&'a mut QWidget) {
  fn addWidget(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn getContentsMargins<RetType, T: QLayout_getContentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.getContentsMargins(self);
    // return 1;
  }
}

pub trait QLayout_getContentsMargins<RetType> {
  fn getContentsMargins(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::getContentsMargins(int * left, int * top, int * right, int * bottom);
impl<'a> /*trait*/ QLayout_getContentsMargins<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getContentsMargins(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK7QLayout18getContentsMarginsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn layout<RetType, T: QLayout_layout<RetType>>(&mut self, value: T) -> RetType {
    return value.layout(self);
    // return 1;
  }
}

pub trait QLayout_layout<RetType> {
  fn layout(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QLayout * QLayout::layout();
impl<'a> /*trait*/ QLayout_layout<QLayout> for () {
  fn layout(self, rsthis: &mut QLayout) -> QLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout6layoutEv()};
    let mut ret = unsafe {_ZN7QLayout6layoutEv(rsthis.qclsinst)};
    let mut ret1 = QLayout{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn activate<RetType, T: QLayout_activate<RetType>>(&mut self, value: T) -> RetType {
    return value.activate(self);
    // return 1;
  }
}

pub trait QLayout_activate<RetType> {
  fn activate(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  bool QLayout::activate();
impl<'a> /*trait*/ QLayout_activate<i8> for () {
  fn activate(self, rsthis: &mut QLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout8activateEv()};
    let mut ret = unsafe {_ZN7QLayout8activateEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn isEnabled<RetType, T: QLayout_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QLayout_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  bool QLayout::isEnabled();
impl<'a> /*trait*/ QLayout_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout9isEnabledEv()};
    let mut ret = unsafe {_ZNK7QLayout9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn FreeQLayout<RetType, T: QLayout_FreeQLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQLayout(self);
    // return 1;
  }
}

pub trait QLayout_FreeQLayout<RetType> {
  fn FreeQLayout(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::FreeQLayout();
impl<'a> /*trait*/ QLayout_FreeQLayout<()> for () {
  fn FreeQLayout(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayoutD0Ev()};
     unsafe {_ZN7QLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn margin<RetType, T: QLayout_margin<RetType>>(&mut self, value: T) -> RetType {
    return value.margin(self);
    // return 1;
  }
}

pub trait QLayout_margin<RetType> {
  fn margin(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  int QLayout::margin();
impl<'a> /*trait*/ QLayout_margin<i32> for () {
  fn margin(self, rsthis: &mut QLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout6marginEv()};
    let mut ret = unsafe {_ZNK7QLayout6marginEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setSpacing<RetType, T: QLayout_setSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.setSpacing(self);
    // return 1;
  }
}

pub trait QLayout_setSpacing<RetType> {
  fn setSpacing(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::setSpacing(int );
impl<'a> /*trait*/ QLayout_setSpacing<()> for (i32) {
  fn setSpacing(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10setSpacingEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QLayout10setSpacingEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn menuBar<RetType, T: QLayout_menuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.menuBar(self);
    // return 1;
  }
}

pub trait QLayout_menuBar<RetType> {
  fn menuBar(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QWidget * QLayout::menuBar();
impl<'a> /*trait*/ QLayout_menuBar<QWidget> for () {
  fn menuBar(self, rsthis: &mut QLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout7menuBarEv()};
    let mut ret = unsafe {_ZNK7QLayout7menuBarEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QLayout::NewQLayout(const QLayout & );
impl<'a> /*trait*/ QLayout_NewQLayout for (&'a  QLayout) {
  fn NewQLayout(self) -> QLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn metaObject<RetType, T: QLayout_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QLayout_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  const QMetaObject * QLayout::metaObject();
impl<'a> /*trait*/ QLayout_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout10metaObjectEv()};
     unsafe {_ZNK7QLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn itemAt<RetType, T: QLayout_itemAt<RetType>>(&mut self, value: T) -> RetType {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QLayout_itemAt<RetType> {
  fn itemAt(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QLayoutItem * QLayout::itemAt(int index);
impl<'a> /*trait*/ QLayout_itemAt<QLayoutItem> for (i32) {
  fn itemAt(self, rsthis: &mut QLayout) -> QLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QLayoutItem{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn parentWidget<RetType, T: QLayout_parentWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.parentWidget(self);
    // return 1;
  }
}

pub trait QLayout_parentWidget<RetType> {
  fn parentWidget(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QWidget * QLayout::parentWidget();
impl<'a> /*trait*/ QLayout_parentWidget<QWidget> for () {
  fn parentWidget(self, rsthis: &mut QLayout) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout12parentWidgetEv()};
    let mut ret = unsafe {_ZNK7QLayout12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn removeWidget<RetType, T: QLayout_removeWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.removeWidget(self);
    // return 1;
  }
}

pub trait QLayout_removeWidget<RetType> {
  fn removeWidget(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::removeWidget(QWidget * w);
impl<'a> /*trait*/ QLayout_removeWidget<()> for (&'a mut QWidget) {
  fn removeWidget(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout12removeWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn removeItem<RetType, T: QLayout_removeItem<RetType>>(&mut self, value: T) -> RetType {
    return value.removeItem(self);
    // return 1;
  }
}

pub trait QLayout_removeItem<RetType> {
  fn removeItem(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::removeItem(QLayoutItem * );
impl<'a> /*trait*/ QLayout_removeItem<()> for (&'a mut QLayoutItem) {
  fn removeItem(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout10removeItemEP11QLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout10removeItemEP11QLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn contentsMargins<RetType, T: QLayout_contentsMargins<RetType>>(&mut self, value: T) -> RetType {
    return value.contentsMargins(self);
    // return 1;
  }
}

pub trait QLayout_contentsMargins<RetType> {
  fn contentsMargins(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QMargins QLayout::contentsMargins();
impl<'a> /*trait*/ QLayout_contentsMargins<QMargins> for () {
  fn contentsMargins(self, rsthis: &mut QLayout) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout15contentsMarginsEv()};
    let mut ret = unsafe {_ZNK7QLayout15contentsMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn totalMinimumSize<RetType, T: QLayout_totalMinimumSize<RetType>>(&mut self, value: T) -> RetType {
    return value.totalMinimumSize(self);
    // return 1;
  }
}

pub trait QLayout_totalMinimumSize<RetType> {
  fn totalMinimumSize(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  QSize QLayout::totalMinimumSize();
impl<'a> /*trait*/ QLayout_totalMinimumSize<QSize> for () {
  fn totalMinimumSize(self, rsthis: &mut QLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLayout16totalMinimumSizeEv()};
    let mut ret = unsafe {_ZNK7QLayout16totalMinimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn setGeometry<RetType, T: QLayout_setGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.setGeometry(self);
    // return 1;
  }
}

pub trait QLayout_setGeometry<RetType> {
  fn setGeometry(self, rsthis: &mut QLayout) -> RetType;
}

// proto:  void QLayout::setGeometry(const QRect & );
impl<'a> /*trait*/ QLayout_setGeometry<()> for (&'a  QRect) {
  fn setGeometry(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLayout {
  pub fn closestAcceptableSize<RetType, T: QLayout_closestAcceptableSize<RetType>>(&mut self, value: T) -> RetType {
    return value.closestAcceptableSize(self);
    // return 1;
  }
}

pub trait QLayout_closestAcceptableSize<RetType> {
  fn closestAcceptableSize(self, rsthis: &mut QLayout) -> RetType;
}

// proto: static QSize QLayout::closestAcceptableSize(const QWidget * w, const QSize & s);
impl<'a> /*trait*/ QLayout_closestAcceptableSize<QSize> for (&'a  QWidget, &'a  QSize) {
  fn closestAcceptableSize(self, rsthis: &mut QLayout) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QLayout21closestAcceptableSizeEPK7QWidgetRK5QSize(arg0, arg1)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QLayout::setContentsMargins(const QMargins & margins);
impl<'a> /*trait*/ QLayout_setContentsMargins<()> for (&'a  QMargins) {
  fn setContentsMargins(self, rsthis: &mut QLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLayout18setContentsMarginsERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QLayout18setContentsMarginsERK8QMargins(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

