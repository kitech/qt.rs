// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qsize::QSize;
use super::qlinef::QLineF;
use super::qlocale::QLocale;
use super::qpersistentmodelindex::QPersistentModelIndex;
use super::qdatastream::QDataStream;
use super::quuid::QUuid;
use super::qregexp::QRegExp;
use super::qrectf::QRectF;
use super::qrect::QRect;
use super::qsizef::QSizeF;
use super::qdate::QDate;
use super::qbitarray::QBitArray;
use super::qmodelindex::QModelIndex;
use super::qline::QLine;
use super::qtime::QTime;
use super::qdatetime::QDateTime;
use super::qchar::QChar;
use super::qeasingcurve::QEasingCurve;
use super::qbytearray::QByteArray;
use super::qurl::QUrl;
use super::qregularexpression::QRegularExpression;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK8QVariant8toDoubleEPb(arg0: *mut int8_t) -> i32;
  fn _ZN8QVariantC1EPKc(qthis: *mut c_void, arg0: *const c_char) -> i32;
  fn _ZNK8QVariant10toLongLongEPb(arg0: *mut int8_t) -> i32;
  fn _ZN8QVariantC1ERK7QPointF(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1ERK6QPoint(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant6toSizeEv() -> i32;
  fn _ZNK8QVariant8toStringEv() -> i32;
  fn _ZNK8QVariant6toRealEPb(arg0: *mut int8_t) -> i32;
  fn _ZNK8QVariant7toFloatEPb(arg0: *mut int8_t) -> i32;
  fn _ZN8QVariantC1ERK7QString(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant11toByteArrayEv() -> i32;
  fn _ZNK8QVariant8toLocaleEv() -> i32;
  fn _ZNK8QVariant5toUrlEv() -> i32;
  fn _ZNK8QVariant6toLineEv() -> i32;
  fn _ZN8QVariantC1ERK5QSize(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1ERK6QLineF(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant8typeNameEv() -> i32;
  fn _ZNK8QVariant11toJsonArrayEv() -> i32;
  fn _ZN8QVariantC1ERK7QLocale(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant12toStringListEv() -> i32;
  fn _ZNK8QVariant6toListEv() -> i32;
  fn _ZNK8QVariant6toUIntEPb(arg0: *mut int8_t) -> i32;
  fn _ZNK8QVariant6toUuidEv() -> i32;
  fn _ZN8QVariantC1ERK21QPersistentModelIndex(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant14toJsonDocumentEv() -> i32;
  fn _ZN8QVariantC1ER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZNK8QVariant7toPointEv() -> i32;
  fn _ZNK8QVariant5toIntEPb(arg0: *mut int8_t) -> i32;
  fn _ZNK8QVariant7isValidEv() -> i32;
  fn _ZN8QVariantC1ERK5QUuid(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariant6detachEv() -> i32;
  fn _ZN8QVariantC1ERK7QRegExp(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant12toModelIndexEv() -> i32;
  fn _ZNK8QVariant6toHashEv() -> i32;
  fn _ZNK8QVariant5toMapEv() -> i32;
  fn _ZNK8QVariant10canConvertEi(arg0: c_int) -> i32;
  fn _ZN8QVariantC1ERK6QRectF(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1ERK5QRect(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant8userTypeEv() -> i32;
  fn _ZNK8QVariant9constDataEv() -> i32;
  fn _ZN8QVariantC1EPv(qthis: *mut c_void, arg0: *mut uint8_t) -> i32;
  fn _ZNK8QVariant22toPersistentModelIndexEv() -> i32;
  fn _ZN8QVariantC1EiPKvj(qthis: *mut c_void, arg0: c_int, arg1: *const uint8_t, arg2: c_uint) -> i32;
  fn _ZNK8QVariant7toLineFEv() -> i32;
  fn _ZNK8QVariant12toJsonObjectEv() -> i32;
  fn _ZN8QVariant4loadER11QDataStream(arg0: *mut c_void) -> i32;
  fn _ZN8QVariantC1ERK6QSizeF(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant6toCharEv() -> i32;
  fn _ZNK8QVariant6isNullEv() -> i32;
  fn _ZN8QVariantC1ERK5QDate(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant7toRectFEv() -> i32;
  fn _ZN8QVariantC1ERK9QBitArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant6toDateEv() -> i32;
  fn _ZN8QVariantC1ERK11QModelIndex(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantD0Ev() -> i32;
  fn _ZNK8QVariant4saveER11QDataStream(arg0: *mut c_void) -> i32;
  fn _ZNK8QVariant6toTimeEv() -> i32;
  fn _ZN8QVariantC1ERK5QLine(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariant4dataEv() -> i32;
  fn _ZN8QVariantC1ERK5QTime(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1ERK9QDateTime(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariant7convertEi(arg0: c_int) -> i32;
  fn _ZNK8QVariant8toRegExpEv() -> i32;
  fn _ZNK8QVariant8toPointFEv() -> i32;
  fn _ZN8QVariantC1E5QChar(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN8QVariant10typeToNameEi(arg0: c_int) -> i32;
  fn _ZNK8QVariant7toSizeFEv() -> i32;
  fn _ZN8QVariant4swapERS_(arg0: *mut c_void) -> i32;
  fn _ZN8QVariantC1EiPKv(qthis: *mut c_void, arg0: c_int, arg1: *const uint8_t) -> i32;
  fn _ZN8QVariantC1ERK12QEasingCurve(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariant5clearEv() -> i32;
  fn _ZNK8QVariant6toRectEv() -> i32;
  fn _ZN8QVariantC1ERK10QByteArray(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1Ex(qthis: *mut c_void, arg0: c_longlong) -> i32;
  fn _ZN8QVariantC1Ey(qthis: *mut c_void, arg0: uint64_t) -> i32;
  fn _ZN8QVariantC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK8QVariant6toBoolEv() -> i32;
  fn _ZN8QVariantC1Ej(qthis: *mut c_void, arg0: c_uint) -> i32;
  fn _ZN8QVariantC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  fn _ZN8QVariantC1Ef(qthis: *mut c_void, arg0: c_float) -> i32;
  fn _ZN8QVariantC1Ed(qthis: *mut c_void, arg0: c_double) -> i32;
  fn _ZN8QVariantC1Eb(qthis: *mut c_void, arg0: int8_t) -> i32;
  fn _ZNK8QVariant11toULongLongEPb(arg0: *mut int8_t) -> i32;
  fn _ZNK8QVariant11toJsonValueEv() -> i32;
  fn _ZNK8QVariant10toDateTimeEv() -> i32;
  fn _ZNK8QVariant10isDetachedEv() -> i32;
  fn _ZNK8QVariant13toEasingCurveEv() -> i32;
  fn _ZN8QVariantC1ERK4QUrl(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK8QVariant10toBitArrayEv() -> i32;
  fn _ZNK8QVariant19toRegularExpressionEv() -> i32;
  fn _ZN8QVariantC1ERK18QRegularExpression(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN8QVariantC1ERK11QStringList(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QVariant)=16
pub struct QVariant {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVariant {
  pub fn toDouble<T: QVariant_toDouble>(&mut self, value: T) -> i32 {
    value.toDouble(self);
    return 1;
  }
}

pub trait QVariant_toDouble {
  fn toDouble(self, this: &mut QVariant) -> i32;
}

// proto: double QVariant::toDouble(bool * ok);
impl<'a> /*trait*/ QVariant_toDouble for (&'a mut i8) {
  fn toDouble(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toDoubleEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant8toDoubleEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn NewQVariant<T: QVariant_NewQVariant>(value: T) -> QVariant {
    let rsthis = value.NewQVariant();
    return rsthis;
    // return 1;
  }
}

pub trait QVariant_NewQVariant {
  fn NewQVariant(self) -> QVariant;
}

// proto: void QVariant::NewQVariant(const char * str);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  String) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN8QVariantC1EPKc(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toLongLong<T: QVariant_toLongLong>(&mut self, value: T) -> i32 {
    value.toLongLong(self);
    return 1;
  }
}

pub trait QVariant_toLongLong {
  fn toLongLong(self, this: &mut QVariant) -> i32;
}

// proto: qint64 QVariant::toLongLong(bool * ok);
impl<'a> /*trait*/ QVariant_toLongLong for (&'a mut i8) {
  fn toLongLong(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toLongLongEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant10toLongLongEPb(arg0)};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QPointF & pt);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QPointF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK7QPointF(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QPoint & pt);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QPoint) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK6QPoint(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toSize<T: QVariant_toSize>(&mut self, value: T) -> i32 {
    value.toSize(self);
    return 1;
  }
}

pub trait QVariant_toSize {
  fn toSize(self, this: &mut QVariant) -> i32;
}

// proto: QSize QVariant::toSize();
impl<'a> /*trait*/ QVariant_toSize for () {
  fn toSize(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toSizeEv()};
    unsafe {_ZNK8QVariant6toSizeEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toString<T: QVariant_toString>(&mut self, value: T) -> i32 {
    value.toString(self);
    return 1;
  }
}

pub trait QVariant_toString {
  fn toString(self, this: &mut QVariant) -> i32;
}

// proto: QString QVariant::toString();
impl<'a> /*trait*/ QVariant_toString for () {
  fn toString(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toStringEv()};
    unsafe {_ZNK8QVariant8toStringEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toReal<T: QVariant_toReal>(&mut self, value: T) -> i32 {
    value.toReal(self);
    return 1;
  }
}

pub trait QVariant_toReal {
  fn toReal(self, this: &mut QVariant) -> i32;
}

// proto: double QVariant::toReal(bool * ok);
impl<'a> /*trait*/ QVariant_toReal for (&'a mut i8) {
  fn toReal(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toRealEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant6toRealEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toFloat<T: QVariant_toFloat>(&mut self, value: T) -> i32 {
    value.toFloat(self);
    return 1;
  }
}

pub trait QVariant_toFloat {
  fn toFloat(self, this: &mut QVariant) -> i32;
}

// proto: float QVariant::toFloat(bool * ok);
impl<'a> /*trait*/ QVariant_toFloat for (&'a mut i8) {
  fn toFloat(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toFloatEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant7toFloatEPb(arg0)};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QString & string);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QString) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK7QString(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toByteArray<T: QVariant_toByteArray>(&mut self, value: T) -> i32 {
    value.toByteArray(self);
    return 1;
  }
}

pub trait QVariant_toByteArray {
  fn toByteArray(self, this: &mut QVariant) -> i32;
}

// proto: QByteArray QVariant::toByteArray();
impl<'a> /*trait*/ QVariant_toByteArray for () {
  fn toByteArray(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toByteArrayEv()};
    unsafe {_ZNK8QVariant11toByteArrayEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toLocale<T: QVariant_toLocale>(&mut self, value: T) -> i32 {
    value.toLocale(self);
    return 1;
  }
}

pub trait QVariant_toLocale {
  fn toLocale(self, this: &mut QVariant) -> i32;
}

// proto: QLocale QVariant::toLocale();
impl<'a> /*trait*/ QVariant_toLocale for () {
  fn toLocale(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toLocaleEv()};
    unsafe {_ZNK8QVariant8toLocaleEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toUrl<T: QVariant_toUrl>(&mut self, value: T) -> i32 {
    value.toUrl(self);
    return 1;
  }
}

pub trait QVariant_toUrl {
  fn toUrl(self, this: &mut QVariant) -> i32;
}

// proto: QUrl QVariant::toUrl();
impl<'a> /*trait*/ QVariant_toUrl for () {
  fn toUrl(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toUrlEv()};
    unsafe {_ZNK8QVariant5toUrlEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toLine<T: QVariant_toLine>(&mut self, value: T) -> i32 {
    value.toLine(self);
    return 1;
  }
}

pub trait QVariant_toLine {
  fn toLine(self, this: &mut QVariant) -> i32;
}

// proto: QLine QVariant::toLine();
impl<'a> /*trait*/ QVariant_toLine for () {
  fn toLine(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toLineEv()};
    unsafe {_ZNK8QVariant6toLineEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QSize & size);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QSize) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK5QSize(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QLineF & line);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QLineF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK6QLineF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK6QLineF(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn typeName<T: QVariant_typeName>(&mut self, value: T) -> i32 {
    value.typeName(self);
    return 1;
  }
}

pub trait QVariant_typeName {
  fn typeName(self, this: &mut QVariant) -> i32;
}

// proto: const char * QVariant::typeName();
impl<'a> /*trait*/ QVariant_typeName for () {
  fn typeName(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8typeNameEv()};
    unsafe {_ZNK8QVariant8typeNameEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonArray<T: QVariant_toJsonArray>(&mut self, value: T) -> i32 {
    value.toJsonArray(self);
    return 1;
  }
}

pub trait QVariant_toJsonArray {
  fn toJsonArray(self, this: &mut QVariant) -> i32;
}

// proto: QJsonArray QVariant::toJsonArray();
impl<'a> /*trait*/ QVariant_toJsonArray for () {
  fn toJsonArray(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toJsonArrayEv()};
    unsafe {_ZNK8QVariant11toJsonArrayEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QLocale & locale);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QLocale) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QLocale()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK7QLocale(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toStringList<T: QVariant_toStringList>(&mut self, value: T) -> i32 {
    value.toStringList(self);
    return 1;
  }
}

pub trait QVariant_toStringList {
  fn toStringList(self, this: &mut QVariant) -> i32;
}

// proto: QStringList QVariant::toStringList();
impl<'a> /*trait*/ QVariant_toStringList for () {
  fn toStringList(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toStringListEv()};
    unsafe {_ZNK8QVariant12toStringListEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toList<T: QVariant_toList>(&mut self, value: T) -> i32 {
    value.toList(self);
    return 1;
  }
}

pub trait QVariant_toList {
  fn toList(self, this: &mut QVariant) -> i32;
}

// proto: QList<QVariant> QVariant::toList();
impl<'a> /*trait*/ QVariant_toList for () {
  fn toList(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toListEv()};
    unsafe {_ZNK8QVariant6toListEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toUInt<T: QVariant_toUInt>(&mut self, value: T) -> i32 {
    value.toUInt(self);
    return 1;
  }
}

pub trait QVariant_toUInt {
  fn toUInt(self, this: &mut QVariant) -> i32;
}

// proto: unsigned int QVariant::toUInt(bool * ok);
impl<'a> /*trait*/ QVariant_toUInt for (&'a mut i8) {
  fn toUInt(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toUIntEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant6toUIntEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toUuid<T: QVariant_toUuid>(&mut self, value: T) -> i32 {
    value.toUuid(self);
    return 1;
  }
}

pub trait QVariant_toUuid {
  fn toUuid(self, this: &mut QVariant) -> i32;
}

// proto: QUuid QVariant::toUuid();
impl<'a> /*trait*/ QVariant_toUuid for () {
  fn toUuid(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toUuidEv()};
    unsafe {_ZNK8QVariant6toUuidEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QPersistentModelIndex & modelIndex);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QPersistentModelIndex) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK21QPersistentModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK21QPersistentModelIndex(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonDocument<T: QVariant_toJsonDocument>(&mut self, value: T) -> i32 {
    value.toJsonDocument(self);
    return 1;
  }
}

pub trait QVariant_toJsonDocument {
  fn toJsonDocument(self, this: &mut QVariant) -> i32;
}

// proto: QJsonDocument QVariant::toJsonDocument();
impl<'a> /*trait*/ QVariant_toJsonDocument for () {
  fn toJsonDocument(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant14toJsonDocumentEv()};
    unsafe {_ZNK8QVariant14toJsonDocumentEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(QDataStream & s);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a mut QDataStream) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ER11QDataStream(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toPoint<T: QVariant_toPoint>(&mut self, value: T) -> i32 {
    value.toPoint(self);
    return 1;
  }
}

pub trait QVariant_toPoint {
  fn toPoint(self, this: &mut QVariant) -> i32;
}

// proto: QPoint QVariant::toPoint();
impl<'a> /*trait*/ QVariant_toPoint for () {
  fn toPoint(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toPointEv()};
    unsafe {_ZNK8QVariant7toPointEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toInt<T: QVariant_toInt>(&mut self, value: T) -> i32 {
    value.toInt(self);
    return 1;
  }
}

pub trait QVariant_toInt {
  fn toInt(self, this: &mut QVariant) -> i32;
}

// proto: int QVariant::toInt(bool * ok);
impl<'a> /*trait*/ QVariant_toInt for (&'a mut i8) {
  fn toInt(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toIntEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant5toIntEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn isValid<T: QVariant_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QVariant_isValid {
  fn isValid(self, this: &mut QVariant) -> i32;
}

// proto: bool QVariant::isValid();
impl<'a> /*trait*/ QVariant_isValid for () {
  fn isValid(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7isValidEv()};
    unsafe {_ZNK8QVariant7isValidEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QUuid & uuid);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QUuid) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QUuid()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK5QUuid(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn detach<T: QVariant_detach>(&mut self, value: T) -> i32 {
    value.detach(self);
    return 1;
  }
}

pub trait QVariant_detach {
  fn detach(self, this: &mut QVariant) -> i32;
}

// proto: void QVariant::detach();
impl<'a> /*trait*/ QVariant_detach for () {
  fn detach(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant6detachEv()};
    unsafe {_ZN8QVariant6detachEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRegExp & regExp);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRegExp) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QRegExp()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK7QRegExp(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toModelIndex<T: QVariant_toModelIndex>(&mut self, value: T) -> i32 {
    value.toModelIndex(self);
    return 1;
  }
}

pub trait QVariant_toModelIndex {
  fn toModelIndex(self, this: &mut QVariant) -> i32;
}

// proto: QModelIndex QVariant::toModelIndex();
impl<'a> /*trait*/ QVariant_toModelIndex for () {
  fn toModelIndex(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toModelIndexEv()};
    unsafe {_ZNK8QVariant12toModelIndexEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toHash<T: QVariant_toHash>(&mut self, value: T) -> i32 {
    value.toHash(self);
    return 1;
  }
}

pub trait QVariant_toHash {
  fn toHash(self, this: &mut QVariant) -> i32;
}

// proto: QHash<QString, QVariant> QVariant::toHash();
impl<'a> /*trait*/ QVariant_toHash for () {
  fn toHash(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toHashEv()};
    unsafe {_ZNK8QVariant6toHashEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toMap<T: QVariant_toMap>(&mut self, value: T) -> i32 {
    value.toMap(self);
    return 1;
  }
}

pub trait QVariant_toMap {
  fn toMap(self, this: &mut QVariant) -> i32;
}

// proto: QMap<QString, QVariant> QVariant::toMap();
impl<'a> /*trait*/ QVariant_toMap for () {
  fn toMap(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toMapEv()};
    unsafe {_ZNK8QVariant5toMapEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn canConvert<T: QVariant_canConvert>(&mut self, value: T) -> i32 {
    value.canConvert(self);
    return 1;
  }
}

pub trait QVariant_canConvert {
  fn canConvert(self, this: &mut QVariant) -> i32;
}

// proto: bool QVariant::canConvert(int targetTypeId);
impl<'a> /*trait*/ QVariant_canConvert for (i32) {
  fn canConvert(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10canConvertEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK8QVariant10canConvertEi(arg0)};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRectF & rect);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRectF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK6QRectF(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRect & rect);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRect) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK5QRect(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn userType<T: QVariant_userType>(&mut self, value: T) -> i32 {
    value.userType(self);
    return 1;
  }
}

pub trait QVariant_userType {
  fn userType(self, this: &mut QVariant) -> i32;
}

// proto: int QVariant::userType();
impl<'a> /*trait*/ QVariant_userType for () {
  fn userType(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8userTypeEv()};
    unsafe {_ZNK8QVariant8userTypeEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn constData<T: QVariant_constData>(&mut self, value: T) -> i32 {
    value.constData(self);
    return 1;
  }
}

pub trait QVariant_constData {
  fn constData(self, this: &mut QVariant) -> i32;
}

// proto: const void * QVariant::constData();
impl<'a> /*trait*/ QVariant_constData for () {
  fn constData(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant9constDataEv()};
    unsafe {_ZNK8QVariant9constDataEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(void * );
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a mut u8) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EPv()};
    let arg0 = self  as *mut uint8_t;
    unsafe {_ZN8QVariantC1EPv(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toPersistentModelIndex<T: QVariant_toPersistentModelIndex>(&mut self, value: T) -> i32 {
    value.toPersistentModelIndex(self);
    return 1;
  }
}

pub trait QVariant_toPersistentModelIndex {
  fn toPersistentModelIndex(self, this: &mut QVariant) -> i32;
}

// proto: QPersistentModelIndex QVariant::toPersistentModelIndex();
impl<'a> /*trait*/ QVariant_toPersistentModelIndex for () {
  fn toPersistentModelIndex(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant22toPersistentModelIndexEv()};
    unsafe {_ZNK8QVariant22toPersistentModelIndexEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(int typeId, const void * copy, uint flags);
impl<'a> /*trait*/ QVariant_NewQVariant for (i32, &'a  u8, u32) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EiPKvj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const uint8_t;
    let arg2 = self.2  as c_uint;
    unsafe {_ZN8QVariantC1EiPKvj(qthis, arg0, arg1, arg2)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toLineF<T: QVariant_toLineF>(&mut self, value: T) -> i32 {
    value.toLineF(self);
    return 1;
  }
}

pub trait QVariant_toLineF {
  fn toLineF(self, this: &mut QVariant) -> i32;
}

// proto: QLineF QVariant::toLineF();
impl<'a> /*trait*/ QVariant_toLineF for () {
  fn toLineF(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toLineFEv()};
    unsafe {_ZNK8QVariant7toLineFEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonObject<T: QVariant_toJsonObject>(&mut self, value: T) -> i32 {
    value.toJsonObject(self);
    return 1;
  }
}

pub trait QVariant_toJsonObject {
  fn toJsonObject(self, this: &mut QVariant) -> i32;
}

// proto: QJsonObject QVariant::toJsonObject();
impl<'a> /*trait*/ QVariant_toJsonObject for () {
  fn toJsonObject(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toJsonObjectEv()};
    unsafe {_ZNK8QVariant12toJsonObjectEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn load<T: QVariant_load>(&mut self, value: T) -> i32 {
    value.load(self);
    return 1;
  }
}

pub trait QVariant_load {
  fn load(self, this: &mut QVariant) -> i32;
}

// proto: void QVariant::load(QDataStream & ds);
impl<'a> /*trait*/ QVariant_load for (&'a mut QDataStream) {
  fn load(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4loadER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariant4loadER11QDataStream(arg0)};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QSizeF & size);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QSizeF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK6QSizeF(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toChar<T: QVariant_toChar>(&mut self, value: T) -> i32 {
    value.toChar(self);
    return 1;
  }
}

pub trait QVariant_toChar {
  fn toChar(self, this: &mut QVariant) -> i32;
}

// proto: QChar QVariant::toChar();
impl<'a> /*trait*/ QVariant_toChar for () {
  fn toChar(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toCharEv()};
    unsafe {_ZNK8QVariant6toCharEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn isNull<T: QVariant_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QVariant_isNull {
  fn isNull(self, this: &mut QVariant) -> i32;
}

// proto: bool QVariant::isNull();
impl<'a> /*trait*/ QVariant_isNull for () {
  fn isNull(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6isNullEv()};
    unsafe {_ZNK8QVariant6isNullEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QDate & date);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QDate) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QDate()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK5QDate(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRectF<T: QVariant_toRectF>(&mut self, value: T) -> i32 {
    value.toRectF(self);
    return 1;
  }
}

pub trait QVariant_toRectF {
  fn toRectF(self, this: &mut QVariant) -> i32;
}

// proto: QRectF QVariant::toRectF();
impl<'a> /*trait*/ QVariant_toRectF for () {
  fn toRectF(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toRectFEv()};
    unsafe {_ZNK8QVariant7toRectFEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QBitArray & bitarray);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QBitArray) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK9QBitArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK9QBitArray(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toDate<T: QVariant_toDate>(&mut self, value: T) -> i32 {
    value.toDate(self);
    return 1;
  }
}

pub trait QVariant_toDate {
  fn toDate(self, this: &mut QVariant) -> i32;
}

// proto: QDate QVariant::toDate();
impl<'a> /*trait*/ QVariant_toDate for () {
  fn toDate(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toDateEv()};
    unsafe {_ZNK8QVariant6toDateEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QModelIndex & modelIndex);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QModelIndex) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn FreeQVariant<T: QVariant_FreeQVariant>(&mut self, value: T) -> i32 {
    value.FreeQVariant(self);
    return 1;
  }
}

pub trait QVariant_FreeQVariant {
  fn FreeQVariant(self, this: &mut QVariant) -> i32;
}

// proto: void QVariant::FreeQVariant();
impl<'a> /*trait*/ QVariant_FreeQVariant for () {
  fn FreeQVariant(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantD0Ev()};
    unsafe {_ZN8QVariantD0Ev()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn save<T: QVariant_save>(&mut self, value: T) -> i32 {
    value.save(self);
    return 1;
  }
}

pub trait QVariant_save {
  fn save(self, this: &mut QVariant) -> i32;
}

// proto: void QVariant::save(QDataStream & ds);
impl<'a> /*trait*/ QVariant_save for (&'a mut QDataStream) {
  fn save(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant4saveER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK8QVariant4saveER11QDataStream(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toTime<T: QVariant_toTime>(&mut self, value: T) -> i32 {
    value.toTime(self);
    return 1;
  }
}

pub trait QVariant_toTime {
  fn toTime(self, this: &mut QVariant) -> i32;
}

// proto: QTime QVariant::toTime();
impl<'a> /*trait*/ QVariant_toTime for () {
  fn toTime(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toTimeEv()};
    unsafe {_ZNK8QVariant6toTimeEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QLine & line);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QLine) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QLine()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK5QLine(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn data<T: QVariant_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QVariant_data {
  fn data(self, this: &mut QVariant) -> i32;
}

// proto: void * QVariant::data();
impl<'a> /*trait*/ QVariant_data for () {
  fn data(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4dataEv()};
    unsafe {_ZN8QVariant4dataEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QTime & time);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QTime) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK5QTime(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QDateTime & datetime);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QDateTime) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK9QDateTime()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK9QDateTime(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn convert<T: QVariant_convert>(&mut self, value: T) -> i32 {
    value.convert(self);
    return 1;
  }
}

pub trait QVariant_convert {
  fn convert(self, this: &mut QVariant) -> i32;
}

// proto: bool QVariant::convert(int targetTypeId);
impl<'a> /*trait*/ QVariant_convert for (i32) {
  fn convert(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant7convertEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QVariant7convertEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRegExp<T: QVariant_toRegExp>(&mut self, value: T) -> i32 {
    value.toRegExp(self);
    return 1;
  }
}

pub trait QVariant_toRegExp {
  fn toRegExp(self, this: &mut QVariant) -> i32;
}

// proto: QRegExp QVariant::toRegExp();
impl<'a> /*trait*/ QVariant_toRegExp for () {
  fn toRegExp(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toRegExpEv()};
    unsafe {_ZNK8QVariant8toRegExpEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toPointF<T: QVariant_toPointF>(&mut self, value: T) -> i32 {
    value.toPointF(self);
    return 1;
  }
}

pub trait QVariant_toPointF {
  fn toPointF(self, this: &mut QVariant) -> i32;
}

// proto: QPointF QVariant::toPointF();
impl<'a> /*trait*/ QVariant_toPointF for () {
  fn toPointF(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toPointFEv()};
    unsafe {_ZNK8QVariant8toPointFEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(QChar qchar);
impl<'a> /*trait*/ QVariant_NewQVariant for (QChar) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1E5QChar()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1E5QChar(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn typeToName<T: QVariant_typeToName>(&mut self, value: T) -> i32 {
    value.typeToName(self);
    return 1;
  }
}

pub trait QVariant_typeToName {
  fn typeToName(self, this: &mut QVariant) -> i32;
}

// proto: const char * QVariant::typeToName(int typeId);
impl<'a> /*trait*/ QVariant_typeToName for (i32) {
  fn typeToName(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant10typeToNameEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QVariant10typeToNameEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toSizeF<T: QVariant_toSizeF>(&mut self, value: T) -> i32 {
    value.toSizeF(self);
    return 1;
  }
}

pub trait QVariant_toSizeF {
  fn toSizeF(self, this: &mut QVariant) -> i32;
}

// proto: QSizeF QVariant::toSizeF();
impl<'a> /*trait*/ QVariant_toSizeF for () {
  fn toSizeF(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toSizeFEv()};
    unsafe {_ZNK8QVariant7toSizeFEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn swap<T: QVariant_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QVariant_swap {
  fn swap(self, this: &mut QVariant) -> i32;
}

// proto: void QVariant::swap(QVariant & other);
impl<'a> /*trait*/ QVariant_swap for (&'a mut QVariant) {
  fn swap(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariant4swapERS_(arg0)};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(int typeId, const void * copy);
impl<'a> /*trait*/ QVariant_NewQVariant for (i32, &'a  u8) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EiPKv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const uint8_t;
    unsafe {_ZN8QVariantC1EiPKv(qthis, arg0, arg1)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QEasingCurve & easing);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QEasingCurve) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK12QEasingCurve()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK12QEasingCurve(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn clear<T: QVariant_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QVariant_clear {
  fn clear(self, this: &mut QVariant) -> i32;
}

// proto: void QVariant::clear();
impl<'a> /*trait*/ QVariant_clear for () {
  fn clear(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant5clearEv()};
    unsafe {_ZN8QVariant5clearEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRect<T: QVariant_toRect>(&mut self, value: T) -> i32 {
    value.toRect(self);
    return 1;
  }
}

pub trait QVariant_toRect {
  fn toRect(self, this: &mut QVariant) -> i32;
}

// proto: QRect QVariant::toRect();
impl<'a> /*trait*/ QVariant_toRect for () {
  fn toRect(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toRectEv()};
    unsafe {_ZNK8QVariant6toRectEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QByteArray & bytearray);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QByteArray) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(qlonglong ll);
impl<'a> /*trait*/ QVariant_NewQVariant for (i64) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ex()};
    let arg0 = self  as c_longlong;
    unsafe {_ZN8QVariantC1Ex(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(qulonglong ull);
impl<'a> /*trait*/ QVariant_NewQVariant for (u64) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ey()};
    let arg0 = self  as uint64_t;
    unsafe {_ZN8QVariantC1Ey(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant();
impl<'a> /*trait*/ QVariant_NewQVariant for () {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ev()};
    unsafe {_ZN8QVariantC1Ev(qthis)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toBool<T: QVariant_toBool>(&mut self, value: T) -> i32 {
    value.toBool(self);
    return 1;
  }
}

pub trait QVariant_toBool {
  fn toBool(self, this: &mut QVariant) -> i32;
}

// proto: bool QVariant::toBool();
impl<'a> /*trait*/ QVariant_toBool for () {
  fn toBool(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toBoolEv()};
    unsafe {_ZNK8QVariant6toBoolEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(uint ui);
impl<'a> /*trait*/ QVariant_NewQVariant for (u32) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ej()};
    let arg0 = self  as c_uint;
    unsafe {_ZN8QVariantC1Ej(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(int i);
impl<'a> /*trait*/ QVariant_NewQVariant for (i32) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QVariantC1Ei(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(float f);
impl<'a> /*trait*/ QVariant_NewQVariant for (f32) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ef()};
    let arg0 = self  as c_float;
    unsafe {_ZN8QVariantC1Ef(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(double d);
impl<'a> /*trait*/ QVariant_NewQVariant for (f64) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ed()};
    let arg0 = self  as c_double;
    unsafe {_ZN8QVariantC1Ed(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(bool b);
impl<'a> /*trait*/ QVariant_NewQVariant for (i8) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Eb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QVariantC1Eb(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toULongLong<T: QVariant_toULongLong>(&mut self, value: T) -> i32 {
    value.toULongLong(self);
    return 1;
  }
}

pub trait QVariant_toULongLong {
  fn toULongLong(self, this: &mut QVariant) -> i32;
}

// proto: quint64 QVariant::toULongLong(bool * ok);
impl<'a> /*trait*/ QVariant_toULongLong for (&'a mut i8) {
  fn toULongLong(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toULongLongEPb()};
    let arg0 = self  as *mut int8_t;
    unsafe {_ZNK8QVariant11toULongLongEPb(arg0)};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonValue<T: QVariant_toJsonValue>(&mut self, value: T) -> i32 {
    value.toJsonValue(self);
    return 1;
  }
}

pub trait QVariant_toJsonValue {
  fn toJsonValue(self, this: &mut QVariant) -> i32;
}

// proto: QJsonValue QVariant::toJsonValue();
impl<'a> /*trait*/ QVariant_toJsonValue for () {
  fn toJsonValue(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toJsonValueEv()};
    unsafe {_ZNK8QVariant11toJsonValueEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toDateTime<T: QVariant_toDateTime>(&mut self, value: T) -> i32 {
    value.toDateTime(self);
    return 1;
  }
}

pub trait QVariant_toDateTime {
  fn toDateTime(self, this: &mut QVariant) -> i32;
}

// proto: QDateTime QVariant::toDateTime();
impl<'a> /*trait*/ QVariant_toDateTime for () {
  fn toDateTime(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toDateTimeEv()};
    unsafe {_ZNK8QVariant10toDateTimeEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn isDetached<T: QVariant_isDetached>(&mut self, value: T) -> i32 {
    value.isDetached(self);
    return 1;
  }
}

pub trait QVariant_isDetached {
  fn isDetached(self, this: &mut QVariant) -> i32;
}

// proto: bool QVariant::isDetached();
impl<'a> /*trait*/ QVariant_isDetached for () {
  fn isDetached(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10isDetachedEv()};
    unsafe {_ZNK8QVariant10isDetachedEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toEasingCurve<T: QVariant_toEasingCurve>(&mut self, value: T) -> i32 {
    value.toEasingCurve(self);
    return 1;
  }
}

pub trait QVariant_toEasingCurve {
  fn toEasingCurve(self, this: &mut QVariant) -> i32;
}

// proto: QEasingCurve QVariant::toEasingCurve();
impl<'a> /*trait*/ QVariant_toEasingCurve for () {
  fn toEasingCurve(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant13toEasingCurveEv()};
    unsafe {_ZNK8QVariant13toEasingCurveEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QUrl & url);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QUrl) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK4QUrl(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QVariant & other);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QVariant) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERKS_(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toBitArray<T: QVariant_toBitArray>(&mut self, value: T) -> i32 {
    value.toBitArray(self);
    return 1;
  }
}

pub trait QVariant_toBitArray {
  fn toBitArray(self, this: &mut QVariant) -> i32;
}

// proto: QBitArray QVariant::toBitArray();
impl<'a> /*trait*/ QVariant_toBitArray for () {
  fn toBitArray(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toBitArrayEv()};
    unsafe {_ZNK8QVariant10toBitArrayEv()};
    return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRegularExpression<T: QVariant_toRegularExpression>(&mut self, value: T) -> i32 {
    value.toRegularExpression(self);
    return 1;
  }
}

pub trait QVariant_toRegularExpression {
  fn toRegularExpression(self, this: &mut QVariant) -> i32;
}

// proto: QRegularExpression QVariant::toRegularExpression();
impl<'a> /*trait*/ QVariant_toRegularExpression for () {
  fn toRegularExpression(self, this: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant19toRegularExpressionEv()};
    unsafe {_ZNK8QVariant19toRegularExpressionEv()};
    return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRegularExpression & re);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRegularExpression) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK18QRegularExpression(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QStringList & stringlist);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QStringList) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK11QStringList()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QVariantC1ERK11QStringList(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

