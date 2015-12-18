// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qpolygonf::QPolygonF;
use super::qrectf::QRectF;
use super::qpolygon::QPolygon;
use super::qpointf::QPointF;
use super::qmatrix::QMatrix;
use super::qbrush::QBrush;
use super::qpoint::QPoint;
use super::qgraphicsscene::QGraphicsScene;
use super::qwidget::QWidget;
use super::qgraphicsitem::QGraphicsItem;
use super::qtransform::QTransform;
use super::qsize::QSize;
use super::qpainterpath::QPainterPath;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsView::scale(qreal sx, qreal sy);
  fn _ZN13QGraphicsView5scaleEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QPolygonF QGraphicsView::mapToScene(const QRect & rect);
  fn _ZNK13QGraphicsView10mapToSceneERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
  fn _ZNK13QGraphicsView12mapFromSceneERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::translate(qreal dx, qreal dy);
  fn _ZN13QGraphicsView9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QPointF QGraphicsView::mapToScene(int x, int y);
  fn _ZNK13QGraphicsView10mapToSceneEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsView::metaObject();
  fn _ZNK13QGraphicsView10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN13QGraphicsView12setSceneRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  QRect QGraphicsView::rubberBandRect();
  fn _ZNK13QGraphicsView14rubberBandRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
  fn _ZN13QGraphicsView9setMatrixERK7QMatrixb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  bool QGraphicsView::isInteractive();
  fn _ZNK13QGraphicsView13isInteractiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QGraphicsView::NewQGraphicsView(const QGraphicsView & );
  fn _ZN13QGraphicsViewC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsView::setBackgroundBrush(const QBrush & brush);
  fn _ZN13QGraphicsView18setBackgroundBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QGraphicsView::isTransformed();
  fn _ZNK13QGraphicsView13isTransformedEv(qthis: *mut c_void) -> int8_t;
  // proto:  QGraphicsItem * QGraphicsView::itemAt(int x, int y);
  fn _ZNK13QGraphicsView6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QGraphicsView::centerOn(const QPointF & pos);
  fn _ZN13QGraphicsView8centerOnERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsView::setForegroundBrush(const QBrush & brush);
  fn _ZN13QGraphicsView18setForegroundBrushERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsView::shear(qreal sh, qreal sv);
  fn _ZN13QGraphicsView5shearEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QBrush QGraphicsView::foregroundBrush();
  fn _ZNK13QGraphicsView15foregroundBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsView::itemAt(const QPoint & pos);
  fn _ZNK13QGraphicsView6itemAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsView::updateSceneRect(const QRectF & rect);
  fn _ZN13QGraphicsView15updateSceneRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QGraphicsView::mapToScene(const QPoint & point);
  fn _ZNK13QGraphicsView10mapToSceneERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::setInteractive(bool allowed);
  fn _ZN13QGraphicsView14setInteractiveEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QMatrix QGraphicsView::matrix();
  fn _ZNK13QGraphicsView6matrixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h);
  fn _ZNK13QGraphicsView10mapToSceneEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QGraphicsView::NewQGraphicsView(QGraphicsScene * scene, QWidget * parent);
  fn _ZN13QGraphicsViewC1EP14QGraphicsSceneP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QGraphicsView::NewQGraphicsView(QWidget * parent);
  fn _ZN13QGraphicsViewC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsView::centerOn(const QGraphicsItem * item);
  fn _ZN13QGraphicsView8centerOnEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QTransform QGraphicsView::viewportTransform();
  fn _ZNK13QGraphicsView17viewportTransformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::resetCachedContent();
  fn _ZN13QGraphicsView18resetCachedContentEv(qthis: *mut c_void) ;
  // proto:  QPolygonF QGraphicsView::mapToScene(const QPolygon & polygon);
  fn _ZNK13QGraphicsView10mapToSceneERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
  fn _ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  QRectF QGraphicsView::sceneRect();
  fn _ZNK13QGraphicsView9sceneRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsScene * QGraphicsView::scene();
  fn _ZNK13QGraphicsView5sceneEv(qthis: *mut c_void) ;
  // proto:  QSize QGraphicsView::sizeHint();
  fn _ZNK13QGraphicsView8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBrush QGraphicsView::backgroundBrush();
  fn _ZNK13QGraphicsView15backgroundBrushEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QGraphicsView::mapFromScene(qreal x, qreal y);
  fn _ZNK13QGraphicsView12mapFromSceneEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QGraphicsView::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
  fn _ZN13QGraphicsView13ensureVisibleERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  QList<QGraphicsItem *> QGraphicsView::items();
  fn _ZNK13QGraphicsView5itemsEv(qthis: *mut c_void) ;
  // proto:  QTransform QGraphicsView::transform();
  fn _ZNK13QGraphicsView9transformEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QGraphicsItem *> QGraphicsView::items(int x, int y);
  fn _ZNK13QGraphicsView5itemsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QGraphicsView::centerOn(qreal x, qreal y);
  fn _ZN13QGraphicsView8centerOnEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QGraphicsView::rubberBandChanged(QRect viewportRect, QPointF fromScenePoint, QPointF toScenePoint);
  fn _ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QGraphicsView::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
  fn _ZN13QGraphicsView13ensureVisibleEddddii(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int) ;
  // proto:  void QGraphicsView::rotate(qreal angle);
  fn _ZN13QGraphicsView6rotateEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QPolygon QGraphicsView::mapFromScene(qreal x, qreal y, qreal w, qreal h);
  fn _ZNK13QGraphicsView12mapFromSceneEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
  // proto:  void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
  fn _ZN13QGraphicsView12setTransformERK10QTransformb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
  // proto:  void QGraphicsView::setSceneRect(const QRectF & rect);
  fn _ZN13QGraphicsView12setSceneRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPolygon QGraphicsView::mapFromScene(const QPolygonF & polygon);
  fn _ZNK13QGraphicsView12mapFromSceneERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPoint QGraphicsView::mapFromScene(const QPointF & point);
  fn _ZNK13QGraphicsView12mapFromSceneERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsView::mapFromScene(const QPainterPath & path);
  fn _ZNK13QGraphicsView12mapFromSceneERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::FreeQGraphicsView();
  fn _ZN13QGraphicsViewD0Ev(qthis: *mut c_void) ;
  // proto:  QList<QGraphicsItem *> QGraphicsView::items(const QPoint & pos);
  fn _ZNK13QGraphicsView5itemsERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsView::resetMatrix();
  fn _ZN13QGraphicsView11resetMatrixEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsView::resetTransform();
  fn _ZN13QGraphicsView14resetTransformEv(qthis: *mut c_void) ;
  // proto:  QPainterPath QGraphicsView::mapToScene(const QPainterPath & path);
  fn _ZNK13QGraphicsView10mapToSceneERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsView::setScene(QGraphicsScene * scene);
  fn _ZN13QGraphicsView8setSceneEP14QGraphicsScene(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsView)=1
pub struct QGraphicsView {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsView {
  pub fn scale<RetType, T: QGraphicsView_scale<RetType>>(&mut self, value: T) -> RetType {
    return value.scale(self);
    // return 1;
  }
}

pub trait QGraphicsView_scale<RetType> {
  fn scale(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::scale(qreal sx, qreal sy);
impl<'a> /*trait*/ QGraphicsView_scale<()> for (f64, f64) {
  fn scale(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView5scaleEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsView5scaleEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn mapToScene<RetType, T: QGraphicsView_mapToScene<RetType>>(&mut self, value: T) -> RetType {
    return value.mapToScene(self);
    // return 1;
  }
}

pub trait QGraphicsView_mapToScene<RetType> {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QPolygonF QGraphicsView::mapToScene(const QRect & rect);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPolygonF> for (&'a  QRect) {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView10mapToSceneERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn mapFromScene<RetType, T: QGraphicsView_mapFromScene<RetType>>(&mut self, value: T) -> RetType {
    return value.mapFromScene(self);
    // return 1;
  }
}

pub trait QGraphicsView_mapFromScene<RetType> {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QPolygon QGraphicsView::mapFromScene(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPolygon> for (&'a  QRectF) {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView12mapFromSceneERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn translate<RetType, T: QGraphicsView_translate<RetType>>(&mut self, value: T) -> RetType {
    return value.translate(self);
    // return 1;
  }
}

pub trait QGraphicsView_translate<RetType> {
  fn translate(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsView_translate<()> for (f64, f64) {
  fn translate(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsView9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsView::mapToScene(int x, int y);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPointF> for (i32, i32) {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK13QGraphicsView10mapToSceneEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn metaObject<RetType, T: QGraphicsView_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsView_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  const QMetaObject * QGraphicsView::metaObject();
impl<'a> /*trait*/ QGraphicsView_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10metaObjectEv()};
     unsafe {_ZNK13QGraphicsView10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setSceneRect<RetType, T: QGraphicsView_setSceneRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setSceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_setSceneRect<RetType> {
  fn setSceneRect(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setSceneRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsView_setSceneRect<()> for (f64, f64, f64, f64) {
  fn setSceneRect(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setSceneRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN13QGraphicsView12setSceneRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn rubberBandRect<RetType, T: QGraphicsView_rubberBandRect<RetType>>(&mut self, value: T) -> RetType {
    return value.rubberBandRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_rubberBandRect<RetType> {
  fn rubberBandRect(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QRect QGraphicsView::rubberBandRect();
impl<'a> /*trait*/ QGraphicsView_rubberBandRect<QRect> for () {
  fn rubberBandRect(self, rsthis: &mut QGraphicsView) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView14rubberBandRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView14rubberBandRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setMatrix<RetType, T: QGraphicsView_setMatrix<RetType>>(&mut self, value: T) -> RetType {
    return value.setMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsView_setMatrix<RetType> {
  fn setMatrix(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setMatrix(const QMatrix & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsView_setMatrix<()> for (&'a  QMatrix, i8) {
  fn setMatrix(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView9setMatrixERK7QMatrixb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN13QGraphicsView9setMatrixERK7QMatrixb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn isInteractive<RetType, T: QGraphicsView_isInteractive<RetType>>(&mut self, value: T) -> RetType {
    return value.isInteractive(self);
    // return 1;
  }
}

pub trait QGraphicsView_isInteractive<RetType> {
  fn isInteractive(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  bool QGraphicsView::isInteractive();
impl<'a> /*trait*/ QGraphicsView_isInteractive<i8> for () {
  fn isInteractive(self, rsthis: &mut QGraphicsView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView13isInteractiveEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView13isInteractiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn NewQGraphicsView<T: QGraphicsView_NewQGraphicsView>(value: T) -> QGraphicsView {
    let rsthis = value.NewQGraphicsView();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsView_NewQGraphicsView {
  fn NewQGraphicsView(self) -> QGraphicsView;
}

// proto: void QGraphicsView::NewQGraphicsView(const QGraphicsView & );
impl<'a> /*trait*/ QGraphicsView_NewQGraphicsView for (&'a  QGraphicsView) {
  fn NewQGraphicsView(self) -> QGraphicsView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsViewC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setBackgroundBrush<RetType, T: QGraphicsView_setBackgroundBrush<RetType>>(&mut self, value: T) -> RetType {
    return value.setBackgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_setBackgroundBrush<RetType> {
  fn setBackgroundBrush(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setBackgroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsView_setBackgroundBrush<()> for (&'a  QBrush) {
  fn setBackgroundBrush(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18setBackgroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView18setBackgroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn isTransformed<RetType, T: QGraphicsView_isTransformed<RetType>>(&mut self, value: T) -> RetType {
    return value.isTransformed(self);
    // return 1;
  }
}

pub trait QGraphicsView_isTransformed<RetType> {
  fn isTransformed(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  bool QGraphicsView::isTransformed();
impl<'a> /*trait*/ QGraphicsView_isTransformed<i8> for () {
  fn isTransformed(self, rsthis: &mut QGraphicsView) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView13isTransformedEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView13isTransformedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn itemAt<RetType, T: QGraphicsView_itemAt<RetType>>(&mut self, value: T) -> RetType {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsView_itemAt<RetType> {
  fn itemAt(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QGraphicsItem * QGraphicsView::itemAt(int x, int y);
impl<'a> /*trait*/ QGraphicsView_itemAt<()> for (i32, i32) {
  fn itemAt(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK13QGraphicsView6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn centerOn<RetType, T: QGraphicsView_centerOn<RetType>>(&mut self, value: T) -> RetType {
    return value.centerOn(self);
    // return 1;
  }
}

pub trait QGraphicsView_centerOn<RetType> {
  fn centerOn(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::centerOn(const QPointF & pos);
impl<'a> /*trait*/ QGraphicsView_centerOn<()> for (&'a  QPointF) {
  fn centerOn(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView8centerOnERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setForegroundBrush<RetType, T: QGraphicsView_setForegroundBrush<RetType>>(&mut self, value: T) -> RetType {
    return value.setForegroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_setForegroundBrush<RetType> {
  fn setForegroundBrush(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setForegroundBrush(const QBrush & brush);
impl<'a> /*trait*/ QGraphicsView_setForegroundBrush<()> for (&'a  QBrush) {
  fn setForegroundBrush(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18setForegroundBrushERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView18setForegroundBrushERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn shear<RetType, T: QGraphicsView_shear<RetType>>(&mut self, value: T) -> RetType {
    return value.shear(self);
    // return 1;
  }
}

pub trait QGraphicsView_shear<RetType> {
  fn shear(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::shear(qreal sh, qreal sv);
impl<'a> /*trait*/ QGraphicsView_shear<()> for (f64, f64) {
  fn shear(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView5shearEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsView5shearEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn foregroundBrush<RetType, T: QGraphicsView_foregroundBrush<RetType>>(&mut self, value: T) -> RetType {
    return value.foregroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_foregroundBrush<RetType> {
  fn foregroundBrush(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QBrush QGraphicsView::foregroundBrush();
impl<'a> /*trait*/ QGraphicsView_foregroundBrush<QBrush> for () {
  fn foregroundBrush(self, rsthis: &mut QGraphicsView) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView15foregroundBrushEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView15foregroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QGraphicsItem * QGraphicsView::itemAt(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsView_itemAt<()> for (&'a  QPoint) {
  fn itemAt(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6itemAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QGraphicsView6itemAtERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn updateSceneRect<RetType, T: QGraphicsView_updateSceneRect<RetType>>(&mut self, value: T) -> RetType {
    return value.updateSceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_updateSceneRect<RetType> {
  fn updateSceneRect(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::updateSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_updateSceneRect<()> for (&'a  QRectF) {
  fn updateSceneRect(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView15updateSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView15updateSceneRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPointF QGraphicsView::mapToScene(const QPoint & point);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPointF> for (&'a  QPoint) {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView10mapToSceneERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setInteractive<RetType, T: QGraphicsView_setInteractive<RetType>>(&mut self, value: T) -> RetType {
    return value.setInteractive(self);
    // return 1;
  }
}

pub trait QGraphicsView_setInteractive<RetType> {
  fn setInteractive(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setInteractive(bool allowed);
impl<'a> /*trait*/ QGraphicsView_setInteractive<()> for (i8) {
  fn setInteractive(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView14setInteractiveEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN13QGraphicsView14setInteractiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn matrix<RetType, T: QGraphicsView_matrix<RetType>>(&mut self, value: T) -> RetType {
    return value.matrix(self);
    // return 1;
  }
}

pub trait QGraphicsView_matrix<RetType> {
  fn matrix(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QMatrix QGraphicsView::matrix();
impl<'a> /*trait*/ QGraphicsView_matrix<QMatrix> for () {
  fn matrix(self, rsthis: &mut QGraphicsView) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView6matrixEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView6matrixEv(rsthis.qclsinst)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPolygonF QGraphicsView::mapToScene(int x, int y, int w, int h);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPolygonF> for (i32, i32, i32, i32) {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK13QGraphicsView10mapToSceneEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QGraphicsView::NewQGraphicsView(QGraphicsScene * scene, QWidget * parent);
impl<'a> /*trait*/ QGraphicsView_NewQGraphicsView for (&'a mut QGraphicsScene, &'a mut QWidget) {
  fn NewQGraphicsView(self) -> QGraphicsView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC1EP14QGraphicsSceneP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsViewC1EP14QGraphicsSceneP7QWidget(qthis, arg0, arg1)};
    let rsthis = QGraphicsView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsView::NewQGraphicsView(QWidget * parent);
impl<'a> /*trait*/ QGraphicsView_NewQGraphicsView for (&'a mut QWidget) {
  fn NewQGraphicsView(self) -> QGraphicsView {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QGraphicsViewC1EP7QWidget(qthis, arg0)};
    let rsthis = QGraphicsView{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QGraphicsView::centerOn(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsView_centerOn<()> for (&'a  QGraphicsItem) {
  fn centerOn(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView8centerOnEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn viewportTransform<RetType, T: QGraphicsView_viewportTransform<RetType>>(&mut self, value: T) -> RetType {
    return value.viewportTransform(self);
    // return 1;
  }
}

pub trait QGraphicsView_viewportTransform<RetType> {
  fn viewportTransform(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QTransform QGraphicsView::viewportTransform();
impl<'a> /*trait*/ QGraphicsView_viewportTransform<QTransform> for () {
  fn viewportTransform(self, rsthis: &mut QGraphicsView) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView17viewportTransformEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView17viewportTransformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn resetCachedContent<RetType, T: QGraphicsView_resetCachedContent<RetType>>(&mut self, value: T) -> RetType {
    return value.resetCachedContent(self);
    // return 1;
  }
}

pub trait QGraphicsView_resetCachedContent<RetType> {
  fn resetCachedContent(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::resetCachedContent();
impl<'a> /*trait*/ QGraphicsView_resetCachedContent<()> for () {
  fn resetCachedContent(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView18resetCachedContentEv()};
     unsafe {_ZN13QGraphicsView18resetCachedContentEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPolygonF QGraphicsView::mapToScene(const QPolygon & polygon);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPolygonF> for (&'a  QPolygon) {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView10mapToSceneERK8QPolygon(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn ensureVisible<RetType, T: QGraphicsView_ensureVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.ensureVisible(self);
    // return 1;
  }
}

pub trait QGraphicsView_ensureVisible<RetType> {
  fn ensureVisible(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::ensureVisible(const QGraphicsItem * item, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible<()> for (&'a  QGraphicsItem, i32, i32) {
  fn ensureVisible(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QGraphicsView13ensureVisibleEPK13QGraphicsItemii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn sceneRect<RetType, T: QGraphicsView_sceneRect<RetType>>(&mut self, value: T) -> RetType {
    return value.sceneRect(self);
    // return 1;
  }
}

pub trait QGraphicsView_sceneRect<RetType> {
  fn sceneRect(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QRectF QGraphicsView::sceneRect();
impl<'a> /*trait*/ QGraphicsView_sceneRect<QRectF> for () {
  fn sceneRect(self, rsthis: &mut QGraphicsView) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView9sceneRectEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView9sceneRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn scene<RetType, T: QGraphicsView_scene<RetType>>(&mut self, value: T) -> RetType {
    return value.scene(self);
    // return 1;
  }
}

pub trait QGraphicsView_scene<RetType> {
  fn scene(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QGraphicsScene * QGraphicsView::scene();
impl<'a> /*trait*/ QGraphicsView_scene<()> for () {
  fn scene(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5sceneEv()};
     unsafe {_ZNK13QGraphicsView5sceneEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn sizeHint<RetType, T: QGraphicsView_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QGraphicsView_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QSize QGraphicsView::sizeHint();
impl<'a> /*trait*/ QGraphicsView_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QGraphicsView) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView8sizeHintEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn backgroundBrush<RetType, T: QGraphicsView_backgroundBrush<RetType>>(&mut self, value: T) -> RetType {
    return value.backgroundBrush(self);
    // return 1;
  }
}

pub trait QGraphicsView_backgroundBrush<RetType> {
  fn backgroundBrush(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QBrush QGraphicsView::backgroundBrush();
impl<'a> /*trait*/ QGraphicsView_backgroundBrush<QBrush> for () {
  fn backgroundBrush(self, rsthis: &mut QGraphicsView) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView15backgroundBrushEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView15backgroundBrushEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPoint QGraphicsView::mapFromScene(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPoint> for (f64, f64) {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsView12mapFromSceneEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QGraphicsView::ensureVisible(const QRectF & rect, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible<()> for (&'a  QRectF, i32, i32) {
  fn ensureVisible(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN13QGraphicsView13ensureVisibleERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn items<RetType, T: QGraphicsView_items<RetType>>(&mut self, value: T) -> RetType {
    return value.items(self);
    // return 1;
  }
}

pub trait QGraphicsView_items<RetType> {
  fn items(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QList<QGraphicsItem *> QGraphicsView::items();
impl<'a> /*trait*/ QGraphicsView_items<()> for () {
  fn items(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsEv()};
     unsafe {_ZNK13QGraphicsView5itemsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn transform<RetType, T: QGraphicsView_transform<RetType>>(&mut self, value: T) -> RetType {
    return value.transform(self);
    // return 1;
  }
}

pub trait QGraphicsView_transform<RetType> {
  fn transform(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  QTransform QGraphicsView::transform();
impl<'a> /*trait*/ QGraphicsView_transform<QTransform> for () {
  fn transform(self, rsthis: &mut QGraphicsView) -> QTransform {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView9transformEv()};
    let mut ret = unsafe {_ZNK13QGraphicsView9transformEv(rsthis.qclsinst)};
    let mut ret1 = QTransform{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QList<QGraphicsItem *> QGraphicsView::items(int x, int y);
impl<'a> /*trait*/ QGraphicsView_items<()> for (i32, i32) {
  fn items(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK13QGraphicsView5itemsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QGraphicsView::centerOn(qreal x, qreal y);
impl<'a> /*trait*/ QGraphicsView_centerOn<()> for (f64, f64) {
  fn centerOn(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8centerOnEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN13QGraphicsView8centerOnEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn rubberBandChanged<RetType, T: QGraphicsView_rubberBandChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.rubberBandChanged(self);
    // return 1;
  }
}

pub trait QGraphicsView_rubberBandChanged<RetType> {
  fn rubberBandChanged(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::rubberBandChanged(QRect viewportRect, QPointF fromScenePoint, QPointF toScenePoint);
impl<'a> /*trait*/ QGraphicsView_rubberBandChanged<()> for (QRect, QPointF, QPointF) {
  fn rubberBandChanged(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView17rubberBandChangedE5QRect7QPointFS1_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QGraphicsView::ensureVisible(qreal x, qreal y, qreal w, qreal h, int xmargin, int ymargin);
impl<'a> /*trait*/ QGraphicsView_ensureVisible<()> for (f64, f64, f64, f64, i32, i32) {
  fn ensureVisible(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView13ensureVisibleEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN13QGraphicsView13ensureVisibleEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn rotate<RetType, T: QGraphicsView_rotate<RetType>>(&mut self, value: T) -> RetType {
    return value.rotate(self);
    // return 1;
  }
}

pub trait QGraphicsView_rotate<RetType> {
  fn rotate(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::rotate(qreal angle);
impl<'a> /*trait*/ QGraphicsView_rotate<()> for (f64) {
  fn rotate(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView6rotateEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN13QGraphicsView6rotateEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPolygon QGraphicsView::mapFromScene(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPolygon> for (f64, f64, f64, f64) {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK13QGraphicsView12mapFromSceneEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setTransform<RetType, T: QGraphicsView_setTransform<RetType>>(&mut self, value: T) -> RetType {
    return value.setTransform(self);
    // return 1;
  }
}

pub trait QGraphicsView_setTransform<RetType> {
  fn setTransform(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setTransform(const QTransform & matrix, bool combine);
impl<'a> /*trait*/ QGraphicsView_setTransform<()> for (&'a  QTransform, i8) {
  fn setTransform(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setTransformERK10QTransformb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN13QGraphicsView12setTransformERK10QTransformb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QGraphicsView::setSceneRect(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsView_setSceneRect<()> for (&'a  QRectF) {
  fn setSceneRect(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView12setSceneRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView12setSceneRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QPolygon QGraphicsView::mapFromScene(const QPolygonF & polygon);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPolygon> for (&'a  QPolygonF) {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView12mapFromSceneERK9QPolygonF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPoint QGraphicsView::mapFromScene(const QPointF & point);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPoint> for (&'a  QPointF) {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView12mapFromSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPainterPath QGraphicsView::mapFromScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsView_mapFromScene<QPainterPath> for (&'a  QPainterPath) {
  fn mapFromScene(self, rsthis: &mut QGraphicsView) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView12mapFromSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn FreeQGraphicsView<RetType, T: QGraphicsView_FreeQGraphicsView<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsView(self);
    // return 1;
  }
}

pub trait QGraphicsView_FreeQGraphicsView<RetType> {
  fn FreeQGraphicsView(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::FreeQGraphicsView();
impl<'a> /*trait*/ QGraphicsView_FreeQGraphicsView<()> for () {
  fn FreeQGraphicsView(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsViewD0Ev()};
     unsafe {_ZN13QGraphicsViewD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QList<QGraphicsItem *> QGraphicsView::items(const QPoint & pos);
impl<'a> /*trait*/ QGraphicsView_items<()> for (&'a  QPoint) {
  fn items(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView5itemsERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK13QGraphicsView5itemsERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn resetMatrix<RetType, T: QGraphicsView_resetMatrix<RetType>>(&mut self, value: T) -> RetType {
    return value.resetMatrix(self);
    // return 1;
  }
}

pub trait QGraphicsView_resetMatrix<RetType> {
  fn resetMatrix(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::resetMatrix();
impl<'a> /*trait*/ QGraphicsView_resetMatrix<()> for () {
  fn resetMatrix(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView11resetMatrixEv()};
     unsafe {_ZN13QGraphicsView11resetMatrixEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn resetTransform<RetType, T: QGraphicsView_resetTransform<RetType>>(&mut self, value: T) -> RetType {
    return value.resetTransform(self);
    // return 1;
  }
}

pub trait QGraphicsView_resetTransform<RetType> {
  fn resetTransform(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::resetTransform();
impl<'a> /*trait*/ QGraphicsView_resetTransform<()> for () {
  fn resetTransform(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView14resetTransformEv()};
     unsafe {_ZN13QGraphicsView14resetTransformEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QPainterPath QGraphicsView::mapToScene(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsView_mapToScene<QPainterPath> for (&'a  QPainterPath) {
  fn mapToScene(self, rsthis: &mut QGraphicsView) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGraphicsView10mapToSceneERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QGraphicsView10mapToSceneERK12QPainterPath(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsView {
  pub fn setScene<RetType, T: QGraphicsView_setScene<RetType>>(&mut self, value: T) -> RetType {
    return value.setScene(self);
    // return 1;
  }
}

pub trait QGraphicsView_setScene<RetType> {
  fn setScene(self, rsthis: &mut QGraphicsView) -> RetType;
}

// proto:  void QGraphicsView::setScene(QGraphicsScene * scene);
impl<'a> /*trait*/ QGraphicsView_setScene<()> for (&'a mut QGraphicsScene) {
  fn setScene(self, rsthis: &mut QGraphicsView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGraphicsView8setSceneEP14QGraphicsScene()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QGraphicsView8setSceneEP14QGraphicsScene(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

