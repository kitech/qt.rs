// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmargins::QMargins;
use super::qrectf::QRectF;
use super::qmarginsf::QMarginsF;
use super::qrect::QRect;
use super::qpagesize::QPageSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QPageLayout::setRightMargin(qreal rightMargin);
  fn _ZN11QPageLayout14setRightMarginEd(qthis: *mut c_void, arg0: c_double) -> int8_t;
  // proto:  void QPageLayout::swap(QPageLayout & other);
  fn _ZN11QPageLayout4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMargins QPageLayout::marginsPoints();
  fn _ZNK11QPageLayout13marginsPointsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMargins QPageLayout::marginsPixels(int resolution);
  fn _ZNK11QPageLayout13marginsPixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QPageLayout::isValid();
  fn _ZNK11QPageLayout7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRectF QPageLayout::fullRect();
  fn _ZNK11QPageLayout8fullRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QPageLayout::paintRect();
  fn _ZNK11QPageLayout9paintRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPageLayout::setMinimumMargins(const QMarginsF & minMargins);
  fn _ZN11QPageLayout17setMinimumMarginsERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPageLayout::setLeftMargin(qreal leftMargin);
  fn _ZN11QPageLayout13setLeftMarginEd(qthis: *mut c_void, arg0: c_double) -> int8_t;
  // proto:  bool QPageLayout::setBottomMargin(qreal bottomMargin);
  fn _ZN11QPageLayout15setBottomMarginEd(qthis: *mut c_void, arg0: c_double) -> int8_t;
  // proto:  QRect QPageLayout::fullRectPoints();
  fn _ZNK11QPageLayout14fullRectPointsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMarginsF QPageLayout::minimumMargins();
  fn _ZNK11QPageLayout14minimumMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPageSize QPageLayout::pageSize();
  fn _ZNK11QPageLayout8pageSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPageLayout::setTopMargin(qreal topMargin);
  fn _ZN11QPageLayout12setTopMarginEd(qthis: *mut c_void, arg0: c_double) -> int8_t;
  // proto:  void QPageLayout::NewQPageLayout();
  fn _ZN11QPageLayoutC1Ev(qthis: *mut c_void) ;
  // proto:  void QPageLayout::NewQPageLayout(const QPageLayout & other);
  fn _ZN11QPageLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPageLayout::FreeQPageLayout();
  fn _ZN11QPageLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  QRect QPageLayout::fullRectPixels(int resolution);
  fn _ZNK11QPageLayout14fullRectPixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QMarginsF QPageLayout::margins();
  fn _ZNK11QPageLayout7marginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QPageLayout::paintRectPoints();
  fn _ZNK11QPageLayout15paintRectPointsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QPageLayout::paintRectPixels(int resolution);
  fn _ZNK11QPageLayout15paintRectPixelsEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QPageLayout::setPageSize(const QPageSize & pageSize, const QMarginsF & minMargins);
  fn _ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QMarginsF QPageLayout::maximumMargins();
  fn _ZNK11QPageLayout14maximumMarginsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPageLayout::isEquivalentTo(const QPageLayout & other);
  fn _ZNK11QPageLayout14isEquivalentToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QPageLayout::setMargins(const QMarginsF & margins);
  fn _ZN11QPageLayout10setMarginsERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QPageLayout)=1
pub struct QPageLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPageLayout {
  pub fn setRightMargin<T: QPageLayout_setRightMargin>(&mut self, value: T) -> i8 {
    return value.setRightMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setRightMargin {
  fn setRightMargin(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::setRightMargin(qreal rightMargin);
impl<'a> /*trait*/ QPageLayout_setRightMargin for (f64) {
  fn setRightMargin(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout14setRightMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout14setRightMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn swap<T: QPageLayout_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QPageLayout_swap {
  fn swap(self, rsthis: &mut QPageLayout) ;
}

// proto:  void QPageLayout::swap(QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_swap for (&'a mut QPageLayout) {
  fn swap(self, rsthis: &mut QPageLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPageLayout4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn marginsPoints<T: QPageLayout_marginsPoints>(&mut self, value: T) -> QMargins {
    return value.marginsPoints(self);
    // return 1;
  }
}

pub trait QPageLayout_marginsPoints {
  fn marginsPoints(self, rsthis: &mut QPageLayout) -> QMargins;
}

// proto:  QMargins QPageLayout::marginsPoints();
impl<'a> /*trait*/ QPageLayout_marginsPoints for () {
  fn marginsPoints(self, rsthis: &mut QPageLayout) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout13marginsPointsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout13marginsPointsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn marginsPixels<T: QPageLayout_marginsPixels>(&mut self, value: T) -> QMargins {
    return value.marginsPixels(self);
    // return 1;
  }
}

pub trait QPageLayout_marginsPixels {
  fn marginsPixels(self, rsthis: &mut QPageLayout) -> QMargins;
}

// proto:  QMargins QPageLayout::marginsPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_marginsPixels for (i32) {
  fn marginsPixels(self, rsthis: &mut QPageLayout) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout13marginsPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QPageLayout13marginsPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn isValid<T: QPageLayout_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QPageLayout_isValid {
  fn isValid(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::isValid();
impl<'a> /*trait*/ QPageLayout_isValid for () {
  fn isValid(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout7isValidEv()};
    let mut ret = unsafe {_ZNK11QPageLayout7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn fullRect<T: QPageLayout_fullRect>(&mut self, value: T) -> QRectF {
    return value.fullRect(self);
    // return 1;
  }
}

pub trait QPageLayout_fullRect {
  fn fullRect(self, rsthis: &mut QPageLayout) -> QRectF;
}

// proto:  QRectF QPageLayout::fullRect();
impl<'a> /*trait*/ QPageLayout_fullRect for () {
  fn fullRect(self, rsthis: &mut QPageLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout8fullRectEv()};
    let mut ret = unsafe {_ZNK11QPageLayout8fullRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn paintRect<T: QPageLayout_paintRect>(&mut self, value: T) -> QRectF {
    return value.paintRect(self);
    // return 1;
  }
}

pub trait QPageLayout_paintRect {
  fn paintRect(self, rsthis: &mut QPageLayout) -> QRectF;
}

// proto:  QRectF QPageLayout::paintRect();
impl<'a> /*trait*/ QPageLayout_paintRect for () {
  fn paintRect(self, rsthis: &mut QPageLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout9paintRectEv()};
    let mut ret = unsafe {_ZNK11QPageLayout9paintRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setMinimumMargins<T: QPageLayout_setMinimumMargins>(&mut self, value: T)  {
     value.setMinimumMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_setMinimumMargins {
  fn setMinimumMargins(self, rsthis: &mut QPageLayout) ;
}

// proto:  void QPageLayout::setMinimumMargins(const QMarginsF & minMargins);
impl<'a> /*trait*/ QPageLayout_setMinimumMargins for (&'a  QMarginsF) {
  fn setMinimumMargins(self, rsthis: &mut QPageLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setLeftMargin<T: QPageLayout_setLeftMargin>(&mut self, value: T) -> i8 {
    return value.setLeftMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setLeftMargin {
  fn setLeftMargin(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::setLeftMargin(qreal leftMargin);
impl<'a> /*trait*/ QPageLayout_setLeftMargin for (f64) {
  fn setLeftMargin(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout13setLeftMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout13setLeftMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setBottomMargin<T: QPageLayout_setBottomMargin>(&mut self, value: T) -> i8 {
    return value.setBottomMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setBottomMargin {
  fn setBottomMargin(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::setBottomMargin(qreal bottomMargin);
impl<'a> /*trait*/ QPageLayout_setBottomMargin for (f64) {
  fn setBottomMargin(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout15setBottomMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout15setBottomMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn fullRectPoints<T: QPageLayout_fullRectPoints>(&mut self, value: T) -> QRect {
    return value.fullRectPoints(self);
    // return 1;
  }
}

pub trait QPageLayout_fullRectPoints {
  fn fullRectPoints(self, rsthis: &mut QPageLayout) -> QRect;
}

// proto:  QRect QPageLayout::fullRectPoints();
impl<'a> /*trait*/ QPageLayout_fullRectPoints for () {
  fn fullRectPoints(self, rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14fullRectPointsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout14fullRectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn minimumMargins<T: QPageLayout_minimumMargins>(&mut self, value: T) -> QMarginsF {
    return value.minimumMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_minimumMargins {
  fn minimumMargins(self, rsthis: &mut QPageLayout) -> QMarginsF;
}

// proto:  QMarginsF QPageLayout::minimumMargins();
impl<'a> /*trait*/ QPageLayout_minimumMargins for () {
  fn minimumMargins(self, rsthis: &mut QPageLayout) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14minimumMarginsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout14minimumMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMarginsF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn pageSize<T: QPageLayout_pageSize>(&mut self, value: T) -> QPageSize {
    return value.pageSize(self);
    // return 1;
  }
}

pub trait QPageLayout_pageSize {
  fn pageSize(self, rsthis: &mut QPageLayout) -> QPageSize;
}

// proto:  QPageSize QPageLayout::pageSize();
impl<'a> /*trait*/ QPageLayout_pageSize for () {
  fn pageSize(self, rsthis: &mut QPageLayout) -> QPageSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout8pageSizeEv()};
    let mut ret = unsafe {_ZNK11QPageLayout8pageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QPageSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setTopMargin<T: QPageLayout_setTopMargin>(&mut self, value: T) -> i8 {
    return value.setTopMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setTopMargin {
  fn setTopMargin(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::setTopMargin(qreal topMargin);
impl<'a> /*trait*/ QPageLayout_setTopMargin for (f64) {
  fn setTopMargin(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout12setTopMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout12setTopMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn NewQPageLayout<T: QPageLayout_NewQPageLayout>(value: T) -> QPageLayout {
    let rsthis = value.NewQPageLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QPageLayout_NewQPageLayout {
  fn NewQPageLayout(self) -> QPageLayout;
}

// proto: void QPageLayout::NewQPageLayout();
impl<'a> /*trait*/ QPageLayout_NewQPageLayout for () {
  fn NewQPageLayout(self) -> QPageLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayoutC1Ev()};
    unsafe {_ZN11QPageLayoutC1Ev(qthis)};
    let rsthis = QPageLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPageLayout::NewQPageLayout(const QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_NewQPageLayout for (&'a  QPageLayout) {
  fn NewQPageLayout(self) -> QPageLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPageLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QPageLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn FreeQPageLayout<T: QPageLayout_FreeQPageLayout>(&mut self, value: T)  {
     value.FreeQPageLayout(self);
    // return 1;
  }
}

pub trait QPageLayout_FreeQPageLayout {
  fn FreeQPageLayout(self, rsthis: &mut QPageLayout) ;
}

// proto:  void QPageLayout::FreeQPageLayout();
impl<'a> /*trait*/ QPageLayout_FreeQPageLayout for () {
  fn FreeQPageLayout(self, rsthis: &mut QPageLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayoutD0Ev()};
     unsafe {_ZN11QPageLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn fullRectPixels<T: QPageLayout_fullRectPixels>(&mut self, value: T) -> QRect {
    return value.fullRectPixels(self);
    // return 1;
  }
}

pub trait QPageLayout_fullRectPixels {
  fn fullRectPixels(self, rsthis: &mut QPageLayout) -> QRect;
}

// proto:  QRect QPageLayout::fullRectPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_fullRectPixels for (i32) {
  fn fullRectPixels(self, rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14fullRectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QPageLayout14fullRectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn margins<T: QPageLayout_margins>(&mut self, value: T) -> QMarginsF {
    return value.margins(self);
    // return 1;
  }
}

pub trait QPageLayout_margins {
  fn margins(self, rsthis: &mut QPageLayout) -> QMarginsF;
}

// proto:  QMarginsF QPageLayout::margins();
impl<'a> /*trait*/ QPageLayout_margins for () {
  fn margins(self, rsthis: &mut QPageLayout) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout7marginsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout7marginsEv(rsthis.qclsinst)};
    let mut ret1 = QMarginsF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn paintRectPoints<T: QPageLayout_paintRectPoints>(&mut self, value: T) -> QRect {
    return value.paintRectPoints(self);
    // return 1;
  }
}

pub trait QPageLayout_paintRectPoints {
  fn paintRectPoints(self, rsthis: &mut QPageLayout) -> QRect;
}

// proto:  QRect QPageLayout::paintRectPoints();
impl<'a> /*trait*/ QPageLayout_paintRectPoints for () {
  fn paintRectPoints(self, rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout15paintRectPointsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout15paintRectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn paintRectPixels<T: QPageLayout_paintRectPixels>(&mut self, value: T) -> QRect {
    return value.paintRectPixels(self);
    // return 1;
  }
}

pub trait QPageLayout_paintRectPixels {
  fn paintRectPixels(self, rsthis: &mut QPageLayout) -> QRect;
}

// proto:  QRect QPageLayout::paintRectPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_paintRectPixels for (i32) {
  fn paintRectPixels(self, rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout15paintRectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QPageLayout15paintRectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setPageSize<T: QPageLayout_setPageSize>(&mut self, value: T)  {
     value.setPageSize(self);
    // return 1;
  }
}

pub trait QPageLayout_setPageSize {
  fn setPageSize(self, rsthis: &mut QPageLayout) ;
}

// proto:  void QPageLayout::setPageSize(const QPageSize & pageSize, const QMarginsF & minMargins);
impl<'a> /*trait*/ QPageLayout_setPageSize for (&'a  QPageSize, &'a  QMarginsF) {
  fn setPageSize(self, rsthis: &mut QPageLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn maximumMargins<T: QPageLayout_maximumMargins>(&mut self, value: T) -> QMarginsF {
    return value.maximumMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_maximumMargins {
  fn maximumMargins(self, rsthis: &mut QPageLayout) -> QMarginsF;
}

// proto:  QMarginsF QPageLayout::maximumMargins();
impl<'a> /*trait*/ QPageLayout_maximumMargins for () {
  fn maximumMargins(self, rsthis: &mut QPageLayout) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14maximumMarginsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout14maximumMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMarginsF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn isEquivalentTo<T: QPageLayout_isEquivalentTo>(&mut self, value: T) -> i8 {
    return value.isEquivalentTo(self);
    // return 1;
  }
}

pub trait QPageLayout_isEquivalentTo {
  fn isEquivalentTo(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::isEquivalentTo(const QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_isEquivalentTo for (&'a  QPageLayout) {
  fn isEquivalentTo(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14isEquivalentToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QPageLayout14isEquivalentToERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setMargins<T: QPageLayout_setMargins>(&mut self, value: T) -> i8 {
    return value.setMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_setMargins {
  fn setMargins(self, rsthis: &mut QPageLayout) -> i8;
}

// proto:  bool QPageLayout::setMargins(const QMarginsF & margins);
impl<'a> /*trait*/ QPageLayout_setMargins for (&'a  QMarginsF) {
  fn setMargins(self, rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout10setMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QPageLayout10setMarginsERK9QMarginsF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

