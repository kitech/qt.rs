// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmarginsf::QMarginsF;
use super::qpagesize::QPageSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QPageLayout::setRightMargin(qreal rightMargin);
  fn _ZN11QPageLayout14setRightMarginEd(arg0: c_double) -> i32;
  // proto: void QPageLayout::swap(QPageLayout & other);
  fn _ZN11QPageLayout4swapERS_(arg0: *mut c_void) -> i32;
  // proto: QMargins QPageLayout::marginsPoints();
  fn _ZNK11QPageLayout13marginsPointsEv() -> i32;
  // proto: QMargins QPageLayout::marginsPixels(int resolution);
  fn _ZNK11QPageLayout13marginsPixelsEi(arg0: c_int) -> i32;
  // proto: bool QPageLayout::isValid();
  fn _ZNK11QPageLayout7isValidEv() -> i32;
  // proto: QRectF QPageLayout::fullRect();
  fn _ZNK11QPageLayout8fullRectEv() -> i32;
  // proto: QRectF QPageLayout::paintRect();
  fn _ZNK11QPageLayout9paintRectEv() -> i32;
  // proto: void QPageLayout::setMinimumMargins(const QMarginsF & minMargins);
  fn _ZN11QPageLayout17setMinimumMarginsERK9QMarginsF(arg0: *const c_void) -> i32;
  // proto: bool QPageLayout::setLeftMargin(qreal leftMargin);
  fn _ZN11QPageLayout13setLeftMarginEd(arg0: c_double) -> i32;
  // proto: bool QPageLayout::setBottomMargin(qreal bottomMargin);
  fn _ZN11QPageLayout15setBottomMarginEd(arg0: c_double) -> i32;
  // proto: QRect QPageLayout::fullRectPoints();
  fn _ZNK11QPageLayout14fullRectPointsEv() -> i32;
  // proto: QMarginsF QPageLayout::minimumMargins();
  fn _ZNK11QPageLayout14minimumMarginsEv() -> i32;
  // proto: QPageSize QPageLayout::pageSize();
  fn _ZNK11QPageLayout8pageSizeEv() -> i32;
  // proto: bool QPageLayout::setTopMargin(qreal topMargin);
  fn _ZN11QPageLayout12setTopMarginEd(arg0: c_double) -> i32;
  // proto: void QPageLayout::NewQPageLayout();
  fn _ZN11QPageLayoutC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPageLayout::NewQPageLayout(const QPageLayout & other);
  fn _ZN11QPageLayoutC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPageLayout::FreeQPageLayout();
  fn _ZN11QPageLayoutD0Ev() -> i32;
  // proto: QRect QPageLayout::fullRectPixels(int resolution);
  fn _ZNK11QPageLayout14fullRectPixelsEi(arg0: c_int) -> i32;
  // proto: QMarginsF QPageLayout::margins();
  fn _ZNK11QPageLayout7marginsEv() -> i32;
  // proto: QRect QPageLayout::paintRectPoints();
  fn _ZNK11QPageLayout15paintRectPointsEv() -> i32;
  // proto: QRect QPageLayout::paintRectPixels(int resolution);
  fn _ZNK11QPageLayout15paintRectPixelsEi(arg0: c_int) -> i32;
  // proto: void QPageLayout::setPageSize(const QPageSize & pageSize, const QMarginsF & minMargins);
  fn _ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QMarginsF QPageLayout::maximumMargins();
  fn _ZNK11QPageLayout14maximumMarginsEv() -> i32;
  // proto: bool QPageLayout::isEquivalentTo(const QPageLayout & other);
  fn _ZNK11QPageLayout14isEquivalentToERKS_(arg0: *const c_void) -> i32;
  // proto: bool QPageLayout::setMargins(const QMarginsF & margins);
  fn _ZN11QPageLayout10setMarginsERK9QMarginsF(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QPageLayout)=1
pub struct QPageLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPageLayout {
  pub fn setRightMargin<T: QPageLayout_setRightMargin>(&mut self, value: T) -> i32 {
    value.setRightMargin(self);
    return 1;
  }
}

pub trait QPageLayout_setRightMargin {
  fn setRightMargin(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::setRightMargin(qreal rightMargin);
impl<'a> /*trait*/ QPageLayout_setRightMargin for (f64) {
  fn setRightMargin(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout14setRightMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN11QPageLayout14setRightMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn swap<T: QPageLayout_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QPageLayout_swap {
  fn swap(self, this: &mut QPageLayout) -> i32;
}

// proto: void QPageLayout::swap(QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_swap for (&'a mut QPageLayout) {
  fn swap(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPageLayout4swapERS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn marginsPoints<T: QPageLayout_marginsPoints>(&mut self, value: T) -> i32 {
    value.marginsPoints(self);
    return 1;
  }
}

pub trait QPageLayout_marginsPoints {
  fn marginsPoints(self, this: &mut QPageLayout) -> i32;
}

// proto: QMargins QPageLayout::marginsPoints();
impl<'a> /*trait*/ QPageLayout_marginsPoints for () {
  fn marginsPoints(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout13marginsPointsEv()};
    unsafe {_ZNK11QPageLayout13marginsPointsEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn marginsPixels<T: QPageLayout_marginsPixels>(&mut self, value: T) -> i32 {
    value.marginsPixels(self);
    return 1;
  }
}

pub trait QPageLayout_marginsPixels {
  fn marginsPixels(self, this: &mut QPageLayout) -> i32;
}

// proto: QMargins QPageLayout::marginsPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_marginsPixels for (i32) {
  fn marginsPixels(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout13marginsPixelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QPageLayout13marginsPixelsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn isValid<T: QPageLayout_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QPageLayout_isValid {
  fn isValid(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::isValid();
impl<'a> /*trait*/ QPageLayout_isValid for () {
  fn isValid(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout7isValidEv()};
    unsafe {_ZNK11QPageLayout7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn fullRect<T: QPageLayout_fullRect>(&mut self, value: T) -> i32 {
    value.fullRect(self);
    return 1;
  }
}

pub trait QPageLayout_fullRect {
  fn fullRect(self, this: &mut QPageLayout) -> i32;
}

// proto: QRectF QPageLayout::fullRect();
impl<'a> /*trait*/ QPageLayout_fullRect for () {
  fn fullRect(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout8fullRectEv()};
    unsafe {_ZNK11QPageLayout8fullRectEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn paintRect<T: QPageLayout_paintRect>(&mut self, value: T) -> i32 {
    value.paintRect(self);
    return 1;
  }
}

pub trait QPageLayout_paintRect {
  fn paintRect(self, this: &mut QPageLayout) -> i32;
}

// proto: QRectF QPageLayout::paintRect();
impl<'a> /*trait*/ QPageLayout_paintRect for () {
  fn paintRect(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout9paintRectEv()};
    unsafe {_ZNK11QPageLayout9paintRectEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setMinimumMargins<T: QPageLayout_setMinimumMargins>(&mut self, value: T) -> i32 {
    value.setMinimumMargins(self);
    return 1;
  }
}

pub trait QPageLayout_setMinimumMargins {
  fn setMinimumMargins(self, this: &mut QPageLayout) -> i32;
}

// proto: void QPageLayout::setMinimumMargins(const QMarginsF & minMargins);
impl<'a> /*trait*/ QPageLayout_setMinimumMargins for (&'a  QMarginsF) {
  fn setMinimumMargins(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPageLayout17setMinimumMarginsERK9QMarginsF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setLeftMargin<T: QPageLayout_setLeftMargin>(&mut self, value: T) -> i32 {
    value.setLeftMargin(self);
    return 1;
  }
}

pub trait QPageLayout_setLeftMargin {
  fn setLeftMargin(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::setLeftMargin(qreal leftMargin);
impl<'a> /*trait*/ QPageLayout_setLeftMargin for (f64) {
  fn setLeftMargin(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout13setLeftMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN11QPageLayout13setLeftMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setBottomMargin<T: QPageLayout_setBottomMargin>(&mut self, value: T) -> i32 {
    value.setBottomMargin(self);
    return 1;
  }
}

pub trait QPageLayout_setBottomMargin {
  fn setBottomMargin(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::setBottomMargin(qreal bottomMargin);
impl<'a> /*trait*/ QPageLayout_setBottomMargin for (f64) {
  fn setBottomMargin(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout15setBottomMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN11QPageLayout15setBottomMarginEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn fullRectPoints<T: QPageLayout_fullRectPoints>(&mut self, value: T) -> i32 {
    value.fullRectPoints(self);
    return 1;
  }
}

pub trait QPageLayout_fullRectPoints {
  fn fullRectPoints(self, this: &mut QPageLayout) -> i32;
}

// proto: QRect QPageLayout::fullRectPoints();
impl<'a> /*trait*/ QPageLayout_fullRectPoints for () {
  fn fullRectPoints(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14fullRectPointsEv()};
    unsafe {_ZNK11QPageLayout14fullRectPointsEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn minimumMargins<T: QPageLayout_minimumMargins>(&mut self, value: T) -> i32 {
    value.minimumMargins(self);
    return 1;
  }
}

pub trait QPageLayout_minimumMargins {
  fn minimumMargins(self, this: &mut QPageLayout) -> i32;
}

// proto: QMarginsF QPageLayout::minimumMargins();
impl<'a> /*trait*/ QPageLayout_minimumMargins for () {
  fn minimumMargins(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14minimumMarginsEv()};
    unsafe {_ZNK11QPageLayout14minimumMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn pageSize<T: QPageLayout_pageSize>(&mut self, value: T) -> i32 {
    value.pageSize(self);
    return 1;
  }
}

pub trait QPageLayout_pageSize {
  fn pageSize(self, this: &mut QPageLayout) -> i32;
}

// proto: QPageSize QPageLayout::pageSize();
impl<'a> /*trait*/ QPageLayout_pageSize for () {
  fn pageSize(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout8pageSizeEv()};
    unsafe {_ZNK11QPageLayout8pageSizeEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setTopMargin<T: QPageLayout_setTopMargin>(&mut self, value: T) -> i32 {
    value.setTopMargin(self);
    return 1;
  }
}

pub trait QPageLayout_setTopMargin {
  fn setTopMargin(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::setTopMargin(qreal topMargin);
impl<'a> /*trait*/ QPageLayout_setTopMargin for (f64) {
  fn setTopMargin(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout12setTopMarginEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN11QPageLayout12setTopMarginEd(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPageLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QPageLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn FreeQPageLayout<T: QPageLayout_FreeQPageLayout>(&mut self, value: T) -> i32 {
    value.FreeQPageLayout(self);
    return 1;
  }
}

pub trait QPageLayout_FreeQPageLayout {
  fn FreeQPageLayout(self, this: &mut QPageLayout) -> i32;
}

// proto: void QPageLayout::FreeQPageLayout();
impl<'a> /*trait*/ QPageLayout_FreeQPageLayout for () {
  fn FreeQPageLayout(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayoutD0Ev()};
    unsafe {_ZN11QPageLayoutD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn fullRectPixels<T: QPageLayout_fullRectPixels>(&mut self, value: T) -> i32 {
    value.fullRectPixels(self);
    return 1;
  }
}

pub trait QPageLayout_fullRectPixels {
  fn fullRectPixels(self, this: &mut QPageLayout) -> i32;
}

// proto: QRect QPageLayout::fullRectPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_fullRectPixels for (i32) {
  fn fullRectPixels(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14fullRectPixelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QPageLayout14fullRectPixelsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn margins<T: QPageLayout_margins>(&mut self, value: T) -> i32 {
    value.margins(self);
    return 1;
  }
}

pub trait QPageLayout_margins {
  fn margins(self, this: &mut QPageLayout) -> i32;
}

// proto: QMarginsF QPageLayout::margins();
impl<'a> /*trait*/ QPageLayout_margins for () {
  fn margins(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout7marginsEv()};
    unsafe {_ZNK11QPageLayout7marginsEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn paintRectPoints<T: QPageLayout_paintRectPoints>(&mut self, value: T) -> i32 {
    value.paintRectPoints(self);
    return 1;
  }
}

pub trait QPageLayout_paintRectPoints {
  fn paintRectPoints(self, this: &mut QPageLayout) -> i32;
}

// proto: QRect QPageLayout::paintRectPoints();
impl<'a> /*trait*/ QPageLayout_paintRectPoints for () {
  fn paintRectPoints(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout15paintRectPointsEv()};
    unsafe {_ZNK11QPageLayout15paintRectPointsEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn paintRectPixels<T: QPageLayout_paintRectPixels>(&mut self, value: T) -> i32 {
    value.paintRectPixels(self);
    return 1;
  }
}

pub trait QPageLayout_paintRectPixels {
  fn paintRectPixels(self, this: &mut QPageLayout) -> i32;
}

// proto: QRect QPageLayout::paintRectPixels(int resolution);
impl<'a> /*trait*/ QPageLayout_paintRectPixels for (i32) {
  fn paintRectPixels(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout15paintRectPixelsEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK11QPageLayout15paintRectPixelsEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setPageSize<T: QPageLayout_setPageSize>(&mut self, value: T) -> i32 {
    value.setPageSize(self);
    return 1;
  }
}

pub trait QPageLayout_setPageSize {
  fn setPageSize(self, this: &mut QPageLayout) -> i32;
}

// proto: void QPageLayout::setPageSize(const QPageSize & pageSize, const QMarginsF & minMargins);
impl<'a> /*trait*/ QPageLayout_setPageSize for (&'a  QPageSize, &'a  QMarginsF) {
  fn setPageSize(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN11QPageLayout11setPageSizeERK9QPageSizeRK9QMarginsF(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn maximumMargins<T: QPageLayout_maximumMargins>(&mut self, value: T) -> i32 {
    value.maximumMargins(self);
    return 1;
  }
}

pub trait QPageLayout_maximumMargins {
  fn maximumMargins(self, this: &mut QPageLayout) -> i32;
}

// proto: QMarginsF QPageLayout::maximumMargins();
impl<'a> /*trait*/ QPageLayout_maximumMargins for () {
  fn maximumMargins(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14maximumMarginsEv()};
    unsafe {_ZNK11QPageLayout14maximumMarginsEv()};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn isEquivalentTo<T: QPageLayout_isEquivalentTo>(&mut self, value: T) -> i32 {
    value.isEquivalentTo(self);
    return 1;
  }
}

pub trait QPageLayout_isEquivalentTo {
  fn isEquivalentTo(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::isEquivalentTo(const QPageLayout & other);
impl<'a> /*trait*/ QPageLayout_isEquivalentTo for (&'a  QPageLayout) {
  fn isEquivalentTo(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPageLayout14isEquivalentToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK11QPageLayout14isEquivalentToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QPageLayout {
  pub fn setMargins<T: QPageLayout_setMargins>(&mut self, value: T) -> i32 {
    value.setMargins(self);
    return 1;
  }
}

pub trait QPageLayout_setMargins {
  fn setMargins(self, this: &mut QPageLayout) -> i32;
}

// proto: bool QPageLayout::setMargins(const QMarginsF & margins);
impl<'a> /*trait*/ QPageLayout_setMargins for (&'a  QMarginsF) {
  fn setMargins(self, this: &mut QPageLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPageLayout10setMarginsERK9QMarginsF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QPageLayout10setMarginsERK9QMarginsF(arg0)};
    return 1;
  }
}

