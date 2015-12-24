// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtGui/qpainter.h
// dst-file: /src/gui/qpainter.rs
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
use super::super::core::qrect::QRectF; // 771
use super::super::core::qstring::QString; // 771
use super::qtextoption::QTextOption; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qpoint::QPointF; // 771
use super::qpicture::QPicture; // 773
use super::qmatrix::QMatrix; // 773
use super::qcolor::QColor; // 773
use super::qpixmap::QPixmap; // 773
use super::qbrush::QBrush; // 773
use super::qimage::QImage; // 773
use super::super::core::qpoint::QPoint; // 771
use super::qpen::QPen; // 773
use super::super::core::qline::QLineF; // 771
use super::qtransform::QTransform; // 773
use super::qpolygon::QPolygonF; // 773
use super::qstatictext::QStaticText; // 773
use super::qpainterpath::QPainterPath; // 773
use super::qpolygon::QPolygon; // 773
use super::qfont::QFont; // 773
use super::super::core::qline::QLine; // 771
use super::qregion::QRegion; // 773
use super::qpaintdevice::QPaintDevice; // 773
use super::qpaintengine::QTextItem; // 773
use super::qpaintengine::QPaintEngine; // 773
use super::qfontmetrics::QFontMetrics; // 773
use super::qglyphrun::QGlyphRun; // 773
use super::qfontinfo::QFontInfo; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  QRectF QPainter::boundingRect(const QRectF & rect, const QString & text, const QTextOption & o);
  fn _ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
  fn _ZN8QPainter11drawPictureERK7QPointFRK8QPicture(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QMatrix & QPainter::worldMatrix();
  fn _ZNK8QPainter11worldMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
  fn _ZN8QPainter8drawTextERK7QPointFRK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
  fn _ZN8QPainter8fillRectEiiiiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void);
  // proto:  const QMatrix & QPainter::matrix();
  fn _ZNK8QPainter6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QPainter::opacity();
  fn _ZNK8QPainter7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainter::drawText(int x, int y, const QString & s);
  fn _ZN8QPainter8drawTextEiiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
  fn _ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainter::setBackground(const QBrush & bg);
  fn _ZN8QPainter13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QPainter::boundingRect(const QRect & rect, int flags, const QString & text);
  fn _ZN8QPainter12boundingRectERK5QRectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawChord(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter9drawChordERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::drawImage(const QRectF & r, const QImage & image);
  fn _ZN8QPainter9drawImageERK6QRectFRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::setClipping(bool enable);
  fn _ZN8QPainter11setClippingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPainter::setBrush(const QBrush & brush);
  fn _ZN8QPainter8setBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN8QPainter9setMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QPainter::drawChord(const QRect & , int a, int alen);
  fn _ZN8QPainter9drawChordERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::eraseRect(const QRectF & );
  fn _ZN8QPainter9eraseRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::translate(const QPoint & offset);
  fn _ZN8QPainter9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPainter::viewTransformEnabled();
  fn _ZNK8QPainter20viewTransformEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainter::setPen(const QPen & pen);
  fn _ZN8QPainter6setPenERK4QPen(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawLines(const QLineF * lines, int lineCount);
  fn _ZN8QPainter9drawLinesEPK6QLineFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::setBrushOrigin(int x, int y);
  fn _ZN8QPainter14setBrushOriginEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  const QTransform & QPainter::worldTransform();
  fn _ZNK8QPainter14worldTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawRects(const QRect * rects, int rectCount);
  fn _ZN8QPainter9drawRectsEPK5QRecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
  fn _ZN8QPainter11drawEllipseERK6QPointii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter7drawArcEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
  // proto:  void QPainter::drawPolyline(const QPolygonF & polyline);
  fn _ZN8QPainter12drawPolylineERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPainter::hasClipping();
  fn _ZNK8QPainter11hasClippingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
  fn _ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextEiiRK11QStaticText(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
  fn _ZN8QPainter10strokePathERK12QPainterPathRK4QPen(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm, int sx, int sy, int sw, int sh);
  fn _ZN8QPainter10drawPixmapEiiRK7QPixmapiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int);
  // proto:  void QPainter::drawRects(const QRectF * rects, int rectCount);
  fn _ZN8QPainter9drawRectsEPK6QRectFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
  fn _ZN8QPainter17drawConvexPolygonEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::drawPath(const QPainterPath & path);
  fn _ZN8QPainter8drawPathERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapEiiRK7QPixmap(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  QMatrix QPainter::combinedMatrix();
  fn _ZNK8QPainter14combinedMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::setMatrixEnabled(bool enabled);
  fn _ZN8QPainter16setMatrixEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPainter::drawPolyline(const QPolygon & polygon);
  fn _ZN8QPainter12drawPolylineERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawTiledPixmap(const QRect & , const QPixmap & , const QPoint & );
  fn _ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainter::setFont(const QFont & f);
  fn _ZN8QPainter7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawChord(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter9drawChordEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
  // proto:  void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapEiiiiRK7QPixmap(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void);
  // proto:  void QPainter::setWindow(const QRect & window);
  fn _ZN8QPainter9setWindowERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMatrix & QPainter::deviceMatrix();
  fn _ZNK8QPainter12deviceMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawLines(const QPointF * pointPairs, int lineCount);
  fn _ZN8QPainter9drawLinesEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QRect QPainter::boundingRect(int x, int y, int w, int h, int flags, const QString & text);
  fn _ZN8QPainter12boundingRectEiiiiiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawLines(const QLine * lines, int lineCount);
  fn _ZN8QPainter9drawLinesEPK5QLinei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter7drawPieEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
  // proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm, const QRect & sr);
  fn _ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainter::drawStaticText(const QPointF & topLeftPosition, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::setWorldMatrixEnabled(bool enabled);
  fn _ZN8QPainter21setWorldMatrixEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPainter::QPainter(const QPainter & );
  fn _ZN8QPainterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawPoints(const QPolygon & points);
  fn _ZN8QPainter10drawPointsERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawPicture(const QPoint & p, const QPicture & picture);
  fn _ZN8QPainter11drawPictureERK6QPointRK8QPicture(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawRect(int x1, int y1, int w, int h);
  fn _ZN8QPainter8drawRectEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::drawEllipse(const QRectF & r);
  fn _ZN8QPainter11drawEllipseERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawRect(const QRectF & rect);
  fn _ZN8QPainter8drawRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawPoints(const QPointF * points, int pointCount);
  fn _ZN8QPainter10drawPointsEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QRegion QPainter::clipRegion();
  fn _ZNK8QPainter10clipRegionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawText(const QRectF & r, int flags, const QString & text, QRectF * br);
  fn _ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QPainter::drawLine(const QLineF & line);
  fn _ZN8QPainter8drawLineERK6QLineF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawLine(const QPointF & p1, const QPointF & p2);
  fn _ZN8QPainter8drawLineERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawPixmap(const QRect & r, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK5QRectRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap & , int sx, int sy);
  fn _ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void, arg5: c_int, arg6: c_int);
  // proto:  QPaintDevice * QPainter::device();
  fn _ZNK8QPainter6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::setViewport(const QRect & viewport);
  fn _ZN8QPainter11setViewportERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::fillRect(const QRect & , const QColor & color);
  fn _ZN8QPainter8fillRectERK5QRectRK6QColor(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::setBrushOrigin(const QPointF & );
  fn _ZN8QPainter14setBrushOriginERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemEiiRK9QTextItem(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QPainter::QPainter(QPaintDevice * );
  fn _ZN8QPainterC1EP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm, int sx, int sy, int sw, int sh);
  fn _ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int);
  // proto:  void QPainter::drawImage(const QPoint & p, const QImage & image);
  fn _ZN8QPainter9drawImageERK6QPointRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawPie(const QRect & , int a, int alen);
  fn _ZN8QPainter7drawPieERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::drawTextItem(const QPoint & p, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemERK6QPointRK9QTextItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawLines(const QPoint * pointPairs, int lineCount);
  fn _ZN8QPainter9drawLinesEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::drawPicture(int x, int y, const QPicture & picture);
  fn _ZN8QPainter11drawPictureEiiRK8QPicture(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void);
  // proto:  void QPainter::save();
  fn _ZN8QPainter4saveEv(qthis: *mut c_void);
  // proto:  void QPainter::translate(qreal dx, qreal dy);
  fn _ZN8QPainter9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QTransform QPainter::combinedTransform();
  fn _ZNK8QPainter17combinedTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPainter::end();
  fn _ZN8QPainter3endEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainter::setViewport(int x, int y, int w, int h);
  fn _ZN8QPainter11setViewportEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
  fn _ZN8QPainter13drawRoundRectERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
  fn _ZN8QPainter17setWorldTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QPainter::drawPoints(const QPolygonF & points);
  fn _ZN8QPainter10drawPointsERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::restore();
  fn _ZN8QPainter7restoreEv(qthis: *mut c_void);
  // proto:  void QPainter::drawStaticText(const QPoint & topLeftPosition, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QRectF QPainter::boundingRect(const QRectF & rect, int flags, const QString & text);
  fn _ZN8QPainter12boundingRectERK6QRectFiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QBrush & );
  fn _ZN8QPainter8fillRectEiiiiRK6QBrush(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void);
  // proto:  void QPainter::drawRoundRect(const QRectF & r, int xround, int yround);
  fn _ZN8QPainter13drawRoundRectERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::drawPoint(const QPoint & p);
  fn _ZN8QPainter9drawPointERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
  fn _ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::shear(qreal sh, qreal sv);
  fn _ZN8QPainter5shearEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPainter::drawText(const QRect & r, int flags, const QString & text, QRect * br);
  fn _ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  const QFont & QPainter::font();
  fn _ZNK8QPainter4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QTransform & QPainter::deviceTransform();
  fn _ZNK8QPainter15deviceTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::eraseRect(int x, int y, int w, int h);
  fn _ZN8QPainter9eraseRectEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::resetMatrix();
  fn _ZN8QPainter11resetMatrixEv(qthis: *mut c_void);
  // proto:  void QPainter::drawPolyline(const QPoint * points, int pointCount);
  fn _ZN8QPainter12drawPolylineEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QPaintEngine * QPainter::paintEngine();
  fn _ZNK8QPainter11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawEllipse(const QRect & r);
  fn _ZN8QPainter11drawEllipseERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawLine(const QLine & line);
  fn _ZN8QPainter8drawLineERK5QLine(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPainter::isActive();
  fn _ZNK8QPainter8isActiveEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainter::drawArc(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter7drawArcERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto: static void QPainter::restoreRedirected(const QPaintDevice * device);
  fn _ZN8QPainter17restoreRedirectedEPK12QPaintDevice(arg0: *mut c_void);
  // proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm, const QRectF & sr);
  fn _ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainter::drawEllipse(const QPointF & center, qreal rx, qreal ry);
  fn _ZN8QPainter11drawEllipseERK7QPointFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double);
  // proto:  void QPainter::drawConvexPolygon(const QPointF * points, int pointCount);
  fn _ZN8QPainter17drawConvexPolygonEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::setBrushOrigin(const QPoint & );
  fn _ZN8QPainter14setBrushOriginERK6QPoint(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawText(const QRectF & r, const QString & text, const QTextOption & o);
  fn _ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QPainter::worldMatrixEnabled();
  fn _ZNK8QPainter18worldMatrixEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK6QPointRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawLine(int x1, int y1, int x2, int y2);
  fn _ZN8QPainter8drawLineEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::drawPoint(int x, int y);
  fn _ZN8QPainter9drawPointEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  const QTransform & QPainter::transform();
  fn _ZNK8QPainter9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
  fn _ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainter::drawPixmap(const QRect & targetRect, const QPixmap & pixmap, const QRect & sourceRect);
  fn _ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QFontMetrics QPainter::fontMetrics();
  fn _ZNK8QPainter11fontMetricsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
  fn _ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::fillRect(const QRectF & , const QBrush & );
  fn _ZN8QPainter8fillRectERK6QRectFRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::resetTransform();
  fn _ZN8QPainter14resetTransformEv(qthis: *mut c_void);
  // proto:  void QPainter::fillRect(const QRect & , const QBrush & );
  fn _ZN8QPainter8fillRectERK5QRectRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QBrush & QPainter::brush();
  fn _ZNK8QPainter5brushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::~QPainter();
  fn _ZN8QPainterD0Ev(qthis: *mut c_void);
  // proto:  bool QPainter::begin(QPaintDevice * );
  fn _ZN8QPainter5beginEP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainter::drawRect(const QRect & rect);
  fn _ZN8QPainter8drawRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawTextItem(const QPointF & p, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::scale(qreal sx, qreal sy);
  fn _ZN8QPainter5scaleEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
  fn _ZN8QPainter14setWorldMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  QPainterPath QPainter::clipPath();
  fn _ZNK8QPainter8clipPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QPainter::brushOrigin();
  fn _ZNK8QPainter11brushOriginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawConvexPolygon(const QPolygonF & polygon);
  fn _ZN8QPainter17drawConvexPolygonERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawEllipse(int x, int y, int w, int h);
  fn _ZN8QPainter11drawEllipseEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::drawConvexPolygon(const QPolygon & polygon);
  fn _ZN8QPainter17drawConvexPolygonERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawPoints(const QPoint * points, int pointCount);
  fn _ZN8QPainter10drawPointsEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  const QBrush & QPainter::background();
  fn _ZNK8QPainter10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawRoundRect(int x, int y, int w, int h, int , int );
  fn _ZN8QPainter13drawRoundRectEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int);
  // proto:  QRect QPainter::viewport();
  fn _ZNK8QPainter8viewportEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawArc(const QRect & , int a, int alen);
  fn _ZN8QPainter7drawArcERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
  fn _ZN8QPainter8fillPathERK12QPainterPathRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawText(int x, int y, int w, int h, int flags, const QString & text, QRect * br);
  fn _ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *mut c_void, arg6: *mut c_void);
  // proto:  bool QPainter::matrixEnabled();
  fn _ZNK8QPainter13matrixEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainter::drawPolyline(const QPointF * points, int pointCount);
  fn _ZN8QPainter12drawPolylineEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainter::setTransform(const QTransform & transform, bool combine);
  fn _ZN8QPainter12setTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  void QPainter::setPen(const QColor & color);
  fn _ZN8QPainter6setPenERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::eraseRect(const QRect & );
  fn _ZN8QPainter9eraseRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QPainter::window();
  fn _ZNK8QPainter6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawImage(const QRect & r, const QImage & image);
  fn _ZN8QPainter9drawImageERK5QRectRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::initFrom(const QPaintDevice * device);
  fn _ZN8QPainter8initFromEPK12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QFontInfo QPainter::fontInfo();
  fn _ZNK8QPainter8fontInfoEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::endNativePainting();
  fn _ZN8QPainter17endNativePaintingEv(qthis: *mut c_void);
  // proto:  void QPainter::setViewTransformEnabled(bool enable);
  fn _ZN8QPainter23setViewTransformEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QPainter::drawPoint(const QPointF & pt);
  fn _ZN8QPainter9drawPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::setOpacity(qreal opacity);
  fn _ZN8QPainter10setOpacityEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QPainter::fillRect(const QRectF & , const QColor & color);
  fn _ZN8QPainter8fillRectERK6QRectFRK6QColor(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::QPainter();
  fn _ZN8QPainterC1Ev(qthis: *mut c_void);
  // proto:  void QPainter::translate(const QPointF & offset);
  fn _ZN8QPainter9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainter::drawText(const QPointF & p, const QString & s);
  fn _ZN8QPainter8drawTextERK7QPointFRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawImage(const QPointF & p, const QImage & image);
  fn _ZN8QPainter9drawImageERK7QPointFRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  const QPen & QPainter::pen();
  fn _ZNK8QPainter3penEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::rotate(qreal a);
  fn _ZN8QPainter6rotateEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QRectF QPainter::clipBoundingRect();
  fn _ZNK8QPainter16clipBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawLine(const QPoint & p1, const QPoint & p2);
  fn _ZN8QPainter8drawLineERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::drawPie(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter7drawPieERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainter::drawText(const QPoint & p, const QString & s);
  fn _ZN8QPainter8drawTextERK6QPointRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QPainter::setWindow(int x, int y, int w, int h);
  fn _ZN8QPainter9setWindowEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QPainter::beginNativePainting();
  fn _ZN8QPainter19beginNativePaintingEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPainter)=1
pub struct QPainter {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainter {
  pub fn inheritFrom(qthis: *mut c_void) -> QPainter {
    return QPainter{qclsinst: qthis};
  }
}
  // proto:  QRectF QPainter::boundingRect(const QRectF & rect, const QString & text, const QTextOption & o);
impl /*struct*/ QPainter {
  pub fn boundingRect<RetType, T: QPainter_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPainter_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QRectF QPainter::boundingRect(const QRectF & rect, const QString & text, const QTextOption & o);
impl<'a> /*trait*/ QPainter_boundingRect<QRectF> for (&'a QRectF, &'a QString, &'a QTextOption) {
  fn boundingRect(self , rsthis: & QPainter) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
impl /*struct*/ QPainter {
  pub fn drawPicture<RetType, T: QPainter_drawPicture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPicture(self);
    // return 1;
  }
}

pub trait QPainter_drawPicture<RetType> {
  fn drawPicture(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture<()> for (&'a QPointF, &'a QPicture) {
  fn drawPicture(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureERK7QPointFRK8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawPictureERK7QPointFRK8QPicture(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMatrix & QPainter::worldMatrix();
impl /*struct*/ QPainter {
  pub fn worldMatrix<RetType, T: QPainter_worldMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.worldMatrix(self);
    // return 1;
  }
}

pub trait QPainter_worldMatrix<RetType> {
  fn worldMatrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QMatrix & QPainter::worldMatrix();
impl<'a> /*trait*/ QPainter_worldMatrix<QMatrix> for () {
  fn worldMatrix(self , rsthis: & QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11worldMatrixEv()};
    let mut ret = unsafe {_ZNK8QPainter11worldMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
impl /*struct*/ QPainter {
  pub fn drawText<RetType, T: QPainter_drawText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawText(self);
    // return 1;
  }
}

pub trait QPainter_drawText<RetType> {
  fn drawText(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
impl<'a> /*trait*/ QPainter_drawText<()> for (&'a QPointF, &'a QString, i32, i32) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK7QPointFRK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter8drawTextERK7QPointFRK7QStringii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
impl /*struct*/ QPainter {
  pub fn fillRect<RetType, T: QPainter_fillRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fillRect(self);
    // return 1;
  }
}

pub trait QPainter_fillRect<RetType> {
  fn fillRect(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect<()> for (i32, i32, i32, i32, &'a QColor) {
  fn fillRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectEiiiiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectEiiiiRK6QColor(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  const QMatrix & QPainter::matrix();
impl /*struct*/ QPainter {
  pub fn matrix<RetType, T: QPainter_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QPainter_matrix<RetType> {
  fn matrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QMatrix & QPainter::matrix();
impl<'a> /*trait*/ QPainter_matrix<QMatrix> for () {
  fn matrix(self , rsthis: & QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6matrixEv()};
    let mut ret = unsafe {_ZNK8QPainter6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QPainter::opacity();
impl /*struct*/ QPainter {
  pub fn opacity<RetType, T: QPainter_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QPainter_opacity<RetType> {
  fn opacity(self , rsthis: & QPainter) -> RetType;
}

  // proto:  qreal QPainter::opacity();
impl<'a> /*trait*/ QPainter_opacity<f64> for () {
  fn opacity(self , rsthis: & QPainter) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter7opacityEv()};
    let mut ret = unsafe {_ZNK8QPainter7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainter::drawText(int x, int y, const QString & s);
impl<'a> /*trait*/ QPainter_drawText<()> for (i32, i32, &'a QString) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextEiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextEiiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
impl /*struct*/ QPainter {
  pub fn drawTiledPixmap<RetType, T: QPainter_drawTiledPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawTiledPixmap(self);
    // return 1;
  }
}

pub trait QPainter_drawTiledPixmap<RetType> {
  fn drawTiledPixmap(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
impl<'a> /*trait*/ QPainter_drawTiledPixmap<()> for (&'a QRectF, &'a QPixmap, &'a QPointF) {
  fn drawTiledPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::setBackground(const QBrush & bg);
impl /*struct*/ QPainter {
  pub fn setBackground<RetType, T: QPainter_setBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QPainter_setBackground<RetType> {
  fn setBackground(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setBackground(const QBrush & bg);
impl<'a> /*trait*/ QPainter_setBackground<()> for (&'a QBrush) {
  fn setBackground(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QPainter::boundingRect(const QRect & rect, int flags, const QString & text);
impl<'a> /*trait*/ QPainter_boundingRect<QRect> for (&'a QRect, i32, &'a QString) {
  fn boundingRect(self , rsthis: & QPainter) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectERK5QRectiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter12boundingRectERK5QRectiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawChord(const QRectF & rect, int a, int alen);
impl /*struct*/ QPainter {
  pub fn drawChord<RetType, T: QPainter_drawChord<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawChord(self);
    // return 1;
  }
}

pub trait QPainter_drawChord<RetType> {
  fn drawChord(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawChord(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord<()> for (&'a QRectF, i32, i32) {
  fn drawChord(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter9drawChordERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawImage(const QRectF & r, const QImage & image);
impl /*struct*/ QPainter {
  pub fn drawImage<RetType, T: QPainter_drawImage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawImage(self);
    // return 1;
  }
}

pub trait QPainter_drawImage<RetType> {
  fn drawImage(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawImage(const QRectF & r, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage<()> for (&'a QRectF, &'a QImage) {
  fn drawImage(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK6QRectFRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK6QRectFRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setClipping(bool enable);
impl /*struct*/ QPainter {
  pub fn setClipping<RetType, T: QPainter_setClipping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClipping(self);
    // return 1;
  }
}

pub trait QPainter_setClipping<RetType> {
  fn setClipping(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setClipping(bool enable);
impl<'a> /*trait*/ QPainter_setClipping<()> for (i8) {
  fn setClipping(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setClippingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QPainter11setClippingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::setBrush(const QBrush & brush);
impl /*struct*/ QPainter {
  pub fn setBrush<RetType, T: QPainter_setBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBrush(self);
    // return 1;
  }
}

pub trait QPainter_setBrush<RetType> {
  fn setBrush(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setBrush(const QBrush & brush);
impl<'a> /*trait*/ QPainter_setBrush<()> for (&'a QBrush) {
  fn setBrush(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8setBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8setBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::setMatrix(const QMatrix & matrix, bool combine);
impl /*struct*/ QPainter {
  pub fn setMatrix<RetType, T: QPainter_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QPainter_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setMatrix<()> for (&'a QMatrix, i8) {
  fn setMatrix(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN8QPainter9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawChord(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord<()> for (&'a QRect, i32, i32) {
  fn drawChord(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter9drawChordERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::eraseRect(const QRectF & );
impl /*struct*/ QPainter {
  pub fn eraseRect<RetType, T: QPainter_eraseRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.eraseRect(self);
    // return 1;
  }
}

pub trait QPainter_eraseRect<RetType> {
  fn eraseRect(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::eraseRect(const QRectF & );
impl<'a> /*trait*/ QPainter_eraseRect<()> for (&'a QRectF) {
  fn eraseRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9eraseRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::translate(const QPoint & offset);
impl /*struct*/ QPainter {
  pub fn translate<RetType, T: QPainter_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QPainter_translate<RetType> {
  fn translate(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::translate(const QPoint & offset);
impl<'a> /*trait*/ QPainter_translate<()> for (&'a QPoint) {
  fn translate(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPainter::viewTransformEnabled();
impl /*struct*/ QPainter {
  pub fn viewTransformEnabled<RetType, T: QPainter_viewTransformEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewTransformEnabled(self);
    // return 1;
  }
}

pub trait QPainter_viewTransformEnabled<RetType> {
  fn viewTransformEnabled(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::viewTransformEnabled();
impl<'a> /*trait*/ QPainter_viewTransformEnabled<i8> for () {
  fn viewTransformEnabled(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter20viewTransformEnabledEv()};
    let mut ret = unsafe {_ZNK8QPainter20viewTransformEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::setPen(const QPen & pen);
impl /*struct*/ QPainter {
  pub fn setPen<RetType, T: QPainter_setPen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPen(self);
    // return 1;
  }
}

pub trait QPainter_setPen<RetType> {
  fn setPen(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setPen(const QPen & pen);
impl<'a> /*trait*/ QPainter_setPen<()> for (&'a QPen) {
  fn setPen(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter6setPenERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawLines(const QLineF * lines, int lineCount);
impl /*struct*/ QPainter {
  pub fn drawLines<RetType, T: QPainter_drawLines<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawLines(self);
    // return 1;
  }
}

pub trait QPainter_drawLines<RetType> {
  fn drawLines(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawLines(const QLineF * lines, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines<()> for (&'a QLineF, i32) {
  fn drawLines(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK6QLineFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK6QLineFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setBrushOrigin(int x, int y);
impl /*struct*/ QPainter {
  pub fn setBrushOrigin<RetType, T: QPainter_setBrushOrigin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBrushOrigin(self);
    // return 1;
  }
}

pub trait QPainter_setBrushOrigin<RetType> {
  fn setBrushOrigin(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setBrushOrigin(int x, int y);
impl<'a> /*trait*/ QPainter_setBrushOrigin<()> for (i32, i32) {
  fn setBrushOrigin(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter14setBrushOriginEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QTransform & QPainter::worldTransform();
impl /*struct*/ QPainter {
  pub fn worldTransform<RetType, T: QPainter_worldTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.worldTransform(self);
    // return 1;
  }
}

pub trait QPainter_worldTransform<RetType> {
  fn worldTransform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QTransform & QPainter::worldTransform();
impl<'a> /*trait*/ QPainter_worldTransform<QTransform> for () {
  fn worldTransform(self , rsthis: & QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter14worldTransformEv()};
    let mut ret = unsafe {_ZNK8QPainter14worldTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawRects(const QRect * rects, int rectCount);
impl /*struct*/ QPainter {
  pub fn drawRects<RetType, T: QPainter_drawRects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawRects(self);
    // return 1;
  }
}

pub trait QPainter_drawRects<RetType> {
  fn drawRects(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawRects(const QRect * rects, int rectCount);
impl<'a> /*trait*/ QPainter_drawRects<()> for (&'a QRect, i32) {
  fn drawRects(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawRectsEPK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
impl /*struct*/ QPainter {
  pub fn drawEllipse<RetType, T: QPainter_drawEllipse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawEllipse(self);
    // return 1;
  }
}

pub trait QPainter_drawEllipse<RetType> {
  fn drawEllipse(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
impl<'a> /*trait*/ QPainter_drawEllipse<()> for (&'a QPoint, i32, i32) {
  fn drawEllipse(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK6QPointii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter11drawEllipseERK6QPointii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
impl /*struct*/ QPainter {
  pub fn drawArc<RetType, T: QPainter_drawArc<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawArc(self);
    // return 1;
  }
}

pub trait QPainter_drawArc<RetType> {
  fn drawArc(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc<()> for (i32, i32, i32, i32, i32, i32) {
  fn drawArc(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN8QPainter7drawArcEiiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPolyline(const QPolygonF & polyline);
impl /*struct*/ QPainter {
  pub fn drawPolyline<RetType, T: QPainter_drawPolyline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPolyline(self);
    // return 1;
  }
}

pub trait QPainter_drawPolyline<RetType> {
  fn drawPolyline(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPolyline(const QPolygonF & polyline);
impl<'a> /*trait*/ QPainter_drawPolyline<()> for (&'a QPolygonF) {
  fn drawPolyline(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawPolylineERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPainter::hasClipping();
impl /*struct*/ QPainter {
  pub fn hasClipping<RetType, T: QPainter_hasClipping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasClipping(self);
    // return 1;
  }
}

pub trait QPainter_hasClipping<RetType> {
  fn hasClipping(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::hasClipping();
impl<'a> /*trait*/ QPainter_hasClipping<i8> for () {
  fn hasClipping(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11hasClippingEv()};
    let mut ret = unsafe {_ZNK8QPainter11hasClippingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
impl /*struct*/ QPainter {
  pub fn drawPixmap<RetType, T: QPainter_drawPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPixmap(self);
    // return 1;
  }
}

pub trait QPainter_drawPixmap<RetType> {
  fn drawPixmap(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QRectF, &'a QPixmap, &'a QRectF) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
impl /*struct*/ QPainter {
  pub fn drawStaticText<RetType, T: QPainter_drawStaticText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawStaticText(self);
    // return 1;
  }
}

pub trait QPainter_drawStaticText<RetType> {
  fn drawStaticText(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText<()> for (i32, i32, &'a QStaticText) {
  fn drawStaticText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextEiiRK11QStaticText()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14drawStaticTextEiiRK11QStaticText(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
impl /*struct*/ QPainter {
  pub fn strokePath<RetType, T: QPainter_strokePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.strokePath(self);
    // return 1;
  }
}

pub trait QPainter_strokePath<RetType> {
  fn strokePath(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
impl<'a> /*trait*/ QPainter_strokePath<()> for (&'a QPainterPath, &'a QPen) {
  fn strokePath(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10strokePathERK12QPainterPathRK4QPen()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10strokePathERK12QPainterPathRK4QPen(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm, int sx, int sy, int sw, int sh);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (i32, i32, &'a QPixmap, i32, i32, i32, i32) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiRK7QPixmapiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
     unsafe {_ZN8QPainter10drawPixmapEiiRK7QPixmapiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  void QPainter::drawRects(const QRectF * rects, int rectCount);
impl<'a> /*trait*/ QPainter_drawRects<()> for (&'a QRectF, i32) {
  fn drawRects(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawRectsEPK6QRectFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawRectsEPK6QRectFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
impl /*struct*/ QPainter {
  pub fn drawConvexPolygon<RetType, T: QPainter_drawConvexPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawConvexPolygon(self);
    // return 1;
  }
}

pub trait QPainter_drawConvexPolygon<RetType> {
  fn drawConvexPolygon(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawConvexPolygon<()> for (&'a QPoint, i32) {
  fn drawConvexPolygon(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter17drawConvexPolygonEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPath(const QPainterPath & path);
impl /*struct*/ QPainter {
  pub fn drawPath<RetType, T: QPainter_drawPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPath(self);
    // return 1;
  }
}

pub trait QPainter_drawPath<RetType> {
  fn drawPath(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainter_drawPath<()> for (&'a QPainterPath) {
  fn drawPath(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (i32, i32, &'a QPixmap) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiRK7QPixmap()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapEiiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QMatrix QPainter::combinedMatrix();
impl /*struct*/ QPainter {
  pub fn combinedMatrix<RetType, T: QPainter_combinedMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.combinedMatrix(self);
    // return 1;
  }
}

pub trait QPainter_combinedMatrix<RetType> {
  fn combinedMatrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QMatrix QPainter::combinedMatrix();
impl<'a> /*trait*/ QPainter_combinedMatrix<QMatrix> for () {
  fn combinedMatrix(self , rsthis: & QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter14combinedMatrixEv()};
    let mut ret = unsafe {_ZNK8QPainter14combinedMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::setMatrixEnabled(bool enabled);
impl /*struct*/ QPainter {
  pub fn setMatrixEnabled<RetType, T: QPainter_setMatrixEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_setMatrixEnabled<RetType> {
  fn setMatrixEnabled(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setMatrixEnabled(bool enabled);
impl<'a> /*trait*/ QPainter_setMatrixEnabled<()> for (i8) {
  fn setMatrixEnabled(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter16setMatrixEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QPainter16setMatrixEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPolyline(const QPolygon & polygon);
impl<'a> /*trait*/ QPainter_drawPolyline<()> for (&'a QPolygon) {
  fn drawPolyline(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawPolylineERK8QPolygon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawTiledPixmap(const QRect & , const QPixmap & , const QPoint & );
impl<'a> /*trait*/ QPainter_drawTiledPixmap<()> for (&'a QRect, &'a QPixmap, &'a QPoint) {
  fn drawTiledPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::setFont(const QFont & f);
impl /*struct*/ QPainter {
  pub fn setFont<RetType, T: QPainter_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QPainter_setFont<RetType> {
  fn setFont(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setFont(const QFont & f);
impl<'a> /*trait*/ QPainter_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawChord(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord<()> for (i32, i32, i32, i32, i32, i32) {
  fn drawChord(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN8QPainter9drawChordEiiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (i32, i32, i32, i32, &'a QPixmap) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiiiRK7QPixmap()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapEiiiiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QPainter::setWindow(const QRect & window);
impl /*struct*/ QPainter {
  pub fn setWindow<RetType, T: QPainter_setWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWindow(self);
    // return 1;
  }
}

pub trait QPainter_setWindow<RetType> {
  fn setWindow(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setWindow(const QRect & window);
impl<'a> /*trait*/ QPainter_setWindow<()> for (&'a QRect) {
  fn setWindow(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setWindowERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9setWindowERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMatrix & QPainter::deviceMatrix();
impl /*struct*/ QPainter {
  pub fn deviceMatrix<RetType, T: QPainter_deviceMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deviceMatrix(self);
    // return 1;
  }
}

pub trait QPainter_deviceMatrix<RetType> {
  fn deviceMatrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QMatrix & QPainter::deviceMatrix();
impl<'a> /*trait*/ QPainter_deviceMatrix<QMatrix> for () {
  fn deviceMatrix(self , rsthis: & QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter12deviceMatrixEv()};
    let mut ret = unsafe {_ZNK8QPainter12deviceMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawLines(const QPointF * pointPairs, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines<()> for (&'a QPointF, i32) {
  fn drawLines(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QPointF, &'a QPixmap) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRect QPainter::boundingRect(int x, int y, int w, int h, int flags, const QString & text);
impl<'a> /*trait*/ QPainter_boundingRect<QRect> for (i32, i32, i32, i32, i32, &'a QString) {
  fn boundingRect(self , rsthis: & QPainter) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectEiiiiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter12boundingRectEiiiiiRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawLines(const QLine * lines, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines<()> for (&'a QLine, i32) {
  fn drawLines(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK5QLinei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK5QLinei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
impl /*struct*/ QPainter {
  pub fn drawPie<RetType, T: QPainter_drawPie<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPie(self);
    // return 1;
  }
}

pub trait QPainter_drawPie<RetType> {
  fn drawPie(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie<()> for (i32, i32, i32, i32, i32, i32) {
  fn drawPie(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawPieEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN8QPainter7drawPieEiiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm, const QRect & sr);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QPoint, &'a QPixmap, &'a QRect) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawStaticText(const QPointF & topLeftPosition, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText<()> for (&'a QPointF, &'a QStaticText) {
  fn drawStaticText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setWorldMatrixEnabled(bool enabled);
impl /*struct*/ QPainter {
  pub fn setWorldMatrixEnabled<RetType, T: QPainter_setWorldMatrixEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWorldMatrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_setWorldMatrixEnabled<RetType> {
  fn setWorldMatrixEnabled(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setWorldMatrixEnabled(bool enabled);
impl<'a> /*trait*/ QPainter_setWorldMatrixEnabled<()> for (i8) {
  fn setWorldMatrixEnabled(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter21setWorldMatrixEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QPainter21setWorldMatrixEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::QPainter(const QPainter & );
impl /*struct*/ QPainter {
  pub fn New<T: QPainter_New>(value: T) -> QPainter {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPainter_New {
  fn New(self) -> QPainter;
}

  // proto:  void QPainter::QPainter(const QPainter & );
impl<'a> /*trait*/ QPainter_New for (&'a QPainter) {
  fn New(self) -> QPainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainterC1ERKS_(qthis, arg0)};
    let rsthis = QPainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainter::drawPoints(const QPolygon & points);
impl /*struct*/ QPainter {
  pub fn drawPoints<RetType, T: QPainter_drawPoints<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPoints(self);
    // return 1;
  }
}

pub trait QPainter_drawPoints<RetType> {
  fn drawPoints(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPoints(const QPolygon & points);
impl<'a> /*trait*/ QPainter_drawPoints<()> for (&'a QPolygon) {
  fn drawPoints(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPointsERK8QPolygon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPicture(const QPoint & p, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture<()> for (&'a QPoint, &'a QPicture) {
  fn drawPicture(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureERK6QPointRK8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawPictureERK6QPointRK8QPicture(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawRect(int x1, int y1, int w, int h);
impl /*struct*/ QPainter {
  pub fn drawRect<RetType, T: QPainter_drawRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawRect(self);
    // return 1;
  }
}

pub trait QPainter_drawRect<RetType> {
  fn drawRect(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawRect(int x1, int y1, int w, int h);
impl<'a> /*trait*/ QPainter_drawRect<()> for (i32, i32, i32, i32) {
  fn drawRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter8drawRectEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::drawEllipse(const QRectF & r);
impl<'a> /*trait*/ QPainter_drawEllipse<()> for (&'a QRectF) {
  fn drawEllipse(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawRect(const QRectF & rect);
impl<'a> /*trait*/ QPainter_drawRect<()> for (&'a QRectF) {
  fn drawRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPoints(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPoints<()> for (&'a QPointF, i32) {
  fn drawPoints(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter10drawPointsEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRegion QPainter::clipRegion();
impl /*struct*/ QPainter {
  pub fn clipRegion<RetType, T: QPainter_clipRegion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipRegion(self);
    // return 1;
  }
}

pub trait QPainter_clipRegion<RetType> {
  fn clipRegion(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QRegion QPainter::clipRegion();
impl<'a> /*trait*/ QPainter_clipRegion<QRegion> for () {
  fn clipRegion(self , rsthis: & QPainter) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter10clipRegionEv()};
    let mut ret = unsafe {_ZNK8QPainter10clipRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawText(const QRectF & r, int flags, const QString & text, QRectF * br);
impl<'a> /*trait*/ QPainter_drawText<()> for (&'a QRectF, i32, &'a QString, &'a QRectF) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::drawLine(const QLineF & line);
impl /*struct*/ QPainter {
  pub fn drawLine<RetType, T: QPainter_drawLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawLine(self);
    // return 1;
  }
}

pub trait QPainter_drawLine<RetType> {
  fn drawLine(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawLine(const QLineF & line);
impl<'a> /*trait*/ QPainter_drawLine<()> for (&'a QLineF) {
  fn drawLine(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK6QLineF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK6QLineF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawLine(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPainter_drawLine<()> for (&'a QPointF, &'a QPointF) {
  fn drawLine(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QRect & r, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QRect, &'a QPixmap) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap & , int sx, int sy);
impl<'a> /*trait*/ QPainter_drawTiledPixmap<()> for (i32, i32, i32, i32, &'a QPixmap, i32, i32) {
  fn drawTiledPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
     unsafe {_ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  QPaintDevice * QPainter::device();
impl /*struct*/ QPainter {
  pub fn device<RetType, T: QPainter_device<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QPainter_device<RetType> {
  fn device(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QPaintDevice * QPainter::device();
impl<'a> /*trait*/ QPainter_device<QPaintDevice> for () {
  fn device(self , rsthis: & QPainter) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6deviceEv()};
    let mut ret = unsafe {_ZNK8QPainter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::setViewport(const QRect & viewport);
impl /*struct*/ QPainter {
  pub fn setViewport<RetType, T: QPainter_setViewport<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setViewport(self);
    // return 1;
  }
}

pub trait QPainter_setViewport<RetType> {
  fn setViewport(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setViewport(const QRect & viewport);
impl<'a> /*trait*/ QPainter_setViewport<()> for (&'a QRect) {
  fn setViewport(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setViewportERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11setViewportERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::fillRect(const QRect & , const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect<()> for (&'a QRect, &'a QColor) {
  fn fillRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK5QRectRK6QColor()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK5QRectRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setBrushOrigin(const QPointF & );
impl<'a> /*trait*/ QPainter_setBrushOrigin<()> for (&'a QPointF) {
  fn setBrushOrigin(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14setBrushOriginERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
impl /*struct*/ QPainter {
  pub fn drawTextItem<RetType, T: QPainter_drawTextItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawTextItem(self);
    // return 1;
  }
}

pub trait QPainter_drawTextItem<RetType> {
  fn drawTextItem(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem<()> for (i32, i32, &'a QTextItem) {
  fn drawTextItem(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemEiiRK9QTextItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawTextItemEiiRK9QTextItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::QPainter(QPaintDevice * );
impl<'a> /*trait*/ QPainter_New for (&'a QPaintDevice) {
  fn New(self) -> QPainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterC1EP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainterC1EP12QPaintDevice(qthis, arg0)};
    let rsthis = QPainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm, int sx, int sy, int sw, int sh);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (i32, i32, i32, i32, &'a QPixmap, i32, i32, i32, i32) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
    let arg8 = self.8  as c_int;
     unsafe {_ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    // return 1;
  }
}

  // proto:  void QPainter::drawImage(const QPoint & p, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage<()> for (&'a QPoint, &'a QImage) {
  fn drawImage(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK6QPointRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK6QPointRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPie(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie<()> for (&'a QRect, i32, i32) {
  fn drawPie(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawPieERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter7drawPieERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawTextItem(const QPoint & p, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem<()> for (&'a QPoint, &'a QTextItem) {
  fn drawTextItem(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawLines(const QPoint * pointPairs, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines<()> for (&'a QPoint, i32) {
  fn drawLines(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPicture(int x, int y, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture<()> for (i32, i32, &'a QPicture) {
  fn drawPicture(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureEiiRK8QPicture()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawPictureEiiRK8QPicture(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::save();
impl /*struct*/ QPainter {
  pub fn save<RetType, T: QPainter_save<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.save(self);
    // return 1;
  }
}

pub trait QPainter_save<RetType> {
  fn save(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::save();
impl<'a> /*trait*/ QPainter_save<()> for () {
  fn save(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter4saveEv()};
     unsafe {_ZN8QPainter4saveEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPainter::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainter_translate<()> for (f64, f64) {
  fn translate(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN8QPainter9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTransform QPainter::combinedTransform();
impl /*struct*/ QPainter {
  pub fn combinedTransform<RetType, T: QPainter_combinedTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.combinedTransform(self);
    // return 1;
  }
}

pub trait QPainter_combinedTransform<RetType> {
  fn combinedTransform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QTransform QPainter::combinedTransform();
impl<'a> /*trait*/ QPainter_combinedTransform<QTransform> for () {
  fn combinedTransform(self , rsthis: & QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter17combinedTransformEv()};
    let mut ret = unsafe {_ZNK8QPainter17combinedTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPainter::end();
impl /*struct*/ QPainter {
  pub fn end<RetType, T: QPainter_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QPainter_end<RetType> {
  fn end(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::end();
impl<'a> /*trait*/ QPainter_end<i8> for () {
  fn end(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter3endEv()};
    let mut ret = unsafe {_ZN8QPainter3endEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::setViewport(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_setViewport<()> for (i32, i32, i32, i32) {
  fn setViewport(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setViewportEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter11setViewportEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
impl /*struct*/ QPainter {
  pub fn drawRoundRect<RetType, T: QPainter_drawRoundRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawRoundRect(self);
    // return 1;
  }
}

pub trait QPainter_drawRoundRect<RetType> {
  fn drawRoundRect(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
impl<'a> /*trait*/ QPainter_drawRoundRect<()> for (&'a QRect, i32, i32) {
  fn drawRoundRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter13drawRoundRectERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
impl /*struct*/ QPainter {
  pub fn setWorldTransform<RetType, T: QPainter_setWorldTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWorldTransform(self);
    // return 1;
  }
}

pub trait QPainter_setWorldTransform<RetType> {
  fn setWorldTransform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setWorldTransform<()> for (&'a QTransform, i8) {
  fn setWorldTransform(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17setWorldTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN8QPainter17setWorldTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPoints(const QPolygonF & points);
impl<'a> /*trait*/ QPainter_drawPoints<()> for (&'a QPolygonF) {
  fn drawPoints(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPointsERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::restore();
impl /*struct*/ QPainter {
  pub fn restore<RetType, T: QPainter_restore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restore(self);
    // return 1;
  }
}

pub trait QPainter_restore<RetType> {
  fn restore(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::restore();
impl<'a> /*trait*/ QPainter_restore<()> for () {
  fn restore(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7restoreEv()};
     unsafe {_ZN8QPainter7restoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPainter::drawStaticText(const QPoint & topLeftPosition, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText<()> for (&'a QPoint, &'a QStaticText) {
  fn drawStaticText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRectF QPainter::boundingRect(const QRectF & rect, int flags, const QString & text);
impl<'a> /*trait*/ QPainter_boundingRect<QRectF> for (&'a QRectF, i32, &'a QString) {
  fn boundingRect(self , rsthis: & QPainter) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectERK6QRectFiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter12boundingRectERK6QRectFiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect<()> for (i32, i32, i32, i32, &'a QBrush) {
  fn fillRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectEiiiiRK6QBrush()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectEiiiiRK6QBrush(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QPainter::drawRoundRect(const QRectF & r, int xround, int yround);
impl<'a> /*trait*/ QPainter_drawRoundRect<()> for (&'a QRectF, i32, i32) {
  fn drawRoundRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter13drawRoundRectERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPoint(const QPoint & p);
impl /*struct*/ QPainter {
  pub fn drawPoint<RetType, T: QPainter_drawPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawPoint(self);
    // return 1;
  }
}

pub trait QPainter_drawPoint<RetType> {
  fn drawPoint(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawPoint(const QPoint & p);
impl<'a> /*trait*/ QPainter_drawPoint<()> for (&'a QPoint) {
  fn drawPoint(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawPointERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
impl /*struct*/ QPainter {
  pub fn redirected_s<RetType, T: QPainter_redirected_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.redirected_s();
    // return 1;
  }
}

pub trait QPainter_redirected_s<RetType> {
  fn redirected_s(self ) -> RetType;
}

  // proto: static QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
impl<'a> /*trait*/ QPainter_redirected_s<QPaintDevice> for (&'a QPaintDevice, &'a QPoint) {
  fn redirected_s(self ) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint(arg0, arg1)};
    let mut ret1 = QPaintDevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::shear(qreal sh, qreal sv);
impl /*struct*/ QPainter {
  pub fn shear<RetType, T: QPainter_shear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shear(self);
    // return 1;
  }
}

pub trait QPainter_shear<RetType> {
  fn shear(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QPainter_shear<()> for (f64, f64) {
  fn shear(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN8QPainter5shearEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawText(const QRect & r, int flags, const QString & text, QRect * br);
impl<'a> /*trait*/ QPainter_drawText<()> for (&'a QRect, i32, &'a QString, &'a QRect) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  const QFont & QPainter::font();
impl /*struct*/ QPainter {
  pub fn font<RetType, T: QPainter_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QPainter_font<RetType> {
  fn font(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QFont & QPainter::font();
impl<'a> /*trait*/ QPainter_font<QFont> for () {
  fn font(self , rsthis: & QPainter) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter4fontEv()};
    let mut ret = unsafe {_ZNK8QPainter4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QTransform & QPainter::deviceTransform();
impl /*struct*/ QPainter {
  pub fn deviceTransform<RetType, T: QPainter_deviceTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deviceTransform(self);
    // return 1;
  }
}

pub trait QPainter_deviceTransform<RetType> {
  fn deviceTransform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QTransform & QPainter::deviceTransform();
impl<'a> /*trait*/ QPainter_deviceTransform<QTransform> for () {
  fn deviceTransform(self , rsthis: & QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter15deviceTransformEv()};
    let mut ret = unsafe {_ZNK8QPainter15deviceTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::eraseRect(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_eraseRect<()> for (i32, i32, i32, i32) {
  fn eraseRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter9eraseRectEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::resetMatrix();
impl /*struct*/ QPainter {
  pub fn resetMatrix<RetType, T: QPainter_resetMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix(self);
    // return 1;
  }
}

pub trait QPainter_resetMatrix<RetType> {
  fn resetMatrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::resetMatrix();
impl<'a> /*trait*/ QPainter_resetMatrix<()> for () {
  fn resetMatrix(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11resetMatrixEv()};
     unsafe {_ZN8QPainter11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPolyline(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPolyline<()> for (&'a QPoint, i32) {
  fn drawPolyline(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter12drawPolylineEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPaintEngine * QPainter::paintEngine();
impl /*struct*/ QPainter {
  pub fn paintEngine<RetType, T: QPainter_paintEngine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintEngine(self);
    // return 1;
  }
}

pub trait QPainter_paintEngine<RetType> {
  fn paintEngine(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QPaintEngine * QPainter::paintEngine();
impl<'a> /*trait*/ QPainter_paintEngine<QPaintEngine> for () {
  fn paintEngine(self , rsthis: & QPainter) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11paintEngineEv()};
    let mut ret = unsafe {_ZNK8QPainter11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawEllipse(const QRect & r);
impl<'a> /*trait*/ QPainter_drawEllipse<()> for (&'a QRect) {
  fn drawEllipse(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawEllipseERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawLine(const QLine & line);
impl<'a> /*trait*/ QPainter_drawLine<()> for (&'a QLine) {
  fn drawLine(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK5QLine(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPainter::isActive();
impl /*struct*/ QPainter {
  pub fn isActive<RetType, T: QPainter_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QPainter_isActive<RetType> {
  fn isActive(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::isActive();
impl<'a> /*trait*/ QPainter_isActive<i8> for () {
  fn isActive(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8isActiveEv()};
    let mut ret = unsafe {_ZNK8QPainter8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::drawArc(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc<()> for (&'a QRectF, i32, i32) {
  fn drawArc(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter7drawArcERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static void QPainter::restoreRedirected(const QPaintDevice * device);
impl /*struct*/ QPainter {
  pub fn restoreRedirected_s<RetType, T: QPainter_restoreRedirected_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.restoreRedirected_s();
    // return 1;
  }
}

pub trait QPainter_restoreRedirected_s<RetType> {
  fn restoreRedirected_s(self ) -> RetType;
}

  // proto: static void QPainter::restoreRedirected(const QPaintDevice * device);
impl<'a> /*trait*/ QPainter_restoreRedirected_s<()> for (&'a QPaintDevice) {
  fn restoreRedirected_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17restoreRedirectedEPK12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter17restoreRedirectedEPK12QPaintDevice(arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm, const QRectF & sr);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QPointF, &'a QPixmap, &'a QRectF) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawEllipse(const QPointF & center, qreal rx, qreal ry);
impl<'a> /*trait*/ QPainter_drawEllipse<()> for (&'a QPointF, f64, f64) {
  fn drawEllipse(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK7QPointFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN8QPainter11drawEllipseERK7QPointFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawConvexPolygon(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawConvexPolygon<()> for (&'a QPointF, i32) {
  fn drawConvexPolygon(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter17drawConvexPolygonEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setBrushOrigin(const QPoint & );
impl<'a> /*trait*/ QPainter_setBrushOrigin<()> for (&'a QPoint) {
  fn setBrushOrigin(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14setBrushOriginERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawText(const QRectF & r, const QString & text, const QTextOption & o);
impl<'a> /*trait*/ QPainter_drawText<()> for (&'a QRectF, &'a QString, &'a QTextOption) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QPainter::worldMatrixEnabled();
impl /*struct*/ QPainter {
  pub fn worldMatrixEnabled<RetType, T: QPainter_worldMatrixEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.worldMatrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_worldMatrixEnabled<RetType> {
  fn worldMatrixEnabled(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::worldMatrixEnabled();
impl<'a> /*trait*/ QPainter_worldMatrixEnabled<i8> for () {
  fn worldMatrixEnabled(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter18worldMatrixEnabledEv()};
    let mut ret = unsafe {_ZNK8QPainter18worldMatrixEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QPoint, &'a QPixmap) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QPainter_drawLine<()> for (i32, i32, i32, i32) {
  fn drawLine(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter8drawLineEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPoint(int x, int y);
impl<'a> /*trait*/ QPainter_drawPoint<()> for (i32, i32) {
  fn drawPoint(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawPointEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QTransform & QPainter::transform();
impl /*struct*/ QPainter {
  pub fn transform<RetType, T: QPainter_transform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QPainter_transform<RetType> {
  fn transform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QTransform & QPainter::transform();
impl<'a> /*trait*/ QPainter_transform<QTransform> for () {
  fn transform(self , rsthis: & QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter9transformEv()};
    let mut ret = unsafe {_ZNK8QPainter9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
impl /*struct*/ QPainter {
  pub fn setRedirected_s<RetType, T: QPainter_setRedirected_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setRedirected_s();
    // return 1;
  }
}

pub trait QPainter_setRedirected_s<RetType> {
  fn setRedirected_s(self ) -> RetType;
}

  // proto: static void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
impl<'a> /*trait*/ QPainter_setRedirected_s<()> for (&'a QPaintDevice, &'a QPaintDevice, &'a QPoint) {
  fn setRedirected_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPixmap(const QRect & targetRect, const QPixmap & pixmap, const QRect & sourceRect);
impl<'a> /*trait*/ QPainter_drawPixmap<()> for (&'a QRect, &'a QPixmap, &'a QRect) {
  fn drawPixmap(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QFontMetrics QPainter::fontMetrics();
impl /*struct*/ QPainter {
  pub fn fontMetrics<RetType, T: QPainter_fontMetrics<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontMetrics(self);
    // return 1;
  }
}

pub trait QPainter_fontMetrics<RetType> {
  fn fontMetrics(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QFontMetrics QPainter::fontMetrics();
impl<'a> /*trait*/ QPainter_fontMetrics<QFontMetrics> for () {
  fn fontMetrics(self , rsthis: & QPainter) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11fontMetricsEv()};
    let mut ret = unsafe {_ZNK8QPainter11fontMetricsEv(rsthis.qclsinst)};
    let mut ret1 = QFontMetrics::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
impl /*struct*/ QPainter {
  pub fn drawGlyphRun<RetType, T: QPainter_drawGlyphRun<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawGlyphRun(self);
    // return 1;
  }
}

pub trait QPainter_drawGlyphRun<RetType> {
  fn drawGlyphRun(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
impl<'a> /*trait*/ QPainter_drawGlyphRun<()> for (&'a QPointF, &'a QGlyphRun) {
  fn drawGlyphRun(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::fillRect(const QRectF & , const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect<()> for (&'a QRectF, &'a QBrush) {
  fn fillRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK6QRectFRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK6QRectFRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::resetTransform();
impl /*struct*/ QPainter {
  pub fn resetTransform<RetType, T: QPainter_resetTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetTransform(self);
    // return 1;
  }
}

pub trait QPainter_resetTransform<RetType> {
  fn resetTransform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::resetTransform();
impl<'a> /*trait*/ QPainter_resetTransform<()> for () {
  fn resetTransform(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14resetTransformEv()};
     unsafe {_ZN8QPainter14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPainter::fillRect(const QRect & , const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect<()> for (&'a QRect, &'a QBrush) {
  fn fillRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK5QRectRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK5QRectRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QBrush & QPainter::brush();
impl /*struct*/ QPainter {
  pub fn brush<RetType, T: QPainter_brush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brush(self);
    // return 1;
  }
}

pub trait QPainter_brush<RetType> {
  fn brush(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QBrush & QPainter::brush();
impl<'a> /*trait*/ QPainter_brush<QBrush> for () {
  fn brush(self , rsthis: & QPainter) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter5brushEv()};
    let mut ret = unsafe {_ZNK8QPainter5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::~QPainter();
impl /*struct*/ QPainter {
  pub fn Free<RetType, T: QPainter_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPainter_Free<RetType> {
  fn Free(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::~QPainter();
impl<'a> /*trait*/ QPainter_Free<()> for () {
  fn Free(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterD0Ev()};
     unsafe {_ZN8QPainterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QPainter::begin(QPaintDevice * );
impl /*struct*/ QPainter {
  pub fn begin<RetType, T: QPainter_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QPainter_begin<RetType> {
  fn begin(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::begin(QPaintDevice * );
impl<'a> /*trait*/ QPainter_begin<i8> for (&'a QPaintDevice) {
  fn begin(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5beginEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter5beginEP12QPaintDevice(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::drawRect(const QRect & rect);
impl<'a> /*trait*/ QPainter_drawRect<()> for (&'a QRect) {
  fn drawRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawTextItem(const QPointF & p, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem<()> for (&'a QPointF, &'a QTextItem) {
  fn drawTextItem(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::scale(qreal sx, qreal sy);
impl /*struct*/ QPainter {
  pub fn scale<RetType, T: QPainter_scale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QPainter_scale<RetType> {
  fn scale(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QPainter_scale<()> for (f64, f64) {
  fn scale(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN8QPainter5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
impl /*struct*/ QPainter {
  pub fn setWorldMatrix<RetType, T: QPainter_setWorldMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWorldMatrix(self);
    // return 1;
  }
}

pub trait QPainter_setWorldMatrix<RetType> {
  fn setWorldMatrix(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setWorldMatrix<()> for (&'a QMatrix, i8) {
  fn setWorldMatrix(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setWorldMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN8QPainter14setWorldMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainter::clipPath();
impl /*struct*/ QPainter {
  pub fn clipPath<RetType, T: QPainter_clipPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipPath(self);
    // return 1;
  }
}

pub trait QPainter_clipPath<RetType> {
  fn clipPath(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QPainterPath QPainter::clipPath();
impl<'a> /*trait*/ QPainter_clipPath<QPainterPath> for () {
  fn clipPath(self , rsthis: & QPainter) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8clipPathEv()};
    let mut ret = unsafe {_ZNK8QPainter8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QPainter::brushOrigin();
impl /*struct*/ QPainter {
  pub fn brushOrigin<RetType, T: QPainter_brushOrigin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brushOrigin(self);
    // return 1;
  }
}

pub trait QPainter_brushOrigin<RetType> {
  fn brushOrigin(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QPoint QPainter::brushOrigin();
impl<'a> /*trait*/ QPainter_brushOrigin<QPoint> for () {
  fn brushOrigin(self , rsthis: & QPainter) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11brushOriginEv()};
    let mut ret = unsafe {_ZNK8QPainter11brushOriginEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawConvexPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QPainter_drawConvexPolygon<()> for (&'a QPolygonF) {
  fn drawConvexPolygon(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter17drawConvexPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawEllipse(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_drawEllipse<()> for (i32, i32, i32, i32) {
  fn drawEllipse(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter11drawEllipseEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::drawConvexPolygon(const QPolygon & polygon);
impl<'a> /*trait*/ QPainter_drawConvexPolygon<()> for (&'a QPolygon) {
  fn drawConvexPolygon(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter17drawConvexPolygonERK8QPolygon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPoints(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPoints<()> for (&'a QPoint, i32) {
  fn drawPoints(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter10drawPointsEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QBrush & QPainter::background();
impl /*struct*/ QPainter {
  pub fn background<RetType, T: QPainter_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QPainter_background<RetType> {
  fn background(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QBrush & QPainter::background();
impl<'a> /*trait*/ QPainter_background<QBrush> for () {
  fn background(self , rsthis: & QPainter) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter10backgroundEv()};
    let mut ret = unsafe {_ZNK8QPainter10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawRoundRect(int x, int y, int w, int h, int , int );
impl<'a> /*trait*/ QPainter_drawRoundRect<()> for (i32, i32, i32, i32, i32, i32) {
  fn drawRoundRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN8QPainter13drawRoundRectEiiiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QRect QPainter::viewport();
impl /*struct*/ QPainter {
  pub fn viewport<RetType, T: QPainter_viewport<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.viewport(self);
    // return 1;
  }
}

pub trait QPainter_viewport<RetType> {
  fn viewport(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QRect QPainter::viewport();
impl<'a> /*trait*/ QPainter_viewport<QRect> for () {
  fn viewport(self , rsthis: & QPainter) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8viewportEv()};
    let mut ret = unsafe {_ZNK8QPainter8viewportEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawArc(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc<()> for (&'a QRect, i32, i32) {
  fn drawArc(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter7drawArcERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
impl /*struct*/ QPainter {
  pub fn fillPath<RetType, T: QPainter_fillPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fillPath(self);
    // return 1;
  }
}

pub trait QPainter_fillPath<RetType> {
  fn fillPath(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
impl<'a> /*trait*/ QPainter_fillPath<()> for (&'a QPainterPath, &'a QBrush) {
  fn fillPath(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawText(int x, int y, int w, int h, int flags, const QString & text, QRect * br);
impl<'a> /*trait*/ QPainter_drawText<()> for (i32, i32, i32, i32, i32, &'a QString, &'a QRect) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5.qclsinst  as *mut c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

  // proto:  bool QPainter::matrixEnabled();
impl /*struct*/ QPainter {
  pub fn matrixEnabled<RetType, T: QPainter_matrixEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_matrixEnabled<RetType> {
  fn matrixEnabled(self , rsthis: & QPainter) -> RetType;
}

  // proto:  bool QPainter::matrixEnabled();
impl<'a> /*trait*/ QPainter_matrixEnabled<i8> for () {
  fn matrixEnabled(self , rsthis: & QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter13matrixEnabledEv()};
    let mut ret = unsafe {_ZNK8QPainter13matrixEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainter::drawPolyline(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPolyline<()> for (&'a QPointF, i32) {
  fn drawPolyline(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter12drawPolylineEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setTransform(const QTransform & transform, bool combine);
impl /*struct*/ QPainter {
  pub fn setTransform<RetType, T: QPainter_setTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransform(self);
    // return 1;
  }
}

pub trait QPainter_setTransform<RetType> {
  fn setTransform(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setTransform(const QTransform & transform, bool combine);
impl<'a> /*trait*/ QPainter_setTransform<()> for (&'a QTransform, i8) {
  fn setTransform(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN8QPainter12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setPen(const QColor & color);
impl<'a> /*trait*/ QPainter_setPen<()> for (&'a QColor) {
  fn setPen(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6setPenERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter6setPenERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::eraseRect(const QRect & );
impl<'a> /*trait*/ QPainter_eraseRect<()> for (&'a QRect) {
  fn eraseRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9eraseRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QPainter::window();
impl /*struct*/ QPainter {
  pub fn window<RetType, T: QPainter_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QPainter_window<RetType> {
  fn window(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QRect QPainter::window();
impl<'a> /*trait*/ QPainter_window<QRect> for () {
  fn window(self , rsthis: & QPainter) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6windowEv()};
    let mut ret = unsafe {_ZNK8QPainter6windowEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawImage(const QRect & r, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage<()> for (&'a QRect, &'a QImage) {
  fn drawImage(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK5QRectRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK5QRectRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::initFrom(const QPaintDevice * device);
impl /*struct*/ QPainter {
  pub fn initFrom<RetType, T: QPainter_initFrom<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initFrom(self);
    // return 1;
  }
}

pub trait QPainter_initFrom<RetType> {
  fn initFrom(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::initFrom(const QPaintDevice * device);
impl<'a> /*trait*/ QPainter_initFrom<()> for (&'a QPaintDevice) {
  fn initFrom(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8initFromEPK12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8initFromEPK12QPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFontInfo QPainter::fontInfo();
impl /*struct*/ QPainter {
  pub fn fontInfo<RetType, T: QPainter_fontInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontInfo(self);
    // return 1;
  }
}

pub trait QPainter_fontInfo<RetType> {
  fn fontInfo(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QFontInfo QPainter::fontInfo();
impl<'a> /*trait*/ QPainter_fontInfo<QFontInfo> for () {
  fn fontInfo(self , rsthis: & QPainter) -> QFontInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8fontInfoEv()};
    let mut ret = unsafe {_ZNK8QPainter8fontInfoEv(rsthis.qclsinst)};
    let mut ret1 = QFontInfo::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::endNativePainting();
impl /*struct*/ QPainter {
  pub fn endNativePainting<RetType, T: QPainter_endNativePainting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endNativePainting(self);
    // return 1;
  }
}

pub trait QPainter_endNativePainting<RetType> {
  fn endNativePainting(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::endNativePainting();
impl<'a> /*trait*/ QPainter_endNativePainting<()> for () {
  fn endNativePainting(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17endNativePaintingEv()};
     unsafe {_ZN8QPainter17endNativePaintingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPainter::setViewTransformEnabled(bool enable);
impl /*struct*/ QPainter {
  pub fn setViewTransformEnabled<RetType, T: QPainter_setViewTransformEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setViewTransformEnabled(self);
    // return 1;
  }
}

pub trait QPainter_setViewTransformEnabled<RetType> {
  fn setViewTransformEnabled(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setViewTransformEnabled(bool enable);
impl<'a> /*trait*/ QPainter_setViewTransformEnabled<()> for (i8) {
  fn setViewTransformEnabled(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter23setViewTransformEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN8QPainter23setViewTransformEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPoint(const QPointF & pt);
impl<'a> /*trait*/ QPainter_drawPoint<()> for (&'a QPointF) {
  fn drawPoint(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::setOpacity(qreal opacity);
impl /*struct*/ QPainter {
  pub fn setOpacity<RetType, T: QPainter_setOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QPainter_setOpacity<RetType> {
  fn setOpacity(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::setOpacity(qreal opacity);
impl<'a> /*trait*/ QPainter_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN8QPainter10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::fillRect(const QRectF & , const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect<()> for (&'a QRectF, &'a QColor) {
  fn fillRect(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK6QRectFRK6QColor()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK6QRectFRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::QPainter();
impl<'a> /*trait*/ QPainter_New for () {
  fn New(self) -> QPainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterC1Ev()};
    unsafe {_ZN8QPainterC1Ev(qthis)};
    let rsthis = QPainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainter::translate(const QPointF & offset);
impl<'a> /*trait*/ QPainter_translate<()> for (&'a QPointF) {
  fn translate(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainter::drawText(const QPointF & p, const QString & s);
impl<'a> /*trait*/ QPainter_drawText<()> for (&'a QPointF, &'a QString) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK7QPointFRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK7QPointFRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawImage(const QPointF & p, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage<()> for (&'a QPointF, &'a QImage) {
  fn drawImage(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK7QPointFRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK7QPointFRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QPen & QPainter::pen();
impl /*struct*/ QPainter {
  pub fn pen<RetType, T: QPainter_pen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pen(self);
    // return 1;
  }
}

pub trait QPainter_pen<RetType> {
  fn pen(self , rsthis: & QPainter) -> RetType;
}

  // proto:  const QPen & QPainter::pen();
impl<'a> /*trait*/ QPainter_pen<QPen> for () {
  fn pen(self , rsthis: & QPainter) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter3penEv()};
    let mut ret = unsafe {_ZNK8QPainter3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::rotate(qreal a);
impl /*struct*/ QPainter {
  pub fn rotate<RetType, T: QPainter_rotate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotate(self);
    // return 1;
  }
}

pub trait QPainter_rotate<RetType> {
  fn rotate(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::rotate(qreal a);
impl<'a> /*trait*/ QPainter_rotate<()> for (f64) {
  fn rotate(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6rotateEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN8QPainter6rotateEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QPainter::clipBoundingRect();
impl /*struct*/ QPainter {
  pub fn clipBoundingRect<RetType, T: QPainter_clipBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipBoundingRect(self);
    // return 1;
  }
}

pub trait QPainter_clipBoundingRect<RetType> {
  fn clipBoundingRect(self , rsthis: & QPainter) -> RetType;
}

  // proto:  QRectF QPainter::clipBoundingRect();
impl<'a> /*trait*/ QPainter_clipBoundingRect<QRectF> for () {
  fn clipBoundingRect(self , rsthis: & QPainter) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter16clipBoundingRectEv()};
    let mut ret = unsafe {_ZNK8QPainter16clipBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainter::drawLine(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPainter_drawLine<()> for (&'a QPoint, &'a QPoint) {
  fn drawLine(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK6QPointS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::drawPie(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie<()> for (&'a QRectF, i32, i32) {
  fn drawPie(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawPieERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter7drawPieERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainter::drawText(const QPoint & p, const QString & s);
impl<'a> /*trait*/ QPainter_drawText<()> for (&'a QPoint, &'a QString) {
  fn drawText(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QPointRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK6QPointRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainter::setWindow(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_setWindow<()> for (i32, i32, i32, i32) {
  fn setWindow(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setWindowEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPainter9setWindowEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainter::beginNativePainting();
impl /*struct*/ QPainter {
  pub fn beginNativePainting<RetType, T: QPainter_beginNativePainting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginNativePainting(self);
    // return 1;
  }
}

pub trait QPainter_beginNativePainting<RetType> {
  fn beginNativePainting(self , rsthis: & QPainter) -> RetType;
}

  // proto:  void QPainter::beginNativePainting();
impl<'a> /*trait*/ QPainter_beginNativePainting<()> for () {
  fn beginNativePainting(self , rsthis: & QPainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter19beginNativePaintingEv()};
     unsafe {_ZN8QPainter19beginNativePaintingEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

