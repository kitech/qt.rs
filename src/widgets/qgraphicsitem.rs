// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qgraphicsitem.h
// dst-file: /src/widgets/qgraphicsitem.rs
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
// use super::qgraphicsitem::QGraphicsObject; // 773
use std::ops::Deref;
use super::super::gui::qtextcursor::QTextCursor; // 771
use super::super::gui::qfont::QFont; // 771
use super::super::core::qstring::QString; // 771
// use super::qgraphicsitem::QGraphicsItem; // 773
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::gui::qtextdocument::QTextDocument; // 771
use super::super::gui::qpainter::QPainter; // 771
use super::qstyleoption::QStyleOptionGraphicsItem; // 773
use super::qwidget::QWidget; // 773
use super::super::gui::qcolor::QColor; // 771
use super::super::gui::qpainterpath::QPainterPath; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::gui::qpixmap::QPixmap; // 771
// use super::qgraphicsitem::QAbstractGraphicsShapeItem; // 773
use super::super::gui::qpolygon::QPolygonF; // 771
use super::super::gui::qpen::QPen; // 771
use super::super::core::qline::QLineF; // 771
use super::super::gui::qbrush::QBrush; // 771
use super::super::gui::qtransform::QTransform; // 771
use super::super::gui::qregion::QRegion; // 771
use super::qgraphicseffect::QGraphicsEffect; // 773
use super::super::gui::qmatrix::QMatrix; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qvariant::QVariant; // 771
// use super::qgraphicsitem::QGraphicsItemGroup; // 773
use super::super::gui::qcursor::QCursor; // 771
use super::super::core::qobject::QObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsTextItem_Class_Size() -> c_int;
  // proto:  bool QGraphicsTextItem::openExternalLinks();
  fn C_ZNK17QGraphicsTextItem17openExternalLinksEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QGraphicsTextItem::textWidth();
  fn C_ZNK17QGraphicsTextItem9textWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsTextItem::setTextWidth(qreal width);
  fn C_ZN17QGraphicsTextItem12setTextWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
  fn C_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QGraphicsTextItem::type();
  fn C_ZNK17QGraphicsTextItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QFont QGraphicsTextItem::font();
  fn C_ZNK17QGraphicsTextItem4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsTextItem::QGraphicsTextItem(const QString & text, QGraphicsItem * parent);
  fn C_ZN17QGraphicsTextItemC2ERK7QStringP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  const QMetaObject * QGraphicsTextItem::metaObject();
  fn C_ZNK17QGraphicsTextItem10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsTextItem::setOpenExternalLinks(bool open);
  fn C_ZN17QGraphicsTextItem20setOpenExternalLinksEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGraphicsTextItem::setTabChangesFocus(bool b);
  fn C_ZN17QGraphicsTextItem18setTabChangesFocusEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QGraphicsTextItem::toHtml();
  fn C_ZNK17QGraphicsTextItem6toHtmlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsTextItem::setDocument(QTextDocument * document);
  fn C_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsTextItem::setPlainText(const QString & text);
  fn C_ZN17QGraphicsTextItem12setPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsTextItem::setFont(const QFont & font);
  fn C_ZN17QGraphicsTextItem7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
  fn C_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QColor QGraphicsTextItem::defaultTextColor();
  fn C_ZNK17QGraphicsTextItem16defaultTextColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsTextItem::~QGraphicsTextItem();
  fn C_ZN17QGraphicsTextItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsTextItem::shape();
  fn C_ZNK17QGraphicsTextItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextCursor QGraphicsTextItem::textCursor();
  fn C_ZNK17QGraphicsTextItem10textCursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsTextItem::boundingRect();
  fn C_ZNK17QGraphicsTextItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QGraphicsTextItem::toPlainText();
  fn C_ZNK17QGraphicsTextItem11toPlainTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsTextItem::setHtml(const QString & html);
  fn C_ZN17QGraphicsTextItem7setHtmlERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGraphicsTextItem::tabChangesFocus();
  fn C_ZNK17QGraphicsTextItem15tabChangesFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsTextItem::QGraphicsTextItem(QGraphicsItem * parent);
  fn C_ZN17QGraphicsTextItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  QTextDocument * QGraphicsTextItem::document();
  fn C_ZNK17QGraphicsTextItem8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QPainterPath QGraphicsTextItem::opaqueArea();
  fn C_ZNK17QGraphicsTextItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsTextItem::contains(const QPointF & point);
  fn C_ZNK17QGraphicsTextItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsTextItem::adjustSize();
  fn C_ZN17QGraphicsTextItem10adjustSizeEv(qthis: u64 /* *mut c_void*/);
  fn QGraphicsPixmapItem_Class_Size() -> c_int;
  // proto:  void QGraphicsPixmapItem::QGraphicsPixmapItem(QGraphicsItem * parent);
  fn C_ZN19QGraphicsPixmapItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsPixmapItem::QGraphicsPixmapItem(const QPixmap & pixmap, QGraphicsItem * parent);
  fn C_ZN19QGraphicsPixmapItemC2ERK7QPixmapP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QGraphicsPixmapItem::~QGraphicsPixmapItem();
  fn C_ZN19QGraphicsPixmapItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsPixmapItem::opaqueArea();
  fn C_ZNK19QGraphicsPixmapItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QGraphicsPixmapItem::type();
  fn C_ZNK19QGraphicsPixmapItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPainterPath QGraphicsPixmapItem::shape();
  fn C_ZNK19QGraphicsPixmapItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPixmap QGraphicsPixmapItem::pixmap();
  fn C_ZNK19QGraphicsPixmapItem6pixmapEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
  fn C_ZN19QGraphicsPixmapItem9setOffsetEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QPointF QGraphicsPixmapItem::offset();
  fn C_ZNK19QGraphicsPixmapItem6offsetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsPixmapItem::boundingRect();
  fn C_ZNK19QGraphicsPixmapItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsPixmapItem::contains(const QPointF & point);
  fn C_ZNK19QGraphicsPixmapItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
  fn C_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsPixmapItem::setOffset(const QPointF & offset);
  fn C_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QGraphicsRectItem_Class_Size() -> c_int;
  // proto:  bool QGraphicsRectItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QRectF QGraphicsRectItem::boundingRect();
  fn C_ZNK17QGraphicsRectItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGraphicsRectItem::type();
  fn C_ZNK17QGraphicsRectItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRectF QGraphicsRectItem::rect();
  fn C_ZNK17QGraphicsRectItem4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QGraphicsRectItem::shape();
  fn C_ZNK17QGraphicsRectItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsRectItem::~QGraphicsRectItem();
  fn C_ZN17QGraphicsRectItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsRectItem::QGraphicsRectItem(const QRectF & rect, QGraphicsItem * parent);
  fn C_ZN17QGraphicsRectItemC2ERK6QRectFP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QPainterPath QGraphicsRectItem::opaqueArea();
  fn C_ZNK17QGraphicsRectItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsRectItem::setRect(const QRectF & rect);
  fn C_ZN17QGraphicsRectItem7setRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsRectItem::setRect(qreal x, qreal y, qreal w, qreal h);
  fn C_ZN17QGraphicsRectItem7setRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsRectItem::QGraphicsRectItem(QGraphicsItem * parent);
  fn C_ZN17QGraphicsRectItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  bool QGraphicsRectItem::contains(const QPointF & point);
  fn C_ZNK17QGraphicsRectItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsRectItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsRectItem::QGraphicsRectItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
  fn C_ZN17QGraphicsRectItemC2EddddP13QGraphicsItem(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> u64;
  fn QGraphicsEllipseItem_Class_Size() -> c_int;
  // proto:  void QGraphicsEllipseItem::setStartAngle(int angle);
  fn C_ZN20QGraphicsEllipseItem13setStartAngleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QGraphicsEllipseItem::contains(const QPointF & point);
  fn C_ZNK20QGraphicsEllipseItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
  fn C_ZN20QGraphicsEllipseItemC2ERK6QRectFP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QGraphicsEllipseItem::setRect(const QRectF & rect);
  fn C_ZN20QGraphicsEllipseItem7setRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QRectF QGraphicsEllipseItem::rect();
  fn C_ZNK20QGraphicsEllipseItem4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGraphicsEllipseItem::spanAngle();
  fn C_ZNK20QGraphicsEllipseItem9spanAngleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QGraphicsEllipseItem::startAngle();
  fn C_ZNK20QGraphicsEllipseItem10startAngleEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
  fn C_ZN20QGraphicsEllipseItemC2EddddP13QGraphicsItem(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> u64;
  // proto:  void QGraphicsEllipseItem::setRect(qreal x, qreal y, qreal w, qreal h);
  fn C_ZN20QGraphicsEllipseItem7setRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsEllipseItem::setSpanAngle(int angle);
  fn C_ZN20QGraphicsEllipseItem12setSpanAngleEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QGraphicsEllipseItem::type();
  fn C_ZNK20QGraphicsEllipseItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRectF QGraphicsEllipseItem::boundingRect();
  fn C_ZNK20QGraphicsEllipseItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QGraphicsEllipseItem::shape();
  fn C_ZNK20QGraphicsEllipseItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsEllipseItem::~QGraphicsEllipseItem();
  fn C_ZN20QGraphicsEllipseItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(QGraphicsItem * parent);
  fn C_ZN20QGraphicsEllipseItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  QPainterPath QGraphicsEllipseItem::opaqueArea();
  fn C_ZNK20QGraphicsEllipseItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QGraphicsPolygonItem_Class_Size() -> c_int;
  // proto:  QPainterPath QGraphicsPolygonItem::shape();
  fn C_ZNK20QGraphicsPolygonItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem * parent);
  fn C_ZN20QGraphicsPolygonItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  QRectF QGraphicsPolygonItem::boundingRect();
  fn C_ZNK20QGraphicsPolygonItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGraphicsPolygonItem::type();
  fn C_ZNK20QGraphicsPolygonItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsPolygonItem::~QGraphicsPolygonItem();
  fn C_ZN20QGraphicsPolygonItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPolygonF QGraphicsPolygonItem::polygon();
  fn C_ZNK20QGraphicsPolygonItem7polygonEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QGraphicsPolygonItem::opaqueArea();
  fn C_ZNK20QGraphicsPolygonItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(const QPolygonF & polygon, QGraphicsItem * parent);
  fn C_ZN20QGraphicsPolygonItemC2ERK9QPolygonFP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  bool QGraphicsPolygonItem::contains(const QPointF & point);
  fn C_ZNK20QGraphicsPolygonItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
  fn C_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QGraphicsPathItem_Class_Size() -> c_int;
  // proto:  void QGraphicsPathItem::setPath(const QPainterPath & path);
  fn C_ZN17QGraphicsPathItem7setPathERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsPathItem::QGraphicsPathItem(const QPainterPath & path, QGraphicsItem * parent);
  fn C_ZN17QGraphicsPathItemC2ERK12QPainterPathP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  bool QGraphicsPathItem::contains(const QPointF & point);
  fn C_ZNK17QGraphicsPathItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QRectF QGraphicsPathItem::boundingRect();
  fn C_ZNK17QGraphicsPathItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QGraphicsPathItem::type();
  fn C_ZNK17QGraphicsPathItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QPainterPath QGraphicsPathItem::opaqueArea();
  fn C_ZNK17QGraphicsPathItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QGraphicsPathItem::path();
  fn C_ZNK17QGraphicsPathItem4pathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsPathItem::~QGraphicsPathItem();
  fn C_ZN17QGraphicsPathItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsPathItem::shape();
  fn C_ZNK17QGraphicsPathItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsPathItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK17QGraphicsPathItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsPathItem::QGraphicsPathItem(QGraphicsItem * parent);
  fn C_ZN17QGraphicsPathItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsPathItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN17QGraphicsPathItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  fn QGraphicsLineItem_Class_Size() -> c_int;
  // proto:  void QGraphicsLineItem::setPen(const QPen & pen);
  fn C_ZN17QGraphicsLineItem6setPenERK4QPen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsLineItem::QGraphicsLineItem(QGraphicsItem * parent);
  fn C_ZN17QGraphicsLineItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsLineItem::QGraphicsLineItem(const QLineF & line, QGraphicsItem * parent);
  fn C_ZN17QGraphicsLineItemC2ERK6QLineFP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QLineF QGraphicsLineItem::line();
  fn C_ZNK17QGraphicsLineItem4lineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPainterPath QGraphicsLineItem::opaqueArea();
  fn C_ZNK17QGraphicsLineItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
  fn C_ZN17QGraphicsLineItem7setLineEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  QRectF QGraphicsLineItem::boundingRect();
  fn C_ZNK17QGraphicsLineItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPen QGraphicsLineItem::pen();
  fn C_ZNK17QGraphicsLineItem3penEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsLineItem::setLine(const QLineF & line);
  fn C_ZN17QGraphicsLineItem7setLineERK6QLineF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsLineItem::shape();
  fn C_ZNK17QGraphicsLineItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  int QGraphicsLineItem::type();
  fn C_ZNK17QGraphicsLineItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsLineItem::QGraphicsLineItem(qreal x1, qreal y1, qreal x2, qreal y2, QGraphicsItem * parent);
  fn C_ZN17QGraphicsLineItemC2EddddP13QGraphicsItem(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: *mut c_void) -> u64;
  // proto:  bool QGraphicsLineItem::contains(const QPointF & point);
  fn C_ZNK17QGraphicsLineItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsLineItem::~QGraphicsLineItem();
  fn C_ZN17QGraphicsLineItemD2Ev(qthis: u64 /* *mut c_void*/);
  fn QGraphicsItemGroup_Class_Size() -> c_int;
  // proto:  bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QGraphicsItemGroup::~QGraphicsItemGroup();
  fn C_ZN18QGraphicsItemGroupD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItemGroup::QGraphicsItemGroup(QGraphicsItem * parent);
  fn C_ZN18QGraphicsItemGroupC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  int QGraphicsItemGroup::type();
  fn C_ZNK18QGraphicsItemGroup4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QRectF QGraphicsItemGroup::boundingRect();
  fn C_ZNK18QGraphicsItemGroup12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
  fn C_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
  fn C_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsItemGroup::opaqueArea();
  fn C_ZNK18QGraphicsItemGroup10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QAbstractGraphicsShapeItem_Class_Size() -> c_int;
  // proto:  bool QAbstractGraphicsShapeItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK26QAbstractGraphicsShapeItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QBrush QAbstractGraphicsShapeItem::brush();
  fn C_ZNK26QAbstractGraphicsShapeItem5brushEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractGraphicsShapeItem::QAbstractGraphicsShapeItem(QGraphicsItem * parent);
  fn C_ZN26QAbstractGraphicsShapeItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  QPainterPath QAbstractGraphicsShapeItem::opaqueArea();
  fn C_ZNK26QAbstractGraphicsShapeItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractGraphicsShapeItem::setBrush(const QBrush & brush);
  fn C_ZN26QAbstractGraphicsShapeItem8setBrushERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractGraphicsShapeItem::setPen(const QPen & pen);
  fn C_ZN26QAbstractGraphicsShapeItem6setPenERK4QPen(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPen QAbstractGraphicsShapeItem::pen();
  fn C_ZNK26QAbstractGraphicsShapeItem3penEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractGraphicsShapeItem::~QAbstractGraphicsShapeItem();
  fn C_ZN26QAbstractGraphicsShapeItemD2Ev(qthis: u64 /* *mut c_void*/);
  fn QGraphicsItem_Class_Size() -> c_int;
  // proto:  QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
  fn C_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
  fn C_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
  fn C_ZNK13QGraphicsItem9focusItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
  fn C_ZNK13QGraphicsItem12parentObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
  fn C_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsItem::ungrabMouse();
  fn C_ZN13QGraphicsItem11ungrabMouseEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QGraphicsItem::type();
  fn C_ZNK13QGraphicsItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QGraphicsItem::isSelected();
  fn C_ZNK13QGraphicsItem10isSelectedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem11mapFromItemEPKS_dddd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
  fn C_ZNK13QGraphicsItem12parentWidgetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::resetTransform();
  fn C_ZN13QGraphicsItem14resetTransformEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
  fn C_ZNK13QGraphicsItem14boundingRegionERK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  bool QGraphicsItem::isActive();
  fn C_ZNK13QGraphicsItem8isActiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::QGraphicsItem(QGraphicsItem * parent);
  fn C_ZN13QGraphicsItemC2EPS_(arg0: *mut c_void) -> u64;
  // proto:  QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
  fn C_ZNK13QGraphicsItem11mapToParentERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::isWidget();
  fn C_ZNK13QGraphicsItem8isWidgetEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
  fn C_ZN13QGraphicsItem13setParentItemEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
  fn C_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsWidget * QGraphicsItem::window();
  fn C_ZNK13QGraphicsItem6windowEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QGraphicsItem::scenePos();
  fn C_ZNK13QGraphicsItem8scenePosEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsItem::handlesChildEvents();
  fn C_ZNK13QGraphicsItem18handlesChildEventsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
  fn C_ZN13QGraphicsItem10setOpacityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QTransform QGraphicsItem::sceneTransform();
  fn C_ZNK13QGraphicsItem14sceneTransformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItem::setZValue(qreal z);
  fn C_ZN13QGraphicsItem9setZValueEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QRectF & rect);
  fn C_ZNK13QGraphicsItem13mapFromParentERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QGraphicsItem::mapFromParent(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem13mapFromParentEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem10isObscuredEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> c_char;
  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
  fn C_ZN13QGraphicsItem23installSceneEventFilterEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsItem::setY(qreal y);
  fn C_ZN13QGraphicsItem4setYEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
  fn C_ZNK13QGraphicsItem10parentItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::clearFocus();
  fn C_ZN13QGraphicsItem10clearFocusEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGraphicsItem::isWindow();
  fn C_ZNK13QGraphicsItem8isWindowEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPointF QGraphicsItem::transformOriginPoint();
  fn C_ZNK13QGraphicsItem20transformOriginPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::boundingRect();
  fn C_ZNK13QGraphicsItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
  fn C_ZNK13QGraphicsItem20childrenBoundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsItem::isObscured(const QRectF & rect);
  fn C_ZNK13QGraphicsItem10isObscuredERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
  fn C_ZNK13QGraphicsItem12mapFromSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::hasCursor();
  fn C_ZNK13QGraphicsItem9hasCursorEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
  fn C_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapToParent(const QPainterPath & path);
  fn C_ZNK13QGraphicsItem11mapToParentERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn C_ZN13QGraphicsItem13ensureVisibleEddddii(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int);
  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem9mapToItemEPKS_dddd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y);
  fn C_ZNK13QGraphicsItem9mapToItemEPKS_dd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
  fn C_ZNK13QGraphicsItem15mapRectToParentERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
  fn C_ZN13QGraphicsItem10setToolTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QGraphicsItem::rotation();
  fn C_ZNK13QGraphicsItem8rotationEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QGraphicsScene * QGraphicsItem::scene();
  fn C_ZNK13QGraphicsItem5sceneEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPainterPath & path);
  fn C_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem15mapRectToParentEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn C_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
  fn C_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
  fn C_ZN13QGraphicsItem13setFocusProxyEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGraphicsItem::acceptDrops();
  fn C_ZNK13QGraphicsItem11acceptDropsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPointF QGraphicsItem::mapToParent(const QPointF & point);
  fn C_ZNK13QGraphicsItem11mapToParentERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
  fn C_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
  fn C_ZNK13QGraphicsItem14focusScopeItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
  fn C_ZN13QGraphicsItem22removeSceneEventFilterEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
  fn C_ZNK13QGraphicsItem10focusProxyEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPointF & point);
  fn C_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
  fn C_ZNK13QGraphicsItem17sceneBoundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItem::~QGraphicsItem();
  fn C_ZN13QGraphicsItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::setX(qreal x);
  fn C_ZN13QGraphicsItem4setXEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
  fn C_ZN13QGraphicsItem6updateEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QGraphicsItem::setSelected(bool selected);
  fn C_ZN13QGraphicsItem11setSelectedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
  fn C_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
  fn C_ZN13QGraphicsItem11stackBeforeEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y);
  fn C_ZNK13QGraphicsItem11mapFromItemEPKS_dd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::resetMatrix();
  fn C_ZN13QGraphicsItem11resetMatrixEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsItem::opaqueArea();
  fn C_ZNK13QGraphicsItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItem::unsetCursor();
  fn C_ZN13QGraphicsItem11unsetCursorEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QGraphicsItem::mapFromParent(qreal x, qreal y);
  fn C_ZNK13QGraphicsItem13mapFromParentEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
  fn C_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double) -> *mut c_void;
  // proto:  qreal QGraphicsItem::scale();
  fn C_ZNK13QGraphicsItem5scaleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
  fn C_ZN13QGraphicsItem28setBoundingRegionGranularityEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
  fn C_ZN13QGraphicsItem14setAcceptDropsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QPolygonF QGraphicsItem::mapFromScene(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem12mapFromSceneEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::ungrabKeyboard();
  fn C_ZN13QGraphicsItem14ungrabKeyboardEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::setEnabled(bool enabled);
  fn C_ZN13QGraphicsItem10setEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
  fn C_ZNK13QGraphicsItem14graphicsEffectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGraphicsItem::acceptHoverEvents();
  fn C_ZNK13QGraphicsItem17acceptHoverEventsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
  fn C_ZNK13QGraphicsItem14topLevelWidgetEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
  fn C_ZNK13QGraphicsItem15transformationsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem10mapToSceneEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToScene(qreal x, qreal y);
  fn C_ZNK13QGraphicsItem10mapToSceneEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem16mapRectFromSceneEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::advance(int phase);
  fn C_ZN13QGraphicsItem7advanceEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QMatrix QGraphicsItem::sceneMatrix();
  fn C_ZNK13QGraphicsItem11sceneMatrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
  fn C_ZN13QGraphicsItem21setFiltersChildEventsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QPolygonF QGraphicsItem::mapToScene(const QPolygonF & polygon);
  fn C_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
  fn C_ZNK13QGraphicsItem13itemTransformEPKS_Pb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> *mut c_void;
  // proto:  void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
  fn C_ZN13QGraphicsItem23setTransformOriginPointEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
  fn C_ZN13QGraphicsItem6moveByEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QPolygonF & polygon);
  fn C_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
  fn C_ZNK13QGraphicsItem5groupEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsItem::shape();
  fn C_ZNK13QGraphicsItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapFromScene(qreal x, qreal y);
  fn C_ZNK13QGraphicsItem12mapFromSceneEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
  fn C_ZN13QGraphicsItem6scrollEddRK6QRectF(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: *mut c_void);
  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK13QGraphicsItem12isObscuredByEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QPointF QGraphicsItem::mapFromParent(const QPointF & point);
  fn C_ZNK13QGraphicsItem13mapFromParentERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
  fn C_ZN13QGraphicsItem7setDataEiRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
  fn C_ZNK13QGraphicsItem18commonAncestorItemEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsItem::mapFromScene(const QPainterPath & path);
  fn C_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsItem::mapToScene(const QPainterPath & path);
  fn C_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QGraphicsItem::mapToParent(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem11mapToParentEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
  fn C_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem17mapRectFromParentEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::show();
  fn C_ZN13QGraphicsItem4showEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
  fn C_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  qreal QGraphicsItem::y();
  fn C_ZNK13QGraphicsItem1yEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QGraphicsItem::mapFromScene(const QPointF & point);
  fn C_ZNK13QGraphicsItem12mapFromSceneERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::hasFocus();
  fn C_ZNK13QGraphicsItem8hasFocusEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPainterPath QGraphicsItem::clipPath();
  fn C_ZNK13QGraphicsItem8clipPathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
  fn C_ZN13QGraphicsItem6setPosEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  bool QGraphicsItem::isEnabled();
  fn C_ZNK13QGraphicsItem9isEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QGraphicsItem::contains(const QPointF & point);
  fn C_ZNK13QGraphicsItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QGraphicsItem::isPanel();
  fn C_ZNK13QGraphicsItem7isPanelEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QGraphicsItem::filtersChildEvents();
  fn C_ZNK13QGraphicsItem18filtersChildEventsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::grabKeyboard();
  fn C_ZN13QGraphicsItem12grabKeyboardEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPainterPath & path);
  fn C_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setActive(bool active);
  fn C_ZN13QGraphicsItem9setActiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
  fn C_ZN13QGraphicsItem16toGraphicsObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPolygonF & polygon);
  fn C_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
  fn C_ZN13QGraphicsItem21setHandlesChildEventsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QPolygonF & polygon);
  fn C_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToParent(qreal x, qreal y);
  fn C_ZNK13QGraphicsItem11mapToParentEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
  fn C_ZN13QGraphicsItem9setMatrixERK7QMatrixb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QGraphicsItem::update(const QRectF & rect);
  fn C_ZN13QGraphicsItem6updateERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPolygonF & polygon);
  fn C_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTransform QGraphicsItem::transform();
  fn C_ZNK13QGraphicsItem9transformEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QVariant QGraphicsItem::data(int key);
  fn C_ZNK13QGraphicsItem4dataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QGraphicsItem::hide();
  fn C_ZN13QGraphicsItem4hideEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGraphicsItem::isUnderMouse();
  fn C_ZNK13QGraphicsItem12isUnderMouseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
  fn C_ZN13QGraphicsItem20setAcceptTouchEventsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
  fn C_ZN13QGraphicsItem20setAcceptHoverEventsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
  fn C_ZNK13QGraphicsItem10childItemsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
  fn C_ZNK13QGraphicsItem12isAncestorOfEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  qreal QGraphicsItem::opacity();
  fn C_ZNK13QGraphicsItem7opacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
  fn C_ZNK13QGraphicsItem11isVisibleToEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QGraphicsItem::toolTip();
  fn C_ZNK13QGraphicsItem7toolTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QCursor QGraphicsItem::cursor();
  fn C_ZNK13QGraphicsItem6cursorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QGraphicsItem::mapToScene(const QPointF & point);
  fn C_ZNK13QGraphicsItem10mapToSceneERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QGraphicsItem::zValue();
  fn C_ZNK13QGraphicsItem6zValueEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QMatrix QGraphicsItem::matrix();
  fn C_ZNK13QGraphicsItem6matrixEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
  fn C_ZNK13QGraphicsItem14mapRectToSceneEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsItem::setPos(const QPointF & pos);
  fn C_ZN13QGraphicsItem6setPosERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsItem * QGraphicsItem::panel();
  fn C_ZNK13QGraphicsItem5panelEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGraphicsItem::isClipped();
  fn C_ZNK13QGraphicsItem9isClippedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
  fn C_ZNK13QGraphicsItem12topLevelItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  QPolygonF QGraphicsItem::mapToScene(const QRectF & rect);
  fn C_ZNK13QGraphicsItem10mapToSceneERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsItem::setScale(qreal scale);
  fn C_ZN13QGraphicsItem8setScaleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
  fn C_ZN13QGraphicsItem9setCursorERK7QCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGraphicsItem::isVisible();
  fn C_ZNK13QGraphicsItem9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QPointF QGraphicsItem::pos();
  fn C_ZNK13QGraphicsItem3posEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
  fn C_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  qreal QGraphicsItem::effectiveOpacity();
  fn C_ZNK13QGraphicsItem16effectiveOpacityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn C_ZN13QGraphicsItem13ensureVisibleERK6QRectFii(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  qreal QGraphicsItem::boundingRegionGranularity();
  fn C_ZNK13QGraphicsItem25boundingRegionGranularityEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QGraphicsItem::x();
  fn C_ZNK13QGraphicsItem1xEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::grabMouse();
  fn C_ZN13QGraphicsItem9grabMouseEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItem::setVisible(bool visible);
  fn C_ZN13QGraphicsItem10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGraphicsItem::setRotation(qreal angle);
  fn C_ZN13QGraphicsItem11setRotationEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
  fn C_ZNK13QGraphicsItem15deviceTransformERK10QTransform(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsItem::acceptTouchEvents();
  fn C_ZNK13QGraphicsItem17acceptTouchEventsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
  fn C_ZN13QGraphicsItem12setTransformERK10QTransformb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  QPolygonF QGraphicsItem::mapToParent(const QRectF & rect);
  fn C_ZNK13QGraphicsItem11mapToParentERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  fn QGraphicsObject_Class_Size() -> c_int;
  // proto:  void QGraphicsObject::QGraphicsObject(QGraphicsItem * parent);
  fn C_ZN15QGraphicsObjectC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsObject::~QGraphicsObject();
  fn C_ZN15QGraphicsObjectD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QGraphicsObject::metaObject();
  fn C_ZNK15QGraphicsObject10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QGraphicsSimpleTextItem_Class_Size() -> c_int;
  // proto:  int QGraphicsSimpleTextItem::type();
  fn C_ZNK23QGraphicsSimpleTextItem4typeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QFont QGraphicsSimpleTextItem::font();
  fn C_ZNK23QGraphicsSimpleTextItem4fontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn C_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QGraphicsSimpleTextItem::~QGraphicsSimpleTextItem();
  fn C_ZN23QGraphicsSimpleTextItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
  fn C_ZN23QGraphicsSimpleTextItem7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QGraphicsSimpleTextItem::text();
  fn C_ZNK23QGraphicsSimpleTextItem4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(const QString & text, QGraphicsItem * parent);
  fn C_ZN23QGraphicsSimpleTextItemC2ERK7QStringP13QGraphicsItem(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
  fn C_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QPainterPath QGraphicsSimpleTextItem::shape();
  fn C_ZNK23QGraphicsSimpleTextItem5shapeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(QGraphicsItem * parent);
  fn C_ZN23QGraphicsSimpleTextItemC2EP13QGraphicsItem(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
  fn C_ZN23QGraphicsSimpleTextItem7setFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
  fn C_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
  fn C_ZNK23QGraphicsSimpleTextItem12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
  fn C_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  fn QGraphicsTextItem_SlotProxy_connect__ZN17QGraphicsTextItem11linkHoveredERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsTextItem_SlotProxy_connect__ZN17QGraphicsTextItem13linkActivatedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8yChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14opacityChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14visibleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject15childrenChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8zChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject12widthChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject15rotationChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14enabledChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject12scaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject13heightChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject13parentChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8xChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsTextItem)=1
#[derive(Default)]
pub struct QGraphicsTextItem {
  qbase: QGraphicsObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _linkActivated: QGraphicsTextItem_linkActivated_signal,
  pub _linkHovered: QGraphicsTextItem_linkHovered_signal,
}

// class sizeof(QGraphicsPixmapItem)=1
#[derive(Default)]
pub struct QGraphicsPixmapItem {
  qbase: QGraphicsItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsRectItem)=1
#[derive(Default)]
pub struct QGraphicsRectItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsEllipseItem)=1
#[derive(Default)]
pub struct QGraphicsEllipseItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsPolygonItem)=1
#[derive(Default)]
pub struct QGraphicsPolygonItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsPathItem)=1
#[derive(Default)]
pub struct QGraphicsPathItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsLineItem)=1
#[derive(Default)]
pub struct QGraphicsLineItem {
  qbase: QGraphicsItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsItemGroup)=1
#[derive(Default)]
pub struct QGraphicsItemGroup {
  qbase: QGraphicsItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractGraphicsShapeItem)=1
#[derive(Default)]
pub struct QAbstractGraphicsShapeItem {
  qbase: QGraphicsItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsItem)=1
#[derive(Default)]
pub struct QGraphicsItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsObject)=1
#[derive(Default)]
pub struct QGraphicsObject {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _childrenChanged: QGraphicsObject_childrenChanged_signal,
  pub _parentChanged: QGraphicsObject_parentChanged_signal,
  pub _heightChanged: QGraphicsObject_heightChanged_signal,
  pub _zChanged: QGraphicsObject_zChanged_signal,
  pub _visibleChanged: QGraphicsObject_visibleChanged_signal,
  pub _yChanged: QGraphicsObject_yChanged_signal,
  pub _widthChanged: QGraphicsObject_widthChanged_signal,
  pub _opacityChanged: QGraphicsObject_opacityChanged_signal,
  pub _rotationChanged: QGraphicsObject_rotationChanged_signal,
  pub _enabledChanged: QGraphicsObject_enabledChanged_signal,
  pub _xChanged: QGraphicsObject_xChanged_signal,
  pub _scaleChanged: QGraphicsObject_scaleChanged_signal,
}

// class sizeof(QGraphicsSimpleTextItem)=1
#[derive(Default)]
pub struct QGraphicsSimpleTextItem {
  qbase: QAbstractGraphicsShapeItem,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsTextItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsTextItem {
    return QGraphicsTextItem{qbase: QGraphicsObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsTextItem {
  type Target = QGraphicsObject;

  fn deref(&self) -> &QGraphicsObject {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsObject> for QGraphicsTextItem {
  fn as_ref(& self) -> & QGraphicsObject {
    return & self.qbase;
  }
}
  // proto:  bool QGraphicsTextItem::openExternalLinks();
impl /*struct*/ QGraphicsTextItem {
  pub fn openExternalLinks<RetType, T: QGraphicsTextItem_openExternalLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_openExternalLinks<RetType> {
  fn openExternalLinks(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  bool QGraphicsTextItem::openExternalLinks();
impl<'a> /*trait*/ QGraphicsTextItem_openExternalLinks<i8> for () {
  fn openExternalLinks(self , rsthis: & QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem17openExternalLinksEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QGraphicsTextItem::textWidth();
impl /*struct*/ QGraphicsTextItem {
  pub fn textWidth<RetType, T: QGraphicsTextItem_textWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textWidth(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_textWidth<RetType> {
  fn textWidth(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  qreal QGraphicsTextItem::textWidth();
impl<'a> /*trait*/ QGraphicsTextItem_textWidth<f64> for () {
  fn textWidth(self , rsthis: & QGraphicsTextItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem9textWidthEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setTextWidth(qreal width);
impl /*struct*/ QGraphicsTextItem {
  pub fn setTextWidth<RetType, T: QGraphicsTextItem_setTextWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setTextWidth<RetType> {
  fn setTextWidth(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setTextWidth(qreal width);
impl<'a> /*trait*/ QGraphicsTextItem_setTextWidth<()> for (f64) {
  fn setTextWidth(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN17QGraphicsTextItem12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
impl /*struct*/ QGraphicsTextItem {
  pub fn setTextCursor<RetType, T: QGraphicsTextItem_setTextCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextCursor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setTextCursor<RetType> {
  fn setTextCursor(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setTextCursor(const QTextCursor & cursor);
impl<'a> /*trait*/ QGraphicsTextItem_setTextCursor<()> for (&'a QTextCursor) {
  fn setTextCursor(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem13setTextCursorERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsTextItem::type();
impl /*struct*/ QGraphicsTextItem {
  pub fn type_<RetType, T: QGraphicsTextItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  int QGraphicsTextItem::type();
impl<'a> /*trait*/ QGraphicsTextItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem4typeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QFont QGraphicsTextItem::font();
impl /*struct*/ QGraphicsTextItem {
  pub fn font<RetType, T: QGraphicsTextItem_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_font<RetType> {
  fn font(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QFont QGraphicsTextItem::font();
impl<'a> /*trait*/ QGraphicsTextItem_font<QFont> for () {
  fn font(self , rsthis: & QGraphicsTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem4fontEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::QGraphicsTextItem(const QString & text, QGraphicsItem * parent);
impl /*struct*/ QGraphicsTextItem {
  pub fn new<T: QGraphicsTextItem_new>(value: T) -> QGraphicsTextItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTextItem_new {
  fn new(self) -> QGraphicsTextItem;
}

  // proto:  void QGraphicsTextItem::QGraphicsTextItem(const QString & text, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsTextItem_new for (&'a QString, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsTextItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemC2ERK7QStringP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsTextItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsTextItemC2ERK7QStringP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsTextItem{qbase: QGraphicsObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsTextItem::metaObject();
impl /*struct*/ QGraphicsTextItem {
  pub fn metaObject<RetType, T: QGraphicsTextItem_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsTextItem::metaObject();
impl<'a> /*trait*/ QGraphicsTextItem_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsTextItem) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10metaObjectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setOpenExternalLinks(bool open);
impl /*struct*/ QGraphicsTextItem {
  pub fn setOpenExternalLinks<RetType, T: QGraphicsTextItem_setOpenExternalLinks<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setOpenExternalLinks<RetType> {
  fn setOpenExternalLinks(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QGraphicsTextItem_setOpenExternalLinks<()> for (i8) {
  fn setOpenExternalLinks(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem20setOpenExternalLinksEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN17QGraphicsTextItem20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setTabChangesFocus(bool b);
impl /*struct*/ QGraphicsTextItem {
  pub fn setTabChangesFocus<RetType, T: QGraphicsTextItem_setTabChangesFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabChangesFocus(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setTabChangesFocus<RetType> {
  fn setTabChangesFocus(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setTabChangesFocus(bool b);
impl<'a> /*trait*/ QGraphicsTextItem_setTabChangesFocus<()> for (i8) {
  fn setTabChangesFocus(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem18setTabChangesFocusEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN17QGraphicsTextItem18setTabChangesFocusEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QGraphicsTextItem::toHtml();
impl /*struct*/ QGraphicsTextItem {
  pub fn toHtml<RetType, T: QGraphicsTextItem_toHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toHtml(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_toHtml<RetType> {
  fn toHtml(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QString QGraphicsTextItem::toHtml();
impl<'a> /*trait*/ QGraphicsTextItem_toHtml<QString> for () {
  fn toHtml(self , rsthis: & QGraphicsTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem6toHtmlEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem6toHtmlEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setDocument(QTextDocument * document);
impl /*struct*/ QGraphicsTextItem {
  pub fn setDocument<RetType, T: QGraphicsTextItem_setDocument<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocument(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setDocument<RetType> {
  fn setDocument(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setDocument(QTextDocument * document);
impl<'a> /*trait*/ QGraphicsTextItem_setDocument<()> for (&'a QTextDocument) {
  fn setDocument(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem11setDocumentEP13QTextDocument(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setPlainText(const QString & text);
impl /*struct*/ QGraphicsTextItem {
  pub fn setPlainText<RetType, T: QGraphicsTextItem_setPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlainText(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setPlainText<RetType> {
  fn setPlainText(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setPlainText(const QString & text);
impl<'a> /*trait*/ QGraphicsTextItem_setPlainText<()> for (&'a QString) {
  fn setPlainText(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsTextItem {
  pub fn paint<RetType, T: QGraphicsTextItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsTextItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setFont(const QFont & font);
impl /*struct*/ QGraphicsTextItem {
  pub fn setFont<RetType, T: QGraphicsTextItem_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setFont<RetType> {
  fn setFont(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsTextItem_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
impl /*struct*/ QGraphicsTextItem {
  pub fn setDefaultTextColor<RetType, T: QGraphicsTextItem_setDefaultTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultTextColor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setDefaultTextColor<RetType> {
  fn setDefaultTextColor(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setDefaultTextColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsTextItem_setDefaultTextColor<()> for (&'a QColor) {
  fn setDefaultTextColor(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem19setDefaultTextColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QGraphicsTextItem::defaultTextColor();
impl /*struct*/ QGraphicsTextItem {
  pub fn defaultTextColor<RetType, T: QGraphicsTextItem_defaultTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultTextColor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_defaultTextColor<RetType> {
  fn defaultTextColor(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QColor QGraphicsTextItem::defaultTextColor();
impl<'a> /*trait*/ QGraphicsTextItem_defaultTextColor<QColor> for () {
  fn defaultTextColor(self , rsthis: & QGraphicsTextItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem16defaultTextColorEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem16defaultTextColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::~QGraphicsTextItem();
impl /*struct*/ QGraphicsTextItem {
  pub fn free<RetType, T: QGraphicsTextItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::~QGraphicsTextItem();
impl<'a> /*trait*/ QGraphicsTextItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemD2Ev()};
     unsafe {C_ZN17QGraphicsTextItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsTextItem::shape();
impl /*struct*/ QGraphicsTextItem {
  pub fn shape<RetType, T: QGraphicsTextItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsTextItem::shape();
impl<'a> /*trait*/ QGraphicsTextItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem5shapeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextCursor QGraphicsTextItem::textCursor();
impl /*struct*/ QGraphicsTextItem {
  pub fn textCursor<RetType, T: QGraphicsTextItem_textCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textCursor(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_textCursor<RetType> {
  fn textCursor(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QTextCursor QGraphicsTextItem::textCursor();
impl<'a> /*trait*/ QGraphicsTextItem_textCursor<QTextCursor> for () {
  fn textCursor(self , rsthis: & QGraphicsTextItem) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10textCursorEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem10textCursorEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsTextItem::boundingRect();
impl /*struct*/ QGraphicsTextItem {
  pub fn boundingRect<RetType, T: QGraphicsTextItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QRectF QGraphicsTextItem::boundingRect();
impl<'a> /*trait*/ QGraphicsTextItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsTextItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QGraphicsTextItem::toPlainText();
impl /*struct*/ QGraphicsTextItem {
  pub fn toPlainText<RetType, T: QGraphicsTextItem_toPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_toPlainText<RetType> {
  fn toPlainText(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QString QGraphicsTextItem::toPlainText();
impl<'a> /*trait*/ QGraphicsTextItem_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: & QGraphicsTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem11toPlainTextEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::setHtml(const QString & html);
impl /*struct*/ QGraphicsTextItem {
  pub fn setHtml<RetType, T: QGraphicsTextItem_setHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHtml(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_setHtml<RetType> {
  fn setHtml(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::setHtml(const QString & html);
impl<'a> /*trait*/ QGraphicsTextItem_setHtml<()> for (&'a QString) {
  fn setHtml(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsTextItem7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsTextItem::tabChangesFocus();
impl /*struct*/ QGraphicsTextItem {
  pub fn tabChangesFocus<RetType, T: QGraphicsTextItem_tabChangesFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabChangesFocus(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_tabChangesFocus<RetType> {
  fn tabChangesFocus(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  bool QGraphicsTextItem::tabChangesFocus();
impl<'a> /*trait*/ QGraphicsTextItem_tabChangesFocus<i8> for () {
  fn tabChangesFocus(self , rsthis: & QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem15tabChangesFocusEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem15tabChangesFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::QGraphicsTextItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsTextItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsTextItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsTextItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsTextItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsTextItem{qbase: QGraphicsObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextDocument * QGraphicsTextItem::document();
impl /*struct*/ QGraphicsTextItem {
  pub fn document<RetType, T: QGraphicsTextItem_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_document<RetType> {
  fn document(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QTextDocument * QGraphicsTextItem::document();
impl<'a> /*trait*/ QGraphicsTextItem_document<QTextDocument> for () {
  fn document(self , rsthis: & QGraphicsTextItem) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem8documentEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsTextItem {
  pub fn isObscuredBy<RetType, T: QGraphicsTextItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  bool QGraphicsTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsTextItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsTextItem::opaqueArea();
impl /*struct*/ QGraphicsTextItem {
  pub fn opaqueArea<RetType, T: QGraphicsTextItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsTextItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsTextItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsTextItem {
  pub fn contains<RetType, T: QGraphicsTextItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  bool QGraphicsTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsTextItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsTextItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsTextItem::adjustSize();
impl /*struct*/ QGraphicsTextItem {
  pub fn adjustSize<RetType, T: QGraphicsTextItem_adjustSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.adjustSize(self);
    // return 1;
  }
}

pub trait QGraphicsTextItem_adjustSize<RetType> {
  fn adjustSize(self , rsthis: & QGraphicsTextItem) -> RetType;
}

  // proto:  void QGraphicsTextItem::adjustSize();
impl<'a> /*trait*/ QGraphicsTextItem_adjustSize<()> for () {
  fn adjustSize(self , rsthis: & QGraphicsTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsTextItem10adjustSizeEv()};
     unsafe {C_ZN17QGraphicsTextItem10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsPixmapItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsPixmapItem {
    return QGraphicsPixmapItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsPixmapItem {
  type Target = QGraphicsItem;

  fn deref(&self) -> &QGraphicsItem {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsItem> for QGraphicsPixmapItem {
  fn as_ref(& self) -> & QGraphicsItem {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsPixmapItem::QGraphicsPixmapItem(QGraphicsItem * parent);
impl /*struct*/ QGraphicsPixmapItem {
  pub fn new<T: QGraphicsPixmapItem_new>(value: T) -> QGraphicsPixmapItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_new {
  fn new(self) -> QGraphicsPixmapItem;
}

  // proto:  void QGraphicsPixmapItem::QGraphicsPixmapItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPixmapItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsPixmapItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsPixmapItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QGraphicsPixmapItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsPixmapItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsPixmapItem::QGraphicsPixmapItem(const QPixmap & pixmap, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPixmapItem_new for (&'a QPixmap, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsPixmapItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemC2ERK7QPixmapP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsPixmapItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN19QGraphicsPixmapItemC2ERK7QPixmapP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsPixmapItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsPixmapItem::~QGraphicsPixmapItem();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn free<RetType, T: QGraphicsPixmapItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  void QGraphicsPixmapItem::~QGraphicsPixmapItem();
impl<'a> /*trait*/ QGraphicsPixmapItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItemD2Ev()};
     unsafe {C_ZN19QGraphicsPixmapItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPixmapItem::opaqueArea();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn opaqueArea<RetType, T: QGraphicsPixmapItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPixmapItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPixmapItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsPixmapItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsPixmapItem {
  pub fn isObscuredBy<RetType, T: QGraphicsPixmapItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPixmapItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsPixmapItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QGraphicsPixmapItem::type();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn type_<RetType, T: QGraphicsPixmapItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  int QGraphicsPixmapItem::type();
impl<'a> /*trait*/ QGraphicsPixmapItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsPixmapItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem4typeEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPixmapItem::shape();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn shape<RetType, T: QGraphicsPixmapItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPixmapItem::shape();
impl<'a> /*trait*/ QGraphicsPixmapItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsPixmapItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem5shapeEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPixmap QGraphicsPixmapItem::pixmap();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn pixmap<RetType, T: QGraphicsPixmapItem_pixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixmap(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_pixmap<RetType> {
  fn pixmap(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  QPixmap QGraphicsPixmapItem::pixmap();
impl<'a> /*trait*/ QGraphicsPixmapItem_pixmap<QPixmap> for () {
  fn pixmap(self , rsthis: & QGraphicsPixmapItem) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem6pixmapEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
impl /*struct*/ QGraphicsPixmapItem {
  pub fn setOffset<RetType, T: QGraphicsPixmapItem_setOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_setOffset<RetType> {
  fn setOffset(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  void QGraphicsPixmapItem::setOffset(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsPixmapItem_setOffset<()> for (f64, f64) {
  fn setOffset(self , rsthis: & QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN19QGraphicsPixmapItem9setOffsetEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsPixmapItem {
  pub fn paint<RetType, T: QGraphicsPixmapItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  void QGraphicsPixmapItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPixmapItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN19QGraphicsPixmapItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsPixmapItem::offset();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn offset<RetType, T: QGraphicsPixmapItem_offset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_offset<RetType> {
  fn offset(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  QPointF QGraphicsPixmapItem::offset();
impl<'a> /*trait*/ QGraphicsPixmapItem_offset<QPointF> for () {
  fn offset(self , rsthis: & QGraphicsPixmapItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem6offsetEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsPixmapItem::boundingRect();
impl /*struct*/ QGraphicsPixmapItem {
  pub fn boundingRect<RetType, T: QGraphicsPixmapItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  QRectF QGraphicsPixmapItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPixmapItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsPixmapItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsPixmapItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsPixmapItem {
  pub fn contains<RetType, T: QGraphicsPixmapItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  bool QGraphicsPixmapItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPixmapItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsPixmapItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsPixmapItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK19QGraphicsPixmapItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
impl /*struct*/ QGraphicsPixmapItem {
  pub fn setPixmap<RetType, T: QGraphicsPixmapItem_setPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPixmap(self);
    // return 1;
  }
}

pub trait QGraphicsPixmapItem_setPixmap<RetType> {
  fn setPixmap(self , rsthis: & QGraphicsPixmapItem) -> RetType;
}

  // proto:  void QGraphicsPixmapItem::setPixmap(const QPixmap & pixmap);
impl<'a> /*trait*/ QGraphicsPixmapItem_setPixmap<()> for (&'a QPixmap) {
  fn setPixmap(self , rsthis: & QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QGraphicsPixmapItem9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsPixmapItem::setOffset(const QPointF & offset);
impl<'a> /*trait*/ QGraphicsPixmapItem_setOffset<()> for (&'a QPointF) {
  fn setOffset(self , rsthis: & QGraphicsPixmapItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN19QGraphicsPixmapItem9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsRectItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsRectItem {
    return QGraphicsRectItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsRectItem {
  type Target = QAbstractGraphicsShapeItem;

  fn deref(&self) -> &QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsRectItem {
  fn as_ref(& self) -> & QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
  // proto:  bool QGraphicsRectItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsRectItem {
  pub fn isObscuredBy<RetType, T: QGraphicsRectItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  bool QGraphicsRectItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsRectItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsRectItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsRectItem::boundingRect();
impl /*struct*/ QGraphicsRectItem {
  pub fn boundingRect<RetType, T: QGraphicsRectItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  QRectF QGraphicsRectItem::boundingRect();
impl<'a> /*trait*/ QGraphicsRectItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsRectItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGraphicsRectItem::type();
impl /*struct*/ QGraphicsRectItem {
  pub fn type_<RetType, T: QGraphicsRectItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  int QGraphicsRectItem::type();
impl<'a> /*trait*/ QGraphicsRectItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsRectItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem4typeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsRectItem::rect();
impl /*struct*/ QGraphicsRectItem {
  pub fn rect<RetType, T: QGraphicsRectItem_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_rect<RetType> {
  fn rect(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  QRectF QGraphicsRectItem::rect();
impl<'a> /*trait*/ QGraphicsRectItem_rect<QRectF> for () {
  fn rect(self , rsthis: & QGraphicsRectItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem4rectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsRectItem::shape();
impl /*struct*/ QGraphicsRectItem {
  pub fn shape<RetType, T: QGraphicsRectItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsRectItem::shape();
impl<'a> /*trait*/ QGraphicsRectItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsRectItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem5shapeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::~QGraphicsRectItem();
impl /*struct*/ QGraphicsRectItem {
  pub fn free<RetType, T: QGraphicsRectItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  void QGraphicsRectItem::~QGraphicsRectItem();
impl<'a> /*trait*/ QGraphicsRectItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsRectItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemD2Ev()};
     unsafe {C_ZN17QGraphicsRectItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::QGraphicsRectItem(const QRectF & rect, QGraphicsItem * parent);
impl /*struct*/ QGraphicsRectItem {
  pub fn new<T: QGraphicsRectItem_new>(value: T) -> QGraphicsRectItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsRectItem_new {
  fn new(self) -> QGraphicsRectItem;
}

  // proto:  void QGraphicsRectItem::QGraphicsRectItem(const QRectF & rect, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsRectItem_new for (&'a QRectF, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsRectItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC2ERK6QRectFP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsRectItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsRectItemC2ERK6QRectFP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsRectItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsRectItem::opaqueArea();
impl /*struct*/ QGraphicsRectItem {
  pub fn opaqueArea<RetType, T: QGraphicsRectItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsRectItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsRectItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsRectItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::setRect(const QRectF & rect);
impl /*struct*/ QGraphicsRectItem {
  pub fn setRect<RetType, T: QGraphicsRectItem_setRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRect(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_setRect<RetType> {
  fn setRect(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  void QGraphicsRectItem::setRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsRectItem_setRect<()> for (&'a QRectF) {
  fn setRect(self , rsthis: & QGraphicsRectItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItem7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsRectItem7setRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsRectItem_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self , rsthis: & QGraphicsRectItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItem7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {C_ZN17QGraphicsRectItem7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::QGraphicsRectItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsRectItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsRectItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsRectItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsRectItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsRectItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsRectItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsRectItem {
  pub fn contains<RetType, T: QGraphicsRectItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  bool QGraphicsRectItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsRectItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsRectItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsRectItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsRectItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsRectItem {
  pub fn paint<RetType, T: QGraphicsRectItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsRectItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsRectItem) -> RetType;
}

  // proto:  void QGraphicsRectItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsRectItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsRectItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsRectItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsRectItem::QGraphicsRectItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsRectItem_new for (f64, f64, f64, f64, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsRectItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsRectItemC2EddddP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsRectItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsRectItemC2EddddP13QGraphicsItem(arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsRectItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsEllipseItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsEllipseItem {
    return QGraphicsEllipseItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsEllipseItem {
  type Target = QAbstractGraphicsShapeItem;

  fn deref(&self) -> &QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsEllipseItem {
  fn as_ref(& self) -> & QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsEllipseItem::setStartAngle(int angle);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn setStartAngle<RetType, T: QGraphicsEllipseItem_setStartAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setStartAngle<RetType> {
  fn setStartAngle(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  void QGraphicsEllipseItem::setStartAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setStartAngle<()> for (i32) {
  fn setStartAngle(self , rsthis: & QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem13setStartAngleEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN20QGraphicsEllipseItem13setStartAngleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsEllipseItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn contains<RetType, T: QGraphicsEllipseItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  bool QGraphicsEllipseItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsEllipseItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsEllipseItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn new<T: QGraphicsEllipseItem_new>(value: T) -> QGraphicsEllipseItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_new {
  fn new(self) -> QGraphicsEllipseItem;
}

  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(const QRectF & rect, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_new for (&'a QRectF, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsEllipseItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC2ERK6QRectFP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsEllipseItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QGraphicsEllipseItemC2ERK6QRectFP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsEllipseItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::setRect(const QRectF & rect);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn setRect<RetType, T: QGraphicsEllipseItem_setRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setRect<RetType> {
  fn setRect(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  void QGraphicsEllipseItem::setRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect<()> for (&'a QRectF) {
  fn setRect(self , rsthis: & QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsEllipseItem7setRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn paint<RetType, T: QGraphicsEllipseItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  void QGraphicsEllipseItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsEllipseItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsEllipseItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn isObscuredBy<RetType, T: QGraphicsEllipseItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsEllipseItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsEllipseItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsEllipseItem::rect();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn rect<RetType, T: QGraphicsEllipseItem_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_rect<RetType> {
  fn rect(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  QRectF QGraphicsEllipseItem::rect();
impl<'a> /*trait*/ QGraphicsEllipseItem_rect<QRectF> for () {
  fn rect(self , rsthis: & QGraphicsEllipseItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4rectEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGraphicsEllipseItem::spanAngle();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn spanAngle<RetType, T: QGraphicsEllipseItem_spanAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spanAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_spanAngle<RetType> {
  fn spanAngle(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  int QGraphicsEllipseItem::spanAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_spanAngle<i32> for () {
  fn spanAngle(self , rsthis: & QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem9spanAngleEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem9spanAngleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QGraphicsEllipseItem::startAngle();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn startAngle<RetType, T: QGraphicsEllipseItem_startAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_startAngle<RetType> {
  fn startAngle(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  int QGraphicsEllipseItem::startAngle();
impl<'a> /*trait*/ QGraphicsEllipseItem_startAngle<i32> for () {
  fn startAngle(self , rsthis: & QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10startAngleEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem10startAngleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(qreal x, qreal y, qreal w, qreal h, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_new for (f64, f64, f64, f64, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsEllipseItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC2EddddP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsEllipseItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QGraphicsEllipseItemC2EddddP13QGraphicsItem(arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsEllipseItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsEllipseItem_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self , rsthis: & QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {C_ZN20QGraphicsEllipseItem7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::setSpanAngle(int angle);
impl /*struct*/ QGraphicsEllipseItem {
  pub fn setSpanAngle<RetType, T: QGraphicsEllipseItem_setSpanAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpanAngle(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_setSpanAngle<RetType> {
  fn setSpanAngle(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  void QGraphicsEllipseItem::setSpanAngle(int angle);
impl<'a> /*trait*/ QGraphicsEllipseItem_setSpanAngle<()> for (i32) {
  fn setSpanAngle(self , rsthis: & QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItem12setSpanAngleEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN20QGraphicsEllipseItem12setSpanAngleEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsEllipseItem::type();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn type_<RetType, T: QGraphicsEllipseItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  int QGraphicsEllipseItem::type();
impl<'a> /*trait*/ QGraphicsEllipseItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsEllipseItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem4typeEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsEllipseItem::boundingRect();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn boundingRect<RetType, T: QGraphicsEllipseItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  QRectF QGraphicsEllipseItem::boundingRect();
impl<'a> /*trait*/ QGraphicsEllipseItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsEllipseItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsEllipseItem::shape();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn shape<RetType, T: QGraphicsEllipseItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsEllipseItem::shape();
impl<'a> /*trait*/ QGraphicsEllipseItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsEllipseItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem5shapeEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::~QGraphicsEllipseItem();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn free<RetType, T: QGraphicsEllipseItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  void QGraphicsEllipseItem::~QGraphicsEllipseItem();
impl<'a> /*trait*/ QGraphicsEllipseItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsEllipseItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemD2Ev()};
     unsafe {C_ZN20QGraphicsEllipseItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEllipseItem::QGraphicsEllipseItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsEllipseItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsEllipseItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsEllipseItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsEllipseItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QGraphicsEllipseItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsEllipseItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsEllipseItem::opaqueArea();
impl /*struct*/ QGraphicsEllipseItem {
  pub fn opaqueArea<RetType, T: QGraphicsEllipseItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsEllipseItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsEllipseItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsEllipseItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsEllipseItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsEllipseItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsEllipseItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsEllipseItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPolygonItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsPolygonItem {
    return QGraphicsPolygonItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsPolygonItem {
  type Target = QAbstractGraphicsShapeItem;

  fn deref(&self) -> &QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsPolygonItem {
  fn as_ref(& self) -> & QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
  // proto:  QPainterPath QGraphicsPolygonItem::shape();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn shape<RetType, T: QGraphicsPolygonItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPolygonItem::shape();
impl<'a> /*trait*/ QGraphicsPolygonItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsPolygonItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem5shapeEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn isObscuredBy<RetType, T: QGraphicsPolygonItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPolygonItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsPolygonItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn paint<RetType, T: QGraphicsPolygonItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  void QGraphicsPolygonItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPolygonItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsPolygonItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsPolygonItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem * parent);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn new<T: QGraphicsPolygonItem_new>(value: T) -> QGraphicsPolygonItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_new {
  fn new(self) -> QGraphicsPolygonItem;
}

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPolygonItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsPolygonItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsPolygonItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QGraphicsPolygonItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsPolygonItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsPolygonItem::boundingRect();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn boundingRect<RetType, T: QGraphicsPolygonItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  QRectF QGraphicsPolygonItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPolygonItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsPolygonItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGraphicsPolygonItem::type();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn type_<RetType, T: QGraphicsPolygonItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  int QGraphicsPolygonItem::type();
impl<'a> /*trait*/ QGraphicsPolygonItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsPolygonItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem4typeEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::~QGraphicsPolygonItem();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn free<RetType, T: QGraphicsPolygonItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  void QGraphicsPolygonItem::~QGraphicsPolygonItem();
impl<'a> /*trait*/ QGraphicsPolygonItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsPolygonItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemD2Ev()};
     unsafe {C_ZN20QGraphicsPolygonItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsPolygonItem::polygon();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn polygon<RetType, T: QGraphicsPolygonItem_polygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polygon(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_polygon<RetType> {
  fn polygon(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsPolygonItem::polygon();
impl<'a> /*trait*/ QGraphicsPolygonItem_polygon<QPolygonF> for () {
  fn polygon(self , rsthis: & QGraphicsPolygonItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem7polygonEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem7polygonEv(rsthis.qclsinst)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPolygonItem::opaqueArea();
impl /*struct*/ QGraphicsPolygonItem {
  pub fn opaqueArea<RetType, T: QGraphicsPolygonItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPolygonItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPolygonItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsPolygonItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::QGraphicsPolygonItem(const QPolygonF & polygon, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPolygonItem_new for (&'a QPolygonF, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsPolygonItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItemC2ERK9QPolygonFP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsPolygonItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN20QGraphicsPolygonItemC2ERK9QPolygonFP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsPolygonItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsPolygonItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn contains<RetType, T: QGraphicsPolygonItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  bool QGraphicsPolygonItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPolygonItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsPolygonItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QGraphicsPolygonItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK20QGraphicsPolygonItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
impl /*struct*/ QGraphicsPolygonItem {
  pub fn setPolygon<RetType, T: QGraphicsPolygonItem_setPolygon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPolygon(self);
    // return 1;
  }
}

pub trait QGraphicsPolygonItem_setPolygon<RetType> {
  fn setPolygon(self , rsthis: & QGraphicsPolygonItem) -> RetType;
}

  // proto:  void QGraphicsPolygonItem::setPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsPolygonItem_setPolygon<()> for (&'a QPolygonF) {
  fn setPolygon(self , rsthis: & QGraphicsPolygonItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN20QGraphicsPolygonItem10setPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsPathItem {
    return QGraphicsPathItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsPathItem {
  type Target = QAbstractGraphicsShapeItem;

  fn deref(&self) -> &QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsPathItem {
  fn as_ref(& self) -> & QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsPathItem::setPath(const QPainterPath & path);
impl /*struct*/ QGraphicsPathItem {
  pub fn setPath<RetType, T: QGraphicsPathItem_setPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPath(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_setPath<RetType> {
  fn setPath(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  void QGraphicsPathItem::setPath(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsPathItem_setPath<()> for (&'a QPainterPath) {
  fn setPath(self , rsthis: & QGraphicsPathItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItem7setPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsPathItem7setPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsPathItem::QGraphicsPathItem(const QPainterPath & path, QGraphicsItem * parent);
impl /*struct*/ QGraphicsPathItem {
  pub fn new<T: QGraphicsPathItem_new>(value: T) -> QGraphicsPathItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsPathItem_new {
  fn new(self) -> QGraphicsPathItem;
}

  // proto:  void QGraphicsPathItem::QGraphicsPathItem(const QPainterPath & path, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPathItem_new for (&'a QPainterPath, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsPathItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemC2ERK12QPainterPathP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsPathItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsPathItemC2ERK12QPainterPathP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsPathItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsPathItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsPathItem {
  pub fn contains<RetType, T: QGraphicsPathItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  bool QGraphicsPathItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPathItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsPathItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsPathItem::boundingRect();
impl /*struct*/ QGraphicsPathItem {
  pub fn boundingRect<RetType, T: QGraphicsPathItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  QRectF QGraphicsPathItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPathItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsPathItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QGraphicsPathItem::type();
impl /*struct*/ QGraphicsPathItem {
  pub fn type_<RetType, T: QGraphicsPathItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  int QGraphicsPathItem::type();
impl<'a> /*trait*/ QGraphicsPathItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsPathItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem4typeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPathItem::opaqueArea();
impl /*struct*/ QGraphicsPathItem {
  pub fn opaqueArea<RetType, T: QGraphicsPathItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPathItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPathItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsPathItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPathItem::path();
impl /*struct*/ QGraphicsPathItem {
  pub fn path<RetType, T: QGraphicsPathItem_path<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_path<RetType> {
  fn path(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPathItem::path();
impl<'a> /*trait*/ QGraphicsPathItem_path<QPainterPath> for () {
  fn path(self , rsthis: & QGraphicsPathItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem4pathEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem4pathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsPathItem::~QGraphicsPathItem();
impl /*struct*/ QGraphicsPathItem {
  pub fn free<RetType, T: QGraphicsPathItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  void QGraphicsPathItem::~QGraphicsPathItem();
impl<'a> /*trait*/ QGraphicsPathItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsPathItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemD2Ev()};
     unsafe {C_ZN17QGraphicsPathItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsPathItem::shape();
impl /*struct*/ QGraphicsPathItem {
  pub fn shape<RetType, T: QGraphicsPathItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsPathItem::shape();
impl<'a> /*trait*/ QGraphicsPathItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsPathItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem5shapeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsPathItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsPathItem {
  pub fn isObscuredBy<RetType, T: QGraphicsPathItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  bool QGraphicsPathItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPathItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsPathItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsPathItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsPathItem::QGraphicsPathItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPathItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsPathItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsPathItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsPathItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsPathItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsPathItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsPathItem {
  pub fn paint<RetType, T: QGraphicsPathItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsPathItem) -> RetType;
}

  // proto:  void QGraphicsPathItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPathItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsPathItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsPathItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLineItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsLineItem {
    return QGraphicsLineItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsLineItem {
  type Target = QGraphicsItem;

  fn deref(&self) -> &QGraphicsItem {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsItem> for QGraphicsLineItem {
  fn as_ref(& self) -> & QGraphicsItem {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsLineItem::setPen(const QPen & pen);
impl /*struct*/ QGraphicsLineItem {
  pub fn setPen<RetType, T: QGraphicsLineItem_setPen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPen(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_setPen<RetType> {
  fn setPen(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  void QGraphicsLineItem::setPen(const QPen & pen);
impl<'a> /*trait*/ QGraphicsLineItem_setPen<()> for (&'a QPen) {
  fn setPen(self , rsthis: & QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsLineItem6setPenERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::QGraphicsLineItem(QGraphicsItem * parent);
impl /*struct*/ QGraphicsLineItem {
  pub fn new<T: QGraphicsLineItem_new>(value: T) -> QGraphicsLineItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLineItem_new {
  fn new(self) -> QGraphicsLineItem;
}

  // proto:  void QGraphicsLineItem::QGraphicsLineItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsLineItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsLineItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsLineItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsLineItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsLineItem {
  pub fn isObscuredBy<RetType, T: QGraphicsLineItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsLineItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsLineItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::QGraphicsLineItem(const QLineF & line, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_new for (&'a QLineF, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsLineItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC2ERK6QLineFP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsLineItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsLineItemC2ERK6QLineFP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsLineItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QLineF QGraphicsLineItem::line();
impl /*struct*/ QGraphicsLineItem {
  pub fn line<RetType, T: QGraphicsLineItem_line<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.line(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_line<RetType> {
  fn line(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  QLineF QGraphicsLineItem::line();
impl<'a> /*trait*/ QGraphicsLineItem_line<QLineF> for () {
  fn line(self , rsthis: & QGraphicsLineItem) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem4lineEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem4lineEv(rsthis.qclsinst)};
    let mut ret1 = QLineF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsLineItem::opaqueArea();
impl /*struct*/ QGraphicsLineItem {
  pub fn opaqueArea<RetType, T: QGraphicsLineItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsLineItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsLineItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsLineItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl /*struct*/ QGraphicsLineItem {
  pub fn setLine<RetType, T: QGraphicsLineItem_setLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLine(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_setLine<RetType> {
  fn setLine(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  void QGraphicsLineItem::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QGraphicsLineItem_setLine<()> for (f64, f64, f64, f64) {
  fn setLine(self , rsthis: & QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem7setLineEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {C_ZN17QGraphicsLineItem7setLineEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsLineItem::boundingRect();
impl /*struct*/ QGraphicsLineItem {
  pub fn boundingRect<RetType, T: QGraphicsLineItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  QRectF QGraphicsLineItem::boundingRect();
impl<'a> /*trait*/ QGraphicsLineItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsLineItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPen QGraphicsLineItem::pen();
impl /*struct*/ QGraphicsLineItem {
  pub fn pen<RetType, T: QGraphicsLineItem_pen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pen(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_pen<RetType> {
  fn pen(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  QPen QGraphicsLineItem::pen();
impl<'a> /*trait*/ QGraphicsLineItem_pen<QPen> for () {
  fn pen(self , rsthis: & QGraphicsLineItem) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem3penEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::setLine(const QLineF & line);
impl<'a> /*trait*/ QGraphicsLineItem_setLine<()> for (&'a QLineF) {
  fn setLine(self , rsthis: & QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem7setLineERK6QLineF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsLineItem7setLineERK6QLineF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsLineItem::shape();
impl /*struct*/ QGraphicsLineItem {
  pub fn shape<RetType, T: QGraphicsLineItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsLineItem::shape();
impl<'a> /*trait*/ QGraphicsLineItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsLineItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem5shapeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsLineItem {
  pub fn paint<RetType, T: QGraphicsLineItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  void QGraphicsLineItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsLineItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN17QGraphicsLineItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QGraphicsLineItem::type();
impl /*struct*/ QGraphicsLineItem {
  pub fn type_<RetType, T: QGraphicsLineItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  int QGraphicsLineItem::type();
impl<'a> /*trait*/ QGraphicsLineItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsLineItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem4typeEv()};
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::QGraphicsLineItem(qreal x1, qreal y1, qreal x2, qreal y2, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsLineItem_new for (f64, f64, f64, f64, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsLineItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemC2EddddP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsLineItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QGraphicsLineItemC2EddddP13QGraphicsItem(arg0, arg1, arg2, arg3, arg4)};
    let rsthis = QGraphicsLineItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsLineItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsLineItem {
  pub fn contains<RetType, T: QGraphicsLineItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  bool QGraphicsLineItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsLineItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsLineItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsLineItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QGraphicsLineItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsLineItem::~QGraphicsLineItem();
impl /*struct*/ QGraphicsLineItem {
  pub fn free<RetType, T: QGraphicsLineItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsLineItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsLineItem) -> RetType;
}

  // proto:  void QGraphicsLineItem::~QGraphicsLineItem();
impl<'a> /*trait*/ QGraphicsLineItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsLineItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsLineItemD2Ev()};
     unsafe {C_ZN17QGraphicsLineItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemGroup {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsItemGroup {
    return QGraphicsItemGroup{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsItemGroup {
  type Target = QGraphicsItem;

  fn deref(&self) -> &QGraphicsItem {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsItem> for QGraphicsItemGroup {
  fn as_ref(& self) -> & QGraphicsItem {
    return & self.qbase;
  }
}
  // proto:  bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsItemGroup {
  pub fn isObscuredBy<RetType, T: QGraphicsItemGroup_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsItemGroup) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QGraphicsItemGroup12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItemGroup::~QGraphicsItemGroup();
impl /*struct*/ QGraphicsItemGroup {
  pub fn free<RetType, T: QGraphicsItemGroup_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_free<RetType> {
  fn free(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  void QGraphicsItemGroup::~QGraphicsItemGroup();
impl<'a> /*trait*/ QGraphicsItemGroup_free<()> for () {
  fn free(self , rsthis: & QGraphicsItemGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupD2Ev()};
     unsafe {C_ZN18QGraphicsItemGroupD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemGroup::QGraphicsItemGroup(QGraphicsItem * parent);
impl /*struct*/ QGraphicsItemGroup {
  pub fn new<T: QGraphicsItemGroup_new>(value: T) -> QGraphicsItemGroup {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemGroup_new {
  fn new(self) -> QGraphicsItemGroup;
}

  // proto:  void QGraphicsItemGroup::QGraphicsItemGroup(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItemGroup_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsItemGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroupC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsItemGroup_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QGraphicsItemGroupC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsItemGroup{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QGraphicsItemGroup::type();
impl /*struct*/ QGraphicsItemGroup {
  pub fn type_<RetType, T: QGraphicsItemGroup_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  int QGraphicsItemGroup::type();
impl<'a> /*trait*/ QGraphicsItemGroup_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsItemGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup4typeEv()};
    let mut ret = unsafe {C_ZNK18QGraphicsItemGroup4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItemGroup::boundingRect();
impl /*struct*/ QGraphicsItemGroup {
  pub fn boundingRect<RetType, T: QGraphicsItemGroup_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  QRectF QGraphicsItemGroup::boundingRect();
impl<'a> /*trait*/ QGraphicsItemGroup_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsItemGroup) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup12boundingRectEv()};
    let mut ret = unsafe {C_ZNK18QGraphicsItemGroup12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsItemGroup {
  pub fn paint<RetType, T: QGraphicsItemGroup_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  void QGraphicsItemGroup::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItemGroup_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsItemGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN18QGraphicsItemGroup5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
impl /*struct*/ QGraphicsItemGroup {
  pub fn removeFromGroup<RetType, T: QGraphicsItemGroup_removeFromGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeFromGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_removeFromGroup<RetType> {
  fn removeFromGroup(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  void QGraphicsItemGroup::removeFromGroup(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_removeFromGroup<()> for (&'a QGraphicsItem) {
  fn removeFromGroup(self , rsthis: & QGraphicsItemGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QGraphicsItemGroup15removeFromGroupEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
impl /*struct*/ QGraphicsItemGroup {
  pub fn addToGroup<RetType, T: QGraphicsItemGroup_addToGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addToGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_addToGroup<RetType> {
  fn addToGroup(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  void QGraphicsItemGroup::addToGroup(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemGroup_addToGroup<()> for (&'a QGraphicsItem) {
  fn addToGroup(self , rsthis: & QGraphicsItemGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QGraphicsItemGroup10addToGroupEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItemGroup::opaqueArea();
impl /*struct*/ QGraphicsItemGroup {
  pub fn opaqueArea<RetType, T: QGraphicsItemGroup_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsItemGroup_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsItemGroup) -> RetType;
}

  // proto:  QPainterPath QGraphicsItemGroup::opaqueArea();
impl<'a> /*trait*/ QGraphicsItemGroup_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsItemGroup) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsItemGroup10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK18QGraphicsItemGroup10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractGraphicsShapeItem {
    return QAbstractGraphicsShapeItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractGraphicsShapeItem {
  type Target = QGraphicsItem;

  fn deref(&self) -> &QGraphicsItem {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsItem> for QAbstractGraphicsShapeItem {
  fn as_ref(& self) -> & QGraphicsItem {
    return & self.qbase;
  }
}
  // proto:  bool QAbstractGraphicsShapeItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn isObscuredBy<RetType, T: QAbstractGraphicsShapeItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  bool QAbstractGraphicsShapeItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QAbstractGraphicsShapeItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAbstractGraphicsShapeItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK26QAbstractGraphicsShapeItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QBrush QAbstractGraphicsShapeItem::brush();
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn brush<RetType, T: QAbstractGraphicsShapeItem_brush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.brush(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_brush<RetType> {
  fn brush(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  QBrush QAbstractGraphicsShapeItem::brush();
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_brush<QBrush> for () {
  fn brush(self , rsthis: & QAbstractGraphicsShapeItem) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAbstractGraphicsShapeItem5brushEv()};
    let mut ret = unsafe {C_ZNK26QAbstractGraphicsShapeItem5brushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractGraphicsShapeItem::QAbstractGraphicsShapeItem(QGraphicsItem * parent);
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn new<T: QAbstractGraphicsShapeItem_new>(value: T) -> QAbstractGraphicsShapeItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_new {
  fn new(self) -> QAbstractGraphicsShapeItem;
}

  // proto:  void QAbstractGraphicsShapeItem::QAbstractGraphicsShapeItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QAbstractGraphicsShapeItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractGraphicsShapeItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QAbstractGraphicsShapeItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN26QAbstractGraphicsShapeItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QAbstractGraphicsShapeItem{qbase: QGraphicsItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QAbstractGraphicsShapeItem::opaqueArea();
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn opaqueArea<RetType, T: QAbstractGraphicsShapeItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  QPainterPath QAbstractGraphicsShapeItem::opaqueArea();
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QAbstractGraphicsShapeItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAbstractGraphicsShapeItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK26QAbstractGraphicsShapeItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractGraphicsShapeItem::setBrush(const QBrush & brush);
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn setBrush<RetType, T: QAbstractGraphicsShapeItem_setBrush<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBrush(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_setBrush<RetType> {
  fn setBrush(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  void QAbstractGraphicsShapeItem::setBrush(const QBrush & brush);
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_setBrush<()> for (&'a QBrush) {
  fn setBrush(self , rsthis: & QAbstractGraphicsShapeItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractGraphicsShapeItem8setBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN26QAbstractGraphicsShapeItem8setBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractGraphicsShapeItem::setPen(const QPen & pen);
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn setPen<RetType, T: QAbstractGraphicsShapeItem_setPen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPen(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_setPen<RetType> {
  fn setPen(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  void QAbstractGraphicsShapeItem::setPen(const QPen & pen);
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_setPen<()> for (&'a QPen) {
  fn setPen(self , rsthis: & QAbstractGraphicsShapeItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractGraphicsShapeItem6setPenERK4QPen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN26QAbstractGraphicsShapeItem6setPenERK4QPen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPen QAbstractGraphicsShapeItem::pen();
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn pen<RetType, T: QAbstractGraphicsShapeItem_pen<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pen(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_pen<RetType> {
  fn pen(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  QPen QAbstractGraphicsShapeItem::pen();
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_pen<QPen> for () {
  fn pen(self , rsthis: & QAbstractGraphicsShapeItem) -> QPen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QAbstractGraphicsShapeItem3penEv()};
    let mut ret = unsafe {C_ZNK26QAbstractGraphicsShapeItem3penEv(rsthis.qclsinst)};
    let mut ret1 = QPen::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractGraphicsShapeItem::~QAbstractGraphicsShapeItem();
impl /*struct*/ QAbstractGraphicsShapeItem {
  pub fn free<RetType, T: QAbstractGraphicsShapeItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractGraphicsShapeItem_free<RetType> {
  fn free(self , rsthis: & QAbstractGraphicsShapeItem) -> RetType;
}

  // proto:  void QAbstractGraphicsShapeItem::~QAbstractGraphicsShapeItem();
impl<'a> /*trait*/ QAbstractGraphicsShapeItem_free<()> for () {
  fn free(self , rsthis: & QAbstractGraphicsShapeItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QAbstractGraphicsShapeItemD2Ev()};
     unsafe {C_ZN26QAbstractGraphicsShapeItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsItem {
    return QGraphicsItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
impl /*struct*/ QGraphicsItem {
  pub fn mapFromParent<RetType, T: QGraphicsItem_mapFromParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapFromParent<RetType> {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::mapFromParent(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPainterPath> for (&'a QPainterPath) {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapFromParentERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
impl /*struct*/ QGraphicsItem {
  pub fn mapFromItem<RetType, T: QGraphicsItem_mapFromItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapFromItem<RetType> {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPointF> for (&'a QGraphicsItem, &'a QPointF) {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapFromItemEPKS_RK7QPointF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
impl /*struct*/ QGraphicsItem {
  pub fn focusItem<RetType, T: QGraphicsItem_focusItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusItem<RetType> {
  fn focusItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::focusItem();
impl<'a> /*trait*/ QGraphicsItem_focusItem<()> for () {
  fn focusItem(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9focusItemEv()};
     unsafe {C_ZNK13QGraphicsItem9focusItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
impl /*struct*/ QGraphicsItem {
  pub fn parentObject<RetType, T: QGraphicsItem_parentObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentObject(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentObject<RetType> {
  fn parentObject(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsObject * QGraphicsItem::parentObject();
impl<'a> /*trait*/ QGraphicsItem_parentObject<()> for () {
  fn parentObject(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentObjectEv()};
     unsafe {C_ZNK13QGraphicsItem12parentObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
impl /*struct*/ QGraphicsItem {
  pub fn setTransformOriginPoint<RetType, T: QGraphicsItem_setTransformOriginPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransformOriginPoint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setTransformOriginPoint<RetType> {
  fn setTransformOriginPoint(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setTransformOriginPoint(const QPointF & origin);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint<()> for (&'a QPointF) {
  fn setTransformOriginPoint(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem23setTransformOriginPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ungrabMouse();
impl /*struct*/ QGraphicsItem {
  pub fn ungrabMouse<RetType, T: QGraphicsItem_ungrabMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ungrabMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ungrabMouse<RetType> {
  fn ungrabMouse(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::ungrabMouse();
impl<'a> /*trait*/ QGraphicsItem_ungrabMouse<()> for () {
  fn ungrabMouse(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11ungrabMouseEv()};
     unsafe {C_ZN13QGraphicsItem11ungrabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QGraphicsItem::type();
impl /*struct*/ QGraphicsItem {
  pub fn type_<RetType, T: QGraphicsItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  int QGraphicsItem::type();
impl<'a> /*trait*/ QGraphicsItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4typeEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isSelected();
impl /*struct*/ QGraphicsItem {
  pub fn isSelected<RetType, T: QGraphicsItem_isSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSelected(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isSelected<RetType> {
  fn isSelected(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isSelected();
impl<'a> /*trait*/ QGraphicsItem_isSelected<i8> for () {
  fn isSelected(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isSelectedEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem10isSelectedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPolygonF> for (&'a QGraphicsItem, f64, f64, f64, f64) {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapFromItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
impl /*struct*/ QGraphicsItem {
  pub fn parentWidget<RetType, T: QGraphicsItem_parentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentWidget<RetType> {
  fn parentWidget(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsItem::parentWidget();
impl<'a> /*trait*/ QGraphicsItem_parentWidget<()> for () {
  fn parentWidget(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12parentWidgetEv()};
     unsafe {C_ZNK13QGraphicsItem12parentWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::resetTransform();
impl /*struct*/ QGraphicsItem {
  pub fn resetTransform<RetType, T: QGraphicsItem_resetTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_resetTransform<RetType> {
  fn resetTransform(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::resetTransform();
impl<'a> /*trait*/ QGraphicsItem_resetTransform<()> for () {
  fn resetTransform(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14resetTransformEv()};
     unsafe {C_ZN13QGraphicsItem14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
impl /*struct*/ QGraphicsItem {
  pub fn boundingRegion<RetType, T: QGraphicsItem_boundingRegion<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRegion(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRegion<RetType> {
  fn boundingRegion(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRegion QGraphicsItem::boundingRegion(const QTransform & itemToDeviceTransform);
impl<'a> /*trait*/ QGraphicsItem_boundingRegion<QRegion> for (&'a QTransform) {
  fn boundingRegion(self , rsthis: & QGraphicsItem) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14boundingRegionERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem14boundingRegionERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsItem {
  pub fn paint<RetType, T: QGraphicsItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isActive();
impl /*struct*/ QGraphicsItem {
  pub fn isActive<RetType, T: QGraphicsItem_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isActive<RetType> {
  fn isActive(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isActive();
impl<'a> /*trait*/ QGraphicsItem_isActive<i8> for () {
  fn isActive(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isActiveEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::QGraphicsItem(QGraphicsItem * parent);
impl /*struct*/ QGraphicsItem {
  pub fn new<T: QGraphicsItem_new>(value: T) -> QGraphicsItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItem_new {
  fn new(self) -> QGraphicsItem;
}

  // proto:  void QGraphicsItem::QGraphicsItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemC2EPS_()};
    let ctysz: c_int = unsafe{QGraphicsItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QGraphicsItemC2EPS_(arg0)};
    let rsthis = QGraphicsItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
impl /*struct*/ QGraphicsItem {
  pub fn mapToParent<RetType, T: QGraphicsItem_mapToParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapToParent<RetType> {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPolygonF> for (&'a QPolygonF) {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapToParentERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isWidget();
impl /*struct*/ QGraphicsItem {
  pub fn isWidget<RetType, T: QGraphicsItem_isWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isWidget<RetType> {
  fn isWidget(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isWidget();
impl<'a> /*trait*/ QGraphicsItem_isWidget<i8> for () {
  fn isWidget(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWidgetEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8isWidgetEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
impl /*struct*/ QGraphicsItem {
  pub fn setParentItem<RetType, T: QGraphicsItem_setParentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParentItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setParentItem<RetType> {
  fn setParentItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setParentItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_setParentItem<()> for (&'a QGraphicsItem) {
  fn setParentItem(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setParentItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem13setParentItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapToItem<RetType, T: QGraphicsItem_mapToItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapToItem<RetType> {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPolygonF> for (&'a QGraphicsItem, &'a QRectF) {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem9mapToItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsItem::window();
impl /*struct*/ QGraphicsItem {
  pub fn window<RetType, T: QGraphicsItem_window<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.window(self);
    // return 1;
  }
}

pub trait QGraphicsItem_window<RetType> {
  fn window(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsItem::window();
impl<'a> /*trait*/ QGraphicsItem_window<()> for () {
  fn window(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6windowEv()};
     unsafe {C_ZNK13QGraphicsItem6windowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::scenePos();
impl /*struct*/ QGraphicsItem {
  pub fn scenePos<RetType, T: QGraphicsItem_scenePos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scenePos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scenePos<RetType> {
  fn scenePos(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::scenePos();
impl<'a> /*trait*/ QGraphicsItem_scenePos<QPointF> for () {
  fn scenePos(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8scenePosEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8scenePosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::handlesChildEvents();
impl /*struct*/ QGraphicsItem {
  pub fn handlesChildEvents<RetType, T: QGraphicsItem_handlesChildEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handlesChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_handlesChildEvents<RetType> {
  fn handlesChildEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::handlesChildEvents();
impl<'a> /*trait*/ QGraphicsItem_handlesChildEvents<i8> for () {
  fn handlesChildEvents(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18handlesChildEventsEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem18handlesChildEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
impl /*struct*/ QGraphicsItem {
  pub fn setOpacity<RetType, T: QGraphicsItem_setOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setOpacity<RetType> {
  fn setOpacity(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsItem_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::sceneTransform();
impl /*struct*/ QGraphicsItem {
  pub fn sceneTransform<RetType, T: QGraphicsItem_sceneTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sceneTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneTransform<RetType> {
  fn sceneTransform(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::sceneTransform();
impl<'a> /*trait*/ QGraphicsItem_sceneTransform<QTransform> for () {
  fn sceneTransform(self , rsthis: & QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14sceneTransformEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem14sceneTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setZValue(qreal z);
impl /*struct*/ QGraphicsItem {
  pub fn setZValue<RetType, T: QGraphicsItem_setZValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setZValue(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setZValue<RetType> {
  fn setZValue(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setZValue(qreal z);
impl<'a> /*trait*/ QGraphicsItem_setZValue<()> for (f64) {
  fn setZValue(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setZValueEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem9setZValueEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPolygonF> for (&'a QRectF) {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapFromParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPolygonF> for (f64, f64, f64, f64) {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapFromParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn isObscured<RetType, T: QGraphicsItem_isObscured<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscured(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isObscured<RetType> {
  fn isObscured(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isObscured(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_isObscured<i8> for (f64, f64, f64, f64) {
  fn isObscured(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10isObscuredEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
impl /*struct*/ QGraphicsItem {
  pub fn installSceneEventFilter<RetType, T: QGraphicsItem_installSceneEventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.installSceneEventFilter(self);
    // return 1;
  }
}

pub trait QGraphicsItem_installSceneEventFilter<RetType> {
  fn installSceneEventFilter(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::installSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_installSceneEventFilter<()> for (&'a QGraphicsItem) {
  fn installSceneEventFilter(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23installSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem23installSceneEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setY(qreal y);
impl /*struct*/ QGraphicsItem {
  pub fn setY<RetType, T: QGraphicsItem_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setY<RetType> {
  fn setY(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setY(qreal y);
impl<'a> /*trait*/ QGraphicsItem_setY<()> for (f64) {
  fn setY(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setYEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToItem<RetType, T: QGraphicsItem_mapRectToItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectToItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToItem<RetType> {
  fn mapRectToItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem<QRectF> for (&'a QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectToItem(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapRectToItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
impl /*struct*/ QGraphicsItem {
  pub fn parentItem<RetType, T: QGraphicsItem_parentItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_parentItem<RetType> {
  fn parentItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::parentItem();
impl<'a> /*trait*/ QGraphicsItem_parentItem<()> for () {
  fn parentItem(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10parentItemEv()};
     unsafe {C_ZNK13QGraphicsItem10parentItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::clearFocus();
impl /*struct*/ QGraphicsItem {
  pub fn clearFocus<RetType, T: QGraphicsItem_clearFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearFocus(self);
    // return 1;
  }
}

pub trait QGraphicsItem_clearFocus<RetType> {
  fn clearFocus(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::clearFocus();
impl<'a> /*trait*/ QGraphicsItem_clearFocus<()> for () {
  fn clearFocus(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10clearFocusEv()};
     unsafe {C_ZN13QGraphicsItem10clearFocusEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isWindow();
impl /*struct*/ QGraphicsItem {
  pub fn isWindow<RetType, T: QGraphicsItem_isWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isWindow(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isWindow<RetType> {
  fn isWindow(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isWindow();
impl<'a> /*trait*/ QGraphicsItem_isWindow<i8> for () {
  fn isWindow(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8isWindowEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8isWindowEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::transformOriginPoint();
impl /*struct*/ QGraphicsItem {
  pub fn transformOriginPoint<RetType, T: QGraphicsItem_transformOriginPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transformOriginPoint(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transformOriginPoint<RetType> {
  fn transformOriginPoint(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::transformOriginPoint();
impl<'a> /*trait*/ QGraphicsItem_transformOriginPoint<QPointF> for () {
  fn transformOriginPoint(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20transformOriginPointEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem20transformOriginPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::boundingRect();
impl /*struct*/ QGraphicsItem {
  pub fn boundingRect<RetType, T: QGraphicsItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::boundingRect();
impl<'a> /*trait*/ QGraphicsItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
impl /*struct*/ QGraphicsItem {
  pub fn childrenBoundingRect<RetType, T: QGraphicsItem_childrenBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childrenBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_childrenBoundingRect<RetType> {
  fn childrenBoundingRect(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::childrenBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_childrenBoundingRect<QRectF> for () {
  fn childrenBoundingRect(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem20childrenBoundingRectEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem20childrenBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isObscured(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_isObscured<i8> for (&'a QRectF) {
  fn isObscured(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10isObscuredERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10isObscuredERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapFromScene<RetType, T: QGraphicsItem_mapFromScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapFromScene<RetType> {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPolygonF> for (&'a QRectF) {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12mapFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::hasCursor();
impl /*struct*/ QGraphicsItem {
  pub fn hasCursor<RetType, T: QGraphicsItem_hasCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hasCursor<RetType> {
  fn hasCursor(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::hasCursor();
impl<'a> /*trait*/ QGraphicsItem_hasCursor<i8> for () {
  fn hasCursor(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9hasCursorEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem9hasCursorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
impl /*struct*/ QGraphicsItem {
  pub fn setGraphicsEffect<RetType, T: QGraphicsItem_setGraphicsEffect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGraphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setGraphicsEffect<RetType> {
  fn setGraphicsEffect(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setGraphicsEffect(QGraphicsEffect * effect);
impl<'a> /*trait*/ QGraphicsItem_setGraphicsEffect<()> for (&'a QGraphicsEffect) {
  fn setGraphicsEffect(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem17setGraphicsEffectEP15QGraphicsEffect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapToParent(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPainterPath> for (&'a QPainterPath) {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapToParentERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl /*struct*/ QGraphicsItem {
  pub fn ensureVisible<RetType, T: QGraphicsItem_ensureVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible<()> for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {C_ZN13QGraphicsItem13ensureVisibleEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPolygonF> for (&'a QGraphicsItem, f64, f64, f64, f64) {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem9mapToItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPointF> for (&'a QGraphicsItem, f64, f64) {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_dd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem9mapToItemEPKS_dd(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToParent<RetType, T: QGraphicsItem_mapRectToParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectToParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToParent<RetType> {
  fn mapRectToParent(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent<QRectF> for (&'a QRectF) {
  fn mapRectToParent(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem15mapRectToParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
impl /*struct*/ QGraphicsItem {
  pub fn setToolTip<RetType, T: QGraphicsItem_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setToolTip(const QString & toolTip);
impl<'a> /*trait*/ QGraphicsItem_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::rotation();
impl /*struct*/ QGraphicsItem {
  pub fn rotation<RetType, T: QGraphicsItem_rotation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotation(self);
    // return 1;
  }
}

pub trait QGraphicsItem_rotation<RetType> {
  fn rotation(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::rotation();
impl<'a> /*trait*/ QGraphicsItem_rotation<f64> for () {
  fn rotation(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8rotationEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8rotationEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QGraphicsScene * QGraphicsItem::scene();
impl /*struct*/ QGraphicsItem {
  pub fn scene<RetType, T: QGraphicsItem_scene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scene<RetType> {
  fn scene(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsScene * QGraphicsItem::scene();
impl<'a> /*trait*/ QGraphicsItem_scene<()> for () {
  fn scene(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5sceneEv()};
     unsafe {C_ZNK13QGraphicsItem5sceneEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPainterPath> for (&'a QGraphicsItem, &'a QPainterPath) {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem9mapToItemEPKS_RK12QPainterPath(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToParent<QRectF> for (f64, f64, f64, f64) {
  fn mapRectToParent(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem15mapRectToParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPolygonF> for (&'a QGraphicsItem, &'a QRectF) {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapFromItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromParent<RetType, T: QGraphicsItem_mapRectFromParent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromParent(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromParent<RetType> {
  fn mapRectFromParent(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectFromParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent<QRectF> for (&'a QRectF) {
  fn mapRectFromParent(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem17mapRectFromParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
impl /*struct*/ QGraphicsItem {
  pub fn setFocusProxy<RetType, T: QGraphicsItem_setFocusProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFocusProxy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setFocusProxy<RetType> {
  fn setFocusProxy(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setFocusProxy(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_setFocusProxy<()> for (&'a QGraphicsItem) {
  fn setFocusProxy(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13setFocusProxyEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem13setFocusProxyEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::acceptDrops();
impl /*struct*/ QGraphicsItem {
  pub fn acceptDrops<RetType, T: QGraphicsItem_acceptDrops<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptDrops(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptDrops<RetType> {
  fn acceptDrops(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::acceptDrops();
impl<'a> /*trait*/ QGraphicsItem_acceptDrops<i8> for () {
  fn acceptDrops(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11acceptDropsEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem11acceptDropsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToParent(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPointF> for (&'a QPointF) {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapToParentERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromScene<RetType, T: QGraphicsItem_mapRectFromScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromScene<RetType> {
  fn mapRectFromScene(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene<QRectF> for (&'a QRectF) {
  fn mapRectFromScene(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem16mapRectFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
impl /*struct*/ QGraphicsItem {
  pub fn focusScopeItem<RetType, T: QGraphicsItem_focusScopeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusScopeItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusScopeItem<RetType> {
  fn focusScopeItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::focusScopeItem();
impl<'a> /*trait*/ QGraphicsItem_focusScopeItem<()> for () {
  fn focusScopeItem(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14focusScopeItemEv()};
     unsafe {C_ZNK13QGraphicsItem14focusScopeItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
impl /*struct*/ QGraphicsItem {
  pub fn removeSceneEventFilter<RetType, T: QGraphicsItem_removeSceneEventFilter<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeSceneEventFilter(self);
    // return 1;
  }
}

pub trait QGraphicsItem_removeSceneEventFilter<RetType> {
  fn removeSceneEventFilter(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::removeSceneEventFilter(QGraphicsItem * filterItem);
impl<'a> /*trait*/ QGraphicsItem_removeSceneEventFilter<()> for (&'a QGraphicsItem) {
  fn removeSceneEventFilter(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem22removeSceneEventFilterEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem22removeSceneEventFilterEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
impl /*struct*/ QGraphicsItem {
  pub fn focusProxy<RetType, T: QGraphicsItem_focusProxy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusProxy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_focusProxy<RetType> {
  fn focusProxy(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::focusProxy();
impl<'a> /*trait*/ QGraphicsItem_focusProxy<()> for () {
  fn focusProxy(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10focusProxyEv()};
     unsafe {C_ZNK13QGraphicsItem10focusProxyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPointF> for (&'a QGraphicsItem, &'a QPointF) {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem9mapToItemEPKS_RK7QPointF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
impl /*struct*/ QGraphicsItem {
  pub fn sceneBoundingRect<RetType, T: QGraphicsItem_sceneBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sceneBoundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneBoundingRect<RetType> {
  fn sceneBoundingRect(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::sceneBoundingRect();
impl<'a> /*trait*/ QGraphicsItem_sceneBoundingRect<QRectF> for () {
  fn sceneBoundingRect(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17sceneBoundingRectEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem17sceneBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::~QGraphicsItem();
impl /*struct*/ QGraphicsItem {
  pub fn free<RetType, T: QGraphicsItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::~QGraphicsItem();
impl<'a> /*trait*/ QGraphicsItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItemD2Ev()};
     unsafe {C_ZN13QGraphicsItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setX(qreal x);
impl /*struct*/ QGraphicsItem {
  pub fn setX<RetType, T: QGraphicsItem_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setX<RetType> {
  fn setX(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setX(qreal x);
impl<'a> /*trait*/ QGraphicsItem_setX<()> for (f64) {
  fn setX(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4setXEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
impl /*struct*/ QGraphicsItem {
  pub fn update<RetType, T: QGraphicsItem_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QGraphicsItem_update<RetType> {
  fn update(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::update(qreal x, qreal y, qreal width, qreal height);
impl<'a> /*trait*/ QGraphicsItem_update<()> for (f64, f64, f64, f64) {
  fn update(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {C_ZN13QGraphicsItem6updateEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setSelected(bool selected);
impl /*struct*/ QGraphicsItem {
  pub fn setSelected<RetType, T: QGraphicsItem_setSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSelected(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setSelected<RetType> {
  fn setSelected(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setSelected(bool selected);
impl<'a> /*trait*/ QGraphicsItem_setSelected<()> for (i8) {
  fn setSelected(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setSelectedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem11setSelectedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToItem<QRectF> for (&'a QGraphicsItem, &'a QRectF) {
  fn mapRectToItem(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapRectToItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
impl /*struct*/ QGraphicsItem {
  pub fn stackBefore<RetType, T: QGraphicsItem_stackBefore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stackBefore(self);
    // return 1;
  }
}

pub trait QGraphicsItem_stackBefore<RetType> {
  fn stackBefore(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::stackBefore(const QGraphicsItem * sibling);
impl<'a> /*trait*/ QGraphicsItem_stackBefore<()> for (&'a QGraphicsItem) {
  fn stackBefore(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11stackBeforeEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem11stackBeforeEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromItem(const QGraphicsItem * item, qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPointF> for (&'a QGraphicsItem, f64, f64) {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_dd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapFromItemEPKS_dd(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::resetMatrix();
impl /*struct*/ QGraphicsItem {
  pub fn resetMatrix<RetType, T: QGraphicsItem_resetMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_resetMatrix<RetType> {
  fn resetMatrix(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::resetMatrix();
impl<'a> /*trait*/ QGraphicsItem_resetMatrix<()> for () {
  fn resetMatrix(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11resetMatrixEv()};
     unsafe {C_ZN13QGraphicsItem11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::opaqueArea();
impl /*struct*/ QGraphicsItem {
  pub fn opaqueArea<RetType, T: QGraphicsItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::unsetCursor();
impl /*struct*/ QGraphicsItem {
  pub fn unsetCursor<RetType, T: QGraphicsItem_unsetCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_unsetCursor<RetType> {
  fn unsetCursor(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::unsetCursor();
impl<'a> /*trait*/ QGraphicsItem_unsetCursor<()> for () {
  fn unsetCursor(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11unsetCursorEv()};
     unsafe {C_ZN13QGraphicsItem11unsetCursorEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromParent(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPointF> for (f64, f64) {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapFromParentEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectToScene<RetType, T: QGraphicsItem_mapRectToScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectToScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectToScene<RetType> {
  fn mapRectToScene(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene<QRectF> for (&'a QRectF) {
  fn mapRectToScene(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem14mapRectToSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn mapRectFromItem<RetType, T: QGraphicsItem_mapRectFromItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapRectFromItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapRectFromItem<RetType> {
  fn mapRectFromItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem<QRectF> for (&'a QGraphicsItem, f64, f64, f64, f64) {
  fn mapRectFromItem(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem15mapRectFromItemEPKS_dddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::scale();
impl /*struct*/ QGraphicsItem {
  pub fn scale<RetType, T: QGraphicsItem_scale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scale(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scale<RetType> {
  fn scale(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::scale();
impl<'a> /*trait*/ QGraphicsItem_scale<f64> for () {
  fn scale(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5scaleEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem5scaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
impl /*struct*/ QGraphicsItem {
  pub fn setBoundingRegionGranularity<RetType, T: QGraphicsItem_setBoundingRegionGranularity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRegionGranularity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setBoundingRegionGranularity<RetType> {
  fn setBoundingRegionGranularity(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setBoundingRegionGranularity(qreal granularity);
impl<'a> /*trait*/ QGraphicsItem_setBoundingRegionGranularity<()> for (f64) {
  fn setBoundingRegionGranularity(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem28setBoundingRegionGranularityEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem28setBoundingRegionGranularityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptDrops<RetType, T: QGraphicsItem_setAcceptDrops<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceptDrops(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptDrops<RetType> {
  fn setAcceptDrops(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setAcceptDrops(bool on);
impl<'a> /*trait*/ QGraphicsItem_setAcceptDrops<()> for (i8) {
  fn setAcceptDrops(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14setAcceptDropsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem14setAcceptDropsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPolygonF> for (f64, f64, f64, f64) {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12mapFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ungrabKeyboard();
impl /*struct*/ QGraphicsItem {
  pub fn ungrabKeyboard<RetType, T: QGraphicsItem_ungrabKeyboard<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ungrabKeyboard(self);
    // return 1;
  }
}

pub trait QGraphicsItem_ungrabKeyboard<RetType> {
  fn ungrabKeyboard(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::ungrabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_ungrabKeyboard<()> for () {
  fn ungrabKeyboard(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem14ungrabKeyboardEv()};
     unsafe {C_ZN13QGraphicsItem14ungrabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setEnabled(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setEnabled<RetType, T: QGraphicsItem_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setEnabled(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
impl /*struct*/ QGraphicsItem {
  pub fn graphicsEffect<RetType, T: QGraphicsItem_graphicsEffect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.graphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsItem_graphicsEffect<RetType> {
  fn graphicsEffect(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsEffect * QGraphicsItem::graphicsEffect();
impl<'a> /*trait*/ QGraphicsItem_graphicsEffect<()> for () {
  fn graphicsEffect(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14graphicsEffectEv()};
     unsafe {C_ZNK13QGraphicsItem14graphicsEffectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::acceptHoverEvents();
impl /*struct*/ QGraphicsItem {
  pub fn acceptHoverEvents<RetType, T: QGraphicsItem_acceptHoverEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptHoverEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptHoverEvents<RetType> {
  fn acceptHoverEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::acceptHoverEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptHoverEvents<i8> for () {
  fn acceptHoverEvents(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptHoverEventsEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem17acceptHoverEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
impl /*struct*/ QGraphicsItem {
  pub fn topLevelWidget<RetType, T: QGraphicsItem_topLevelWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelWidget(self);
    // return 1;
  }
}

pub trait QGraphicsItem_topLevelWidget<RetType> {
  fn topLevelWidget(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsWidget * QGraphicsItem::topLevelWidget();
impl<'a> /*trait*/ QGraphicsItem_topLevelWidget<()> for () {
  fn topLevelWidget(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14topLevelWidgetEv()};
     unsafe {C_ZNK13QGraphicsItem14topLevelWidgetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
impl /*struct*/ QGraphicsItem {
  pub fn transformations<RetType, T: QGraphicsItem_transformations<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transformations(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transformations<RetType> {
  fn transformations(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QList<QGraphicsTransform *> QGraphicsItem::transformations();
impl<'a> /*trait*/ QGraphicsItem_transformations<()> for () {
  fn transformations(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15transformationsEv()};
     unsafe {C_ZNK13QGraphicsItem15transformationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QGraphicsItem {
  pub fn mapToScene<RetType, T: QGraphicsItem_mapToScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToScene(self);
    // return 1;
  }
}

pub trait QGraphicsItem_mapToScene<RetType> {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPolygonF> for (f64, f64, f64, f64) {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10mapToSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPointF> for (f64, f64) {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10mapToSceneEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromScene<QRectF> for (f64, f64, f64, f64) {
  fn mapRectFromScene(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16mapRectFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem16mapRectFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::advance(int phase);
impl /*struct*/ QGraphicsItem {
  pub fn advance<RetType, T: QGraphicsItem_advance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.advance(self);
    // return 1;
  }
}

pub trait QGraphicsItem_advance<RetType> {
  fn advance(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::advance(int phase);
impl<'a> /*trait*/ QGraphicsItem_advance<()> for (i32) {
  fn advance(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7advanceEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QGraphicsItem7advanceEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMatrix QGraphicsItem::sceneMatrix();
impl /*struct*/ QGraphicsItem {
  pub fn sceneMatrix<RetType, T: QGraphicsItem_sceneMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sceneMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_sceneMatrix<RetType> {
  fn sceneMatrix(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QMatrix QGraphicsItem::sceneMatrix();
impl<'a> /*trait*/ QGraphicsItem_sceneMatrix<QMatrix> for () {
  fn sceneMatrix(self , rsthis: & QGraphicsItem) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11sceneMatrixEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem11sceneMatrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setFiltersChildEvents<RetType, T: QGraphicsItem_setFiltersChildEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFiltersChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setFiltersChildEvents<RetType> {
  fn setFiltersChildEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setFiltersChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setFiltersChildEvents<()> for (i8) {
  fn setFiltersChildEvents(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setFiltersChildEventsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem21setFiltersChildEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPolygonF> for (&'a QPolygonF) {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10mapToSceneERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
impl /*struct*/ QGraphicsItem {
  pub fn itemTransform<RetType, T: QGraphicsItem_itemTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_itemTransform<RetType> {
  fn itemTransform(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::itemTransform(const QGraphicsItem * other, bool * ok);
impl<'a> /*trait*/ QGraphicsItem_itemTransform<QTransform> for (&'a QGraphicsItem, &'a mut Vec<i8>) {
  fn itemTransform(self , rsthis: & QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13itemTransformEPKS_Pb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13itemTransformEPKS_Pb(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setTransformOriginPoint(qreal ax, qreal ay);
impl<'a> /*trait*/ QGraphicsItem_setTransformOriginPoint<()> for (f64, f64) {
  fn setTransformOriginPoint(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem23setTransformOriginPointEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsItem23setTransformOriginPointEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
impl /*struct*/ QGraphicsItem {
  pub fn moveBy<RetType, T: QGraphicsItem_moveBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveBy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_moveBy<RetType> {
  fn moveBy(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::moveBy(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItem_moveBy<()> for (f64, f64) {
  fn moveBy(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6moveByEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsItem6moveByEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPolygonF> for (&'a QPolygonF) {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12mapFromSceneERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
impl /*struct*/ QGraphicsItem {
  pub fn group<RetType, T: QGraphicsItem_group<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QGraphicsItem_group<RetType> {
  fn group(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItemGroup * QGraphicsItem::group();
impl<'a> /*trait*/ QGraphicsItem_group<()> for () {
  fn group(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5groupEv()};
     unsafe {C_ZNK13QGraphicsItem5groupEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::shape();
impl /*struct*/ QGraphicsItem {
  pub fn shape<RetType, T: QGraphicsItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::shape();
impl<'a> /*trait*/ QGraphicsItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5shapeEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPointF> for (f64, f64) {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12mapFromSceneEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
impl /*struct*/ QGraphicsItem {
  pub fn scroll<RetType, T: QGraphicsItem_scroll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scroll(self);
    // return 1;
  }
}

pub trait QGraphicsItem_scroll<RetType> {
  fn scroll(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::scroll(qreal dx, qreal dy, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_scroll<()> for (f64, f64, &'a QRectF) {
  fn scroll(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6scrollEddRK6QRectF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem6scrollEddRK6QRectF(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsItem {
  pub fn isObscuredBy<RetType, T: QGraphicsItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isObscuredByEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12isObscuredByEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromParent(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPointF> for (&'a QPointF) {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapFromParentERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
impl /*struct*/ QGraphicsItem {
  pub fn setData<RetType, T: QGraphicsItem_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setData<RetType> {
  fn setData(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setData(int key, const QVariant & value);
impl<'a> /*trait*/ QGraphicsItem_setData<()> for (i32, &'a QVariant) {
  fn setData(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem7setDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem7setDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
impl /*struct*/ QGraphicsItem {
  pub fn commonAncestorItem<RetType, T: QGraphicsItem_commonAncestorItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.commonAncestorItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_commonAncestorItem<RetType> {
  fn commonAncestorItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::commonAncestorItem(const QGraphicsItem * other);
impl<'a> /*trait*/ QGraphicsItem_commonAncestorItem<()> for (&'a QGraphicsItem) {
  fn commonAncestorItem(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18commonAncestorItemEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QGraphicsItem18commonAncestorItemEPKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapFromScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPainterPath> for (&'a QPainterPath) {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12mapFromSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapToScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPainterPath> for (&'a QPainterPath) {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10mapToSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPolygonF> for (f64, f64, f64, f64) {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapToParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
impl /*struct*/ QGraphicsItem {
  pub fn setGroup<RetType, T: QGraphicsItem_setGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGroup(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setGroup<RetType> {
  fn setGroup(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setGroup(QGraphicsItemGroup * group);
impl<'a> /*trait*/ QGraphicsItem_setGroup<()> for (&'a QGraphicsItemGroup) {
  fn setGroup(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem8setGroupEP18QGraphicsItemGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromParent(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromParent<QRectF> for (f64, f64, f64, f64) {
  fn mapRectFromParent(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17mapRectFromParentEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem17mapRectFromParentEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::show();
impl /*struct*/ QGraphicsItem {
  pub fn show<RetType, T: QGraphicsItem_show<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QGraphicsItem_show<RetType> {
  fn show(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::show();
impl<'a> /*trait*/ QGraphicsItem_show<()> for () {
  fn show(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4showEv()};
     unsafe {C_ZN13QGraphicsItem4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectFromItem(const QGraphicsItem * item, const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapRectFromItem<QRectF> for (&'a QGraphicsItem, &'a QRectF) {
  fn mapRectFromItem(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem15mapRectFromItemEPKS_RK6QRectF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::y();
impl /*struct*/ QGraphicsItem {
  pub fn y<RetType, T: QGraphicsItem_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QGraphicsItem_y<RetType> {
  fn y(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::y();
impl<'a> /*trait*/ QGraphicsItem_y<()> for () {
  fn y(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1yEv()};
     unsafe {C_ZNK13QGraphicsItem1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapFromScene<QPointF> for (&'a QPointF) {
  fn mapFromScene(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12mapFromSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::hasFocus();
impl /*struct*/ QGraphicsItem {
  pub fn hasFocus<RetType, T: QGraphicsItem_hasFocus<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFocus(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hasFocus<RetType> {
  fn hasFocus(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::hasFocus();
impl<'a> /*trait*/ QGraphicsItem_hasFocus<i8> for () {
  fn hasFocus(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8hasFocusEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8hasFocusEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::clipPath();
impl /*struct*/ QGraphicsItem {
  pub fn clipPath<RetType, T: QGraphicsItem_clipPath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clipPath(self);
    // return 1;
  }
}

pub trait QGraphicsItem_clipPath<RetType> {
  fn clipPath(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsItem::clipPath();
impl<'a> /*trait*/ QGraphicsItem_clipPath<QPainterPath> for () {
  fn clipPath(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8clipPathEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem8clipPathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
impl /*struct*/ QGraphicsItem {
  pub fn setPos<RetType, T: QGraphicsItem_setPos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setPos<RetType> {
  fn setPos(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setPos(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_setPos<()> for (f64, f64) {
  fn setPos(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN13QGraphicsItem6setPosEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isEnabled();
impl /*struct*/ QGraphicsItem {
  pub fn isEnabled<RetType, T: QGraphicsItem_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isEnabled();
impl<'a> /*trait*/ QGraphicsItem_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isEnabledEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsItem {
  pub fn contains<RetType, T: QGraphicsItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isPanel();
impl /*struct*/ QGraphicsItem {
  pub fn isPanel<RetType, T: QGraphicsItem_isPanel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPanel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isPanel<RetType> {
  fn isPanel(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isPanel();
impl<'a> /*trait*/ QGraphicsItem_isPanel<i8> for () {
  fn isPanel(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7isPanelEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem7isPanelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::filtersChildEvents();
impl /*struct*/ QGraphicsItem {
  pub fn filtersChildEvents<RetType, T: QGraphicsItem_filtersChildEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filtersChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_filtersChildEvents<RetType> {
  fn filtersChildEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::filtersChildEvents();
impl<'a> /*trait*/ QGraphicsItem_filtersChildEvents<i8> for () {
  fn filtersChildEvents(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem18filtersChildEventsEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem18filtersChildEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::grabKeyboard();
impl /*struct*/ QGraphicsItem {
  pub fn grabKeyboard<RetType, T: QGraphicsItem_grabKeyboard<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabKeyboard(self);
    // return 1;
  }
}

pub trait QGraphicsItem_grabKeyboard<RetType> {
  fn grabKeyboard(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::grabKeyboard();
impl<'a> /*trait*/ QGraphicsItem_grabKeyboard<()> for () {
  fn grabKeyboard(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12grabKeyboardEv()};
     unsafe {C_ZN13QGraphicsItem12grabKeyboardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPainterPath> for (&'a QGraphicsItem, &'a QPainterPath) {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapFromItemEPKS_RK12QPainterPath(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setActive(bool active);
impl /*struct*/ QGraphicsItem {
  pub fn setActive<RetType, T: QGraphicsItem_setActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActive(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setActive<RetType> {
  fn setActive(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setActive(bool active);
impl<'a> /*trait*/ QGraphicsItem_setActive<()> for (i8) {
  fn setActive(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setActiveEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
impl /*struct*/ QGraphicsItem {
  pub fn toGraphicsObject<RetType, T: QGraphicsItem_toGraphicsObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toGraphicsObject(self);
    // return 1;
  }
}

pub trait QGraphicsItem_toGraphicsObject<RetType> {
  fn toGraphicsObject(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsObject * QGraphicsItem::toGraphicsObject();
impl<'a> /*trait*/ QGraphicsItem_toGraphicsObject<()> for () {
  fn toGraphicsObject(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem16toGraphicsObjectEv()};
     unsafe {C_ZN13QGraphicsItem16toGraphicsObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromItem(const QGraphicsItem * item, const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromItem<QPolygonF> for (&'a QGraphicsItem, &'a QPolygonF) {
  fn mapFromItem(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapFromItemEPKS_RK9QPolygonF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setHandlesChildEvents<RetType, T: QGraphicsItem_setHandlesChildEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHandlesChildEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setHandlesChildEvents<RetType> {
  fn setHandlesChildEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setHandlesChildEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setHandlesChildEvents<()> for (i8) {
  fn setHandlesChildEvents(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem21setHandlesChildEventsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem21setHandlesChildEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapFromParent(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapFromParent<QPolygonF> for (&'a QPolygonF) {
  fn mapFromParent(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem13mapFromParentERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToParent(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPointF> for (f64, f64) {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapToParentEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
impl /*struct*/ QGraphicsItem {
  pub fn setMatrix<RetType, T: QGraphicsItem_setMatrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setMatrix<RetType> {
  fn setMatrix(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setMatrix<()> for (&'a QMatrix, i8) {
  fn setMatrix(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN13QGraphicsItem9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::update(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_update<()> for (&'a QRectF) {
  fn update(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem6updateERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToItem(const QGraphicsItem * item, const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsItem_mapToItem<QPolygonF> for (&'a QGraphicsItem, &'a QPolygonF) {
  fn mapToItem(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem9mapToItemEPKS_RK9QPolygonF(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::transform();
impl /*struct*/ QGraphicsItem {
  pub fn transform<RetType, T: QGraphicsItem_transform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_transform<RetType> {
  fn transform(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::transform();
impl<'a> /*trait*/ QGraphicsItem_transform<QTransform> for () {
  fn transform(self , rsthis: & QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9transformEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QGraphicsItem::data(int key);
impl /*struct*/ QGraphicsItem {
  pub fn data<RetType, T: QGraphicsItem_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QGraphicsItem_data<RetType> {
  fn data(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QVariant QGraphicsItem::data(int key);
impl<'a> /*trait*/ QGraphicsItem_data<QVariant> for (i32) {
  fn data(self , rsthis: & QGraphicsItem) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QGraphicsItem4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::hide();
impl /*struct*/ QGraphicsItem {
  pub fn hide<RetType, T: QGraphicsItem_hide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QGraphicsItem_hide<RetType> {
  fn hide(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::hide();
impl<'a> /*trait*/ QGraphicsItem_hide<()> for () {
  fn hide(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem4hideEv()};
     unsafe {C_ZN13QGraphicsItem4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isUnderMouse();
impl /*struct*/ QGraphicsItem {
  pub fn isUnderMouse<RetType, T: QGraphicsItem_isUnderMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUnderMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isUnderMouse<RetType> {
  fn isUnderMouse(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isUnderMouse();
impl<'a> /*trait*/ QGraphicsItem_isUnderMouse<i8> for () {
  fn isUnderMouse(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isUnderMouseEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem12isUnderMouseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptTouchEvents<RetType, T: QGraphicsItem_setAcceptTouchEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceptTouchEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptTouchEvents<RetType> {
  fn setAcceptTouchEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setAcceptTouchEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptTouchEvents<()> for (i8) {
  fn setAcceptTouchEvents(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptTouchEventsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem20setAcceptTouchEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
impl /*struct*/ QGraphicsItem {
  pub fn setAcceptHoverEvents<RetType, T: QGraphicsItem_setAcceptHoverEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceptHoverEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setAcceptHoverEvents<RetType> {
  fn setAcceptHoverEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setAcceptHoverEvents(bool enabled);
impl<'a> /*trait*/ QGraphicsItem_setAcceptHoverEvents<()> for (i8) {
  fn setAcceptHoverEvents(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem20setAcceptHoverEventsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem20setAcceptHoverEventsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
impl /*struct*/ QGraphicsItem {
  pub fn childItems<RetType, T: QGraphicsItem_childItems<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childItems(self);
    // return 1;
  }
}

pub trait QGraphicsItem_childItems<RetType> {
  fn childItems(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QList<QGraphicsItem *> QGraphicsItem::childItems();
impl<'a> /*trait*/ QGraphicsItem_childItems<()> for () {
  fn childItems(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10childItemsEv()};
     unsafe {C_ZNK13QGraphicsItem10childItemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
impl /*struct*/ QGraphicsItem {
  pub fn isAncestorOf<RetType, T: QGraphicsItem_isAncestorOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAncestorOf(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isAncestorOf<RetType> {
  fn isAncestorOf(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isAncestorOf(const QGraphicsItem * child);
impl<'a> /*trait*/ QGraphicsItem_isAncestorOf<i8> for (&'a QGraphicsItem) {
  fn isAncestorOf(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12isAncestorOfEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem12isAncestorOfEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::opacity();
impl /*struct*/ QGraphicsItem {
  pub fn opacity<RetType, T: QGraphicsItem_opacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_opacity<RetType> {
  fn opacity(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::opacity();
impl<'a> /*trait*/ QGraphicsItem_opacity<f64> for () {
  fn opacity(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7opacityEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
impl /*struct*/ QGraphicsItem {
  pub fn isVisibleTo<RetType, T: QGraphicsItem_isVisibleTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisibleTo(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isVisibleTo<RetType> {
  fn isVisibleTo(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isVisibleTo(const QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsItem_isVisibleTo<i8> for (&'a QGraphicsItem) {
  fn isVisibleTo(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11isVisibleToEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11isVisibleToEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QGraphicsItem::toolTip();
impl /*struct*/ QGraphicsItem {
  pub fn toolTip<RetType, T: QGraphicsItem_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QGraphicsItem_toolTip<RetType> {
  fn toolTip(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QString QGraphicsItem::toolTip();
impl<'a> /*trait*/ QGraphicsItem_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QGraphicsItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem7toolTipEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QCursor QGraphicsItem::cursor();
impl /*struct*/ QGraphicsItem {
  pub fn cursor<RetType, T: QGraphicsItem_cursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_cursor<RetType> {
  fn cursor(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QCursor QGraphicsItem::cursor();
impl<'a> /*trait*/ QGraphicsItem_cursor<QCursor> for () {
  fn cursor(self , rsthis: & QGraphicsItem) -> QCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6cursorEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem6cursorEv(rsthis.qclsinst)};
    let mut ret1 = QCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::mapToScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPointF> for (&'a QPointF) {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10mapToSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::zValue();
impl /*struct*/ QGraphicsItem {
  pub fn zValue<RetType, T: QGraphicsItem_zValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zValue(self);
    // return 1;
  }
}

pub trait QGraphicsItem_zValue<RetType> {
  fn zValue(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::zValue();
impl<'a> /*trait*/ QGraphicsItem_zValue<f64> for () {
  fn zValue(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6zValueEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem6zValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QMatrix QGraphicsItem::matrix();
impl /*struct*/ QGraphicsItem {
  pub fn matrix<RetType, T: QGraphicsItem_matrix<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrix(self);
    // return 1;
  }
}

pub trait QGraphicsItem_matrix<RetType> {
  fn matrix(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QMatrix QGraphicsItem::matrix();
impl<'a> /*trait*/ QGraphicsItem_matrix<QMatrix> for () {
  fn matrix(self , rsthis: & QGraphicsItem) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem6matrixEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsItem::mapRectToScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsItem_mapRectToScene<QRectF> for (f64, f64, f64, f64) {
  fn mapRectToScene(self , rsthis: & QGraphicsItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem14mapRectToSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {C_ZNK13QGraphicsItem14mapRectToSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setPos(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItem_setPos<()> for (&'a QPointF) {
  fn setPos(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem6setPosERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem6setPosERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::panel();
impl /*struct*/ QGraphicsItem {
  pub fn panel<RetType, T: QGraphicsItem_panel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.panel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_panel<RetType> {
  fn panel(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::panel();
impl<'a> /*trait*/ QGraphicsItem_panel<()> for () {
  fn panel(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem5panelEv()};
     unsafe {C_ZNK13QGraphicsItem5panelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isClipped();
impl /*struct*/ QGraphicsItem {
  pub fn isClipped<RetType, T: QGraphicsItem_isClipped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isClipped(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isClipped<RetType> {
  fn isClipped(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isClipped();
impl<'a> /*trait*/ QGraphicsItem_isClipped<i8> for () {
  fn isClipped(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isClippedEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem9isClippedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
impl /*struct*/ QGraphicsItem {
  pub fn topLevelItem<RetType, T: QGraphicsItem_topLevelItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.topLevelItem(self);
    // return 1;
  }
}

pub trait QGraphicsItem_topLevelItem<RetType> {
  fn topLevelItem(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItem::topLevelItem();
impl<'a> /*trait*/ QGraphicsItem_topLevelItem<()> for () {
  fn topLevelItem(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem12topLevelItemEv()};
     unsafe {C_ZNK13QGraphicsItem12topLevelItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToScene<QPolygonF> for (&'a QRectF) {
  fn mapToScene(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem10mapToSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem10mapToSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setScale(qreal scale);
impl /*struct*/ QGraphicsItem {
  pub fn setScale<RetType, T: QGraphicsItem_setScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScale(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setScale<RetType> {
  fn setScale(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setScale(qreal scale);
impl<'a> /*trait*/ QGraphicsItem_setScale<()> for (f64) {
  fn setScale(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem8setScaleEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem8setScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
impl /*struct*/ QGraphicsItem {
  pub fn setCursor<RetType, T: QGraphicsItem_setCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCursor(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setCursor<RetType> {
  fn setCursor(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setCursor(const QCursor & cursor);
impl<'a> /*trait*/ QGraphicsItem_setCursor<()> for (&'a QCursor) {
  fn setCursor(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9setCursorERK7QCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGraphicsItem9setCursorERK7QCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isVisible();
impl /*struct*/ QGraphicsItem {
  pub fn isVisible<RetType, T: QGraphicsItem_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isVisible<RetType> {
  fn isVisible(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isVisible();
impl<'a> /*trait*/ QGraphicsItem_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem9isVisibleEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItem::pos();
impl /*struct*/ QGraphicsItem {
  pub fn pos<RetType, T: QGraphicsItem_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QGraphicsItem_pos<RetType> {
  fn pos(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QPointF QGraphicsItem::pos();
impl<'a> /*trait*/ QGraphicsItem_pos<QPointF> for () {
  fn pos(self , rsthis: & QGraphicsItem) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem3posEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem3posEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
impl /*struct*/ QGraphicsItem {
  pub fn isBlockedByModalPanel<RetType, T: QGraphicsItem_isBlockedByModalPanel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isBlockedByModalPanel(self);
    // return 1;
  }
}

pub trait QGraphicsItem_isBlockedByModalPanel<RetType> {
  fn isBlockedByModalPanel(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::isBlockedByModalPanel(QGraphicsItem ** blockingPanel);
impl<'a> /*trait*/ QGraphicsItem_isBlockedByModalPanel<i8> for (&'a QGraphicsItem) {
  fn isBlockedByModalPanel(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem21isBlockedByModalPanelEPPS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::effectiveOpacity();
impl /*struct*/ QGraphicsItem {
  pub fn effectiveOpacity<RetType, T: QGraphicsItem_effectiveOpacity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.effectiveOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_effectiveOpacity<RetType> {
  fn effectiveOpacity(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::effectiveOpacity();
impl<'a> /*trait*/ QGraphicsItem_effectiveOpacity<f64> for () {
  fn effectiveOpacity(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem16effectiveOpacityEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem16effectiveOpacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsItem_ensureVisible<()> for (&'a QRectF, i32, i32) {
  fn ensureVisible(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN13QGraphicsItem13ensureVisibleERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::boundingRegionGranularity();
impl /*struct*/ QGraphicsItem {
  pub fn boundingRegionGranularity<RetType, T: QGraphicsItem_boundingRegionGranularity<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRegionGranularity(self);
    // return 1;
  }
}

pub trait QGraphicsItem_boundingRegionGranularity<RetType> {
  fn boundingRegionGranularity(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::boundingRegionGranularity();
impl<'a> /*trait*/ QGraphicsItem_boundingRegionGranularity<f64> for () {
  fn boundingRegionGranularity(self , rsthis: & QGraphicsItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem25boundingRegionGranularityEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem25boundingRegionGranularityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItem::x();
impl /*struct*/ QGraphicsItem {
  pub fn x<RetType, T: QGraphicsItem_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QGraphicsItem_x<RetType> {
  fn x(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  qreal QGraphicsItem::x();
impl<'a> /*trait*/ QGraphicsItem_x<()> for () {
  fn x(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem1xEv()};
     unsafe {C_ZNK13QGraphicsItem1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::grabMouse();
impl /*struct*/ QGraphicsItem {
  pub fn grabMouse<RetType, T: QGraphicsItem_grabMouse<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabMouse(self);
    // return 1;
  }
}

pub trait QGraphicsItem_grabMouse<RetType> {
  fn grabMouse(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::grabMouse();
impl<'a> /*trait*/ QGraphicsItem_grabMouse<()> for () {
  fn grabMouse(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem9grabMouseEv()};
     unsafe {C_ZN13QGraphicsItem9grabMouseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setVisible(bool visible);
impl /*struct*/ QGraphicsItem {
  pub fn setVisible<RetType, T: QGraphicsItem_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setVisible<RetType> {
  fn setVisible(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setVisible(bool visible);
impl<'a> /*trait*/ QGraphicsItem_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QGraphicsItem10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setRotation(qreal angle);
impl /*struct*/ QGraphicsItem {
  pub fn setRotation<RetType, T: QGraphicsItem_setRotation<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRotation(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setRotation<RetType> {
  fn setRotation(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setRotation(qreal angle);
impl<'a> /*trait*/ QGraphicsItem_setRotation<()> for (f64) {
  fn setRotation(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem11setRotationEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QGraphicsItem11setRotationEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
impl /*struct*/ QGraphicsItem {
  pub fn deviceTransform<RetType, T: QGraphicsItem_deviceTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.deviceTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_deviceTransform<RetType> {
  fn deviceTransform(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  QTransform QGraphicsItem::deviceTransform(const QTransform & viewportTransform);
impl<'a> /*trait*/ QGraphicsItem_deviceTransform<QTransform> for (&'a QTransform) {
  fn deviceTransform(self , rsthis: & QGraphicsItem) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem15deviceTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem15deviceTransformERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QTransform::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsItem::acceptTouchEvents();
impl /*struct*/ QGraphicsItem {
  pub fn acceptTouchEvents<RetType, T: QGraphicsItem_acceptTouchEvents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceptTouchEvents(self);
    // return 1;
  }
}

pub trait QGraphicsItem_acceptTouchEvents<RetType> {
  fn acceptTouchEvents(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  bool QGraphicsItem::acceptTouchEvents();
impl<'a> /*trait*/ QGraphicsItem_acceptTouchEvents<i8> for () {
  fn acceptTouchEvents(self , rsthis: & QGraphicsItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem17acceptTouchEventsEv()};
    let mut ret = unsafe {C_ZNK13QGraphicsItem17acceptTouchEventsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
impl /*struct*/ QGraphicsItem {
  pub fn setTransform<RetType, T: QGraphicsItem_setTransform<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTransform(self);
    // return 1;
  }
}

pub trait QGraphicsItem_setTransform<RetType> {
  fn setTransform(self , rsthis: & QGraphicsItem) -> RetType;
}

  // proto:  void QGraphicsItem::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsItem_setTransform<()> for (&'a QTransform, i8) {
  fn setTransform(self , rsthis: & QGraphicsItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsItem12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN13QGraphicsItem12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPolygonF QGraphicsItem::mapToParent(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsItem_mapToParent<QPolygonF> for (&'a QRectF) {
  fn mapToParent(self , rsthis: & QGraphicsItem) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsItem11mapToParentERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGraphicsItem11mapToParentERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsObject {
    return QGraphicsObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsObject {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsObject {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsObject::QGraphicsObject(QGraphicsItem * parent);
impl /*struct*/ QGraphicsObject {
  pub fn new<T: QGraphicsObject_new>(value: T) -> QGraphicsObject {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsObject_new {
  fn new(self) -> QGraphicsObject;
}

  // proto:  void QGraphicsObject::QGraphicsObject(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsObject_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObjectC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QGraphicsObjectC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsObject::~QGraphicsObject();
impl /*struct*/ QGraphicsObject {
  pub fn free<RetType, T: QGraphicsObject_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsObject_free<RetType> {
  fn free(self , rsthis: & QGraphicsObject) -> RetType;
}

  // proto:  void QGraphicsObject::~QGraphicsObject();
impl<'a> /*trait*/ QGraphicsObject_free<()> for () {
  fn free(self , rsthis: & QGraphicsObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsObjectD2Ev()};
     unsafe {C_ZN15QGraphicsObjectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsObject::metaObject();
impl /*struct*/ QGraphicsObject {
  pub fn metaObject<RetType, T: QGraphicsObject_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsObject_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsObject) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsObject::metaObject();
impl<'a> /*trait*/ QGraphicsObject_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsObject) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsObject10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsObject10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsSimpleTextItem {
    return QGraphicsSimpleTextItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsSimpleTextItem {
  type Target = QAbstractGraphicsShapeItem;

  fn deref(&self) -> &QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
impl AsRef<QAbstractGraphicsShapeItem> for QGraphicsSimpleTextItem {
  fn as_ref(& self) -> & QAbstractGraphicsShapeItem {
    return & self.qbase;
  }
}
  // proto:  int QGraphicsSimpleTextItem::type();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn type_<RetType, T: QGraphicsSimpleTextItem_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_type_<RetType> {
  fn type_(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  int QGraphicsSimpleTextItem::type();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_type_<i32> for () {
  fn type_(self , rsthis: & QGraphicsSimpleTextItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4typeEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QFont QGraphicsSimpleTextItem::font();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn font<RetType, T: QGraphicsSimpleTextItem_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_font<RetType> {
  fn font(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  QFont QGraphicsSimpleTextItem::font();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_font<QFont> for () {
  fn font(self , rsthis: & QGraphicsSimpleTextItem) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4fontEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn paint<RetType, T: QGraphicsSimpleTextItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_paint<RetType> {
  fn paint(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  void QGraphicsSimpleTextItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_paint<()> for (&'a QPainter, &'a QStyleOptionGraphicsItem, &'a QWidget) {
  fn paint(self , rsthis: & QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN23QGraphicsSimpleTextItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsSimpleTextItem::~QGraphicsSimpleTextItem();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn free<RetType, T: QGraphicsSimpleTextItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_free<RetType> {
  fn free(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  void QGraphicsSimpleTextItem::~QGraphicsSimpleTextItem();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_free<()> for () {
  fn free(self , rsthis: & QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemD2Ev()};
     unsafe {C_ZN23QGraphicsSimpleTextItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setText<RetType, T: QGraphicsSimpleTextItem_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setText<RetType> {
  fn setText(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  void QGraphicsSimpleTextItem::setText(const QString & text);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN23QGraphicsSimpleTextItem7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QGraphicsSimpleTextItem::text();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn text<RetType, T: QGraphicsSimpleTextItem_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_text<RetType> {
  fn text(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  QString QGraphicsSimpleTextItem::text();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_text<QString> for () {
  fn text(self , rsthis: & QGraphicsSimpleTextItem) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem4textEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(const QString & text, QGraphicsItem * parent);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn new<T: QGraphicsSimpleTextItem_new>(value: T) -> QGraphicsSimpleTextItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_new {
  fn new(self) -> QGraphicsSimpleTextItem;
}

  // proto:  void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(const QString & text, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_new for (&'a QString, &'a QGraphicsItem) {
  fn new(self) -> QGraphicsSimpleTextItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemC2ERK7QStringP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsSimpleTextItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QGraphicsSimpleTextItemC2ERK7QStringP13QGraphicsItem(arg0, arg1)};
    let rsthis = QGraphicsSimpleTextItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn isObscuredBy<RetType, T: QGraphicsSimpleTextItem_isObscuredBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_isObscuredBy<RetType> {
  fn isObscuredBy(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_isObscuredBy<i8> for (&'a QGraphicsItem) {
  fn isObscuredBy(self , rsthis: & QGraphicsSimpleTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsSimpleTextItem::shape();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn shape<RetType, T: QGraphicsSimpleTextItem_shape<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shape(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_shape<RetType> {
  fn shape(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsSimpleTextItem::shape();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_shape<QPainterPath> for () {
  fn shape(self , rsthis: & QGraphicsSimpleTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem5shapeEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_new for (&'a QGraphicsItem) {
  fn new(self) -> QGraphicsSimpleTextItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItemC2EP13QGraphicsItem()};
    let ctysz: c_int = unsafe{QGraphicsSimpleTextItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QGraphicsSimpleTextItemC2EP13QGraphicsItem(arg0)};
    let rsthis = QGraphicsSimpleTextItem{qbase: QAbstractGraphicsShapeItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn setFont<RetType, T: QGraphicsSimpleTextItem_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_setFont<RetType> {
  fn setFont(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  void QGraphicsSimpleTextItem::setFont(const QFont & font);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QGraphicsSimpleTextItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsSimpleTextItem7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN23QGraphicsSimpleTextItem7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn opaqueArea<RetType, T: QGraphicsSimpleTextItem_opaqueArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_opaqueArea<RetType> {
  fn opaqueArea(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  QPainterPath QGraphicsSimpleTextItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_opaqueArea<QPainterPath> for () {
  fn opaqueArea(self , rsthis: & QGraphicsSimpleTextItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn boundingRect<RetType, T: QGraphicsSimpleTextItem_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  QRectF QGraphicsSimpleTextItem::boundingRect();
impl<'a> /*trait*/ QGraphicsSimpleTextItem_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGraphicsSimpleTextItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem12boundingRectEv()};
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
impl /*struct*/ QGraphicsSimpleTextItem {
  pub fn contains<RetType, T: QGraphicsSimpleTextItem_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QGraphicsSimpleTextItem_contains<RetType> {
  fn contains(self , rsthis: & QGraphicsSimpleTextItem) -> RetType;
}

  // proto:  bool QGraphicsSimpleTextItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsSimpleTextItem_contains<i8> for (&'a QPointF) {
  fn contains(self , rsthis: & QGraphicsSimpleTextItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK23QGraphicsSimpleTextItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QGraphicsTextItem_linkActivated
pub struct QGraphicsTextItem_linkActivated_signal{poi:u64}
impl /* struct */ QGraphicsTextItem {
  pub fn linkActivated(&self) -> QGraphicsTextItem_linkActivated_signal {
     return QGraphicsTextItem_linkActivated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsTextItem_linkActivated_signal {
  pub fn connect<T: QGraphicsTextItem_linkActivated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsTextItem_linkActivated_signal_connect {
  fn connect(self, sigthis: QGraphicsTextItem_linkActivated_signal);
}

#[derive(Default)] // for QGraphicsTextItem_linkHovered
pub struct QGraphicsTextItem_linkHovered_signal{poi:u64}
impl /* struct */ QGraphicsTextItem {
  pub fn linkHovered(&self) -> QGraphicsTextItem_linkHovered_signal {
     return QGraphicsTextItem_linkHovered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsTextItem_linkHovered_signal {
  pub fn connect<T: QGraphicsTextItem_linkHovered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsTextItem_linkHovered_signal_connect {
  fn connect(self, sigthis: QGraphicsTextItem_linkHovered_signal);
}

// linkHovered(const class QString &)
extern fn QGraphicsTextItem_linkHovered_signal_connect_cb_0(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QGraphicsTextItem_linkHovered_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsTextItem_linkHovered_signal_connect for fn(QString) {
  fn connect(self, sigthis: QGraphicsTextItem_linkHovered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsTextItem_linkHovered_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsTextItem_SlotProxy_connect__ZN17QGraphicsTextItem11linkHoveredERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsTextItem_linkHovered_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QGraphicsTextItem_linkHovered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsTextItem_linkHovered_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsTextItem_SlotProxy_connect__ZN17QGraphicsTextItem11linkHoveredERK7QString(arg0, arg1, arg2)};
  }
}
// linkActivated(const class QString &)
extern fn QGraphicsTextItem_linkActivated_signal_connect_cb_1(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QGraphicsTextItem_linkActivated_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QGraphicsTextItem_linkActivated_signal_connect for fn(QString) {
  fn connect(self, sigthis: QGraphicsTextItem_linkActivated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsTextItem_linkActivated_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsTextItem_SlotProxy_connect__ZN17QGraphicsTextItem13linkActivatedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsTextItem_linkActivated_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QGraphicsTextItem_linkActivated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsTextItem_linkActivated_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsTextItem_SlotProxy_connect__ZN17QGraphicsTextItem13linkActivatedERK7QString(arg0, arg1, arg2)};
  }
}
#[derive(Default)] // for QGraphicsObject_childrenChanged
pub struct QGraphicsObject_childrenChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn childrenChanged(&self) -> QGraphicsObject_childrenChanged_signal {
     return QGraphicsObject_childrenChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_childrenChanged_signal {
  pub fn connect<T: QGraphicsObject_childrenChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_childrenChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_childrenChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_parentChanged
pub struct QGraphicsObject_parentChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn parentChanged(&self) -> QGraphicsObject_parentChanged_signal {
     return QGraphicsObject_parentChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_parentChanged_signal {
  pub fn connect<T: QGraphicsObject_parentChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_parentChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_parentChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_heightChanged
pub struct QGraphicsObject_heightChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn heightChanged(&self) -> QGraphicsObject_heightChanged_signal {
     return QGraphicsObject_heightChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_heightChanged_signal {
  pub fn connect<T: QGraphicsObject_heightChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_heightChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_heightChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_zChanged
pub struct QGraphicsObject_zChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn zChanged(&self) -> QGraphicsObject_zChanged_signal {
     return QGraphicsObject_zChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_zChanged_signal {
  pub fn connect<T: QGraphicsObject_zChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_zChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_zChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_visibleChanged
pub struct QGraphicsObject_visibleChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn visibleChanged(&self) -> QGraphicsObject_visibleChanged_signal {
     return QGraphicsObject_visibleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_visibleChanged_signal {
  pub fn connect<T: QGraphicsObject_visibleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_visibleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_visibleChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_yChanged
pub struct QGraphicsObject_yChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn yChanged(&self) -> QGraphicsObject_yChanged_signal {
     return QGraphicsObject_yChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_yChanged_signal {
  pub fn connect<T: QGraphicsObject_yChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_yChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_yChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_widthChanged
pub struct QGraphicsObject_widthChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn widthChanged(&self) -> QGraphicsObject_widthChanged_signal {
     return QGraphicsObject_widthChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_widthChanged_signal {
  pub fn connect<T: QGraphicsObject_widthChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_widthChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_widthChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_opacityChanged
pub struct QGraphicsObject_opacityChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn opacityChanged(&self) -> QGraphicsObject_opacityChanged_signal {
     return QGraphicsObject_opacityChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_opacityChanged_signal {
  pub fn connect<T: QGraphicsObject_opacityChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_opacityChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_opacityChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_rotationChanged
pub struct QGraphicsObject_rotationChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn rotationChanged(&self) -> QGraphicsObject_rotationChanged_signal {
     return QGraphicsObject_rotationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_rotationChanged_signal {
  pub fn connect<T: QGraphicsObject_rotationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_rotationChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_rotationChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_enabledChanged
pub struct QGraphicsObject_enabledChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn enabledChanged(&self) -> QGraphicsObject_enabledChanged_signal {
     return QGraphicsObject_enabledChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_enabledChanged_signal {
  pub fn connect<T: QGraphicsObject_enabledChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_enabledChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_enabledChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_xChanged
pub struct QGraphicsObject_xChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn xChanged(&self) -> QGraphicsObject_xChanged_signal {
     return QGraphicsObject_xChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_xChanged_signal {
  pub fn connect<T: QGraphicsObject_xChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_xChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_xChanged_signal);
}

#[derive(Default)] // for QGraphicsObject_scaleChanged
pub struct QGraphicsObject_scaleChanged_signal{poi:u64}
impl /* struct */ QGraphicsObject {
  pub fn scaleChanged(&self) -> QGraphicsObject_scaleChanged_signal {
     return QGraphicsObject_scaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QGraphicsObject_scaleChanged_signal {
  pub fn connect<T: QGraphicsObject_scaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QGraphicsObject_scaleChanged_signal_connect {
  fn connect(self, sigthis: QGraphicsObject_scaleChanged_signal);
}

// yChanged()
extern fn QGraphicsObject_yChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_yChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_yChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_yChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_yChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8yChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_yChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_yChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_yChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8yChangedEv(arg0, arg1, arg2)};
  }
}
// opacityChanged()
extern fn QGraphicsObject_opacityChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_opacityChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_opacityChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_opacityChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_opacityChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14opacityChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_opacityChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_opacityChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_opacityChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14opacityChangedEv(arg0, arg1, arg2)};
  }
}
// visibleChanged()
extern fn QGraphicsObject_visibleChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_visibleChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_visibleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_visibleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_visibleChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14visibleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_visibleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_visibleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_visibleChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14visibleChangedEv(arg0, arg1, arg2)};
  }
}
// childrenChanged()
extern fn QGraphicsObject_childrenChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_childrenChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_childrenChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_childrenChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_childrenChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject15childrenChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_childrenChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_childrenChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_childrenChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject15childrenChangedEv(arg0, arg1, arg2)};
  }
}
// zChanged()
extern fn QGraphicsObject_zChanged_signal_connect_cb_4(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_zChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_zChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_zChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_zChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8zChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_zChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_zChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_zChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8zChangedEv(arg0, arg1, arg2)};
  }
}
// widthChanged()
extern fn QGraphicsObject_widthChanged_signal_connect_cb_5(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_widthChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_widthChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_widthChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_widthChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject12widthChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_widthChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_widthChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_widthChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject12widthChangedEv(arg0, arg1, arg2)};
  }
}
// rotationChanged()
extern fn QGraphicsObject_rotationChanged_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_rotationChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_rotationChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_rotationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_rotationChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject15rotationChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_rotationChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_rotationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_rotationChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject15rotationChangedEv(arg0, arg1, arg2)};
  }
}
// enabledChanged()
extern fn QGraphicsObject_enabledChanged_signal_connect_cb_7(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_enabledChanged_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_enabledChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_enabledChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_enabledChanged_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14enabledChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_enabledChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_enabledChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_enabledChanged_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject14enabledChangedEv(arg0, arg1, arg2)};
  }
}
// scaleChanged()
extern fn QGraphicsObject_scaleChanged_signal_connect_cb_8(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_scaleChanged_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_scaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_scaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_scaleChanged_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject12scaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_scaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_scaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_scaleChanged_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject12scaleChangedEv(arg0, arg1, arg2)};
  }
}
// heightChanged()
extern fn QGraphicsObject_heightChanged_signal_connect_cb_9(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_heightChanged_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_heightChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_heightChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_heightChanged_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject13heightChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_heightChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_heightChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_heightChanged_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject13heightChangedEv(arg0, arg1, arg2)};
  }
}
// parentChanged()
extern fn QGraphicsObject_parentChanged_signal_connect_cb_10(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_parentChanged_signal_connect_cb_box_10(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_parentChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_parentChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_parentChanged_signal_connect_cb_10 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject13parentChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_parentChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_parentChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_parentChanged_signal_connect_cb_box_10 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject13parentChangedEv(arg0, arg1, arg2)};
  }
}
// xChanged()
extern fn QGraphicsObject_xChanged_signal_connect_cb_11(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QGraphicsObject_xChanged_signal_connect_cb_box_11(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QGraphicsObject_xChanged_signal_connect for fn() {
  fn connect(self, sigthis: QGraphicsObject_xChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_xChanged_signal_connect_cb_11 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8xChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QGraphicsObject_xChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QGraphicsObject_xChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QGraphicsObject_xChanged_signal_connect_cb_box_11 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QGraphicsObject_SlotProxy_connect__ZN15QGraphicsObject8xChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

