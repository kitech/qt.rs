// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qpicture::QPicture;
use super::qmatrix::QMatrix;
use super::qstring::QString;
use super::qcolor::QColor;
use super::qrectf::QRectF;
use super::qpixmap::QPixmap;
use super::qbrush::QBrush;
use super::qimage::QImage;
use super::qrect::QRect;
use super::qpoint::QPoint;
use super::qpen::QPen;
use super::qlinef::QLineF;
use super::qtransform::QTransform;
use super::qpolygonf::QPolygonF;
use super::qstatictext::QStaticText;
use super::qpainterpath::QPainterPath;
use super::qpolygon::QPolygon;
use super::qfont::QFont;
use super::qline::QLine;
use super::qregion::QRegion;
use super::qpaintdevice::QPaintDevice;
use super::qtextitem::QTextItem;
use super::qpaintengine::QPaintEngine;
use super::qtextoption::QTextOption;
use super::qfontmetrics::QFontMetrics;
use super::qglyphrun::QGlyphRun;
use super::qfontinfo::QFontInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
  fn _ZN8QPainter11drawPictureERK7QPointFRK8QPicture(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QMatrix & QPainter::worldMatrix();
  fn _ZNK8QPainter11worldMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
  fn _ZN8QPainter8drawTextERK7QPointFRK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
  fn _ZN8QPainter8fillRectEiiiiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void) ;
  // proto:  const QMatrix & QPainter::matrix();
  fn _ZNK8QPainter6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QPainter::opacity();
  fn _ZNK8QPainter7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainter::drawText(int x, int y, const QString & s);
  fn _ZN8QPainter8drawTextEiiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
  fn _ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainter::setBackground(const QBrush & bg);
  fn _ZN8QPainter13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawChord(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter9drawChordERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::drawImage(const QRectF & r, const QImage & image);
  fn _ZN8QPainter9drawImageERK6QRectFRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::setClipping(bool enable);
  fn _ZN8QPainter11setClippingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPainter::setBrush(const QBrush & brush);
  fn _ZN8QPainter8setBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN8QPainter9setMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QPainter::drawChord(const QRect & , int a, int alen);
  fn _ZN8QPainter9drawChordERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::eraseRect(const QRectF & );
  fn _ZN8QPainter9eraseRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::translate(const QPoint & offset);
  fn _ZN8QPainter9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPainter::viewTransformEnabled();
  fn _ZNK8QPainter20viewTransformEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainter::setPen(const QPen & pen);
  fn _ZN8QPainter6setPenERK4QPen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawLines(const QLineF * lines, int lineCount);
  fn _ZN8QPainter9drawLinesEPK6QLineFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::setBrushOrigin(int x, int y);
  fn _ZN8QPainter14setBrushOriginEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  const QTransform & QPainter::worldTransform();
  fn _ZNK8QPainter14worldTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawRects(const QRect * rects, int rectCount);
  fn _ZN8QPainter9drawRectsEPK5QRecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
  fn _ZN8QPainter11drawEllipseERK6QPointii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter7drawArcEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) ;
  // proto:  void QPainter::drawPolyline(const QPolygonF & polyline);
  fn _ZN8QPainter12drawPolylineERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPainter::hasClipping();
  fn _ZNK8QPainter11hasClippingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
  fn _ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextEiiRK11QStaticText(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
  fn _ZN8QPainter10strokePathERK12QPainterPathRK4QPen(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm, int sx, int sy, int sw, int sh);
  fn _ZN8QPainter10drawPixmapEiiRK7QPixmapiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int) ;
  // proto:  void QPainter::drawRects(const QRectF * rects, int rectCount);
  fn _ZN8QPainter9drawRectsEPK6QRectFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
  fn _ZN8QPainter17drawConvexPolygonEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::drawPath(const QPainterPath & path);
  fn _ZN8QPainter8drawPathERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapEiiRK7QPixmap(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QMatrix QPainter::combinedMatrix();
  fn _ZNK8QPainter14combinedMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::setMatrixEnabled(bool enabled);
  fn _ZN8QPainter16setMatrixEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPainter::drawPolyline(const QPolygon & polygon);
  fn _ZN8QPainter12drawPolylineERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawTiledPixmap(const QRect & , const QPixmap & , const QPoint & );
  fn _ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainter::setFont(const QFont & f);
  fn _ZN8QPainter7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawChord(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter9drawChordEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) ;
  // proto:  void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapEiiiiRK7QPixmap(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void) ;
  // proto:  void QPainter::setWindow(const QRect & window);
  fn _ZN8QPainter9setWindowERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMatrix & QPainter::deviceMatrix();
  fn _ZNK8QPainter12deviceMatrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawLines(const QPointF * pointPairs, int lineCount);
  fn _ZN8QPainter9drawLinesEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawLines(const QLine * lines, int lineCount);
  fn _ZN8QPainter9drawLinesEPK5QLinei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter7drawPieEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) ;
  // proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm, const QRect & sr);
  fn _ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainter::drawStaticText(const QPointF & topLeftPosition, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::setWorldMatrixEnabled(bool enabled);
  fn _ZN8QPainter21setWorldMatrixEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPainter::NewQPainter(const QPainter & );
  fn _ZN8QPainterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawPoints(const QPolygon & points);
  fn _ZN8QPainter10drawPointsERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawPicture(const QPoint & p, const QPicture & picture);
  fn _ZN8QPainter11drawPictureERK6QPointRK8QPicture(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawRect(int x1, int y1, int w, int h);
  fn _ZN8QPainter8drawRectEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::drawEllipse(const QRectF & r);
  fn _ZN8QPainter11drawEllipseERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawRect(const QRectF & rect);
  fn _ZN8QPainter8drawRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawPoints(const QPointF * points, int pointCount);
  fn _ZN8QPainter10drawPointsEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QRegion QPainter::clipRegion();
  fn _ZNK8QPainter10clipRegionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawText(const QRectF & r, int flags, const QString & text, QRectF * br);
  fn _ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  void QPainter::drawLine(const QLineF & line);
  fn _ZN8QPainter8drawLineERK6QLineF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawLine(const QPointF & p1, const QPointF & p2);
  fn _ZN8QPainter8drawLineERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawPixmap(const QRect & r, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK5QRectRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap & , int sx, int sy);
  fn _ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void, arg5: c_int, arg6: c_int) ;
  // proto:  QPaintDevice * QPainter::device();
  fn _ZNK8QPainter6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::setViewport(const QRect & viewport);
  fn _ZN8QPainter11setViewportERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::fillRect(const QRect & , const QColor & color);
  fn _ZN8QPainter8fillRectERK5QRectRK6QColor(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::setBrushOrigin(const QPointF & );
  fn _ZN8QPainter14setBrushOriginERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemEiiRK9QTextItem(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QPainter::NewQPainter(QPaintDevice * );
  fn _ZN8QPainterC1EP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm, int sx, int sy, int sw, int sh);
  fn _ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int) ;
  // proto:  void QPainter::drawImage(const QPoint & p, const QImage & image);
  fn _ZN8QPainter9drawImageERK6QPointRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawPie(const QRect & , int a, int alen);
  fn _ZN8QPainter7drawPieERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::drawTextItem(const QPoint & p, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemERK6QPointRK9QTextItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawLines(const QPoint * pointPairs, int lineCount);
  fn _ZN8QPainter9drawLinesEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::drawPicture(int x, int y, const QPicture & picture);
  fn _ZN8QPainter11drawPictureEiiRK8QPicture(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) ;
  // proto:  void QPainter::save();
  fn _ZN8QPainter4saveEv(qthis: *mut c_void) ;
  // proto:  void QPainter::translate(qreal dx, qreal dy);
  fn _ZN8QPainter9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QTransform QPainter::combinedTransform();
  fn _ZNK8QPainter17combinedTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPainter::end();
  fn _ZN8QPainter3endEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainter::setViewport(int x, int y, int w, int h);
  fn _ZN8QPainter11setViewportEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
  fn _ZN8QPainter13drawRoundRectERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
  fn _ZN8QPainter17setWorldTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QPainter::drawPoints(const QPolygonF & points);
  fn _ZN8QPainter10drawPointsERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::restore();
  fn _ZN8QPainter7restoreEv(qthis: *mut c_void) ;
  // proto:  void QPainter::drawStaticText(const QPoint & topLeftPosition, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::fillRect(int x, int y, int w, int h, const QBrush & );
  fn _ZN8QPainter8fillRectEiiiiRK6QBrush(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut c_void) ;
  // proto:  void QPainter::drawRoundRect(const QRectF & r, int xround, int yround);
  fn _ZN8QPainter13drawRoundRectERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::drawPoint(const QPoint & p);
  fn _ZN8QPainter9drawPointERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
  fn _ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::shear(qreal sh, qreal sv);
  fn _ZN8QPainter5shearEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QPainter::drawText(const QRect & r, int flags, const QString & text, QRect * br);
  fn _ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  const QFont & QPainter::font();
  fn _ZNK8QPainter4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QTransform & QPainter::deviceTransform();
  fn _ZNK8QPainter15deviceTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::eraseRect(int x, int y, int w, int h);
  fn _ZN8QPainter9eraseRectEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::resetMatrix();
  fn _ZN8QPainter11resetMatrixEv(qthis: *mut c_void) ;
  // proto:  void QPainter::drawPolyline(const QPoint * points, int pointCount);
  fn _ZN8QPainter12drawPolylineEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QPaintEngine * QPainter::paintEngine();
  fn _ZNK8QPainter11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawEllipse(const QRect & r);
  fn _ZN8QPainter11drawEllipseERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawLine(const QLine & line);
  fn _ZN8QPainter8drawLineERK5QLine(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPainter::isActive();
  fn _ZNK8QPainter8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainter::drawArc(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter7drawArcERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto: static void QPainter::restoreRedirected(const QPaintDevice * device);
  fn _ZN8QPainter17restoreRedirectedEPK12QPaintDevice(arg0: *mut c_void) ;
  // proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm, const QRectF & sr);
  fn _ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainter::drawEllipse(const QPointF & center, qreal rx, qreal ry);
  fn _ZN8QPainter11drawEllipseERK7QPointFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double) ;
  // proto:  void QPainter::drawConvexPolygon(const QPointF * points, int pointCount);
  fn _ZN8QPainter17drawConvexPolygonEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::setBrushOrigin(const QPoint & );
  fn _ZN8QPainter14setBrushOriginERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawText(const QRectF & r, const QString & text, const QTextOption & o);
  fn _ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  bool QPainter::worldMatrixEnabled();
  fn _ZNK8QPainter18worldMatrixEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK6QPointRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawLine(int x1, int y1, int x2, int y2);
  fn _ZN8QPainter8drawLineEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::drawPoint(int x, int y);
  fn _ZN8QPainter9drawPointEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  const QTransform & QPainter::transform();
  fn _ZNK8QPainter9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
  fn _ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QPainter::drawPixmap(const QRect & targetRect, const QPixmap & pixmap, const QRect & sourceRect);
  fn _ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  QFontMetrics QPainter::fontMetrics();
  fn _ZNK8QPainter11fontMetricsEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
  fn _ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::fillRect(const QRectF & , const QBrush & );
  fn _ZN8QPainter8fillRectERK6QRectFRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::resetTransform();
  fn _ZN8QPainter14resetTransformEv(qthis: *mut c_void) ;
  // proto:  void QPainter::fillRect(const QRect & , const QBrush & );
  fn _ZN8QPainter8fillRectERK5QRectRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QBrush & QPainter::brush();
  fn _ZNK8QPainter5brushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::FreeQPainter();
  fn _ZN8QPainterD0Ev(qthis: *mut c_void) ;
  // proto:  bool QPainter::begin(QPaintDevice * );
  fn _ZN8QPainter5beginEP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QPainter::drawRect(const QRect & rect);
  fn _ZN8QPainter8drawRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawTextItem(const QPointF & p, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::scale(qreal sx, qreal sy);
  fn _ZN8QPainter5scaleEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
  fn _ZN8QPainter14setWorldMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  QPainterPath QPainter::clipPath();
  fn _ZNK8QPainter8clipPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QPainter::brushOrigin();
  fn _ZNK8QPainter11brushOriginEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawConvexPolygon(const QPolygonF & polygon);
  fn _ZN8QPainter17drawConvexPolygonERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawEllipse(int x, int y, int w, int h);
  fn _ZN8QPainter11drawEllipseEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::drawConvexPolygon(const QPolygon & polygon);
  fn _ZN8QPainter17drawConvexPolygonERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawPoints(const QPoint * points, int pointCount);
  fn _ZN8QPainter10drawPointsEPK6QPointi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  const QBrush & QPainter::background();
  fn _ZNK8QPainter10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawRoundRect(int x, int y, int w, int h, int , int );
  fn _ZN8QPainter13drawRoundRectEiiiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) ;
  // proto:  QRect QPainter::viewport();
  fn _ZNK8QPainter8viewportEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawArc(const QRect & , int a, int alen);
  fn _ZN8QPainter7drawArcERK5QRectii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
  fn _ZN8QPainter8fillPathERK12QPainterPathRK6QBrush(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawText(int x, int y, int w, int h, int flags, const QString & text, QRect * br);
  fn _ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *mut c_void, arg6: *mut c_void) ;
  // proto:  bool QPainter::matrixEnabled();
  fn _ZNK8QPainter13matrixEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPainter::drawPolyline(const QPointF * points, int pointCount);
  fn _ZN8QPainter12drawPolylineEPK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPainter::setTransform(const QTransform & transform, bool combine);
  fn _ZN8QPainter12setTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QPainter::setPen(const QColor & color);
  fn _ZN8QPainter6setPenERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::eraseRect(const QRect & );
  fn _ZN8QPainter9eraseRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QPainter::window();
  fn _ZNK8QPainter6windowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawImage(const QRect & r, const QImage & image);
  fn _ZN8QPainter9drawImageERK5QRectRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::initFrom(const QPaintDevice * device);
  fn _ZN8QPainter8initFromEPK12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QFontInfo QPainter::fontInfo();
  fn _ZNK8QPainter8fontInfoEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::endNativePainting();
  fn _ZN8QPainter17endNativePaintingEv(qthis: *mut c_void) ;
  // proto:  void QPainter::setViewTransformEnabled(bool enable);
  fn _ZN8QPainter23setViewTransformEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QPainter::drawPoint(const QPointF & pt);
  fn _ZN8QPainter9drawPointERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::setOpacity(qreal opacity);
  fn _ZN8QPainter10setOpacityEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPainter::fillRect(const QRectF & , const QColor & color);
  fn _ZN8QPainter8fillRectERK6QRectFRK6QColor(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::NewQPainter();
  fn _ZN8QPainterC1Ev(qthis: *mut c_void) ;
  // proto:  void QPainter::translate(const QPointF & offset);
  fn _ZN8QPainter9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPainter::drawText(const QPointF & p, const QString & s);
  fn _ZN8QPainter8drawTextERK7QPointFRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawImage(const QPointF & p, const QImage & image);
  fn _ZN8QPainter9drawImageERK7QPointFRK6QImage(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  const QPen & QPainter::pen();
  fn _ZNK8QPainter3penEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::rotate(qreal a);
  fn _ZN8QPainter6rotateEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QRectF QPainter::clipBoundingRect();
  fn _ZNK8QPainter16clipBoundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainter::drawLine(const QPoint & p1, const QPoint & p2);
  fn _ZN8QPainter8drawLineERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::drawPie(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter7drawPieERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPainter::drawText(const QPoint & p, const QString & s);
  fn _ZN8QPainter8drawTextERK6QPointRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QPainter::setWindow(int x, int y, int w, int h);
  fn _ZN8QPainter9setWindowEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPainter::beginNativePainting();
  fn _ZN8QPainter19beginNativePaintingEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QPainter)=1
pub struct QPainter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainter {
  pub fn drawPicture<T: QPainter_drawPicture>(&mut self, value: T)  {
     value.drawPicture(self);
    // return 1;
  }
}

pub trait QPainter_drawPicture {
  fn drawPicture(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture for (&'a  QPointF, &'a  QPicture) {
  fn drawPicture(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureERK7QPointFRK8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawPictureERK7QPointFRK8QPicture(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn worldMatrix<T: QPainter_worldMatrix>(&mut self, value: T) -> QMatrix {
    return value.worldMatrix(self);
    // return 1;
  }
}

pub trait QPainter_worldMatrix {
  fn worldMatrix(self, rsthis: &mut QPainter) -> QMatrix;
}

// proto:  const QMatrix & QPainter::worldMatrix();
impl<'a> /*trait*/ QPainter_worldMatrix for () {
  fn worldMatrix(self, rsthis: &mut QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11worldMatrixEv()};
    let mut ret = unsafe {_ZNK8QPainter11worldMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawText<T: QPainter_drawText>(&mut self, value: T)  {
     value.drawText(self);
    // return 1;
  }
}

pub trait QPainter_drawText {
  fn drawText(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QPointF, &'a  QString, i32, i32) {
  fn drawText(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn fillRect<T: QPainter_fillRect>(&mut self, value: T)  {
     value.fillRect(self);
    // return 1;
  }
}

pub trait QPainter_fillRect {
  fn fillRect(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect for (i32, i32, i32, i32, &'a  QColor) {
  fn fillRect(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn matrix<T: QPainter_matrix>(&mut self, value: T) -> QMatrix {
    return value.matrix(self);
    // return 1;
  }
}

pub trait QPainter_matrix {
  fn matrix(self, rsthis: &mut QPainter) -> QMatrix;
}

// proto:  const QMatrix & QPainter::matrix();
impl<'a> /*trait*/ QPainter_matrix for () {
  fn matrix(self, rsthis: &mut QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6matrixEv()};
    let mut ret = unsafe {_ZNK8QPainter6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn opacity<T: QPainter_opacity>(&mut self, value: T) -> f64 {
    return value.opacity(self);
    // return 1;
  }
}

pub trait QPainter_opacity {
  fn opacity(self, rsthis: &mut QPainter) -> f64;
}

// proto:  double QPainter::opacity();
impl<'a> /*trait*/ QPainter_opacity for () {
  fn opacity(self, rsthis: &mut QPainter) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter7opacityEv()};
    let mut ret = unsafe {_ZNK8QPainter7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QPainter::drawText(int x, int y, const QString & s);
impl<'a> /*trait*/ QPainter_drawText for (i32, i32, &'a  QString) {
  fn drawText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextEiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextEiiRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawTiledPixmap<T: QPainter_drawTiledPixmap>(&mut self, value: T)  {
     value.drawTiledPixmap(self);
    // return 1;
  }
}

pub trait QPainter_drawTiledPixmap {
  fn drawTiledPixmap(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
impl<'a> /*trait*/ QPainter_drawTiledPixmap for (&'a  QRectF, &'a  QPixmap, &'a  QPointF) {
  fn drawTiledPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setBackground<T: QPainter_setBackground>(&mut self, value: T)  {
     value.setBackground(self);
    // return 1;
  }
}

pub trait QPainter_setBackground {
  fn setBackground(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setBackground(const QBrush & bg);
impl<'a> /*trait*/ QPainter_setBackground for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawChord<T: QPainter_drawChord>(&mut self, value: T)  {
     value.drawChord(self);
    // return 1;
  }
}

pub trait QPainter_drawChord {
  fn drawChord(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawChord(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord for (&'a  QRectF, i32, i32) {
  fn drawChord(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter9drawChordERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawImage<T: QPainter_drawImage>(&mut self, value: T)  {
     value.drawImage(self);
    // return 1;
  }
}

pub trait QPainter_drawImage {
  fn drawImage(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawImage(const QRectF & r, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QRectF, &'a  QImage) {
  fn drawImage(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK6QRectFRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK6QRectFRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setClipping<T: QPainter_setClipping>(&mut self, value: T)  {
     value.setClipping(self);
    // return 1;
  }
}

pub trait QPainter_setClipping {
  fn setClipping(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setClipping(bool enable);
impl<'a> /*trait*/ QPainter_setClipping for (i8) {
  fn setClipping(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setClippingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QPainter11setClippingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setBrush<T: QPainter_setBrush>(&mut self, value: T)  {
     value.setBrush(self);
    // return 1;
  }
}

pub trait QPainter_setBrush {
  fn setBrush(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setBrush(const QBrush & brush);
impl<'a> /*trait*/ QPainter_setBrush for (&'a  QBrush) {
  fn setBrush(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8setBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8setBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setMatrix<T: QPainter_setMatrix>(&mut self, value: T)  {
     value.setMatrix(self);
    // return 1;
  }
}

pub trait QPainter_setMatrix {
  fn setMatrix(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setMatrix for (&'a  QMatrix, i8) {
  fn setMatrix(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN8QPainter9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawChord(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord for (&'a  QRect, i32, i32) {
  fn drawChord(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter9drawChordERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn eraseRect<T: QPainter_eraseRect>(&mut self, value: T)  {
     value.eraseRect(self);
    // return 1;
  }
}

pub trait QPainter_eraseRect {
  fn eraseRect(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::eraseRect(const QRectF & );
impl<'a> /*trait*/ QPainter_eraseRect for (&'a  QRectF) {
  fn eraseRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9eraseRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn translate<T: QPainter_translate>(&mut self, value: T)  {
     value.translate(self);
    // return 1;
  }
}

pub trait QPainter_translate {
  fn translate(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::translate(const QPoint & offset);
impl<'a> /*trait*/ QPainter_translate for (&'a  QPoint) {
  fn translate(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn viewTransformEnabled<T: QPainter_viewTransformEnabled>(&mut self, value: T) -> i8 {
    return value.viewTransformEnabled(self);
    // return 1;
  }
}

pub trait QPainter_viewTransformEnabled {
  fn viewTransformEnabled(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::viewTransformEnabled();
impl<'a> /*trait*/ QPainter_viewTransformEnabled for () {
  fn viewTransformEnabled(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter20viewTransformEnabledEv()};
    let mut ret = unsafe {_ZNK8QPainter20viewTransformEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setPen<T: QPainter_setPen>(&mut self, value: T)  {
     value.setPen(self);
    // return 1;
  }
}

pub trait QPainter_setPen {
  fn setPen(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setPen(const QPen & pen);
impl<'a> /*trait*/ QPainter_setPen for (&'a  QPen) {
  fn setPen(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter6setPenERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawLines<T: QPainter_drawLines>(&mut self, value: T)  {
     value.drawLines(self);
    // return 1;
  }
}

pub trait QPainter_drawLines {
  fn drawLines(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawLines(const QLineF * lines, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QLineF, i32) {
  fn drawLines(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK6QLineFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK6QLineFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setBrushOrigin<T: QPainter_setBrushOrigin>(&mut self, value: T)  {
     value.setBrushOrigin(self);
    // return 1;
  }
}

pub trait QPainter_setBrushOrigin {
  fn setBrushOrigin(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setBrushOrigin(int x, int y);
impl<'a> /*trait*/ QPainter_setBrushOrigin for (i32, i32) {
  fn setBrushOrigin(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter14setBrushOriginEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn worldTransform<T: QPainter_worldTransform>(&mut self, value: T) -> QTransform {
    return value.worldTransform(self);
    // return 1;
  }
}

pub trait QPainter_worldTransform {
  fn worldTransform(self, rsthis: &mut QPainter) -> QTransform;
}

// proto:  const QTransform & QPainter::worldTransform();
impl<'a> /*trait*/ QPainter_worldTransform for () {
  fn worldTransform(self, rsthis: &mut QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter14worldTransformEv()};
    let mut ret = unsafe {_ZNK8QPainter14worldTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawRects<T: QPainter_drawRects>(&mut self, value: T)  {
     value.drawRects(self);
    // return 1;
  }
}

pub trait QPainter_drawRects {
  fn drawRects(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawRects(const QRect * rects, int rectCount);
impl<'a> /*trait*/ QPainter_drawRects for (&'a  QRect, i32) {
  fn drawRects(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawRectsEPK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawEllipse<T: QPainter_drawEllipse>(&mut self, value: T)  {
     value.drawEllipse(self);
    // return 1;
  }
}

pub trait QPainter_drawEllipse {
  fn drawEllipse(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QPoint, i32, i32) {
  fn drawEllipse(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK6QPointii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter11drawEllipseERK6QPointii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawArc<T: QPainter_drawArc>(&mut self, value: T)  {
     value.drawArc(self);
    // return 1;
  }
}

pub trait QPainter_drawArc {
  fn drawArc(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc for (i32, i32, i32, i32, i32, i32) {
  fn drawArc(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn drawPolyline<T: QPainter_drawPolyline>(&mut self, value: T)  {
     value.drawPolyline(self);
    // return 1;
  }
}

pub trait QPainter_drawPolyline {
  fn drawPolyline(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPolyline(const QPolygonF & polyline);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPolygonF) {
  fn drawPolyline(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawPolylineERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn hasClipping<T: QPainter_hasClipping>(&mut self, value: T) -> i8 {
    return value.hasClipping(self);
    // return 1;
  }
}

pub trait QPainter_hasClipping {
  fn hasClipping(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::hasClipping();
impl<'a> /*trait*/ QPainter_hasClipping for () {
  fn hasClipping(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11hasClippingEv()};
    let mut ret = unsafe {_ZNK8QPainter11hasClippingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPixmap<T: QPainter_drawPixmap>(&mut self, value: T)  {
     value.drawPixmap(self);
    // return 1;
  }
}

pub trait QPainter_drawPixmap {
  fn drawPixmap(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QRectF, &'a  QPixmap, &'a  QRectF) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawStaticText<T: QPainter_drawStaticText>(&mut self, value: T)  {
     value.drawStaticText(self);
    // return 1;
  }
}

pub trait QPainter_drawStaticText {
  fn drawStaticText(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText for (i32, i32, &'a  QStaticText) {
  fn drawStaticText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextEiiRK11QStaticText()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14drawStaticTextEiiRK11QStaticText(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn strokePath<T: QPainter_strokePath>(&mut self, value: T)  {
     value.strokePath(self);
    // return 1;
  }
}

pub trait QPainter_strokePath {
  fn strokePath(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
impl<'a> /*trait*/ QPainter_strokePath for (&'a  QPainterPath, &'a  QPen) {
  fn strokePath(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10strokePathERK12QPainterPathRK4QPen()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10strokePathERK12QPainterPathRK4QPen(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm, int sx, int sy, int sw, int sh);
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, &'a  QPixmap, i32, i32, i32, i32) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawRects for (&'a  QRectF, i32) {
  fn drawRects(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawRectsEPK6QRectFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawRectsEPK6QRectFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawConvexPolygon<T: QPainter_drawConvexPolygon>(&mut self, value: T)  {
     value.drawConvexPolygon(self);
    // return 1;
  }
}

pub trait QPainter_drawConvexPolygon {
  fn drawConvexPolygon(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPoint, i32) {
  fn drawConvexPolygon(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter17drawConvexPolygonEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPath<T: QPainter_drawPath>(&mut self, value: T)  {
     value.drawPath(self);
    // return 1;
  }
}

pub trait QPainter_drawPath {
  fn drawPath(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainter_drawPath for (&'a  QPainterPath) {
  fn drawPath(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPixmap(int x, int y, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, &'a  QPixmap) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiRK7QPixmap()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapEiiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn combinedMatrix<T: QPainter_combinedMatrix>(&mut self, value: T) -> QMatrix {
    return value.combinedMatrix(self);
    // return 1;
  }
}

pub trait QPainter_combinedMatrix {
  fn combinedMatrix(self, rsthis: &mut QPainter) -> QMatrix;
}

// proto:  QMatrix QPainter::combinedMatrix();
impl<'a> /*trait*/ QPainter_combinedMatrix for () {
  fn combinedMatrix(self, rsthis: &mut QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter14combinedMatrixEv()};
    let mut ret = unsafe {_ZNK8QPainter14combinedMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setMatrixEnabled<T: QPainter_setMatrixEnabled>(&mut self, value: T)  {
     value.setMatrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_setMatrixEnabled {
  fn setMatrixEnabled(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setMatrixEnabled(bool enabled);
impl<'a> /*trait*/ QPainter_setMatrixEnabled for (i8) {
  fn setMatrixEnabled(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter16setMatrixEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QPainter16setMatrixEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPolyline(const QPolygon & polygon);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPolygon) {
  fn drawPolyline(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawPolylineERK8QPolygon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawTiledPixmap(const QRect & , const QPixmap & , const QPoint & );
impl<'a> /*trait*/ QPainter_drawTiledPixmap for (&'a  QRect, &'a  QPixmap, &'a  QPoint) {
  fn drawTiledPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setFont<T: QPainter_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QPainter_setFont {
  fn setFont(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setFont(const QFont & f);
impl<'a> /*trait*/ QPainter_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawChord(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord for (i32, i32, i32, i32, i32, i32) {
  fn drawChord(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, i32, i32, &'a  QPixmap) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn setWindow<T: QPainter_setWindow>(&mut self, value: T)  {
     value.setWindow(self);
    // return 1;
  }
}

pub trait QPainter_setWindow {
  fn setWindow(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setWindow(const QRect & window);
impl<'a> /*trait*/ QPainter_setWindow for (&'a  QRect) {
  fn setWindow(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setWindowERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9setWindowERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn deviceMatrix<T: QPainter_deviceMatrix>(&mut self, value: T) -> QMatrix {
    return value.deviceMatrix(self);
    // return 1;
  }
}

pub trait QPainter_deviceMatrix {
  fn deviceMatrix(self, rsthis: &mut QPainter) -> QMatrix;
}

// proto:  const QMatrix & QPainter::deviceMatrix();
impl<'a> /*trait*/ QPainter_deviceMatrix for () {
  fn deviceMatrix(self, rsthis: &mut QPainter) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter12deviceMatrixEv()};
    let mut ret = unsafe {_ZNK8QPainter12deviceMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawLines(const QPointF * pointPairs, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QPointF, i32) {
  fn drawLines(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPointF, &'a  QPixmap) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawLines(const QLine * lines, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QLine, i32) {
  fn drawLines(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK5QLinei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK5QLinei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPie<T: QPainter_drawPie>(&mut self, value: T)  {
     value.drawPie(self);
    // return 1;
  }
}

pub trait QPainter_drawPie {
  fn drawPie(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie for (i32, i32, i32, i32, i32, i32) {
  fn drawPie(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPoint, &'a  QPixmap, &'a  QRect) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawStaticText for (&'a  QPointF, &'a  QStaticText) {
  fn drawStaticText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWorldMatrixEnabled<T: QPainter_setWorldMatrixEnabled>(&mut self, value: T)  {
     value.setWorldMatrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_setWorldMatrixEnabled {
  fn setWorldMatrixEnabled(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setWorldMatrixEnabled(bool enabled);
impl<'a> /*trait*/ QPainter_setWorldMatrixEnabled for (i8) {
  fn setWorldMatrixEnabled(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter21setWorldMatrixEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QPainter21setWorldMatrixEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn NewQPainter<T: QPainter_NewQPainter>(value: T) -> QPainter {
    let rsthis = value.NewQPainter();
    return rsthis;
    // return 1;
  }
}

pub trait QPainter_NewQPainter {
  fn NewQPainter(self) -> QPainter;
}

// proto: void QPainter::NewQPainter(const QPainter & );
impl<'a> /*trait*/ QPainter_NewQPainter for (&'a  QPainter) {
  fn NewQPainter(self) -> QPainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainterC1ERKS_(qthis, arg0)};
    let rsthis = QPainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPoints<T: QPainter_drawPoints>(&mut self, value: T)  {
     value.drawPoints(self);
    // return 1;
  }
}

pub trait QPainter_drawPoints {
  fn drawPoints(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPoints(const QPolygon & points);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPolygon) {
  fn drawPoints(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPointsERK8QPolygon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPicture(const QPoint & p, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture for (&'a  QPoint, &'a  QPicture) {
  fn drawPicture(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureERK6QPointRK8QPicture()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawPictureERK6QPointRK8QPicture(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawRect<T: QPainter_drawRect>(&mut self, value: T)  {
     value.drawRect(self);
    // return 1;
  }
}

pub trait QPainter_drawRect {
  fn drawRect(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawRect(int x1, int y1, int w, int h);
impl<'a> /*trait*/ QPainter_drawRect for (i32, i32, i32, i32) {
  fn drawRect(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QRectF) {
  fn drawEllipse(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawRect(const QRectF & rect);
impl<'a> /*trait*/ QPainter_drawRect for (&'a  QRectF) {
  fn drawRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPoints(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPointF, i32) {
  fn drawPoints(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter10drawPointsEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn clipRegion<T: QPainter_clipRegion>(&mut self, value: T) -> QRegion {
    return value.clipRegion(self);
    // return 1;
  }
}

pub trait QPainter_clipRegion {
  fn clipRegion(self, rsthis: &mut QPainter) -> QRegion;
}

// proto:  QRegion QPainter::clipRegion();
impl<'a> /*trait*/ QPainter_clipRegion for () {
  fn clipRegion(self, rsthis: &mut QPainter) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter10clipRegionEv()};
    let mut ret = unsafe {_ZNK8QPainter10clipRegionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawText(const QRectF & r, int flags, const QString & text, QRectF * br);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QRectF, i32, &'a  QString, &'a mut QRectF) {
  fn drawText(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn drawLine<T: QPainter_drawLine>(&mut self, value: T)  {
     value.drawLine(self);
    // return 1;
  }
}

pub trait QPainter_drawLine {
  fn drawLine(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawLine(const QLineF & line);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QLineF) {
  fn drawLine(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK6QLineF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK6QLineF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawLine(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QPointF, &'a  QPointF) {
  fn drawLine(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPixmap(const QRect & r, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QRect, &'a  QPixmap) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap & , int sx, int sy);
impl<'a> /*trait*/ QPainter_drawTiledPixmap for (i32, i32, i32, i32, &'a  QPixmap, i32, i32) {
  fn drawTiledPixmap(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn device<T: QPainter_device>(&mut self, value: T) -> QPaintDevice {
    return value.device(self);
    // return 1;
  }
}

pub trait QPainter_device {
  fn device(self, rsthis: &mut QPainter) -> QPaintDevice;
}

// proto:  QPaintDevice * QPainter::device();
impl<'a> /*trait*/ QPainter_device for () {
  fn device(self, rsthis: &mut QPainter) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6deviceEv()};
    let mut ret = unsafe {_ZNK8QPainter6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setViewport<T: QPainter_setViewport>(&mut self, value: T)  {
     value.setViewport(self);
    // return 1;
  }
}

pub trait QPainter_setViewport {
  fn setViewport(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setViewport(const QRect & viewport);
impl<'a> /*trait*/ QPainter_setViewport for (&'a  QRect) {
  fn setViewport(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setViewportERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11setViewportERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::fillRect(const QRect & , const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRect, &'a  QColor) {
  fn fillRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK5QRectRK6QColor()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK5QRectRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::setBrushOrigin(const QPointF & );
impl<'a> /*trait*/ QPainter_setBrushOrigin for (&'a  QPointF) {
  fn setBrushOrigin(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14setBrushOriginERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawTextItem<T: QPainter_drawTextItem>(&mut self, value: T)  {
     value.drawTextItem(self);
    // return 1;
  }
}

pub trait QPainter_drawTextItem {
  fn drawTextItem(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem for (i32, i32, &'a  QTextItem) {
  fn drawTextItem(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemEiiRK9QTextItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawTextItemEiiRK9QTextItem(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto: void QPainter::NewQPainter(QPaintDevice * );
impl<'a> /*trait*/ QPainter_NewQPainter for (&'a mut QPaintDevice) {
  fn NewQPainter(self) -> QPainter {
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
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, i32, i32, &'a  QPixmap, i32, i32, i32, i32) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QPoint, &'a  QImage) {
  fn drawImage(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK6QPointRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK6QPointRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPie(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie for (&'a  QRect, i32, i32) {
  fn drawPie(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawTextItem for (&'a  QPoint, &'a  QTextItem) {
  fn drawTextItem(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawLines(const QPoint * pointPairs, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QPoint, i32) {
  fn drawLines(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawLinesEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPicture(int x, int y, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture for (i32, i32, &'a  QPicture) {
  fn drawPicture(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureEiiRK8QPicture()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawPictureEiiRK8QPicture(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn save<T: QPainter_save>(&mut self, value: T)  {
     value.save(self);
    // return 1;
  }
}

pub trait QPainter_save {
  fn save(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::save();
impl<'a> /*trait*/ QPainter_save for () {
  fn save(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter4saveEv()};
     unsafe {_ZN8QPainter4saveEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPainter::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainter_translate for (f64, f64) {
  fn translate(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN8QPainter9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn combinedTransform<T: QPainter_combinedTransform>(&mut self, value: T) -> QTransform {
    return value.combinedTransform(self);
    // return 1;
  }
}

pub trait QPainter_combinedTransform {
  fn combinedTransform(self, rsthis: &mut QPainter) -> QTransform;
}

// proto:  QTransform QPainter::combinedTransform();
impl<'a> /*trait*/ QPainter_combinedTransform for () {
  fn combinedTransform(self, rsthis: &mut QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter17combinedTransformEv()};
    let mut ret = unsafe {_ZNK8QPainter17combinedTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn end<T: QPainter_end>(&mut self, value: T) -> i8 {
    return value.end(self);
    // return 1;
  }
}

pub trait QPainter_end {
  fn end(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::end();
impl<'a> /*trait*/ QPainter_end for () {
  fn end(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter3endEv()};
    let mut ret = unsafe {_ZN8QPainter3endEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainter::setViewport(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_setViewport for (i32, i32, i32, i32) {
  fn setViewport(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn drawRoundRect<T: QPainter_drawRoundRect>(&mut self, value: T)  {
     value.drawRoundRect(self);
    // return 1;
  }
}

pub trait QPainter_drawRoundRect {
  fn drawRoundRect(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
impl<'a> /*trait*/ QPainter_drawRoundRect for (&'a  QRect, i32, i32) {
  fn drawRoundRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter13drawRoundRectERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWorldTransform<T: QPainter_setWorldTransform>(&mut self, value: T)  {
     value.setWorldTransform(self);
    // return 1;
  }
}

pub trait QPainter_setWorldTransform {
  fn setWorldTransform(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setWorldTransform for (&'a  QTransform, i8) {
  fn setWorldTransform(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17setWorldTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN8QPainter17setWorldTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPoints(const QPolygonF & points);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPolygonF) {
  fn drawPoints(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPointsERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn restore<T: QPainter_restore>(&mut self, value: T)  {
     value.restore(self);
    // return 1;
  }
}

pub trait QPainter_restore {
  fn restore(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::restore();
impl<'a> /*trait*/ QPainter_restore for () {
  fn restore(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7restoreEv()};
     unsafe {_ZN8QPainter7restoreEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPainter::drawStaticText(const QPoint & topLeftPosition, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText for (&'a  QPoint, &'a  QStaticText) {
  fn drawStaticText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::fillRect(int x, int y, int w, int h, const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect for (i32, i32, i32, i32, &'a  QBrush) {
  fn fillRect(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawRoundRect for (&'a  QRectF, i32, i32) {
  fn drawRoundRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter13drawRoundRectERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPoint<T: QPainter_drawPoint>(&mut self, value: T)  {
     value.drawPoint(self);
    // return 1;
  }
}

pub trait QPainter_drawPoint {
  fn drawPoint(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawPoint(const QPoint & p);
impl<'a> /*trait*/ QPainter_drawPoint for (&'a  QPoint) {
  fn drawPoint(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawPointERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn redirected<T: QPainter_redirected>(&mut self, value: T) -> QPaintDevice {
    return value.redirected(self);
    // return 1;
  }
}

pub trait QPainter_redirected {
  fn redirected(self, rsthis: &mut QPainter) -> QPaintDevice;
}

// proto: static QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
impl<'a> /*trait*/ QPainter_redirected for (&'a  QPaintDevice, &'a mut QPoint) {
  fn redirected(self, rsthis: &mut QPainter) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint(arg0, arg1)};
    let mut ret1 = QPaintDevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn shear<T: QPainter_shear>(&mut self, value: T)  {
     value.shear(self);
    // return 1;
  }
}

pub trait QPainter_shear {
  fn shear(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QPainter_shear for (f64, f64) {
  fn shear(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN8QPainter5shearEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawText(const QRect & r, int flags, const QString & text, QRect * br);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QRect, i32, &'a  QString, &'a mut QRect) {
  fn drawText(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn font<T: QPainter_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QPainter_font {
  fn font(self, rsthis: &mut QPainter) -> QFont;
}

// proto:  const QFont & QPainter::font();
impl<'a> /*trait*/ QPainter_font for () {
  fn font(self, rsthis: &mut QPainter) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter4fontEv()};
    let mut ret = unsafe {_ZNK8QPainter4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn deviceTransform<T: QPainter_deviceTransform>(&mut self, value: T) -> QTransform {
    return value.deviceTransform(self);
    // return 1;
  }
}

pub trait QPainter_deviceTransform {
  fn deviceTransform(self, rsthis: &mut QPainter) -> QTransform;
}

// proto:  const QTransform & QPainter::deviceTransform();
impl<'a> /*trait*/ QPainter_deviceTransform for () {
  fn deviceTransform(self, rsthis: &mut QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter15deviceTransformEv()};
    let mut ret = unsafe {_ZNK8QPainter15deviceTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::eraseRect(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_eraseRect for (i32, i32, i32, i32) {
  fn eraseRect(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn resetMatrix<T: QPainter_resetMatrix>(&mut self, value: T)  {
     value.resetMatrix(self);
    // return 1;
  }
}

pub trait QPainter_resetMatrix {
  fn resetMatrix(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::resetMatrix();
impl<'a> /*trait*/ QPainter_resetMatrix for () {
  fn resetMatrix(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11resetMatrixEv()};
     unsafe {_ZN8QPainter11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPainter::drawPolyline(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPoint, i32) {
  fn drawPolyline(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter12drawPolylineEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn paintEngine<T: QPainter_paintEngine>(&mut self, value: T) -> QPaintEngine {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QPainter_paintEngine {
  fn paintEngine(self, rsthis: &mut QPainter) -> QPaintEngine;
}

// proto:  QPaintEngine * QPainter::paintEngine();
impl<'a> /*trait*/ QPainter_paintEngine for () {
  fn paintEngine(self, rsthis: &mut QPainter) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11paintEngineEv()};
    let mut ret = unsafe {_ZNK8QPainter11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawEllipse(const QRect & r);
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QRect) {
  fn drawEllipse(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter11drawEllipseERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawLine(const QLine & line);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QLine) {
  fn drawLine(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK5QLine(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn isActive<T: QPainter_isActive>(&mut self, value: T) -> i8 {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QPainter_isActive {
  fn isActive(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::isActive();
impl<'a> /*trait*/ QPainter_isActive for () {
  fn isActive(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8isActiveEv()};
    let mut ret = unsafe {_ZNK8QPainter8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainter::drawArc(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc for (&'a  QRectF, i32, i32) {
  fn drawArc(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter7drawArcERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn restoreRedirected<T: QPainter_restoreRedirected>(&mut self, value: T)  {
     value.restoreRedirected(self);
    // return 1;
  }
}

pub trait QPainter_restoreRedirected {
  fn restoreRedirected(self, rsthis: &mut QPainter) ;
}

// proto: static void QPainter::restoreRedirected(const QPaintDevice * device);
impl<'a> /*trait*/ QPainter_restoreRedirected for (&'a  QPaintDevice) {
  fn restoreRedirected(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17restoreRedirectedEPK12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter17restoreRedirectedEPK12QPaintDevice(arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm, const QRectF & sr);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPointF, &'a  QPixmap, &'a  QRectF) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QPointF, f64, f64) {
  fn drawEllipse(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPointF, i32) {
  fn drawConvexPolygon(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter17drawConvexPolygonEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::setBrushOrigin(const QPoint & );
impl<'a> /*trait*/ QPainter_setBrushOrigin for (&'a  QPoint) {
  fn setBrushOrigin(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter14setBrushOriginERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawText(const QRectF & r, const QString & text, const QTextOption & o);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QRectF, &'a  QString, &'a  QTextOption) {
  fn drawText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn worldMatrixEnabled<T: QPainter_worldMatrixEnabled>(&mut self, value: T) -> i8 {
    return value.worldMatrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_worldMatrixEnabled {
  fn worldMatrixEnabled(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::worldMatrixEnabled();
impl<'a> /*trait*/ QPainter_worldMatrixEnabled for () {
  fn worldMatrixEnabled(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter18worldMatrixEnabledEv()};
    let mut ret = unsafe {_ZNK8QPainter18worldMatrixEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPoint, &'a  QPixmap) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QPainter_drawLine for (i32, i32, i32, i32) {
  fn drawLine(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawPoint for (i32, i32) {
  fn drawPoint(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter9drawPointEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn transform<T: QPainter_transform>(&mut self, value: T) -> QTransform {
    return value.transform(self);
    // return 1;
  }
}

pub trait QPainter_transform {
  fn transform(self, rsthis: &mut QPainter) -> QTransform;
}

// proto:  const QTransform & QPainter::transform();
impl<'a> /*trait*/ QPainter_transform for () {
  fn transform(self, rsthis: &mut QPainter) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter9transformEv()};
    let mut ret = unsafe {_ZNK8QPainter9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setRedirected<T: QPainter_setRedirected>(&mut self, value: T)  {
     value.setRedirected(self);
    // return 1;
  }
}

pub trait QPainter_setRedirected {
  fn setRedirected(self, rsthis: &mut QPainter) ;
}

// proto: static void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
impl<'a> /*trait*/ QPainter_setRedirected for (&'a  QPaintDevice, &'a mut QPaintDevice, &'a  QPoint) {
  fn setRedirected(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QRect, &'a  QPixmap, &'a  QRect) {
  fn drawPixmap(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fontMetrics<T: QPainter_fontMetrics>(&mut self, value: T) -> QFontMetrics {
    return value.fontMetrics(self);
    // return 1;
  }
}

pub trait QPainter_fontMetrics {
  fn fontMetrics(self, rsthis: &mut QPainter) -> QFontMetrics;
}

// proto:  QFontMetrics QPainter::fontMetrics();
impl<'a> /*trait*/ QPainter_fontMetrics for () {
  fn fontMetrics(self, rsthis: &mut QPainter) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11fontMetricsEv()};
    let mut ret = unsafe {_ZNK8QPainter11fontMetricsEv(rsthis.qclsinst)};
    let mut ret1 = QFontMetrics{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawGlyphRun<T: QPainter_drawGlyphRun>(&mut self, value: T)  {
     value.drawGlyphRun(self);
    // return 1;
  }
}

pub trait QPainter_drawGlyphRun {
  fn drawGlyphRun(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
impl<'a> /*trait*/ QPainter_drawGlyphRun for (&'a  QPointF, &'a  QGlyphRun) {
  fn drawGlyphRun(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::fillRect(const QRectF & , const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRectF, &'a  QBrush) {
  fn fillRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK6QRectFRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK6QRectFRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn resetTransform<T: QPainter_resetTransform>(&mut self, value: T)  {
     value.resetTransform(self);
    // return 1;
  }
}

pub trait QPainter_resetTransform {
  fn resetTransform(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::resetTransform();
impl<'a> /*trait*/ QPainter_resetTransform for () {
  fn resetTransform(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14resetTransformEv()};
     unsafe {_ZN8QPainter14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPainter::fillRect(const QRect & , const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRect, &'a  QBrush) {
  fn fillRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK5QRectRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK5QRectRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn brush<T: QPainter_brush>(&mut self, value: T) -> QBrush {
    return value.brush(self);
    // return 1;
  }
}

pub trait QPainter_brush {
  fn brush(self, rsthis: &mut QPainter) -> QBrush;
}

// proto:  const QBrush & QPainter::brush();
impl<'a> /*trait*/ QPainter_brush for () {
  fn brush(self, rsthis: &mut QPainter) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter5brushEv()};
    let mut ret = unsafe {_ZNK8QPainter5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn FreeQPainter<T: QPainter_FreeQPainter>(&mut self, value: T)  {
     value.FreeQPainter(self);
    // return 1;
  }
}

pub trait QPainter_FreeQPainter {
  fn FreeQPainter(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::FreeQPainter();
impl<'a> /*trait*/ QPainter_FreeQPainter for () {
  fn FreeQPainter(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterD0Ev()};
     unsafe {_ZN8QPainterD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn begin<T: QPainter_begin>(&mut self, value: T) -> i8 {
    return value.begin(self);
    // return 1;
  }
}

pub trait QPainter_begin {
  fn begin(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::begin(QPaintDevice * );
impl<'a> /*trait*/ QPainter_begin for (&'a mut QPaintDevice) {
  fn begin(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5beginEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QPainter5beginEP12QPaintDevice(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainter::drawRect(const QRect & rect);
impl<'a> /*trait*/ QPainter_drawRect for (&'a  QRect) {
  fn drawRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawTextItem(const QPointF & p, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem for (&'a  QPointF, &'a  QTextItem) {
  fn drawTextItem(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn scale<T: QPainter_scale>(&mut self, value: T)  {
     value.scale(self);
    // return 1;
  }
}

pub trait QPainter_scale {
  fn scale(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QPainter_scale for (f64, f64) {
  fn scale(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN8QPainter5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWorldMatrix<T: QPainter_setWorldMatrix>(&mut self, value: T)  {
     value.setWorldMatrix(self);
    // return 1;
  }
}

pub trait QPainter_setWorldMatrix {
  fn setWorldMatrix(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setWorldMatrix for (&'a  QMatrix, i8) {
  fn setWorldMatrix(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setWorldMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN8QPainter14setWorldMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn clipPath<T: QPainter_clipPath>(&mut self, value: T) -> QPainterPath {
    return value.clipPath(self);
    // return 1;
  }
}

pub trait QPainter_clipPath {
  fn clipPath(self, rsthis: &mut QPainter) -> QPainterPath;
}

// proto:  QPainterPath QPainter::clipPath();
impl<'a> /*trait*/ QPainter_clipPath for () {
  fn clipPath(self, rsthis: &mut QPainter) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8clipPathEv()};
    let mut ret = unsafe {_ZNK8QPainter8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn brushOrigin<T: QPainter_brushOrigin>(&mut self, value: T) -> QPoint {
    return value.brushOrigin(self);
    // return 1;
  }
}

pub trait QPainter_brushOrigin {
  fn brushOrigin(self, rsthis: &mut QPainter) -> QPoint;
}

// proto:  QPoint QPainter::brushOrigin();
impl<'a> /*trait*/ QPainter_brushOrigin for () {
  fn brushOrigin(self, rsthis: &mut QPainter) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11brushOriginEv()};
    let mut ret = unsafe {_ZNK8QPainter11brushOriginEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawConvexPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPolygonF) {
  fn drawConvexPolygon(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter17drawConvexPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawEllipse(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_drawEllipse for (i32, i32, i32, i32) {
  fn drawEllipse(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPolygon) {
  fn drawConvexPolygon(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter17drawConvexPolygonERK8QPolygon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPoints(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPoint, i32) {
  fn drawPoints(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter10drawPointsEPK6QPointi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn background<T: QPainter_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QPainter_background {
  fn background(self, rsthis: &mut QPainter) -> QBrush;
}

// proto:  const QBrush & QPainter::background();
impl<'a> /*trait*/ QPainter_background for () {
  fn background(self, rsthis: &mut QPainter) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter10backgroundEv()};
    let mut ret = unsafe {_ZNK8QPainter10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawRoundRect(int x, int y, int w, int h, int , int );
impl<'a> /*trait*/ QPainter_drawRoundRect for (i32, i32, i32, i32, i32, i32) {
  fn drawRoundRect(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn viewport<T: QPainter_viewport>(&mut self, value: T) -> QRect {
    return value.viewport(self);
    // return 1;
  }
}

pub trait QPainter_viewport {
  fn viewport(self, rsthis: &mut QPainter) -> QRect;
}

// proto:  QRect QPainter::viewport();
impl<'a> /*trait*/ QPainter_viewport for () {
  fn viewport(self, rsthis: &mut QPainter) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8viewportEv()};
    let mut ret = unsafe {_ZNK8QPainter8viewportEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawArc(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc for (&'a  QRect, i32, i32) {
  fn drawArc(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcERK5QRectii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPainter7drawArcERK5QRectii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fillPath<T: QPainter_fillPath>(&mut self, value: T)  {
     value.fillPath(self);
    // return 1;
  }
}

pub trait QPainter_fillPath {
  fn fillPath(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
impl<'a> /*trait*/ QPainter_fillPath for (&'a  QPainterPath, &'a  QBrush) {
  fn fillPath(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawText(int x, int y, int w, int h, int flags, const QString & text, QRect * br);
impl<'a> /*trait*/ QPainter_drawText for (i32, i32, i32, i32, i32, &'a  QString, &'a mut QRect) {
  fn drawText(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn matrixEnabled<T: QPainter_matrixEnabled>(&mut self, value: T) -> i8 {
    return value.matrixEnabled(self);
    // return 1;
  }
}

pub trait QPainter_matrixEnabled {
  fn matrixEnabled(self, rsthis: &mut QPainter) -> i8;
}

// proto:  bool QPainter::matrixEnabled();
impl<'a> /*trait*/ QPainter_matrixEnabled for () {
  fn matrixEnabled(self, rsthis: &mut QPainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter13matrixEnabledEv()};
    let mut ret = unsafe {_ZNK8QPainter13matrixEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QPainter::drawPolyline(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPointF, i32) {
  fn drawPolyline(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPainter12drawPolylineEPK7QPointFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setTransform<T: QPainter_setTransform>(&mut self, value: T)  {
     value.setTransform(self);
    // return 1;
  }
}

pub trait QPainter_setTransform {
  fn setTransform(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setTransform(const QTransform & transform, bool combine);
impl<'a> /*trait*/ QPainter_setTransform for (&'a  QTransform, i8) {
  fn setTransform(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN8QPainter12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::setPen(const QColor & color);
impl<'a> /*trait*/ QPainter_setPen for (&'a  QColor) {
  fn setPen(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6setPenERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter6setPenERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::eraseRect(const QRect & );
impl<'a> /*trait*/ QPainter_eraseRect for (&'a  QRect) {
  fn eraseRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9eraseRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn window<T: QPainter_window>(&mut self, value: T) -> QRect {
    return value.window(self);
    // return 1;
  }
}

pub trait QPainter_window {
  fn window(self, rsthis: &mut QPainter) -> QRect;
}

// proto:  QRect QPainter::window();
impl<'a> /*trait*/ QPainter_window for () {
  fn window(self, rsthis: &mut QPainter) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6windowEv()};
    let mut ret = unsafe {_ZNK8QPainter6windowEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawImage(const QRect & r, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QRect, &'a  QImage) {
  fn drawImage(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK5QRectRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK5QRectRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn initFrom<T: QPainter_initFrom>(&mut self, value: T)  {
     value.initFrom(self);
    // return 1;
  }
}

pub trait QPainter_initFrom {
  fn initFrom(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::initFrom(const QPaintDevice * device);
impl<'a> /*trait*/ QPainter_initFrom for (&'a  QPaintDevice) {
  fn initFrom(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8initFromEPK12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8initFromEPK12QPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fontInfo<T: QPainter_fontInfo>(&mut self, value: T) -> QFontInfo {
    return value.fontInfo(self);
    // return 1;
  }
}

pub trait QPainter_fontInfo {
  fn fontInfo(self, rsthis: &mut QPainter) -> QFontInfo;
}

// proto:  QFontInfo QPainter::fontInfo();
impl<'a> /*trait*/ QPainter_fontInfo for () {
  fn fontInfo(self, rsthis: &mut QPainter) -> QFontInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8fontInfoEv()};
    let mut ret = unsafe {_ZNK8QPainter8fontInfoEv(rsthis.qclsinst)};
    let mut ret1 = QFontInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn endNativePainting<T: QPainter_endNativePainting>(&mut self, value: T)  {
     value.endNativePainting(self);
    // return 1;
  }
}

pub trait QPainter_endNativePainting {
  fn endNativePainting(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::endNativePainting();
impl<'a> /*trait*/ QPainter_endNativePainting for () {
  fn endNativePainting(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17endNativePaintingEv()};
     unsafe {_ZN8QPainter17endNativePaintingEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setViewTransformEnabled<T: QPainter_setViewTransformEnabled>(&mut self, value: T)  {
     value.setViewTransformEnabled(self);
    // return 1;
  }
}

pub trait QPainter_setViewTransformEnabled {
  fn setViewTransformEnabled(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setViewTransformEnabled(bool enable);
impl<'a> /*trait*/ QPainter_setViewTransformEnabled for (i8) {
  fn setViewTransformEnabled(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter23setViewTransformEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QPainter23setViewTransformEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawPoint(const QPointF & pt);
impl<'a> /*trait*/ QPainter_drawPoint for (&'a  QPointF) {
  fn drawPoint(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setOpacity<T: QPainter_setOpacity>(&mut self, value: T)  {
     value.setOpacity(self);
    // return 1;
  }
}

pub trait QPainter_setOpacity {
  fn setOpacity(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::setOpacity(qreal opacity);
impl<'a> /*trait*/ QPainter_setOpacity for (f64) {
  fn setOpacity(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN8QPainter10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::fillRect(const QRectF & , const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRectF, &'a  QColor) {
  fn fillRect(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK6QRectFRK6QColor()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8fillRectERK6QRectFRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QPainter::NewQPainter();
impl<'a> /*trait*/ QPainter_NewQPainter for () {
  fn NewQPainter(self) -> QPainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterC1Ev()};
    unsafe {_ZN8QPainterC1Ev(qthis)};
    let rsthis = QPainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QPainter::translate(const QPointF & offset);
impl<'a> /*trait*/ QPainter_translate for (&'a  QPointF) {
  fn translate(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QPainter::drawText(const QPointF & p, const QString & s);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QPointF, &'a  QString) {
  fn drawText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK7QPointFRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK7QPointFRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawImage(const QPointF & p, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QPointF, &'a  QImage) {
  fn drawImage(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK7QPointFRK6QImage()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter9drawImageERK7QPointFRK6QImage(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn pen<T: QPainter_pen>(&mut self, value: T) -> QPen {
    return value.pen(self);
    // return 1;
  }
}

pub trait QPainter_pen {
  fn pen(self, rsthis: &mut QPainter) -> QPen;
}

// proto:  const QPen & QPainter::pen();
impl<'a> /*trait*/ QPainter_pen for () {
  fn pen(self, rsthis: &mut QPainter) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter3penEv()};
    let mut ret = unsafe {_ZNK8QPainter3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn rotate<T: QPainter_rotate>(&mut self, value: T)  {
     value.rotate(self);
    // return 1;
  }
}

pub trait QPainter_rotate {
  fn rotate(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::rotate(qreal a);
impl<'a> /*trait*/ QPainter_rotate for (f64) {
  fn rotate(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6rotateEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN8QPainter6rotateEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn clipBoundingRect<T: QPainter_clipBoundingRect>(&mut self, value: T) -> QRectF {
    return value.clipBoundingRect(self);
    // return 1;
  }
}

pub trait QPainter_clipBoundingRect {
  fn clipBoundingRect(self, rsthis: &mut QPainter) -> QRectF;
}

// proto:  QRectF QPainter::clipBoundingRect();
impl<'a> /*trait*/ QPainter_clipBoundingRect for () {
  fn clipBoundingRect(self, rsthis: &mut QPainter) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter16clipBoundingRectEv()};
    let mut ret = unsafe {_ZNK8QPainter16clipBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPainter::drawLine(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QPoint, &'a  QPoint) {
  fn drawLine(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawLineERK6QPointS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::drawPie(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie for (&'a  QRectF, i32, i32) {
  fn drawPie(self, rsthis: &mut QPainter)  {
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
impl<'a> /*trait*/ QPainter_drawText for (&'a  QPoint, &'a  QString) {
  fn drawText(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QPointRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPainter8drawTextERK6QPointRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPainter::setWindow(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_setWindow for (i32, i32, i32, i32) {
  fn setWindow(self, rsthis: &mut QPainter)  {
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

impl /*struct*/ QPainter {
  pub fn beginNativePainting<T: QPainter_beginNativePainting>(&mut self, value: T)  {
     value.beginNativePainting(self);
    // return 1;
  }
}

pub trait QPainter_beginNativePainting {
  fn beginNativePainting(self, rsthis: &mut QPainter) ;
}

// proto:  void QPainter::beginNativePainting();
impl<'a> /*trait*/ QPainter_beginNativePainting for () {
  fn beginNativePainting(self, rsthis: &mut QPainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter19beginNativePaintingEv()};
     unsafe {_ZN8QPainter19beginNativePaintingEv(rsthis.qclsinst)};
    // return 1;
  }
}

