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
  fn _ZNK8QVariant8toDoubleEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_double;
  // proto:  void QVariant::QVariant(const char * str);
  fn _ZN8QVariantC1EPKc(qthis: *mut c_void, arg0: *mut c_char);
  // proto:  qlonglong QVariant::toLongLong(bool * ok);
  fn _ZNK8QVariant10toLongLongEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_longlong;
  // proto:  void QVariant::QVariant(const QPointF & pt);
  fn _ZN8QVariantC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QPoint & pt);
  fn _ZN8QVariantC1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QVariant::toSize();
  fn _ZNK8QVariant6toSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QVariant::toString();
  fn _ZNK8QVariant8toStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QVariant::toReal(bool * ok);
  fn _ZNK8QVariant6toRealEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_double;
  // proto:  float QVariant::toFloat(bool * ok);
  fn _ZNK8QVariant7toFloatEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_float;
  // proto:  void QVariant::QVariant(const QString & string);
  fn _ZN8QVariantC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QVariant::toByteArray();
  fn _ZNK8QVariant11toByteArrayEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLocale QVariant::toLocale();
  fn _ZNK8QVariant8toLocaleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QUrl QVariant::toUrl();
  fn _ZNK8QVariant5toUrlEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLine QVariant::toLine();
  fn _ZNK8QVariant6toLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QSize & size);
  fn _ZN8QVariantC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QLineF & line);
  fn _ZN8QVariantC1ERK6QLineF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const char * QVariant::typeName();
  fn _ZNK8QVariant8typeNameEv(qthis: *mut c_void) -> *mut c_char;
  // proto:  QJsonArray QVariant::toJsonArray();
  fn _ZNK8QVariant11toJsonArrayEv(qthis: *mut c_void);
  // proto:  void QVariant::QVariant(const QLocale & locale);
  fn _ZN8QVariantC1ERK7QLocale(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QVariant::toStringList();
  fn _ZNK8QVariant12toStringListEv(qthis: *mut c_void);
  // proto:  QList<QVariant> QVariant::toList();
  fn _ZNK8QVariant6toListEv(qthis: *mut c_void);
  // proto:  uint QVariant::toUInt(bool * ok);
  fn _ZNK8QVariant6toUIntEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_uint;
  // proto:  QUuid QVariant::toUuid();
  fn _ZNK8QVariant6toUuidEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QPersistentModelIndex & modelIndex);
  fn _ZN8QVariantC1ERK21QPersistentModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QJsonDocument QVariant::toJsonDocument();
  fn _ZNK8QVariant14toJsonDocumentEv(qthis: *mut c_void);
  // proto:  void QVariant::QVariant(QDataStream & s);
  fn _ZN8QVariantC1ER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPoint QVariant::toPoint();
  fn _ZNK8QVariant7toPointEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QVariant::toInt(bool * ok);
  fn _ZNK8QVariant5toIntEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_int;
  // proto:  bool QVariant::isValid();
  fn _ZNK8QVariant7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QVariant::QVariant(const QUuid & uuid);
  fn _ZN8QVariantC1ERK5QUuid(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::detach();
  fn _ZN8QVariant6detachEv(qthis: *mut c_void);
  // proto:  void QVariant::QVariant(const QRegExp & regExp);
  fn _ZN8QVariantC1ERK7QRegExp(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QModelIndex QVariant::toModelIndex();
  fn _ZNK8QVariant12toModelIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QHash<QString, QVariant> QVariant::toHash();
  fn _ZNK8QVariant6toHashEv(qthis: *mut c_void);
  // proto:  QMap<QString, QVariant> QVariant::toMap();
  fn _ZNK8QVariant5toMapEv(qthis: *mut c_void);
  // proto:  bool QVariant::canConvert(int targetTypeId);
  fn _ZNK8QVariant10canConvertEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QVariant::QVariant(const QRectF & rect);
  fn _ZN8QVariantC1ERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QRect & rect);
  fn _ZN8QVariantC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QVariant::userType();
  fn _ZNK8QVariant8userTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  const void * QVariant::constData();
  fn _ZNK8QVariant9constDataEv(qthis: *mut c_void);
  // proto:  void QVariant::QVariant(void * );
  fn _ZN8QVariantC1EPv(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPersistentModelIndex QVariant::toPersistentModelIndex();
  fn _ZNK8QVariant22toPersistentModelIndexEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(int typeId, const void * copy, uint flags);
  fn _ZN8QVariantC1EiPKvj(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_uint);
  // proto:  QLineF QVariant::toLineF();
  fn _ZNK8QVariant7toLineFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QJsonObject QVariant::toJsonObject();
  fn _ZNK8QVariant12toJsonObjectEv(qthis: *mut c_void);
  // proto:  void QVariant::load(QDataStream & ds);
  fn _ZN8QVariant4loadER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QSizeF & size);
  fn _ZN8QVariantC1ERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QChar QVariant::toChar();
  fn _ZNK8QVariant6toCharEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QVariant::isNull();
  fn _ZNK8QVariant6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QVariant::QVariant(const QDate & date);
  fn _ZN8QVariantC1ERK5QDate(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QVariant::toRectF();
  fn _ZNK8QVariant7toRectFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QBitArray & bitarray);
  fn _ZN8QVariantC1ERK9QBitArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QDate QVariant::toDate();
  fn _ZNK8QVariant6toDateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QModelIndex & modelIndex);
  fn _ZN8QVariantC1ERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::~QVariant();
  fn _ZN8QVariantD0Ev(qthis: *mut c_void);
  // proto:  void QVariant::save(QDataStream & ds);
  fn _ZNK8QVariant4saveER11QDataStream(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTime QVariant::toTime();
  fn _ZNK8QVariant6toTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QLine & line);
  fn _ZN8QVariantC1ERK5QLine(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void * QVariant::data();
  fn _ZN8QVariant4dataEv(qthis: *mut c_void);
  // proto:  void QVariant::QVariant(const QTime & time);
  fn _ZN8QVariantC1ERK5QTime(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QDateTime & datetime);
  fn _ZN8QVariantC1ERK9QDateTime(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QVariant::convert(int targetTypeId);
  fn _ZN8QVariant7convertEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QRegExp QVariant::toRegExp();
  fn _ZNK8QVariant8toRegExpEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QVariant::toPointF();
  fn _ZNK8QVariant8toPointFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(QChar qchar);
  fn _ZN8QVariantC1E5QChar(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static const char * QVariant::typeToName(int typeId);
  fn _ZN8QVariant10typeToNameEi(arg0: c_int) -> *mut c_char;
  // proto:  QSizeF QVariant::toSizeF();
  fn _ZNK8QVariant7toSizeFEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::swap(QVariant & other);
  fn _ZN8QVariant4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(int typeId, const void * copy);
  fn _ZN8QVariantC1EiPKv(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QVariant::QVariant(const QEasingCurve & easing);
  fn _ZN8QVariantC1ERK12QEasingCurve(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::clear();
  fn _ZN8QVariant5clearEv(qthis: *mut c_void);
  // proto:  QRect QVariant::toRect();
  fn _ZNK8QVariant6toRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QByteArray & bytearray);
  fn _ZN8QVariantC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(qlonglong ll);
  fn _ZN8QVariantC1Ex(qthis: *mut c_void, arg0: c_longlong);
  // proto:  void QVariant::QVariant(qulonglong ull);
  fn _ZN8QVariantC1Ey(qthis: *mut c_void, arg0: c_ulonglong);
  // proto:  void QVariant::QVariant();
  fn _ZN8QVariantC1Ev(qthis: *mut c_void);
  // proto:  bool QVariant::toBool();
  fn _ZNK8QVariant6toBoolEv(qthis: *mut c_void) -> c_char;
  // proto:  void QVariant::QVariant(uint ui);
  fn _ZN8QVariantC1Ej(qthis: *mut c_void, arg0: c_uint);
  // proto:  void QVariant::QVariant(int i);
  fn _ZN8QVariantC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QVariant::QVariant(float f);
  fn _ZN8QVariantC1Ef(qthis: *mut c_void, arg0: c_float);
  // proto:  void QVariant::QVariant(double d);
  fn _ZN8QVariantC1Ed(qthis: *mut c_void, arg0: c_double);
  // proto:  void QVariant::QVariant(bool b);
  fn _ZN8QVariantC1Eb(qthis: *mut c_void, arg0: c_char);
  // proto:  qulonglong QVariant::toULongLong(bool * ok);
  fn _ZNK8QVariant11toULongLongEPb(qthis: *mut c_void, arg0: *mut c_char) -> c_ulonglong;
  // proto:  QJsonValue QVariant::toJsonValue();
  fn _ZNK8QVariant11toJsonValueEv(qthis: *mut c_void);
  // proto:  QDateTime QVariant::toDateTime();
  fn _ZNK8QVariant10toDateTimeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QVariant::isDetached();
  fn _ZNK8QVariant10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  QEasingCurve QVariant::toEasingCurve();
  fn _ZNK8QVariant13toEasingCurveEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QUrl & url);
  fn _ZN8QVariantC1ERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QVariant & other);
  fn _ZN8QVariantC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QBitArray QVariant::toBitArray();
  fn _ZNK8QVariant10toBitArrayEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRegularExpression QVariant::toRegularExpression();
  fn _ZNK8QVariant19toRegularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QVariant::QVariant(const QRegularExpression & re);
  fn _ZN8QVariantC1ERK18QRegularExpression(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QVariant::QVariant(const QStringList & stringlist);
  fn _ZN8QVariantC1ERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QVariant)=16
pub struct QVariant {
  pub qclsinst: *mut c_void,
}

  // proto:  double QVariant::toDouble(bool * ok);
impl /*struct*/ QVariant {
  pub fn toDouble<RetType, T: QVariant_toDouble<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QVariant_toDouble<RetType> {
  fn toDouble(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  double QVariant::toDouble(bool * ok);
impl<'a> /*trait*/ QVariant_toDouble<f64> for (&'a mut Vec<i8>) {
  fn toDouble(self , rsthis: &mut QVariant) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toDoubleEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant8toDoubleEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const char * str);
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

  // proto:  void QVariant::QVariant(const char * str);
impl<'a> /*trait*/ QVariant_NewQVariant for (&'a  String) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    unsafe {_ZN8QVariantC1EPKc(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qlonglong QVariant::toLongLong(bool * ok);
impl /*struct*/ QVariant {
  pub fn toLongLong<RetType, T: QVariant_toLongLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QVariant_toLongLong<RetType> {
  fn toLongLong(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  qlonglong QVariant::toLongLong(bool * ok);
impl<'a> /*trait*/ QVariant_toLongLong<i64> for (&'a mut Vec<i8>) {
  fn toLongLong(self , rsthis: &mut QVariant) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toLongLongEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant10toLongLongEPb(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QPointF & pt);
impl<'a> /*trait*/ QVariant_NewQVariant for (QPointF) {
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

  // proto:  void QVariant::QVariant(const QPoint & pt);
impl<'a> /*trait*/ QVariant_NewQVariant for (QPoint) {
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

  // proto:  QSize QVariant::toSize();
impl /*struct*/ QVariant {
  pub fn toSize<RetType, T: QVariant_toSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toSize(self);
    // return 1;
  }
}

pub trait QVariant_toSize<RetType> {
  fn toSize(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QSize QVariant::toSize();
impl<'a> /*trait*/ QVariant_toSize<QSize> for () {
  fn toSize(self , rsthis: &mut QVariant) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toSizeEv()};
    let mut ret = unsafe {_ZNK8QVariant6toSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QVariant::toString();
impl /*struct*/ QVariant {
  pub fn toString<RetType, T: QVariant_toString<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QVariant_toString<RetType> {
  fn toString(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QString QVariant::toString();
impl<'a> /*trait*/ QVariant_toString<QString> for () {
  fn toString(self , rsthis: &mut QVariant) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toStringEv()};
    let mut ret = unsafe {_ZNK8QVariant8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QVariant::toReal(bool * ok);
impl /*struct*/ QVariant {
  pub fn toReal<RetType, T: QVariant_toReal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toReal(self);
    // return 1;
  }
}

pub trait QVariant_toReal<RetType> {
  fn toReal(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  qreal QVariant::toReal(bool * ok);
impl<'a> /*trait*/ QVariant_toReal<f64> for (&'a mut Vec<i8>) {
  fn toReal(self , rsthis: &mut QVariant) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toRealEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant6toRealEPb(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  float QVariant::toFloat(bool * ok);
impl /*struct*/ QVariant {
  pub fn toFloat<RetType, T: QVariant_toFloat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QVariant_toFloat<RetType> {
  fn toFloat(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  float QVariant::toFloat(bool * ok);
impl<'a> /*trait*/ QVariant_toFloat<f32> for (&'a mut Vec<i8>) {
  fn toFloat(self , rsthis: &mut QVariant) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toFloatEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant7toFloatEPb(rsthis.qclsinst, arg0)};
    return ret as f32;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QString & string);
impl<'a> /*trait*/ QVariant_NewQVariant for (QString) {
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

  // proto:  QByteArray QVariant::toByteArray();
impl /*struct*/ QVariant {
  pub fn toByteArray<RetType, T: QVariant_toByteArray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toByteArray(self);
    // return 1;
  }
}

pub trait QVariant_toByteArray<RetType> {
  fn toByteArray(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QByteArray QVariant::toByteArray();
impl<'a> /*trait*/ QVariant_toByteArray<QByteArray> for () {
  fn toByteArray(self , rsthis: &mut QVariant) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toByteArrayEv()};
    let mut ret = unsafe {_ZNK8QVariant11toByteArrayEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QLocale QVariant::toLocale();
impl /*struct*/ QVariant {
  pub fn toLocale<RetType, T: QVariant_toLocale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLocale(self);
    // return 1;
  }
}

pub trait QVariant_toLocale<RetType> {
  fn toLocale(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QLocale QVariant::toLocale();
impl<'a> /*trait*/ QVariant_toLocale<QLocale> for () {
  fn toLocale(self , rsthis: &mut QVariant) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toLocaleEv()};
    let mut ret = unsafe {_ZNK8QVariant8toLocaleEv(rsthis.qclsinst)};
    let mut ret1 = QLocale{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QVariant::toUrl();
impl /*struct*/ QVariant {
  pub fn toUrl<RetType, T: QVariant_toUrl<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUrl(self);
    // return 1;
  }
}

pub trait QVariant_toUrl<RetType> {
  fn toUrl(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QUrl QVariant::toUrl();
impl<'a> /*trait*/ QVariant_toUrl<QUrl> for () {
  fn toUrl(self , rsthis: &mut QVariant) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toUrlEv()};
    let mut ret = unsafe {_ZNK8QVariant5toUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QLine QVariant::toLine();
impl /*struct*/ QVariant {
  pub fn toLine<RetType, T: QVariant_toLine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLine(self);
    // return 1;
  }
}

pub trait QVariant_toLine<RetType> {
  fn toLine(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QLine QVariant::toLine();
impl<'a> /*trait*/ QVariant_toLine<QLine> for () {
  fn toLine(self , rsthis: &mut QVariant) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toLineEv()};
    let mut ret = unsafe {_ZNK8QVariant6toLineEv(rsthis.qclsinst)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QSize & size);
impl<'a> /*trait*/ QVariant_NewQVariant for (QSize) {
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

  // proto:  void QVariant::QVariant(const QLineF & line);
impl<'a> /*trait*/ QVariant_NewQVariant for (QLineF) {
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

  // proto:  const char * QVariant::typeName();
impl /*struct*/ QVariant {
  pub fn typeName<RetType, T: QVariant_typeName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.typeName(self);
    // return 1;
  }
}

pub trait QVariant_typeName<RetType> {
  fn typeName(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  const char * QVariant::typeName();
impl<'a> /*trait*/ QVariant_typeName<String> for () {
  fn typeName(self , rsthis: &mut QVariant) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8typeNameEv()};
    let mut ret = unsafe {_ZNK8QVariant8typeNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  QJsonArray QVariant::toJsonArray();
impl /*struct*/ QVariant {
  pub fn toJsonArray<RetType, T: QVariant_toJsonArray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toJsonArray(self);
    // return 1;
  }
}

pub trait QVariant_toJsonArray<RetType> {
  fn toJsonArray(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QJsonArray QVariant::toJsonArray();
impl<'a> /*trait*/ QVariant_toJsonArray<()> for () {
  fn toJsonArray(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toJsonArrayEv()};
     unsafe {_ZNK8QVariant11toJsonArrayEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QLocale & locale);
impl<'a> /*trait*/ QVariant_NewQVariant for (QLocale) {
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

  // proto:  QStringList QVariant::toStringList();
impl /*struct*/ QVariant {
  pub fn toStringList<RetType, T: QVariant_toStringList<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toStringList(self);
    // return 1;
  }
}

pub trait QVariant_toStringList<RetType> {
  fn toStringList(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QStringList QVariant::toStringList();
impl<'a> /*trait*/ QVariant_toStringList<()> for () {
  fn toStringList(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toStringListEv()};
     unsafe {_ZNK8QVariant12toStringListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QVariant> QVariant::toList();
impl /*struct*/ QVariant {
  pub fn toList<RetType, T: QVariant_toList<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toList(self);
    // return 1;
  }
}

pub trait QVariant_toList<RetType> {
  fn toList(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QList<QVariant> QVariant::toList();
impl<'a> /*trait*/ QVariant_toList<()> for () {
  fn toList(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toListEv()};
     unsafe {_ZNK8QVariant6toListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  uint QVariant::toUInt(bool * ok);
impl /*struct*/ QVariant {
  pub fn toUInt<RetType, T: QVariant_toUInt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QVariant_toUInt<RetType> {
  fn toUInt(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  uint QVariant::toUInt(bool * ok);
impl<'a> /*trait*/ QVariant_toUInt<u32> for (&'a mut Vec<i8>) {
  fn toUInt(self , rsthis: &mut QVariant) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toUIntEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant6toUIntEPb(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QUuid QVariant::toUuid();
impl /*struct*/ QVariant {
  pub fn toUuid<RetType, T: QVariant_toUuid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toUuid(self);
    // return 1;
  }
}

pub trait QVariant_toUuid<RetType> {
  fn toUuid(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QUuid QVariant::toUuid();
impl<'a> /*trait*/ QVariant_toUuid<QUuid> for () {
  fn toUuid(self , rsthis: &mut QVariant) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toUuidEv()};
    let mut ret = unsafe {_ZNK8QVariant6toUuidEv(rsthis.qclsinst)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QPersistentModelIndex & modelIndex);
impl<'a> /*trait*/ QVariant_NewQVariant for (QPersistentModelIndex) {
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

  // proto:  QJsonDocument QVariant::toJsonDocument();
impl /*struct*/ QVariant {
  pub fn toJsonDocument<RetType, T: QVariant_toJsonDocument<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toJsonDocument(self);
    // return 1;
  }
}

pub trait QVariant_toJsonDocument<RetType> {
  fn toJsonDocument(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QJsonDocument QVariant::toJsonDocument();
impl<'a> /*trait*/ QVariant_toJsonDocument<()> for () {
  fn toJsonDocument(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant14toJsonDocumentEv()};
     unsafe {_ZNK8QVariant14toJsonDocumentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(QDataStream & s);
impl<'a> /*trait*/ QVariant_NewQVariant for (QDataStream) {
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

  // proto:  QPoint QVariant::toPoint();
impl /*struct*/ QVariant {
  pub fn toPoint<RetType, T: QVariant_toPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPoint(self);
    // return 1;
  }
}

pub trait QVariant_toPoint<RetType> {
  fn toPoint(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QPoint QVariant::toPoint();
impl<'a> /*trait*/ QVariant_toPoint<QPoint> for () {
  fn toPoint(self , rsthis: &mut QVariant) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toPointEv()};
    let mut ret = unsafe {_ZNK8QVariant7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QVariant::toInt(bool * ok);
impl /*struct*/ QVariant {
  pub fn toInt<RetType, T: QVariant_toInt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QVariant_toInt<RetType> {
  fn toInt(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  int QVariant::toInt(bool * ok);
impl<'a> /*trait*/ QVariant_toInt<i32> for (&'a mut Vec<i8>) {
  fn toInt(self , rsthis: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toIntEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant5toIntEPb(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QVariant::isValid();
impl /*struct*/ QVariant {
  pub fn isValid<RetType, T: QVariant_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QVariant_isValid<RetType> {
  fn isValid(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  bool QVariant::isValid();
impl<'a> /*trait*/ QVariant_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7isValidEv()};
    let mut ret = unsafe {_ZNK8QVariant7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QUuid & uuid);
impl<'a> /*trait*/ QVariant_NewQVariant for (QUuid) {
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

  // proto:  void QVariant::detach();
impl /*struct*/ QVariant {
  pub fn detach<RetType, T: QVariant_detach<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QVariant_detach<RetType> {
  fn detach(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void QVariant::detach();
impl<'a> /*trait*/ QVariant_detach<()> for () {
  fn detach(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant6detachEv()};
     unsafe {_ZN8QVariant6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QRegExp & regExp);
impl<'a> /*trait*/ QVariant_NewQVariant for (QRegExp) {
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

  // proto:  QModelIndex QVariant::toModelIndex();
impl /*struct*/ QVariant {
  pub fn toModelIndex<RetType, T: QVariant_toModelIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toModelIndex(self);
    // return 1;
  }
}

pub trait QVariant_toModelIndex<RetType> {
  fn toModelIndex(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QModelIndex QVariant::toModelIndex();
impl<'a> /*trait*/ QVariant_toModelIndex<QModelIndex> for () {
  fn toModelIndex(self , rsthis: &mut QVariant) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toModelIndexEv()};
    let mut ret = unsafe {_ZNK8QVariant12toModelIndexEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QHash<QString, QVariant> QVariant::toHash();
impl /*struct*/ QVariant {
  pub fn toHash<RetType, T: QVariant_toHash<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toHash(self);
    // return 1;
  }
}

pub trait QVariant_toHash<RetType> {
  fn toHash(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QHash<QString, QVariant> QVariant::toHash();
impl<'a> /*trait*/ QVariant_toHash<()> for () {
  fn toHash(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toHashEv()};
     unsafe {_ZNK8QVariant6toHashEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QMap<QString, QVariant> QVariant::toMap();
impl /*struct*/ QVariant {
  pub fn toMap<RetType, T: QVariant_toMap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toMap(self);
    // return 1;
  }
}

pub trait QVariant_toMap<RetType> {
  fn toMap(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QMap<QString, QVariant> QVariant::toMap();
impl<'a> /*trait*/ QVariant_toMap<()> for () {
  fn toMap(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant5toMapEv()};
     unsafe {_ZNK8QVariant5toMapEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QVariant::canConvert(int targetTypeId);
impl /*struct*/ QVariant {
  pub fn canConvert<RetType, T: QVariant_canConvert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canConvert(self);
    // return 1;
  }
}

pub trait QVariant_canConvert<RetType> {
  fn canConvert(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  bool QVariant::canConvert(int targetTypeId);
impl<'a> /*trait*/ QVariant_canConvert<i8> for (i32) {
  fn canConvert(self , rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10canConvertEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QVariant10canConvertEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QRectF & rect);
impl<'a> /*trait*/ QVariant_NewQVariant for (QRectF) {
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

  // proto:  void QVariant::QVariant(const QRect & rect);
impl<'a> /*trait*/ QVariant_NewQVariant for (QRect) {
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

  // proto:  int QVariant::userType();
impl /*struct*/ QVariant {
  pub fn userType<RetType, T: QVariant_userType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.userType(self);
    // return 1;
  }
}

pub trait QVariant_userType<RetType> {
  fn userType(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  int QVariant::userType();
impl<'a> /*trait*/ QVariant_userType<i32> for () {
  fn userType(self , rsthis: &mut QVariant) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8userTypeEv()};
    let mut ret = unsafe {_ZNK8QVariant8userTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const void * QVariant::constData();
impl /*struct*/ QVariant {
  pub fn constData<RetType, T: QVariant_constData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QVariant_constData<RetType> {
  fn constData(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  const void * QVariant::constData();
impl<'a> /*trait*/ QVariant_constData<()> for () {
  fn constData(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant9constDataEv()};
     unsafe {_ZNK8QVariant9constDataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(void * );
impl<'a> /*trait*/ QVariant_NewQVariant for (*mut c_void) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EPv()};
    let arg0 = self  as *mut c_void;
    unsafe {_ZN8QVariantC1EPv(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPersistentModelIndex QVariant::toPersistentModelIndex();
impl /*struct*/ QVariant {
  pub fn toPersistentModelIndex<RetType, T: QVariant_toPersistentModelIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPersistentModelIndex(self);
    // return 1;
  }
}

pub trait QVariant_toPersistentModelIndex<RetType> {
  fn toPersistentModelIndex(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QPersistentModelIndex QVariant::toPersistentModelIndex();
impl<'a> /*trait*/ QVariant_toPersistentModelIndex<QPersistentModelIndex> for () {
  fn toPersistentModelIndex(self , rsthis: &mut QVariant) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant22toPersistentModelIndexEv()};
    let mut ret = unsafe {_ZNK8QVariant22toPersistentModelIndexEv(rsthis.qclsinst)};
    let mut ret1 = QPersistentModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(int typeId, const void * copy, uint flags);
impl<'a> /*trait*/ QVariant_NewQVariant for (i32, *mut c_void, u32) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EiPKvj()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
    let arg2 = self.2  as c_uint;
    unsafe {_ZN8QVariantC1EiPKvj(qthis, arg0, arg1, arg2)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QLineF QVariant::toLineF();
impl /*struct*/ QVariant {
  pub fn toLineF<RetType, T: QVariant_toLineF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toLineF(self);
    // return 1;
  }
}

pub trait QVariant_toLineF<RetType> {
  fn toLineF(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QLineF QVariant::toLineF();
impl<'a> /*trait*/ QVariant_toLineF<QLineF> for () {
  fn toLineF(self , rsthis: &mut QVariant) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toLineFEv()};
    let mut ret = unsafe {_ZNK8QVariant7toLineFEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QJsonObject QVariant::toJsonObject();
impl /*struct*/ QVariant {
  pub fn toJsonObject<RetType, T: QVariant_toJsonObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toJsonObject(self);
    // return 1;
  }
}

pub trait QVariant_toJsonObject<RetType> {
  fn toJsonObject(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QJsonObject QVariant::toJsonObject();
impl<'a> /*trait*/ QVariant_toJsonObject<()> for () {
  fn toJsonObject(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant12toJsonObjectEv()};
     unsafe {_ZNK8QVariant12toJsonObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::load(QDataStream & ds);
impl /*struct*/ QVariant {
  pub fn load<RetType, T: QVariant_load<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.load(self);
    // return 1;
  }
}

pub trait QVariant_load<RetType> {
  fn load(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void QVariant::load(QDataStream & ds);
impl<'a> /*trait*/ QVariant_load<()> for (QDataStream) {
  fn load(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4loadER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QVariant4loadER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QSizeF & size);
impl<'a> /*trait*/ QVariant_NewQVariant for (QSizeF) {
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

  // proto:  QChar QVariant::toChar();
impl /*struct*/ QVariant {
  pub fn toChar<RetType, T: QVariant_toChar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toChar(self);
    // return 1;
  }
}

pub trait QVariant_toChar<RetType> {
  fn toChar(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QChar QVariant::toChar();
impl<'a> /*trait*/ QVariant_toChar<QChar> for () {
  fn toChar(self , rsthis: &mut QVariant) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toCharEv()};
    let mut ret = unsafe {_ZNK8QVariant6toCharEv(rsthis.qclsinst)};
    let mut ret1 = QChar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QVariant::isNull();
impl /*struct*/ QVariant {
  pub fn isNull<RetType, T: QVariant_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QVariant_isNull<RetType> {
  fn isNull(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  bool QVariant::isNull();
impl<'a> /*trait*/ QVariant_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6isNullEv()};
    let mut ret = unsafe {_ZNK8QVariant6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QDate & date);
impl<'a> /*trait*/ QVariant_NewQVariant for (QDate) {
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

  // proto:  QRectF QVariant::toRectF();
impl /*struct*/ QVariant {
  pub fn toRectF<RetType, T: QVariant_toRectF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toRectF(self);
    // return 1;
  }
}

pub trait QVariant_toRectF<RetType> {
  fn toRectF(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QRectF QVariant::toRectF();
impl<'a> /*trait*/ QVariant_toRectF<QRectF> for () {
  fn toRectF(self , rsthis: &mut QVariant) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toRectFEv()};
    let mut ret = unsafe {_ZNK8QVariant7toRectFEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QBitArray & bitarray);
impl<'a> /*trait*/ QVariant_NewQVariant for (QBitArray) {
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

  // proto:  QDate QVariant::toDate();
impl /*struct*/ QVariant {
  pub fn toDate<RetType, T: QVariant_toDate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDate(self);
    // return 1;
  }
}

pub trait QVariant_toDate<RetType> {
  fn toDate(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QDate QVariant::toDate();
impl<'a> /*trait*/ QVariant_toDate<QDate> for () {
  fn toDate(self , rsthis: &mut QVariant) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toDateEv()};
    let mut ret = unsafe {_ZNK8QVariant6toDateEv(rsthis.qclsinst)};
    let mut ret1 = QDate{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QModelIndex & modelIndex);
impl<'a> /*trait*/ QVariant_NewQVariant for (QModelIndex) {
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

  // proto:  void QVariant::~QVariant();
impl /*struct*/ QVariant {
  pub fn FreeQVariant<RetType, T: QVariant_FreeQVariant<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQVariant(self);
    // return 1;
  }
}

pub trait QVariant_FreeQVariant<RetType> {
  fn FreeQVariant(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void QVariant::~QVariant();
impl<'a> /*trait*/ QVariant_FreeQVariant<()> for () {
  fn FreeQVariant(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantD0Ev()};
     unsafe {_ZN8QVariantD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::save(QDataStream & ds);
impl /*struct*/ QVariant {
  pub fn save<RetType, T: QVariant_save<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QVariant_save<RetType> {
  fn save(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void QVariant::save(QDataStream & ds);
impl<'a> /*trait*/ QVariant_save<()> for (QDataStream) {
  fn save(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant4saveER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK8QVariant4saveER11QDataStream(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTime QVariant::toTime();
impl /*struct*/ QVariant {
  pub fn toTime<RetType, T: QVariant_toTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toTime(self);
    // return 1;
  }
}

pub trait QVariant_toTime<RetType> {
  fn toTime(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QTime QVariant::toTime();
impl<'a> /*trait*/ QVariant_toTime<QTime> for () {
  fn toTime(self , rsthis: &mut QVariant) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toTimeEv()};
    let mut ret = unsafe {_ZNK8QVariant6toTimeEv(rsthis.qclsinst)};
    let mut ret1 = QTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QLine & line);
impl<'a> /*trait*/ QVariant_NewQVariant for (QLine) {
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

  // proto:  void * QVariant::data();
impl /*struct*/ QVariant {
  pub fn data<RetType, T: QVariant_data<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QVariant_data<RetType> {
  fn data(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void * QVariant::data();
impl<'a> /*trait*/ QVariant_data<()> for () {
  fn data(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4dataEv()};
     unsafe {_ZN8QVariant4dataEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QTime & time);
impl<'a> /*trait*/ QVariant_NewQVariant for (QTime) {
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

  // proto:  void QVariant::QVariant(const QDateTime & datetime);
impl<'a> /*trait*/ QVariant_NewQVariant for (QDateTime) {
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

  // proto:  bool QVariant::convert(int targetTypeId);
impl /*struct*/ QVariant {
  pub fn convert<RetType, T: QVariant_convert<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.convert(self);
    // return 1;
  }
}

pub trait QVariant_convert<RetType> {
  fn convert(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  bool QVariant::convert(int targetTypeId);
impl<'a> /*trait*/ QVariant_convert<i8> for (i32) {
  fn convert(self , rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant7convertEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QVariant7convertEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegExp QVariant::toRegExp();
impl /*struct*/ QVariant {
  pub fn toRegExp<RetType, T: QVariant_toRegExp<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toRegExp(self);
    // return 1;
  }
}

pub trait QVariant_toRegExp<RetType> {
  fn toRegExp(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QRegExp QVariant::toRegExp();
impl<'a> /*trait*/ QVariant_toRegExp<QRegExp> for () {
  fn toRegExp(self , rsthis: &mut QVariant) -> QRegExp {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toRegExpEv()};
    let mut ret = unsafe {_ZNK8QVariant8toRegExpEv(rsthis.qclsinst)};
    let mut ret1 = QRegExp{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QVariant::toPointF();
impl /*struct*/ QVariant {
  pub fn toPointF<RetType, T: QVariant_toPointF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPointF(self);
    // return 1;
  }
}

pub trait QVariant_toPointF<RetType> {
  fn toPointF(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QPointF QVariant::toPointF();
impl<'a> /*trait*/ QVariant_toPointF<QPointF> for () {
  fn toPointF(self , rsthis: &mut QVariant) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant8toPointFEv()};
    let mut ret = unsafe {_ZNK8QVariant8toPointFEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(QChar qchar);
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

  // proto: static const char * QVariant::typeToName(int typeId);
impl /*struct*/ QVariant {
  pub fn typeToName_s<RetType, T: QVariant_typeToName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.typeToName_s();
    // return 1;
  }
}

pub trait QVariant_typeToName_s<RetType> {
  fn typeToName_s(self ) -> RetType;
}

  // proto: static const char * QVariant::typeToName(int typeId);
impl<'a> /*trait*/ QVariant_typeToName_s<String> for (i32) {
  fn typeToName_s(self ) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant10typeToNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN8QVariant10typeToNameEi(arg0)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  QSizeF QVariant::toSizeF();
impl /*struct*/ QVariant {
  pub fn toSizeF<RetType, T: QVariant_toSizeF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toSizeF(self);
    // return 1;
  }
}

pub trait QVariant_toSizeF<RetType> {
  fn toSizeF(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QSizeF QVariant::toSizeF();
impl<'a> /*trait*/ QVariant_toSizeF<QSizeF> for () {
  fn toSizeF(self , rsthis: &mut QVariant) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant7toSizeFEv()};
    let mut ret = unsafe {_ZNK8QVariant7toSizeFEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::swap(QVariant & other);
impl /*struct*/ QVariant {
  pub fn swap<RetType, T: QVariant_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QVariant_swap<RetType> {
  fn swap(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void QVariant::swap(QVariant & other);
impl<'a> /*trait*/ QVariant_swap<()> for (QVariant) {
  fn swap(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QVariant4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(int typeId, const void * copy);
impl<'a> /*trait*/ QVariant_NewQVariant for (i32, *mut c_void) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1EiPKv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut c_void;
    unsafe {_ZN8QVariantC1EiPKv(qthis, arg0, arg1)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QEasingCurve & easing);
impl<'a> /*trait*/ QVariant_NewQVariant for (QEasingCurve) {
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

  // proto:  void QVariant::clear();
impl /*struct*/ QVariant {
  pub fn clear<RetType, T: QVariant_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QVariant_clear<RetType> {
  fn clear(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  void QVariant::clear();
impl<'a> /*trait*/ QVariant_clear<()> for () {
  fn clear(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariant5clearEv()};
     unsafe {_ZN8QVariant5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRect QVariant::toRect();
impl /*struct*/ QVariant {
  pub fn toRect<RetType, T: QVariant_toRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toRect(self);
    // return 1;
  }
}

pub trait QVariant_toRect<RetType> {
  fn toRect(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QRect QVariant::toRect();
impl<'a> /*trait*/ QVariant_toRect<QRect> for () {
  fn toRect(self , rsthis: &mut QVariant) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toRectEv()};
    let mut ret = unsafe {_ZNK8QVariant6toRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QByteArray & bytearray);
impl<'a> /*trait*/ QVariant_NewQVariant for (QByteArray) {
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

  // proto:  void QVariant::QVariant(qlonglong ll);
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

  // proto:  void QVariant::QVariant(qulonglong ull);
impl<'a> /*trait*/ QVariant_NewQVariant for (u64) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Ey()};
    let arg0 = self  as c_ulonglong;
    unsafe {_ZN8QVariantC1Ey(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant();
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

  // proto:  bool QVariant::toBool();
impl /*struct*/ QVariant {
  pub fn toBool<RetType, T: QVariant_toBool<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toBool(self);
    // return 1;
  }
}

pub trait QVariant_toBool<RetType> {
  fn toBool(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  bool QVariant::toBool();
impl<'a> /*trait*/ QVariant_toBool<i8> for () {
  fn toBool(self , rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant6toBoolEv()};
    let mut ret = unsafe {_ZNK8QVariant6toBoolEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(uint ui);
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

  // proto:  void QVariant::QVariant(int i);
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

  // proto:  void QVariant::QVariant(float f);
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

  // proto:  void QVariant::QVariant(double d);
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

  // proto:  void QVariant::QVariant(bool b);
impl<'a> /*trait*/ QVariant_NewQVariant for (i8) {
  fn NewQVariant(self) -> QVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QVariantC1Eb()};
    let arg0 = self  as c_char;
    unsafe {_ZN8QVariantC1Eb(qthis, arg0)};
    let rsthis = QVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qulonglong QVariant::toULongLong(bool * ok);
impl /*struct*/ QVariant {
  pub fn toULongLong<RetType, T: QVariant_toULongLong<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QVariant_toULongLong<RetType> {
  fn toULongLong(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  qulonglong QVariant::toULongLong(bool * ok);
impl<'a> /*trait*/ QVariant_toULongLong<u64> for (&'a mut Vec<i8>) {
  fn toULongLong(self , rsthis: &mut QVariant) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toULongLongEPb()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZNK8QVariant11toULongLongEPb(rsthis.qclsinst, arg0)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QJsonValue QVariant::toJsonValue();
impl /*struct*/ QVariant {
  pub fn toJsonValue<RetType, T: QVariant_toJsonValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toJsonValue(self);
    // return 1;
  }
}

pub trait QVariant_toJsonValue<RetType> {
  fn toJsonValue(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QJsonValue QVariant::toJsonValue();
impl<'a> /*trait*/ QVariant_toJsonValue<()> for () {
  fn toJsonValue(self , rsthis: &mut QVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant11toJsonValueEv()};
     unsafe {_ZNK8QVariant11toJsonValueEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QDateTime QVariant::toDateTime();
impl /*struct*/ QVariant {
  pub fn toDateTime<RetType, T: QVariant_toDateTime<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toDateTime(self);
    // return 1;
  }
}

pub trait QVariant_toDateTime<RetType> {
  fn toDateTime(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QDateTime QVariant::toDateTime();
impl<'a> /*trait*/ QVariant_toDateTime<QDateTime> for () {
  fn toDateTime(self , rsthis: &mut QVariant) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toDateTimeEv()};
    let mut ret = unsafe {_ZNK8QVariant10toDateTimeEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QVariant::isDetached();
impl /*struct*/ QVariant {
  pub fn isDetached<RetType, T: QVariant_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QVariant_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  bool QVariant::isDetached();
impl<'a> /*trait*/ QVariant_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QVariant) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10isDetachedEv()};
    let mut ret = unsafe {_ZNK8QVariant10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QEasingCurve QVariant::toEasingCurve();
impl /*struct*/ QVariant {
  pub fn toEasingCurve<RetType, T: QVariant_toEasingCurve<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toEasingCurve(self);
    // return 1;
  }
}

pub trait QVariant_toEasingCurve<RetType> {
  fn toEasingCurve(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QEasingCurve QVariant::toEasingCurve();
impl<'a> /*trait*/ QVariant_toEasingCurve<QEasingCurve> for () {
  fn toEasingCurve(self , rsthis: &mut QVariant) -> QEasingCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant13toEasingCurveEv()};
    let mut ret = unsafe {_ZNK8QVariant13toEasingCurveEv(rsthis.qclsinst)};
    let mut ret1 = QEasingCurve{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QUrl & url);
impl<'a> /*trait*/ QVariant_NewQVariant for (QUrl) {
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

  // proto:  void QVariant::QVariant(const QVariant & other);
impl<'a> /*trait*/ QVariant_NewQVariant for (QVariant) {
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

  // proto:  QBitArray QVariant::toBitArray();
impl /*struct*/ QVariant {
  pub fn toBitArray<RetType, T: QVariant_toBitArray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toBitArray(self);
    // return 1;
  }
}

pub trait QVariant_toBitArray<RetType> {
  fn toBitArray(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QBitArray QVariant::toBitArray();
impl<'a> /*trait*/ QVariant_toBitArray<QBitArray> for () {
  fn toBitArray(self , rsthis: &mut QVariant) -> QBitArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant10toBitArrayEv()};
    let mut ret = unsafe {_ZNK8QVariant10toBitArrayEv(rsthis.qclsinst)};
    let mut ret1 = QBitArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRegularExpression QVariant::toRegularExpression();
impl /*struct*/ QVariant {
  pub fn toRegularExpression<RetType, T: QVariant_toRegularExpression<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toRegularExpression(self);
    // return 1;
  }
}

pub trait QVariant_toRegularExpression<RetType> {
  fn toRegularExpression(self , rsthis: &mut QVariant) -> RetType;
}

  // proto:  QRegularExpression QVariant::toRegularExpression();
impl<'a> /*trait*/ QVariant_toRegularExpression<QRegularExpression> for () {
  fn toRegularExpression(self , rsthis: &mut QVariant) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QVariant19toRegularExpressionEv()};
    let mut ret = unsafe {_ZNK8QVariant19toRegularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QVariant::QVariant(const QRegularExpression & re);
impl<'a> /*trait*/ QVariant_NewQVariant for (QRegularExpression) {
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

  // proto:  void QVariant::QVariant(const QStringList & stringlist);
impl<'a> /*trait*/ QVariant_NewQVariant for (QStringList) {
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

