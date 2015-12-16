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
use super::qsize::QSize;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qlocale::QLocale;
use super::qurl::QUrl;
use super::qline::QLine;
use super::qlinef::QLineF;
use super::quuid::QUuid;
use super::qpersistentmodelindex::QPersistentModelIndex;
use super::qdatastream::QDataStream;
use super::qregexp::QRegExp;
use super::qmodelindex::QModelIndex;
use super::qrectf::QRectF;
use super::qrect::QRect;
use super::qsizef::QSizeF;
use super::qchar::QChar;
use super::qdate::QDate;
use super::qbitarray::QBitArray;
use super::qtime::QTime;
use super::qdatetime::QDateTime;
use super::qeasingcurve::QEasingCurve;
use super::qregularexpression::QRegularExpression;
use super::qstringlist::QStringList;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QVariant::toDouble(bool * ok);
  fn _ZNK8QVariant8toDoubleEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_double;
  // proto:  void QVariant::NewQVariant(const char * str);
  fn _ZN8QVariantC1EPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto:  qint64 QVariant::toLongLong(bool * ok);
  fn _ZNK8QVariant10toLongLongEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_longlong;
  // proto:  void QVariant::NewQVariant(const QPointF & pt);
  fn _ZN8QVariantC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QPoint & pt);
  fn _ZN8QVariantC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QVariant::toSize();
  fn _ZNK8QVariant6toSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QVariant::toString();
  fn _ZNK8QVariant8toStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QVariant::toReal(bool * ok);
  fn _ZNK8QVariant6toRealEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_double;
  // proto:  float QVariant::toFloat(bool * ok);
  fn _ZNK8QVariant7toFloatEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_float;
  // proto:  void QVariant::NewQVariant(const QString & string);
  fn _ZN8QVariantC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QVariant::toByteArray();
  fn _ZNK8QVariant11toByteArrayEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLocale QVariant::toLocale();
  fn _ZNK8QVariant8toLocaleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUrl QVariant::toUrl();
  fn _ZNK8QVariant5toUrlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLine QVariant::toLine();
  fn _ZNK8QVariant6toLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QSize & size);
  fn _ZN8QVariantC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QLineF & line);
  fn _ZN8QVariantC1ERK6QLineF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const char * QVariant::typeName();
  fn _ZNK8QVariant8typeNameEv(qthis: *mut c_void) -> *const c_char;
  // proto:  QJsonArray QVariant::toJsonArray();
  fn _ZNK8QVariant11toJsonArrayEv(qthis: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QLocale & locale);
  fn _ZN8QVariantC1ERK7QLocale(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QStringList QVariant::toStringList();
  fn _ZNK8QVariant12toStringListEv(qthis: *mut c_void) ;
  // proto:  QList<QVariant> QVariant::toList();
  fn _ZNK8QVariant6toListEv(qthis: *mut c_void) ;
  // proto:  unsigned int QVariant::toUInt(bool * ok);
  fn _ZNK8QVariant6toUIntEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_uint;
  // proto:  QUuid QVariant::toUuid();
  fn _ZNK8QVariant6toUuidEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QPersistentModelIndex & modelIndex);
  fn _ZN8QVariantC1ERK21QPersistentModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QJsonDocument QVariant::toJsonDocument();
  fn _ZNK8QVariant14toJsonDocumentEv(qthis: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(QDataStream & s);
  fn _ZN8QVariantC1ER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QVariant::toPoint();
  fn _ZNK8QVariant7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QVariant::toInt(bool * ok);
  fn _ZNK8QVariant5toIntEPb(qthis: *mut c_void, arg0: *mut int8_t) -> c_int;
  // proto:  bool QVariant::isValid();
  fn _ZNK8QVariant7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QVariant::NewQVariant(const QUuid & uuid);
  fn _ZN8QVariantC1ERK5QUuid(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::detach();
  fn _ZN8QVariant6detachEv(qthis: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QRegExp & regExp);
  fn _ZN8QVariantC1ERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QVariant::toModelIndex();
  fn _ZNK8QVariant12toModelIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QHash<QString, QVariant> QVariant::toHash();
  fn _ZNK8QVariant6toHashEv(qthis: *mut c_void) ;
  // proto:  QMap<QString, QVariant> QVariant::toMap();
  fn _ZNK8QVariant5toMapEv(qthis: *mut c_void) ;
  // proto:  bool QVariant::canConvert(int targetTypeId);
  fn _ZNK8QVariant10canConvertEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QVariant::NewQVariant(const QRectF & rect);
  fn _ZN8QVariantC1ERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QRect & rect);
  fn _ZN8QVariantC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QVariant::userType();
  fn _ZNK8QVariant8userTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  const void * QVariant::constData();
  fn _ZNK8QVariant9constDataEv(qthis: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(void * );
  fn _ZN8QVariantC1EPv(qthis: *mut c_void, arg0: *mut uint8_t) ;
  // proto:  QPersistentModelIndex QVariant::toPersistentModelIndex();
  fn _ZNK8QVariant22toPersistentModelIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(int typeId, const void * copy, uint flags);
  fn _ZN8QVariantC1EiPKvj(qthis: *mut c_void, arg0: c_int, arg1: *const uint8_t, arg2: c_uint) ;
  // proto:  QLineF QVariant::toLineF();
  fn _ZNK8QVariant7toLineFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QJsonObject QVariant::toJsonObject();
  fn _ZNK8QVariant12toJsonObjectEv(qthis: *mut c_void) ;
  // proto:  void QVariant::load(QDataStream & ds);
  fn _ZN8QVariant4loadER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QSizeF & size);
  fn _ZN8QVariantC1ERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QChar QVariant::toChar();
  fn _ZNK8QVariant6toCharEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QVariant::isNull();
  fn _ZNK8QVariant6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QVariant::NewQVariant(const QDate & date);
  fn _ZN8QVariantC1ERK5QDate(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QVariant::toRectF();
  fn _ZNK8QVariant7toRectFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QBitArray & bitarray);
  fn _ZN8QVariantC1ERK9QBitArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QDate QVariant::toDate();
  fn _ZNK8QVariant6toDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QModelIndex & modelIndex);
  fn _ZN8QVariantC1ERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::FreeQVariant();
  fn _ZN8QVariantD0Ev(qthis: *mut c_void) ;
  // proto:  void QVariant::save(QDataStream & ds);
  fn _ZNK8QVariant4saveER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTime QVariant::toTime();
  fn _ZNK8QVariant6toTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QLine & line);
  fn _ZN8QVariantC1ERK5QLine(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void * QVariant::data();
  fn _ZN8QVariant4dataEv(qthis: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QTime & time);
  fn _ZN8QVariantC1ERK5QTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QDateTime & datetime);
  fn _ZN8QVariantC1ERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QVariant::convert(int targetTypeId);
  fn _ZN8QVariant7convertEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  QRegExp QVariant::toRegExp();
  fn _ZNK8QVariant8toRegExpEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QVariant::toPointF();
  fn _ZNK8QVariant8toPointFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(QChar qchar);
  fn _ZN8QVariantC1E5QChar(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static const char * QVariant::typeToName(int typeId);
  fn _ZN8QVariant10typeToNameEi(arg0: c_int) -> *const c_char;
  // proto:  QSizeF QVariant::toSizeF();
  fn _ZNK8QVariant7toSizeFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::swap(QVariant & other);
  fn _ZN8QVariant4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(int typeId, const void * copy);
  fn _ZN8QVariantC1EiPKv(qthis: *mut c_void, arg0: c_int, arg1: *const uint8_t) ;
  // proto:  void QVariant::NewQVariant(const QEasingCurve & easing);
  fn _ZN8QVariantC1ERK12QEasingCurve(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::clear();
  fn _ZN8QVariant5clearEv(qthis: *mut c_void) ;
  // proto:  QRect QVariant::toRect();
  fn _ZNK8QVariant6toRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QByteArray & bytearray);
  fn _ZN8QVariantC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(qlonglong ll);
  fn _ZN8QVariantC1Ex(qthis: *mut c_void, arg0: c_longlong) ;
  // proto:  void QVariant::NewQVariant(qulonglong ull);
  fn _ZN8QVariantC1Ey(qthis: *mut c_void, arg0: uint64_t) ;
  // proto:  void QVariant::NewQVariant();
  fn _ZN8QVariantC1Ev(qthis: *mut c_void) ;
  // proto:  bool QVariant::toBool();
  fn _ZNK8QVariant6toBoolEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QVariant::NewQVariant(uint ui);
  fn _ZN8QVariantC1Ej(qthis: *mut c_void, arg0: c_uint) ;
  // proto:  void QVariant::NewQVariant(int i);
  fn _ZN8QVariantC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QVariant::NewQVariant(float f);
  fn _ZN8QVariantC1Ef(qthis: *mut c_void, arg0: c_float) ;
  // proto:  void QVariant::NewQVariant(double d);
  fn _ZN8QVariantC1Ed(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QVariant::NewQVariant(bool b);
  fn _ZN8QVariantC1Eb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  quint64 QVariant::toULongLong(bool * ok);
  fn _ZNK8QVariant11toULongLongEPb(qthis: *mut c_void, arg0: *mut int8_t) -> uint64_t;
  // proto:  QJsonValue QVariant::toJsonValue();
  fn _ZNK8QVariant11toJsonValueEv(qthis: *mut c_void) ;
  // proto:  QDateTime QVariant::toDateTime();
  fn _ZNK8QVariant10toDateTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QVariant::isDetached();
  fn _ZNK8QVariant10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QEasingCurve QVariant::toEasingCurve();
  fn _ZNK8QVariant13toEasingCurveEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QUrl & url);
  fn _ZN8QVariantC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QVariant & other);
  fn _ZN8QVariantC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QBitArray QVariant::toBitArray();
  fn _ZNK8QVariant10toBitArrayEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRegularExpression QVariant::toRegularExpression();
  fn _ZNK8QVariant19toRegularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::NewQVariant(const QRegularExpression & re);
  fn _ZN8QVariantC1ERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVariant::NewQVariant(const QStringList & stringlist);
  fn _ZN8QVariantC1ERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QVariant)=16
pub struct QVariant {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVariant {
  pub fn toDouble<T: QVariant_toDouble>(&mut self, value: T) -> f64 {
    return value.toDouble(self);
    // return 1;
  }
}

pub trait QVariant_toDouble {
  fn toDouble(self, rsthis: &mut QVariant) -> f64;
}

// proto:  double QVariant::toDouble(bool * ok);
impl<'a> /*trait*/ QVariant_toDouble for (&'a mut i8) {
  fn toDouble(self, rsthis: &mut QVariant) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toDoubleEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
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
  pub fn toLongLong<T: QVariant_toLongLong>(&mut self, value: T) -> i64 {
    return value.toLongLong(self);
    // return 1;
  }
}

pub trait QVariant_toLongLong {
  fn toLongLong(self, rsthis: &mut QVariant) -> i64;
}

// proto:  qint64 QVariant::toLongLong(bool * ok);
impl<'a> /*trait*/ QVariant_toLongLong for (&'a mut i8) {
  fn toLongLong(self, rsthis: &mut QVariant) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toLongLongEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant10toLongLongEPb(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QPointF & pt);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QPointF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK6QPoint(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toSize<T: QVariant_toSize>(&mut self, value: T) -> QSize {
    return value.toSize(self);
    // return 1;
  }
}

pub trait QVariant_toSize {
  fn toSize(self, rsthis: &mut QVariant) -> QSize;
}

// proto:  QSize QVariant::toSize();
impl<'a> /*trait*/ QVariant_toSize for () {
  fn toSize(self, rsthis: &mut QVariant) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toSizeEv()};
    let mut ret = unsafe {_ZNK8QVariant6toSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toString<T: QVariant_toString>(&mut self, value: T) -> QString {
    return value.toString(self);
    // return 1;
  }
}

pub trait QVariant_toString {
  fn toString(self, rsthis: &mut QVariant) -> QString;
}

// proto:  QString QVariant::toString();
impl<'a> /*trait*/ QVariant_toString for () {
  fn toString(self, rsthis: &mut QVariant) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toStringEv()};
    let mut ret = unsafe {_ZNK8QVariant8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toReal<T: QVariant_toReal>(&mut self, value: T) -> f64 {
    return value.toReal(self);
    // return 1;
  }
}

pub trait QVariant_toReal {
  fn toReal(self, rsthis: &mut QVariant) -> f64;
}

// proto:  double QVariant::toReal(bool * ok);
impl<'a> /*trait*/ QVariant_toReal for (&'a mut i8) {
  fn toReal(self, rsthis: &mut QVariant) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toRealEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant6toRealEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toFloat<T: QVariant_toFloat>(&mut self, value: T) -> f32 {
    return value.toFloat(self);
    // return 1;
  }
}

pub trait QVariant_toFloat {
  fn toFloat(self, rsthis: &mut QVariant) -> f32;
}

// proto:  float QVariant::toFloat(bool * ok);
impl<'a> /*trait*/ QVariant_toFloat for (&'a mut i8) {
  fn toFloat(self, rsthis: &mut QVariant) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toFloatEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QString & string);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QString) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK7QString(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toByteArray<T: QVariant_toByteArray>(&mut self, value: T) -> QByteArray {
    return value.toByteArray(self);
    // return 1;
  }
}

pub trait QVariant_toByteArray {
  fn toByteArray(self, rsthis: &mut QVariant) -> QByteArray;
}

// proto:  QByteArray QVariant::toByteArray();
impl<'a> /*trait*/ QVariant_toByteArray for () {
  fn toByteArray(self, rsthis: &mut QVariant) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toByteArrayEv()};
    let mut ret = unsafe {_ZNK8QVariant11toByteArrayEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toLocale<T: QVariant_toLocale>(&mut self, value: T) -> QLocale {
    return value.toLocale(self);
    // return 1;
  }
}

pub trait QVariant_toLocale {
  fn toLocale(self, rsthis: &mut QVariant) -> QLocale;
}

// proto:  QLocale QVariant::toLocale();
impl<'a> /*trait*/ QVariant_toLocale for () {
  fn toLocale(self, rsthis: &mut QVariant) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toLocaleEv()};
    let mut ret = unsafe {_ZNK8QVariant8toLocaleEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toUrl<T: QVariant_toUrl>(&mut self, value: T) -> QUrl {
    return value.toUrl(self);
    // return 1;
  }
}

pub trait QVariant_toUrl {
  fn toUrl(self, rsthis: &mut QVariant) -> QUrl;
}

// proto:  QUrl QVariant::toUrl();
impl<'a> /*trait*/ QVariant_toUrl for () {
  fn toUrl(self, rsthis: &mut QVariant) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toUrlEv()};
    let mut ret = unsafe {_ZNK8QVariant5toUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toLine<T: QVariant_toLine>(&mut self, value: T) -> QLine {
    return value.toLine(self);
    // return 1;
  }
}

pub trait QVariant_toLine {
  fn toLine(self, rsthis: &mut QVariant) -> QLine;
}

// proto:  QLine QVariant::toLine();
impl<'a> /*trait*/ QVariant_toLine for () {
  fn toLine(self, rsthis: &mut QVariant) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toLineEv()};
    let mut ret = unsafe {_ZNK8QVariant6toLineEv(rsthis.qclsinst)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QSize & size);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QSize) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK6QLineF(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn typeName<T: QVariant_typeName>(&mut self, value: T) -> String {
    return value.typeName(self);
    // return 1;
  }
}

pub trait QVariant_typeName {
  fn typeName(self, rsthis: &mut QVariant) -> String;
}

// proto:  const char * QVariant::typeName();
impl<'a> /*trait*/ QVariant_typeName for () {
  fn typeName(self, rsthis: &mut QVariant) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8typeNameEv()};
    let mut ret = unsafe {_ZNK8QVariant8typeNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonArray<T: QVariant_toJsonArray>(&mut self, value: T)  {
     value.toJsonArray(self);
    // return 1;
  }
}

pub trait QVariant_toJsonArray {
  fn toJsonArray(self, rsthis: &mut QVariant) ;
}

// proto:  QJsonArray QVariant::toJsonArray();
impl<'a> /*trait*/ QVariant_toJsonArray for () {
  fn toJsonArray(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toJsonArrayEv()};
     unsafe {_ZNK8QVariant11toJsonArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QLocale & locale);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QLocale) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK7QLocale(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toStringList<T: QVariant_toStringList>(&mut self, value: T)  {
     value.toStringList(self);
    // return 1;
  }
}

pub trait QVariant_toStringList {
  fn toStringList(self, rsthis: &mut QVariant) ;
}

// proto:  QStringList QVariant::toStringList();
impl<'a> /*trait*/ QVariant_toStringList for () {
  fn toStringList(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toStringListEv()};
     unsafe {_ZNK8QVariant12toStringListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toList<T: QVariant_toList>(&mut self, value: T)  {
     value.toList(self);
    // return 1;
  }
}

pub trait QVariant_toList {
  fn toList(self, rsthis: &mut QVariant) ;
}

// proto:  QList<QVariant> QVariant::toList();
impl<'a> /*trait*/ QVariant_toList for () {
  fn toList(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toListEv()};
     unsafe {_ZNK8QVariant6toListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toUInt<T: QVariant_toUInt>(&mut self, value: T) -> u32 {
    return value.toUInt(self);
    // return 1;
  }
}

pub trait QVariant_toUInt {
  fn toUInt(self, rsthis: &mut QVariant) -> u32;
}

// proto:  unsigned int QVariant::toUInt(bool * ok);
impl<'a> /*trait*/ QVariant_toUInt for (&'a mut i8) {
  fn toUInt(self, rsthis: &mut QVariant) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toUIntEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant6toUIntEPb(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toUuid<T: QVariant_toUuid>(&mut self, value: T) -> QUuid {
    return value.toUuid(self);
    // return 1;
  }
}

pub trait QVariant_toUuid {
  fn toUuid(self, rsthis: &mut QVariant) -> QUuid;
}

// proto:  QUuid QVariant::toUuid();
impl<'a> /*trait*/ QVariant_toUuid for () {
  fn toUuid(self, rsthis: &mut QVariant) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toUuidEv()};
    let mut ret = unsafe {_ZNK8QVariant6toUuidEv(rsthis.qclsinst)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QPersistentModelIndex & modelIndex);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QPersistentModelIndex) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK21QPersistentModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK21QPersistentModelIndex(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonDocument<T: QVariant_toJsonDocument>(&mut self, value: T)  {
     value.toJsonDocument(self);
    // return 1;
  }
}

pub trait QVariant_toJsonDocument {
  fn toJsonDocument(self, rsthis: &mut QVariant) ;
}

// proto:  QJsonDocument QVariant::toJsonDocument();
impl<'a> /*trait*/ QVariant_toJsonDocument for () {
  fn toJsonDocument(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant14toJsonDocumentEv()};
     unsafe {_ZNK8QVariant14toJsonDocumentEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn toPoint<T: QVariant_toPoint>(&mut self, value: T) -> QPoint {
    return value.toPoint(self);
    // return 1;
  }
}

pub trait QVariant_toPoint {
  fn toPoint(self, rsthis: &mut QVariant) -> QPoint;
}

// proto:  QPoint QVariant::toPoint();
impl<'a> /*trait*/ QVariant_toPoint for () {
  fn toPoint(self, rsthis: &mut QVariant) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toPointEv()};
    let mut ret = unsafe {_ZNK8QVariant7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toInt<T: QVariant_toInt>(&mut self, value: T) -> i32 {
    return value.toInt(self);
    // return 1;
  }
}

pub trait QVariant_toInt {
  fn toInt(self, rsthis: &mut QVariant) -> i32;
}

// proto:  int QVariant::toInt(bool * ok);
impl<'a> /*trait*/ QVariant_toInt for (&'a mut i8) {
  fn toInt(self, rsthis: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toIntEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant5toIntEPb(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn isValid<T: QVariant_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QVariant_isValid {
  fn isValid(self, rsthis: &mut QVariant) -> i8;
}

// proto:  bool QVariant::isValid();
impl<'a> /*trait*/ QVariant_isValid for () {
  fn isValid(self, rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7isValidEv()};
    let mut ret = unsafe {_ZNK8QVariant7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QUuid & uuid);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QUuid) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QUuid()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK5QUuid(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn detach<T: QVariant_detach>(&mut self, value: T)  {
     value.detach(self);
    // return 1;
  }
}

pub trait QVariant_detach {
  fn detach(self, rsthis: &mut QVariant) ;
}

// proto:  void QVariant::detach();
impl<'a> /*trait*/ QVariant_detach for () {
  fn detach(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant6detachEv()};
     unsafe {_ZN8QVariant6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRegExp & regExp);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRegExp) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK7QRegExp()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK7QRegExp(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toModelIndex<T: QVariant_toModelIndex>(&mut self, value: T) -> QModelIndex {
    return value.toModelIndex(self);
    // return 1;
  }
}

pub trait QVariant_toModelIndex {
  fn toModelIndex(self, rsthis: &mut QVariant) -> QModelIndex;
}

// proto:  QModelIndex QVariant::toModelIndex();
impl<'a> /*trait*/ QVariant_toModelIndex for () {
  fn toModelIndex(self, rsthis: &mut QVariant) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toModelIndexEv()};
    let mut ret = unsafe {_ZNK8QVariant12toModelIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toHash<T: QVariant_toHash>(&mut self, value: T)  {
     value.toHash(self);
    // return 1;
  }
}

pub trait QVariant_toHash {
  fn toHash(self, rsthis: &mut QVariant) ;
}

// proto:  QHash<QString, QVariant> QVariant::toHash();
impl<'a> /*trait*/ QVariant_toHash for () {
  fn toHash(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toHashEv()};
     unsafe {_ZNK8QVariant6toHashEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toMap<T: QVariant_toMap>(&mut self, value: T)  {
     value.toMap(self);
    // return 1;
  }
}

pub trait QVariant_toMap {
  fn toMap(self, rsthis: &mut QVariant) ;
}

// proto:  QMap<QString, QVariant> QVariant::toMap();
impl<'a> /*trait*/ QVariant_toMap for () {
  fn toMap(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toMapEv()};
     unsafe {_ZNK8QVariant5toMapEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn canConvert<T: QVariant_canConvert>(&mut self, value: T) -> i8 {
    return value.canConvert(self);
    // return 1;
  }
}

pub trait QVariant_canConvert {
  fn canConvert(self, rsthis: &mut QVariant) -> i8;
}

// proto:  bool QVariant::canConvert(int targetTypeId);
impl<'a> /*trait*/ QVariant_canConvert for (i32) {
  fn canConvert(self, rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10canConvertEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QVariant10canConvertEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRectF & rect);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRectF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK5QRect(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn userType<T: QVariant_userType>(&mut self, value: T) -> i32 {
    return value.userType(self);
    // return 1;
  }
}

pub trait QVariant_userType {
  fn userType(self, rsthis: &mut QVariant) -> i32;
}

// proto:  int QVariant::userType();
impl<'a> /*trait*/ QVariant_userType for () {
  fn userType(self, rsthis: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8userTypeEv()};
    let mut ret = unsafe {_ZNK8QVariant8userTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn constData<T: QVariant_constData>(&mut self, value: T)  {
     value.constData(self);
    // return 1;
  }
}

pub trait QVariant_constData {
  fn constData(self, rsthis: &mut QVariant) ;
}

// proto:  const void * QVariant::constData();
impl<'a> /*trait*/ QVariant_constData for () {
  fn constData(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant9constDataEv()};
     unsafe {_ZNK8QVariant9constDataEv(rsthis.qclsinst)};
    // return 1;
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
  pub fn toPersistentModelIndex<T: QVariant_toPersistentModelIndex>(&mut self, value: T) -> QPersistentModelIndex {
    return value.toPersistentModelIndex(self);
    // return 1;
  }
}

pub trait QVariant_toPersistentModelIndex {
  fn toPersistentModelIndex(self, rsthis: &mut QVariant) -> QPersistentModelIndex;
}

// proto:  QPersistentModelIndex QVariant::toPersistentModelIndex();
impl<'a> /*trait*/ QVariant_toPersistentModelIndex for () {
  fn toPersistentModelIndex(self, rsthis: &mut QVariant) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant22toPersistentModelIndexEv()};
    let mut ret = unsafe {_ZNK8QVariant22toPersistentModelIndexEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn toLineF<T: QVariant_toLineF>(&mut self, value: T) -> QLineF {
    return value.toLineF(self);
    // return 1;
  }
}

pub trait QVariant_toLineF {
  fn toLineF(self, rsthis: &mut QVariant) -> QLineF;
}

// proto:  QLineF QVariant::toLineF();
impl<'a> /*trait*/ QVariant_toLineF for () {
  fn toLineF(self, rsthis: &mut QVariant) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toLineFEv()};
    let mut ret = unsafe {_ZNK8QVariant7toLineFEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonObject<T: QVariant_toJsonObject>(&mut self, value: T)  {
     value.toJsonObject(self);
    // return 1;
  }
}

pub trait QVariant_toJsonObject {
  fn toJsonObject(self, rsthis: &mut QVariant) ;
}

// proto:  QJsonObject QVariant::toJsonObject();
impl<'a> /*trait*/ QVariant_toJsonObject for () {
  fn toJsonObject(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toJsonObjectEv()};
     unsafe {_ZNK8QVariant12toJsonObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn load<T: QVariant_load>(&mut self, value: T)  {
     value.load(self);
    // return 1;
  }
}

pub trait QVariant_load {
  fn load(self, rsthis: &mut QVariant) ;
}

// proto:  void QVariant::load(QDataStream & ds);
impl<'a> /*trait*/ QVariant_load for (&'a mut QDataStream) {
  fn load(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4loadER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QVariant4loadER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QSizeF & size);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QSizeF) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK6QSizeF(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toChar<T: QVariant_toChar>(&mut self, value: T) -> QChar {
    return value.toChar(self);
    // return 1;
  }
}

pub trait QVariant_toChar {
  fn toChar(self, rsthis: &mut QVariant) -> QChar;
}

// proto:  QChar QVariant::toChar();
impl<'a> /*trait*/ QVariant_toChar for () {
  fn toChar(self, rsthis: &mut QVariant) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toCharEv()};
    let mut ret = unsafe {_ZNK8QVariant6toCharEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn isNull<T: QVariant_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QVariant_isNull {
  fn isNull(self, rsthis: &mut QVariant) -> i8;
}

// proto:  bool QVariant::isNull();
impl<'a> /*trait*/ QVariant_isNull for () {
  fn isNull(self, rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6isNullEv()};
    let mut ret = unsafe {_ZNK8QVariant6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QDate & date);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QDate) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QDate()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK5QDate(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRectF<T: QVariant_toRectF>(&mut self, value: T) -> QRectF {
    return value.toRectF(self);
    // return 1;
  }
}

pub trait QVariant_toRectF {
  fn toRectF(self, rsthis: &mut QVariant) -> QRectF;
}

// proto:  QRectF QVariant::toRectF();
impl<'a> /*trait*/ QVariant_toRectF for () {
  fn toRectF(self, rsthis: &mut QVariant) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toRectFEv()};
    let mut ret = unsafe {_ZNK8QVariant7toRectFEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QBitArray & bitarray);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QBitArray) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK9QBitArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK9QBitArray(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toDate<T: QVariant_toDate>(&mut self, value: T) -> QDate {
    return value.toDate(self);
    // return 1;
  }
}

pub trait QVariant_toDate {
  fn toDate(self, rsthis: &mut QVariant) -> QDate;
}

// proto:  QDate QVariant::toDate();
impl<'a> /*trait*/ QVariant_toDate for () {
  fn toDate(self, rsthis: &mut QVariant) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toDateEv()};
    let mut ret = unsafe {_ZNK8QVariant6toDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QModelIndex & modelIndex);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QModelIndex) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK11QModelIndex(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn FreeQVariant<T: QVariant_FreeQVariant>(&mut self, value: T)  {
     value.FreeQVariant(self);
    // return 1;
  }
}

pub trait QVariant_FreeQVariant {
  fn FreeQVariant(self, rsthis: &mut QVariant) ;
}

// proto:  void QVariant::FreeQVariant();
impl<'a> /*trait*/ QVariant_FreeQVariant for () {
  fn FreeQVariant(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantD0Ev()};
     unsafe {_ZN8QVariantD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn save<T: QVariant_save>(&mut self, value: T)  {
     value.save(self);
    // return 1;
  }
}

pub trait QVariant_save {
  fn save(self, rsthis: &mut QVariant) ;
}

// proto:  void QVariant::save(QDataStream & ds);
impl<'a> /*trait*/ QVariant_save for (&'a mut QDataStream) {
  fn save(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant4saveER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK8QVariant4saveER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toTime<T: QVariant_toTime>(&mut self, value: T) -> QTime {
    return value.toTime(self);
    // return 1;
  }
}

pub trait QVariant_toTime {
  fn toTime(self, rsthis: &mut QVariant) -> QTime;
}

// proto:  QTime QVariant::toTime();
impl<'a> /*trait*/ QVariant_toTime for () {
  fn toTime(self, rsthis: &mut QVariant) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toTimeEv()};
    let mut ret = unsafe {_ZNK8QVariant6toTimeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QLine & line);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QLine) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK5QLine(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn data<T: QVariant_data>(&mut self, value: T)  {
     value.data(self);
    // return 1;
  }
}

pub trait QVariant_data {
  fn data(self, rsthis: &mut QVariant) ;
}

// proto:  void * QVariant::data();
impl<'a> /*trait*/ QVariant_data for () {
  fn data(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4dataEv()};
     unsafe {_ZN8QVariant4dataEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QTime & time);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QTime) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK5QTime()};
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK9QDateTime(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn convert<T: QVariant_convert>(&mut self, value: T) -> i8 {
    return value.convert(self);
    // return 1;
  }
}

pub trait QVariant_convert {
  fn convert(self, rsthis: &mut QVariant) -> i8;
}

// proto:  bool QVariant::convert(int targetTypeId);
impl<'a> /*trait*/ QVariant_convert for (i32) {
  fn convert(self, rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant7convertEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QVariant7convertEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRegExp<T: QVariant_toRegExp>(&mut self, value: T) -> QRegExp {
    return value.toRegExp(self);
    // return 1;
  }
}

pub trait QVariant_toRegExp {
  fn toRegExp(self, rsthis: &mut QVariant) -> QRegExp;
}

// proto:  QRegExp QVariant::toRegExp();
impl<'a> /*trait*/ QVariant_toRegExp for () {
  fn toRegExp(self, rsthis: &mut QVariant) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toRegExpEv()};
    let mut ret = unsafe {_ZNK8QVariant8toRegExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toPointF<T: QVariant_toPointF>(&mut self, value: T) -> QPointF {
    return value.toPointF(self);
    // return 1;
  }
}

pub trait QVariant_toPointF {
  fn toPointF(self, rsthis: &mut QVariant) -> QPointF;
}

// proto:  QPointF QVariant::toPointF();
impl<'a> /*trait*/ QVariant_toPointF for () {
  fn toPointF(self, rsthis: &mut QVariant) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toPointFEv()};
    let mut ret = unsafe {_ZNK8QVariant8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn typeToName<T: QVariant_typeToName>(&mut self, value: T) -> String {
    return value.typeToName(self);
    // return 1;
  }
}

pub trait QVariant_typeToName {
  fn typeToName(self, rsthis: &mut QVariant) -> String;
}

// proto: static const char * QVariant::typeToName(int typeId);
impl<'a> /*trait*/ QVariant_typeToName for (i32) {
  fn typeToName(self, rsthis: &mut QVariant) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant10typeToNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QVariant10typeToNameEi(arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toSizeF<T: QVariant_toSizeF>(&mut self, value: T) -> QSizeF {
    return value.toSizeF(self);
    // return 1;
  }
}

pub trait QVariant_toSizeF {
  fn toSizeF(self, rsthis: &mut QVariant) -> QSizeF;
}

// proto:  QSizeF QVariant::toSizeF();
impl<'a> /*trait*/ QVariant_toSizeF for () {
  fn toSizeF(self, rsthis: &mut QVariant) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toSizeFEv()};
    let mut ret = unsafe {_ZNK8QVariant7toSizeFEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn swap<T: QVariant_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QVariant_swap {
  fn swap(self, rsthis: &mut QVariant) ;
}

// proto:  void QVariant::swap(QVariant & other);
impl<'a> /*trait*/ QVariant_swap for (&'a mut QVariant) {
  fn swap(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QVariant4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK12QEasingCurve(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn clear<T: QVariant_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QVariant_clear {
  fn clear(self, rsthis: &mut QVariant) ;
}

// proto:  void QVariant::clear();
impl<'a> /*trait*/ QVariant_clear for () {
  fn clear(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant5clearEv()};
     unsafe {_ZN8QVariant5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRect<T: QVariant_toRect>(&mut self, value: T) -> QRect {
    return value.toRect(self);
    // return 1;
  }
}

pub trait QVariant_toRect {
  fn toRect(self, rsthis: &mut QVariant) -> QRect;
}

// proto:  QRect QVariant::toRect();
impl<'a> /*trait*/ QVariant_toRect for () {
  fn toRect(self, rsthis: &mut QVariant) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toRectEv()};
    let mut ret = unsafe {_ZNK8QVariant6toRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QByteArray & bytearray);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QByteArray) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
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
  pub fn toBool<T: QVariant_toBool>(&mut self, value: T) -> i8 {
    return value.toBool(self);
    // return 1;
  }
}

pub trait QVariant_toBool {
  fn toBool(self, rsthis: &mut QVariant) -> i8;
}

// proto:  bool QVariant::toBool();
impl<'a> /*trait*/ QVariant_toBool for () {
  fn toBool(self, rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toBoolEv()};
    let mut ret = unsafe {_ZNK8QVariant6toBoolEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
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
  pub fn toULongLong<T: QVariant_toULongLong>(&mut self, value: T) -> u64 {
    return value.toULongLong(self);
    // return 1;
  }
}

pub trait QVariant_toULongLong {
  fn toULongLong(self, rsthis: &mut QVariant) -> u64;
}

// proto:  quint64 QVariant::toULongLong(bool * ok);
impl<'a> /*trait*/ QVariant_toULongLong for (&'a mut i8) {
  fn toULongLong(self, rsthis: &mut QVariant) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toULongLongEPb()};
    let arg0 = self  as *mut int8_t;
    let mut ret = unsafe {_ZNK8QVariant11toULongLongEPb(rsthis.qclsinst, arg0)};
    return ret as u64;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toJsonValue<T: QVariant_toJsonValue>(&mut self, value: T)  {
     value.toJsonValue(self);
    // return 1;
  }
}

pub trait QVariant_toJsonValue {
  fn toJsonValue(self, rsthis: &mut QVariant) ;
}

// proto:  QJsonValue QVariant::toJsonValue();
impl<'a> /*trait*/ QVariant_toJsonValue for () {
  fn toJsonValue(self, rsthis: &mut QVariant)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toJsonValueEv()};
     unsafe {_ZNK8QVariant11toJsonValueEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toDateTime<T: QVariant_toDateTime>(&mut self, value: T) -> QDateTime {
    return value.toDateTime(self);
    // return 1;
  }
}

pub trait QVariant_toDateTime {
  fn toDateTime(self, rsthis: &mut QVariant) -> QDateTime;
}

// proto:  QDateTime QVariant::toDateTime();
impl<'a> /*trait*/ QVariant_toDateTime for () {
  fn toDateTime(self, rsthis: &mut QVariant) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toDateTimeEv()};
    let mut ret = unsafe {_ZNK8QVariant10toDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn isDetached<T: QVariant_isDetached>(&mut self, value: T) -> i8 {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QVariant_isDetached {
  fn isDetached(self, rsthis: &mut QVariant) -> i8;
}

// proto:  bool QVariant::isDetached();
impl<'a> /*trait*/ QVariant_isDetached for () {
  fn isDetached(self, rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10isDetachedEv()};
    let mut ret = unsafe {_ZNK8QVariant10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toEasingCurve<T: QVariant_toEasingCurve>(&mut self, value: T) -> QEasingCurve {
    return value.toEasingCurve(self);
    // return 1;
  }
}

pub trait QVariant_toEasingCurve {
  fn toEasingCurve(self, rsthis: &mut QVariant) -> QEasingCurve;
}

// proto:  QEasingCurve QVariant::toEasingCurve();
impl<'a> /*trait*/ QVariant_toEasingCurve for () {
  fn toEasingCurve(self, rsthis: &mut QVariant) -> QEasingCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant13toEasingCurveEv()};
    let mut ret = unsafe {_ZNK8QVariant13toEasingCurveEv(rsthis.qclsinst)};
    let mut ret1 = QEasingCurve{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QUrl & url);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QUrl) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERKS_(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toBitArray<T: QVariant_toBitArray>(&mut self, value: T) -> QBitArray {
    return value.toBitArray(self);
    // return 1;
  }
}

pub trait QVariant_toBitArray {
  fn toBitArray(self, rsthis: &mut QVariant) -> QBitArray;
}

// proto:  QBitArray QVariant::toBitArray();
impl<'a> /*trait*/ QVariant_toBitArray for () {
  fn toBitArray(self, rsthis: &mut QVariant) -> QBitArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toBitArrayEv()};
    let mut ret = unsafe {_ZNK8QVariant10toBitArrayEv(rsthis.qclsinst)};
    let mut ret1 = QBitArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QVariant {
  pub fn toRegularExpression<T: QVariant_toRegularExpression>(&mut self, value: T) -> QRegularExpression {
    return value.toRegularExpression(self);
    // return 1;
  }
}

pub trait QVariant_toRegularExpression {
  fn toRegularExpression(self, rsthis: &mut QVariant) -> QRegularExpression;
}

// proto:  QRegularExpression QVariant::toRegularExpression();
impl<'a> /*trait*/ QVariant_toRegularExpression for () {
  fn toRegularExpression(self, rsthis: &mut QVariant) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant19toRegularExpressionEv()};
    let mut ret = unsafe {_ZNK8QVariant19toRegularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QVariant::NewQVariant(const QRegularExpression & re);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  QRegularExpression) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1ERK18QRegularExpression()};
    let arg0 = self.qclsinst  as *mut c_void;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QVariantC1ERK11QStringList(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

