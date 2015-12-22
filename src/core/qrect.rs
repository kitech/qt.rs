// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qrect.h
// dst-file: /src/core/qrect.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use std::ops::Deref;
use super::qpoint::QPoint; // 773
use super::qsize::QSize; // 773
use super::qmargins::QMargins; // 773
use super::qpoint::QPointF; // 773
use super::qsize::QSizeF; // 773
// use super::qrect::QRect; // 773
use super::qmargins::QMarginsF; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  int QRect::right();
  fn _ZNK5QRect5rightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::moveTo(const QPoint & p);
  fn _ZN5QRect6moveToERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::moveTopLeft(const QPoint & p);
  fn _ZN5QRect11moveTopLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::moveRight(int pos);
  fn _ZN5QRect9moveRightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::QRect(const QPoint & topleft, const QPoint & bottomright);
  fn _ZN5QRectC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QRect QRect::translated(int dx, int dy);
  fn _ZNK5QRect10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QPoint QRect::center();
  fn _ZNK5QRect6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::moveTopRight(const QPoint & p);
  fn _ZN5QRect12moveTopRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::setLeft(int pos);
  fn _ZN5QRect7setLeftEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QRect::left();
  fn _ZNK5QRect4leftEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QRect::intersected(const QRect & other);
  fn _ZNK5QRect11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRect::contains(int x, int y, bool proper);
  fn _ZNK5QRect8containsEiib(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_char) -> c_char;
  // proto:  QPoint QRect::bottomRight();
  fn _ZNK5QRect11bottomRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRect::isValid();
  fn _ZNK5QRect7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QSize QRect::size();
  fn _ZNK5QRect4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QRect::united(const QRect & other);
  fn _ZNK5QRect6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRect::adjust(int x1, int y1, int x2, int y2);
  fn _ZN5QRect6adjustEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  bool QRect::isNull();
  fn _ZNK5QRect6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QRect::setBottom(int pos);
  fn _ZN5QRect9setBottomEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::setSize(const QSize & s);
  fn _ZN5QRect7setSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QRect::y();
  fn _ZNK5QRect1yEv(qthis: *mut c_void);
  // proto:  int QRect::x();
  fn _ZNK5QRect1xEv(qthis: *mut c_void);
  // proto:  QRect QRect::adjusted(int x1, int y1, int x2, int y2);
  fn _ZNK5QRect8adjustedEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QRect::QRect(const QPoint & topleft, const QSize & size);
  fn _ZN5QRectC1ERK6QPointRK5QSize(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  int QRect::height();
  fn _ZNK5QRect6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::QRect(int left, int top, int width, int height);
  fn _ZN5QRectC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QRect::moveBottomLeft(const QPoint & p);
  fn _ZN5QRect14moveBottomLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QRect::top();
  fn _ZNK5QRect3topEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::moveTo(int x, int t);
  fn _ZN5QRect6moveToEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QRect::getRect(int * x, int * y, int * w, int * h);
  fn _ZNK5QRect7getRectEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  bool QRect::contains(const QRect & r, bool proper);
  fn _ZNK5QRect8containsERKS_b(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char) -> c_char;
  // proto:  QRect QRect::marginsRemoved(const QMargins & margins);
  fn _ZNK5QRect14marginsRemovedERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRect::translate(int dx, int dy);
  fn _ZN5QRect9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QPoint QRect::topLeft();
  fn _ZNK5QRect7topLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRect::contains(const QPoint & p, bool proper);
  fn _ZNK5QRect8containsERK6QPointb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char) -> c_char;
  // proto:  int QRect::width();
  fn _ZNK5QRect5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::setRect(int x, int y, int w, int h);
  fn _ZN5QRect7setRectEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QRect::moveCenter(const QPoint & p);
  fn _ZN5QRect10moveCenterERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QRect::intersects(const QRect & r);
  fn _ZNK5QRect10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QRect::setTopRight(const QPoint & p);
  fn _ZN5QRect11setTopRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::setCoords(int x1, int y1, int x2, int y2);
  fn _ZN5QRect9setCoordsEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QRect::translate(const QPoint & p);
  fn _ZN5QRect9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::moveBottom(int pos);
  fn _ZN5QRect10moveBottomEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::setBottomLeft(const QPoint & p);
  fn _ZN5QRect13setBottomLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::getCoords(int * x1, int * y1, int * x2, int * y2);
  fn _ZNK5QRect9getCoordsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int);
  // proto:  QPoint QRect::topRight();
  fn _ZNK5QRect8topRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::setBottomRight(const QPoint & p);
  fn _ZN5QRect14setBottomRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::setHeight(int h);
  fn _ZN5QRect9setHeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QRect::isEmpty();
  fn _ZNK5QRect7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QRect::contains(int x, int y);
  fn _ZNK5QRect8containsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> c_char;
  // proto:  void QRect::moveBottomRight(const QPoint & p);
  fn _ZN5QRect15moveBottomRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPoint QRect::bottomLeft();
  fn _ZNK5QRect10bottomLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::setTop(int pos);
  fn _ZN5QRect6setTopEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QRect::bottom();
  fn _ZNK5QRect6bottomEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::QRect();
  fn _ZN5QRectC1Ev(qthis: *mut c_void);
  // proto:  QRect QRect::marginsAdded(const QMargins & margins);
  fn _ZNK5QRect12marginsAddedERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QRect::normalized();
  fn _ZNK5QRect10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::setWidth(int w);
  fn _ZN5QRect8setWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::setY(int y);
  fn _ZN5QRect4setYEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::moveTop(int pos);
  fn _ZN5QRect7moveTopEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::setX(int x);
  fn _ZN5QRect4setXEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::setRight(int pos);
  fn _ZN5QRect8setRightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QRect::setTopLeft(const QPoint & p);
  fn _ZN5QRect10setTopLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRect::moveLeft(int pos);
  fn _ZN5QRect8moveLeftEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QRect QRect::translated(const QPoint & p);
  fn _ZNK5QRect10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::QRectF();
  fn _ZN6QRectFC1Ev(qthis: *mut c_void);
  // proto:  void QRectF::moveBottomRight(const QPointF & p);
  fn _ZN6QRectF15moveBottomRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRectF::moveTo(qreal x, qreal y);
  fn _ZN6QRectF6moveToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  qreal QRectF::top();
  fn _ZNK6QRectF3topEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QRectF::bottomLeft();
  fn _ZNK6QRectF10bottomLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setHeight(qreal h);
  fn _ZN6QRectF9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::setSize(const QSizeF & s);
  fn _ZN6QRectF7setSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRectF::QRectF(const QPointF & topleft, const QPointF & bottomRight);
  fn _ZN6QRectFC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QRectF::moveTo(const QPointF & p);
  fn _ZN6QRectF6moveToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QRectF::toAlignedRect();
  fn _ZNK6QRectF13toAlignedRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setRight(qreal pos);
  fn _ZN6QRectF8setRightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::setBottomLeft(const QPointF & p);
  fn _ZN6QRectF13setBottomLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QRectF::topRight();
  fn _ZNK6QRectF8topRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QRectF::size();
  fn _ZNK6QRectF4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QRectF6adjustEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QRectF::moveRight(qreal pos);
  fn _ZN6QRectF9moveRightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QRectF::y();
  fn _ZNK6QRectF1yEv(qthis: *mut c_void);
  // proto:  QPointF QRectF::bottomRight();
  fn _ZNK6QRectF11bottomRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setBottom(qreal pos);
  fn _ZN6QRectF9setBottomEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::moveBottomLeft(const QPointF & p);
  fn _ZN6QRectF14moveBottomLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRectF::moveBottom(qreal pos);
  fn _ZN6QRectF10moveBottomEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
  fn _ZNK6QRectF7getRectEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  qreal QRectF::x();
  fn _ZNK6QRectF1xEv(qthis: *mut c_void);
  // proto:  qreal QRectF::bottom();
  fn _ZNK6QRectF6bottomEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QRectF::isNull();
  fn _ZNK6QRectF6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QRectF::QRectF(const QPointF & topleft, const QSizeF & size);
  fn _ZN6QRectFC1ERK7QPointFRK6QSizeF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QRectF::setWidth(qreal w);
  fn _ZN6QRectF8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QRectF::height();
  fn _ZNK6QRectF6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::translate(const QPointF & p);
  fn _ZN6QRectF9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRectF::moveCenter(const QPointF & p);
  fn _ZN6QRectF10moveCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QRectF::contains(const QRectF & r);
  fn _ZNK6QRectF8containsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QRectF QRectF::marginsRemoved(const QMarginsF & margins);
  fn _ZNK6QRectF14marginsRemovedERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRectF::contains(qreal x, qreal y);
  fn _ZNK6QRectF8containsEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> c_char;
  // proto:  void QRectF::setX(qreal pos);
  fn _ZN6QRectF4setXEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN6QRectF7setRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  QPointF QRectF::center();
  fn _ZNK6QRectF6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setLeft(qreal pos);
  fn _ZN6QRectF7setLeftEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QRectF QRectF::intersected(const QRectF & other);
  fn _ZNK6QRectF11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QRectF::topLeft();
  fn _ZNK6QRectF7topLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QRectF::left();
  fn _ZNK6QRectF4leftEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::setY(qreal pos);
  fn _ZN6QRectF4setYEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::moveTopLeft(const QPointF & p);
  fn _ZN6QRectF11moveTopLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QRectF::width();
  fn _ZNK6QRectF5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::setTop(qreal pos);
  fn _ZN6QRectF6setTopEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QRectF::isValid();
  fn _ZNK6QRectF7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QRectF::translate(qreal dx, qreal dy);
  fn _ZN6QRectF9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QRectF::QRectF(qreal left, qreal top, qreal width, qreal height);
  fn _ZN6QRectFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  QRect QRectF::toRect();
  fn _ZNK6QRectF6toRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::moveLeft(qreal pos);
  fn _ZN6QRectF8moveLeftEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::setTopLeft(const QPointF & p);
  fn _ZN6QRectF10setTopLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRectF::setBottomRight(const QPointF & p);
  fn _ZN6QRectF14setBottomRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QRectF::marginsAdded(const QMarginsF & margins);
  fn _ZNK6QRectF12marginsAddedERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QRectF::translated(const QPointF & p);
  fn _ZNK6QRectF10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QRectF::normalized();
  fn _ZNK6QRectF10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
  fn _ZNK6QRectF9getCoordsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QRectF::setTopRight(const QPointF & p);
  fn _ZN6QRectF11setTopRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QRectF::contains(const QPointF & p);
  fn _ZNK6QRectF8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QRectF::intersects(const QRectF & r);
  fn _ZNK6QRectF10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QRectF::moveTop(qreal pos);
  fn _ZN6QRectF7moveTopEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QRectF9setCoordsEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  QRectF QRectF::translated(qreal dx, qreal dy);
  fn _ZNK6QRectF10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QRectF::isEmpty();
  fn _ZNK6QRectF7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QRectF::moveTopRight(const QPointF & p);
  fn _ZN6QRectF12moveTopRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QRectF::united(const QRectF & other);
  fn _ZNK6QRectF6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QRectF::right();
  fn _ZNK6QRectF5rightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::QRectF(const QRect & rect);
  fn _ZN6QRectFC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZNK6QRectF8adjustedEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QRect)=16
pub struct QRect {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QRectF)=32
pub struct QRectF {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRect {
  pub fn inheritFrom(qthis: *mut c_void) -> QRect {
    return QRect{qclsinst: qthis};
  }
}
  // proto:  int QRect::right();
impl /*struct*/ QRect {
  pub fn right<RetType, T: QRect_right<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QRect_right<RetType> {
  fn right(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::right();
impl<'a> /*trait*/ QRect_right<i32> for () {
  fn right(self , rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect5rightEv()};
    let mut ret = unsafe {_ZNK5QRect5rightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRect::moveTo(const QPoint & p);
impl /*struct*/ QRect {
  pub fn moveTo<RetType, T: QRect_moveTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTo(self);
    // return 1;
  }
}

pub trait QRect_moveTo<RetType> {
  fn moveTo(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveTo(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTo<()> for (QPoint) {
  fn moveTo(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6moveToERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect6moveToERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::moveTopLeft(const QPoint & p);
impl /*struct*/ QRect {
  pub fn moveTopLeft<RetType, T: QRect_moveTopLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTopLeft(self);
    // return 1;
  }
}

pub trait QRect_moveTopLeft<RetType> {
  fn moveTopLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveTopLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTopLeft<()> for (QPoint) {
  fn moveTopLeft(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect11moveTopLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect11moveTopLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::moveRight(int pos);
impl /*struct*/ QRect {
  pub fn moveRight<RetType, T: QRect_moveRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveRight(self);
    // return 1;
  }
}

pub trait QRect_moveRight<RetType> {
  fn moveRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveRight(int pos);
impl<'a> /*trait*/ QRect_moveRight<()> for (i32) {
  fn moveRight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9moveRightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect9moveRightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::QRect(const QPoint & topleft, const QPoint & bottomright);
impl /*struct*/ QRect {
  pub fn NewQRect<T: QRect_NewQRect>(value: T) -> QRect {
    let rsthis = value.NewQRect();
    return rsthis;
    // return 1;
  }
}

pub trait QRect_NewQRect {
  fn NewQRect(self) -> QRect;
}

  // proto:  void QRect::QRect(const QPoint & topleft, const QPoint & bottomright);
impl<'a> /*trait*/ QRect_NewQRect for (QPoint, QPoint) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QRectC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRect QRect::translated(int dx, int dy);
impl /*struct*/ QRect {
  pub fn translated<RetType, T: QRect_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QRect_translated<RetType> {
  fn translated(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::translated(int dx, int dy);
impl<'a> /*trait*/ QRect_translated<QRect> for (i32, i32) {
  fn translated(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QRect10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QRect::center();
impl /*struct*/ QRect {
  pub fn center<RetType, T: QRect_center<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QRect_center<RetType> {
  fn center(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QPoint QRect::center();
impl<'a> /*trait*/ QRect_center<QPoint> for () {
  fn center(self , rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6centerEv()};
    let mut ret = unsafe {_ZNK5QRect6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::moveTopRight(const QPoint & p);
impl /*struct*/ QRect {
  pub fn moveTopRight<RetType, T: QRect_moveTopRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTopRight(self);
    // return 1;
  }
}

pub trait QRect_moveTopRight<RetType> {
  fn moveTopRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveTopRight(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTopRight<()> for (QPoint) {
  fn moveTopRight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect12moveTopRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect12moveTopRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setLeft(int pos);
impl /*struct*/ QRect {
  pub fn setLeft<RetType, T: QRect_setLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLeft(self);
    // return 1;
  }
}

pub trait QRect_setLeft<RetType> {
  fn setLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setLeft(int pos);
impl<'a> /*trait*/ QRect_setLeft<()> for (i32) {
  fn setLeft(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setLeftEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect7setLeftEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QRect::left();
impl /*struct*/ QRect {
  pub fn left<RetType, T: QRect_left<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QRect_left<RetType> {
  fn left(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::left();
impl<'a> /*trait*/ QRect_left<i32> for () {
  fn left(self , rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect4leftEv()};
    let mut ret = unsafe {_ZNK5QRect4leftEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QRect::intersected(const QRect & other);
impl /*struct*/ QRect {
  pub fn intersected<RetType, T: QRect_intersected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QRect_intersected<RetType> {
  fn intersected(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::intersected(const QRect & other);
impl<'a> /*trait*/ QRect_intersected<QRect> for (QRect) {
  fn intersected(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRect::contains(int x, int y, bool proper);
impl /*struct*/ QRect {
  pub fn contains<RetType, T: QRect_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QRect_contains<RetType> {
  fn contains(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  bool QRect::contains(int x, int y, bool proper);
impl<'a> /*trait*/ QRect_contains<i8> for (i32, i32, i8) {
  fn contains(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsEiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_char;
    let mut ret = unsafe {_ZNK5QRect8containsEiib(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPoint QRect::bottomRight();
impl /*struct*/ QRect {
  pub fn bottomRight<RetType, T: QRect_bottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomRight(self);
    // return 1;
  }
}

pub trait QRect_bottomRight<RetType> {
  fn bottomRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QPoint QRect::bottomRight();
impl<'a> /*trait*/ QRect_bottomRight<QPoint> for () {
  fn bottomRight(self , rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect11bottomRightEv()};
    let mut ret = unsafe {_ZNK5QRect11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRect::isValid();
impl /*struct*/ QRect {
  pub fn isValid<RetType, T: QRect_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRect_isValid<RetType> {
  fn isValid(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  bool QRect::isValid();
impl<'a> /*trait*/ QRect_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7isValidEv()};
    let mut ret = unsafe {_ZNK5QRect7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QRect::size();
impl /*struct*/ QRect {
  pub fn size<RetType, T: QRect_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QRect_size<RetType> {
  fn size(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QSize QRect::size();
impl<'a> /*trait*/ QRect_size<QSize> for () {
  fn size(self , rsthis: &mut QRect) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect4sizeEv()};
    let mut ret = unsafe {_ZNK5QRect4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QRect::united(const QRect & other);
impl /*struct*/ QRect {
  pub fn united<RetType, T: QRect_united<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QRect_united<RetType> {
  fn united(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::united(const QRect & other);
impl<'a> /*trait*/ QRect_united<QRect> for (QRect) {
  fn united(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::adjust(int x1, int y1, int x2, int y2);
impl /*struct*/ QRect {
  pub fn adjust<RetType, T: QRect_adjust<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.adjust(self);
    // return 1;
  }
}

pub trait QRect_adjust<RetType> {
  fn adjust(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::adjust(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_adjust<()> for (i32, i32, i32, i32) {
  fn adjust(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6adjustEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QRect6adjustEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QRect::isNull();
impl /*struct*/ QRect {
  pub fn isNull<RetType, T: QRect_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QRect_isNull<RetType> {
  fn isNull(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  bool QRect::isNull();
impl<'a> /*trait*/ QRect_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6isNullEv()};
    let mut ret = unsafe {_ZNK5QRect6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRect::setBottom(int pos);
impl /*struct*/ QRect {
  pub fn setBottom<RetType, T: QRect_setBottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QRect_setBottom<RetType> {
  fn setBottom(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setBottom(int pos);
impl<'a> /*trait*/ QRect_setBottom<()> for (i32) {
  fn setBottom(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setBottomEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect9setBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setSize(const QSize & s);
impl /*struct*/ QRect {
  pub fn setSize<RetType, T: QRect_setSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSize(self);
    // return 1;
  }
}

pub trait QRect_setSize<RetType> {
  fn setSize(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setSize(const QSize & s);
impl<'a> /*trait*/ QRect_setSize<()> for (QSize) {
  fn setSize(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect7setSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QRect::y();
impl /*struct*/ QRect {
  pub fn y<RetType, T: QRect_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QRect_y<RetType> {
  fn y(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::y();
impl<'a> /*trait*/ QRect_y<()> for () {
  fn y(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect1yEv()};
     unsafe {_ZNK5QRect1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QRect::x();
impl /*struct*/ QRect {
  pub fn x<RetType, T: QRect_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QRect_x<RetType> {
  fn x(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::x();
impl<'a> /*trait*/ QRect_x<()> for () {
  fn x(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect1xEv()};
     unsafe {_ZNK5QRect1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRect QRect::adjusted(int x1, int y1, int x2, int y2);
impl /*struct*/ QRect {
  pub fn adjusted<RetType, T: QRect_adjusted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.adjusted(self);
    // return 1;
  }
}

pub trait QRect_adjusted<RetType> {
  fn adjusted(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::adjusted(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_adjusted<QRect> for (i32, i32, i32, i32) {
  fn adjusted(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8adjustedEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK5QRect8adjustedEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::QRect(const QPoint & topleft, const QSize & size);
impl<'a> /*trait*/ QRect_NewQRect for (QPoint, QSize) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1ERK6QPointRK5QSize()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QRectC1ERK6QPointRK5QSize(qthis, arg0, arg1)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QRect::height();
impl /*struct*/ QRect {
  pub fn height<RetType, T: QRect_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QRect_height<RetType> {
  fn height(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::height();
impl<'a> /*trait*/ QRect_height<i32> for () {
  fn height(self , rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6heightEv()};
    let mut ret = unsafe {_ZNK5QRect6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRect::QRect(int left, int top, int width, int height);
impl<'a> /*trait*/ QRect_NewQRect for (i32, i32, i32, i32) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QRectC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRect::moveBottomLeft(const QPoint & p);
impl /*struct*/ QRect {
  pub fn moveBottomLeft<RetType, T: QRect_moveBottomLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBottomLeft(self);
    // return 1;
  }
}

pub trait QRect_moveBottomLeft<RetType> {
  fn moveBottomLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveBottomLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_moveBottomLeft<()> for (QPoint) {
  fn moveBottomLeft(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect14moveBottomLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect14moveBottomLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QRect::top();
impl /*struct*/ QRect {
  pub fn top<RetType, T: QRect_top<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QRect_top<RetType> {
  fn top(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::top();
impl<'a> /*trait*/ QRect_top<i32> for () {
  fn top(self , rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect3topEv()};
    let mut ret = unsafe {_ZNK5QRect3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRect::moveTo(int x, int t);
impl<'a> /*trait*/ QRect_moveTo<()> for (i32, i32) {
  fn moveTo(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6moveToEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QRect6moveToEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QRect::getRect(int * x, int * y, int * w, int * h);
impl /*struct*/ QRect {
  pub fn getRect<RetType, T: QRect_getRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getRect(self);
    // return 1;
  }
}

pub trait QRect_getRect<RetType> {
  fn getRect(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::getRect(int * x, int * y, int * w, int * h);
impl<'a> /*trait*/ QRect_getRect<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getRect(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7getRectEPiS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZNK5QRect7getRectEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QRect::contains(const QRect & r, bool proper);
impl<'a> /*trait*/ QRect_contains<i8> for (QRect, i8) {
  fn contains(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsERKS_b()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    let mut ret = unsafe {_ZNK5QRect8containsERKS_b(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QRect::marginsRemoved(const QMargins & margins);
impl /*struct*/ QRect {
  pub fn marginsRemoved<RetType, T: QRect_marginsRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.marginsRemoved(self);
    // return 1;
  }
}

pub trait QRect_marginsRemoved<RetType> {
  fn marginsRemoved(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::marginsRemoved(const QMargins & margins);
impl<'a> /*trait*/ QRect_marginsRemoved<QRect> for (QMargins) {
  fn marginsRemoved(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect14marginsRemovedERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect14marginsRemovedERK8QMargins(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::translate(int dx, int dy);
impl /*struct*/ QRect {
  pub fn translate<RetType, T: QRect_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QRect_translate<RetType> {
  fn translate(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::translate(int dx, int dy);
impl<'a> /*trait*/ QRect_translate<()> for (i32, i32) {
  fn translate(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QRect9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPoint QRect::topLeft();
impl /*struct*/ QRect {
  pub fn topLeft<RetType, T: QRect_topLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLeft(self);
    // return 1;
  }
}

pub trait QRect_topLeft<RetType> {
  fn topLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QPoint QRect::topLeft();
impl<'a> /*trait*/ QRect_topLeft<QPoint> for () {
  fn topLeft(self , rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7topLeftEv()};
    let mut ret = unsafe {_ZNK5QRect7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRect::contains(const QPoint & p, bool proper);
impl<'a> /*trait*/ QRect_contains<i8> for (QPoint, i8) {
  fn contains(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsERK6QPointb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    let mut ret = unsafe {_ZNK5QRect8containsERK6QPointb(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QRect::width();
impl /*struct*/ QRect {
  pub fn width<RetType, T: QRect_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QRect_width<RetType> {
  fn width(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::width();
impl<'a> /*trait*/ QRect_width<i32> for () {
  fn width(self , rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect5widthEv()};
    let mut ret = unsafe {_ZNK5QRect5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRect::setRect(int x, int y, int w, int h);
impl /*struct*/ QRect {
  pub fn setRect<RetType, T: QRect_setRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRect(self);
    // return 1;
  }
}

pub trait QRect_setRect<RetType> {
  fn setRect(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setRect(int x, int y, int w, int h);
impl<'a> /*trait*/ QRect_setRect<()> for (i32, i32, i32, i32) {
  fn setRect(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QRect7setRectEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QRect::moveCenter(const QPoint & p);
impl /*struct*/ QRect {
  pub fn moveCenter<RetType, T: QRect_moveCenter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveCenter(self);
    // return 1;
  }
}

pub trait QRect_moveCenter<RetType> {
  fn moveCenter(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveCenter(const QPoint & p);
impl<'a> /*trait*/ QRect_moveCenter<()> for (QPoint) {
  fn moveCenter(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10moveCenterERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect10moveCenterERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRect::intersects(const QRect & r);
impl /*struct*/ QRect {
  pub fn intersects<RetType, T: QRect_intersects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QRect_intersects<RetType> {
  fn intersects(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  bool QRect::intersects(const QRect & r);
impl<'a> /*trait*/ QRect_intersects<i8> for (QRect) {
  fn intersects(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRect::setTopRight(const QPoint & p);
impl /*struct*/ QRect {
  pub fn setTopRight<RetType, T: QRect_setTopRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopRight(self);
    // return 1;
  }
}

pub trait QRect_setTopRight<RetType> {
  fn setTopRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setTopRight(const QPoint & p);
impl<'a> /*trait*/ QRect_setTopRight<()> for (QPoint) {
  fn setTopRight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect11setTopRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect11setTopRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setCoords(int x1, int y1, int x2, int y2);
impl /*struct*/ QRect {
  pub fn setCoords<RetType, T: QRect_setCoords<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCoords(self);
    // return 1;
  }
}

pub trait QRect_setCoords<RetType> {
  fn setCoords(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setCoords(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_setCoords<()> for (i32, i32, i32, i32) {
  fn setCoords(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setCoordsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QRect9setCoordsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QRect::translate(const QPoint & p);
impl<'a> /*trait*/ QRect_translate<()> for (QPoint) {
  fn translate(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::moveBottom(int pos);
impl /*struct*/ QRect {
  pub fn moveBottom<RetType, T: QRect_moveBottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBottom(self);
    // return 1;
  }
}

pub trait QRect_moveBottom<RetType> {
  fn moveBottom(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveBottom(int pos);
impl<'a> /*trait*/ QRect_moveBottom<()> for (i32) {
  fn moveBottom(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10moveBottomEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect10moveBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setBottomLeft(const QPoint & p);
impl /*struct*/ QRect {
  pub fn setBottomLeft<RetType, T: QRect_setBottomLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomLeft(self);
    // return 1;
  }
}

pub trait QRect_setBottomLeft<RetType> {
  fn setBottomLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setBottomLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_setBottomLeft<()> for (QPoint) {
  fn setBottomLeft(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect13setBottomLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect13setBottomLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::getCoords(int * x1, int * y1, int * x2, int * y2);
impl /*struct*/ QRect {
  pub fn getCoords<RetType, T: QRect_getCoords<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getCoords(self);
    // return 1;
  }
}

pub trait QRect_getCoords<RetType> {
  fn getCoords(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::getCoords(int * x1, int * y1, int * x2, int * y2);
impl<'a> /*trait*/ QRect_getCoords<()> for (&'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>, &'a mut Vec<i32>) {
  fn getCoords(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect9getCoordsEPiS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_int;
    let arg2 = self.2.as_ptr()  as *mut c_int;
    let arg3 = self.3.as_ptr()  as *mut c_int;
     unsafe {_ZNK5QRect9getCoordsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QPoint QRect::topRight();
impl /*struct*/ QRect {
  pub fn topRight<RetType, T: QRect_topRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topRight(self);
    // return 1;
  }
}

pub trait QRect_topRight<RetType> {
  fn topRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QPoint QRect::topRight();
impl<'a> /*trait*/ QRect_topRight<QPoint> for () {
  fn topRight(self , rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8topRightEv()};
    let mut ret = unsafe {_ZNK5QRect8topRightEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::setBottomRight(const QPoint & p);
impl /*struct*/ QRect {
  pub fn setBottomRight<RetType, T: QRect_setBottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomRight(self);
    // return 1;
  }
}

pub trait QRect_setBottomRight<RetType> {
  fn setBottomRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setBottomRight(const QPoint & p);
impl<'a> /*trait*/ QRect_setBottomRight<()> for (QPoint) {
  fn setBottomRight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect14setBottomRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect14setBottomRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setHeight(int h);
impl /*struct*/ QRect {
  pub fn setHeight<RetType, T: QRect_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QRect_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setHeight(int h);
impl<'a> /*trait*/ QRect_setHeight<()> for (i32) {
  fn setHeight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRect::isEmpty();
impl /*struct*/ QRect {
  pub fn isEmpty<RetType, T: QRect_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QRect_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  bool QRect::isEmpty();
impl<'a> /*trait*/ QRect_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7isEmptyEv()};
    let mut ret = unsafe {_ZNK5QRect7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRect::contains(int x, int y);
impl<'a> /*trait*/ QRect_contains<i8> for (i32, i32) {
  fn contains(self , rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QRect8containsEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRect::moveBottomRight(const QPoint & p);
impl /*struct*/ QRect {
  pub fn moveBottomRight<RetType, T: QRect_moveBottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBottomRight(self);
    // return 1;
  }
}

pub trait QRect_moveBottomRight<RetType> {
  fn moveBottomRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveBottomRight(const QPoint & p);
impl<'a> /*trait*/ QRect_moveBottomRight<()> for (QPoint) {
  fn moveBottomRight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect15moveBottomRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect15moveBottomRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPoint QRect::bottomLeft();
impl /*struct*/ QRect {
  pub fn bottomLeft<RetType, T: QRect_bottomLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomLeft(self);
    // return 1;
  }
}

pub trait QRect_bottomLeft<RetType> {
  fn bottomLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QPoint QRect::bottomLeft();
impl<'a> /*trait*/ QRect_bottomLeft<QPoint> for () {
  fn bottomLeft(self , rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10bottomLeftEv()};
    let mut ret = unsafe {_ZNK5QRect10bottomLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::setTop(int pos);
impl /*struct*/ QRect {
  pub fn setTop<RetType, T: QRect_setTop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QRect_setTop<RetType> {
  fn setTop(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setTop(int pos);
impl<'a> /*trait*/ QRect_setTop<()> for (i32) {
  fn setTop(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6setTopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect6setTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QRect::bottom();
impl /*struct*/ QRect {
  pub fn bottom<RetType, T: QRect_bottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QRect_bottom<RetType> {
  fn bottom(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  int QRect::bottom();
impl<'a> /*trait*/ QRect_bottom<i32> for () {
  fn bottom(self , rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6bottomEv()};
    let mut ret = unsafe {_ZNK5QRect6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRect::QRect();
impl<'a> /*trait*/ QRect_NewQRect for () {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1Ev()};
    unsafe {_ZN5QRectC1Ev(qthis)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRect QRect::marginsAdded(const QMargins & margins);
impl /*struct*/ QRect {
  pub fn marginsAdded<RetType, T: QRect_marginsAdded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.marginsAdded(self);
    // return 1;
  }
}

pub trait QRect_marginsAdded<RetType> {
  fn marginsAdded(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::marginsAdded(const QMargins & margins);
impl<'a> /*trait*/ QRect_marginsAdded<QRect> for (QMargins) {
  fn marginsAdded(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect12marginsAddedERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect12marginsAddedERK8QMargins(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QRect::normalized();
impl /*struct*/ QRect {
  pub fn normalized<RetType, T: QRect_normalized<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.normalized(self);
    // return 1;
  }
}

pub trait QRect_normalized<RetType> {
  fn normalized(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  QRect QRect::normalized();
impl<'a> /*trait*/ QRect_normalized<QRect> for () {
  fn normalized(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10normalizedEv()};
    let mut ret = unsafe {_ZNK5QRect10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRect::setWidth(int w);
impl /*struct*/ QRect {
  pub fn setWidth<RetType, T: QRect_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QRect_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setWidth(int w);
impl<'a> /*trait*/ QRect_setWidth<()> for (i32) {
  fn setWidth(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setY(int y);
impl /*struct*/ QRect {
  pub fn setY<RetType, T: QRect_setY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QRect_setY<RetType> {
  fn setY(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setY(int y);
impl<'a> /*trait*/ QRect_setY<()> for (i32) {
  fn setY(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::moveTop(int pos);
impl /*struct*/ QRect {
  pub fn moveTop<RetType, T: QRect_moveTop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTop(self);
    // return 1;
  }
}

pub trait QRect_moveTop<RetType> {
  fn moveTop(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveTop(int pos);
impl<'a> /*trait*/ QRect_moveTop<()> for (i32) {
  fn moveTop(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7moveTopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect7moveTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setX(int x);
impl /*struct*/ QRect {
  pub fn setX<RetType, T: QRect_setX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QRect_setX<RetType> {
  fn setX(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setX(int x);
impl<'a> /*trait*/ QRect_setX<()> for (i32) {
  fn setX(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setRight(int pos);
impl /*struct*/ QRect {
  pub fn setRight<RetType, T: QRect_setRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRight(self);
    // return 1;
  }
}

pub trait QRect_setRight<RetType> {
  fn setRight(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setRight(int pos);
impl<'a> /*trait*/ QRect_setRight<()> for (i32) {
  fn setRight(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8setRightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect8setRightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::setTopLeft(const QPoint & p);
impl /*struct*/ QRect {
  pub fn setTopLeft<RetType, T: QRect_setTopLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopLeft(self);
    // return 1;
  }
}

pub trait QRect_setTopLeft<RetType> {
  fn setTopLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::setTopLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_setTopLeft<()> for (QPoint) {
  fn setTopLeft(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10setTopLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect10setTopLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRect::moveLeft(int pos);
impl /*struct*/ QRect {
  pub fn moveLeft<RetType, T: QRect_moveLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveLeft(self);
    // return 1;
  }
}

pub trait QRect_moveLeft<RetType> {
  fn moveLeft(self , rsthis: &mut QRect) -> RetType;
}

  // proto:  void QRect::moveLeft(int pos);
impl<'a> /*trait*/ QRect_moveLeft<()> for (i32) {
  fn moveLeft(self , rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8moveLeftEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect8moveLeftEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QRect::translated(const QPoint & p);
impl<'a> /*trait*/ QRect_translated<QRect> for (QPoint) {
  fn translated(self , rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn inheritFrom(qthis: *mut c_void) -> QRectF {
    return QRectF{qclsinst: qthis};
  }
}
  // proto:  void QRectF::QRectF();
impl /*struct*/ QRectF {
  pub fn NewQRectF<T: QRectF_NewQRectF>(value: T) -> QRectF {
    let rsthis = value.NewQRectF();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_NewQRectF {
  fn NewQRectF(self) -> QRectF;
}

  // proto:  void QRectF::QRectF();
impl<'a> /*trait*/ QRectF_NewQRectF for () {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1Ev()};
    unsafe {_ZN6QRectFC1Ev(qthis)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRectF::moveBottomRight(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn moveBottomRight<RetType, T: QRectF_moveBottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBottomRight(self);
    // return 1;
  }
}

pub trait QRectF_moveBottomRight<RetType> {
  fn moveBottomRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomRight<()> for (QPointF) {
  fn moveBottomRight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF15moveBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF15moveBottomRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::moveTo(qreal x, qreal y);
impl /*struct*/ QRectF {
  pub fn moveTo<RetType, T: QRectF_moveTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTo(self);
    // return 1;
  }
}

pub trait QRectF_moveTo<RetType> {
  fn moveTo(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_moveTo<()> for (f64, f64) {
  fn moveTo(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QRectF6moveToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QRectF::top();
impl /*struct*/ QRectF {
  pub fn top<RetType, T: QRectF_top<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.top(self);
    // return 1;
  }
}

pub trait QRectF_top<RetType> {
  fn top(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::top();
impl<'a> /*trait*/ QRectF_top<f64> for () {
  fn top(self , rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF3topEv()};
    let mut ret = unsafe {_ZNK6QRectF3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QPointF QRectF::bottomLeft();
impl /*struct*/ QRectF {
  pub fn bottomLeft<RetType, T: QRectF_bottomLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_bottomLeft<RetType> {
  fn bottomLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QPointF QRectF::bottomLeft();
impl<'a> /*trait*/ QRectF_bottomLeft<QPointF> for () {
  fn bottomLeft(self , rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10bottomLeftEv()};
    let mut ret = unsafe {_ZNK6QRectF10bottomLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::setHeight(qreal h);
impl /*struct*/ QRectF {
  pub fn setHeight<RetType, T: QRectF_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QRectF_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setHeight(qreal h);
impl<'a> /*trait*/ QRectF_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::setSize(const QSizeF & s);
impl /*struct*/ QRectF {
  pub fn setSize<RetType, T: QRectF_setSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSize(self);
    // return 1;
  }
}

pub trait QRectF_setSize<RetType> {
  fn setSize(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setSize(const QSizeF & s);
impl<'a> /*trait*/ QRectF_setSize<()> for (QSizeF) {
  fn setSize(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF7setSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::QRectF(const QPointF & topleft, const QPointF & bottomRight);
impl<'a> /*trait*/ QRectF_NewQRectF for (QPointF, QPointF) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QRectFC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRectF::moveTo(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTo<()> for (QPointF) {
  fn moveTo(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF6moveToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QRectF::toAlignedRect();
impl /*struct*/ QRectF {
  pub fn toAlignedRect<RetType, T: QRectF_toAlignedRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toAlignedRect(self);
    // return 1;
  }
}

pub trait QRectF_toAlignedRect<RetType> {
  fn toAlignedRect(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRect QRectF::toAlignedRect();
impl<'a> /*trait*/ QRectF_toAlignedRect<QRect> for () {
  fn toAlignedRect(self , rsthis: &mut QRectF) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF13toAlignedRectEv()};
    let mut ret = unsafe {_ZNK6QRectF13toAlignedRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::setRight(qreal pos);
impl /*struct*/ QRectF {
  pub fn setRight<RetType, T: QRectF_setRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRight(self);
    // return 1;
  }
}

pub trait QRectF_setRight<RetType> {
  fn setRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setRight(qreal pos);
impl<'a> /*trait*/ QRectF_setRight<()> for (f64) {
  fn setRight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8setRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::setBottomLeft(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn setBottomLeft<RetType, T: QRectF_setBottomLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_setBottomLeft<RetType> {
  fn setBottomLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomLeft<()> for (QPointF) {
  fn setBottomLeft(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF13setBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF13setBottomLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QRectF::topRight();
impl /*struct*/ QRectF {
  pub fn topRight<RetType, T: QRectF_topRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topRight(self);
    // return 1;
  }
}

pub trait QRectF_topRight<RetType> {
  fn topRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QPointF QRectF::topRight();
impl<'a> /*trait*/ QRectF_topRight<QPointF> for () {
  fn topRight(self , rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8topRightEv()};
    let mut ret = unsafe {_ZNK6QRectF8topRightEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizeF QRectF::size();
impl /*struct*/ QRectF {
  pub fn size<RetType, T: QRectF_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QRectF_size<RetType> {
  fn size(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QSizeF QRectF::size();
impl<'a> /*trait*/ QRectF_size<QSizeF> for () {
  fn size(self , rsthis: &mut QRectF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4sizeEv()};
    let mut ret = unsafe {_ZNK6QRectF4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
impl /*struct*/ QRectF {
  pub fn adjust<RetType, T: QRectF_adjust<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.adjust(self);
    // return 1;
  }
}

pub trait QRectF_adjust<RetType> {
  fn adjust(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjust<()> for (f64, f64, f64, f64) {
  fn adjust(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6adjustEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QRectF6adjustEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QRectF::moveRight(qreal pos);
impl /*struct*/ QRectF {
  pub fn moveRight<RetType, T: QRectF_moveRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveRight(self);
    // return 1;
  }
}

pub trait QRectF_moveRight<RetType> {
  fn moveRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveRight(qreal pos);
impl<'a> /*trait*/ QRectF_moveRight<()> for (f64) {
  fn moveRight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9moveRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9moveRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QRectF::y();
impl /*struct*/ QRectF {
  pub fn y<RetType, T: QRectF_y<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QRectF_y<RetType> {
  fn y(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::y();
impl<'a> /*trait*/ QRectF_y<()> for () {
  fn y(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1yEv()};
     unsafe {_ZNK6QRectF1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QRectF::bottomRight();
impl /*struct*/ QRectF {
  pub fn bottomRight<RetType, T: QRectF_bottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottomRight(self);
    // return 1;
  }
}

pub trait QRectF_bottomRight<RetType> {
  fn bottomRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QPointF QRectF::bottomRight();
impl<'a> /*trait*/ QRectF_bottomRight<QPointF> for () {
  fn bottomRight(self , rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF11bottomRightEv()};
    let mut ret = unsafe {_ZNK6QRectF11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::setBottom(qreal pos);
impl /*struct*/ QRectF {
  pub fn setBottom<RetType, T: QRectF_setBottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottom(self);
    // return 1;
  }
}

pub trait QRectF_setBottom<RetType> {
  fn setBottom(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setBottom(qreal pos);
impl<'a> /*trait*/ QRectF_setBottom<()> for (f64) {
  fn setBottom(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::moveBottomLeft(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn moveBottomLeft<RetType, T: QRectF_moveBottomLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveBottomLeft<RetType> {
  fn moveBottomLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomLeft<()> for (QPointF) {
  fn moveBottomLeft(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14moveBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF14moveBottomLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::moveBottom(qreal pos);
impl /*struct*/ QRectF {
  pub fn moveBottom<RetType, T: QRectF_moveBottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveBottom(self);
    // return 1;
  }
}

pub trait QRectF_moveBottom<RetType> {
  fn moveBottom(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveBottom(qreal pos);
impl<'a> /*trait*/ QRectF_moveBottom<()> for (f64) {
  fn moveBottom(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF10moveBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
impl /*struct*/ QRectF {
  pub fn getRect<RetType, T: QRectF_getRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getRect(self);
    // return 1;
  }
}

pub trait QRectF_getRect<RetType> {
  fn getRect(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
impl<'a> /*trait*/ QRectF_getRect<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getRect(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7getRectEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK6QRectF7getRectEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  qreal QRectF::x();
impl /*struct*/ QRectF {
  pub fn x<RetType, T: QRectF_x<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QRectF_x<RetType> {
  fn x(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::x();
impl<'a> /*trait*/ QRectF_x<()> for () {
  fn x(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1xEv()};
     unsafe {_ZNK6QRectF1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QRectF::bottom();
impl /*struct*/ QRectF {
  pub fn bottom<RetType, T: QRectF_bottom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bottom(self);
    // return 1;
  }
}

pub trait QRectF_bottom<RetType> {
  fn bottom(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::bottom();
impl<'a> /*trait*/ QRectF_bottom<f64> for () {
  fn bottom(self , rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6bottomEv()};
    let mut ret = unsafe {_ZNK6QRectF6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QRectF::isNull();
impl /*struct*/ QRectF {
  pub fn isNull<RetType, T: QRectF_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QRectF_isNull<RetType> {
  fn isNull(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  bool QRectF::isNull();
impl<'a> /*trait*/ QRectF_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6isNullEv()};
    let mut ret = unsafe {_ZNK6QRectF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRectF::QRectF(const QPointF & topleft, const QSizeF & size);
impl<'a> /*trait*/ QRectF_NewQRectF for (QPointF, QSizeF) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK7QPointFRK6QSizeF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QRectFC1ERK7QPointFRK6QSizeF(qthis, arg0, arg1)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRectF::setWidth(qreal w);
impl /*struct*/ QRectF {
  pub fn setWidth<RetType, T: QRectF_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QRectF_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setWidth(qreal w);
impl<'a> /*trait*/ QRectF_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QRectF::height();
impl /*struct*/ QRectF {
  pub fn height<RetType, T: QRectF_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QRectF_height<RetType> {
  fn height(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::height();
impl<'a> /*trait*/ QRectF_height<f64> for () {
  fn height(self , rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6heightEv()};
    let mut ret = unsafe {_ZNK6QRectF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRectF::translate(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn translate<RetType, T: QRectF_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QRectF_translate<RetType> {
  fn translate(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::translate(const QPointF & p);
impl<'a> /*trait*/ QRectF_translate<()> for (QPointF) {
  fn translate(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::moveCenter(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn moveCenter<RetType, T: QRectF_moveCenter<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveCenter(self);
    // return 1;
  }
}

pub trait QRectF_moveCenter<RetType> {
  fn moveCenter(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveCenter(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveCenter<()> for (QPointF) {
  fn moveCenter(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF10moveCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRectF::contains(const QRectF & r);
impl /*struct*/ QRectF {
  pub fn contains<RetType, T: QRectF_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QRectF_contains<RetType> {
  fn contains(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  bool QRectF::contains(const QRectF & r);
impl<'a> /*trait*/ QRectF_contains<i8> for (QRectF) {
  fn contains(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF8containsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QRectF::marginsRemoved(const QMarginsF & margins);
impl /*struct*/ QRectF {
  pub fn marginsRemoved<RetType, T: QRectF_marginsRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.marginsRemoved(self);
    // return 1;
  }
}

pub trait QRectF_marginsRemoved<RetType> {
  fn marginsRemoved(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::marginsRemoved(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsRemoved<QRectF> for (QMarginsF) {
  fn marginsRemoved(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF14marginsRemovedERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF14marginsRemovedERK9QMarginsF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRectF::contains(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_contains<i8> for (f64, f64) {
  fn contains(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK6QRectF8containsEdd(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRectF::setX(qreal pos);
impl /*struct*/ QRectF {
  pub fn setX<RetType, T: QRectF_setX<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QRectF_setX<RetType> {
  fn setX(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setX(qreal pos);
impl<'a> /*trait*/ QRectF_setX<()> for (f64) {
  fn setX(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QRectF {
  pub fn setRect<RetType, T: QRectF_setRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRect(self);
    // return 1;
  }
}

pub trait QRectF_setRect<RetType> {
  fn setRect(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QRectF_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QRectF7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QPointF QRectF::center();
impl /*struct*/ QRectF {
  pub fn center<RetType, T: QRectF_center<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.center(self);
    // return 1;
  }
}

pub trait QRectF_center<RetType> {
  fn center(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QPointF QRectF::center();
impl<'a> /*trait*/ QRectF_center<QPointF> for () {
  fn center(self , rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6centerEv()};
    let mut ret = unsafe {_ZNK6QRectF6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::setLeft(qreal pos);
impl /*struct*/ QRectF {
  pub fn setLeft<RetType, T: QRectF_setLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLeft(self);
    // return 1;
  }
}

pub trait QRectF_setLeft<RetType> {
  fn setLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setLeft(qreal pos);
impl<'a> /*trait*/ QRectF_setLeft<()> for (f64) {
  fn setLeft(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF7setLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QRectF::intersected(const QRectF & other);
impl /*struct*/ QRectF {
  pub fn intersected<RetType, T: QRectF_intersected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QRectF_intersected<RetType> {
  fn intersected(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::intersected(const QRectF & other);
impl<'a> /*trait*/ QRectF_intersected<QRectF> for (QRectF) {
  fn intersected(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QRectF::topLeft();
impl /*struct*/ QRectF {
  pub fn topLeft<RetType, T: QRectF_topLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.topLeft(self);
    // return 1;
  }
}

pub trait QRectF_topLeft<RetType> {
  fn topLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QPointF QRectF::topLeft();
impl<'a> /*trait*/ QRectF_topLeft<QPointF> for () {
  fn topLeft(self , rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7topLeftEv()};
    let mut ret = unsafe {_ZNK6QRectF7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRectF::left();
impl /*struct*/ QRectF {
  pub fn left<RetType, T: QRectF_left<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.left(self);
    // return 1;
  }
}

pub trait QRectF_left<RetType> {
  fn left(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::left();
impl<'a> /*trait*/ QRectF_left<f64> for () {
  fn left(self , rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4leftEv()};
    let mut ret = unsafe {_ZNK6QRectF4leftEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRectF::setY(qreal pos);
impl /*struct*/ QRectF {
  pub fn setY<RetType, T: QRectF_setY<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QRectF_setY<RetType> {
  fn setY(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setY(qreal pos);
impl<'a> /*trait*/ QRectF_setY<()> for (f64) {
  fn setY(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::moveTopLeft(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn moveTopLeft<RetType, T: QRectF_moveTopLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTopLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveTopLeft<RetType> {
  fn moveTopLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopLeft<()> for (QPointF) {
  fn moveTopLeft(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11moveTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF11moveTopLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QRectF::width();
impl /*struct*/ QRectF {
  pub fn width<RetType, T: QRectF_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QRectF_width<RetType> {
  fn width(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::width();
impl<'a> /*trait*/ QRectF_width<f64> for () {
  fn width(self , rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5widthEv()};
    let mut ret = unsafe {_ZNK6QRectF5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRectF::setTop(qreal pos);
impl /*struct*/ QRectF {
  pub fn setTop<RetType, T: QRectF_setTop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTop(self);
    // return 1;
  }
}

pub trait QRectF_setTop<RetType> {
  fn setTop(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setTop(qreal pos);
impl<'a> /*trait*/ QRectF_setTop<()> for (f64) {
  fn setTop(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRectF::isValid();
impl /*struct*/ QRectF {
  pub fn isValid<RetType, T: QRectF_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRectF_isValid<RetType> {
  fn isValid(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  bool QRectF::isValid();
impl<'a> /*trait*/ QRectF_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isValidEv()};
    let mut ret = unsafe {_ZNK6QRectF7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRectF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translate<()> for (f64, f64) {
  fn translate(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QRectF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QRectF::QRectF(qreal left, qreal top, qreal width, qreal height);
impl<'a> /*trait*/ QRectF_NewQRectF for (f64, f64, f64, f64) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QRectFC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRect QRectF::toRect();
impl /*struct*/ QRectF {
  pub fn toRect<RetType, T: QRectF_toRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toRect(self);
    // return 1;
  }
}

pub trait QRectF_toRect<RetType> {
  fn toRect(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRect QRectF::toRect();
impl<'a> /*trait*/ QRectF_toRect<QRect> for () {
  fn toRect(self , rsthis: &mut QRectF) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6toRectEv()};
    let mut ret = unsafe {_ZNK6QRectF6toRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::moveLeft(qreal pos);
impl /*struct*/ QRectF {
  pub fn moveLeft<RetType, T: QRectF_moveLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveLeft<RetType> {
  fn moveLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveLeft(qreal pos);
impl<'a> /*trait*/ QRectF_moveLeft<()> for (f64) {
  fn moveLeft(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8moveLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8moveLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::setTopLeft(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn setTopLeft<RetType, T: QRectF_setTopLeft<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopLeft(self);
    // return 1;
  }
}

pub trait QRectF_setTopLeft<RetType> {
  fn setTopLeft(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopLeft<()> for (QPointF) {
  fn setTopLeft(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10setTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF10setTopLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::setBottomRight(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn setBottomRight<RetType, T: QRectF_setBottomRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBottomRight(self);
    // return 1;
  }
}

pub trait QRectF_setBottomRight<RetType> {
  fn setBottomRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomRight<()> for (QPointF) {
  fn setBottomRight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14setBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF14setBottomRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QRectF::marginsAdded(const QMarginsF & margins);
impl /*struct*/ QRectF {
  pub fn marginsAdded<RetType, T: QRectF_marginsAdded<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.marginsAdded(self);
    // return 1;
  }
}

pub trait QRectF_marginsAdded<RetType> {
  fn marginsAdded(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::marginsAdded(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsAdded<QRectF> for (QMarginsF) {
  fn marginsAdded(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF12marginsAddedERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF12marginsAddedERK9QMarginsF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QRectF::translated(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn translated<RetType, T: QRectF_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QRectF_translated<RetType> {
  fn translated(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::translated(const QPointF & p);
impl<'a> /*trait*/ QRectF_translated<QRectF> for (QPointF) {
  fn translated(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QRectF::normalized();
impl /*struct*/ QRectF {
  pub fn normalized<RetType, T: QRectF_normalized<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.normalized(self);
    // return 1;
  }
}

pub trait QRectF_normalized<RetType> {
  fn normalized(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::normalized();
impl<'a> /*trait*/ QRectF_normalized<QRectF> for () {
  fn normalized(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10normalizedEv()};
    let mut ret = unsafe {_ZNK6QRectF10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
impl /*struct*/ QRectF {
  pub fn getCoords<RetType, T: QRectF_getCoords<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getCoords(self);
    // return 1;
  }
}

pub trait QRectF_getCoords<RetType> {
  fn getCoords(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
impl<'a> /*trait*/ QRectF_getCoords<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getCoords(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF9getCoordsEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK6QRectF9getCoordsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QRectF::setTopRight(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn setTopRight<RetType, T: QRectF_setTopRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTopRight(self);
    // return 1;
  }
}

pub trait QRectF_setTopRight<RetType> {
  fn setTopRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopRight<()> for (QPointF) {
  fn setTopRight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11setTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF11setTopRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRectF::contains(const QPointF & p);
impl<'a> /*trait*/ QRectF_contains<i8> for (QPointF) {
  fn contains(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRectF::intersects(const QRectF & r);
impl /*struct*/ QRectF {
  pub fn intersects<RetType, T: QRectF_intersects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QRectF_intersects<RetType> {
  fn intersects(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  bool QRectF::intersects(const QRectF & r);
impl<'a> /*trait*/ QRectF_intersects<i8> for (QRectF) {
  fn intersects(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRectF::moveTop(qreal pos);
impl /*struct*/ QRectF {
  pub fn moveTop<RetType, T: QRectF_moveTop<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTop(self);
    // return 1;
  }
}

pub trait QRectF_moveTop<RetType> {
  fn moveTop(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveTop(qreal pos);
impl<'a> /*trait*/ QRectF_moveTop<()> for (f64) {
  fn moveTop(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7moveTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF7moveTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
impl /*struct*/ QRectF {
  pub fn setCoords<RetType, T: QRectF_setCoords<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCoords(self);
    // return 1;
  }
}

pub trait QRectF_setCoords<RetType> {
  fn setCoords(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_setCoords<()> for (f64, f64, f64, f64) {
  fn setCoords(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setCoordsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QRectF9setCoordsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QRectF QRectF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translated<QRectF> for (f64, f64) {
  fn translated(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK6QRectF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRectF::isEmpty();
impl /*struct*/ QRectF {
  pub fn isEmpty<RetType, T: QRectF_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QRectF_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  bool QRectF::isEmpty();
impl<'a> /*trait*/ QRectF_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isEmptyEv()};
    let mut ret = unsafe {_ZNK6QRectF7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRectF::moveTopRight(const QPointF & p);
impl /*struct*/ QRectF {
  pub fn moveTopRight<RetType, T: QRectF_moveTopRight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTopRight(self);
    // return 1;
  }
}

pub trait QRectF_moveTopRight<RetType> {
  fn moveTopRight(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  void QRectF::moveTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopRight<()> for (QPointF) {
  fn moveTopRight(self , rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF12moveTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF12moveTopRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QRectF::united(const QRectF & other);
impl /*struct*/ QRectF {
  pub fn united<RetType, T: QRectF_united<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QRectF_united<RetType> {
  fn united(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::united(const QRectF & other);
impl<'a> /*trait*/ QRectF_united<QRectF> for (QRectF) {
  fn united(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QRectF::right();
impl /*struct*/ QRectF {
  pub fn right<RetType, T: QRectF_right<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.right(self);
    // return 1;
  }
}

pub trait QRectF_right<RetType> {
  fn right(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  qreal QRectF::right();
impl<'a> /*trait*/ QRectF_right<f64> for () {
  fn right(self , rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5rightEv()};
    let mut ret = unsafe {_ZNK6QRectF5rightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QRectF::QRectF(const QRect & rect);
impl<'a> /*trait*/ QRectF_NewQRectF for (QRect) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QRectFC1ERK5QRect(qthis, arg0)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
impl /*struct*/ QRectF {
  pub fn adjusted<RetType, T: QRectF_adjusted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.adjusted(self);
    // return 1;
  }
}

pub trait QRectF_adjusted<RetType> {
  fn adjusted(self , rsthis: &mut QRectF) -> RetType;
}

  // proto:  QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjusted<QRectF> for (f64, f64, f64, f64) {
  fn adjusted(self , rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8adjustedEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK6QRectF8adjustedEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

