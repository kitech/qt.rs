// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QVariantComparisonHelper::NewQVariantComparisonHelper(const QVariant & var);
  fn _ZN24QVariantComparisonHelperC1ERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QVariantComparisonHelper)=8
pub struct QVariantComparisonHelper {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVariantComparisonHelper {
  pub fn NewQVariantComparisonHelper<T: QVariantComparisonHelper_NewQVariantComparisonHelper>(value: T) -> QVariantComparisonHelper {
    let rsthis = value.NewQVariantComparisonHelper();
    return rsthis;
    // return 1;
  }
}

pub trait QVariantComparisonHelper_NewQVariantComparisonHelper {
  fn NewQVariantComparisonHelper(self) -> QVariantComparisonHelper;
}

// proto: void QVariantComparisonHelper::NewQVariantComparisonHelper(const QVariant & var);
impl<'a> /*trait*/ QVariantComparisonHelper_NewQVariantComparisonHelper for (&'a  QVariant) {
  fn NewQVariantComparisonHelper(self) -> QVariantComparisonHelper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QVariantComparisonHelperC1ERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QVariantComparisonHelperC1ERK8QVariant(qthis, arg0)};
    let rsthis = QVariantComparisonHelper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

