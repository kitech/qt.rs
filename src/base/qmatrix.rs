// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qpolygon::QPolygon;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QMatrix::dx();
  fn _ZNK7QMatrix2dxEv(qthis: *mut c_void) -> c_double;
  // proto:  void QMatrix::NewQMatrix(bool );
  fn _ZN7QMatrixC1Eb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  double QMatrix::dy();
  fn _ZNK7QMatrix2dyEv(qthis: *mut c_void) -> c_double;
  // proto:  QMatrix & QMatrix::scale(qreal sx, qreal sy);
  fn _ZN7QMatrix5scaleEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QMatrix & QMatrix::translate(qreal dx, qreal dy);
  fn _ZN7QMatrix9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  double QMatrix::determinant();
  fn _ZNK7QMatrix11determinantEv(qthis: *mut c_void) -> c_double;
  // proto:  QMatrix & QMatrix::shear(qreal sh, qreal sv);
  fn _ZN7QMatrix5shearEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QMatrix::NewQMatrix();
  fn _ZN7QMatrixC1Ev(qthis: *mut c_void) ;
  // proto:  double QMatrix::m21();
  fn _ZNK7QMatrix3m21Ev(qthis: *mut c_void) -> c_double;
  // proto:  QMatrix & QMatrix::rotate(qreal a);
  fn _ZN7QMatrix6rotateEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
  fn _ZN7QMatrix9setMatrixEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) ;
  // proto:  void QMatrix::NewQMatrix(const QMatrix & matrix);
  fn _ZN7QMatrixC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMatrix::reset();
  fn _ZN7QMatrix5resetEv(qthis: *mut c_void) ;
  // proto:  double QMatrix::m11();
  fn _ZNK7QMatrix3m11Ev(qthis: *mut c_void) -> c_double;
  // proto:  QPolygon QMatrix::mapToPolygon(const QRect & r);
  fn _ZNK7QMatrix12mapToPolygonERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMatrix QMatrix::inverted(bool * invertible);
  fn _ZNK7QMatrix8invertedEPb(qthis: *mut c_void, arg0: *mut int8_t) -> *mut c_void;
  // proto:  bool QMatrix::isIdentity();
  fn _ZNK7QMatrix10isIdentityEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMatrix::NewQMatrix(qreal am11, qreal am12, qreal am21, qreal am22, qreal adx, qreal ady, bool );
  fn _ZN7QMatrixC1Eddddddb(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double, arg6: int8_t) ;
  // proto:  double QMatrix::m12();
  fn _ZNK7QMatrix3m12Ev(qthis: *mut c_void) -> c_double;
  // proto:  bool QMatrix::isInvertible();
  fn _ZNK7QMatrix12isInvertibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMatrix::NewQMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
  fn _ZN7QMatrixC1Edddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double) ;
  // proto:  double QMatrix::m22();
  fn _ZNK7QMatrix3m22Ev(qthis: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QMatrix)=48
pub struct QMatrix {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMatrix {
  pub fn dx<RetType, T: QMatrix_dx<RetType>>(&mut self, value: T) -> RetType {
    return value.dx(self);
    // return 1;
  }
}

pub trait QMatrix_dx<RetType> {
  fn dx(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::dx();
impl<'a> /*trait*/ QMatrix_dx<f64> for () {
  fn dx(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix2dxEv()};
    let mut ret = unsafe {_ZNK7QMatrix2dxEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn NewQMatrix<T: QMatrix_NewQMatrix>(value: T) -> QMatrix {
    let rsthis = value.NewQMatrix();
    return rsthis;
    // return 1;
  }
}

pub trait QMatrix_NewQMatrix {
  fn NewQMatrix(self) -> QMatrix;
}

// proto: void QMatrix::NewQMatrix(bool );
impl<'a> /*trait*/ QMatrix_NewQMatrix for (i8) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Eb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QMatrixC1Eb(qthis, arg0)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn dy<RetType, T: QMatrix_dy<RetType>>(&mut self, value: T) -> RetType {
    return value.dy(self);
    // return 1;
  }
}

pub trait QMatrix_dy<RetType> {
  fn dy(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::dy();
impl<'a> /*trait*/ QMatrix_dy<f64> for () {
  fn dy(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix2dyEv()};
    let mut ret = unsafe {_ZNK7QMatrix2dyEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn scale<RetType, T: QMatrix_scale<RetType>>(&mut self, value: T) -> RetType {
    return value.scale(self);
    // return 1;
  }
}

pub trait QMatrix_scale<RetType> {
  fn scale(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  QMatrix & QMatrix::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QMatrix_scale<QMatrix> for (f64, f64) {
  fn scale(self, rsthis: &mut QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN7QMatrix5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn translate<RetType, T: QMatrix_translate<RetType>>(&mut self, value: T) -> RetType {
    return value.translate(self);
    // return 1;
  }
}

pub trait QMatrix_translate<RetType> {
  fn translate(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  QMatrix & QMatrix::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_translate<QMatrix> for (f64, f64) {
  fn translate(self, rsthis: &mut QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN7QMatrix9translateEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn determinant<RetType, T: QMatrix_determinant<RetType>>(&mut self, value: T) -> RetType {
    return value.determinant(self);
    // return 1;
  }
}

pub trait QMatrix_determinant<RetType> {
  fn determinant(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::determinant();
impl<'a> /*trait*/ QMatrix_determinant<f64> for () {
  fn determinant(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix11determinantEv()};
    let mut ret = unsafe {_ZNK7QMatrix11determinantEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn shear<RetType, T: QMatrix_shear<RetType>>(&mut self, value: T) -> RetType {
    return value.shear(self);
    // return 1;
  }
}

pub trait QMatrix_shear<RetType> {
  fn shear(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  QMatrix & QMatrix::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QMatrix_shear<QMatrix> for (f64, f64) {
  fn shear(self, rsthis: &mut QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN7QMatrix5shearEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QMatrix::NewQMatrix();
impl<'a> /*trait*/ QMatrix_NewQMatrix for () {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Ev()};
    unsafe {_ZN7QMatrixC1Ev(qthis)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m21<RetType, T: QMatrix_m21<RetType>>(&mut self, value: T) -> RetType {
    return value.m21(self);
    // return 1;
  }
}

pub trait QMatrix_m21<RetType> {
  fn m21(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::m21();
impl<'a> /*trait*/ QMatrix_m21<f64> for () {
  fn m21(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m21Ev()};
    let mut ret = unsafe {_ZNK7QMatrix3m21Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn rotate<RetType, T: QMatrix_rotate<RetType>>(&mut self, value: T) -> RetType {
    return value.rotate(self);
    // return 1;
  }
}

pub trait QMatrix_rotate<RetType> {
  fn rotate(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  QMatrix & QMatrix::rotate(qreal a);
impl<'a> /*trait*/ QMatrix_rotate<QMatrix> for (f64) {
  fn rotate(self, rsthis: &mut QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix6rotateEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZN7QMatrix6rotateEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn setMatrix<RetType, T: QMatrix_setMatrix<RetType>>(&mut self, value: T) -> RetType {
    return value.setMatrix(self);
    // return 1;
  }
}

pub trait QMatrix_setMatrix<RetType> {
  fn setMatrix(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  void QMatrix::setMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_setMatrix<()> for (f64, f64, f64, f64, f64, f64) {
  fn setMatrix(self, rsthis: &mut QMatrix) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix9setMatrixEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
     unsafe {_ZN7QMatrix9setMatrixEdddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

// proto: void QMatrix::NewQMatrix(const QMatrix & matrix);
impl<'a> /*trait*/ QMatrix_NewQMatrix for (&'a  QMatrix) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QMatrixC1ERKS_(qthis, arg0)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn reset<RetType, T: QMatrix_reset<RetType>>(&mut self, value: T) -> RetType {
    return value.reset(self);
    // return 1;
  }
}

pub trait QMatrix_reset<RetType> {
  fn reset(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  void QMatrix::reset();
impl<'a> /*trait*/ QMatrix_reset<()> for () {
  fn reset(self, rsthis: &mut QMatrix) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrix5resetEv()};
     unsafe {_ZN7QMatrix5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m11<RetType, T: QMatrix_m11<RetType>>(&mut self, value: T) -> RetType {
    return value.m11(self);
    // return 1;
  }
}

pub trait QMatrix_m11<RetType> {
  fn m11(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::m11();
impl<'a> /*trait*/ QMatrix_m11<f64> for () {
  fn m11(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m11Ev()};
    let mut ret = unsafe {_ZNK7QMatrix3m11Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn mapToPolygon<RetType, T: QMatrix_mapToPolygon<RetType>>(&mut self, value: T) -> RetType {
    return value.mapToPolygon(self);
    // return 1;
  }
}

pub trait QMatrix_mapToPolygon<RetType> {
  fn mapToPolygon(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  QPolygon QMatrix::mapToPolygon(const QRect & r);
impl<'a> /*trait*/ QMatrix_mapToPolygon<QPolygon> for (&'a  QRect) {
  fn mapToPolygon(self, rsthis: &mut QMatrix) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix12mapToPolygonERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QMatrix12mapToPolygonERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn inverted<RetType, T: QMatrix_inverted<RetType>>(&mut self, value: T) -> RetType {
    return value.inverted(self);
    // return 1;
  }
}

pub trait QMatrix_inverted<RetType> {
  fn inverted(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  QMatrix QMatrix::inverted(bool * invertible);
impl<'a> /*trait*/ QMatrix_inverted<QMatrix> for (&'a mut i8) {
  fn inverted(self, rsthis: &mut QMatrix) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix8invertedEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK7QMatrix8invertedEPb(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn isIdentity<RetType, T: QMatrix_isIdentity<RetType>>(&mut self, value: T) -> RetType {
    return value.isIdentity(self);
    // return 1;
  }
}

pub trait QMatrix_isIdentity<RetType> {
  fn isIdentity(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  bool QMatrix::isIdentity();
impl<'a> /*trait*/ QMatrix_isIdentity<i8> for () {
  fn isIdentity(self, rsthis: &mut QMatrix) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix10isIdentityEv()};
    let mut ret = unsafe {_ZNK7QMatrix10isIdentityEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QMatrix::NewQMatrix(qreal am11, qreal am12, qreal am21, qreal am22, qreal adx, qreal ady, bool );
impl<'a> /*trait*/ QMatrix_NewQMatrix for (f64, f64, f64, f64, f64, f64, i8) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Eddddddb()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    let arg6 = self.6  as int8_t;
    unsafe {_ZN7QMatrixC1Eddddddb(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m12<RetType, T: QMatrix_m12<RetType>>(&mut self, value: T) -> RetType {
    return value.m12(self);
    // return 1;
  }
}

pub trait QMatrix_m12<RetType> {
  fn m12(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::m12();
impl<'a> /*trait*/ QMatrix_m12<f64> for () {
  fn m12(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m12Ev()};
    let mut ret = unsafe {_ZNK7QMatrix3m12Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn isInvertible<RetType, T: QMatrix_isInvertible<RetType>>(&mut self, value: T) -> RetType {
    return value.isInvertible(self);
    // return 1;
  }
}

pub trait QMatrix_isInvertible<RetType> {
  fn isInvertible(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  bool QMatrix::isInvertible();
impl<'a> /*trait*/ QMatrix_isInvertible<i8> for () {
  fn isInvertible(self, rsthis: &mut QMatrix) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix12isInvertibleEv()};
    let mut ret = unsafe {_ZNK7QMatrix12isInvertibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QMatrix::NewQMatrix(qreal m11, qreal m12, qreal m21, qreal m22, qreal dx, qreal dy);
impl<'a> /*trait*/ QMatrix_NewQMatrix for (f64, f64, f64, f64, f64, f64) {
  fn NewQMatrix(self) -> QMatrix {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN7QMatrixC1Edddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
    unsafe {_ZN7QMatrixC1Edddddd(qthis, arg0, arg1, arg2, arg3, arg4, arg5)};
    let rsthis = QMatrix{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMatrix {
  pub fn m22<RetType, T: QMatrix_m22<RetType>>(&mut self, value: T) -> RetType {
    return value.m22(self);
    // return 1;
  }
}

pub trait QMatrix_m22<RetType> {
  fn m22(self, rsthis: &mut QMatrix) -> RetType;
}

// proto:  double QMatrix::m22();
impl<'a> /*trait*/ QMatrix_m22<f64> for () {
  fn m22(self, rsthis: &mut QMatrix) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK7QMatrix3m22Ev()};
    let mut ret = unsafe {_ZNK7QMatrix3m22Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

