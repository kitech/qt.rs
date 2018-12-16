

// mod ::core::QEvent
// package qtcore
// /usr/include/qt/QtCore/qcoreevent.h
// #include <qcoreevent.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QEvent)=24
pub struct QEvent {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QEvent_ITF interface {
//    QEvent_PTR() *QEvent
//}
//func (ptr *QEvent) QEvent_PTR() *QEvent { return ptr }

impl /*struct*/ QEvent {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QEvent {
    return QEvent{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QEvent {
//  type Target = QEventBASE;
//
//  fn deref(&self) -> &QEventBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QEventBASE> for QEvent {
//  fn as_ref(& self) -> & QEventBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qcoreevent.h:297
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QEvent(QEvent::Type)

/*

*/
// QEvent(QEvent::Type) ctx.fn_proto_cpp
impl /*struct*/ QEvent {
  pub fn QEvent_0<T: QEvent_QEvent_0>(value: T) -> QEvent {
    let rsthis = value.QEvent_0();
    return rsthis;
    // return 1;
  }
}

pub trait QEvent_QEvent_0 {
  fn QEvent_0(self) -> QEvent;
}
// QEvent(QEvent::Type) ctx.fn_proto_cpp
impl<'a> /*trait*/ QEvent_QEvent_0 for (i32) {
  fn QEvent_0(self) -> QEvent {
    // unsafe{_ZN6QEventC2ENS_4TypeE()};
    let arg0 = (&self) as *const i32 as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN6QEventC2ENS_4TypeE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QEvent{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:299
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QEvent()

/*

*/
pub fn DeleteQEvent(this :*mut QEvent) {
    // let rv = qtrt::InvokeQtFunc6("_ZN6QEventD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 24)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qcoreevent.h:300
// index:0
// Public Visibility=Default Availability=Available
// [24] QEvent & operator=(const QEvent &)

/*

*/
impl /*struct*/ QEvent {
  pub fn operator_equal_0<RetType, T: QEvent_operator_equal_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.operator_equal_0(self);
    // return 1;
  }
}
pub trait QEvent_operator_equal_0<RetType> {
  fn operator_equal_0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_operator_equal_0<usize> for (usize) {
  fn operator_equal_0(self , rsthis: & QEvent) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QEventaSERKS_", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:301
// index:0
// Public inline Visibility=Default Availability=Available
// [4] QEvent::Type type() const

/*

*/
impl /*struct*/ QEvent {
  pub fn type__0<RetType, T: QEvent_type__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.type__0(self);
    // return 1;
  }
}
pub trait QEvent_type__0<RetType> {
  fn type__0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_type__0<i32> for () {
  fn type__0(self , rsthis: & QEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QEvent4typeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:302
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool spontaneous() const

/*

*/
impl /*struct*/ QEvent {
  pub fn spontaneous_0<RetType, T: QEvent_spontaneous_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.spontaneous_0(self);
    // return 1;
  }
}
pub trait QEvent_spontaneous_0<RetType> {
  fn spontaneous_0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_spontaneous_0<bool> for () {
  fn spontaneous_0(self , rsthis: & QEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QEvent11spontaneousEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:304
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void setAccepted(bool)

/*

*/
impl /*struct*/ QEvent {
  pub fn setAccepted_0<RetType, T: QEvent_setAccepted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAccepted_0(self);
    // return 1;
  }
}
pub trait QEvent_setAccepted_0<RetType> {
  fn setAccepted_0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_setAccepted_0<(/*void*/)> for (bool) {
  fn setAccepted_0(self , rsthis: & QEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN6QEvent11setAcceptedEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:305
// index:0
// Public inline Visibility=Default Availability=Available
// [1] bool isAccepted() const

/*

*/
impl /*struct*/ QEvent {
  pub fn isAccepted_0<RetType, T: QEvent_isAccepted_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAccepted_0(self);
    // return 1;
  }
}
pub trait QEvent_isAccepted_0<RetType> {
  fn isAccepted_0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_isAccepted_0<bool> for () {
  fn isAccepted_0(self , rsthis: & QEvent) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK6QEvent10isAcceptedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:307
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void accept()

/*

*/
impl /*struct*/ QEvent {
  pub fn accept_0<RetType, T: QEvent_accept_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.accept_0(self);
    // return 1;
  }
}
pub trait QEvent_accept_0<RetType> {
  fn accept_0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_accept_0<(/*void*/)> for () {
  fn accept_0(self , rsthis: & QEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QEvent6acceptEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:308
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void ignore()

/*

*/
impl /*struct*/ QEvent {
  pub fn ignore_0<RetType, T: QEvent_ignore_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.ignore_0(self);
    // return 1;
  }
}
pub trait QEvent_ignore_0<RetType> {
  fn ignore_0(self , rsthis: & QEvent) -> RetType;
}
impl<'a> /*trait*/ QEvent_ignore_0<(/*void*/)> for () {
  fn ignore_0(self , rsthis: & QEvent) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN6QEvent6ignoreEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qcoreevent.h:310
// index:0
// Public static Visibility=Default Availability=Available
// [4] int registerEventType(int)

/*

*/
impl /*struct*/ QEvent {
  pub fn registerEventType_0<RetType, T: QEvent_registerEventType_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.registerEventType_0();
    // return 1;
  }
}
pub trait QEvent_registerEventType_0<RetType> {
  fn registerEventType_0(self ) -> RetType;
}
impl<'a> /*trait*/ QEvent_registerEventType_0<i32> for (i32) {
  fn registerEventType_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN6QEvent17registerEventTypeEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QEvent__Type = i32;
// 
pub const QEvent__None :QEvent__Type = 0;
// 
pub const QEvent__Timer :QEvent__Type = 1;
// 
pub const QEvent__MouseButtonPress :QEvent__Type = 2;
// 
pub const QEvent__MouseButtonRelease :QEvent__Type = 3;
// 
pub const QEvent__MouseButtonDblClick :QEvent__Type = 4;
// 
pub const QEvent__MouseMove :QEvent__Type = 5;
// 
pub const QEvent__KeyPress :QEvent__Type = 6;
// 
pub const QEvent__KeyRelease :QEvent__Type = 7;
// 
pub const QEvent__FocusIn :QEvent__Type = 8;
// 
pub const QEvent__FocusOut :QEvent__Type = 9;
// 
pub const QEvent__FocusAboutToChange :QEvent__Type = 23;
// 
pub const QEvent__Enter :QEvent__Type = 10;
// 
pub const QEvent__Leave :QEvent__Type = 11;
// 
pub const QEvent__Paint :QEvent__Type = 12;
// 
pub const QEvent__Move :QEvent__Type = 13;
// 
pub const QEvent__Resize :QEvent__Type = 14;
// 
pub const QEvent__Create :QEvent__Type = 15;
// 
pub const QEvent__Destroy :QEvent__Type = 16;
// 
pub const QEvent__Show :QEvent__Type = 17;
// 
pub const QEvent__Hide :QEvent__Type = 18;
// 
pub const QEvent__Close :QEvent__Type = 19;
// 
pub const QEvent__Quit :QEvent__Type = 20;
// 
pub const QEvent__ParentChange :QEvent__Type = 21;
// 
pub const QEvent__ParentAboutToChange :QEvent__Type = 131;
// 
pub const QEvent__ThreadChange :QEvent__Type = 22;
// 
pub const QEvent__WindowActivate :QEvent__Type = 24;
// 
pub const QEvent__WindowDeactivate :QEvent__Type = 25;
// 
pub const QEvent__ShowToParent :QEvent__Type = 26;
// 
pub const QEvent__HideToParent :QEvent__Type = 27;
// 
pub const QEvent__Wheel :QEvent__Type = 31;
// 
pub const QEvent__WindowTitleChange :QEvent__Type = 33;
// 
pub const QEvent__WindowIconChange :QEvent__Type = 34;
// 
pub const QEvent__ApplicationWindowIconChange :QEvent__Type = 35;
// 
pub const QEvent__ApplicationFontChange :QEvent__Type = 36;
// 
pub const QEvent__ApplicationLayoutDirectionChange :QEvent__Type = 37;
// 
pub const QEvent__ApplicationPaletteChange :QEvent__Type = 38;
// 
pub const QEvent__PaletteChange :QEvent__Type = 39;
// 
pub const QEvent__Clipboard :QEvent__Type = 40;
// 
pub const QEvent__Speech :QEvent__Type = 42;
// 
pub const QEvent__MetaCall :QEvent__Type = 43;
// 
pub const QEvent__SockAct :QEvent__Type = 50;
// 
pub const QEvent__WinEventAct :QEvent__Type = 132;
// 
pub const QEvent__DeferredDelete :QEvent__Type = 52;
// 
pub const QEvent__DragEnter :QEvent__Type = 60;
// 
pub const QEvent__DragMove :QEvent__Type = 61;
// 
pub const QEvent__DragLeave :QEvent__Type = 62;
// 
pub const QEvent__Drop :QEvent__Type = 63;
// 
pub const QEvent__DragResponse :QEvent__Type = 64;
// 
pub const QEvent__ChildAdded :QEvent__Type = 68;
// 
pub const QEvent__ChildPolished :QEvent__Type = 69;
// 
pub const QEvent__ChildRemoved :QEvent__Type = 71;
// 
pub const QEvent__ShowWindowRequest :QEvent__Type = 73;
// 
pub const QEvent__PolishRequest :QEvent__Type = 74;
// 
pub const QEvent__Polish :QEvent__Type = 75;
// 
pub const QEvent__LayoutRequest :QEvent__Type = 76;
// 
pub const QEvent__UpdateRequest :QEvent__Type = 77;
// 
pub const QEvent__UpdateLater :QEvent__Type = 78;
// 
pub const QEvent__EmbeddingControl :QEvent__Type = 79;
// 
pub const QEvent__ActivateControl :QEvent__Type = 80;
// 
pub const QEvent__DeactivateControl :QEvent__Type = 81;
// 
pub const QEvent__ContextMenu :QEvent__Type = 82;
// 
pub const QEvent__InputMethod :QEvent__Type = 83;
// 
pub const QEvent__TabletMove :QEvent__Type = 87;
// 
pub const QEvent__LocaleChange :QEvent__Type = 88;
// 
pub const QEvent__LanguageChange :QEvent__Type = 89;
// 
pub const QEvent__LayoutDirectionChange :QEvent__Type = 90;
// 
pub const QEvent__Style :QEvent__Type = 91;
// 
pub const QEvent__TabletPress :QEvent__Type = 92;
// 
pub const QEvent__TabletRelease :QEvent__Type = 93;
// 
pub const QEvent__OkRequest :QEvent__Type = 94;
// 
pub const QEvent__HelpRequest :QEvent__Type = 95;
// 
pub const QEvent__IconDrag :QEvent__Type = 96;
// 
pub const QEvent__FontChange :QEvent__Type = 97;
// 
pub const QEvent__EnabledChange :QEvent__Type = 98;
// 
pub const QEvent__ActivationChange :QEvent__Type = 99;
// 
pub const QEvent__StyleChange :QEvent__Type = 100;
// 
pub const QEvent__IconTextChange :QEvent__Type = 101;
// 
pub const QEvent__ModifiedChange :QEvent__Type = 102;
// 
pub const QEvent__MouseTrackingChange :QEvent__Type = 109;
// 
pub const QEvent__WindowBlocked :QEvent__Type = 103;
// 
pub const QEvent__WindowUnblocked :QEvent__Type = 104;
// 
pub const QEvent__WindowStateChange :QEvent__Type = 105;
// 
pub const QEvent__ReadOnlyChange :QEvent__Type = 106;
// 
pub const QEvent__ToolTip :QEvent__Type = 110;
// 
pub const QEvent__WhatsThis :QEvent__Type = 111;
// 
pub const QEvent__StatusTip :QEvent__Type = 112;
// 
pub const QEvent__ActionChanged :QEvent__Type = 113;
// 
pub const QEvent__ActionAdded :QEvent__Type = 114;
// 
pub const QEvent__ActionRemoved :QEvent__Type = 115;
// 
pub const QEvent__FileOpen :QEvent__Type = 116;
// 
pub const QEvent__Shortcut :QEvent__Type = 117;
// 
pub const QEvent__ShortcutOverride :QEvent__Type = 51;
// 
pub const QEvent__WhatsThisClicked :QEvent__Type = 118;
// 
pub const QEvent__ToolBarChange :QEvent__Type = 120;
// 
pub const QEvent__ApplicationActivate :QEvent__Type = 121;
// 
pub const QEvent__ApplicationActivated :QEvent__Type = 121;
// 
pub const QEvent__ApplicationDeactivate :QEvent__Type = 122;
// 
pub const QEvent__ApplicationDeactivated :QEvent__Type = 122;
// 
pub const QEvent__QueryWhatsThis :QEvent__Type = 123;
// 
pub const QEvent__EnterWhatsThisMode :QEvent__Type = 124;
// 
pub const QEvent__LeaveWhatsThisMode :QEvent__Type = 125;
// 
pub const QEvent__ZOrderChange :QEvent__Type = 126;
// 
pub const QEvent__HoverEnter :QEvent__Type = 127;
// 
pub const QEvent__HoverLeave :QEvent__Type = 128;
// 
pub const QEvent__HoverMove :QEvent__Type = 129;
// 
pub const QEvent__AcceptDropsChange :QEvent__Type = 152;
// 
pub const QEvent__ZeroTimerEvent :QEvent__Type = 154;
// 
pub const QEvent__GraphicsSceneMouseMove :QEvent__Type = 155;
// 
pub const QEvent__GraphicsSceneMousePress :QEvent__Type = 156;
// 
pub const QEvent__GraphicsSceneMouseRelease :QEvent__Type = 157;
// 
pub const QEvent__GraphicsSceneMouseDoubleClick :QEvent__Type = 158;
// 
pub const QEvent__GraphicsSceneContextMenu :QEvent__Type = 159;
// 
pub const QEvent__GraphicsSceneHoverEnter :QEvent__Type = 160;
// 
pub const QEvent__GraphicsSceneHoverMove :QEvent__Type = 161;
// 
pub const QEvent__GraphicsSceneHoverLeave :QEvent__Type = 162;
// 
pub const QEvent__GraphicsSceneHelp :QEvent__Type = 163;
// 
pub const QEvent__GraphicsSceneDragEnter :QEvent__Type = 164;
// 
pub const QEvent__GraphicsSceneDragMove :QEvent__Type = 165;
// 
pub const QEvent__GraphicsSceneDragLeave :QEvent__Type = 166;
// 
pub const QEvent__GraphicsSceneDrop :QEvent__Type = 167;
// 
pub const QEvent__GraphicsSceneWheel :QEvent__Type = 168;
// 
pub const QEvent__KeyboardLayoutChange :QEvent__Type = 169;
// 
pub const QEvent__DynamicPropertyChange :QEvent__Type = 170;
// 
pub const QEvent__TabletEnterProximity :QEvent__Type = 171;
// 
pub const QEvent__TabletLeaveProximity :QEvent__Type = 172;
// 
pub const QEvent__NonClientAreaMouseMove :QEvent__Type = 173;
// 
pub const QEvent__NonClientAreaMouseButtonPress :QEvent__Type = 174;
// 
pub const QEvent__NonClientAreaMouseButtonRelease :QEvent__Type = 175;
// 
pub const QEvent__NonClientAreaMouseButtonDblClick :QEvent__Type = 176;
// 
pub const QEvent__MacSizeChange :QEvent__Type = 177;
// 
pub const QEvent__ContentsRectChange :QEvent__Type = 178;
// 
pub const QEvent__MacGLWindowChange :QEvent__Type = 179;
// 
pub const QEvent__FutureCallOut :QEvent__Type = 180;
// 
pub const QEvent__GraphicsSceneResize :QEvent__Type = 181;
// 
pub const QEvent__GraphicsSceneMove :QEvent__Type = 182;
// 
pub const QEvent__CursorChange :QEvent__Type = 183;
// 
pub const QEvent__ToolTipChange :QEvent__Type = 184;
// 
pub const QEvent__NetworkReplyUpdated :QEvent__Type = 185;
// 
pub const QEvent__GrabMouse :QEvent__Type = 186;
// 
pub const QEvent__UngrabMouse :QEvent__Type = 187;
// 
pub const QEvent__GrabKeyboard :QEvent__Type = 188;
// 
pub const QEvent__UngrabKeyboard :QEvent__Type = 189;
// 
pub const QEvent__MacGLClearDrawable :QEvent__Type = 191;
// 
pub const QEvent__StateMachineSignal :QEvent__Type = 192;
// 
pub const QEvent__StateMachineWrapped :QEvent__Type = 193;
// 
pub const QEvent__TouchBegin :QEvent__Type = 194;
// 
pub const QEvent__TouchUpdate :QEvent__Type = 195;
// 
pub const QEvent__TouchEnd :QEvent__Type = 196;
// 
pub const QEvent__NativeGesture :QEvent__Type = 197;
// 
pub const QEvent__RequestSoftwareInputPanel :QEvent__Type = 199;
// 
pub const QEvent__CloseSoftwareInputPanel :QEvent__Type = 200;
// 
pub const QEvent__WinIdChange :QEvent__Type = 203;
// 
pub const QEvent__Gesture :QEvent__Type = 198;
// 
pub const QEvent__GestureOverride :QEvent__Type = 202;
// 
pub const QEvent__ScrollPrepare :QEvent__Type = 204;
// 
pub const QEvent__Scroll :QEvent__Type = 205;
// 
pub const QEvent__Expose :QEvent__Type = 206;
// 
pub const QEvent__InputMethodQuery :QEvent__Type = 207;
// 
pub const QEvent__OrientationChange :QEvent__Type = 208;
// 
pub const QEvent__TouchCancel :QEvent__Type = 209;
// 
pub const QEvent__ThemeChange :QEvent__Type = 210;
// 
pub const QEvent__SockClose :QEvent__Type = 211;
// 
pub const QEvent__PlatformPanel :QEvent__Type = 212;
// 
pub const QEvent__StyleAnimationUpdate :QEvent__Type = 213;
// 
pub const QEvent__ApplicationStateChange :QEvent__Type = 214;
// 
pub const QEvent__WindowChangeInternal :QEvent__Type = 215;
// 
pub const QEvent__ScreenChangeInternal :QEvent__Type = 216;
// 
pub const QEvent__PlatformSurface :QEvent__Type = 217;
// 
pub const QEvent__Pointer :QEvent__Type = 218;
// 
pub const QEvent__TabletTrackingChange :QEvent__Type = 219;
// 
pub const QEvent__User :QEvent__Type = 1000;
// 
pub const QEvent__MaxUser :QEvent__Type = 65535;
pub fn QEvent_TypeItemName(val: i32) ->String {
  match val {
     QEvent__None => // 0
     {return String::from("None");}
     QEvent__Timer => // 1
     {return String::from("Timer");}
     QEvent__MouseButtonPress => // 2
     {return String::from("MouseButtonPress");}
     QEvent__MouseButtonRelease => // 3
     {return String::from("MouseButtonRelease");}
     QEvent__MouseButtonDblClick => // 4
     {return String::from("MouseButtonDblClick");}
     QEvent__MouseMove => // 5
     {return String::from("MouseMove");}
     QEvent__KeyPress => // 6
     {return String::from("KeyPress");}
     QEvent__KeyRelease => // 7
     {return String::from("KeyRelease");}
     QEvent__FocusIn => // 8
     {return String::from("FocusIn");}
     QEvent__FocusOut => // 9
     {return String::from("FocusOut");}
     QEvent__FocusAboutToChange => // 23
     {return String::from("FocusAboutToChange");}
     QEvent__Enter => // 10
     {return String::from("Enter");}
     QEvent__Leave => // 11
     {return String::from("Leave");}
     QEvent__Paint => // 12
     {return String::from("Paint");}
     QEvent__Move => // 13
     {return String::from("Move");}
     QEvent__Resize => // 14
     {return String::from("Resize");}
     QEvent__Create => // 15
     {return String::from("Create");}
     QEvent__Destroy => // 16
     {return String::from("Destroy");}
     QEvent__Show => // 17
     {return String::from("Show");}
     QEvent__Hide => // 18
     {return String::from("Hide");}
     QEvent__Close => // 19
     {return String::from("Close");}
     QEvent__Quit => // 20
     {return String::from("Quit");}
     QEvent__ParentChange => // 21
     {return String::from("ParentChange");}
     QEvent__ParentAboutToChange => // 131
     {return String::from("ParentAboutToChange");}
     QEvent__ThreadChange => // 22
     {return String::from("ThreadChange");}
     QEvent__WindowActivate => // 24
     {return String::from("WindowActivate");}
     QEvent__WindowDeactivate => // 25
     {return String::from("WindowDeactivate");}
     QEvent__ShowToParent => // 26
     {return String::from("ShowToParent");}
     QEvent__HideToParent => // 27
     {return String::from("HideToParent");}
     QEvent__Wheel => // 31
     {return String::from("Wheel");}
     QEvent__WindowTitleChange => // 33
     {return String::from("WindowTitleChange");}
     QEvent__WindowIconChange => // 34
     {return String::from("WindowIconChange");}
     QEvent__ApplicationWindowIconChange => // 35
     {return String::from("ApplicationWindowIconChange");}
     QEvent__ApplicationFontChange => // 36
     {return String::from("ApplicationFontChange");}
     QEvent__ApplicationLayoutDirectionChange => // 37
     {return String::from("ApplicationLayoutDirectionChange");}
     QEvent__ApplicationPaletteChange => // 38
     {return String::from("ApplicationPaletteChange");}
     QEvent__PaletteChange => // 39
     {return String::from("PaletteChange");}
     QEvent__Clipboard => // 40
     {return String::from("Clipboard");}
     QEvent__Speech => // 42
     {return String::from("Speech");}
     QEvent__MetaCall => // 43
     {return String::from("MetaCall");}
     QEvent__SockAct => // 50
     {return String::from("SockAct");}
     QEvent__WinEventAct => // 132
     {return String::from("WinEventAct");}
     QEvent__DeferredDelete => // 52
     {return String::from("DeferredDelete");}
     QEvent__DragEnter => // 60
     {return String::from("DragEnter");}
     QEvent__DragMove => // 61
     {return String::from("DragMove");}
     QEvent__DragLeave => // 62
     {return String::from("DragLeave");}
     QEvent__Drop => // 63
     {return String::from("Drop");}
     QEvent__DragResponse => // 64
     {return String::from("DragResponse");}
     QEvent__ChildAdded => // 68
     {return String::from("ChildAdded");}
     QEvent__ChildPolished => // 69
     {return String::from("ChildPolished");}
     QEvent__ChildRemoved => // 71
     {return String::from("ChildRemoved");}
     QEvent__ShowWindowRequest => // 73
     {return String::from("ShowWindowRequest");}
     QEvent__PolishRequest => // 74
     {return String::from("PolishRequest");}
     QEvent__Polish => // 75
     {return String::from("Polish");}
     QEvent__LayoutRequest => // 76
     {return String::from("LayoutRequest");}
     QEvent__UpdateRequest => // 77
     {return String::from("UpdateRequest");}
     QEvent__UpdateLater => // 78
     {return String::from("UpdateLater");}
     QEvent__EmbeddingControl => // 79
     {return String::from("EmbeddingControl");}
     QEvent__ActivateControl => // 80
     {return String::from("ActivateControl");}
     QEvent__DeactivateControl => // 81
     {return String::from("DeactivateControl");}
     QEvent__ContextMenu => // 82
     {return String::from("ContextMenu");}
     QEvent__InputMethod => // 83
     {return String::from("InputMethod");}
     QEvent__TabletMove => // 87
     {return String::from("TabletMove");}
     QEvent__LocaleChange => // 88
     {return String::from("LocaleChange");}
     QEvent__LanguageChange => // 89
     {return String::from("LanguageChange");}
     QEvent__LayoutDirectionChange => // 90
     {return String::from("LayoutDirectionChange");}
     QEvent__Style => // 91
     {return String::from("Style");}
     QEvent__TabletPress => // 92
     {return String::from("TabletPress");}
     QEvent__TabletRelease => // 93
     {return String::from("TabletRelease");}
     QEvent__OkRequest => // 94
     {return String::from("OkRequest");}
     QEvent__HelpRequest => // 95
     {return String::from("HelpRequest");}
     QEvent__IconDrag => // 96
     {return String::from("IconDrag");}
     QEvent__FontChange => // 97
     {return String::from("FontChange");}
     QEvent__EnabledChange => // 98
     {return String::from("EnabledChange");}
     QEvent__ActivationChange => // 99
     {return String::from("ActivationChange");}
     QEvent__StyleChange => // 100
     {return String::from("StyleChange");}
     QEvent__IconTextChange => // 101
     {return String::from("IconTextChange");}
     QEvent__ModifiedChange => // 102
     {return String::from("ModifiedChange");}
     QEvent__MouseTrackingChange => // 109
     {return String::from("MouseTrackingChange");}
     QEvent__WindowBlocked => // 103
     {return String::from("WindowBlocked");}
     QEvent__WindowUnblocked => // 104
     {return String::from("WindowUnblocked");}
     QEvent__WindowStateChange => // 105
     {return String::from("WindowStateChange");}
     QEvent__ReadOnlyChange => // 106
     {return String::from("ReadOnlyChange");}
     QEvent__ToolTip => // 110
     {return String::from("ToolTip");}
     QEvent__WhatsThis => // 111
     {return String::from("WhatsThis");}
     QEvent__StatusTip => // 112
     {return String::from("StatusTip");}
     QEvent__ActionChanged => // 113
     {return String::from("ActionChanged");}
     QEvent__ActionAdded => // 114
     {return String::from("ActionAdded");}
     QEvent__ActionRemoved => // 115
     {return String::from("ActionRemoved");}
     QEvent__FileOpen => // 116
     {return String::from("FileOpen");}
     QEvent__Shortcut => // 117
     {return String::from("Shortcut");}
     QEvent__ShortcutOverride => // 51
     {return String::from("ShortcutOverride");}
     QEvent__WhatsThisClicked => // 118
     {return String::from("WhatsThisClicked");}
     QEvent__ToolBarChange => // 120
     {return String::from("ToolBarChange");}
     QEvent__ApplicationActivate => // 121
     {return String::from("ApplicationActivate,ApplicationActivated");}
    // QEvent__ApplicationActivated => // 121
    // {return String::from("");}
     QEvent__ApplicationDeactivate => // 122
     {return String::from("ApplicationDeactivate,ApplicationDeactivated");}
    // QEvent__ApplicationDeactivated => // 122
    // {return String::from("");}
     QEvent__QueryWhatsThis => // 123
     {return String::from("QueryWhatsThis");}
     QEvent__EnterWhatsThisMode => // 124
     {return String::from("EnterWhatsThisMode");}
     QEvent__LeaveWhatsThisMode => // 125
     {return String::from("LeaveWhatsThisMode");}
     QEvent__ZOrderChange => // 126
     {return String::from("ZOrderChange");}
     QEvent__HoverEnter => // 127
     {return String::from("HoverEnter");}
     QEvent__HoverLeave => // 128
     {return String::from("HoverLeave");}
     QEvent__HoverMove => // 129
     {return String::from("HoverMove");}
     QEvent__AcceptDropsChange => // 152
     {return String::from("AcceptDropsChange");}
     QEvent__ZeroTimerEvent => // 154
     {return String::from("ZeroTimerEvent");}
     QEvent__GraphicsSceneMouseMove => // 155
     {return String::from("GraphicsSceneMouseMove");}
     QEvent__GraphicsSceneMousePress => // 156
     {return String::from("GraphicsSceneMousePress");}
     QEvent__GraphicsSceneMouseRelease => // 157
     {return String::from("GraphicsSceneMouseRelease");}
     QEvent__GraphicsSceneMouseDoubleClick => // 158
     {return String::from("GraphicsSceneMouseDoubleClick");}
     QEvent__GraphicsSceneContextMenu => // 159
     {return String::from("GraphicsSceneContextMenu");}
     QEvent__GraphicsSceneHoverEnter => // 160
     {return String::from("GraphicsSceneHoverEnter");}
     QEvent__GraphicsSceneHoverMove => // 161
     {return String::from("GraphicsSceneHoverMove");}
     QEvent__GraphicsSceneHoverLeave => // 162
     {return String::from("GraphicsSceneHoverLeave");}
     QEvent__GraphicsSceneHelp => // 163
     {return String::from("GraphicsSceneHelp");}
     QEvent__GraphicsSceneDragEnter => // 164
     {return String::from("GraphicsSceneDragEnter");}
     QEvent__GraphicsSceneDragMove => // 165
     {return String::from("GraphicsSceneDragMove");}
     QEvent__GraphicsSceneDragLeave => // 166
     {return String::from("GraphicsSceneDragLeave");}
     QEvent__GraphicsSceneDrop => // 167
     {return String::from("GraphicsSceneDrop");}
     QEvent__GraphicsSceneWheel => // 168
     {return String::from("GraphicsSceneWheel");}
     QEvent__KeyboardLayoutChange => // 169
     {return String::from("KeyboardLayoutChange");}
     QEvent__DynamicPropertyChange => // 170
     {return String::from("DynamicPropertyChange");}
     QEvent__TabletEnterProximity => // 171
     {return String::from("TabletEnterProximity");}
     QEvent__TabletLeaveProximity => // 172
     {return String::from("TabletLeaveProximity");}
     QEvent__NonClientAreaMouseMove => // 173
     {return String::from("NonClientAreaMouseMove");}
     QEvent__NonClientAreaMouseButtonPress => // 174
     {return String::from("NonClientAreaMouseButtonPress");}
     QEvent__NonClientAreaMouseButtonRelease => // 175
     {return String::from("NonClientAreaMouseButtonRelease");}
     QEvent__NonClientAreaMouseButtonDblClick => // 176
     {return String::from("NonClientAreaMouseButtonDblClick");}
     QEvent__MacSizeChange => // 177
     {return String::from("MacSizeChange");}
     QEvent__ContentsRectChange => // 178
     {return String::from("ContentsRectChange");}
     QEvent__MacGLWindowChange => // 179
     {return String::from("MacGLWindowChange");}
     QEvent__FutureCallOut => // 180
     {return String::from("FutureCallOut");}
     QEvent__GraphicsSceneResize => // 181
     {return String::from("GraphicsSceneResize");}
     QEvent__GraphicsSceneMove => // 182
     {return String::from("GraphicsSceneMove");}
     QEvent__CursorChange => // 183
     {return String::from("CursorChange");}
     QEvent__ToolTipChange => // 184
     {return String::from("ToolTipChange");}
     QEvent__NetworkReplyUpdated => // 185
     {return String::from("NetworkReplyUpdated");}
     QEvent__GrabMouse => // 186
     {return String::from("GrabMouse");}
     QEvent__UngrabMouse => // 187
     {return String::from("UngrabMouse");}
     QEvent__GrabKeyboard => // 188
     {return String::from("GrabKeyboard");}
     QEvent__UngrabKeyboard => // 189
     {return String::from("UngrabKeyboard");}
     QEvent__MacGLClearDrawable => // 191
     {return String::from("MacGLClearDrawable");}
     QEvent__StateMachineSignal => // 192
     {return String::from("StateMachineSignal");}
     QEvent__StateMachineWrapped => // 193
     {return String::from("StateMachineWrapped");}
     QEvent__TouchBegin => // 194
     {return String::from("TouchBegin");}
     QEvent__TouchUpdate => // 195
     {return String::from("TouchUpdate");}
     QEvent__TouchEnd => // 196
     {return String::from("TouchEnd");}
     QEvent__NativeGesture => // 197
     {return String::from("NativeGesture");}
     QEvent__RequestSoftwareInputPanel => // 199
     {return String::from("RequestSoftwareInputPanel");}
     QEvent__CloseSoftwareInputPanel => // 200
     {return String::from("CloseSoftwareInputPanel");}
     QEvent__WinIdChange => // 203
     {return String::from("WinIdChange");}
     QEvent__Gesture => // 198
     {return String::from("Gesture");}
     QEvent__GestureOverride => // 202
     {return String::from("GestureOverride");}
     QEvent__ScrollPrepare => // 204
     {return String::from("ScrollPrepare");}
     QEvent__Scroll => // 205
     {return String::from("Scroll");}
     QEvent__Expose => // 206
     {return String::from("Expose");}
     QEvent__InputMethodQuery => // 207
     {return String::from("InputMethodQuery");}
     QEvent__OrientationChange => // 208
     {return String::from("OrientationChange");}
     QEvent__TouchCancel => // 209
     {return String::from("TouchCancel");}
     QEvent__ThemeChange => // 210
     {return String::from("ThemeChange");}
     QEvent__SockClose => // 211
     {return String::from("SockClose");}
     QEvent__PlatformPanel => // 212
     {return String::from("PlatformPanel");}
     QEvent__StyleAnimationUpdate => // 213
     {return String::from("StyleAnimationUpdate");}
     QEvent__ApplicationStateChange => // 214
     {return String::from("ApplicationStateChange");}
     QEvent__WindowChangeInternal => // 215
     {return String::from("WindowChangeInternal");}
     QEvent__ScreenChangeInternal => // 216
     {return String::from("ScreenChangeInternal");}
     QEvent__PlatformSurface => // 217
     {return String::from("PlatformSurface");}
     QEvent__Pointer => // 218
     {return String::from("Pointer");}
     QEvent__TabletTrackingChange => // 219
     {return String::from("TabletTrackingChange");}
     QEvent__User => // 1000
     {return String::from("User");}
     QEvent__MaxUser => // 65535
     {return String::from("MaxUser");}
  _ => {return format!("{}", val);}
}
}
pub fn QEvent_TypeItemName_s(val: i32) ->String {
  //var nilthis *QEvent
  //return nilthis.TypeItemName(val);
  return QEvent_TypeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
