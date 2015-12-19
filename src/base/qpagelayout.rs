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

// proto:  bool QPageLayout::setRightMargin(qreal rightMargin);
impl /*struct*/ QPageLayout {
  pub fn setRightMargin<RetType, T: QPageLayout_setRightMargin<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setRightMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setRightMargin<RetType> {
  fn setRightMargin(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::setRightMargin(qreal rightMargin);
impl<'a> /*trait*/ QPageLayout_setRightMargin<i8> for (f64) {
  fn setRightMargin(self , rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout14setRightMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout14setRightMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPageLayout::swap(QPageLayout & other);
impl /*struct*/ QPageLayout {
  pub fn swap<RetType, T: QPageLayout_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPageLayout_swap<RetType> {
  fn swap(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  void QPageLayout::swap(QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_swap<()> for (&'a mut QPageLayout) {
  fn swap(self , rsthis: &mut QPageLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPageLayout4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QMargins QPageLayout::marginsPoints();
impl /*struct*/ QPageLayout {
  pub fn marginsPoints<RetType, T: QPageLayout_marginsPoints<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.marginsPoints(self);
    // return 1;
  }
}

pub trait QPageLayout_marginsPoints<RetType> {
  fn marginsPoints(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QMargins QPageLayout::marginsPoints();
impl<'a> /*trait*/ QPageLayout_marginsPoints<QMargins> for () {
  fn marginsPoints(self , rsthis: &mut QPageLayout) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout13marginsPointsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout13marginsPointsEv(rsthis.qclsinst)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMargins QPageLayout::marginsPixels(int resolution);
impl /*struct*/ QPageLayout {
  pub fn marginsPixels<RetType, T: QPageLayout_marginsPixels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.marginsPixels(self);
    // return 1;
  }
}

pub trait QPageLayout_marginsPixels<RetType> {
  fn marginsPixels(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QMargins QPageLayout::marginsPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_marginsPixels<QMargins> for (i32) {
  fn marginsPixels(self , rsthis: &mut QPageLayout) -> QMargins {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout13marginsPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QPageLayout13marginsPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMargins{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QPageLayout::isValid();
impl /*struct*/ QPageLayout {
  pub fn isValid<RetType, T: QPageLayout_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QPageLayout_isValid<RetType> {
  fn isValid(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::isValid();
impl<'a> /*trait*/ QPageLayout_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout7isValidEv()};
    let mut ret = unsafe {_ZNK11QPageLayout7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QRectF QPageLayout::fullRect();
impl /*struct*/ QPageLayout {
  pub fn fullRect<RetType, T: QPageLayout_fullRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fullRect(self);
    // return 1;
  }
}

pub trait QPageLayout_fullRect<RetType> {
  fn fullRect(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QRectF QPageLayout::fullRect();
impl<'a> /*trait*/ QPageLayout_fullRect<QRectF> for () {
  fn fullRect(self , rsthis: &mut QPageLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout8fullRectEv()};
    let mut ret = unsafe {_ZNK11QPageLayout8fullRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRectF QPageLayout::paintRect();
impl /*struct*/ QPageLayout {
  pub fn paintRect<RetType, T: QPageLayout_paintRect<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.paintRect(self);
    // return 1;
  }
}

pub trait QPageLayout_paintRect<RetType> {
  fn paintRect(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QRectF QPageLayout::paintRect();
impl<'a> /*trait*/ QPageLayout_paintRect<QRectF> for () {
  fn paintRect(self , rsthis: &mut QPageLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout9paintRectEv()};
    let mut ret = unsafe {_ZNK11QPageLayout9paintRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPageLayout::setMinimumMargins(const QMarginsF & minMargins);
impl /*struct*/ QPageLayout {
  pub fn setMinimumMargins<RetType, T: QPageLayout_setMinimumMargins<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMinimumMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_setMinimumMargins<RetType> {
  fn setMinimumMargins(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  void QPageLayout::setMinimumMargins(const QMarginsF & minMargins);
impl<'a> /*trait*/ QPageLayout_setMinimumMargins<()> for (&'a  QMarginsF) {
  fn setMinimumMargins(self , rsthis: &mut QPageLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QPageLayout::setLeftMargin(qreal leftMargin);
impl /*struct*/ QPageLayout {
  pub fn setLeftMargin<RetType, T: QPageLayout_setLeftMargin<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLeftMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setLeftMargin<RetType> {
  fn setLeftMargin(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::setLeftMargin(qreal leftMargin);
impl<'a> /*trait*/ QPageLayout_setLeftMargin<i8> for (f64) {
  fn setLeftMargin(self , rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout13setLeftMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout13setLeftMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QPageLayout::setBottomMargin(qreal bottomMargin);
impl /*struct*/ QPageLayout {
  pub fn setBottomMargin<RetType, T: QPageLayout_setBottomMargin<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setBottomMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setBottomMargin<RetType> {
  fn setBottomMargin(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::setBottomMargin(qreal bottomMargin);
impl<'a> /*trait*/ QPageLayout_setBottomMargin<i8> for (f64) {
  fn setBottomMargin(self , rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout15setBottomMarginEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN11QPageLayout15setBottomMarginEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QRect QPageLayout::fullRectPoints();
impl /*struct*/ QPageLayout {
  pub fn fullRectPoints<RetType, T: QPageLayout_fullRectPoints<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fullRectPoints(self);
    // return 1;
  }
}

pub trait QPageLayout_fullRectPoints<RetType> {
  fn fullRectPoints(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QRect QPageLayout::fullRectPoints();
impl<'a> /*trait*/ QPageLayout_fullRectPoints<QRect> for () {
  fn fullRectPoints(self , rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14fullRectPointsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout14fullRectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMarginsF QPageLayout::minimumMargins();
impl /*struct*/ QPageLayout {
  pub fn minimumMargins<RetType, T: QPageLayout_minimumMargins<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.minimumMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_minimumMargins<RetType> {
  fn minimumMargins(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QMarginsF QPageLayout::minimumMargins();
impl<'a> /*trait*/ QPageLayout_minimumMargins<QMarginsF> for () {
  fn minimumMargins(self , rsthis: &mut QPageLayout) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14minimumMarginsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout14minimumMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMarginsF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPageSize QPageLayout::pageSize();
impl /*struct*/ QPageLayout {
  pub fn pageSize<RetType, T: QPageLayout_pageSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.pageSize(self);
    // return 1;
  }
}

pub trait QPageLayout_pageSize<RetType> {
  fn pageSize(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QPageSize QPageLayout::pageSize();
impl<'a> /*trait*/ QPageLayout_pageSize<QPageSize> for () {
  fn pageSize(self , rsthis: &mut QPageLayout) -> QPageSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout8pageSizeEv()};
    let mut ret = unsafe {_ZNK11QPageLayout8pageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QPageSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QPageLayout::setTopMargin(qreal topMargin);
impl /*struct*/ QPageLayout {
  pub fn setTopMargin<RetType, T: QPageLayout_setTopMargin<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setTopMargin(self);
    // return 1;
  }
}

pub trait QPageLayout_setTopMargin<RetType> {
  fn setTopMargin(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::setTopMargin(qreal topMargin);
impl<'a> /*trait*/ QPageLayout_setTopMargin<i8> for (f64) {
  fn setTopMargin(self , rsthis: &mut QPageLayout) -> i8 {
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

// proto:  void QPageLayout::FreeQPageLayout();
impl /*struct*/ QPageLayout {
  pub fn FreeQPageLayout<RetType, T: QPageLayout_FreeQPageLayout<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQPageLayout(self);
    // return 1;
  }
}

pub trait QPageLayout_FreeQPageLayout<RetType> {
  fn FreeQPageLayout(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  void QPageLayout::FreeQPageLayout();
impl<'a> /*trait*/ QPageLayout_FreeQPageLayout<()> for () {
  fn FreeQPageLayout(self , rsthis: &mut QPageLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayoutD0Ev()};
     unsafe {_ZN11QPageLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QRect QPageLayout::fullRectPixels(int resolution);
impl /*struct*/ QPageLayout {
  pub fn fullRectPixels<RetType, T: QPageLayout_fullRectPixels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.fullRectPixels(self);
    // return 1;
  }
}

pub trait QPageLayout_fullRectPixels<RetType> {
  fn fullRectPixels(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QRect QPageLayout::fullRectPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_fullRectPixels<QRect> for (i32) {
  fn fullRectPixels(self , rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14fullRectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QPageLayout14fullRectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMarginsF QPageLayout::margins();
impl /*struct*/ QPageLayout {
  pub fn margins<RetType, T: QPageLayout_margins<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.margins(self);
    // return 1;
  }
}

pub trait QPageLayout_margins<RetType> {
  fn margins(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QMarginsF QPageLayout::margins();
impl<'a> /*trait*/ QPageLayout_margins<QMarginsF> for () {
  fn margins(self , rsthis: &mut QPageLayout) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout7marginsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout7marginsEv(rsthis.qclsinst)};
    let mut ret1 = QMarginsF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRect QPageLayout::paintRectPoints();
impl /*struct*/ QPageLayout {
  pub fn paintRectPoints<RetType, T: QPageLayout_paintRectPoints<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.paintRectPoints(self);
    // return 1;
  }
}

pub trait QPageLayout_paintRectPoints<RetType> {
  fn paintRectPoints(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QRect QPageLayout::paintRectPoints();
impl<'a> /*trait*/ QPageLayout_paintRectPoints<QRect> for () {
  fn paintRectPoints(self , rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout15paintRectPointsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout15paintRectPointsEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QRect QPageLayout::paintRectPixels(int resolution);
impl /*struct*/ QPageLayout {
  pub fn paintRectPixels<RetType, T: QPageLayout_paintRectPixels<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.paintRectPixels(self);
    // return 1;
  }
}

pub trait QPageLayout_paintRectPixels<RetType> {
  fn paintRectPixels(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QRect QPageLayout::paintRectPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_paintRectPixels<QRect> for (i32) {
  fn paintRectPixels(self , rsthis: &mut QPageLayout) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout15paintRectPixelsEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QPageLayout15paintRectPixelsEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPageLayout::setPageSize(const QPageSize & pageSize, const QMarginsF & minMargins);
impl /*struct*/ QPageLayout {
  pub fn setPageSize<RetType, T: QPageLayout_setPageSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPageSize(self);
    // return 1;
  }
}

pub trait QPageLayout_setPageSize<RetType> {
  fn setPageSize(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  void QPageLayout::setPageSize(const QPageSize & pageSize, const QMarginsF & minMargins);
impl<'a> /*trait*/ QPageLayout_setPageSize<()> for (&'a  QPageSize, &'a  QMarginsF) {
  fn setPageSize(self , rsthis: &mut QPageLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QMarginsF QPageLayout::maximumMargins();
impl /*struct*/ QPageLayout {
  pub fn maximumMargins<RetType, T: QPageLayout_maximumMargins<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.maximumMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_maximumMargins<RetType> {
  fn maximumMargins(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  QMarginsF QPageLayout::maximumMargins();
impl<'a> /*trait*/ QPageLayout_maximumMargins<QMarginsF> for () {
  fn maximumMargins(self , rsthis: &mut QPageLayout) -> QMarginsF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14maximumMarginsEv()};
    let mut ret = unsafe {_ZNK11QPageLayout14maximumMarginsEv(rsthis.qclsinst)};
    let mut ret1 = QMarginsF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QPageLayout::isEquivalentTo(const QPageLayout & other);
impl /*struct*/ QPageLayout {
  pub fn isEquivalentTo<RetType, T: QPageLayout_isEquivalentTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isEquivalentTo(self);
    // return 1;
  }
}

pub trait QPageLayout_isEquivalentTo<RetType> {
  fn isEquivalentTo(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::isEquivalentTo(const QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_isEquivalentTo<i8> for (&'a  QPageLayout) {
  fn isEquivalentTo(self , rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14isEquivalentToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QPageLayout14isEquivalentToERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QPageLayout::setMargins(const QMarginsF & margins);
impl /*struct*/ QPageLayout {
  pub fn setMargins<RetType, T: QPageLayout_setMargins<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setMargins(self);
    // return 1;
  }
}

pub trait QPageLayout_setMargins<RetType> {
  fn setMargins(self , rsthis: &mut QPageLayout) -> RetType;
}

// proto:  bool QPageLayout::setMargins(const QMarginsF & margins);
impl<'a> /*trait*/ QPageLayout_setMargins<i8> for (&'a  QMarginsF) {
  fn setMargins(self , rsthis: &mut QPageLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout10setMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QPageLayout10setMarginsERK9QMarginsF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

