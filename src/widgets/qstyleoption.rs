// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qstyleoption.h
// dst-file: /src/widgets/qstyleoption.rs
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
use super::super::gui::qtransform::QTransform; // 771
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
  fn _ZN20QStyleOptionComboBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox();
  fn _ZN20QStyleOptionComboBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(int version);
  fn _ZN20QStyleOptionComboBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
  fn _ZN20QStyleOptionMenuItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem();
  fn _ZN20QStyleOptionMenuItemC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(int version);
  fn _ZN20QStyleOptionMenuItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
  fn _ZN23QStyleHintReturnVariantD0Ev(qthis: *mut c_void);
  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
  fn _ZN23QStyleHintReturnVariantC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
  fn _ZN20QStyleOptionTitleBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
  fn _ZN20QStyleOptionTitleBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
  fn _ZN20QStyleOptionTitleBarC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
  fn _ZN24QStyleOptionGraphicsItemC1Ev(qthis: *mut c_void);
  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
  fn _ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0: *mut c_void) -> c_double;
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
  fn _ZN24QStyleOptionGraphicsItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(int version);
  fn _ZN24QStyleOptionGraphicsItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOption::~QStyleOption();
  fn _ZN12QStyleOptionD0Ev(qthis: *mut c_void);
  // proto:  void QStyleOption::init(const QWidget * w);
  fn _ZN12QStyleOption4initEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
  fn _ZN12QStyleOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOption::QStyleOption(int version, int type);
  fn _ZN12QStyleOptionC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QStyleOption::initFrom(const QWidget * w);
  fn _ZN12QStyleOption8initFromEPK7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
  fn _ZN22QStyleOptionDockWidgetC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(int version);
  fn _ZN22QStyleOptionDockWidgetC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
  fn _ZN22QStyleOptionDockWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
  fn _ZN23QStyleOptionProgressBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(int version);
  fn _ZN23QStyleOptionProgressBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
  fn _ZN23QStyleOptionProgressBarC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
  fn _ZN18QStyleOptionSliderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionSlider::QStyleOptionSlider(int version);
  fn _ZN18QStyleOptionSliderC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionSlider::QStyleOptionSlider();
  fn _ZN18QStyleOptionSliderC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
  fn _ZN17QStyleOptionFrameC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
  fn _ZN17QStyleOptionFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionFrame::QStyleOptionFrame(int version);
  fn _ZN17QStyleOptionFrameC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
  fn _ZN19QStyleOptionComplexC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex & other);
  fn _ZN19QStyleOptionComplexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleHintReturn::~QStyleHintReturn();
  fn _ZN16QStyleHintReturnD0Ev(qthis: *mut c_void);
  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
  fn _ZN16QStyleHintReturnC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
  fn _ZN18QStyleOptionHeaderC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
  fn _ZN18QStyleOptionHeaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionHeader::QStyleOptionHeader(int version);
  fn _ZN18QStyleOptionHeaderC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
  fn _ZN19QStyleOptionToolBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox & other);
  fn _ZN19QStyleOptionToolBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(int version);
  fn _ZN19QStyleOptionToolBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
  fn _ZN21QStyleOptionFocusRectC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
  fn _ZN21QStyleOptionFocusRectC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
  fn _ZN21QStyleOptionFocusRectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
  fn _ZN20QStyleOptionGroupBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
  fn _ZN20QStyleOptionGroupBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
  fn _ZN20QStyleOptionGroupBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
  fn _ZN15QStyleOptionTabC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTab::QStyleOptionTab(int version);
  fn _ZN15QStyleOptionTabC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTab::QStyleOptionTab();
  fn _ZN15QStyleOptionTabC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
  fn _ZN22QStyleOptionTabBarBaseC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(int version);
  fn _ZN22QStyleOptionTabBarBaseC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
  fn _ZN22QStyleOptionTabBarBaseC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
  fn _ZN22QStyleOptionRubberBandC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
  fn _ZN22QStyleOptionRubberBandC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
  fn _ZN22QStyleOptionRubberBandC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionButton::QStyleOptionButton(int version);
  fn _ZN18QStyleOptionButtonC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionButton::QStyleOptionButton();
  fn _ZN18QStyleOptionButtonC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton & other);
  fn _ZN18QStyleOptionButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
  fn _ZN20QStyleHintReturnMaskC1Ev(qthis: *mut c_void);
  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
  fn _ZN20QStyleHintReturnMaskD0Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(int version);
  fn _ZN22QStyleOptionToolButtonC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
  fn _ZN22QStyleOptionToolButtonC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton & other);
  fn _ZN22QStyleOptionToolButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
  fn _ZN20QStyleOptionSizeGripC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
  fn _ZN20QStyleOptionSizeGripC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
  fn _ZN20QStyleOptionSizeGripC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
  fn _ZN20QStyleOptionViewItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(int version);
  fn _ZN20QStyleOptionViewItemC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem();
  fn _ZN20QStyleOptionViewItemC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
  fn _ZN19QStyleOptionSpinBoxC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox & other);
  fn _ZN19QStyleOptionSpinBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(int version);
  fn _ZN19QStyleOptionSpinBoxC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
  fn _ZN19QStyleOptionToolBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(int version);
  fn _ZN19QStyleOptionToolBarC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
  fn _ZN19QStyleOptionToolBarC1Ev(qthis: *mut c_void);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
  fn _ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(int version);
  fn _ZN26QStyleOptionTabWidgetFrameC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
  fn _ZN26QStyleOptionTabWidgetFrameC1Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStyleOptionComboBox)=1
pub struct QStyleOptionComboBox {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionMenuItem)=1
pub struct QStyleOptionMenuItem {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleHintReturnVariant)=24
pub struct QStyleHintReturnVariant {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTitleBar)=1
pub struct QStyleOptionTitleBar {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionGraphicsItem)=1
pub struct QStyleOptionGraphicsItem {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOption)=1
pub struct QStyleOption {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionDockWidget)=1
pub struct QStyleOptionDockWidget {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionProgressBar)=1
pub struct QStyleOptionProgressBar {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionSlider)=1
pub struct QStyleOptionSlider {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionFrame)=1
pub struct QStyleOptionFrame {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionComplex)=1
pub struct QStyleOptionComplex {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleHintReturn)=8
pub struct QStyleHintReturn {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionHeader)=1
pub struct QStyleOptionHeader {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionToolBox)=1
pub struct QStyleOptionToolBox {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionFocusRect)=1
pub struct QStyleOptionFocusRect {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionGroupBox)=1
pub struct QStyleOptionGroupBox {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTab)=1
pub struct QStyleOptionTab {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTabBarBase)=1
pub struct QStyleOptionTabBarBase {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionRubberBand)=1
pub struct QStyleOptionRubberBand {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionButton)=1
pub struct QStyleOptionButton {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleHintReturnMask)=16
pub struct QStyleHintReturnMask {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionToolButton)=1
pub struct QStyleOptionToolButton {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionSizeGrip)=1
pub struct QStyleOptionSizeGrip {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionViewItem)=1
pub struct QStyleOptionViewItem {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionSpinBox)=1
pub struct QStyleOptionSpinBox {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionToolBar)=1
pub struct QStyleOptionToolBar {
  pub qclsinst: *mut c_void,
}

// class sizeof(QStyleOptionTabWidgetFrame)=1
pub struct QStyleOptionTabWidgetFrame {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
impl /*struct*/ QStyleOptionComboBox {
  pub fn NewQStyleOptionComboBox<T: QStyleOptionComboBox_NewQStyleOptionComboBox>(value: T) -> QStyleOptionComboBox {
    let rsthis = value.NewQStyleOptionComboBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComboBox_NewQStyleOptionComboBox {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox;
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox & other);
impl<'a> /*trait*/ QStyleOptionComboBox_NewQStyleOptionComboBox for (QStyleOptionComboBox) {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox();
impl<'a> /*trait*/ QStyleOptionComboBox_NewQStyleOptionComboBox for () {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1Ev()};
    unsafe {_ZN20QStyleOptionComboBoxC1Ev(qthis)};
    let rsthis = QStyleOptionComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComboBox::QStyleOptionComboBox(int version);
impl<'a> /*trait*/ QStyleOptionComboBox_NewQStyleOptionComboBox for (i32) {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionComboBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
impl /*struct*/ QStyleOptionMenuItem {
  pub fn NewQStyleOptionMenuItem<T: QStyleOptionMenuItem_NewQStyleOptionMenuItem>(value: T) -> QStyleOptionMenuItem {
    let rsthis = value.NewQStyleOptionMenuItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionMenuItem_NewQStyleOptionMenuItem {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem;
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem & other);
impl<'a> /*trait*/ QStyleOptionMenuItem_NewQStyleOptionMenuItem for (QStyleOptionMenuItem) {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionMenuItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem();
impl<'a> /*trait*/ QStyleOptionMenuItem_NewQStyleOptionMenuItem for () {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1Ev()};
    unsafe {_ZN20QStyleOptionMenuItemC1Ev(qthis)};
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionMenuItem::QStyleOptionMenuItem(int version);
impl<'a> /*trait*/ QStyleOptionMenuItem_NewQStyleOptionMenuItem for (i32) {
  fn NewQStyleOptionMenuItem(self) -> QStyleOptionMenuItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionMenuItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionMenuItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionMenuItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
impl /*struct*/ QStyleHintReturnVariant {
  pub fn FreeQStyleHintReturnVariant<RetType, T: QStyleHintReturnVariant_FreeQStyleHintReturnVariant<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStyleHintReturnVariant(self);
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_FreeQStyleHintReturnVariant<RetType> {
  fn FreeQStyleHintReturnVariant(self , rsthis: &mut QStyleHintReturnVariant) -> RetType;
}

  // proto:  void QStyleHintReturnVariant::~QStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_FreeQStyleHintReturnVariant<()> for () {
  fn FreeQStyleHintReturnVariant(self , rsthis: &mut QStyleHintReturnVariant) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantD0Ev()};
     unsafe {_ZN23QStyleHintReturnVariantD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
impl /*struct*/ QStyleHintReturnVariant {
  pub fn NewQStyleHintReturnVariant<T: QStyleHintReturnVariant_NewQStyleHintReturnVariant>(value: T) -> QStyleHintReturnVariant {
    let rsthis = value.NewQStyleHintReturnVariant();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnVariant_NewQStyleHintReturnVariant {
  fn NewQStyleHintReturnVariant(self) -> QStyleHintReturnVariant;
}

  // proto:  void QStyleHintReturnVariant::QStyleHintReturnVariant();
impl<'a> /*trait*/ QStyleHintReturnVariant_NewQStyleHintReturnVariant for () {
  fn NewQStyleHintReturnVariant(self) -> QStyleHintReturnVariant {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleHintReturnVariantC1Ev()};
    unsafe {_ZN23QStyleHintReturnVariantC1Ev(qthis)};
    let rsthis = QStyleHintReturnVariant{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
impl /*struct*/ QStyleOptionTitleBar {
  pub fn NewQStyleOptionTitleBar<T: QStyleOptionTitleBar_NewQStyleOptionTitleBar>(value: T) -> QStyleOptionTitleBar {
    let rsthis = value.NewQStyleOptionTitleBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTitleBar_NewQStyleOptionTitleBar {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar;
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(int version);
impl<'a> /*trait*/ QStyleOptionTitleBar_NewQStyleOptionTitleBar for (i32) {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionTitleBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar & other);
impl<'a> /*trait*/ QStyleOptionTitleBar_NewQStyleOptionTitleBar for (QStyleOptionTitleBar) {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionTitleBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTitleBar::QStyleOptionTitleBar();
impl<'a> /*trait*/ QStyleOptionTitleBar_NewQStyleOptionTitleBar for () {
  fn NewQStyleOptionTitleBar(self) -> QStyleOptionTitleBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionTitleBarC1Ev()};
    unsafe {_ZN20QStyleOptionTitleBarC1Ev(qthis)};
    let rsthis = QStyleOptionTitleBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn NewQStyleOptionGraphicsItem<T: QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem>(value: T) -> QStyleOptionGraphicsItem {
    let rsthis = value.NewQStyleOptionGraphicsItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem;
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem();
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for () {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1Ev()};
    unsafe {_ZN24QStyleOptionGraphicsItemC1Ev(qthis)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl /*struct*/ QStyleOptionGraphicsItem {
  pub fn levelOfDetailFromTransform_s<RetType, T: QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.levelOfDetailFromTransform_s();
    // return 1;
  }
}

pub trait QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<RetType> {
  fn levelOfDetailFromTransform_s(self ) -> RetType;
}

  // proto: static qreal QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform & worldTransform);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_levelOfDetailFromTransform_s<f64> for (QTransform) {
  fn levelOfDetailFromTransform_s(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN24QStyleOptionGraphicsItem26levelOfDetailFromTransformERK10QTransform(arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem & other);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for (QStyleOptionGraphicsItem) {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QStyleOptionGraphicsItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(int version);
impl<'a> /*trait*/ QStyleOptionGraphicsItem_NewQStyleOptionGraphicsItem for (i32) {
  fn NewQStyleOptionGraphicsItem(self) -> QStyleOptionGraphicsItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QStyleOptionGraphicsItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN24QStyleOptionGraphicsItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionGraphicsItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOption::~QStyleOption();
impl /*struct*/ QStyleOption {
  pub fn FreeQStyleOption<RetType, T: QStyleOption_FreeQStyleOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStyleOption(self);
    // return 1;
  }
}

pub trait QStyleOption_FreeQStyleOption<RetType> {
  fn FreeQStyleOption(self , rsthis: &mut QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::~QStyleOption();
impl<'a> /*trait*/ QStyleOption_FreeQStyleOption<()> for () {
  fn FreeQStyleOption(self , rsthis: &mut QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionD0Ev()};
     unsafe {_ZN12QStyleOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleOption::init(const QWidget * w);
impl /*struct*/ QStyleOption {
  pub fn init<RetType, T: QStyleOption_init<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.init(self);
    // return 1;
  }
}

pub trait QStyleOption_init<RetType> {
  fn init(self , rsthis: &mut QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::init(const QWidget * w);
impl<'a> /*trait*/ QStyleOption_init<()> for (QWidget) {
  fn init(self , rsthis: &mut QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOption4initEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStyleOption4initEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
impl /*struct*/ QStyleOption {
  pub fn NewQStyleOption<T: QStyleOption_NewQStyleOption>(value: T) -> QStyleOption {
    let rsthis = value.NewQStyleOption();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOption_NewQStyleOption {
  fn NewQStyleOption(self) -> QStyleOption;
}

  // proto:  void QStyleOption::QStyleOption(const QStyleOption & other);
impl<'a> /*trait*/ QStyleOption_NewQStyleOption for (QStyleOption) {
  fn NewQStyleOption(self) -> QStyleOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QStyleOptionC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOption::QStyleOption(int version, int type);
impl<'a> /*trait*/ QStyleOption_NewQStyleOption for (i32, i32) {
  fn NewQStyleOption(self) -> QStyleOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOptionC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QStyleOptionC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOption::initFrom(const QWidget * w);
impl /*struct*/ QStyleOption {
  pub fn initFrom<RetType, T: QStyleOption_initFrom<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.initFrom(self);
    // return 1;
  }
}

pub trait QStyleOption_initFrom<RetType> {
  fn initFrom(self , rsthis: &mut QStyleOption) -> RetType;
}

  // proto:  void QStyleOption::initFrom(const QWidget * w);
impl<'a> /*trait*/ QStyleOption_initFrom<()> for (QWidget) {
  fn initFrom(self , rsthis: &mut QStyleOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QStyleOption8initFromEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QStyleOption8initFromEPK7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl /*struct*/ QStyleOptionDockWidget {
  pub fn NewQStyleOptionDockWidget<T: QStyleOptionDockWidget_NewQStyleOptionDockWidget>(value: T) -> QStyleOptionDockWidget {
    let rsthis = value.NewQStyleOptionDockWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionDockWidget_NewQStyleOptionDockWidget {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget;
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget();
impl<'a> /*trait*/ QStyleOptionDockWidget_NewQStyleOptionDockWidget for () {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1Ev()};
    unsafe {_ZN22QStyleOptionDockWidgetC1Ev(qthis)};
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(int version);
impl<'a> /*trait*/ QStyleOptionDockWidget_NewQStyleOptionDockWidget for (i32) {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionDockWidgetC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget & other);
impl<'a> /*trait*/ QStyleOptionDockWidget_NewQStyleOptionDockWidget for (QStyleOptionDockWidget) {
  fn NewQStyleOptionDockWidget(self) -> QStyleOptionDockWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionDockWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionDockWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionDockWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl /*struct*/ QStyleOptionProgressBar {
  pub fn NewQStyleOptionProgressBar<T: QStyleOptionProgressBar_NewQStyleOptionProgressBar>(value: T) -> QStyleOptionProgressBar {
    let rsthis = value.NewQStyleOptionProgressBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionProgressBar_NewQStyleOptionProgressBar {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar;
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar & other);
impl<'a> /*trait*/ QStyleOptionProgressBar_NewQStyleOptionProgressBar for (QStyleOptionProgressBar) {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QStyleOptionProgressBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar(int version);
impl<'a> /*trait*/ QStyleOptionProgressBar_NewQStyleOptionProgressBar for (i32) {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN23QStyleOptionProgressBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionProgressBar::QStyleOptionProgressBar();
impl<'a> /*trait*/ QStyleOptionProgressBar_NewQStyleOptionProgressBar for () {
  fn NewQStyleOptionProgressBar(self) -> QStyleOptionProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QStyleOptionProgressBarC1Ev()};
    unsafe {_ZN23QStyleOptionProgressBarC1Ev(qthis)};
    let rsthis = QStyleOptionProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
impl /*struct*/ QStyleOptionSlider {
  pub fn NewQStyleOptionSlider<T: QStyleOptionSlider_NewQStyleOptionSlider>(value: T) -> QStyleOptionSlider {
    let rsthis = value.NewQStyleOptionSlider();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSlider_NewQStyleOptionSlider {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider;
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider(const QStyleOptionSlider & other);
impl<'a> /*trait*/ QStyleOptionSlider_NewQStyleOptionSlider for (QStyleOptionSlider) {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionSliderC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider(int version);
impl<'a> /*trait*/ QStyleOptionSlider_NewQStyleOptionSlider for (i32) {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionSliderC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSlider::QStyleOptionSlider();
impl<'a> /*trait*/ QStyleOptionSlider_NewQStyleOptionSlider for () {
  fn NewQStyleOptionSlider(self) -> QStyleOptionSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionSliderC1Ev()};
    unsafe {_ZN18QStyleOptionSliderC1Ev(qthis)};
    let rsthis = QStyleOptionSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl /*struct*/ QStyleOptionFrame {
  pub fn NewQStyleOptionFrame<T: QStyleOptionFrame_NewQStyleOptionFrame>(value: T) -> QStyleOptionFrame {
    let rsthis = value.NewQStyleOptionFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFrame_NewQStyleOptionFrame {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame;
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame();
impl<'a> /*trait*/ QStyleOptionFrame_NewQStyleOptionFrame for () {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1Ev()};
    unsafe {_ZN17QStyleOptionFrameC1Ev(qthis)};
    let rsthis = QStyleOptionFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(const QStyleOptionFrame & other);
impl<'a> /*trait*/ QStyleOptionFrame_NewQStyleOptionFrame for (QStyleOptionFrame) {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QStyleOptionFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFrame::QStyleOptionFrame(int version);
impl<'a> /*trait*/ QStyleOptionFrame_NewQStyleOptionFrame for (i32) {
  fn NewQStyleOptionFrame(self) -> QStyleOptionFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QStyleOptionFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN17QStyleOptionFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
impl /*struct*/ QStyleOptionComplex {
  pub fn NewQStyleOptionComplex<T: QStyleOptionComplex_NewQStyleOptionComplex>(value: T) -> QStyleOptionComplex {
    let rsthis = value.NewQStyleOptionComplex();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComplex_NewQStyleOptionComplex {
  fn NewQStyleOptionComplex(self) -> QStyleOptionComplex;
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(int version, int type);
impl<'a> /*trait*/ QStyleOptionComplex_NewQStyleOptionComplex for (i32, i32) {
  fn NewQStyleOptionComplex(self) -> QStyleOptionComplex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN19QStyleOptionComplexC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleOptionComplex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex & other);
impl<'a> /*trait*/ QStyleOptionComplex_NewQStyleOptionComplex for (QStyleOptionComplex) {
  fn NewQStyleOptionComplex(self) -> QStyleOptionComplex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionComplexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionComplexC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionComplex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHintReturn::~QStyleHintReturn();
impl /*struct*/ QStyleHintReturn {
  pub fn FreeQStyleHintReturn<RetType, T: QStyleHintReturn_FreeQStyleHintReturn<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStyleHintReturn(self);
    // return 1;
  }
}

pub trait QStyleHintReturn_FreeQStyleHintReturn<RetType> {
  fn FreeQStyleHintReturn(self , rsthis: &mut QStyleHintReturn) -> RetType;
}

  // proto:  void QStyleHintReturn::~QStyleHintReturn();
impl<'a> /*trait*/ QStyleHintReturn_FreeQStyleHintReturn<()> for () {
  fn FreeQStyleHintReturn(self , rsthis: &mut QStyleHintReturn) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnD0Ev()};
     unsafe {_ZN16QStyleHintReturnD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
impl /*struct*/ QStyleHintReturn {
  pub fn NewQStyleHintReturn<T: QStyleHintReturn_NewQStyleHintReturn>(value: T) -> QStyleHintReturn {
    let rsthis = value.NewQStyleHintReturn();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturn_NewQStyleHintReturn {
  fn NewQStyleHintReturn(self) -> QStyleHintReturn;
}

  // proto:  void QStyleHintReturn::QStyleHintReturn(int version, int type);
impl<'a> /*trait*/ QStyleHintReturn_NewQStyleHintReturn for (i32, i32) {
  fn NewQStyleHintReturn(self) -> QStyleHintReturn {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStyleHintReturnC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN16QStyleHintReturnC1Eii(qthis, arg0, arg1)};
    let rsthis = QStyleHintReturn{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl /*struct*/ QStyleOptionHeader {
  pub fn NewQStyleOptionHeader<T: QStyleOptionHeader_NewQStyleOptionHeader>(value: T) -> QStyleOptionHeader {
    let rsthis = value.NewQStyleOptionHeader();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionHeader_NewQStyleOptionHeader {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader;
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader();
impl<'a> /*trait*/ QStyleOptionHeader_NewQStyleOptionHeader for () {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1Ev()};
    unsafe {_ZN18QStyleOptionHeaderC1Ev(qthis)};
    let rsthis = QStyleOptionHeader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(const QStyleOptionHeader & other);
impl<'a> /*trait*/ QStyleOptionHeader_NewQStyleOptionHeader for (QStyleOptionHeader) {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionHeaderC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionHeader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionHeader::QStyleOptionHeader(int version);
impl<'a> /*trait*/ QStyleOptionHeader_NewQStyleOptionHeader for (i32) {
  fn NewQStyleOptionHeader(self) -> QStyleOptionHeader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionHeaderC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionHeaderC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionHeader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
impl /*struct*/ QStyleOptionToolBox {
  pub fn NewQStyleOptionToolBox<T: QStyleOptionToolBox_NewQStyleOptionToolBox>(value: T) -> QStyleOptionToolBox {
    let rsthis = value.NewQStyleOptionToolBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBox_NewQStyleOptionToolBox {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox;
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox();
impl<'a> /*trait*/ QStyleOptionToolBox_NewQStyleOptionToolBox for () {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1Ev()};
    unsafe {_ZN19QStyleOptionToolBoxC1Ev(qthis)};
    let rsthis = QStyleOptionToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(const QStyleOptionToolBox & other);
impl<'a> /*trait*/ QStyleOptionToolBox_NewQStyleOptionToolBox for (QStyleOptionToolBox) {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionToolBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBox::QStyleOptionToolBox(int version);
impl<'a> /*trait*/ QStyleOptionToolBox_NewQStyleOptionToolBox for (i32) {
  fn NewQStyleOptionToolBox(self) -> QStyleOptionToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionToolBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
impl /*struct*/ QStyleOptionFocusRect {
  pub fn NewQStyleOptionFocusRect<T: QStyleOptionFocusRect_NewQStyleOptionFocusRect>(value: T) -> QStyleOptionFocusRect {
    let rsthis = value.NewQStyleOptionFocusRect();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionFocusRect_NewQStyleOptionFocusRect {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect;
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(int version);
impl<'a> /*trait*/ QStyleOptionFocusRect_NewQStyleOptionFocusRect for (i32) {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN21QStyleOptionFocusRectC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect();
impl<'a> /*trait*/ QStyleOptionFocusRect_NewQStyleOptionFocusRect for () {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1Ev()};
    unsafe {_ZN21QStyleOptionFocusRectC1Ev(qthis)};
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect & other);
impl<'a> /*trait*/ QStyleOptionFocusRect_NewQStyleOptionFocusRect for (QStyleOptionFocusRect) {
  fn NewQStyleOptionFocusRect(self) -> QStyleOptionFocusRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QStyleOptionFocusRectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QStyleOptionFocusRectC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionFocusRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
impl /*struct*/ QStyleOptionGroupBox {
  pub fn NewQStyleOptionGroupBox<T: QStyleOptionGroupBox_NewQStyleOptionGroupBox>(value: T) -> QStyleOptionGroupBox {
    let rsthis = value.NewQStyleOptionGroupBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionGroupBox_NewQStyleOptionGroupBox {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox;
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(int version);
impl<'a> /*trait*/ QStyleOptionGroupBox_NewQStyleOptionGroupBox for (i32) {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionGroupBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox & other);
impl<'a> /*trait*/ QStyleOptionGroupBox_NewQStyleOptionGroupBox for (QStyleOptionGroupBox) {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionGroupBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionGroupBox::QStyleOptionGroupBox();
impl<'a> /*trait*/ QStyleOptionGroupBox_NewQStyleOptionGroupBox for () {
  fn NewQStyleOptionGroupBox(self) -> QStyleOptionGroupBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionGroupBoxC1Ev()};
    unsafe {_ZN20QStyleOptionGroupBoxC1Ev(qthis)};
    let rsthis = QStyleOptionGroupBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl /*struct*/ QStyleOptionTab {
  pub fn NewQStyleOptionTab<T: QStyleOptionTab_NewQStyleOptionTab>(value: T) -> QStyleOptionTab {
    let rsthis = value.NewQStyleOptionTab();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTab_NewQStyleOptionTab {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab;
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab & other);
impl<'a> /*trait*/ QStyleOptionTab_NewQStyleOptionTab for (QStyleOptionTab) {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QStyleOptionTabC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTab{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab(int version);
impl<'a> /*trait*/ QStyleOptionTab_NewQStyleOptionTab for (i32) {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN15QStyleOptionTabC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTab{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTab::QStyleOptionTab();
impl<'a> /*trait*/ QStyleOptionTab_NewQStyleOptionTab for () {
  fn NewQStyleOptionTab(self) -> QStyleOptionTab {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QStyleOptionTabC1Ev()};
    unsafe {_ZN15QStyleOptionTabC1Ev(qthis)};
    let rsthis = QStyleOptionTab{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl /*struct*/ QStyleOptionTabBarBase {
  pub fn NewQStyleOptionTabBarBase<T: QStyleOptionTabBarBase_NewQStyleOptionTabBarBase>(value: T) -> QStyleOptionTabBarBase {
    let rsthis = value.NewQStyleOptionTabBarBase();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabBarBase_NewQStyleOptionTabBarBase {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase;
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase();
impl<'a> /*trait*/ QStyleOptionTabBarBase_NewQStyleOptionTabBarBase for () {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1Ev()};
    unsafe {_ZN22QStyleOptionTabBarBaseC1Ev(qthis)};
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(int version);
impl<'a> /*trait*/ QStyleOptionTabBarBase_NewQStyleOptionTabBarBase for (i32) {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionTabBarBaseC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase & other);
impl<'a> /*trait*/ QStyleOptionTabBarBase_NewQStyleOptionTabBarBase for (QStyleOptionTabBarBase) {
  fn NewQStyleOptionTabBarBase(self) -> QStyleOptionTabBarBase {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionTabBarBaseC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionTabBarBaseC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabBarBase{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
impl /*struct*/ QStyleOptionRubberBand {
  pub fn NewQStyleOptionRubberBand<T: QStyleOptionRubberBand_NewQStyleOptionRubberBand>(value: T) -> QStyleOptionRubberBand {
    let rsthis = value.NewQStyleOptionRubberBand();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionRubberBand_NewQStyleOptionRubberBand {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand;
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(int version);
impl<'a> /*trait*/ QStyleOptionRubberBand_NewQStyleOptionRubberBand for (i32) {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionRubberBandC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand();
impl<'a> /*trait*/ QStyleOptionRubberBand_NewQStyleOptionRubberBand for () {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1Ev()};
    unsafe {_ZN22QStyleOptionRubberBandC1Ev(qthis)};
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand & other);
impl<'a> /*trait*/ QStyleOptionRubberBand_NewQStyleOptionRubberBand for (QStyleOptionRubberBand) {
  fn NewQStyleOptionRubberBand(self) -> QStyleOptionRubberBand {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionRubberBandC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionRubberBandC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionRubberBand{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionButton::QStyleOptionButton(int version);
impl /*struct*/ QStyleOptionButton {
  pub fn NewQStyleOptionButton<T: QStyleOptionButton_NewQStyleOptionButton>(value: T) -> QStyleOptionButton {
    let rsthis = value.NewQStyleOptionButton();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionButton_NewQStyleOptionButton {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton;
}

  // proto:  void QStyleOptionButton::QStyleOptionButton(int version);
impl<'a> /*trait*/ QStyleOptionButton_NewQStyleOptionButton for (i32) {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN18QStyleOptionButtonC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionButton::QStyleOptionButton();
impl<'a> /*trait*/ QStyleOptionButton_NewQStyleOptionButton for () {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1Ev()};
    unsafe {_ZN18QStyleOptionButtonC1Ev(qthis)};
    let rsthis = QStyleOptionButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton & other);
impl<'a> /*trait*/ QStyleOptionButton_NewQStyleOptionButton for (QStyleOptionButton) {
  fn NewQStyleOptionButton(self) -> QStyleOptionButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QStyleOptionButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QStyleOptionButtonC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
impl /*struct*/ QStyleHintReturnMask {
  pub fn NewQStyleHintReturnMask<T: QStyleHintReturnMask_NewQStyleHintReturnMask>(value: T) -> QStyleHintReturnMask {
    let rsthis = value.NewQStyleHintReturnMask();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleHintReturnMask_NewQStyleHintReturnMask {
  fn NewQStyleHintReturnMask(self) -> QStyleHintReturnMask;
}

  // proto:  void QStyleHintReturnMask::QStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_NewQStyleHintReturnMask for () {
  fn NewQStyleHintReturnMask(self) -> QStyleHintReturnMask {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskC1Ev()};
    unsafe {_ZN20QStyleHintReturnMaskC1Ev(qthis)};
    let rsthis = QStyleHintReturnMask{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
impl /*struct*/ QStyleHintReturnMask {
  pub fn FreeQStyleHintReturnMask<RetType, T: QStyleHintReturnMask_FreeQStyleHintReturnMask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStyleHintReturnMask(self);
    // return 1;
  }
}

pub trait QStyleHintReturnMask_FreeQStyleHintReturnMask<RetType> {
  fn FreeQStyleHintReturnMask(self , rsthis: &mut QStyleHintReturnMask) -> RetType;
}

  // proto:  void QStyleHintReturnMask::~QStyleHintReturnMask();
impl<'a> /*trait*/ QStyleHintReturnMask_FreeQStyleHintReturnMask<()> for () {
  fn FreeQStyleHintReturnMask(self , rsthis: &mut QStyleHintReturnMask) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleHintReturnMaskD0Ev()};
     unsafe {_ZN20QStyleHintReturnMaskD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(int version);
impl /*struct*/ QStyleOptionToolButton {
  pub fn NewQStyleOptionToolButton<T: QStyleOptionToolButton_NewQStyleOptionToolButton>(value: T) -> QStyleOptionToolButton {
    let rsthis = value.NewQStyleOptionToolButton();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolButton_NewQStyleOptionToolButton {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton;
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(int version);
impl<'a> /*trait*/ QStyleOptionToolButton_NewQStyleOptionToolButton for (i32) {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN22QStyleOptionToolButtonC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton();
impl<'a> /*trait*/ QStyleOptionToolButton_NewQStyleOptionToolButton for () {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1Ev()};
    unsafe {_ZN22QStyleOptionToolButtonC1Ev(qthis)};
    let rsthis = QStyleOptionToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton & other);
impl<'a> /*trait*/ QStyleOptionToolButton_NewQStyleOptionToolButton for (QStyleOptionToolButton) {
  fn NewQStyleOptionToolButton(self) -> QStyleOptionToolButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QStyleOptionToolButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QStyleOptionToolButtonC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolButton{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
impl /*struct*/ QStyleOptionSizeGrip {
  pub fn NewQStyleOptionSizeGrip<T: QStyleOptionSizeGrip_NewQStyleOptionSizeGrip>(value: T) -> QStyleOptionSizeGrip {
    let rsthis = value.NewQStyleOptionSizeGrip();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSizeGrip_NewQStyleOptionSizeGrip {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip;
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(int version);
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for (i32) {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionSizeGripC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip();
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for () {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1Ev()};
    unsafe {_ZN20QStyleOptionSizeGripC1Ev(qthis)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip & other);
impl<'a> /*trait*/ QStyleOptionSizeGrip_NewQStyleOptionSizeGrip for (QStyleOptionSizeGrip) {
  fn NewQStyleOptionSizeGrip(self) -> QStyleOptionSizeGrip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionSizeGripC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionSizeGripC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSizeGrip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
impl /*struct*/ QStyleOptionViewItem {
  pub fn NewQStyleOptionViewItem<T: QStyleOptionViewItem_NewQStyleOptionViewItem>(value: T) -> QStyleOptionViewItem {
    let rsthis = value.NewQStyleOptionViewItem();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionViewItem_NewQStyleOptionViewItem {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem;
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem & other);
impl<'a> /*trait*/ QStyleOptionViewItem_NewQStyleOptionViewItem for (QStyleOptionViewItem) {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN20QStyleOptionViewItemC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionViewItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem(int version);
impl<'a> /*trait*/ QStyleOptionViewItem_NewQStyleOptionViewItem for (i32) {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionViewItemC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionViewItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionViewItem::QStyleOptionViewItem();
impl<'a> /*trait*/ QStyleOptionViewItem_NewQStyleOptionViewItem for () {
  fn NewQStyleOptionViewItem(self) -> QStyleOptionViewItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionViewItemC1Ev()};
    unsafe {_ZN20QStyleOptionViewItemC1Ev(qthis)};
    let rsthis = QStyleOptionViewItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
impl /*struct*/ QStyleOptionSpinBox {
  pub fn NewQStyleOptionSpinBox<T: QStyleOptionSpinBox_NewQStyleOptionSpinBox>(value: T) -> QStyleOptionSpinBox {
    let rsthis = value.NewQStyleOptionSpinBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionSpinBox_NewQStyleOptionSpinBox {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox;
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox();
impl<'a> /*trait*/ QStyleOptionSpinBox_NewQStyleOptionSpinBox for () {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1Ev()};
    unsafe {_ZN19QStyleOptionSpinBoxC1Ev(qthis)};
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox & other);
impl<'a> /*trait*/ QStyleOptionSpinBox_NewQStyleOptionSpinBox for (QStyleOptionSpinBox) {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionSpinBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionSpinBox::QStyleOptionSpinBox(int version);
impl<'a> /*trait*/ QStyleOptionSpinBox_NewQStyleOptionSpinBox for (i32) {
  fn NewQStyleOptionSpinBox(self) -> QStyleOptionSpinBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionSpinBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionSpinBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionSpinBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl /*struct*/ QStyleOptionToolBar {
  pub fn NewQStyleOptionToolBar<T: QStyleOptionToolBar_NewQStyleOptionToolBar>(value: T) -> QStyleOptionToolBar {
    let rsthis = value.NewQStyleOptionToolBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionToolBar_NewQStyleOptionToolBar {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar;
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar & other);
impl<'a> /*trait*/ QStyleOptionToolBar_NewQStyleOptionToolBar for (QStyleOptionToolBar) {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyleOptionToolBarC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar(int version);
impl<'a> /*trait*/ QStyleOptionToolBar_NewQStyleOptionToolBar for (i32) {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QStyleOptionToolBarC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionToolBar::QStyleOptionToolBar();
impl<'a> /*trait*/ QStyleOptionToolBar_NewQStyleOptionToolBar for () {
  fn NewQStyleOptionToolBar(self) -> QStyleOptionToolBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyleOptionToolBarC1Ev()};
    unsafe {_ZN19QStyleOptionToolBarC1Ev(qthis)};
    let rsthis = QStyleOptionToolBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn NewQStyleOptionTabWidgetFrame<T: QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.NewQStyleOptionTabWidgetFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame;
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for (QStyleOptionTabWidgetFrame) {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(int version);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for (i32) {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for () {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ev()};
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ev(qthis)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

