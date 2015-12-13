// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qstring::QString;
use super::qtextoption::QTextOption;
use super::qpointf::QPointF;
use super::qpicture::QPicture;
use super::qcolor::QColor;
use super::qpixmap::QPixmap;
use super::qbrush::QBrush;
use super::qrect::QRect;
use super::qimage::QImage;
use super::qmatrix::QMatrix;
use super::qpoint::QPoint;
use super::qpen::QPen;
use super::qlinef::QLineF;
use super::qpolygonf::QPolygonF;
use super::qstatictext::QStaticText;
use super::qpainterpath::QPainterPath;
use super::qpolygon::QPolygon;
use super::qfont::QFont;
use super::qline::QLine;
use super::qtextitem::QTextItem;
use super::qpaintdevice::QPaintDevice;
use super::qtransform::QTransform;
use super::qglyphrun::QGlyphRun;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QRectF QPainter::boundingRect(const QRectF & rect, const QString & text, const QTextOption & o);
  fn _ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
  fn _ZN8QPainter11drawPictureERK7QPointFRK8QPicture(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: const QMatrix & QPainter::worldMatrix();
  fn _ZNK8QPainter11worldMatrixEv() -> i32;
  // proto: void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
  fn _ZN8QPainter8drawTextERK7QPointFRK7QStringii(arg0: *const c_void, arg1: *const c_void, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
  fn _ZN8QPainter8fillRectEiiiiRK6QColor(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *const c_void) -> i32;
  // proto: const QMatrix & QPainter::matrix();
  fn _ZNK8QPainter6matrixEv() -> i32;
  // proto: double QPainter::opacity();
  fn _ZNK8QPainter7opacityEv() -> i32;
  // proto: void QPainter::drawText(int x, int y, const QString & s);
  fn _ZN8QPainter8drawTextEiiRK7QString(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
  fn _ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::setBackground(const QBrush & bg);
  fn _ZN8QPainter13setBackgroundERK6QBrush(arg0: *const c_void) -> i32;
  // proto: QRect QPainter::boundingRect(const QRect & rect, int flags, const QString & text);
  fn _ZN8QPainter12boundingRectERK5QRectiRK7QString(arg0: *const c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawChord(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter9drawChordERK6QRectFii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::drawImage(const QRectF & r, const QImage & image);
  fn _ZN8QPainter9drawImageERK6QRectFRK6QImage(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::setClipping(bool enable);
  fn _ZN8QPainter11setClippingEb(arg0: int8_t) -> i32;
  // proto: void QPainter::setBrush(const QBrush & brush);
  fn _ZN8QPainter8setBrushERK6QBrush(arg0: *const c_void) -> i32;
  // proto: void QPainter::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN8QPainter9setMatrixERK7QMatrixb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QPainter::drawChord(const QRect & , int a, int alen);
  fn _ZN8QPainter9drawChordERK5QRectii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::eraseRect(const QRectF & );
  fn _ZN8QPainter9eraseRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QPainter::translate(const QPoint & offset);
  fn _ZN8QPainter9translateERK6QPoint(arg0: *const c_void) -> i32;
  // proto: bool QPainter::viewTransformEnabled();
  fn _ZNK8QPainter20viewTransformEnabledEv() -> i32;
  // proto: void QPainter::setPen(const QPen & pen);
  fn _ZN8QPainter6setPenERK4QPen(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawLines(const QLineF * lines, int lineCount);
  fn _ZN8QPainter9drawLinesEPK6QLineFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::setBrushOrigin(int x, int y);
  fn _ZN8QPainter14setBrushOriginEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QTransform & QPainter::worldTransform();
  fn _ZNK8QPainter14worldTransformEv() -> i32;
  // proto: void QPainter::drawRects(const QRect * rects, int rectCount);
  fn _ZN8QPainter9drawRectsEPK5QRecti(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
  fn _ZN8QPainter11drawEllipseERK6QPointii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter7drawArcEiiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> i32;
  // proto: void QPainter::drawPolyline(const QPolygonF & polyline);
  fn _ZN8QPainter12drawPolylineERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: bool QPainter::hasClipping();
  fn _ZNK8QPainter11hasClippingEv() -> i32;
  // proto: void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
  fn _ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextEiiRK11QStaticText(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
  fn _ZN8QPainter10strokePathERK12QPainterPathRK4QPen(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawPixmap(int x, int y, const QPixmap & pm, int sx, int sy, int sw, int sh);
  fn _ZN8QPainter10drawPixmapEiiRK7QPixmapiiii(arg0: c_int, arg1: c_int, arg2: *const c_void, arg3: c_int, arg4: c_int, arg5: c_int, arg6: c_int) -> i32;
  // proto: void QPainter::drawRects(const QRectF * rects, int rectCount);
  fn _ZN8QPainter9drawRectsEPK6QRectFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
  fn _ZN8QPainter17drawConvexPolygonEPK6QPointi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::drawPath(const QPainterPath & path);
  fn _ZN8QPainter8drawPathERK12QPainterPath(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawPixmap(int x, int y, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapEiiRK7QPixmap(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: QMatrix QPainter::combinedMatrix();
  fn _ZNK8QPainter14combinedMatrixEv() -> i32;
  // proto: void QPainter::setMatrixEnabled(bool enabled);
  fn _ZN8QPainter16setMatrixEnabledEb(arg0: int8_t) -> i32;
  // proto: void QPainter::drawPolyline(const QPolygon & polygon);
  fn _ZN8QPainter12drawPolylineERK8QPolygon(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawTiledPixmap(const QRect & , const QPixmap & , const QPoint & );
  fn _ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::setFont(const QFont & f);
  fn _ZN8QPainter7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawChord(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter9drawChordEiiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> i32;
  // proto: void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapEiiiiRK7QPixmap(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *const c_void) -> i32;
  // proto: void QPainter::setWindow(const QRect & window);
  fn _ZN8QPainter9setWindowERK5QRect(arg0: *const c_void) -> i32;
  // proto: const QMatrix & QPainter::deviceMatrix();
  fn _ZNK8QPainter12deviceMatrixEv() -> i32;
  // proto: void QPainter::drawLines(const QPointF * pointPairs, int lineCount);
  fn _ZN8QPainter9drawLinesEPK7QPointFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QRect QPainter::boundingRect(int x, int y, int w, int h, int flags, const QString & text);
  fn _ZN8QPainter12boundingRectEiiiiiRK7QString(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *const c_void) -> i32;
  // proto: void QPainter::drawLines(const QLine * lines, int lineCount);
  fn _ZN8QPainter9drawLinesEPK5QLinei(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
  fn _ZN8QPainter7drawPieEiiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> i32;
  // proto: void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm, const QRect & sr);
  fn _ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawStaticText(const QPointF & topLeftPosition, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::setWorldMatrixEnabled(bool enabled);
  fn _ZN8QPainter21setWorldMatrixEnabledEb(arg0: int8_t) -> i32;
  // proto: void QPainter::NewQPainter(const QPainter & );
  fn _ZN8QPainterC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QPainter::drawPoints(const QPolygon & points);
  fn _ZN8QPainter10drawPointsERK8QPolygon(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawPicture(const QPoint & p, const QPicture & picture);
  fn _ZN8QPainter11drawPictureERK6QPointRK8QPicture(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawRect(int x1, int y1, int w, int h);
  fn _ZN8QPainter8drawRectEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::drawEllipse(const QRectF & r);
  fn _ZN8QPainter11drawEllipseERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawRect(const QRectF & rect);
  fn _ZN8QPainter8drawRectERK6QRectF(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawPoints(const QPointF * points, int pointCount);
  fn _ZN8QPainter10drawPointsEPK7QPointFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QRegion QPainter::clipRegion();
  fn _ZNK8QPainter10clipRegionEv() -> i32;
  // proto: void QPainter::drawText(const QRectF & r, int flags, const QString & text, QRectF * br);
  fn _ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_(arg0: *const c_void, arg1: c_int, arg2: *const c_void, arg3: *mut c_void) -> i32;
  // proto: void QPainter::drawLine(const QLineF & line);
  fn _ZN8QPainter8drawLineERK6QLineF(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawLine(const QPointF & p1, const QPointF & p2);
  fn _ZN8QPainter8drawLineERK7QPointFS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawPixmap(const QRect & r, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK5QRectRK7QPixmap(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap & , int sx, int sy);
  fn _ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *const c_void, arg5: c_int, arg6: c_int) -> i32;
  // proto: QPaintDevice * QPainter::device();
  fn _ZNK8QPainter6deviceEv() -> i32;
  // proto: void QPainter::setViewport(const QRect & viewport);
  fn _ZN8QPainter11setViewportERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QPainter::fillRect(const QRect & , const QColor & color);
  fn _ZN8QPainter8fillRectERK5QRectRK6QColor(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::setBrushOrigin(const QPointF & );
  fn _ZN8QPainter14setBrushOriginERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemEiiRK9QTextItem(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QPainter::NewQPainter(QPaintDevice * );
  fn _ZN8QPainterC1EP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm, int sx, int sy, int sw, int sh);
  fn _ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *const c_void, arg5: c_int, arg6: c_int, arg7: c_int, arg8: c_int) -> i32;
  // proto: void QPainter::drawImage(const QPoint & p, const QImage & image);
  fn _ZN8QPainter9drawImageERK6QPointRK6QImage(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawPie(const QRect & , int a, int alen);
  fn _ZN8QPainter7drawPieERK5QRectii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::drawTextItem(const QPoint & p, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemERK6QPointRK9QTextItem(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawLines(const QPoint * pointPairs, int lineCount);
  fn _ZN8QPainter9drawLinesEPK6QPointi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::drawPicture(int x, int y, const QPicture & picture);
  fn _ZN8QPainter11drawPictureEiiRK8QPicture(arg0: c_int, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QPainter::save();
  fn _ZN8QPainter4saveEv() -> i32;
  // proto: void QPainter::translate(qreal dx, qreal dy);
  fn _ZN8QPainter9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QTransform QPainter::combinedTransform();
  fn _ZNK8QPainter17combinedTransformEv() -> i32;
  // proto: bool QPainter::end();
  fn _ZN8QPainter3endEv() -> i32;
  // proto: void QPainter::setViewport(int x, int y, int w, int h);
  fn _ZN8QPainter11setViewportEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
  fn _ZN8QPainter13drawRoundRectERK5QRectii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
  fn _ZN8QPainter17setWorldTransformERK10QTransformb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QPainter::drawPoints(const QPolygonF & points);
  fn _ZN8QPainter10drawPointsERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: void QPainter::restore();
  fn _ZN8QPainter7restoreEv() -> i32;
  // proto: void QPainter::drawStaticText(const QPoint & topLeftPosition, const QStaticText & staticText);
  fn _ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: QRectF QPainter::boundingRect(const QRectF & rect, int flags, const QString & text);
  fn _ZN8QPainter12boundingRectERK6QRectFiRK7QString(arg0: *const c_void, arg1: c_int, arg2: *const c_void) -> i32;
  // proto: void QPainter::fillRect(int x, int y, int w, int h, const QBrush & );
  fn _ZN8QPainter8fillRectEiiiiRK6QBrush(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: *const c_void) -> i32;
  // proto: void QPainter::drawRoundRect(const QRectF & r, int xround, int yround);
  fn _ZN8QPainter13drawRoundRectERK6QRectFii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::drawPoint(const QPoint & p);
  fn _ZN8QPainter9drawPointERK6QPoint(arg0: *const c_void) -> i32;
  // proto: QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
  fn _ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint(arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QPainter::shear(qreal sh, qreal sv);
  fn _ZN8QPainter5shearEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QPainter::drawText(const QRect & r, int flags, const QString & text, QRect * br);
  fn _ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_(arg0: *const c_void, arg1: c_int, arg2: *const c_void, arg3: *mut c_void) -> i32;
  // proto: const QFont & QPainter::font();
  fn _ZNK8QPainter4fontEv() -> i32;
  // proto: const QTransform & QPainter::deviceTransform();
  fn _ZNK8QPainter15deviceTransformEv() -> i32;
  // proto: void QPainter::eraseRect(int x, int y, int w, int h);
  fn _ZN8QPainter9eraseRectEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::resetMatrix();
  fn _ZN8QPainter11resetMatrixEv() -> i32;
  // proto: void QPainter::drawPolyline(const QPoint * points, int pointCount);
  fn _ZN8QPainter12drawPolylineEPK6QPointi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: QPaintEngine * QPainter::paintEngine();
  fn _ZNK8QPainter11paintEngineEv() -> i32;
  // proto: void QPainter::drawEllipse(const QRect & r);
  fn _ZN8QPainter11drawEllipseERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawLine(const QLine & line);
  fn _ZN8QPainter8drawLineERK5QLine(arg0: *const c_void) -> i32;
  // proto: bool QPainter::isActive();
  fn _ZNK8QPainter8isActiveEv() -> i32;
  // proto: void QPainter::drawArc(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter7drawArcERK6QRectFii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::restoreRedirected(const QPaintDevice * device);
  fn _ZN8QPainter17restoreRedirectedEPK12QPaintDevice(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm, const QRectF & sr);
  fn _ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawEllipse(const QPointF & center, qreal rx, qreal ry);
  fn _ZN8QPainter11drawEllipseERK7QPointFdd(arg0: *const c_void, arg1: c_double, arg2: c_double) -> i32;
  // proto: void QPainter::drawConvexPolygon(const QPointF * points, int pointCount);
  fn _ZN8QPainter17drawConvexPolygonEPK7QPointFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::setBrushOrigin(const QPoint & );
  fn _ZN8QPainter14setBrushOriginERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawText(const QRectF & r, const QString & text, const QTextOption & o);
  fn _ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: bool QPainter::worldMatrixEnabled();
  fn _ZNK8QPainter18worldMatrixEnabledEv() -> i32;
  // proto: void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm);
  fn _ZN8QPainter10drawPixmapERK6QPointRK7QPixmap(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawLine(int x1, int y1, int x2, int y2);
  fn _ZN8QPainter8drawLineEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::drawPoint(int x, int y);
  fn _ZN8QPainter9drawPointEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: const QTransform & QPainter::transform();
  fn _ZNK8QPainter9transformEv() -> i32;
  // proto: void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
  fn _ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint(arg0: *const c_void, arg1: *mut c_void, arg2: *const c_void) -> i32;
  // proto: void QPainter::drawPixmap(const QRect & targetRect, const QPixmap & pixmap, const QRect & sourceRect);
  fn _ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_(arg0: *const c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: QFontMetrics QPainter::fontMetrics();
  fn _ZNK8QPainter11fontMetricsEv() -> i32;
  // proto: void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
  fn _ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::fillRect(const QRectF & , const QBrush & );
  fn _ZN8QPainter8fillRectERK6QRectFRK6QBrush(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::resetTransform();
  fn _ZN8QPainter14resetTransformEv() -> i32;
  // proto: void QPainter::fillRect(const QRect & , const QBrush & );
  fn _ZN8QPainter8fillRectERK5QRectRK6QBrush(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: const QBrush & QPainter::brush();
  fn _ZNK8QPainter5brushEv() -> i32;
  // proto: void QPainter::FreeQPainter();
  fn _ZN8QPainterD0Ev() -> i32;
  // proto: bool QPainter::begin(QPaintDevice * );
  fn _ZN8QPainter5beginEP12QPaintDevice(arg0: *mut c_void) -> i32;
  // proto: void QPainter::drawRect(const QRect & rect);
  fn _ZN8QPainter8drawRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawTextItem(const QPointF & p, const QTextItem & ti);
  fn _ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::scale(qreal sx, qreal sy);
  fn _ZN8QPainter5scaleEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
  fn _ZN8QPainter14setWorldMatrixERK7QMatrixb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: QPainterPath QPainter::clipPath();
  fn _ZNK8QPainter8clipPathEv() -> i32;
  // proto: QPoint QPainter::brushOrigin();
  fn _ZNK8QPainter11brushOriginEv() -> i32;
  // proto: void QPainter::drawConvexPolygon(const QPolygonF & polygon);
  fn _ZN8QPainter17drawConvexPolygonERK9QPolygonF(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawEllipse(int x, int y, int w, int h);
  fn _ZN8QPainter11drawEllipseEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::drawConvexPolygon(const QPolygon & polygon);
  fn _ZN8QPainter17drawConvexPolygonERK8QPolygon(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawPoints(const QPoint * points, int pointCount);
  fn _ZN8QPainter10drawPointsEPK6QPointi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: const QBrush & QPainter::background();
  fn _ZNK8QPainter10backgroundEv() -> i32;
  // proto: void QPainter::drawRoundRect(int x, int y, int w, int h, int , int );
  fn _ZN8QPainter13drawRoundRectEiiiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int) -> i32;
  // proto: QRect QPainter::viewport();
  fn _ZNK8QPainter8viewportEv() -> i32;
  // proto: void QPainter::drawArc(const QRect & , int a, int alen);
  fn _ZN8QPainter7drawArcERK5QRectii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
  fn _ZN8QPainter8fillPathERK12QPainterPathRK6QBrush(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawText(int x, int y, int w, int h, int flags, const QString & text, QRect * br);
  fn _ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: *const c_void, arg6: *mut c_void) -> i32;
  // proto: bool QPainter::matrixEnabled();
  fn _ZNK8QPainter13matrixEnabledEv() -> i32;
  // proto: void QPainter::drawPolyline(const QPointF * points, int pointCount);
  fn _ZN8QPainter12drawPolylineEPK7QPointFi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: void QPainter::setTransform(const QTransform & transform, bool combine);
  fn _ZN8QPainter12setTransformERK10QTransformb(arg0: *const c_void, arg1: int8_t) -> i32;
  // proto: void QPainter::setPen(const QColor & color);
  fn _ZN8QPainter6setPenERK6QColor(arg0: *const c_void) -> i32;
  // proto: void QPainter::eraseRect(const QRect & );
  fn _ZN8QPainter9eraseRectERK5QRect(arg0: *const c_void) -> i32;
  // proto: QRect QPainter::window();
  fn _ZNK8QPainter6windowEv() -> i32;
  // proto: void QPainter::drawImage(const QRect & r, const QImage & image);
  fn _ZN8QPainter9drawImageERK5QRectRK6QImage(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::initFrom(const QPaintDevice * device);
  fn _ZN8QPainter8initFromEPK12QPaintDevice(arg0: *const c_void) -> i32;
  // proto: QFontInfo QPainter::fontInfo();
  fn _ZNK8QPainter8fontInfoEv() -> i32;
  // proto: void QPainter::endNativePainting();
  fn _ZN8QPainter17endNativePaintingEv() -> i32;
  // proto: void QPainter::setViewTransformEnabled(bool enable);
  fn _ZN8QPainter23setViewTransformEnabledEb(arg0: int8_t) -> i32;
  // proto: void QPainter::drawPoint(const QPointF & pt);
  fn _ZN8QPainter9drawPointERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QPainter::setOpacity(qreal opacity);
  fn _ZN8QPainter10setOpacityEd(arg0: c_double) -> i32;
  // proto: void QPainter::fillRect(const QRectF & , const QColor & color);
  fn _ZN8QPainter8fillRectERK6QRectFRK6QColor(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::NewQPainter();
  fn _ZN8QPainterC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QPainter::translate(const QPointF & offset);
  fn _ZN8QPainter9translateERK7QPointF(arg0: *const c_void) -> i32;
  // proto: void QPainter::drawText(const QPointF & p, const QString & s);
  fn _ZN8QPainter8drawTextERK7QPointFRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawImage(const QPointF & p, const QImage & image);
  fn _ZN8QPainter9drawImageERK7QPointFRK6QImage(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: const QPen & QPainter::pen();
  fn _ZNK8QPainter3penEv() -> i32;
  // proto: void QPainter::rotate(qreal a);
  fn _ZN8QPainter6rotateEd(arg0: c_double) -> i32;
  // proto: QRectF QPainter::clipBoundingRect();
  fn _ZNK8QPainter16clipBoundingRectEv() -> i32;
  // proto: void QPainter::drawLine(const QPoint & p1, const QPoint & p2);
  fn _ZN8QPainter8drawLineERK6QPointS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::drawPie(const QRectF & rect, int a, int alen);
  fn _ZN8QPainter7drawPieERK6QRectFii(arg0: *const c_void, arg1: c_int, arg2: c_int) -> i32;
  // proto: void QPainter::drawText(const QPoint & p, const QString & s);
  fn _ZN8QPainter8drawTextERK6QPointRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QPainter::setWindow(int x, int y, int w, int h);
  fn _ZN8QPainter9setWindowEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  // proto: void QPainter::beginNativePainting();
  fn _ZN8QPainter19beginNativePaintingEv() -> i32;
}

// body block begin
// class sizeof(QPainter)=1
pub struct QPainter {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPainter {
  pub fn boundingRect<T: QPainter_boundingRect>(&mut self, value: T) -> i32 {
    value.boundingRect(self);
    return 1;
  }
}

pub trait QPainter_boundingRect {
  fn boundingRect(self, this: &mut QPainter) -> i32;
}

// proto: QRectF QPainter::boundingRect(const QRectF & rect, const QString & text, const QTextOption & o);
impl<'a> /*trait*/ QPainter_boundingRect for (&'a  QRectF, &'a  QString, &'a  QTextOption) {
  fn boundingRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12boundingRectERK6QRectFRK7QStringRK11QTextOption(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPicture<T: QPainter_drawPicture>(&mut self, value: T) -> i32 {
    value.drawPicture(self);
    return 1;
  }
}

pub trait QPainter_drawPicture {
  fn drawPicture(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPicture(const QPointF & p, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture for (&'a  QPointF, &'a  QPicture) {
  fn drawPicture(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureERK7QPointFRK8QPicture()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter11drawPictureERK7QPointFRK8QPicture(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn worldMatrix<T: QPainter_worldMatrix>(&mut self, value: T) -> i32 {
    value.worldMatrix(self);
    return 1;
  }
}

pub trait QPainter_worldMatrix {
  fn worldMatrix(self, this: &mut QPainter) -> i32;
}

// proto: const QMatrix & QPainter::worldMatrix();
impl<'a> /*trait*/ QPainter_worldMatrix for () {
  fn worldMatrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11worldMatrixEv()};
    unsafe {_ZNK8QPainter11worldMatrixEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawText<T: QPainter_drawText>(&mut self, value: T) -> i32 {
    value.drawText(self);
    return 1;
  }
}

pub trait QPainter_drawText {
  fn drawText(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawText(const QPointF & p, const QString & str, int tf, int justificationPadding);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QPointF, &'a  QString, i32, i32) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK7QPointFRK7QStringii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter8drawTextERK7QPointFRK7QStringii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fillRect<T: QPainter_fillRect>(&mut self, value: T) -> i32 {
    value.fillRect(self);
    return 1;
  }
}

pub trait QPainter_fillRect {
  fn fillRect(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::fillRect(int x, int y, int w, int h, const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect for (i32, i32, i32, i32, &'a  QColor) {
  fn fillRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectEiiiiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillRectEiiiiRK6QColor(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn matrix<T: QPainter_matrix>(&mut self, value: T) -> i32 {
    value.matrix(self);
    return 1;
  }
}

pub trait QPainter_matrix {
  fn matrix(self, this: &mut QPainter) -> i32;
}

// proto: const QMatrix & QPainter::matrix();
impl<'a> /*trait*/ QPainter_matrix for () {
  fn matrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6matrixEv()};
    unsafe {_ZNK8QPainter6matrixEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn opacity<T: QPainter_opacity>(&mut self, value: T) -> i32 {
    value.opacity(self);
    return 1;
  }
}

pub trait QPainter_opacity {
  fn opacity(self, this: &mut QPainter) -> i32;
}

// proto: double QPainter::opacity();
impl<'a> /*trait*/ QPainter_opacity for () {
  fn opacity(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter7opacityEv()};
    unsafe {_ZNK8QPainter7opacityEv()};
    return 1;
  }
}

// proto: void QPainter::drawText(int x, int y, const QString & s);
impl<'a> /*trait*/ QPainter_drawText for (i32, i32, &'a  QString) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextEiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawTextEiiRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawTiledPixmap<T: QPainter_drawTiledPixmap>(&mut self, value: T) -> i32 {
    value.drawTiledPixmap(self);
    return 1;
  }
}

pub trait QPainter_drawTiledPixmap {
  fn drawTiledPixmap(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawTiledPixmap(const QRectF & rect, const QPixmap & pm, const QPointF & offset);
impl<'a> /*trait*/ QPainter_drawTiledPixmap for (&'a  QRectF, &'a  QPixmap, &'a  QPointF) {
  fn drawTiledPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter15drawTiledPixmapERK6QRectFRK7QPixmapRK7QPointF(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setBackground<T: QPainter_setBackground>(&mut self, value: T) -> i32 {
    value.setBackground(self);
    return 1;
  }
}

pub trait QPainter_setBackground {
  fn setBackground(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setBackground(const QBrush & bg);
impl<'a> /*trait*/ QPainter_setBackground for (&'a  QBrush) {
  fn setBackground(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter13setBackgroundERK6QBrush(arg0)};
    return 1;
  }
}

// proto: QRect QPainter::boundingRect(const QRect & rect, int flags, const QString & text);
impl<'a> /*trait*/ QPainter_boundingRect for (&'a  QRect, i32, &'a  QString) {
  fn boundingRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectERK5QRectiRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12boundingRectERK5QRectiRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawChord<T: QPainter_drawChord>(&mut self, value: T) -> i32 {
    value.drawChord(self);
    return 1;
  }
}

pub trait QPainter_drawChord {
  fn drawChord(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawChord(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord for (&'a  QRectF, i32, i32) {
  fn drawChord(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter9drawChordERK6QRectFii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawImage<T: QPainter_drawImage>(&mut self, value: T) -> i32 {
    value.drawImage(self);
    return 1;
  }
}

pub trait QPainter_drawImage {
  fn drawImage(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawImage(const QRectF & r, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QRectF, &'a  QImage) {
  fn drawImage(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK6QRectFRK6QImage()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9drawImageERK6QRectFRK6QImage(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setClipping<T: QPainter_setClipping>(&mut self, value: T) -> i32 {
    value.setClipping(self);
    return 1;
  }
}

pub trait QPainter_setClipping {
  fn setClipping(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setClipping(bool enable);
impl<'a> /*trait*/ QPainter_setClipping for (i8) {
  fn setClipping(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setClippingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QPainter11setClippingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setBrush<T: QPainter_setBrush>(&mut self, value: T) -> i32 {
    value.setBrush(self);
    return 1;
  }
}

pub trait QPainter_setBrush {
  fn setBrush(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setBrush(const QBrush & brush);
impl<'a> /*trait*/ QPainter_setBrush for (&'a  QBrush) {
  fn setBrush(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8setBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8setBrushERK6QBrush(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setMatrix<T: QPainter_setMatrix>(&mut self, value: T) -> i32 {
    value.setMatrix(self);
    return 1;
  }
}

pub trait QPainter_setMatrix {
  fn setMatrix(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setMatrix for (&'a  QMatrix, i8) {
  fn setMatrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QPainter9setMatrixERK7QMatrixb(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawChord(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord for (&'a  QRect, i32, i32) {
  fn drawChord(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordERK5QRectii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter9drawChordERK5QRectii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn eraseRect<T: QPainter_eraseRect>(&mut self, value: T) -> i32 {
    value.eraseRect(self);
    return 1;
  }
}

pub trait QPainter_eraseRect {
  fn eraseRect(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::eraseRect(const QRectF & );
impl<'a> /*trait*/ QPainter_eraseRect for (&'a  QRectF) {
  fn eraseRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9eraseRectERK6QRectF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn translate<T: QPainter_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QPainter_translate {
  fn translate(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::translate(const QPoint & offset);
impl<'a> /*trait*/ QPainter_translate for (&'a  QPoint) {
  fn translate(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9translateERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn viewTransformEnabled<T: QPainter_viewTransformEnabled>(&mut self, value: T) -> i32 {
    value.viewTransformEnabled(self);
    return 1;
  }
}

pub trait QPainter_viewTransformEnabled {
  fn viewTransformEnabled(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::viewTransformEnabled();
impl<'a> /*trait*/ QPainter_viewTransformEnabled for () {
  fn viewTransformEnabled(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter20viewTransformEnabledEv()};
    unsafe {_ZNK8QPainter20viewTransformEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setPen<T: QPainter_setPen>(&mut self, value: T) -> i32 {
    value.setPen(self);
    return 1;
  }
}

pub trait QPainter_setPen {
  fn setPen(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setPen(const QPen & pen);
impl<'a> /*trait*/ QPainter_setPen for (&'a  QPen) {
  fn setPen(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter6setPenERK4QPen(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawLines<T: QPainter_drawLines>(&mut self, value: T) -> i32 {
    value.drawLines(self);
    return 1;
  }
}

pub trait QPainter_drawLines {
  fn drawLines(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawLines(const QLineF * lines, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QLineF, i32) {
  fn drawLines(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK6QLineFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawLinesEPK6QLineFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setBrushOrigin<T: QPainter_setBrushOrigin>(&mut self, value: T) -> i32 {
    value.setBrushOrigin(self);
    return 1;
  }
}

pub trait QPainter_setBrushOrigin {
  fn setBrushOrigin(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setBrushOrigin(int x, int y);
impl<'a> /*trait*/ QPainter_setBrushOrigin for (i32, i32) {
  fn setBrushOrigin(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter14setBrushOriginEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn worldTransform<T: QPainter_worldTransform>(&mut self, value: T) -> i32 {
    value.worldTransform(self);
    return 1;
  }
}

pub trait QPainter_worldTransform {
  fn worldTransform(self, this: &mut QPainter) -> i32;
}

// proto: const QTransform & QPainter::worldTransform();
impl<'a> /*trait*/ QPainter_worldTransform for () {
  fn worldTransform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter14worldTransformEv()};
    unsafe {_ZNK8QPainter14worldTransformEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawRects<T: QPainter_drawRects>(&mut self, value: T) -> i32 {
    value.drawRects(self);
    return 1;
  }
}

pub trait QPainter_drawRects {
  fn drawRects(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawRects(const QRect * rects, int rectCount);
impl<'a> /*trait*/ QPainter_drawRects for (&'a  QRect, i32) {
  fn drawRects(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawRectsEPK5QRecti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawEllipse<T: QPainter_drawEllipse>(&mut self, value: T) -> i32 {
    value.drawEllipse(self);
    return 1;
  }
}

pub trait QPainter_drawEllipse {
  fn drawEllipse(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawEllipse(const QPoint & center, int rx, int ry);
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QPoint, i32, i32) {
  fn drawEllipse(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK6QPointii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter11drawEllipseERK6QPointii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawArc<T: QPainter_drawArc>(&mut self, value: T) -> i32 {
    value.drawArc(self);
    return 1;
  }
}

pub trait QPainter_drawArc {
  fn drawArc(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawArc(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc for (i32, i32, i32, i32, i32, i32) {
  fn drawArc(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    unsafe {_ZN8QPainter7drawArcEiiiiii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPolyline<T: QPainter_drawPolyline>(&mut self, value: T) -> i32 {
    value.drawPolyline(self);
    return 1;
  }
}

pub trait QPainter_drawPolyline {
  fn drawPolyline(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPolyline(const QPolygonF & polyline);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPolygonF) {
  fn drawPolyline(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12drawPolylineERK9QPolygonF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn hasClipping<T: QPainter_hasClipping>(&mut self, value: T) -> i32 {
    value.hasClipping(self);
    return 1;
  }
}

pub trait QPainter_hasClipping {
  fn hasClipping(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::hasClipping();
impl<'a> /*trait*/ QPainter_hasClipping for () {
  fn hasClipping(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11hasClippingEv()};
    unsafe {_ZNK8QPainter11hasClippingEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPixmap<T: QPainter_drawPixmap>(&mut self, value: T) -> i32 {
    value.drawPixmap(self);
    return 1;
  }
}

pub trait QPainter_drawPixmap {
  fn drawPixmap(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPixmap(const QRectF & targetRect, const QPixmap & pixmap, const QRectF & sourceRect);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QRectF, &'a  QPixmap, &'a  QRectF) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK6QRectFRK7QPixmapS2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawStaticText<T: QPainter_drawStaticText>(&mut self, value: T) -> i32 {
    value.drawStaticText(self);
    return 1;
  }
}

pub trait QPainter_drawStaticText {
  fn drawStaticText(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawStaticText(int left, int top, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText for (i32, i32, &'a  QStaticText) {
  fn drawStaticText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextEiiRK11QStaticText()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter14drawStaticTextEiiRK11QStaticText(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn strokePath<T: QPainter_strokePath>(&mut self, value: T) -> i32 {
    value.strokePath(self);
    return 1;
  }
}

pub trait QPainter_strokePath {
  fn strokePath(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::strokePath(const QPainterPath & path, const QPen & pen);
impl<'a> /*trait*/ QPainter_strokePath for (&'a  QPainterPath, &'a  QPen) {
  fn strokePath(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10strokePathERK12QPainterPathRK4QPen()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10strokePathERK12QPainterPathRK4QPen(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(int x, int y, const QPixmap & pm, int sx, int sy, int sw, int sh);
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, &'a  QPixmap, i32, i32, i32, i32) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiRK7QPixmapiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    unsafe {_ZN8QPainter10drawPixmapEiiRK7QPixmapiiii(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

// proto: void QPainter::drawRects(const QRectF * rects, int rectCount);
impl<'a> /*trait*/ QPainter_drawRects for (&'a  QRectF, i32) {
  fn drawRects(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawRectsEPK6QRectFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawRectsEPK6QRectFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawConvexPolygon<T: QPainter_drawConvexPolygon>(&mut self, value: T) -> i32 {
    value.drawConvexPolygon(self);
    return 1;
  }
}

pub trait QPainter_drawConvexPolygon {
  fn drawConvexPolygon(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawConvexPolygon(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPoint, i32) {
  fn drawConvexPolygon(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter17drawConvexPolygonEPK6QPointi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPath<T: QPainter_drawPath>(&mut self, value: T) -> i32 {
    value.drawPath(self);
    return 1;
  }
}

pub trait QPainter_drawPath {
  fn drawPath(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainter_drawPath for (&'a  QPainterPath) {
  fn drawPath(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawPathERK12QPainterPath(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(int x, int y, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, &'a  QPixmap) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiRK7QPixmap()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapEiiRK7QPixmap(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn combinedMatrix<T: QPainter_combinedMatrix>(&mut self, value: T) -> i32 {
    value.combinedMatrix(self);
    return 1;
  }
}

pub trait QPainter_combinedMatrix {
  fn combinedMatrix(self, this: &mut QPainter) -> i32;
}

// proto: QMatrix QPainter::combinedMatrix();
impl<'a> /*trait*/ QPainter_combinedMatrix for () {
  fn combinedMatrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter14combinedMatrixEv()};
    unsafe {_ZNK8QPainter14combinedMatrixEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setMatrixEnabled<T: QPainter_setMatrixEnabled>(&mut self, value: T) -> i32 {
    value.setMatrixEnabled(self);
    return 1;
  }
}

pub trait QPainter_setMatrixEnabled {
  fn setMatrixEnabled(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setMatrixEnabled(bool enabled);
impl<'a> /*trait*/ QPainter_setMatrixEnabled for (i8) {
  fn setMatrixEnabled(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter16setMatrixEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QPainter16setMatrixEnabledEb(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPolyline(const QPolygon & polygon);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPolygon) {
  fn drawPolyline(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12drawPolylineERK8QPolygon(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawTiledPixmap(const QRect & , const QPixmap & , const QPoint & );
impl<'a> /*trait*/ QPainter_drawTiledPixmap for (&'a  QRect, &'a  QPixmap, &'a  QPoint) {
  fn drawTiledPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter15drawTiledPixmapERK5QRectRK7QPixmapRK6QPoint(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setFont<T: QPainter_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QPainter_setFont {
  fn setFont(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setFont(const QFont & f);
impl<'a> /*trait*/ QPainter_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter7setFontERK5QFont(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawChord(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawChord for (i32, i32, i32, i32, i32, i32) {
  fn drawChord(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawChordEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    unsafe {_ZN8QPainter9drawChordEiiiiii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, i32, i32, &'a  QPixmap) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiiiRK7QPixmap()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapEiiiiRK7QPixmap(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWindow<T: QPainter_setWindow>(&mut self, value: T) -> i32 {
    value.setWindow(self);
    return 1;
  }
}

pub trait QPainter_setWindow {
  fn setWindow(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setWindow(const QRect & window);
impl<'a> /*trait*/ QPainter_setWindow for (&'a  QRect) {
  fn setWindow(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setWindowERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9setWindowERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn deviceMatrix<T: QPainter_deviceMatrix>(&mut self, value: T) -> i32 {
    value.deviceMatrix(self);
    return 1;
  }
}

pub trait QPainter_deviceMatrix {
  fn deviceMatrix(self, this: &mut QPainter) -> i32;
}

// proto: const QMatrix & QPainter::deviceMatrix();
impl<'a> /*trait*/ QPainter_deviceMatrix for () {
  fn deviceMatrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter12deviceMatrixEv()};
    unsafe {_ZNK8QPainter12deviceMatrixEv()};
    return 1;
  }
}

// proto: void QPainter::drawLines(const QPointF * pointPairs, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QPointF, i32) {
  fn drawLines(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawLinesEPK7QPointFi(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPointF, &'a  QPixmap) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmap(arg0, arg1)};
    return 1;
  }
}

// proto: QRect QPainter::boundingRect(int x, int y, int w, int h, int flags, const QString & text);
impl<'a> /*trait*/ QPainter_boundingRect for (i32, i32, i32, i32, i32, &'a  QString) {
  fn boundingRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectEiiiiiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12boundingRectEiiiiiRK7QString(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

// proto: void QPainter::drawLines(const QLine * lines, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QLine, i32) {
  fn drawLines(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK5QLinei()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawLinesEPK5QLinei(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPie<T: QPainter_drawPie>(&mut self, value: T) -> i32 {
    value.drawPie(self);
    return 1;
  }
}

pub trait QPainter_drawPie {
  fn drawPie(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPie(int x, int y, int w, int h, int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie for (i32, i32, i32, i32, i32, i32) {
  fn drawPie(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawPieEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    unsafe {_ZN8QPainter7drawPieEiiiiii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm, const QRect & sr);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPoint, &'a  QPixmap, &'a  QRect) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK6QPointRK7QPixmapRK5QRect(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::drawStaticText(const QPointF & topLeftPosition, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText for (&'a  QPointF, &'a  QStaticText) {
  fn drawStaticText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter14drawStaticTextERK7QPointFRK11QStaticText(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWorldMatrixEnabled<T: QPainter_setWorldMatrixEnabled>(&mut self, value: T) -> i32 {
    value.setWorldMatrixEnabled(self);
    return 1;
  }
}

pub trait QPainter_setWorldMatrixEnabled {
  fn setWorldMatrixEnabled(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setWorldMatrixEnabled(bool enabled);
impl<'a> /*trait*/ QPainter_setWorldMatrixEnabled for (i8) {
  fn setWorldMatrixEnabled(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter21setWorldMatrixEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QPainter21setWorldMatrixEnabledEb(arg0)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainterC1ERKS_(qthis, arg0)};
    let rsthis = QPainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPoints<T: QPainter_drawPoints>(&mut self, value: T) -> i32 {
    value.drawPoints(self);
    return 1;
  }
}

pub trait QPainter_drawPoints {
  fn drawPoints(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPoints(const QPolygon & points);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPolygon) {
  fn drawPoints(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPointsERK8QPolygon(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPicture(const QPoint & p, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture for (&'a  QPoint, &'a  QPicture) {
  fn drawPicture(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureERK6QPointRK8QPicture()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter11drawPictureERK6QPointRK8QPicture(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawRect<T: QPainter_drawRect>(&mut self, value: T) -> i32 {
    value.drawRect(self);
    return 1;
  }
}

pub trait QPainter_drawRect {
  fn drawRect(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawRect(int x1, int y1, int w, int h);
impl<'a> /*trait*/ QPainter_drawRect for (i32, i32, i32, i32) {
  fn drawRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter8drawRectEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QPainter::drawEllipse(const QRectF & r);
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QRectF) {
  fn drawEllipse(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter11drawEllipseERK6QRectF(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawRect(const QRectF & rect);
impl<'a> /*trait*/ QPainter_drawRect for (&'a  QRectF) {
  fn drawRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectERK6QRectF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawRectERK6QRectF(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPoints(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPointF, i32) {
  fn drawPoints(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter10drawPointsEPK7QPointFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn clipRegion<T: QPainter_clipRegion>(&mut self, value: T) -> i32 {
    value.clipRegion(self);
    return 1;
  }
}

pub trait QPainter_clipRegion {
  fn clipRegion(self, this: &mut QPainter) -> i32;
}

// proto: QRegion QPainter::clipRegion();
impl<'a> /*trait*/ QPainter_clipRegion for () {
  fn clipRegion(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter10clipRegionEv()};
    unsafe {_ZNK8QPainter10clipRegionEv()};
    return 1;
  }
}

// proto: void QPainter::drawText(const QRectF & r, int flags, const QString & text, QRectF * br);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QRectF, i32, &'a  QString, &'a mut QRectF) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainter8drawTextERK6QRectFiRK7QStringPS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawLine<T: QPainter_drawLine>(&mut self, value: T) -> i32 {
    value.drawLine(self);
    return 1;
  }
}

pub trait QPainter_drawLine {
  fn drawLine(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawLine(const QLineF & line);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QLineF) {
  fn drawLine(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK6QLineF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawLineERK6QLineF(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawLine(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QPointF, &'a  QPointF) {
  fn drawLine(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawLineERK7QPointFS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(const QRect & r, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QRect, &'a  QPixmap) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK5QRectRK7QPixmap(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap & , int sx, int sy);
impl<'a> /*trait*/ QPainter_drawTiledPixmap for (i32, i32, i32, i32, &'a  QPixmap, i32, i32) {
  fn drawTiledPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    unsafe {_ZN8QPainter15drawTiledPixmapEiiiiRK7QPixmapii(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn device<T: QPainter_device>(&mut self, value: T) -> i32 {
    value.device(self);
    return 1;
  }
}

pub trait QPainter_device {
  fn device(self, this: &mut QPainter) -> i32;
}

// proto: QPaintDevice * QPainter::device();
impl<'a> /*trait*/ QPainter_device for () {
  fn device(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6deviceEv()};
    unsafe {_ZNK8QPainter6deviceEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setViewport<T: QPainter_setViewport>(&mut self, value: T) -> i32 {
    value.setViewport(self);
    return 1;
  }
}

pub trait QPainter_setViewport {
  fn setViewport(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setViewport(const QRect & viewport);
impl<'a> /*trait*/ QPainter_setViewport for (&'a  QRect) {
  fn setViewport(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setViewportERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter11setViewportERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QPainter::fillRect(const QRect & , const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRect, &'a  QColor) {
  fn fillRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK5QRectRK6QColor()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillRectERK5QRectRK6QColor(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::setBrushOrigin(const QPointF & );
impl<'a> /*trait*/ QPainter_setBrushOrigin for (&'a  QPointF) {
  fn setBrushOrigin(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter14setBrushOriginERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawTextItem<T: QPainter_drawTextItem>(&mut self, value: T) -> i32 {
    value.drawTextItem(self);
    return 1;
  }
}

pub trait QPainter_drawTextItem {
  fn drawTextItem(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawTextItem(int x, int y, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem for (i32, i32, &'a  QTextItem) {
  fn drawTextItem(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemEiiRK9QTextItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12drawTextItemEiiRK9QTextItem(arg0, arg1, arg2)};
    return 1;
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

// proto: void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap & pm, int sx, int sy, int sw, int sh);
impl<'a> /*trait*/ QPainter_drawPixmap for (i32, i32, i32, i32, &'a  QPixmap, i32, i32, i32, i32) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *const c_void;
    let arg5 = self.5  as c_int;
    let arg6 = self.6  as c_int;
    let arg7 = self.7  as c_int;
    let arg8 = self.8  as c_int;
    unsafe {_ZN8QPainter10drawPixmapEiiiiRK7QPixmapiiii(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8)};
    return 1;
  }
}

// proto: void QPainter::drawImage(const QPoint & p, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QPoint, &'a  QImage) {
  fn drawImage(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK6QPointRK6QImage()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9drawImageERK6QPointRK6QImage(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPie(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie for (&'a  QRect, i32, i32) {
  fn drawPie(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawPieERK5QRectii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter7drawPieERK5QRectii(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::drawTextItem(const QPoint & p, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem for (&'a  QPoint, &'a  QTextItem) {
  fn drawTextItem(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12drawTextItemERK6QPointRK9QTextItem(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawLines(const QPoint * pointPairs, int lineCount);
impl<'a> /*trait*/ QPainter_drawLines for (&'a  QPoint, i32) {
  fn drawLines(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawLinesEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawLinesEPK6QPointi(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPicture(int x, int y, const QPicture & picture);
impl<'a> /*trait*/ QPainter_drawPicture for (i32, i32, &'a  QPicture) {
  fn drawPicture(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawPictureEiiRK8QPicture()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter11drawPictureEiiRK8QPicture(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn save<T: QPainter_save>(&mut self, value: T) -> i32 {
    value.save(self);
    return 1;
  }
}

pub trait QPainter_save {
  fn save(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::save();
impl<'a> /*trait*/ QPainter_save for () {
  fn save(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter4saveEv()};
    unsafe {_ZN8QPainter4saveEv()};
    return 1;
  }
}

// proto: void QPainter::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainter_translate for (f64, f64) {
  fn translate(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN8QPainter9translateEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn combinedTransform<T: QPainter_combinedTransform>(&mut self, value: T) -> i32 {
    value.combinedTransform(self);
    return 1;
  }
}

pub trait QPainter_combinedTransform {
  fn combinedTransform(self, this: &mut QPainter) -> i32;
}

// proto: QTransform QPainter::combinedTransform();
impl<'a> /*trait*/ QPainter_combinedTransform for () {
  fn combinedTransform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter17combinedTransformEv()};
    unsafe {_ZNK8QPainter17combinedTransformEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn end<T: QPainter_end>(&mut self, value: T) -> i32 {
    value.end(self);
    return 1;
  }
}

pub trait QPainter_end {
  fn end(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::end();
impl<'a> /*trait*/ QPainter_end for () {
  fn end(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter3endEv()};
    unsafe {_ZN8QPainter3endEv()};
    return 1;
  }
}

// proto: void QPainter::setViewport(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_setViewport for (i32, i32, i32, i32) {
  fn setViewport(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11setViewportEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter11setViewportEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawRoundRect<T: QPainter_drawRoundRect>(&mut self, value: T) -> i32 {
    value.drawRoundRect(self);
    return 1;
  }
}

pub trait QPainter_drawRoundRect {
  fn drawRoundRect(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawRoundRect(const QRect & r, int xround, int yround);
impl<'a> /*trait*/ QPainter_drawRoundRect for (&'a  QRect, i32, i32) {
  fn drawRoundRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectERK5QRectii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter13drawRoundRectERK5QRectii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWorldTransform<T: QPainter_setWorldTransform>(&mut self, value: T) -> i32 {
    value.setWorldTransform(self);
    return 1;
  }
}

pub trait QPainter_setWorldTransform {
  fn setWorldTransform(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setWorldTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setWorldTransform for (&'a  QTransform, i8) {
  fn setWorldTransform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17setWorldTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QPainter17setWorldTransformERK10QTransformb(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPoints(const QPolygonF & points);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPolygonF) {
  fn drawPoints(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPointsERK9QPolygonF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn restore<T: QPainter_restore>(&mut self, value: T) -> i32 {
    value.restore(self);
    return 1;
  }
}

pub trait QPainter_restore {
  fn restore(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::restore();
impl<'a> /*trait*/ QPainter_restore for () {
  fn restore(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7restoreEv()};
    unsafe {_ZN8QPainter7restoreEv()};
    return 1;
  }
}

// proto: void QPainter::drawStaticText(const QPoint & topLeftPosition, const QStaticText & staticText);
impl<'a> /*trait*/ QPainter_drawStaticText for (&'a  QPoint, &'a  QStaticText) {
  fn drawStaticText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter14drawStaticTextERK6QPointRK11QStaticText(arg0, arg1)};
    return 1;
  }
}

// proto: QRectF QPainter::boundingRect(const QRectF & rect, int flags, const QString & text);
impl<'a> /*trait*/ QPainter_boundingRect for (&'a  QRectF, i32, &'a  QString) {
  fn boundingRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12boundingRectERK6QRectFiRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12boundingRectERK6QRectFiRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::fillRect(int x, int y, int w, int h, const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect for (i32, i32, i32, i32, &'a  QBrush) {
  fn fillRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectEiiiiRK6QBrush()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillRectEiiiiRK6QBrush(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

// proto: void QPainter::drawRoundRect(const QRectF & r, int xround, int yround);
impl<'a> /*trait*/ QPainter_drawRoundRect for (&'a  QRectF, i32, i32) {
  fn drawRoundRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter13drawRoundRectERK6QRectFii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawPoint<T: QPainter_drawPoint>(&mut self, value: T) -> i32 {
    value.drawPoint(self);
    return 1;
  }
}

pub trait QPainter_drawPoint {
  fn drawPoint(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawPoint(const QPoint & p);
impl<'a> /*trait*/ QPainter_drawPoint for (&'a  QPoint) {
  fn drawPoint(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9drawPointERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn redirected<T: QPainter_redirected>(&mut self, value: T) -> i32 {
    value.redirected(self);
    return 1;
  }
}

pub trait QPainter_redirected {
  fn redirected(self, this: &mut QPainter) -> i32;
}

// proto: QPaintDevice * QPainter::redirected(const QPaintDevice * device, QPoint * offset);
impl<'a> /*trait*/ QPainter_redirected for (&'a  QPaintDevice, &'a mut QPoint) {
  fn redirected(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainter10redirectedEPK12QPaintDeviceP6QPoint(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn shear<T: QPainter_shear>(&mut self, value: T) -> i32 {
    value.shear(self);
    return 1;
  }
}

pub trait QPainter_shear {
  fn shear(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QPainter_shear for (f64, f64) {
  fn shear(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN8QPainter5shearEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawText(const QRect & r, int flags, const QString & text, QRect * br);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QRect, i32, &'a  QString, &'a mut QRect) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainter8drawTextERK5QRectiRK7QStringPS0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn font<T: QPainter_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QPainter_font {
  fn font(self, this: &mut QPainter) -> i32;
}

// proto: const QFont & QPainter::font();
impl<'a> /*trait*/ QPainter_font for () {
  fn font(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter4fontEv()};
    unsafe {_ZNK8QPainter4fontEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn deviceTransform<T: QPainter_deviceTransform>(&mut self, value: T) -> i32 {
    value.deviceTransform(self);
    return 1;
  }
}

pub trait QPainter_deviceTransform {
  fn deviceTransform(self, this: &mut QPainter) -> i32;
}

// proto: const QTransform & QPainter::deviceTransform();
impl<'a> /*trait*/ QPainter_deviceTransform for () {
  fn deviceTransform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter15deviceTransformEv()};
    unsafe {_ZNK8QPainter15deviceTransformEv()};
    return 1;
  }
}

// proto: void QPainter::eraseRect(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_eraseRect for (i32, i32, i32, i32) {
  fn eraseRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter9eraseRectEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn resetMatrix<T: QPainter_resetMatrix>(&mut self, value: T) -> i32 {
    value.resetMatrix(self);
    return 1;
  }
}

pub trait QPainter_resetMatrix {
  fn resetMatrix(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::resetMatrix();
impl<'a> /*trait*/ QPainter_resetMatrix for () {
  fn resetMatrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11resetMatrixEv()};
    unsafe {_ZN8QPainter11resetMatrixEv()};
    return 1;
  }
}

// proto: void QPainter::drawPolyline(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPoint, i32) {
  fn drawPolyline(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter12drawPolylineEPK6QPointi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn paintEngine<T: QPainter_paintEngine>(&mut self, value: T) -> i32 {
    value.paintEngine(self);
    return 1;
  }
}

pub trait QPainter_paintEngine {
  fn paintEngine(self, this: &mut QPainter) -> i32;
}

// proto: QPaintEngine * QPainter::paintEngine();
impl<'a> /*trait*/ QPainter_paintEngine for () {
  fn paintEngine(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11paintEngineEv()};
    unsafe {_ZNK8QPainter11paintEngineEv()};
    return 1;
  }
}

// proto: void QPainter::drawEllipse(const QRect & r);
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QRect) {
  fn drawEllipse(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter11drawEllipseERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawLine(const QLine & line);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QLine) {
  fn drawLine(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK5QLine()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawLineERK5QLine(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn isActive<T: QPainter_isActive>(&mut self, value: T) -> i32 {
    value.isActive(self);
    return 1;
  }
}

pub trait QPainter_isActive {
  fn isActive(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::isActive();
impl<'a> /*trait*/ QPainter_isActive for () {
  fn isActive(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8isActiveEv()};
    unsafe {_ZNK8QPainter8isActiveEv()};
    return 1;
  }
}

// proto: void QPainter::drawArc(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc for (&'a  QRectF, i32, i32) {
  fn drawArc(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter7drawArcERK6QRectFii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn restoreRedirected<T: QPainter_restoreRedirected>(&mut self, value: T) -> i32 {
    value.restoreRedirected(self);
    return 1;
  }
}

pub trait QPainter_restoreRedirected {
  fn restoreRedirected(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::restoreRedirected(const QPaintDevice * device);
impl<'a> /*trait*/ QPainter_restoreRedirected for (&'a  QPaintDevice) {
  fn restoreRedirected(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17restoreRedirectedEPK12QPaintDevice()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter17restoreRedirectedEPK12QPaintDevice(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(const QPointF & p, const QPixmap & pm, const QRectF & sr);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPointF, &'a  QPixmap, &'a  QRectF) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK7QPointFRK7QPixmapRK6QRectF(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::drawEllipse(const QPointF & center, qreal rx, qreal ry);
impl<'a> /*trait*/ QPainter_drawEllipse for (&'a  QPointF, f64, f64) {
  fn drawEllipse(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseERK7QPointFdd()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN8QPainter11drawEllipseERK7QPointFdd(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::drawConvexPolygon(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPointF, i32) {
  fn drawConvexPolygon(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter17drawConvexPolygonEPK7QPointFi(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::setBrushOrigin(const QPoint & );
impl<'a> /*trait*/ QPainter_setBrushOrigin for (&'a  QPoint) {
  fn setBrushOrigin(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setBrushOriginERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter14setBrushOriginERK6QPoint(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawText(const QRectF & r, const QString & text, const QTextOption & o);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QRectF, &'a  QString, &'a  QTextOption) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawTextERK6QRectFRK7QStringRK11QTextOption(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn worldMatrixEnabled<T: QPainter_worldMatrixEnabled>(&mut self, value: T) -> i32 {
    value.worldMatrixEnabled(self);
    return 1;
  }
}

pub trait QPainter_worldMatrixEnabled {
  fn worldMatrixEnabled(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::worldMatrixEnabled();
impl<'a> /*trait*/ QPainter_worldMatrixEnabled for () {
  fn worldMatrixEnabled(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter18worldMatrixEnabledEv()};
    unsafe {_ZNK8QPainter18worldMatrixEnabledEv()};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(const QPoint & p, const QPixmap & pm);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QPoint, &'a  QPixmap) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK6QPointRK7QPixmap(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QPainter_drawLine for (i32, i32, i32, i32) {
  fn drawLine(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter8drawLineEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QPainter::drawPoint(int x, int y);
impl<'a> /*trait*/ QPainter_drawPoint for (i32, i32) {
  fn drawPoint(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter9drawPointEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn transform<T: QPainter_transform>(&mut self, value: T) -> i32 {
    value.transform(self);
    return 1;
  }
}

pub trait QPainter_transform {
  fn transform(self, this: &mut QPainter) -> i32;
}

// proto: const QTransform & QPainter::transform();
impl<'a> /*trait*/ QPainter_transform for () {
  fn transform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter9transformEv()};
    unsafe {_ZNK8QPainter9transformEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setRedirected<T: QPainter_setRedirected>(&mut self, value: T) -> i32 {
    value.setRedirected(self);
    return 1;
  }
}

pub trait QPainter_setRedirected {
  fn setRedirected(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setRedirected(const QPaintDevice * device, QPaintDevice * replacement, const QPoint & offset);
impl<'a> /*trait*/ QPainter_setRedirected for (&'a  QPaintDevice, &'a mut QPaintDevice, &'a  QPoint) {
  fn setRedirected(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter13setRedirectedEPK12QPaintDevicePS0_RK6QPoint(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::drawPixmap(const QRect & targetRect, const QPixmap & pixmap, const QRect & sourceRect);
impl<'a> /*trait*/ QPainter_drawPixmap for (&'a  QRect, &'a  QPixmap, &'a  QRect) {
  fn drawPixmap(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter10drawPixmapERK5QRectRK7QPixmapS2_(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fontMetrics<T: QPainter_fontMetrics>(&mut self, value: T) -> i32 {
    value.fontMetrics(self);
    return 1;
  }
}

pub trait QPainter_fontMetrics {
  fn fontMetrics(self, this: &mut QPainter) -> i32;
}

// proto: QFontMetrics QPainter::fontMetrics();
impl<'a> /*trait*/ QPainter_fontMetrics for () {
  fn fontMetrics(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11fontMetricsEv()};
    unsafe {_ZNK8QPainter11fontMetricsEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn drawGlyphRun<T: QPainter_drawGlyphRun>(&mut self, value: T) -> i32 {
    value.drawGlyphRun(self);
    return 1;
  }
}

pub trait QPainter_drawGlyphRun {
  fn drawGlyphRun(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::drawGlyphRun(const QPointF & position, const QGlyphRun & glyphRun);
impl<'a> /*trait*/ QPainter_drawGlyphRun for (&'a  QPointF, &'a  QGlyphRun) {
  fn drawGlyphRun(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12drawGlyphRunERK7QPointFRK9QGlyphRun(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::fillRect(const QRectF & , const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRectF, &'a  QBrush) {
  fn fillRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK6QRectFRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillRectERK6QRectFRK6QBrush(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn resetTransform<T: QPainter_resetTransform>(&mut self, value: T) -> i32 {
    value.resetTransform(self);
    return 1;
  }
}

pub trait QPainter_resetTransform {
  fn resetTransform(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::resetTransform();
impl<'a> /*trait*/ QPainter_resetTransform for () {
  fn resetTransform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14resetTransformEv()};
    unsafe {_ZN8QPainter14resetTransformEv()};
    return 1;
  }
}

// proto: void QPainter::fillRect(const QRect & , const QBrush & );
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRect, &'a  QBrush) {
  fn fillRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK5QRectRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillRectERK5QRectRK6QBrush(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn brush<T: QPainter_brush>(&mut self, value: T) -> i32 {
    value.brush(self);
    return 1;
  }
}

pub trait QPainter_brush {
  fn brush(self, this: &mut QPainter) -> i32;
}

// proto: const QBrush & QPainter::brush();
impl<'a> /*trait*/ QPainter_brush for () {
  fn brush(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter5brushEv()};
    unsafe {_ZNK8QPainter5brushEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn FreeQPainter<T: QPainter_FreeQPainter>(&mut self, value: T) -> i32 {
    value.FreeQPainter(self);
    return 1;
  }
}

pub trait QPainter_FreeQPainter {
  fn FreeQPainter(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::FreeQPainter();
impl<'a> /*trait*/ QPainter_FreeQPainter for () {
  fn FreeQPainter(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainterD0Ev()};
    unsafe {_ZN8QPainterD0Ev()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn begin<T: QPainter_begin>(&mut self, value: T) -> i32 {
    value.begin(self);
    return 1;
  }
}

pub trait QPainter_begin {
  fn begin(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::begin(QPaintDevice * );
impl<'a> /*trait*/ QPainter_begin for (&'a mut QPaintDevice) {
  fn begin(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5beginEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainter5beginEP12QPaintDevice(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawRect(const QRect & rect);
impl<'a> /*trait*/ QPainter_drawRect for (&'a  QRect) {
  fn drawRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawRectERK5QRect(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawTextItem(const QPointF & p, const QTextItem & ti);
impl<'a> /*trait*/ QPainter_drawTextItem for (&'a  QPointF, &'a  QTextItem) {
  fn drawTextItem(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter12drawTextItemERK7QPointFRK9QTextItem(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn scale<T: QPainter_scale>(&mut self, value: T) -> i32 {
    value.scale(self);
    return 1;
  }
}

pub trait QPainter_scale {
  fn scale(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QPainter_scale for (f64, f64) {
  fn scale(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN8QPainter5scaleEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setWorldMatrix<T: QPainter_setWorldMatrix>(&mut self, value: T) -> i32 {
    value.setWorldMatrix(self);
    return 1;
  }
}

pub trait QPainter_setWorldMatrix {
  fn setWorldMatrix(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setWorldMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QPainter_setWorldMatrix for (&'a  QMatrix, i8) {
  fn setWorldMatrix(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter14setWorldMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QPainter14setWorldMatrixERK7QMatrixb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn clipPath<T: QPainter_clipPath>(&mut self, value: T) -> i32 {
    value.clipPath(self);
    return 1;
  }
}

pub trait QPainter_clipPath {
  fn clipPath(self, this: &mut QPainter) -> i32;
}

// proto: QPainterPath QPainter::clipPath();
impl<'a> /*trait*/ QPainter_clipPath for () {
  fn clipPath(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8clipPathEv()};
    unsafe {_ZNK8QPainter8clipPathEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn brushOrigin<T: QPainter_brushOrigin>(&mut self, value: T) -> i32 {
    value.brushOrigin(self);
    return 1;
  }
}

pub trait QPainter_brushOrigin {
  fn brushOrigin(self, this: &mut QPainter) -> i32;
}

// proto: QPoint QPainter::brushOrigin();
impl<'a> /*trait*/ QPainter_brushOrigin for () {
  fn brushOrigin(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter11brushOriginEv()};
    unsafe {_ZNK8QPainter11brushOriginEv()};
    return 1;
  }
}

// proto: void QPainter::drawConvexPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPolygonF) {
  fn drawConvexPolygon(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter17drawConvexPolygonERK9QPolygonF(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawEllipse(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_drawEllipse for (i32, i32, i32, i32) {
  fn drawEllipse(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter11drawEllipseEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter11drawEllipseEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QPainter::drawConvexPolygon(const QPolygon & polygon);
impl<'a> /*trait*/ QPainter_drawConvexPolygon for (&'a  QPolygon) {
  fn drawConvexPolygon(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17drawConvexPolygonERK8QPolygon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter17drawConvexPolygonERK8QPolygon(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPoints(const QPoint * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPoints for (&'a  QPoint, i32) {
  fn drawPoints(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10drawPointsEPK6QPointi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter10drawPointsEPK6QPointi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn background<T: QPainter_background>(&mut self, value: T) -> i32 {
    value.background(self);
    return 1;
  }
}

pub trait QPainter_background {
  fn background(self, this: &mut QPainter) -> i32;
}

// proto: const QBrush & QPainter::background();
impl<'a> /*trait*/ QPainter_background for () {
  fn background(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter10backgroundEv()};
    unsafe {_ZNK8QPainter10backgroundEv()};
    return 1;
  }
}

// proto: void QPainter::drawRoundRect(int x, int y, int w, int h, int , int );
impl<'a> /*trait*/ QPainter_drawRoundRect for (i32, i32, i32, i32, i32, i32) {
  fn drawRoundRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter13drawRoundRectEiiiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    unsafe {_ZN8QPainter13drawRoundRectEiiiiii(arg0, arg1, arg2, arg3, arg4, arg5)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn viewport<T: QPainter_viewport>(&mut self, value: T) -> i32 {
    value.viewport(self);
    return 1;
  }
}

pub trait QPainter_viewport {
  fn viewport(self, this: &mut QPainter) -> i32;
}

// proto: QRect QPainter::viewport();
impl<'a> /*trait*/ QPainter_viewport for () {
  fn viewport(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8viewportEv()};
    unsafe {_ZNK8QPainter8viewportEv()};
    return 1;
  }
}

// proto: void QPainter::drawArc(const QRect & , int a, int alen);
impl<'a> /*trait*/ QPainter_drawArc for (&'a  QRect, i32, i32) {
  fn drawArc(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawArcERK5QRectii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter7drawArcERK5QRectii(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fillPath<T: QPainter_fillPath>(&mut self, value: T) -> i32 {
    value.fillPath(self);
    return 1;
  }
}

pub trait QPainter_fillPath {
  fn fillPath(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::fillPath(const QPainterPath & path, const QBrush & brush);
impl<'a> /*trait*/ QPainter_fillPath for (&'a  QPainterPath, &'a  QBrush) {
  fn fillPath(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillPathERK12QPainterPathRK6QBrush(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawText(int x, int y, int w, int h, int flags, const QString & text, QRect * br);
impl<'a> /*trait*/ QPainter_drawText for (i32, i32, i32, i32, i32, &'a  QString, &'a mut QRect) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5.qclsinst  as *const c_void;
    let arg6 = self.6.qclsinst  as *mut c_void;
    unsafe {_ZN8QPainter8drawTextEiiiiiRK7QStringP5QRect(arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn matrixEnabled<T: QPainter_matrixEnabled>(&mut self, value: T) -> i32 {
    value.matrixEnabled(self);
    return 1;
  }
}

pub trait QPainter_matrixEnabled {
  fn matrixEnabled(self, this: &mut QPainter) -> i32;
}

// proto: bool QPainter::matrixEnabled();
impl<'a> /*trait*/ QPainter_matrixEnabled for () {
  fn matrixEnabled(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter13matrixEnabledEv()};
    unsafe {_ZNK8QPainter13matrixEnabledEv()};
    return 1;
  }
}

// proto: void QPainter::drawPolyline(const QPointF * points, int pointCount);
impl<'a> /*trait*/ QPainter_drawPolyline for (&'a  QPointF, i32) {
  fn drawPolyline(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12drawPolylineEPK7QPointFi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN8QPainter12drawPolylineEPK7QPointFi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setTransform<T: QPainter_setTransform>(&mut self, value: T) -> i32 {
    value.setTransform(self);
    return 1;
  }
}

pub trait QPainter_setTransform {
  fn setTransform(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setTransform(const QTransform & transform, bool combine);
impl<'a> /*trait*/ QPainter_setTransform for (&'a  QTransform, i8) {
  fn setTransform(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QPainter12setTransformERK10QTransformb(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::setPen(const QColor & color);
impl<'a> /*trait*/ QPainter_setPen for (&'a  QColor) {
  fn setPen(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6setPenERK6QColor()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter6setPenERK6QColor(arg0)};
    return 1;
  }
}

// proto: void QPainter::eraseRect(const QRect & );
impl<'a> /*trait*/ QPainter_eraseRect for (&'a  QRect) {
  fn eraseRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9eraseRectERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9eraseRectERK5QRect(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn window<T: QPainter_window>(&mut self, value: T) -> i32 {
    value.window(self);
    return 1;
  }
}

pub trait QPainter_window {
  fn window(self, this: &mut QPainter) -> i32;
}

// proto: QRect QPainter::window();
impl<'a> /*trait*/ QPainter_window for () {
  fn window(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter6windowEv()};
    unsafe {_ZNK8QPainter6windowEv()};
    return 1;
  }
}

// proto: void QPainter::drawImage(const QRect & r, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QRect, &'a  QImage) {
  fn drawImage(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK5QRectRK6QImage()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9drawImageERK5QRectRK6QImage(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn initFrom<T: QPainter_initFrom>(&mut self, value: T) -> i32 {
    value.initFrom(self);
    return 1;
  }
}

pub trait QPainter_initFrom {
  fn initFrom(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::initFrom(const QPaintDevice * device);
impl<'a> /*trait*/ QPainter_initFrom for (&'a  QPaintDevice) {
  fn initFrom(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8initFromEPK12QPaintDevice()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8initFromEPK12QPaintDevice(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn fontInfo<T: QPainter_fontInfo>(&mut self, value: T) -> i32 {
    value.fontInfo(self);
    return 1;
  }
}

pub trait QPainter_fontInfo {
  fn fontInfo(self, this: &mut QPainter) -> i32;
}

// proto: QFontInfo QPainter::fontInfo();
impl<'a> /*trait*/ QPainter_fontInfo for () {
  fn fontInfo(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter8fontInfoEv()};
    unsafe {_ZNK8QPainter8fontInfoEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn endNativePainting<T: QPainter_endNativePainting>(&mut self, value: T) -> i32 {
    value.endNativePainting(self);
    return 1;
  }
}

pub trait QPainter_endNativePainting {
  fn endNativePainting(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::endNativePainting();
impl<'a> /*trait*/ QPainter_endNativePainting for () {
  fn endNativePainting(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter17endNativePaintingEv()};
    unsafe {_ZN8QPainter17endNativePaintingEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setViewTransformEnabled<T: QPainter_setViewTransformEnabled>(&mut self, value: T) -> i32 {
    value.setViewTransformEnabled(self);
    return 1;
  }
}

pub trait QPainter_setViewTransformEnabled {
  fn setViewTransformEnabled(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setViewTransformEnabled(bool enable);
impl<'a> /*trait*/ QPainter_setViewTransformEnabled for (i8) {
  fn setViewTransformEnabled(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter23setViewTransformEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN8QPainter23setViewTransformEnabledEb(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawPoint(const QPointF & pt);
impl<'a> /*trait*/ QPainter_drawPoint for (&'a  QPointF) {
  fn drawPoint(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawPointERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9drawPointERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn setOpacity<T: QPainter_setOpacity>(&mut self, value: T) -> i32 {
    value.setOpacity(self);
    return 1;
  }
}

pub trait QPainter_setOpacity {
  fn setOpacity(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::setOpacity(qreal opacity);
impl<'a> /*trait*/ QPainter_setOpacity for (f64) {
  fn setOpacity(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter10setOpacityEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN8QPainter10setOpacityEd(arg0)};
    return 1;
  }
}

// proto: void QPainter::fillRect(const QRectF & , const QColor & color);
impl<'a> /*trait*/ QPainter_fillRect for (&'a  QRectF, &'a  QColor) {
  fn fillRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8fillRectERK6QRectFRK6QColor()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8fillRectERK6QRectFRK6QColor(arg0, arg1)};
    return 1;
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

// proto: void QPainter::translate(const QPointF & offset);
impl<'a> /*trait*/ QPainter_translate for (&'a  QPointF) {
  fn translate(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9translateERK7QPointF(arg0)};
    return 1;
  }
}

// proto: void QPainter::drawText(const QPointF & p, const QString & s);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QPointF, &'a  QString) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK7QPointFRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawTextERK7QPointFRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawImage(const QPointF & p, const QImage & image);
impl<'a> /*trait*/ QPainter_drawImage for (&'a  QPointF, &'a  QImage) {
  fn drawImage(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9drawImageERK7QPointFRK6QImage()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter9drawImageERK7QPointFRK6QImage(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn pen<T: QPainter_pen>(&mut self, value: T) -> i32 {
    value.pen(self);
    return 1;
  }
}

pub trait QPainter_pen {
  fn pen(self, this: &mut QPainter) -> i32;
}

// proto: const QPen & QPainter::pen();
impl<'a> /*trait*/ QPainter_pen for () {
  fn pen(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter3penEv()};
    unsafe {_ZNK8QPainter3penEv()};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn rotate<T: QPainter_rotate>(&mut self, value: T) -> i32 {
    value.rotate(self);
    return 1;
  }
}

pub trait QPainter_rotate {
  fn rotate(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::rotate(qreal a);
impl<'a> /*trait*/ QPainter_rotate for (f64) {
  fn rotate(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter6rotateEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN8QPainter6rotateEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn clipBoundingRect<T: QPainter_clipBoundingRect>(&mut self, value: T) -> i32 {
    value.clipBoundingRect(self);
    return 1;
  }
}

pub trait QPainter_clipBoundingRect {
  fn clipBoundingRect(self, this: &mut QPainter) -> i32;
}

// proto: QRectF QPainter::clipBoundingRect();
impl<'a> /*trait*/ QPainter_clipBoundingRect for () {
  fn clipBoundingRect(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPainter16clipBoundingRectEv()};
    unsafe {_ZNK8QPainter16clipBoundingRectEv()};
    return 1;
  }
}

// proto: void QPainter::drawLine(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPainter_drawLine for (&'a  QPoint, &'a  QPoint) {
  fn drawLine(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawLineERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawLineERK6QPointS2_(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::drawPie(const QRectF & rect, int a, int alen);
impl<'a> /*trait*/ QPainter_drawPie for (&'a  QRectF, i32, i32) {
  fn drawPie(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter7drawPieERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN8QPainter7drawPieERK6QRectFii(arg0, arg1, arg2)};
    return 1;
  }
}

// proto: void QPainter::drawText(const QPoint & p, const QString & s);
impl<'a> /*trait*/ QPainter_drawText for (&'a  QPoint, &'a  QString) {
  fn drawText(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter8drawTextERK6QPointRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN8QPainter8drawTextERK6QPointRK7QString(arg0, arg1)};
    return 1;
  }
}

// proto: void QPainter::setWindow(int x, int y, int w, int h);
impl<'a> /*trait*/ QPainter_setWindow for (i32, i32, i32, i32) {
  fn setWindow(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter9setWindowEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN8QPainter9setWindowEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QPainter {
  pub fn beginNativePainting<T: QPainter_beginNativePainting>(&mut self, value: T) -> i32 {
    value.beginNativePainting(self);
    return 1;
  }
}

pub trait QPainter_beginNativePainting {
  fn beginNativePainting(self, this: &mut QPainter) -> i32;
}

// proto: void QPainter::beginNativePainting();
impl<'a> /*trait*/ QPainter_beginNativePainting for () {
  fn beginNativePainting(self, this: &mut QPainter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPainter19beginNativePaintingEv()};
    unsafe {_ZN8QPainter19beginNativePaintingEv()};
    return 1;
  }
}

